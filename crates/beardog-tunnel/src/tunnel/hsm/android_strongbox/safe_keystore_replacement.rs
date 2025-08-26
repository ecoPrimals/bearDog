

use crate::tunnel::hsm::android_strongbox::safe_android_provider::{
    SafeMobileHardwareProvider, SoftwareFallback, StrongBoxAvailable, TeeAvailable,
};
use crate::tunnel::hsm::types::{HsmKey, KeyType};
use crate::tunnel::hsm::types::Algorithm; // Use Algorithm instead of SigningAlgorithm
use beardog_errors::{BearDogError, BearDogResult};
use beardog_utils::utils::safe_memory_enhanced::{GlobalBufferPools, SafePinnedBuffer};
use chrono::{DateTime, Utc};
use std::sync::Arc;
use tracing::{debug, info};

pub struct SafeAndroidKeystoreOps {
    provider: Box<dyn PlatformProvider>,
    buffer_pools: Arc<GlobalBufferPools>,
}
impl SafeAndroidKeystoreOps {

    pub async fn new() -> BearDogResult<Self> {
        info!("🛡️ Initializing SafeAndroidKeystoreOps - ZERO UNSAFE CODE");

        let provider = Self::detect_best_provider().await?;
        let buffer_pools = Arc::new(GlobalBufferPools::new());
        Ok(Self {
            provider,
            buffer_pools,
        })
    }

    async fn detect_best_provider() -> BearDogResult<Box<dyn PlatformProvider>> {
        info!("🔍 Safe hardware detection starting");

        if let Some(strongbox) =
            SafeMobileHardwareProvider::<StrongBoxAvailable>::detect_strongbox().await?
        {
            info!("✅ StrongBox detected - using highest security level");
            return Ok(strongbox);
        }

        if let Some(tee) = SafeMobileHardwareProvider::<TeeAvailable>::detect_tee().await? {
            info!("✅ TEE detected - using hardware security");
            return Ok(tee);

        info!("ℹ️ Using software fallback - no hardware security available");
        let software =
            SafeMobileHardwareProvider::<SoftwareFallback>::new(SoftwareFallback).await?;
        Ok(software)

    pub async fn safe_generate_key_with_strongbox(
        &self,
        key_id: &str,
        key_type: &KeyType,
        strongbox_required: bool,
    ) -> BearDogResult<HsmKey> {
        info!("🔐 Safe Android: Generating StrongBox key: {}", key_id);

        if strongbox_required && !self.provider.supports_strongbox() {
            return Err(BearDogError::Unavailable {
                message: "StrongBox required but not available".to_string(),
            });

        let key_request = KeyGenerationRequest {
            key_id: key_id.to_string(),
            key_type: key_type.clone(),
            hardware_backed: strongbox_required,
            biometric_required: true,
        };
        let generated_key = self.provider.generate_key(&key_request).await?;
        info!("✅ Safe Android: Key generated successfully");
        Ok(generated_key)

    pub async fn safe_sign_with_strongbox(
        data: &[u8],
        algorithm: Algorithm,
    ) -> BearDogResult<Vec<u8>> {
        info!("✍️ Safe Android: Signing with StrongBox key: {}", key_id);

        let data_buffer = SafePinnedBuffer::from_slice(data)?;
        let signing_request = SigningRequest {
            data: data_buffer,
            algorithm,
        let signature = self.provider.sign(&signing_request).await?;
        info!("✅ Safe Android: Data signed successfully");
        Ok(signature.into_vec())

    pub async fn safe_verify_with_strongbox(
        signature: &[u8],
    ) -> BearDogResult<bool> {
        info!("🔍 Safe Android: Verifying with StrongBox key: {}", key_id);

        let signature_buffer = SafePinnedBuffer::from_slice(signature)?;
        let verification_request = VerificationRequest {
            signature: signature_buffer,
        let is_valid = self.provider.verify(&verification_request).await?;
        info!("✅ Safe Android: Verification completed: {}", is_valid);
        Ok(is_valid)

    pub async fn safe_delete_key(&self, key_id: &str) -> BearDogResult<()> {
        info!("🗑️ Safe Android: Deleting key: {}", key_id);
        self.provider.delete_key(key_id).await?;
        info!("✅ Safe Android: Key deleted successfully");
        Ok(())

    pub async fn safe_key_exists(&self, key_id: &str) -> BearDogResult<bool> {
        debug!("🔍 Safe Android: Checking key existence: {}", key_id);
        let exists = self.provider.key_exists(key_id).await?;
        debug!("✅ Safe Android: Key exists check: {}", exists);
        Ok(exists)

    pub async fn safe_get_key_info(&self, key_id: &str) -> BearDogResult<KeyInfo> {
        debug!("ℹ️ Safe Android: Getting key info: {}", key_id);
        let key_info = self.provider.get_key_info(key_id).await?;
        debug!("✅ Safe Android: Key info retrieved");
        Ok(key_info)

#[deprecated(since = "3.1.0", note = "Use PlatformProvider instead")]
#[deprecated(since = "3.1.0", note = "Use PlatformProvider instead")]
pub trait SafeHardwareProvider: Send + Sync {
    fn supports_strongbox(&self) -> bool;
    async fn generate_key(&self, request: &KeyGenerationRequest) -> BearDogResult<HsmKey>;
    async fn sign(&self, request: &SigningRequest) -> BearDogResult<SafePinnedBuffer>;
    async fn verify(&self, request: &VerificationRequest) -> BearDogResult<bool>;
    async fn delete_key(&self, key_id: &str) -> BearDogResult<()>;
    async fn key_exists(&self, key_id: &str) -> BearDogResult<bool>;
    async fn get_key_info(&self, key_id: &str) -> BearDogResult<KeyInfo>;
#[derive(Debug, Clone)]
pub struct KeyGenerationRequest {
    pub key_id: String,
    pub key_type: KeyType,
    pub hardware_backed: bool,
    pub biometric_required: bool,
#[derive(Debug)]
pub struct SigningRequest {
    pub data: SafePinnedBuffer,
    pub algorithm: Algorithm,
}

pub struct VerificationRequest {
    pub signature: SafePinnedBuffer,
pub struct KeyInfo {
    pub created_at: DateTime<Utc>,
    pub biometric_protected: bool,

pub async fn create_safe_android_keystore() -> BearDogResult<SafeAndroidKeystoreOps> {
    SafeAndroidKeystoreOps::new().await

pub async fn safe_generate_key_wrapper(
    key_id: &str,
    key_type: &KeyType,
    strongbox_required: bool,
) -> BearDogResult<HsmKey> {
    let keystore = create_safe_android_keystore().await?;
    keystore
        .safe_generate_key_with_strongbox(key_id, key_type, strongbox_required)
        .await

pub async fn safe_sign_data_wrapper(
    data: &[u8],
    algorithm: Algorithm,
) -> BearDogResult<Vec<u8>> {
        .safe_sign_with_strongbox(key_id, data, algorithm)

pub async fn safe_verify_signature_wrapper(
    signature: &[u8],
) -> BearDogResult<bool> {
        .safe_verify_with_strongbox(key_id, data, signature, Algorithm::EcdsaSha256)}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_safe_keystore_creation() -> beardog_errors::BearDogResult<()> {
        let keystore = SafeAndroidKeystoreOps::new().await;
        assert!(keystore.is_ok());}

    async fn test_safe_key_operations() -> beardog_errors::BearDogResult<()> {
        let keystore = SafeAndroidKeystoreOps::new().await.map_err(|e| {
            tracing::error!("Operation failed: {e:?}");
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        })?;

        let key_type = KeyType::EccP256;
        let key = keystore
            .safe_generate_key_with_strongbox("test_key", &key_type, false)
            .await;
        assert!(key.is_ok());

        let data = b"test data";
        let signature = keystore
            .safe_sign_data_wrapper("test_key", data, Algorithm::EcdsaSha256)
        assert!(signature.is_ok());

        let sig_bytes = signature.map_err(|e| {
        let valid = keystore
            .safe_verify_signature_wrapper("test_key", data, &sig_bytes)
        assert!(valid.is_ok());
    async fn test_wrapper_functions() -> beardog_errors::BearDogResult<()> {

        let key = safe_generate_key_wrapper("wrapper_test", &key_type, false).await;
        let data = b"wrapper test data";
        let signature = safe_sign_data_wrapper("wrapper_test", data, Algorithm::EcdsaSha256).await;
        let valid = safe_verify_signature_wrapper("wrapper_test", data, &sig_bytes).await;
