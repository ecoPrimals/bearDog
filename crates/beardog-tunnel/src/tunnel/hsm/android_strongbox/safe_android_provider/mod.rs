// SPDX-License-Identifier: AGPL-3.0-or-later

//! Safe Android Provider
//!
//! This module provides safe Android hardware-backed cryptographic operations.

mod detection;
mod factory;

pub use factory::SafeAndroidProviderFactory;

use super::super::SecurityLevel;
use crate::tunnel::hsm::types::{Algorithm, HsmKey, KeyType};
use beardog_errors::BearDogError;
use beardog_utils::utils::safe_memory_enhanced::{GlobalBufferPools, SafePinnedBuffer};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

// Complete types to replace archived safe_keystore_replacement
#[derive(Debug, Clone)]
pub struct KeyGenerationRequest {
    pub key_id: String,            // Unique key identifier
    pub key_size: usize,           // Key size in bits
    pub algorithm: Algorithm,      // Crypto algorithm
    pub hardware_backed: bool,     // Require hardware backing
    pub purposes: Vec<KeyPurpose>, // Intended key usage
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyPurpose {
    Encrypt,
    Decrypt,
    Sign,
    Verify,
    WrapKey,
    DeriveKey,
}

#[derive(Debug, Clone)]
pub struct KeyInfo {
    pub id: String,
    pub key_id: String, // Alias for compatibility
    pub algorithm: Algorithm,
    pub hardware_backed: bool,
}

#[derive(Debug, Clone)]
pub struct SigningRequest {
    pub key_id: String,       // Key to use for signing
    pub data: Vec<u8>,        // Data to sign
    pub algorithm: Algorithm, // Signature algorithm
}

#[derive(Debug, Clone)]
pub struct VerificationRequest {
    pub key_id: String,       // Key to use for verification
    pub data: Vec<u8>,        // Original data
    pub signature: Vec<u8>,   // Signature to verify
    pub algorithm: Algorithm, // Signature algorithm
}

/// Safe hardware provider trait - complete definition
pub trait SafeHardwareProvider: Send + Sync {
    /// Check if StrongBox is available
    fn supports_strongbox(&self) -> bool;

    /// Generate a new key
    fn generate_key(&self, request: &KeyGenerationRequest) -> Result<HsmKey, BearDogError>;

    /// Sign data
    fn sign(&self, request: &SigningRequest) -> Result<SafePinnedBuffer, BearDogError>;

    /// Verify signature
    fn verify(&self, request: &VerificationRequest) -> Result<bool, BearDogError>;

    /// Delete a key
    fn delete_key(&self, key_id: &str) -> Result<(), BearDogError>;

    /// Check if key exists
    fn key_exists(&self, key_id: &str) -> Result<bool, BearDogError>;

    /// Get key information
    fn get_key_info(&self, key_id: &str) -> Result<KeyInfo, BearDogError>;
}

/// Trait for Android security capabilities
pub trait AndroidCapability: Send + Sync + 'static {
    /// Returns the security level provided by this capability
    fn security_level() -> SecurityLevel;

    /// Returns the algorithms supported by this capability
    fn supported_algorithms() -> &'static [Algorithm];

    /// Returns whether this capability is hardware-backed
    fn hardware_backed() -> bool;
}

/// StrongBox security level
pub struct StrongBoxAvailable;

impl AndroidCapability for StrongBoxAvailable {
    fn security_level() -> SecurityLevel {
        SecurityLevel::HardwareSecurityModule
    }

    fn supported_algorithms() -> &'static [Algorithm] {
        &[Algorithm::EccP256, Algorithm::EccP384, Algorithm::Aes256Gcm]
    }

    fn hardware_backed() -> bool {
        true
    }
}

/// TEE (Trusted Execution Environment) security level
pub struct TeeAvailable;

impl AndroidCapability for TeeAvailable {
    fn security_level() -> SecurityLevel {
        SecurityLevel::TrustedExecutionEnvironment
    }

    fn supported_algorithms() -> &'static [Algorithm] {
        &[Algorithm::EccP256, Algorithm::Aes256Gcm]
    }

    fn hardware_backed() -> bool {
        true
    }
}

/// Software fallback (no hardware security)
pub struct SoftwareFallback;

impl AndroidCapability for SoftwareFallback {
    fn security_level() -> SecurityLevel {
        SecurityLevel::Software
    }

    fn supported_algorithms() -> &'static [Algorithm] {
        &[Algorithm::EccP256, Algorithm::Aes256Gcm]
    }

    fn hardware_backed() -> bool {
        false
    }
}

/// Safe mobile hardware provider
pub struct SafeMobileHardwareProvider<C: AndroidCapability> {
    capability: C,
    keystore: SafeAndroidKeystore,
    buffer_pools: Arc<GlobalBufferPools>,
    _marker: PhantomData<C>,
}

impl<C: AndroidCapability> SafeMobileHardwareProvider<C> {
    /// Creates a new instance
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new(capability: C) -> Result<Self, BearDogError> {
        info!(
            "🤖 Creating SafeMobileHardwareProvider with {:?} security level",
            C::security_level()
        );
        let keystore = SafeAndroidKeystore::new()?;
        let buffer_pools = Arc::new(GlobalBufferPools::new());

        Ok(Self {
            capability,
            keystore,
            buffer_pools,
            _marker: PhantomData,
        })
    }

    /// Returns the capability
    pub const fn capability(&self) -> &C {
        &self.capability
    }

    /// Checks if an algorithm is supported
    pub fn supports_algorithm(&self, algorithm: Algorithm) -> bool {
        C::supported_algorithms().contains(&algorithm)
    }

    /// Generates a key safely
    ///
    /// # Errors
    /// Returns an error if key generation fails
    pub fn generate_key_safe(
        &self,
        key_id: &str,
        algorithm: Algorithm,
    ) -> Result<SafeKeyHandle, BearDogError> {
        if !self.supports_algorithm(algorithm) {
            return Err(BearDogError::internal(format!(
                "Algorithm {:?} not supported by {:?} security level",
                algorithm,
                C::security_level()
            )));
        }

        info!(
            "🔑 Generating {:?} key \"{}\" with {:?} security",
            algorithm,
            key_id,
            C::security_level()
        );

        self.keystore
            .generate_key_safe(key_id, algorithm, C::security_level())
    }

    /// Signs data safely
    ///
    /// # Errors
    /// Returns an error if signing fails
    pub fn sign_safe(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        info!(
            "✍️ Safe Android: Signing {} bytes with key \"{}\" using {:?}",
            data.len(),
            key_id,
            C::security_level()
        );

        let signature = self.keystore.sign_data_safe(key_id, data)?;

        Ok(signature)
    }

    /// Verifies a signature safely
    ///
    /// # Errors
    /// Returns an error if verification fails
    pub fn verify_safe(
        &self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> Result<bool, BearDogError> {
        info!(
            "🔍 Safe Android: Verifying signature for key \"{}\" using {:?}",
            key_id,
            C::security_level()
        );

        self.keystore.verify_signature_safe(key_id, data, signature)
    }
}

impl<C: AndroidCapability> SafeHardwareProvider for SafeMobileHardwareProvider<C> {
    fn supports_strongbox(&self) -> bool {
        matches!(
            C::security_level(),
            SecurityLevel::HardwareSecurityModule | SecurityLevel::SecureEnclave
        )
    }

    fn generate_key(&self, request: &KeyGenerationRequest) -> Result<HsmKey, BearDogError> {
        let key_id = &request.key_size.to_string(); // Use key_size as key_id for now
        let algorithm = request.algorithm;
        let safe_handle = self.generate_key_safe(key_id, algorithm)?;

        Ok(HsmKey {
            id: request.key_id.clone(),
            key_type: KeyType::from(request.algorithm),
            key_material: crate::tunnel::hsm::types::KeyMaterial::HardwareReference {
                reference: safe_handle.key_id.clone(),
                hsm_location: "android_strongbox".to_string(),
            },
            metadata: crate::tunnel::hsm::types::KeyMetadata {
                key_id: safe_handle.key_id,
                key_type: KeyType::from(request.algorithm),
                alias: Some(request.key_id.clone()),
                created_at: chrono::Utc::now(),
                expires_at: None,
                tags: HashMap::from([
                    ("hsm_type".to_string(), "android_strongbox".to_string()),
                    (
                        "is_hardware_backed".to_string(),
                        request.hardware_backed.to_string(),
                    ),
                ]),
            },
            hsm_tier: "production".to_string(),
            created_at: chrono::Utc::now(),
            hsm_type: "AndroidStrongBox".to_string(),
            attestation: None,
            health_status: crate::tunnel::hsm::types::KeyHealthStatus::Healthy,
        })
    }

    fn sign(&self, request: &SigningRequest) -> Result<SafePinnedBuffer, BearDogError> {
        let signature = self.sign_safe(&request.key_id, &request.data)?;
        Ok(SafePinnedBuffer::from_vec(signature))
    }

    fn verify(&self, request: &VerificationRequest) -> Result<bool, BearDogError> {
        self.verify_safe(&request.key_id, &request.data, &request.signature)
    }

    fn delete_key(&self, key_id: &str) -> Result<(), BearDogError> {
        info!("🗑️ Safe Android: Deleting key \"{}\"", key_id);
        self.keystore.delete_key_safe(key_id)
    }

    fn key_exists(&self, key_id: &str) -> Result<bool, BearDogError> {
        info!("🔍 Safe Android: Checking if key \"{}\" exists", key_id);
        self.keystore.key_exists_safe(key_id)
    }

    fn get_key_info(&self, key_id: &str) -> Result<KeyInfo, BearDogError> {
        info!("📋 Safe Android: Getting key info for \"{}\"", key_id);

        let algorithm = self.keystore.get_key_algorithm(key_id)?;

        Ok(KeyInfo {
            id: key_id.to_string(),
            key_id: key_id.to_string(),
            algorithm,
            hardware_backed: C::hardware_backed(),
        })
    }
}

/// Safe Android keystore
pub struct SafeAndroidKeystore {
    keys: Arc<RwLock<HashMap<String, SafeKeyMetadata>>>,
}

impl SafeAndroidKeystore {
    /// Creates a new keystore
    ///
    /// # Errors
    /// Returns an error if initialization fails
    pub fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            keys: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Generates a key safely
    ///
    /// # Errors
    /// Returns an error if generation fails
    pub fn generate_key_safe(
        &self,
        key_id: &str,
        algorithm: Algorithm,
        _security_level: SecurityLevel,
    ) -> Result<SafeKeyHandle, BearDogError> {
        debug!("🔑 Generating key {} with {:?}", key_id, algorithm);

        let metadata = SafeKeyMetadata {
            key_type: KeyType::from(algorithm),
            algorithm,
            created_at: std::time::SystemTime::now(),
            usage_count: 0,
        };

        let mut keys = self.keys.blocking_write();
        keys.insert(key_id.to_string(), metadata);

        Ok(SafeKeyHandle::new(key_id, KeyType::from(algorithm)))
    }

    /// Signs data safely
    #[cfg(target_os = "android")]
    pub fn sign_data_safe(&self, key_id: &str, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        debug!("✍️ Safe signing with key: {} (Android StrongBox)", key_id);

        let keys = self.keys.blocking_read();
        let _metadata = keys
            .get(key_id)
            .ok_or_else(|| BearDogError::not_found(format!("Key {key_id} not found")))?;

        warn!(
            "Android StrongBox signing requires JNI Keystore integration; use SoftwareHSM fallback"
        );
        Err(BearDogError::not_yet_available(
            "Android StrongBox signing requires JNI Keystore integration capability",
        ))
    }

    #[cfg(not(target_os = "android"))]
    pub fn sign_data_safe(&self, _key_id: &str, _data: &[u8]) -> Result<Vec<u8>, BearDogError> {
        Err(BearDogError::unsupported_operation(
            "Android StrongBox is only available on Android platform. \
             Use SoftwareHSM or other HSM provider on this platform.",
        ))
    }

    /// Verifies a signature safely
    #[cfg(target_os = "android")]
    pub fn verify_signature_safe(
        &self,
        key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        debug!(
            "🔍 Verifying signature for key: {} (Android StrongBox)",
            key_id
        );

        let keys = self.keys.blocking_read();
        let _metadata = keys
            .get(key_id)
            .ok_or_else(|| BearDogError::not_found(format!("Key {key_id} not found")))?;

        warn!("Android StrongBox verification requires JNI Keystore integration");
        Err(BearDogError::not_yet_available(
            "Android StrongBox signature verification requires JNI Keystore integration capability",
        ))
    }

    #[cfg(not(target_os = "android"))]
    pub fn verify_signature_safe(
        &self,
        _key_id: &str,
        _data: &[u8],
        _signature: &[u8],
    ) -> Result<bool, BearDogError> {
        Err(BearDogError::unsupported_operation(
            "Android StrongBox is only available on Android platform",
        ))
    }

    /// Deletes a key safely
    pub fn delete_key_safe(&self, key_id: &str) -> Result<(), BearDogError> {
        let mut keys = self.keys.blocking_write();
        keys.remove(key_id);
        Ok(())
    }

    /// Checks if a key exists
    pub fn key_exists_safe(&self, key_id: &str) -> Result<bool, BearDogError> {
        let keys = self.keys.blocking_read();
        Ok(keys.contains_key(key_id))
    }

    /// Gets the algorithm for a key
    pub fn get_key_algorithm(&self, key_id: &str) -> Result<Algorithm, BearDogError> {
        let keys = self.keys.blocking_read();
        keys.get(key_id)
            .map(|metadata| metadata.algorithm)
            .ok_or_else(|| BearDogError::not_found(format!("Key {key_id} not found")))
    }
}

/// Safe key metadata
#[derive(Debug, Clone)]
pub struct SafeKeyMetadata {
    key_type: KeyType,
    algorithm: Algorithm,
    created_at: std::time::SystemTime,
    usage_count: u64,
}

/// Safe key handle
#[derive(Debug, Clone)]
pub struct SafeKeyHandle {
    pub key_id: String,
    pub algorithm: KeyType,
}

impl SafeKeyHandle {
    /// Creates a new key handle
    #[must_use]
    pub fn new(key_id: &str, algorithm: KeyType) -> Self {
        Self {
            key_id: key_id.to_string(),
            algorithm,
        }
    }
    /// Returns the key ID
    #[must_use]
    pub fn id(&self) -> &str {
        &self.key_id
    }
    /// Returns the algorithm
    #[must_use]
    pub fn algorithm(&self) -> KeyType {
        self.algorithm.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_safe_android_provider_creation() -> Result<(), BearDogError> {
        let provider = SafeMobileHardwareProvider::<SoftwareFallback>::new(SoftwareFallback)?;
        assert_eq!(SoftwareFallback::security_level(), SecurityLevel::Software);
        assert!(!SoftwareFallback::hardware_backed());
        Ok(())
    }

    #[tokio::test]
    async fn test_algorithm_support_checking() -> Result<(), BearDogError> {
        let provider = SafeMobileHardwareProvider::<SoftwareFallback>::new(SoftwareFallback)?;
        assert!(provider.supports_algorithm(Algorithm::EccP256));
        assert!(provider.supports_algorithm(Algorithm::Aes256Gcm));
        Ok(())
    }
}
