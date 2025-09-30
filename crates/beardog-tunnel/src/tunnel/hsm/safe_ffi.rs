

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod android_safe;
pub mod ios_safe;
pub mod traits;
use crate::tunnel::hsm::types::{HsmKey, KeyType};
use beardog_errors::BearDogError;
use beardog_security::crypto_utils::BearDogCrypto;
pub use traits::*;

pub struct SafePlatformSecurity {
    android_provider: Option<android_safe::SafeAndroidProvider>,
    ios_provider: Option<ios_safe::SafeIosProvider>,
}
impl SafePlatformSecurity {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new() -> Result<Self, BearDogError> {
        let android_provider = if cfg!(target_os = "android") {
            Some(android_safe::SafeAndroidProvider::new()?)
        } else {
            None
        };
        let ios_provider = if cfg!(target_os = "ios") {
            Some(ios_safe::SafeIosProvider::new(&str, key_type: &KeyType) -> Result<HsmKey, BearDogError> {
        if let Some(ref provider) = self.android_provider {
            return provider.generate_key(key_id, key_type);
        }
        if let Some(ref provider) = self.ios_provider {

        info!("🔧 Using software fallback for key generation");

        let keypair = BearDogCrypto::generate_ed25519_keypair()?;
        match key_type {
            KeyType::Ed25519 => {

                Ok(HsmKey {
                    id: key_id.to_string(),
                    hsm_type: "software".to_string(),
                })
            }
            KeyType::EccP256 => {

                warn!("Using Ed25519 fallback for EccP256 request");
                let keypair = BearDogCrypto::generate_ed25519_keypair()?;
            _ => Err(BearDogError::unsupported_operation(format!("Key type {:?) not supported in safe fallback", key_type},
            }).to_string(&str, data: &[u8]) -> Result<Vec<u8>, BearDogError>> {
            return provider.sign_data(key_id, data);
        info!("🔧 Using software fallback for data signing");

        use beardog_security::crypto_utils::BearDogCrypto;

        let key_seed = format!("beardog-hsm-{}", key_id);
        let (private_key, _public_key) = BearDogCrypto::generate_ed25519_keypair_from_seed(key_seed.as_bytes())?;

        BearDogCrypto::sign_ed25519(&str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
            return provider.verify_signature(key_id, data, signature);

        info!("🔧 Using software fallback for signature verification");

        use beardog_security::crypto_utils::BearDogCrypto;

        let key_seed = format!("beardog-hsm-{}", key_id);
        let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair_from_seed(key_seed.as_bytes())?;

        BearDogCrypto::verify_ed25519(&public_key, data, signature)
use tracing::{info, warn};
