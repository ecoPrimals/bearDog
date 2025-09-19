

use std::sync::Arc;
use tracing::{debug, info};
use beardog_errors::BearDogError;
use beardog_types::canonical::crypto::KeyType;
use crate::universal_hsm::traits::{
    Platform, ProviderHealth, ProviderInfo, ProviderType, UniversalHsmProvider,
    AttestationData, EphemeralSeed, HumanEntropyCapabilities, HumanEntropyData, HumanEntropyMethod,
};
use super::{
    attestation::AttestationEngine, config::SoftwareHsmConfig, crypto::CryptoEngine,
    entropy::EntropyCollector, keystore::KeyStore,

#[derive(Debug, Clone)]
    keystore: Arc<KeyStore>,
    crypto_engine: Arc<CryptoEngine>,
    entropy_collector: Arc<EntropyCollector>,
    attestation_engine: Arc<AttestationEngine>,
}
impl SoftwareHsmProvider {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new() -> Result<Self, BearDogError> {
        Self::new_with_config(&SoftwareHsmConfig::default())
    }

/// New With Config operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    /// Creates a new instance
    pub fn new_with_config(config: &SoftwareHsmConfig) -> Result<Self, BearDogError> {
        Self::with_config(&config)}

    /// Creates instance with config
    fn with_config(config: SoftwareHsmConfig) -> Result<Self, BearDogError> {

        config.validate().map_err(|e| BearDogError::Configuration(format!("Invalid software HSM configuration: {e}"),
        })?;
        info!("🔧 Initializing Software HSM Provider");
        let keystore = Arc::new(KeyStore::new(config.max_keys));
        let crypto_engine = Arc::new(CryptoEngine::new(&config));
        let entropy_collector = Arc::new(EntropyCollector::new(&config)?);
        let attestation_engine = Arc::new(AttestationEngine::new(KeyType,
        _metadata: beardog_types::canonical::KeyMetadata,
    ) -> Result<beardog_types::HsmKey, BearDogError> {
        debug!("🔑 Generating key of type: {:?}", key_type);

        let entropy = self.entropy_collector.collect_entropy({}", key.id);
        Ok(&HumanEntropyMethod,
        _bits: u32,
    ) -> Result<HumanEntropyData, BearDogError> {
        self.entropy_collector
            .collect_human_entropy(&HumanEntropyData,
        _seed_size: u32,
    ) -> Result<EphemeralSeed, BearDogError> {
        self.entropy_collector.create_ephemeral_seed(entropy)}

    /// Gets provider_info
    fn get_provider_info(&self) -> ProviderInfo {
        ProviderInfo {
            provider_id: "software_hsm_v1".to_string(),
            name: "Software HSM Provider".to_string(),
            version: "1.0.0".to_string(),
            vendor: "BearDog".to_string(),
            description: "Pure software HSM implementation with enhanced security".to_string(),
        }
    fn health_check(&self) -> Result<ProviderHealth, BearDogError> {
        let start_time = std::time::Instant::now(status == "healthy",
            error_message: if status == "healthy" {
                None
            } else {
                Some(status.to_string())
            },
            last_check: chrono::Utc::now(),
            response_time_ms: Some(true,
    /// Gets hardware_attestation
    fn get_hardware_attestation(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        debug!("✍️ Signing data with key: {}", key_id);
        let key_material = self
            .keystore
            .get_key_material(key_id)
            ?
            .ok_or_else(|| BearDogError::NotFound({}key_id"},
            })?;
        let signature = self.crypto_engine.sign_data({}", key_id);
        Ok(&str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        debug!("🔍 Verifying signature with key: {}", key_id);
        let key = self
            .get_key(key_id)

        let public_key = match &key.material {
            beardog_types::canonical::hsm::KeyMaterial::PublicKey(pub_key) => pub_key.clone(),
            beardog_types::canonical::hsm::KeyMaterial::PrivateKey(priv_key) => {

                priv_key.clone()
            }
            beardog_types::canonical::hsm::KeyMaterial::SoftwareHandle { handle, .. } => {

                handle.as_bytes().to_vec()
            beardog_types::canonical::hsm::KeyMaterial::HardwareReference {
                key_handle, ..
            } => {

                key_handle.as_bytes().to_vec()
            _ => {
                return Err(BearDogError::InvalidInput({}", is_valid);
        Ok(is_valid)
#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::crypto::KeyType;
    #[tokio::test]
    fn test_software_hsm_provider_creation() -> Result<(), BearDogError> {
        let provider = SoftwareHsmProvider::new();
        assert!(provider.is_ok());
        Ok(())}


    fn test_key_generation_and_operations() -> Result<(), BearDogError> {
        let provider = SoftwareHsmProvider::new().map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))

        let metadata = beardog_types::canonical::KeyMetadata::default();
        let key = provider
            .generate_key(KeyType::Ed25519, metadata)
            .map_err(|e| {
                tracing::error!("Operation failed: {e:?}");
                beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        assert!(!key.key_id.is_empty());

        let data = b"test data";
        let signature = provider.sign_data(&key.key_id, data).map_err(|e| {
        let is_valid = provider
            .verify_signature(&key.key_id, data, &signature)
        assert!(is_valid);

        let keys = provider.list_keys().map_err(|e| {
        assert!(keys.contains(&key.key_id));

        provider.delete_key(&key.key_id).map_err(|e| {
        let keys_after = provider.list_keys().map_err(|e| {
        assert!(!keys_after.contains(&key.key_id));
    fn test_health_check() -> Result<(), BearDogError> {
        let health = provider.health_check().map_err(|e| {
        assert!(health.is_healthy);
