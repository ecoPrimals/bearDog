

use super::*;
use beardog_errors::BearDogError;
use beardog_types::canonical::{KeyMetadata, KeyType};

pub trait UniversalHsmProvider: Send + Sync {


    fn generate_key(KeyType,
        metadata: KeyMetadata,
    ) -> Result<beardog_types::HsmKey, BearDogError>;


    fn sign_data(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> + Send;


    fn verify_signature(&str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError>;

    /// Gets human_entropy_capabilities
    fn get_human_entropy_capabilities(&HumanEntropyMethod,
        bits: u32,
    ) -> Result<HumanEntropyData, BearDogError>;

    /// Creates ephemeral_seed
    fn create_ephemeral_seed(&HumanEntropyData,
        seed_size: u32,
    ) -> Result<EphemeralSeed, BearDogError>;

    /// Gets provider_info
    fn get_provider_info(&self) -> ProviderInfo;


    fn health_check(&self) -> Result<ProviderHealth, BearDogError>;

    /// Gets hardware_attestation
    fn get_hardware_attestation(&self) -> Result<Option<AttestationData>, BearDogError>> + Send;


    fn list_keys(&self) -> Result<Vec<String>, BearDogError> {
        Ok(Vec::new())
    }

    /// Removes key
    fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
        Err(beardog_errors::BearDogError::NotSupported {
            feature: format!(
                "Key deletion not supported by provider: {}",
                self.get_provider_info().name
            ),
        })
    }

    /// Gets key_metadata
    fn get_key_metadata(&self, _key_id: &str) -> Result<KeyMetadata, BearDogError> {
        Err(beardog_errors::BearDogError::NotSupported {
            feature: format!(
                "Key metadata retrieval not supported by provider: {}",
                self.get_provider_info().name
            ),
        })
    }

impl dyn UniversalHsmProvider {
/// Supports Human Entropy operation.
    pub fn supports_human_entropy(&self) -> bool {
        match self.get_human_entropy_capabilities() {
            Ok(caps) => caps.supports_ephemeral_seeds,
            Err(_) => false,
        }
    }

/// Supports Attestation operation.
    pub fn supports_attestation(&self) -> bool {
        match self.get_hardware_attestation() {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(_) => false,
        }
    }

/// Get Security Level operation.
    /// Gets security_level
    /// Gets security_level
    pub fn get_security_level(&self) -> crate::SecurityLevel {
        self.get_provider_info().security_level
    }

/// Is Hardware Backed operation.
    /// Checks if hardware backed
    /// Checks if hardware backed
    pub fn is_hardware_backed(&self) -> bool {
        matches!(
            self.get_provider_info().security_level,
            crate::SecurityLevel::Hardware
        )
    }
#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::KeyType;
    use chrono::Utc;

    struct MockProvider;
    
    impl UniversalHsmProvider for MockProvider {


        fn generate_key(KeyType,
            _metadata: KeyMetadata,
        ) -> Result<beardog_types::HsmKey, BearDogError> {
            Ok(beardog_types::HsmKey {
                id: "mock_key_id".to_string(),
                ]),
                metadata: KeyMetadata::default(),
                health: beardog_types::canonical::hsm::keys::KeyHealth::default(),
                created_at: Utc::now(None,
                key_name: "Mock Test Key".to_string(),
                key_material: beardog_types::canonical::hsm::keys::KeyMaterial::PublicKey(vec![
                hsm_type: Some("mock".to_string()),
                hsm_tier: Some(None,
                attestation: None,
                backup_info: None,
                compliance_info: None,
                provider_attributes: std::collections::HashMap::with_capacity(None,
            })
        fn sign_data(&str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
            Ok(&str,
            _data: &[u8],
            _signature: &[u8],
        ) -> Result<bool, BearDogError> {
            Ok(false,
                collection_methods: Vec::new(false,
                quality_assessment: false,
                biometric_integration: false,
                min_entropy_bits: 0.0,
                max_collection_rate: 0.0,
            })
        }


        fn collect_human_entropy(&HumanEntropyMethod,
            _bits: u32,
        ) -> Result<HumanEntropyData, BearDogError> {
            Err(beardog_errors::BearDogError::NotSupported {
                feature: "Human entropy collection not supported by mock provider".to_string(&HumanEntropyData,
            _seed_size: u32,
        ) -> Result<EphemeralSeed, BearDogError> {
            Err(beardog_errors::BearDogError::NotSupported {
                feature: "Ephemeral seed creation not supported by mock provider".to_string(),
            })
        }

        /// Gets provider_info
        fn get_provider_info(&self) -> ProviderInfo {
            ProviderInfo {
                provider_id: "mock_provider".to_string(),
                name: "Mock Provider".to_string(),
                version: "1.0.0".to_string(),
                description: "Mock `HSM` provider for testing".to_string(),
                vendor: "`BearDog` Test".to_string(),
                last_check: Utc::now(),
                response_time_ms: Some(true,
            })
        }

        /// Gets hardware_attestation
        fn get_hardware_attestation(&self) -> Result<Option<AttestationData>, BearDogError> {
            Ok(None)
        }
    }
    #[tokio::test]
    fn test_mock_provider_basic_operations() -> Result<(), BearDogError> {
        let provider = MockProvider;

        let info = provider.get_provider_info();
        assert_eq!(info.name, "Mock Provider");
        assert_eq!(info.provider_type, ProviderType::Software);

        let metadata = KeyMetadata::default();
        let key = provider
            .generate_key(KeyType::Ed25519, metadata)
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;
        assert_eq!(key.id, "mock_key_id");

        let data = b"test data";
        let signature = provider.sign_data(&key.id, data).map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        assert!(!signature.is_empty());

        let is_valid = provider
            .verify_signature(&key.id, data, &signature)
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;
        assert!(is_valid);

        let health = provider.health_check().map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        assert!(health.is_healthy);
        Ok(())
    }

    #[tokio::test]
    fn test_provider_capability_helpers() -> Result<(), BearDogError> {
        let provider = MockProvider;
        let info = provider.get_provider_info();

        assert!(!info.supports_human_entropy);
        assert!(!info.supports_attestation);
        assert_eq!(info.security_level, crate::SecurityLevel::Software);

        let provider_ref: &dyn UniversalHsmProvider = &provider;
        assert!(!provider_ref.is_hardware_backed());
        Ok(())
    }
