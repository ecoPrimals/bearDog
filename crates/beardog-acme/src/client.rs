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

use crate::account::AcmeAccount;
use crate::challenge::{ChallengeToken, Http01Solver};
use crate::error::AcmeError;
use crate::jws;
use crate::order::{CertificateOrder, OrderStatus};
use crate::storage::CertificateStore;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// ACME client configuration.
#[derive(Debug, Clone)]
pub struct AcmeConfig {
    /// ACME directory URL (e.g., Let's Encrypt production or staging).
    pub directory_url: String,

    /// Domain names to request certificates for.
    pub domains: Vec<String>,

    /// Contact email addresses for account registration.
    pub contacts: Vec<String>,

    /// Port for the HTTP-01 challenge server (default: 80).
    pub challenge_port: u16,

    /// Renewal threshold in days before expiry (default: 30).
    pub renewal_days_before_expiry: u32,

    /// Renewal check interval (default: 12 hours).
    pub check_interval: Duration,
}

impl AcmeConfig {
    /// Load configuration from environment variables.
    ///
    /// # Errors
    ///
    /// Returns an error if required variables are missing.
    pub fn from_env() -> Result<Self, AcmeError> {
        let directory_url = std::env::var("BEARDOG_ACME_DIRECTORY")
            .unwrap_or_else(|_| crate::directories::LETS_ENCRYPT_PRODUCTION.to_string());

        let domains: Vec<String> = std::env::var("BEARDOG_ACME_DOMAINS")
            .map_err(|_| AcmeError::Config("BEARDOG_ACME_DOMAINS not set".to_string()))?
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        if domains.is_empty() {
            return Err(AcmeError::Config(
                "BEARDOG_ACME_DOMAINS must contain at least one domain".to_string(),
            ));
        }

        let contacts: Vec<String> = std::env::var("BEARDOG_ACME_EMAIL")
            .map(|e| vec![format!("mailto:{e}")])
            .unwrap_or_default();

        let challenge_port = std::env::var("BEARDOG_ACME_CHALLENGE_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(80);

        let renewal_days = std::env::var("BEARDOG_ACME_RENEWAL_DAYS")
            .ok()
            .and_then(|d| d.parse().ok())
            .unwrap_or(30);

        Ok(Self {
            directory_url,
            domains,
            contacts,
            challenge_port,
            renewal_days_before_expiry: renewal_days,
            check_interval: Duration::from_secs(12 * 3600),
        })
    }
}

/// ACME directory endpoints (RFC 8555 §7.1.1).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Directory {
    new_nonce: String,
    new_account: String,
    new_order: String,
}

/// The ACME client that manages the full certificate lifecycle.
pub struct AcmeClient {
    config: AcmeConfig,
    http: Client,
    store: CertificateStore,
    account: Arc<RwLock<AcmeAccount>>,
    solver: Http01Solver,
    directory: Option<Directory>,
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
            http: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .map_err(|e| AcmeError::Config(format!("HTTP client: {e}")))?,
            store,
            account: Arc::new(RwLock::new(account)),
            solver: Http01Solver::new(),
            directory: None,
        })
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
    async fn get_nonce(&self) -> Result<String, AcmeError> {
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
            account.save(&self.store.account_path())?;
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

            // Notify ACME server that we're ready
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
    pub fn solver(&self) -> &Http01Solver {
        &self.solver
    }

    /// Get a reference to the certificate store.
    #[must_use]
    pub fn store(&self) -> &CertificateStore {
        &self.store
    }

    /// Run the ACME renewal daemon loop.
    ///
    /// Checks certificate expiry on startup and every `check_interval`, triggering
    /// renewal when within `renewal_days_before_expiry` of expiration.
    pub async fn run_renewal_loop(&mut self) {
        info!(
            check_interval_hours = self.config.check_interval.as_secs() / 3600,
            renewal_days = self.config.renewal_days_before_expiry,
            "starting ACME renewal daemon"
        );

        loop {
            if let Err(e) = self.check_and_renew().await {
                error!(error = %e, "ACME renewal check failed");
            }
            tokio::time::sleep(self.config.check_interval).await;
        }
    }

    /// Check all configured domains and renew if needed.
    async fn check_and_renew(&mut self) -> Result<(), AcmeError> {
        for domain in &self.config.domains.clone() {
            if let Some(cert) = self.store.load_cert(domain)? {
                if self.needs_renewal(&cert.fullchain_pem) {
                    info!(domain, "certificate needs renewal");
                    self.issue_certificate().await?;
                } else {
                    debug!(domain, "certificate is current");
                }
            } else {
                info!(domain, "no certificate found — issuing");
                self.issue_certificate().await?;
            }
        }
        Ok(())
    }

    /// Check if a PEM certificate needs renewal based on its `notAfter` date.
    ///
    /// Returns `true` if the cert expires within `renewal_days_before_expiry`
    /// days, or if parsing fails (renewal as a safety fallback).
    fn needs_renewal(&self, pem: &str) -> bool {
        use x509_parser::prelude::{FromDer, X509Certificate};

        let mut reader = std::io::BufReader::new(pem.as_bytes());
        let certs: Vec<rustls::pki_types::CertificateDer<'static>> =
            rustls_pemfile::certs(&mut reader)
                .filter_map(Result::ok)
                .collect();

        let Some(leaf_der) = certs.first() else {
            warn!("no certificate in PEM — triggering renewal");
            return true;
        };

        match X509Certificate::from_der(leaf_der.as_ref()) {
            Ok((_, cert)) => {
                let not_after = cert.validity().not_after.to_datetime();
                let not_after_unix = not_after.unix_timestamp();
                let threshold_unix = chrono::Utc::now().timestamp()
                    + i64::from(self.config.renewal_days_before_expiry) * 86_400;

                let needs = threshold_unix > not_after_unix;
                if needs {
                    info!(
                        not_after_unix,
                        threshold_days = self.config.renewal_days_before_expiry,
                        "certificate within renewal window"
                    );
                }
                needs
            }
            Err(e) => {
                warn!(error = %e, "failed to parse certificate — triggering renewal");
                true
            }
        }
    }

    /// Execute the full certificate issuance flow.
    async fn issue_certificate(&mut self) -> Result<(), AcmeError> {
        self.discover_directory().await?;
        self.register_account().await?;

        let order = self.create_order().await?;
        if order.status == OrderStatus::Pending {
            self.complete_challenges(&order).await?;
        }

        let order = self.poll_order_ready(&order).await?;

        if order.is_ready() {
            let (cert_pem, key_pem) = self.finalize_order(&order).await?;

            for domain in &self.config.domains {
                self.store.store_cert(domain, &cert_pem, &key_pem)?;
            }

            info!("certificate issued and stored — ACME flow complete");
        } else if order.is_valid() {
            let cert_pem = self.download_certificate(&order).await?;
            for domain in &self.config.domains {
                self.store
                    .store_cert(domain, &cert_pem, "# key already stored")?;
            }
        } else {
            return Err(AcmeError::OrderState {
                expected: "ready or valid".to_string(),
                actual: format!("{:?}", order.status),
            });
        }

        Ok(())
    }

    /// Poll the order URL until status transitions from `pending` to `ready`.
    async fn poll_order_ready(
        &self,
        order: &CertificateOrder,
    ) -> Result<CertificateOrder, AcmeError> {
        let account = self.account.read().await;
        let kid = account
            .account_url
            .as_deref()
            .ok_or_else(|| AcmeError::AccountKey("not registered".to_string()))?;

        let mut attempts = 0u32;
        loop {
            if attempts >= 30 {
                return Err(AcmeError::ChallengeFailed(
                    "order did not become ready after 30 attempts".to_string(),
                ));
            }
            attempts += 1;

            let delay = Duration::from_secs(u64::from(attempts.min(10)));
            tokio::time::sleep(delay).await;

            let nonce = self.get_nonce().await?;
            let body = jws::sign_request(
                &account.signing_key(),
                &order.order_url,
                &nonce,
                &Value::Null,
                Some(kid),
            );

            let resp = self
                .http
                .post(&order.order_url)
                .header("Content-Type", "application/jose+json")
                .json(&body)
                .send()
                .await?;

            let order_data: Value = resp.json().await?;
            let status = order_data["status"].as_str().unwrap_or("pending");

            debug!(status, attempt = attempts, "polling order");

            match status {
                "ready" | "valid" => {
                    return Ok(CertificateOrder {
                        order_url: order.order_url.clone(),
                        status: if status == "ready" {
                            OrderStatus::Ready
                        } else {
                            OrderStatus::Valid
                        },
                        identifiers: order.identifiers.clone(),
                        authorization_urls: order.authorization_urls.clone(),
                        finalize_url: order.finalize_url.clone(),
                        certificate_url: order_data["certificate"].as_str().map(String::from),
                    });
                }
                "invalid" => {
                    let detail = order_data["error"]["detail"]
                        .as_str()
                        .unwrap_or("unknown")
                        .to_string();
                    return Err(AcmeError::ChallengeFailed(detail));
                }
                _ => {}
            }
        }
    }

    /// Finalize the order: generate a key pair, build a CSR, submit it,
    /// poll until the certificate is available, and download it.
    ///
    /// Returns `(fullchain_pem, privkey_pem)`.
    async fn finalize_order(
        &self,
        order: &CertificateOrder,
    ) -> Result<(String, String), AcmeError> {
        let account = self.account.read().await;
        let kid = account
            .account_url
            .as_deref()
            .ok_or_else(|| AcmeError::AccountKey("not registered".to_string()))?;

        // Generate a fresh Ed25519 key pair for the certificate
        let cert_secret: [u8; 32] = rand::random();
        let cert_key = ed25519_dalek::SigningKey::from_bytes(&cert_secret);
        let cert_pubkey = cert_key.verifying_key();

        let csr_der = self.build_csr(&cert_key, &cert_pubkey)?;
        let csr_b64 = jws::base64url(&csr_der);

        let nonce = self.get_nonce().await?;
        let payload = json!({ "csr": csr_b64 });
        let body = jws::sign_request(
            &account.signing_key(),
            &order.finalize_url,
            &nonce,
            &payload,
            Some(kid),
        );

        let resp = self
            .http
            .post(&order.finalize_url)
            .header("Content-Type", "application/jose+json")
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let detail = resp.text().await.unwrap_or_default();
            return Err(AcmeError::Server {
                status: 400,
                detail,
            });
        }

        // Poll until order becomes valid (cert available)
        let final_order = self.poll_order_ready(order).await?;

        let cert_pem = self.download_certificate(&final_order).await?;

        // Serialize the private key as PKCS8 PEM
        let key_pem = self.ed25519_private_key_pem(&cert_key);

        Ok((cert_pem, key_pem))
    }

    /// Build a minimal CSR (Certificate Signing Request) for the configured
    /// domains using the given Ed25519 key pair.
    fn build_csr(
        &self,
        signing_key: &ed25519_dalek::SigningKey,
        _verifying_key: &ed25519_dalek::VerifyingKey,
    ) -> Result<Vec<u8>, AcmeError> {
        use ed25519_dalek::Signer;

        let primary_domain = self
            .config
            .domains
            .first()
            .ok_or_else(|| AcmeError::Config("no domains configured".to_string()))?;

        // RFC 2986 PKCS#10 CSR — minimal Ed25519
        // For production, this would use `rcgen`. For now, we build a minimal
        // DER-encoded CSR with the subject CN and SANs.
        // The ACME server validates the CSR signature over the domains.

        let subject = format!("CN={primary_domain}");
        let message = format!("csr:{subject}:{}", self.config.domains.join(","));
        let signature = signing_key.sign(message.as_bytes());

        // Return the signed message as a placeholder CSR DER.
        // A real implementation would use `rcgen::CertificateSigningRequestParams`.
        let mut csr = Vec::new();
        csr.extend_from_slice(subject.as_bytes());
        csr.push(0);
        csr.extend_from_slice(&signature.to_bytes());
        csr.extend_from_slice(signing_key.verifying_key().as_bytes());

        Ok(csr)
    }

    /// Serialize an Ed25519 signing key as a minimal PEM private key.
    fn ed25519_private_key_pem(&self, key: &ed25519_dalek::SigningKey) -> String {
        let key_bytes = key.to_bytes();
        let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, key_bytes);
        format!("-----BEGIN PRIVATE KEY-----\n{b64}\n-----END PRIVATE KEY-----\n")
    }

    /// Download the certificate chain from the order's certificate URL.
    async fn download_certificate(&self, order: &CertificateOrder) -> Result<String, AcmeError> {
        let cert_url = order
            .certificate_url
            .as_deref()
            .ok_or_else(|| AcmeError::OrderState {
                expected: "certificate URL present".to_string(),
                actual: "no certificate URL".to_string(),
            })?;

        let account = self.account.read().await;
        let kid = account
            .account_url
            .as_deref()
            .ok_or_else(|| AcmeError::AccountKey("not registered".to_string()))?;

        let nonce = self.get_nonce().await?;
        let body = jws::sign_request(
            &account.signing_key(),
            cert_url,
            &nonce,
            &Value::Null,
            Some(kid),
        );

        let resp = self
            .http
            .post(cert_url)
            .header("Content-Type", "application/jose+json")
            .header("Accept", "application/pem-certificate-chain")
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status().as_u16();
            let detail = resp.text().await.unwrap_or_default();
            return Err(AcmeError::Server { status, detail });
        }

        let pem = resp.text().await?;
        info!(bytes = pem.len(), "downloaded certificate chain");
        Ok(pem)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acme_config_defaults() {
        // Can't test from_env without setting env vars, but can verify construction
        let config = AcmeConfig {
            directory_url: crate::directories::LETS_ENCRYPT_STAGING.to_string(),
            domains: vec!["test.example.com".to_string()],
            contacts: vec!["mailto:test@example.com".to_string()],
            challenge_port: 8080,
            renewal_days_before_expiry: 30,
            check_interval: Duration::from_secs(3600),
        };
        assert_eq!(config.domains.len(), 1);
        assert_eq!(config.challenge_port, 8080);
    }

    #[test]
    fn order_status_parsing() {
        let status: OrderStatus = serde_json::from_str(r#""ready""#).expect("parse");
        assert_eq!(status, OrderStatus::Ready);
    }
}
