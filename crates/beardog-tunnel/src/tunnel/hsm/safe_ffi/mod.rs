

pub mod android_safe;
pub mod ios_safe;
pub mod traits;
use crate::tunnel::hsm::types::{HsmKey, KeyType};
use beardog_errors::{BearDogError, BearDogResult};
use beardog_security::crypto_utils::BearDogCrypto;
pub use traits::*;

pub struct SafePlatformSecurity {
    android_provider: Option<android_safe::SafeAndroidProvider>,
    ios_provider: Option<ios_safe::SafeIosProvider>,
}
impl SafePlatformSecurity {

    pub fn new() -> BearDogResult<Self> {
        let android_provider = if cfg!(target_os = "android") {
            Some(android_safe::SafeAndroidProvider::new()?)
        } else {
            None
        };
        let ios_provider = if cfg!(target_os = "ios") {
            Some(ios_safe::SafeIosProvider::new()?)
        Ok(Self {
            android_provider,
            ios_provider,
        })
    }

    pub async fn generate_key(&self, key_id: &str, key_type: &KeyType) -> BearDogResult<HsmKey> {
        if let Some(ref provider) = self.android_provider {
            return provider.generate_key(key_id, key_type).await;
        }
        if let Some(ref provider) = self.ios_provider {

        info!("🔧 Using software fallback for key generation");

        let keypair = BearDogCrypto::generate_ed25519_keypair()?;
        match key_type {
            KeyType::Ed25519 => {

                Ok(HsmKey {
                    id: key_id.to_string(),
                    hsm_type: "software".to_string(),
                    key_type: crate::tunnel::hsm::types::key::KeyType::Ed25519,
                    metadata: crate::tunnel::hsm::types::key::KeyMetadata::default(),
                    key_material: crate::tunnel::hsm::types::key::KeyMaterial::Encrypted {
                        encrypted_data: keypair.0, // Use first element of tuple as key material
                        encryption_algorithm: "Ed25519".to_string(),
                        kdf_params: None,
                    },
                    hsm_tier: "software".to_string(),
                    health_status: crate::tunnel::hsm::types::key::KeyHealthStatus::Healthy,
                    attestation: None,
                    created_at: chrono::Utc::now(),
                })
            }
            KeyType::EccP256 => {

                warn!("Using Ed25519 fallback for EccP256 request");
                let keypair = BearDogCrypto::generate_ed25519_keypair()?;
            _ => Err(BearDogError::unsupported_operation(format_args!("Key type {:?) not supported in safe fallback", key_type},
            }).to_string(),

    pub async fn sign_data(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
            return provider.sign_data(key_id, data).await;
        info!("🔧 Using software fallback for data signing");

        use beardog_security::crypto_utils::BearDogCrypto;

        let key_seed = format_args!("beardog-hsm-{}", key_id).to_string();
        let (private_key, _public_key) = BearDogCrypto::generate_ed25519_keypair_from_seed(key_seed.as_bytes())?;

        BearDogCrypto::sign_ed25519(&private_key, data)

    pub async fn verify_signature(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> BearDogResult<bool> {
            return provider.verify_signature(key_id, data, signature).await;

        info!("🔧 Using software fallback for signature verification");

        use beardog_security::crypto_utils::BearDogCrypto;

        let key_seed = format_args!("beardog-hsm-{}", key_id).to_string();
        let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair_from_seed(key_seed.as_bytes())?;

        BearDogCrypto::verify_ed25519(&public_key, data, signature)
use tracing::{info, warn};
