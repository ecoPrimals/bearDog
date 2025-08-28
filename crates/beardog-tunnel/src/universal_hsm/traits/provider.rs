

use super::*;
use beardog_errors::BearDogError;
use beardog_types::canonical::{KeyMetadata, KeyType};

pub trait UniversalHsmProvider: Send + Sync {

    async fn generate_key(
        &self,
        key_type: KeyType,
        metadata: KeyMetadata,
    ) -> Result<beardog_types::HsmKey, BearDogError>;

    async fn sign_data(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>;

    async fn verify_signature(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError>;

    async fn get_human_entropy_capabilities(&self) -> Result<HumanEntropyCapabilities, BearDogError>;

    async fn collect_human_entropy(
        &self,
        method: &HumanEntropyMethod,
        bits: u32,
    ) -> Result<HumanEntropyData, BearDogError>;

    async fn create_ephemeral_seed(
        &self,
        entropy: &HumanEntropyData,
        seed_size: u32,
    ) -> Result<EphemeralSeed, BearDogError>;

    fn get_provider_info(&self) -> ProviderInfo;

    async fn health_check(&self) -> Result<ProviderHealth, BearDogError>;

    async fn get_hardware_attestation(&self) -> Result<Option<AttestationData>, BearDogError>;

    async fn list_keys(&self) -> Result<Vec<String>, BearDogError> {
        Ok(Vec::new())
    }

    async fn delete_key(&self, _key_id: &str) -> Result<(), BearDogError> {
        Err(beardog_errors::BearDogError::NotSupported {
            feature: format!(
                "Key deletion not supported by provider: {}",
                self.get_provider_info().name
            ),
        })
    }

    async fn get_key_metadata(&self, _key_id: &str) -> Result<KeyMetadata, BearDogError> {
        Err(beardog_errors::BearDogError::NotSupported {
            feature: format!(
                "Key metadata retrieval not supported by provider: {}",
                self.get_provider_info().name
            ),
        })
    }

impl dyn UniversalHsmProvider {
    pub async fn supports_human_entropy(&self) -> bool {
        match self.get_human_entropy_capabilities().await {
            Ok(caps) => caps.supports_ephemeral_seeds,
            Err(_) => false,
        }
    }

    pub async fn supports_attestation(&self) -> bool {
        match self.get_hardware_attestation().await {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(_) => false,
        }
    }

    pub fn get_security_level(&self) -> crate::SecurityLevel {
        self.get_provider_info().security_level
    }

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

        async fn generate_key(
            &self,
            _key_type: KeyType,
            _metadata: KeyMetadata,
        ) -> Result<beardog_types::HsmKey, BearDogError> {
            Ok(beardog_types::HsmKey {
                id: "mock_key_id".to_string(),
                key_type: beardog_types::canonical::KeyType::Ed25519,
                material: beardog_types::canonical::hsm::keys::KeyMaterial::PublicKey(vec![
                    1, 2, 3, 4,
                ]),
                metadata: KeyMetadata::default(),
                health: beardog_types::canonical::hsm::keys::KeyHealth::default(),
                created_at: Utc::now(),
                expires_at: None,
                key_name: "Mock Test Key".to_string(),
                last_used: None,
                usage_count: 0,
                key_material: beardog_types::canonical::hsm::keys::KeyMaterial::PublicKey(vec![
                hsm_type: Some("mock".to_string()),
                hsm_tier: Some("test".to_string()),
                health_status: None,
                attestation: None,
                backup_info: None,
                compliance_info: None,
                provider_attributes: std::collections::HashMap::with_capacity(16),
                derivation_path: None,
            })
        async fn sign_data(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
            Ok(vec![1, 2, 3, 4, 5, 6, 7, 8])
        }

        async fn verify_signature(
            &self,
            _key_id: &str,
            _data: &[u8],
            _signature: &[u8],
        ) -> Result<bool, BearDogError> {
            Ok(true)
        }
        async fn get_human_entropy_capabilities(&self) -> Result<HumanEntropyCapabilities, BearDogError> {
            Ok(HumanEntropyCapabilities {
                supports_ephemeral_seeds: false,
                collection_methods: Vec::new(),
                realtime_entropy: false,
                quality_assessment: false,
                biometric_integration: false,
                min_entropy_bits: 0.0,
                max_collection_rate: 0.0,
            })
        }

        async fn collect_human_entropy(
            &self,
            _method: &HumanEntropyMethod,
            _bits: u32,
        ) -> Result<HumanEntropyData, BearDogError> {
            Err(beardog_errors::BearDogError::NotSupported {
                feature: "Human entropy collection not supported by mock provider".to_string(),
            })
        }

        async fn create_ephemeral_seed(
            &self,
            _entropy: &HumanEntropyData,
            _seed_size: u32,
        ) -> Result<EphemeralSeed, BearDogError> {
            Err(beardog_errors::BearDogError::NotSupported {
                feature: "Ephemeral seed creation not supported by mock provider".to_string(),
            })
        }

        fn get_provider_info(&self) -> ProviderInfo {
            ProviderInfo {
                provider_id: "mock_provider".to_string(),
                name: "Mock Provider".to_string(),
                version: "1.0.0".to_string(),
                provider_type: ProviderType::Software,
                security_level: crate::SecurityLevel::Software,
                supports_attestation: false,
                supports_biometric: false,
                supports_human_entropy: false,
                supported_key_types: vec![KeyType::Ed25519],
                description: "Mock `HSM` provider for testing".to_string(),
                vendor: "`BearDog` Test".to_string(),
                platforms: vec![Platform::Linux],
            }
        }

        async fn health_check(&self) -> Result<ProviderHealth, BearDogError> {
            Ok(ProviderHealth {
                is_healthy: true,
                error_message: None,
                last_check: Utc::now(),
                response_time_ms: Some(1.0),
                capabilities_verified: true,
            })
        }

        async fn get_hardware_attestation(&self) -> Result<Option<AttestationData>, BearDogError> {
            Ok(None)
        }
    }
    #[tokio::test]
    async fn test_mock_provider_basic_operations() -> Result<(), BearDogError> {
        let provider = MockProvider;

        let info = provider.get_provider_info();
        assert_eq!(info.name, "Mock Provider");
        assert_eq!(info.provider_type, ProviderType::Software);

        let metadata = KeyMetadata::default();
        let key = provider
            .generate_key(KeyType::Ed25519, metadata)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;
        assert_eq!(key.id, "mock_key_id");

        let data = b"test data";
        let signature = provider.sign_data(&key.id, data).await.map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        assert!(!signature.is_empty());

        let is_valid = provider
            .verify_signature(&key.id, data, &signature)
            .await
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
            })?;
        assert!(is_valid);

        let health = provider.health_check().await.map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;
        assert!(health.is_healthy);
        Ok(())
    }

    #[tokio::test]
    async fn test_provider_capability_helpers() -> Result<(), BearDogError> {
        let provider = MockProvider;
        let info = provider.get_provider_info();

        assert!(!info.supports_human_entropy);
        assert!(!info.supports_attestation);
        assert_eq!(info.security_level, crate::SecurityLevel::Software);

        let provider_ref: &dyn UniversalHsmProvider = &provider;
        assert!(!provider_ref.is_hardware_backed());
        Ok(())
    }
