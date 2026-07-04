// SPDX-License-Identifier: AGPL-3.0-or-later

//! ACME issuance pipeline — CSR generation, order finalization, polling, and cert download.

use crate::error::AcmeError;
use crate::jws;
use crate::order::{CertificateOrder, OrderStatus};
use serde_json::{Value, json};
use std::time::Duration;
use tracing::{debug, info};

use super::AcmeClient;

impl AcmeClient {
    /// Execute the full certificate issuance flow.
    pub(super) async fn issue_certificate(&mut self) -> Result<(), AcmeError> {
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
                self.store.store_cert(domain, &cert_pem, &key_pem).await?;
            }

            info!("certificate issued and stored — ACME flow complete");
        } else if order.is_valid() {
            let cert_pem = self.download_certificate(&order).await?;
            for domain in &self.config.domains {
                self.store
                    .store_cert(domain, &cert_pem, "# key already stored")
                    .await?;
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
    pub(super) async fn poll_order_ready(
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

        let (csr_der, key_pem) = self.build_csr()?;
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

        let final_order = self.poll_order_ready(order).await?;

        let cert_pem = self.download_certificate(&final_order).await?;

        Ok((cert_pem, key_pem))
    }

    /// Build a PKCS#10 CSR (RFC 2986) for the configured domains.
    ///
    /// Returns DER-encoded CSR bytes and the ECDSA P-256 private key as PEM.
    /// Uses pure-Rust `p256` + `x509-cert` — zero C dependencies.
    pub(super) fn build_csr(&self) -> Result<(Vec<u8>, String), AcmeError> {
        use p256::ecdsa::SigningKey;
        use p256::pkcs8::EncodePrivateKey;
        use x509_cert::builder::{Builder, RequestBuilder};
        use x509_cert::der::Encode;
        use x509_cert::name::Name;

        let primary_domain = self
            .config
            .domains
            .first()
            .ok_or_else(|| AcmeError::Config("no domains configured".to_string()))?;

        let signing_key = SigningKey::random(&mut p256::elliptic_curve::rand_core::OsRng);

        let subject: Name = format!("CN={primary_domain}")
            .parse()
            .map_err(|e| AcmeError::CertParse(format!("invalid CN: {e}")))?;

        let builder = RequestBuilder::new(subject, &signing_key)
            .map_err(|e| AcmeError::CertParse(format!("CSR builder: {e}")))?;

        let csr = builder
            .build::<p256::ecdsa::DerSignature>()
            .map_err(|e| AcmeError::CertParse(format!("CSR sign: {e}")))?;

        let csr_der = csr
            .to_der()
            .map_err(|e| AcmeError::CertParse(format!("CSR DER encode: {e}")))?;

        let key_pem = signing_key
            .to_pkcs8_pem(p256::pkcs8::LineEnding::LF)
            .map_err(|e| AcmeError::CertParse(format!("key PEM encode: {e}")))?;

        Ok((csr_der, key_pem.to_string()))
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
    use crate::storage::CertificateStore;

    #[test]
    fn order_status_parsing() {
        let status: OrderStatus = serde_json::from_str(r#""ready""#).expect("parse");
        assert_eq!(status, OrderStatus::Ready);
    }

    #[test]
    fn build_csr_produces_valid_pkcs10() {
        use tempfile::TempDir;
        use x509_parser::certification_request::X509CertificationRequest;
        use x509_parser::prelude::FromDer;

        let data_dir = TempDir::new().expect("tempdir");
        let store = CertificateStore::new(data_dir.path()).expect("store");

        let config = super::super::AcmeConfig {
            directory_url: crate::directories::LETS_ENCRYPT_STAGING.to_string(),
            domains: vec![
                "primary.example.com".to_string(),
                "alt.example.org".to_string(),
            ],
            contacts: vec!["mailto:test@example.com".to_string()],
            challenge_port: 8080,
            renewal_days_before_expiry: 30,
            check_interval: std::time::Duration::from_secs(3600),
        };

        let client = AcmeClient::new_with_store(config, store).expect("client");
        let (csr_der, key_pem) = client.build_csr().expect("csr");

        let (_, csr) = X509CertificationRequest::from_der(&csr_der).expect("parse PKCS#10 CSR");

        let cn: Vec<_> = csr
            .certification_request_info
            .subject
            .iter_common_name()
            .map(|attr| attr.as_str().expect("CN is UTF-8"))
            .collect();
        assert_eq!(cn, vec!["primary.example.com"]);

        assert!(key_pem.starts_with("-----BEGIN PRIVATE KEY-----"));
    }
}
