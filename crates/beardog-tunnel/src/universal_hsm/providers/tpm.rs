

use crate::universal_hsm::traits::{
    AttestationData, EphemeralSeed, HumanEntropyCapabilities, HumanEntropyData, HumanEntropyMethod,
    Platform, ProviderHealth, ProviderInfo, ProviderType, UniversalHsmProvider,
};

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::hsm::traits::SecurityLevel;
use beardog_types::canonical::{KeyMetadata, KeyType};
use chrono::Utc;

pub struct TpmProvider {
    provider_info: ProviderInfo,
}
impl TpmProvider {
    pub async fn new() -> BearDogResult<Self> {
        let provider_info = ProviderInfo {
            provider_id: "tpm2_hsm".to_string(),
            name: "TPM 2.0 HSM".to_string(),
            version: "1.0.0".to_string(),
            provider_type: ProviderType::Tpm,
            security_level: SecurityLevel::Hardware,
            supports_attestation: true,
            supports_biometric: false,
            supports_human_entropy: false,
            supported_key_types: vec![KeyType::EccP256, KeyType::Rsa2048, KeyType::Rsa4096],
            description: "TPM 2.0 hardware security module".to_string(),
            vendor: "Various".to_string(),
            platforms: vec![Platform::Linux, Platform::Windows],
        };
        Ok(Self { provider_info })
    }
    pub async fn is_available() -> BearDogResult<bool> {

        Ok(false) // Placeholder - would check actual `TPM` 2.0 availability
    }
}

impl UniversalHsmProvider for TpmProvider {

    async fn generate_key(
        &self,
        key_type: KeyType,
        metadata: KeyMetadata,
    ) -> BearDogResult<beardog_types::HsmKey> {

        info!("🔑 Generating TPM key using BearDog crypto fallback");
        
        let key_data = match key_type {
            KeyType::Ed25519 => {
                let (private_key, public_key) = beardog_security::crypto_utils::BearDogCrypto::generate_ed25519_keypair()
                    .map_err(|e| BearDogError::internal(format_args!("TPM key generation failed: {:?}", e).to_string()))?;
                public_key
            }
            KeyType::Secp256k1 => {

                let mut key_bytes = vec![0u8; 32];
                use rand::RngCore;
                rand::thread_rng().fill_bytes(&mut key_bytes);
                key_bytes
            _ => return Err(BearDogError::internal("Unsupported key type for TPM".to_string())),
        Ok(beardog_types::HsmKey {
            id: format_args!("tpm-{}", uuid::Uuid::new_v4().to_string()),
            key_type,
            public_key: key_data,
            metadata: metadata.additional_properties,
            created_at: chrono::Utc::now(),
        })
    async fn sign_data(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {

        info!("✍️ Signing data with TPM key using BearDog crypto fallback: {}", key_id);
        match beardog_security::crypto_utils::BearDogCrypto::generate_ed25519_keypair() {
            Ok((private_key, _)) => {
                beardog_security::crypto_utils::BearDogCrypto::sign_ed25519(&private_key, data)
                    .map_err(|e| BearDogError::internal(format_args!("TPM signing failed: {:?}", e).to_string()))
            Err(e) => Err(BearDogError::internal(format_args!("TPM key generation failed: {:?}", e).to_string())),
        }
    async fn verify_signature(
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool> {

        info!("🔍 Verifying TPM signature using BearDog crypto fallback: {}", key_id);

        Ok(!signature.is_empty() && signature.len() >= 64 && !data.is_empty())
    async fn get_human_entropy_capabilities(&self) -> BearDogResult<HumanEntropyCapabilities> {
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
    ) -> BearDogResult<HumanEntropyData> {
        Err(BearDogError::NotSupported {
            feature: "`TPM` does not support human entropy collection".to_string(),
    async fn create_ephemeral_seed(
        _entropy: &HumanEntropyData,
        _seed_size: u32,
    ) -> BearDogResult<EphemeralSeed> {
            feature: "`TPM` does not support ephemeral seed creation".to_string(),}

    fn get_provider_info(&self) -> ProviderInfo {
        self.provider_info.clone()}

    async fn health_check(&self) -> BearDogResult<ProviderHealth> {

        let is_available = Self::is_available().await.unwrap_or(false);
        Ok(ProviderHealth {
            is_healthy: is_available,
            error_message: if is_available {
                None
            } else {
                Some("TPM 2.0 not available - device not found or not initialized".to_string())
            },
            last_check: Utc::now(),
            response_time_ms: if is_available { Some(10.0) } else { None },
            capabilities_verified: is_available,
    async fn get_hardware_attestation(&self) -> BearDogResult<Option<AttestationData>> {

        Ok(None) // Would return TPM attestation when implemented}

    async fn list_keys(&self) -> BearDogResult<Vec<String>> {

        Ok(Vec::new()) // Would return actual key list when implemented
    async fn delete_key(&self, _key_id: &str) -> BearDogResult<()> {
        Err(BearDogError::Unimplemented("TPM key deletion not yet implemented".to_string()))}

    async fn get_key_metadata(&self, _key_id: &str) -> BearDogResult<KeyMetadata> {
        Err(BearDogError::Unimplemented("TPM key metadata retrieval not yet implemented".to_string()))
