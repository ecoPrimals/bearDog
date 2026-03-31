// SPDX-License-Identifier: AGPL-3.0-only

//! Service construction, operation counters, validation, and audit logging.

use super::BearDogCryptoService;
use crate::crypto_service::Result;
use crate::crypto_service::algorithms::discovery;
use crate::crypto_service::types::CryptoServiceConfig;
use std::sync::Arc;
use std::sync::atomic::Ordering;

impl BearDogCryptoService {
    /// Create a new crypto service
    ///
    /// # Errors
    ///
    /// Returns error if service initialization fails
    pub fn new(config: CryptoServiceConfig) -> Result<Self> {
        // Discover available algorithms at startup
        let algorithms = discovery::AlgorithmRegistry::new();

        Ok(Self {
            config: Arc::new(config),
            state: Arc::new(crate::crypto_service::types::CryptoServiceState::new()),
            algorithms,
            public_keys: Arc::new(parking_lot::RwLock::new(std::collections::HashMap::new())),
            rsa_keys: Arc::new(parking_lot::RwLock::new(std::collections::HashMap::new())),
        })
    }

    /// Increment operation counter and return operation ID
    pub(crate) fn next_operation_id(&self) -> u64 {
        self.state.operation_count.fetch_add(1, Ordering::Relaxed)
    }

    /// Validate data size against configured limits
    pub(crate) fn validate_data_size(&self, data: &[u8]) -> Result<()> {
        use beardog_errors::BearDogError;
        if data.len() > self.config.max_data_size {
            return Err(BearDogError::business(format!(
                "Data size {} exceeds maximum {} bytes",
                data.len(),
                self.config.max_data_size
            )));
        }
        Ok(())
    }

    /// Audit log an operation (if enabled)
    pub(crate) fn audit_log(&self, operation: &str, key_id: Option<&str>, success: bool) {
        if self.config.audit_enabled {
            tracing::info!(
                service = %self.config.service_name,
                operation = %operation,
                key_id = ?key_id,
                success = %success,
                "Crypto operation"
            );
        }
    }

    pub(crate) async fn get_capabilities_impl(
        &self,
    ) -> Result<beardog_types::crypto_service::ServiceCapabilities> {
        use beardog_types::crypto_service::ServiceCapabilities;
        let capabilities = self.algorithms.all_capabilities();

        let supported_algorithms: Vec<String> =
            capabilities.iter().map(|c| c.name.clone()).collect();

        let mut features = Vec::new();
        if self.config.hsm_enabled {
            features.push("hsm".to_string());
        }
        if self.config.genetic_enabled {
            features.push("genetic".to_string());
        }
        if self.config.audit_enabled {
            features.push("audit".to_string());
        }
        if capabilities.iter().any(|c| c.hardware_accelerated) {
            features.push("hardware_acceleration".to_string());
        }

        Ok(ServiceCapabilities {
            service_name: self.config.service_name.clone(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            supported_algorithms,
            features,
            max_data_size: self.config.max_data_size,
        })
    }

    pub(crate) async fn get_health_impl(
        &self,
    ) -> Result<beardog_types::crypto_service::HealthStatus> {
        use beardog_types::crypto_service::HealthStatus;
        let uptime = self.state.uptime();
        let operations = self.state.operation_count();

        Ok(HealthStatus {
            healthy: true,
            uptime_seconds: uptime.as_secs(),
            operations_completed: operations,
            hsm_connected: self.config.hsm_enabled,
        })
    }
}
