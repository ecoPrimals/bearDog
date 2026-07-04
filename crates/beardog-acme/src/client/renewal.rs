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
    pub(super) fn needs_renewal(&self, pem: &str) -> bool {
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
