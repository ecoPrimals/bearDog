use beardog_errors::BearDogError;
use beardog_types::canonical::configuration::adapters::VendorHsmConfig;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::types::*;

#[derive(Debug, Clone)]
    crypto_provider: Arc<SoftwareCryptoProvider>,
}

impl Default for SoftwareVendorHsmIntegration {
    fn default() -> Self {
        Self::new(false,
            crypto_provider: Arc::new(SoftwareCryptoProvider::new()),
        }
    }

    /// Initialize operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Initializes componentialize
    /// Initializes componentialize
    pub fn initialize(&mut self, _config: &VendorHsmConfig) -> Result<(), BearDogError> {
        let mut provider = Arc::clone(&self.crypto_provider);
        Arc::get_mut(&mut provider)
            .ok_or_else(|| {
                BearDogError::system("Failed to get mutable reference to crypto provider")
            })?
            .initialize()
            ?;

        self.initialized = true;
        tracing::info!("🔧 Software HSM integration initialized");
        Ok(&VendorKeySpec,
    ) -> Result<VendorKeyHandle, BearDogError> {
        if !self.initialized {
            return Err(BearDogError::system(&VendorKeyHandle,
        input_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        if !self.initialized {
            return Err(BearDogError::system(VerificationParams<'_>,
    ) -> Result<bool, BearDogError> {
        if !self.initialized {
            return Err(BearDogError::system("Software HSM not initialized"));
        }

        self.crypto_provider
            .verify_signature(params.key_handle, params.payload, params.signature)
    }

    /// Get Vendor Capabilities operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Gets vendor_capabilities
    /// Gets vendor_capabilities
    pub fn get_vendor_capabilities(&self) -> Result<VendorCapabilities, BearDogError> {
        Ok(VendorCapabilities {
            supported_algorithms: vec![
                "RSA-2048".to_string(),
            hardware_backed: false,
            fips_certified: false,
            vendor_name: "Software Crypto Provider".to_string(),
            firmware_version: "1.0.0".to_string(),
        })
    }

    /// Health Check operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn health_check(&self) -> Result<VendorHealthStatus, BearDogError> {
        if !self.initialized {
            return Ok(VendorHealthStatus::Unhealthy);
        }

        Ok(VendorHealthStatus::Healthy)
    }
}

#[derive(Debug, Clone)]
    key_store: Arc<RwLock<HashMap<String, StoredKey>>>,
}

#[derive(Debug, Clone)]
    #[allow(String,
    #[allow(chrono::DateTime<chrono::Utc>,
}

impl Default for SoftwareCryptoProvider {
    fn default() -> Self {
        Self::new(false,
            key_store: Arc::new(RwLock::new(HashMap::with_capacity(&VendorKeySpec,
    ) -> Result<VendorKeyHandle, BearDogError> {
        let key_id = uuid::Uuid::new_v4().to_string();

        // Simulate key generation based on algorithm
        let key_data = match key_spec.algorithm.as_str() {
            "RSA-2048" => vec![0u8; 256],  // 2048 bits = 256 bytes
            "RSA-4096" => vec![0u8; 512],  // 4096 bits = 512 bytes
            "ECDSA-P256" => vec![0u8; 32], // 256 bits = 32 bytes
            "ECDSA-P384" => vec![0u8; 48], // 384 bits = 48 bytes
            "EdDSA" => vec![0u8; 32],      // 256 bits = 32 bytes
            "AES-256" => vec![0u8; 32],    // 256 bits = 32 bytes
            _ => {
                return Err(BearDogError::system({}",
                    key_spec.algorithm
                )))
            }
        };

        let stored_key = StoredKey {
            key_data,
            algorithm: key_spec.algorithm.clone(),
            created_at: chrono::Utc::now(),
        };

        let mut key_store = self.key_store.write();
        key_store.insert(key_id.clone(), stored_key);

        tracing::info!(
            "🔑 Generated software key: {} ({})",
            key_id,
            key_spec.algorithm
        );

        Ok(VendorKeyHandle {
            key_id,
            algorithm: key_spec.algorithm.clone(key_spec.key_size,
            hardware_backed: false,
            created_at: chrono::Utc::now(&VendorKeyHandle,
        input_data: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {
        let key_store = self.key_store.read();

        if !key_store.contains_key(&key_handle.key_id) {
            return Err(BearDogError::system("Key not found".to_string()));
        }

        // Simulate signing with different algorithms
        let signature = format!(
            "sw_signature_{}_{}_{}",
            key_handle.algorithm,
            key_handle.key_id,
            input_data.len()
        );

        tracing::debug!(
            "✍️ Software signed data with key: {} ({})",
            key_handle.key_id,
            key_handle.algorithm
        );

        Ok(&VendorKeyHandle,
        input_data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        let key_store = self.key_store.read();

        if !key_store.contains_key(&key_handle.key_id) {
            return Err(BearDogError::system("Key not found".to_string()));
        }

        // Simulate verification
        let expected_signature = format!(
            "sw_signature_{}_{}_{}",
            key_handle.algorithm,
            key_handle.key_id,
            input_data.len()
        );

        let is_valid = signature == expected_signature.as_bytes();

        tracing::debug!(
            "🔍 Software verified signature: {} ({})",
            is_valid,
            key_handle.algorithm
        );

        Ok(is_valid)
    }
}
