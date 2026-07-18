// SPDX-License-Identifier: AGPL-3.0-or-later

//! ACME renewal daemon — periodic certificate expiry checks and automatic re-issuance.

use crate::error::AcmeError;
use tracing::{debug, error, info, warn};

use super::AcmeClient;

impl AcmeClient {
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
            if let Some(cert) = self.store.load_cert(domain).await? {
                if self.needs_renewal(&cert.fullchain_pem) {
                    info!(domain, "certificate needs renewal");
                    self.issue_certificate().await?;
                    self.reload_after_issuance(domain).await;
                } else {
                    debug!(domain, "certificate is current");
                }
            } else {
                info!(domain, "no certificate found — issuing");
                self.issue_certificate().await?;
                self.reload_after_issuance(domain).await;
            }
        }
        Ok(())
    }

    async fn reload_after_issuance(&self, domain: &str) {
        if let Some(ctrl) = &self.reload_controller
            && let Err(e) = ctrl.reload_from_store(&self.store, domain).await
        {
            warn!(domain, error = %e, "hot-reload after renewal failed");
        }
    }

    /// Check if a PEM certificate needs renewal based on its `notAfter` date.
    ///
    /// Returns `true` if the cert expires within `renewal_days_before_expiry`
    /// days, or if parsing fails (renewal as a safety fallback).
    pub fn needs_renewal(&self, pem: &str) -> bool {
        use rustls_pki_types::{CertificateDer, pem::PemObject};
        use x509_parser::prelude::{FromDer, X509Certificate};

        let certs: Vec<CertificateDer<'static>> = CertificateDer::pem_slice_iter(pem.as_bytes())
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
}

#[cfg(test)]
mod tests {
    use crate::storage::CertificateStore;
    use std::time::Duration;

    fn test_config(renewal_days: u32) -> super::super::AcmeConfig {
        super::super::AcmeConfig {
            directory_url: "https://example.com/dir".to_string(),
            domains: vec!["test.example.com".to_string()],
            contacts: vec![],
            challenge_port: 8080,
            renewal_days_before_expiry: renewal_days,
            check_interval: Duration::from_secs(3600),
        }
    }

    /// Build a self-signed P-256 certificate expiring in `valid_days` from now.
    fn generate_cert_pem(valid_days: i64) -> String {
        use p256::ecdsa::SigningKey;
        use p256::elliptic_curve::rand_core::OsRng;
        use p256::pkcs8::EncodePublicKey;
        use x509_cert::builder::{Builder, CertificateBuilder, Profile};
        use x509_cert::der::EncodePem;
        use x509_cert::name::Name;
        use x509_cert::serial_number::SerialNumber;
        use x509_cert::spki::SubjectPublicKeyInfoOwned;
        use x509_cert::time::Validity;

        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = signing_key.verifying_key();

        let serial = SerialNumber::from(42u32);
        let subject: Name = "CN=test".parse().unwrap();

        let now = std::time::SystemTime::now();
        let not_before = x509_cert::time::Time::try_from(now).unwrap();

        let future = if valid_days >= 0 {
            now + Duration::from_secs(valid_days.cast_unsigned() * 86_400)
        } else {
            now - Duration::from_secs(valid_days.unsigned_abs() * 86_400)
        };
        let not_after = x509_cert::time::Time::try_from(future).unwrap();
        let validity = Validity {
            not_before,
            not_after,
        };

        let spki_der = verifying_key.to_public_key_der().unwrap();
        let spki = SubjectPublicKeyInfoOwned::try_from(spki_der.as_bytes()).unwrap();

        let cert =
            CertificateBuilder::new(Profile::Root, serial, validity, subject, spki, &signing_key)
                .unwrap()
                .build::<p256::ecdsa::DerSignature>()
                .unwrap();

        cert.to_pem(x509_cert::der::pem::LineEnding::LF).unwrap()
    }

    fn make_test_client(renewal_days: u32) -> super::super::AcmeClient {
        let dir = tempfile::tempdir().unwrap();
        let store = CertificateStore::new(dir.path()).unwrap();
        super::super::AcmeClient::new_with_store(test_config(renewal_days), store).unwrap()
    }

    #[test]
    fn needs_renewal_returns_false_for_fresh_cert() {
        let client = make_test_client(30);
        let pem = generate_cert_pem(90);
        assert!(!client.needs_renewal(&pem));
    }

    #[test]
    fn needs_renewal_returns_true_for_expiring_cert() {
        let client = make_test_client(30);
        let pem = generate_cert_pem(10);
        assert!(client.needs_renewal(&pem));
    }

    #[test]
    fn needs_renewal_returns_true_for_garbage_pem() {
        let client = make_test_client(30);
        assert!(client.needs_renewal("not a certificate"));
    }

    #[test]
    fn needs_renewal_returns_true_for_empty_pem() {
        let client = make_test_client(30);
        assert!(client.needs_renewal(""));
    }

    #[test]
    fn needs_renewal_just_outside_threshold() {
        let client = make_test_client(30);
        let pem = generate_cert_pem(60);
        assert!(!client.needs_renewal(&pem));
    }
}
