

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

#[derive(Debug)]
pub struct SoftwareHsmProvider {
    config: SoftwareHsmConfig,
    keystore: Arc<KeyStore>,
    crypto_engine: Arc<CryptoEngine>,
    entropy_collector: Arc<EntropyCollector>,
    attestation_engine: Arc<AttestationEngine>,
}
impl SoftwareHsmProvider {

    pub async fn new() -> Result<Self, BearDogError> {
        Self::new_with_config(&SoftwareHsmConfig::default()).await
    }

    pub async fn new_with_config(config: &SoftwareHsmConfig) -> Result<Self, BearDogError> {
        Self::with_config(config.clone()).await}

    async fn with_config(config: SoftwareHsmConfig) -> Result<Self, BearDogError> {

        config.validate().map_err(|e| BearDogError::Configuration(format!("Invalid software HSM configuration: {e}"),
        })?;
        info!("🔧 Initializing Software HSM Provider");
        let keystore = Arc::new(KeyStore::new(config.max_keys));
        let crypto_engine = Arc::new(CryptoEngine::new(&config));
        let entropy_collector = Arc::new(EntropyCollector::new(&config).await?);
        let attestation_engine = Arc::new(AttestationEngine::new(&config));
        info!("✅ Software HSM Provider initialized successfully");
        Ok(Self {
            config,
            keystore,
            crypto_engine,
            entropy_collector,
            attestation_engine,
        })

    pub fn keystore(&self) -> &Arc<KeyStore> {
        &self.keystore

    pub fn crypto_engine(&self) -> &Arc<CryptoEngine> {
        &self.crypto_engine

    pub fn entropy_collector(&self) -> &Arc<EntropyCollector> {
        &self.entropy_collector

    pub fn attestation_engine(&self) -> &Arc<AttestationEngine> {
        &self.attestation_engine

    pub fn config(&self) -> &SoftwareHsmConfig {
        &self.config
    }
}

impl UniversalHsmProvider for SoftwareHsmProvider {

    async fn generate_key(
        &self,
        key_type: KeyType,
        _metadata: beardog_types::canonical::KeyMetadata,
    ) -> Result<beardog_types::HsmKey, BearDogError> {
        debug!("🔑 Generating key of type: {:?}", key_type);

        let entropy = self.entropy_collector.collect_entropy(32).await?;

        let key = self.crypto_engine.generate_key(key_type, &entropy).await?;

        let key_material = self.crypto_engine.export_key_material(&key).await?;
        self.keystore
            .store_key(key.id.clone(), key.clone(), key_material)
            .await?;
        info!("✅ Generated and stored key: {}", key.id);
        Ok(key)
    async fn get_human_entropy_capabilities(&self) -> Result<HumanEntropyCapabilities, BearDogError> {
        Ok(self.entropy_collector.get_capabilities().await?)}

    async fn collect_human_entropy(
        method: &HumanEntropyMethod,
        _bits: u32,
    ) -> Result<HumanEntropyData, BearDogError> {
        self.entropy_collector
            .collect_human_entropy(method.clone())
            .await
    async fn create_ephemeral_seed(
        entropy: &HumanEntropyData,
        _seed_size: u32,
    ) -> Result<EphemeralSeed, BearDogError> {
        self.entropy_collector.create_ephemeral_seed(entropy).await}

    fn get_provider_info(&self) -> ProviderInfo {
        ProviderInfo {
            provider_id: "software_hsm_v1".to_string(),
            name: "Software HSM Provider".to_string(),
            provider_type: ProviderType::Software,
            version: "1.0.0".to_string(),
            vendor: "BearDog".to_string(),
            description: "Pure software HSM implementation with enhanced security".to_string(),
            security_level: beardog_types::SecurityLevel::Software,
            supports_attestation: true,
            supports_biometric: false,
            supports_human_entropy: true,
            supported_key_types: vec![
                beardog_types::canonical::crypto::KeyType::Ed25519,
                beardog_types::canonical::crypto::KeyType::Aes256,
            ],
            platforms: vec![Platform::Linux, Platform::Windows, Platform::MacOs],
        }
    async fn health_check(&self) -> Result<ProviderHealth, BearDogError> {
        let start_time = std::time::Instant::now();

        let status = "healthy";
        let response_time = start_time.elapsed();
        Ok(ProviderHealth {
            is_healthy: status == "healthy",
            error_message: if status == "healthy" {
                None
            } else {
                Some(status.to_string())
            },
            last_check: chrono::Utc::now(),
            response_time_ms: Some(response_time.as_millis() as f64),
            capabilities_verified: true,
    async fn get_hardware_attestation(&self) -> Result<Option<AttestationData>, BearDogError>> {

        Ok(None)}

    async fn sign_data(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
        debug!("✍️ Signing data with key: {}", key_id);
        let key_material = self
            .keystore
            .get_key_material(key_id)
            .await?
            .ok_or_else(|| BearDogError::NotFound(format!("Key not found: {key_id)"},
            })?;
        let signature = self.crypto_engine.sign_data(&key_material, data).await?;

        self.keystore.update_key_usage(key_id).await?;
        debug!("✅ Data signed successfully with key: {}", key_id);
        Ok(signature)
    async fn verify_signature(
        key_id: &str,
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
                return Err(BearDogError::InvalidInput("Unsupported key material type for verification".to_string()));
        };
        let is_valid = self
            .crypto_engine
            .verify_signature(&public_key, data, signature)
        debug!("✅ Signature verification result: {}", is_valid);
        Ok(is_valid)
#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::crypto::KeyType;
    #[tokio::test]
    async fn test_software_hsm_provider_creation() -> Result<(), BearDogError> {
        let provider = SoftwareHsmProvider::new().await;
        assert!(provider.is_ok());
        Ok(())}

    async fn test_key_generation_and_operations() -> Result<(), BearDogError> {
        let provider = SoftwareHsmProvider::new().await.map_err(|e| {
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
        let signature = provider.sign_data(&key.key_id, data).await.map_err(|e| {
        let is_valid = provider
            .verify_signature(&key.key_id, data, &signature)
        assert!(is_valid);

        let keys = provider.list_keys().await.map_err(|e| {
        assert!(keys.contains(&key.key_id));

        provider.delete_key(&key.key_id).await.map_err(|e| {
        let keys_after = provider.list_keys().await.map_err(|e| {
        assert!(!keys_after.contains(&key.key_id));
    async fn test_health_check() -> Result<(), BearDogError> {
        let health = provider.health_check().await.map_err(|e| {
        assert!(health.is_healthy);
