

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
    /// Whether enable_hardware_backing is enabled
    pub enable_hardware_backing: bool,
    /// Number of key_retention_days
    pub key_retention_days: u32,
}

impl Default for HsmConfig {
    fn default(HsmProviderType::Software,
            enable_hardware_backing: false,
            key_retention_days: 365,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Types of hsm provider
pub enum HsmProviderType {
    /// Represents software variant
    Software,
    /// Represents hardware variant
    Hardware,
    /// Represents cloud variant
    Cloud,
    /// Represents mobile variant
    Mobile,
}

#[derive(Debug, thiserror::Error)]

pub enum HsmError {}

    #[error("HSM operation failed: {0}")]
    #[error("HSM operation failed: {0}")]
    #[error("HSM operation failed: {0}")]
    OperationFailed(String),
    #[error("HSM not available: {0}")]
    NotAvailable(String),
    #[error("Key not found: {0}")]
    KeyNotFound(String),
    #[error("Invalid key type: {0}")]
    InvalidKeyType(String),
}

impl From<HsmError> for BearDogError {}


    fn from(err: HsmError) -> Self {
        BearDogError::security(Uuid,
    /// The key type value
    pub key_type: KeyType,
    /// The algorithm value
    pub algorithm: String,
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
    keys: HashMap<Uuid, HsmKey>,
}

impl SimpleHsmProvider {

/// New operation.
    /// Creates a new instance
    pub fn new(config: HsmConfig) -> Self {
        Self {
            config,
            keys: HashMap::with_capacity(16),
    }

/// Config operation.
    pub fn config(&self) -> &HsmConfig {
        &self.config
    }


/// Key Count operation.
    pub fn key_count(&self) -> usize {
        self.keys.len()
    }

/// Generate Key operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn generate_key(&mut self, key_type: KeyType) -> Result<Uuid, HsmError> {

        if self.config.enable_hardware_backing && !self.is_hardware_available() {
            return Err(HsmError::NotAvailable(
                "Hardware backing required but not available"));
        }

        let key_id = Uuid::new_v4();
        let algorithm = match key_type {
            KeyType::Signing => "Ed25519",
            KeyType::Encryption => "X25519",
            KeyType::Authentication => "Ed25519",
        };

        let key = HsmKey {
            key_id,
            key_type,
            algorithm: algorithm.to_string(),
            created_at: chrono::Utc::now(),
        };

        self.keys.insert(key_id, key);
        Ok(key_id)
    }


/// Get Key operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets key
    /// Gets key
    pub fn get_key(&self, key_id: &Uuid) -> Result<&HsmKey, BearDogError> {
        self.keys.get(key_id)
            .ok_or_else(|| BearDogError::security({}", key_id)))
    }

/// Sign operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn sign(&Uuid, data: &[u8]) -> Result<Vec<u8>, HsmError> {
        let key = self
            .get_key(key_id)
            .map_err(|e| BearDogError::security({}", e)))?;

        if key.key_type != KeyType::Signing {
            return Err(HsmError::InvalidKeyType(format!(
                "Key {:?} is not a signing key",
                key.key_type
            )));
        }

        if data.is_empty() {
            return Err(HsmError::OperationFailed(
                "Cannot sign empty data"));
        }

        Ok(format!("signature_for_{:x}_with_{}", data.len(), key.algorithm).into_bytes())
    }


/// Is Available operation.
    /// Checks if available
    /// Checks if available
    pub fn is_available(&self) -> bool {
        match self.config.provider_type {
            HsmProviderType::Software => true,
            _ => false, // Hardware/Cloud would need actual detection
    }

    /// Checks if hardware available
    fn is_hardware_available(&self) -> bool {

        false
    }


/// Cleanup Expired Keys operation.
    /// Cleans up expired_keys
    /// Cleans up expired_keys
    pub fn cleanup_expired_keys(&mut self) -> usize {
        let retention_duration = chrono::Duration::days(self.config.key_retention_days as i64);
        let cutoff_time = chrono::Utc::now() - retention_duration;

        let initial_count = self.keys.len();
        self.keys.retain(|_, key| key.created_at > cutoff_time);
        initial_count - self.keys.len()
}

#[cfg(test)]
mod tests {
    use super::*;}

    #[tokio::test]
    fn test_hsm_provider_creation() {
        let config = HsmConfig::default();
        let provider = SimpleHsmProvider::new(config);
        assert_eq!(provider.key_count(), 0);
        assert!(provider.is_available());
    }

    #[tokio::test]
    fn test_key_generation() -> Result<(), HsmError> {
        let config = HsmConfig::default();
        let mut provider = SimpleHsmProvider::new(config);

        let key_id = provider.generate_key(KeyType::Signing)?;
        assert_eq!(provider.key_count(), 1);

        let key = provider.get_key(&key_id)
            .map_err(|e| BearDogError::system({}", e)))?;
        assert_eq!(key.key_type, KeyType::Signing);
        assert_eq!(key.algorithm, "Ed25519");
        Ok(())
    }

    #[tokio::test]
    fn test_hardware_backing_requirement(HsmProviderType::Software,
            enable_hardware_backing: true,
            key_retention_days: 365,
        };
        let mut provider = SimpleHsmProvider::new(config);

        let result = provider.generate_key(KeyType::Signing);
        assert!(result.is_err()); // Should fail because hardware backing not available
    }

    #[tokio::test]
    fn test_signing_operation() -> Result<(), HsmError> {
        let config = HsmConfig::default();
        let mut provider = SimpleHsmProvider::new(config);

        let key_id = provider.generate_key(KeyType::Signing)?;
        let signature = provider.sign(&key_id, b"test data")?;

        assert!(!signature.is_empty());
        assert!(String::from_utf8_lossy(&signature).contains("Ed25519"));
        Ok(())
    }

    #[tokio::test]
    fn test_sign_with_wrong_key_type() -> Result<(), HsmError> {
        let config = HsmConfig::default();
        let mut provider = SimpleHsmProvider::new(config);

        let key_id = provider.generate_key(KeyType::Encryption)?;
        let result = provider.sign(&key_id, b"test data");

        assert!(result.is_err()); // Should fail with wrong key type
        Ok(())
    }

    #[tokio::test]
    fn test_sign_empty_data() -> Result<(), HsmError> {
        let config = HsmConfig::default();
        let mut provider = SimpleHsmProvider::new(config);

        let key_id = provider.generate_key(KeyType::Signing)?;
        let result = provider.sign(&key_id, &[]);

        assert!(result.is_err()); // Should fail with empty data
        Ok(())
    }

    #[tokio::test]
    fn test_sign_nonexistent_key() {
        let config = HsmConfig::default();
        let provider = SimpleHsmProvider::new(config);

        let fake_key_id = Uuid::new_v4();
        let result = provider.sign(&fake_key_id, b"test data");

        assert!(result.is_err()); // Should fail with nonexistent key
    }

    #[tokio::test]
    fn test_cleanup_expired_keys(HsmProviderType::Software,
            enable_hardware_backing: false,
            key_retention_days: 0, // Immediate expiry for testing
        };
        let mut provider = SimpleHsmProvider::new(config);

        provider.generate_key(KeyType::Signing)?;
        assert_eq!(provider.key_count(), 1);

        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
        let removed = provider.cleanup_expired_keys();
        assert_eq!(removed, 1);
        assert_eq!(provider.key_count(), 0);
        Ok(())
}
