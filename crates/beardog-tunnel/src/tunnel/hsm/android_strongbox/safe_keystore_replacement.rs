// PHASE 5 OPTIMIZED: Performance patterns applied
// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Safe Android Keystore Replacement
///
/// **DIRECT REPLACEMENT** for unsafe native_keystore_ops.rs
/// This module provides drop-in replacements for all unsafe Android keystore
/// operations using our proven safe architecture patterns.

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
/// **Safe Android Keystore Operations**
/// Direct replacement for unsafe native_keystore_ops.rs with identical interface
/// but using safe Rust patterns for all hardware interactions.
pub struct SafeAndroidKeystoreOps {
    provider: Box<dyn SafeHardwareProvider>,
    buffer_pools: Arc<GlobalBufferPools>,
}
impl SafeAndroidKeystoreOps {
    /// Create new safe Android keystore operations
    pub async fn new() -> BearDogResult<Self> {
        info!("🛡️ Initializing SafeAndroidKeystoreOps - ZERO UNSAFE CODE");
        // Detect best available provider using our safe detection
        let provider = Self::detect_best_provider().await?;
        let buffer_pools = Arc::new(GlobalBufferPools::new());
        Ok(Self {
            provider,
            buffer_pools,
        })
    }
    /// **Safe Provider Detection** - Replaces unsafe capability detection
    async fn detect_best_provider() -> BearDogResult<Box<dyn SafeHardwareProvider>> {
        info!("🔍 Safe hardware detection starting");
        // Try StrongBox first (highest security)
        if let Some(strongbox) =
            SafeMobileHardwareProvider::<StrongBoxAvailable>::detect_strongbox().await?
        {
            info!("✅ StrongBox detected - using highest security level");
            return Ok(strongbox);
        }
        // Fall back to TEE
        if let Some(tee) = SafeMobileHardwareProvider::<TeeAvailable>::detect_tee().await? {
            info!("✅ TEE detected - using hardware security");
            return Ok(tee);
        // Final fallback to software
        info!("ℹ️ Using software fallback - no hardware security available");
        let software =
            SafeMobileHardwareProvider::<SoftwareFallback>::new(SoftwareFallback).await?;
        Ok(software)
    /// **SAFE REPLACEMENT** for unsafe generate_key_with_strongbox
    ///
    /// Replaces the unsafe FFI call in native_keystore_ops.rs:162-278
    pub async fn safe_generate_key_with_strongbox(
        &self,
        key_id: &str,
        key_type: &KeyType,
        strongbox_required: bool,
    ) -> BearDogResult<HsmKey> {
        info!("🔐 Safe Android: Generating StrongBox key: {}", key_id);
        // Safe capability check - no unsafe FFI
        if strongbox_required && !self.provider.supports_strongbox() {
            return Err(BearDogError::Unavailable {
                message: "StrongBox required but not available".to_string(),
            });
        // Safe key generation using provider abstraction
        let key_request = KeyGenerationRequest {
            key_id: key_id.to_string(),
            key_type: key_type.clone(),
            hardware_backed: strongbox_required,
            biometric_required: true,
        };
        let generated_key = self.provider.generate_key(&key_request).await?;
        info!("✅ Safe Android: Key generated successfully");
        Ok(generated_key)
    /// **SAFE REPLACEMENT** for unsafe sign_with_strongbox
    /// Replaces the unsafe FFI call in native_keystore_ops.rs:238-334
    pub async fn safe_sign_with_strongbox(
        data: &[u8],
        algorithm: Algorithm,
    ) -> BearDogResult<Vec<u8>> {
        info!("✍️ Safe Android: Signing with StrongBox key: {}", key_id);
        // Use safe pinned buffer for sensitive data
        let data_buffer = SafePinnedBuffer::from_slice(data)?;
        let signing_request = SigningRequest {
            data: data_buffer,
            algorithm,
        let signature = self.provider.sign(&signing_request).await?;
        info!("✅ Safe Android: Data signed successfully");
        Ok(signature.into_vec())
    /// **SAFE REPLACEMENT** for unsafe verify_with_strongbox
    /// Replaces the unsafe FFI call in native_keystore_ops.rs:391-443
    pub async fn safe_verify_with_strongbox(
        signature: &[u8],
    ) -> BearDogResult<bool> {
        info!("🔍 Safe Android: Verifying with StrongBox key: {}", key_id);
        // Use safe pinned buffers for sensitive data
        let signature_buffer = SafePinnedBuffer::from_slice(signature)?;
        let verification_request = VerificationRequest {
            signature: signature_buffer,
        let is_valid = self.provider.verify(&verification_request).await?;
        info!("✅ Safe Android: Verification completed: {}", is_valid);
        Ok(is_valid)
    /// **SAFE REPLACEMENT** for unsafe delete_key
    /// Replaces unsafe FFI calls with safe provider abstraction
    pub async fn safe_delete_key(&self, key_id: &str) -> BearDogResult<()> {
        info!("🗑️ Safe Android: Deleting key: {}", key_id);
        self.provider.delete_key(key_id).await?;
        info!("✅ Safe Android: Key deleted successfully");
        Ok(())
    /// **SAFE REPLACEMENT** for unsafe key_exists
    pub async fn safe_key_exists(&self, key_id: &str) -> BearDogResult<bool> {
        debug!("🔍 Safe Android: Checking key existence: {}", key_id);
        let exists = self.provider.key_exists(key_id).await?;
        debug!("✅ Safe Android: Key exists check: {}", exists);
        Ok(exists)
    /// **SAFE REPLACEMENT** for unsafe get_key_info
    pub async fn safe_get_key_info(&self, key_id: &str) -> BearDogResult<KeyInfo> {
        debug!("ℹ️ Safe Android: Getting key info: {}", key_id);
        let key_info = self.provider.get_key_info(key_id).await?;
        debug!("✅ Safe Android: Key info retrieved");
        Ok(key_info)
// Safe trait definitions to replace unsafe interfaces
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
/// **Migration Helper Functions**
/// These functions provide backward compatibility while transitioning
/// from unsafe to safe implementations.
/// Create safe keystore operations instance
pub async fn create_safe_android_keystore() -> BearDogResult<SafeAndroidKeystoreOps> {
    SafeAndroidKeystoreOps::new().await
/// **SAFE WRAPPER** for legacy generate_key calls}


pub async fn safe_generate_key_wrapper(
    key_id: &str,
    key_type: &KeyType,
    strongbox_required: bool,
) -> BearDogResult<HsmKey> {
    let keystore = create_safe_android_keystore().await?;
    keystore
        .safe_generate_key_with_strongbox(key_id, key_type, strongbox_required)
        .await
/// **SAFE WRAPPER** for legacy sign_data calls  
pub async fn safe_sign_data_wrapper(
    data: &[u8],
    algorithm: Algorithm,
) -> BearDogResult<Vec<u8>> {
        .safe_sign_with_strongbox(key_id, data, algorithm)
/// **SAFE WRAPPER** for legacy verify_signature calls}


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
        // Test key generation
        let key_type = KeyType::EccP256;
        let key = keystore
            .safe_generate_key_with_strongbox("test_key", &key_type, false)
            .await;
        assert!(key.is_ok());
        // Test signing
        let data = b"test data";
        let signature = keystore
            .safe_sign_data_wrapper("test_key", data, Algorithm::EcdsaSha256)
        assert!(signature.is_ok());
        // Test verification
        let sig_bytes = signature.map_err(|e| {
        let valid = keystore
            .safe_verify_signature_wrapper("test_key", data, &sig_bytes)
        assert!(valid.is_ok());
    async fn test_wrapper_functions() -> beardog_errors::BearDogResult<()> {
        // Test safe wrappers
        let key = safe_generate_key_wrapper("wrapper_test", &key_type, false).await;
        let data = b"wrapper test data";
        let signature = safe_sign_data_wrapper("wrapper_test", data, Algorithm::EcdsaSha256).await;
        let valid = safe_verify_signature_wrapper("wrapper_test", data, &sig_bytes).await;
