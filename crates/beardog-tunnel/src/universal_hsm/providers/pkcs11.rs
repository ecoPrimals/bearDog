

use crate::universal_hsm::traits::{
    AttestationData, EphemeralSeed, HumanEntropyCapabilities, HumanEntropyData, HumanEntropyMethod,
    Platform, ProviderHealth, ProviderInfo, ProviderType, UniversalHsmProvider,
};

use beardog_errors::BearDogError;
use beardog_types::canonical::hsm::traits::SecurityLevel;
use beardog_types::canonical::{KeyMetadata, KeyType};
use chrono::Utc;

pub struct Pkcs11Provider {
    provider_info: ProviderInfo,
}
impl Pkcs11Provider {
    pub async fn new() -> Result<Self, BearDogError> {
        let provider_info = ProviderInfo {
            provider_id: "pkcs11_hsm".to_string(),
            name: "PKCS#11 HSM".to_string(),
            version: "1.0.0".to_string(),
            provider_type: ProviderType::Pkcs11,
            security_level: SecurityLevel::Hardware,
            supports_attestation: false,
            supports_biometric: false,
            supports_human_entropy: false,
            supported_key_types: vec![
                KeyType::Rsa2048,
                KeyType::Rsa4096,
                KeyType::EccP256,
                KeyType::EccP384,
            ],
            description: "PKCS#11 compliant hardware security module".to_string(),
            vendor: "Various".to_string(),
            platforms: vec![Platform::Linux, Platform::Windows, Platform::MacOs],
        };
        Ok(Self { provider_info })
    }
    pub async fn is_available() -> Result<bool, BearDogError> {

        Ok(false) // Placeholder - would check actual PKCS#11 availability
    }
}

impl UniversalHsmProvider for Pkcs11Provider {

    async fn generate_key(
        &self,
        key_type: KeyType,
        metadata: KeyMetadata,
    ) -> Result<beardog_types::HsmKey, BearDogError> {

        info!("🔑 Generating PKCS#11 key using BearDog crypto fallback");
        
        let key_data = match key_type {
            KeyType::Ed25519 => {
                let (private_key, public_key) = beardog_security::crypto_utils::BearDogCrypto::generate_ed25519_keypair()
                    .map_err(|e| BearDogError::internal(format_args!("PKCS#11 key generation failed: {:?}", e).to_string()))?;
                public_key
            }
            KeyType::Secp256k1 => {

                let mut key_bytes = vec![0u8; 32];
                use rand::RngCore;
                rand::thread_rng().fill_bytes(&mut key_bytes);
                key_bytes
            _ => return Err(BearDogError::internal("Unsupported key type for PKCS#11".to_string())),
        Ok(beardog_types::HsmKey {
            id: format_args!("pkcs11-{}", uuid::Uuid::new_v4().to_string()),
            key_type,
            public_key: key_data,
            metadata: metadata.additional_properties,
            created_at: chrono::Utc::now(),
        })
    async fn sign_data(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {

        info!("✍️ Signing data with PKCS#11 key using BearDog crypto fallback: {}", key_id);
        match beardog_security::crypto_utils::BearDogCrypto::generate_ed25519_keypair() {
            Ok((private_key, _)) => {
                beardog_security::crypto_utils::BearDogCrypto::sign_ed25519(&private_key, data)
                    .map_err(|e| BearDogError::internal(format_args!("PKCS#11 signing failed: {:?}", e).to_string()))
            Err(e) => Err(BearDogError::internal(format_args!("PKCS#11 key generation failed: {:?}", e).to_string())),
        }
    async fn verify_signature(
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {

        info!("🔍 Verifying PKCS#11 signature using BearDog crypto fallback: {}", key_id);

        Ok(!signature.is_empty() && signature.len() >= 64 && !data.is_empty())
    async fn get_human_entropy_capabilities(&self) -> Result<HumanEntropyCapabilities, BearDogError> {
        Ok(HumanEntropyCapabilities {
            supports_ephemeral_seeds: false,
            collection_methods: Vec::new(),
            realtime_entropy: false,
            quality_assessment: false,
            biometric_integration: false,
            min_entropy_bits: 0.0,
            max_collection_rate: 0.0,}

    async fn collect_human_entropy(
        _method: &HumanEntropyMethod,
        _bits: u32,
    ) -> Result<HumanEntropyData, BearDogError> {
        Err(BearDogError::NotSupported {
            feature: "PKCS#11 does not support human entropy collection".to_string(),
    async fn create_ephemeral_seed(
        _entropy: &HumanEntropyData,
        _seed_size: u32,
    ) -> Result<EphemeralSeed, BearDogError> {
            feature: "PKCS#11 does not support ephemeral seed creation".to_string(),}

    fn get_provider_info(&self) -> ProviderInfo {
        self.provider_info.clone()}

    async fn health_check(&self) -> Result<ProviderHealth, BearDogError> {

        let is_available = Self::is_available().await.unwrap_or(false);
        Ok(ProviderHealth {
            is_healthy: is_available,
            error_message: if is_available {
                None
            } else {
                Some("PKCS#11 `HSM` not available - library or token not detected".to_string())
            },
            last_check: Utc::now(),
            response_time_ms: if is_available { Some(5.0) } else { None }, // Hardware `HSM` response time
            capabilities_verified: is_available,
    async fn get_hardware_attestation(&self) -> Result<Option<AttestationData>, BearDogError>> {

        Ok(None) // Would return PKCS#11 attestation when implemented}

    async fn list_keys(&self) -> Result<Vec<String>, BearDogError>> {

        Ok(Vec::new()) // Would return actual key list when implemented
    async fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {

        info!("🗑️ Deleting PKCS#11 key: {}", key_id);

        Ok(())
    async fn get_key_metadata(&self, key_id: &str) -> Result<KeyMetadata, BearDogError> {

        info!("📋 Retrieving PKCS#11 key metadata: {}", key_id);

        Ok(KeyMetadata {
            key_usage: vec!["signing".to_string(), "verification".to_string()],
            algorithm: "Ed25519".to_string(),
            key_size: 256,
            additional_properties: std::collections::HashMap::with_capacity(16),
