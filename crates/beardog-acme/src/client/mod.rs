// SPDX-License-Identifier: AGPL-3.0-or-later

//! ACME client — orchestrates the full certificate lifecycle.
//!
//! Handles:
//! 1. Directory discovery (RFC 8555 §7.1.1)
//! 2. Account registration (§7.3)
//! 3. Order creation (§7.4)
//! 4. Challenge completion (§7.5)
//! 5. Certificate download (§7.4.2)
//! 6. Renewal scheduling

mod config;
mod issuance;
mod renewal;

pub use config::AcmeConfig;
use config::Directory;

use crate::account::AcmeAccount;
use crate::challenge::{ChallengeToken, Http01Solver};
use crate::error::AcmeError;
use crate::hot_reload::HotReloadController;
use crate::jws;
use crate::order::{CertificateOrder, OrderStatus};
use crate::storage::CertificateStore;
use reqwest::Client;
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// The ACME client that manages the full certificate lifecycle.
pub struct AcmeClient {
    pub(super) config: AcmeConfig,
    pub(super) http: Client,
    pub(super) store: CertificateStore,
    pub(super) account: Arc<RwLock<AcmeAccount>>,
    pub(super) solver: Http01Solver,
    pub(super) directory: Option<Directory>,
    reload_controller: Option<Arc<HotReloadController>>,
}

impl AcmeClient {
    /// Create a new ACME client.
    ///
    /// # Errors
    ///
    /// Returns an error if the certificate store cannot be initialized.
    pub fn new(config: AcmeConfig) -> Result<Self, AcmeError> {
        let store = CertificateStore::from_env()?;
        let account = AcmeAccount::load_or_create(&store.account_path(), config.contacts.clone())?;

        Ok(Self {
            config,
            http: config::build_http_client()?,
            store,
            account: Arc::new(RwLock::new(account)),
            solver: Http01Solver::new(),
            directory: None,
            reload_controller: None,
        })
    }

    /// Test-only constructor with an explicit certificate store path.
    #[cfg(test)]
    pub(super) fn new_with_store(
        config: AcmeConfig,
        store: CertificateStore,
    ) -> Result<Self, AcmeError> {
        let account = AcmeAccount::generate(config.contacts.clone());

        Ok(Self {
            config,
            http: config::build_http_client()?,
            store,
            account: Arc::new(RwLock::new(account)),
            solver: Http01Solver::new(),
            directory: None,
            reload_controller: None,
        })
    }

    /// Wire a hot-reload controller so certificate renewals update the TLS acceptor.
    pub fn set_reload_controller(&mut self, controller: Arc<HotReloadController>) {
        self.reload_controller = Some(controller);
    }

    /// Discover the ACME directory endpoints.
    ///
    /// # Errors
    ///
    /// Returns an error if the directory cannot be fetched or parsed.
    pub async fn discover_directory(&mut self) -> Result<(), AcmeError> {
        info!(url = %self.config.directory_url, "discovering ACME directory");
        let resp = self.http.get(&self.config.directory_url).send().await?;

        if !resp.status().is_success() {
            return Err(AcmeError::Server {
                status: resp.status().as_u16(),
                detail: "directory fetch failed".to_string(),
            });
        }

        self.directory = Some(resp.json().await?);
        info!("ACME directory discovered");
        Ok(())
    }

    /// Get a fresh anti-replay nonce from the ACME server.
    pub(super) async fn get_nonce(&self) -> Result<String, AcmeError> {
        let dir = self
            .directory
            .as_ref()
            .ok_or_else(|| AcmeError::Config("directory not discovered".to_string()))?;

        let resp = self.http.head(&dir.new_nonce).send().await?;
        resp.headers()
            .get("replay-nonce")
            .and_then(|v| v.to_str().ok())
            .map(String::from)
            .ok_or_else(|| AcmeError::Server {
                status: resp.status().as_u16(),
                detail: "no replay-nonce header".to_string(),
            })
    }

    /// Register or retrieve the ACME account.
    ///
    /// # Errors
    ///
    /// Returns an error if account registration fails.
    pub async fn register_account(&self) -> Result<(), AcmeError> {
        let dir = self
            .directory
            .as_ref()
            .ok_or_else(|| AcmeError::Config("directory not discovered".to_string()))?;

        let mut account = self.account.write().await;
        if account.is_registered() {
            debug!("ACME account already registered");
            return Ok(());
        }

        let nonce = self.get_nonce().await?;
        let payload = json!({
            "termsOfServiceAgreed": true,
            "contact": account.contacts,
        });

        let body = jws::sign_request(
            &account.signing_key(),
            &dir.new_account,
            &nonce,
            &payload,
            None,
        );

        let resp = self
            .http
            .post(&dir.new_account)
            .header("Content-Type", "application/jose+json")
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() && resp.status().as_u16() != 201 {
            let status = resp.status().as_u16();
            let detail = resp.text().await.unwrap_or_default();
            return Err(AcmeError::Server { status, detail });
        }

        if let Some(location) = resp.headers().get("location") {
            let url = location.to_str().unwrap_or("").to_string();
            account.set_account_url(url.clone());
            account.save(&self.store.account_path()).await?;
            info!(account_url = %url, "ACME account registered");
        }

        Ok(())
    }

    /// Create a new certificate order.
    ///
    /// # Errors
    ///
    /// Returns an error if the order cannot be created.
    pub async fn create_order(&self) -> Result<CertificateOrder, AcmeError> {
        let dir = self
            .directory
            .as_ref()
            .ok_or_else(|| AcmeError::Config("directory not discovered".to_string()))?;

        let account = self.account.read().await;
        let kid = account
            .account_url
            .as_deref()
            .ok_or_else(|| AcmeError::AccountKey("account not registered".to_string()))?;

        let nonce = self.get_nonce().await?;

        let identifiers: Vec<Value> = self
            .config
            .domains
            .iter()
            .map(|d| json!({"type": "dns", "value": d}))
            .collect();

        let payload = json!({ "identifiers": identifiers });
        let body = jws::sign_request(
            &account.signing_key(),
            &dir.new_order,
            &nonce,
            &payload,
            Some(kid),
        );

        let resp = self
            .http
            .post(&dir.new_order)
            .header("Content-Type", "application/jose+json")
            .json(&body)
            .send()
            .await?;

        let order_url = resp
            .headers()
            .get("location")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        if !resp.status().is_success() && resp.status().as_u16() != 201 {
            let status = resp.status().as_u16();
            let detail = resp.text().await.unwrap_or_default();
            return Err(AcmeError::Server { status, detail });
        }

        let order_data: Value = resp.json().await?;

        let status = match order_data["status"].as_str().unwrap_or("pending") {
            "ready" => OrderStatus::Ready,
            "processing" => OrderStatus::Processing,
            "valid" => OrderStatus::Valid,
            "invalid" => OrderStatus::Invalid,
            _ => OrderStatus::Pending,
        };

        let authorization_urls: Vec<String> = order_data["authorizations"]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let finalize_url = order_data["finalize"].as_str().unwrap_or("").to_string();

        info!(
            order_url = %order_url,
            domains = ?self.config.domains,
            status = ?status,
            "ACME order created"
        );

        Ok(CertificateOrder {
            order_url,
            status,
            identifiers: self.config.domains.clone(),
            authorization_urls,
            finalize_url,
            certificate_url: order_data["certificate"].as_str().map(String::from),
        })
    }

    /// Complete HTTP-01 challenges for all authorizations in an order.
    ///
    /// # Errors
    ///
    /// Returns an error if challenge completion fails.
    pub async fn complete_challenges(&self, order: &CertificateOrder) -> Result<(), AcmeError> {
        let account = self.account.read().await;
        let kid = account
            .account_url
            .as_deref()
            .ok_or_else(|| AcmeError::AccountKey("not registered".to_string()))?;
        let thumbprint = account.thumbprint();

        for authz_url in &order.authorization_urls {
            let nonce = self.get_nonce().await?;
            let body = jws::sign_request(
                &account.signing_key(),
                authz_url,
                &nonce,
                &Value::Null,
                Some(kid),
            );

            let resp = self
                .http
                .post(authz_url)
                .header("Content-Type", "application/jose+json")
                .json(&body)
                .send()
                .await?;

            let authz: Value = resp.json().await?;

            let challenges = authz["challenges"]
                .as_array()
                .ok_or_else(|| AcmeError::ChallengeFailed("no challenges array".to_string()))?;

            let http01 = challenges
                .iter()
                .find(|c| c["type"].as_str() == Some("http-01"))
                .ok_or_else(|| {
                    AcmeError::ChallengeFailed("no http-01 challenge offered".to_string())
                })?;

            let token = http01["token"]
                .as_str()
                .ok_or_else(|| AcmeError::ChallengeFailed("no token".to_string()))?;
            let challenge_url = http01["url"]
                .as_str()
                .ok_or_else(|| AcmeError::ChallengeFailed("no challenge url".to_string()))?;

            let challenge_token = ChallengeToken::new(token.to_string(), &thumbprint);
            self.solver.add_challenge(&challenge_token).await;

            let nonce = self.get_nonce().await?;
            let body = jws::sign_request(
                &account.signing_key(),
                challenge_url,
                &nonce,
                &json!({}),
                Some(kid),
            );

            let resp = self
                .http
                .post(challenge_url)
                .header("Content-Type", "application/jose+json")
                .json(&body)
                .send()
                .await?;

            if !resp.status().is_success() {
                let detail = resp.text().await.unwrap_or_default();
                self.solver.remove_challenge(token).await;
                return Err(AcmeError::ChallengeFailed(detail));
            }

            info!(token, "notified ACME server — awaiting validation");
        }

        Ok(())
    }

    /// Get a reference to the HTTP-01 solver (for starting the challenge server).
    #[must_use]
    pub const fn solver(&self) -> &Http01Solver {
        &self.solver
    }

    /// Get a reference to the certificate store.
    #[must_use]
    pub const fn store(&self) -> &CertificateStore {
        &self.store
    }
}
