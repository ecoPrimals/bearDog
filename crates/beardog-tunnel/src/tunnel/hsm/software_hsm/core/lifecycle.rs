// SPDX-License-Identifier: AGPL-3.0-or-later

//! Initialization, reload, and configuration hooks for [`super::RustSoftwareHsm`].

use super::RustSoftwareHsm;
use crate::tunnel::hsm::HsmConfig;
use beardog_errors::BearDogError;
use beardog_types::hsm::{AuditEvent, CryptoProvider};
use tracing::info;

impl RustSoftwareHsm {
    /// # Errors
    ///
    /// Returns an error if hashing fails.
    /// Initialize the HSM (custom method, not part of trait)
    pub async fn initialize_hsm(&self, _config: HsmConfig) -> Result<(), BearDogError> {
        info!("🔄 Initializing Rust Software HSM");

        self.crypto_provider.initialize().await?;
        self.memory_protector.initialize()?;

        let mut key_store = self.key_store.write().await;
        key_store.initialize()?;

        self.audit_logger
            .log_audit_event(AuditEvent::new("initialize"))
            .await?;

        Ok(())
    }

    /// # Errors
    ///
    /// Returns an error if hashing fails.
    /// Reload HSM configuration (custom method, not part of trait)
    ///
    /// Intentional no-op until hot-reload of software HSM settings is implemented.
    pub fn reload_configuration(&self, _new_config: HsmConfig) -> Result<(), BearDogError> {
        info!("🔧 Reloading HSM configuration");
        Ok(())
    }
}
