

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use crate::tunnel::hsm::android_strongbox::safe_android_provider::{
    SafeMobileHardwareProvider, SoftwareFallback, StrongBoxAvailable, TeeAvailable,
};
use crate::tunnel::hsm::types::{HsmKey, KeyType};
use crate::tunnel::hsm::types::Algorithm; // Use Algorithm instead of SigningAlgorithm
use beardog_errors::BearDogError;
use beardog_types::canonical::providers_unified::traits::UnifiedProvider as PlatformProvider;
use beardog_utils::utils::safe_memory_enhanced::{GlobalBufferPools, SafePinnedBuffer};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tracing::{debug, info};

pub struct SafeAndroidKeystoreOps {
    provider: Box<dyn PlatformProvider>,
    buffer_pools: Arc<GlobalBufferPools>,
}
impl SafeAndroidKeystoreOps {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new() -> Result<Self, BearDogError> {
        info!("🛡️ Initializing SafeAndroidKeystoreOps - ZERO UNSAFE CODE");

        let provider = Self::detect_best_provider()?;
        let buffer_pools = Arc::new(GlobalBufferPools::new());
        Ok(Self {
            provider,
            buffer_pools,
        })
    }


    fn detect_best_provider() -> Result<Box<dyn PlatformProvider>, BearDogError> {
        info!("🔍 Safe hardware detection starting");

        if let Some(strongbox) =
            SafeMobileHardwareProvider::<StrongBoxAvailable>::detect_strongbox()?
        {
            info!("✅ StrongBox detected - using highest security level");
            return Ok(strongbox);
        }

        if let Some(tee) = SafeMobileHardwareProvider::<TeeAvailable>::detect_tee()? {
            info!("✅ TEE detected - using hardware security");
            return Ok(tee);

        info!("ℹ️ Using software fallback - no hardware security available");
        let software =
            SafeMobileHardwareProvider::<SoftwareFallback>::new(&str,
        key_type: &KeyType,
        strongbox_required: bool,
    ) -> Result<HsmKey, BearDogError> {
        info!("🔐 Safe Android: Generating StrongBox key: {}", key_id);

        if strongbox_required && !self.provider.supports_strongbox() {
            return Err(BearDogError::Unavailable {
                message: "StrongBox required but not available".to_string(),
            });

        let key_request = KeyGenerationRequest {
            key_id: key_id.to_string(),
            key_type: key_type.clone(),
            strongbox_required,
            biometric_required: true,
        };
        let generated_key = self.provider.generate_key(&key_request)?;
        info!("✅ Safe Android: Key generated successfully");
        Ok(&[u8],
        algorithm: Algorithm,
    ) -> Result<Vec<u8>, BearDogError>> {
        info!("✍️ Safe Android: Signing with StrongBox key: {}", key_id);

        let data_buffer = SafePinnedBuffer::from_slice(data_buffer,
            algorithm,
        let signature = self.provider.sign(&signing_request)?;
        info!("✅ Safe Android: Data signed successfully");
        Ok(&[u8],
    ) -> Result<bool, BearDogError> {
        info!("🔍 Safe Android: Verifying with StrongBox key: {}", key_id);

        let signature_buffer = SafePinnedBuffer::from_slice(signature_buffer,
        let is_valid = self.provider.verify(Verification completed: {}", is_valid);
        Ok(is_valid)

/// Safe Delete Key operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn safe_delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        info!("🗑️ Safe Android: Deleting key: {}", key_id);
        self.provider.delete_key(key_id)?;
        info!("✅ Safe Android: Key deleted successfully");
        Ok(())

/// Safe Key Exists operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn safe_key_exists(&self, key_id: &str) -> Result<bool, BearDogError> {
        debug!("🔍 Safe Android: Checking key existence: {}", key_id);
        let exists = self.provider.key_exists(Key exists check: {}", exists);
        Ok(exists)

/// Safe Get Key Info operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn safe_get_key_info(&self, key_id: &str) -> Result<KeyInfo, BearDogError> {
        debug!("ℹ️ Safe Android: Getting key info: {}", key_id);
        let key_info = self.provider.get_key_info(key_id)?;
        debug!("✅ Safe Android: Key info retrieved");
        Ok(key_info)

    /// Gets key_info
    fn get_key_info(&self, key_id: &str) -> Result<KeyInfo, BearDogError>;
#[derive(Debug, Clone)]
    /// The key type value
    pub key_type: KeyType,
    /// Whether hardware_backed is enabled
    pub hardware_backed: bool,
    /// Whether biometric_required is enabled
    pub biometric_required: bool,
#[derive(Debug, Clone)]
    /// The algorithm value
    pub algorithm: Algorithm,
}

pub struct VerificationRequest {
    /// The signature value
    pub signature: SafePinnedBuffer,
pub struct KeyInfo {
    /// The created at value
    pub created_at: DateTime<Utc>,
    /// Whether biometric_protected is enabled
    pub biometric_protected: bool,

/// Create Safe Android Keystore operation.
///
/// # Errors
/// Returns an error if the operation fails.
/// Creates safe_android_keystore
pub async fn create_safe_android_keystore() -> Result<SafeAndroidKeystoreOps, BearDogError> {
    SafeAndroidKeystoreOps::new(&str,
    key_type: &KeyType,
    strongbox_required: bool,
) -> Result<HsmKey, BearDogError> {
    let keystore = create_safe_android_keystore(&[u8],
    algorithm: Algorithm,
) -> Result<Vec<u8>, BearDogError>> {
        .safe_sign_with_strongbox(&[u8],
) -> Result<bool, BearDogError> {
        .safe_verify_with_strongbox(key_id, data, signature, Algorithm::EcdsaSha256)}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    fn test_safe_keystore_creation() -> Result<(), BearDogError> {
        let keystore = SafeAndroidKeystoreOps::new();
        assert!(keystore.is_ok());}


    fn test_safe_key_operations() -> Result<(), BearDogError> {
        let keystore = SafeAndroidKeystoreOps::new().map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;

        let key_type = KeyType::EllipticCurve;
        let key = keystore
            .safe_generate_key_with_strongbox("test_key", &key_type, false)
            ;
        assert!(key.is_ok());

        let data = b"test data";
        let signature = keystore
            .safe_sign_data_wrapper("test_key", data, Algorithm::EcdsaSha256)
        assert!(signature.is_ok());

        let sig_bytes = signature.map_err(|e| {
        let valid = keystore
            .safe_verify_signature_wrapper("test_key", data, &sig_bytes)
        assert!(valid.is_ok());
    fn test_wrapper_functions() -> Result<(), BearDogError> {

        let key = safe_generate_key_wrapper("wrapper_test", &key_type, false);
        let data = b"wrapper test data";
        let signature = safe_sign_data_wrapper("wrapper_test", data, Algorithm::EcdsaSha256);
        let valid = safe_verify_signature_wrapper("wrapper_test", data, &sig_bytes);
