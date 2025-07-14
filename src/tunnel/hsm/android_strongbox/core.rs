//! # Android StrongBox HSM Core Implementation
//!
//! This module contains the main AndroidStrongBoxHsm implementation and the
//! HsmProvider trait implementation for Android StrongBox functionality.

use super::types::*;
use crate::error::{BearDogError, BearDogResult};
use crate::tunnel::hsm::types::*;
use crate::tunnel::hsm::HsmProvider;
use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};

impl AndroidStrongBoxHsm {
    /// Create a new Android StrongBox HSM instance
    ///
    /// This method initializes all components of the Android StrongBox HSM,
    /// including device detection, keystore validation, and health monitoring.
    ///
    /// # Arguments
    /// * `config` - Android HSM configuration
    ///
    /// # Returns
    /// * `Ok(AndroidStrongBoxHsm)` - Successfully initialized HSM
    /// * `Err(BearDogError)` - Initialization failure
    pub async fn new(config: AndroidHsmConfig) -> BearDogResult<Self> {
        info!("🔐 Initializing Android StrongBox HSM");

        // Initialize device information
        let device_info = Arc::new(AndroidDeviceInfo::detect().await?);

        // Verify GrapheneOS compatibility
        if device_info.manufacturer == "Google" && device_info.model.contains("Pixel") {
            info!("📱 Detected Pixel device - excellent StrongBox support");
            if device_info.verified_boot_state == VerifiedBootState::Green {
                info!("✅ Verified boot state: GREEN - optimal security");
            } else {
                warn!(
                    "⚠️ Verified boot state: {:?} - may impact security",
                    device_info.verified_boot_state
                );
            }
        }

        // Initialize keystore
        let keystore = Arc::new(AndroidKeystore::new(config.keystore_config.clone()).await?);

        // Verify StrongBox availability
        if !keystore.is_strongbox_available() {
            return Err(BearDogError::HsmUnavailable {
                hsm_type: "Android StrongBox".to_string(),
                reason: "StrongBox not available on this device".to_string(),
            });
        }

        // Initialize attestation service
        let attestation_service =
            Arc::new(AndroidAttestationService::new(config.attestation_config.clone()).await?);

        // Initialize health monitor
        let health_monitor = Arc::new(AndroidHealthMonitor::new().await?);

        let hsm = Self {
            config,
            keystore,
            attestation_service,
            device_info,
            key_cache: Arc::new(RwLock::new(HashMap::new())),
            health_monitor,
        };

        info!("✅ Android StrongBox HSM initialized successfully");
        debug!(
            "Device: {} {}",
            hsm.device_info.manufacturer, hsm.device_info.model
        );
        debug!("Android version: {}", hsm.device_info.android_version);
        debug!("StrongBox version: {:?}", hsm.device_info.strongbox_version);

        Ok(hsm)
    }

    /// Generate a new key in Android StrongBox
    ///
    /// This method creates a new hardware-backed key in the Android StrongBox,
    /// with optional user presence validation and key attestation.
    ///
    /// # Arguments
    /// * `request` - Key generation request with specifications
    ///
    /// # Returns
    /// * `Ok(HsmKey)` - Successfully generated key
    /// * `Err(BearDogError)` - Key generation failure
    pub async fn generate_strongbox_key(&self, request: &GenerateKeyRequest) -> BearDogResult<HsmKey> {
        info!("🔐 Generating StrongBox key: {}", request.key_id);

        // Configure StrongBox parameters
        let key_params = self.configure_strongbox_parameters(request)?;

        // Generate key in Android Keystore/StrongBox
        let _key_result = self
            .keystore
            .generate_key(&request.key_id, &key_params)
            .await?;

        // Generate attestation if requested
        let attestation = if request.attestation_challenge.is_some() {
            let challenge = request.attestation_challenge.as_ref().unwrap();
            Some(self.generate_key_attestation(&request.key_id, challenge).await?)
        } else {
            None
        };

        // Create HsmKey structure
        let hsm_key = HsmKey {
            id: request.key_id.clone(),
            hsm_type: HsmTier::SmartphoneHsm {
                device_type: SmartphoneType::Android {
                    manufacturer: self.device_info.manufacturer.clone(),
                    model: self.device_info.model.clone(),
                    android_version: self.device_info.android_version.clone(),
                    strongbox_version: self.device_info.strongbox_version.clone(),
                },
                secure_enclave: SecureEnclaveType::AndroidStrongBox {
                    implementation: self.keystore.strongbox_implementation.clone(),
                    hardware_backed: true,
                    key_attestation: true,
                },
                attestation_level: AttestationLevel::Hardware,
                user_presence_required: request.require_user_presence,
            },
            key_type: request.key_type.clone(),
            attestation,
            created_at: Utc::now(),
            metadata: request.metadata.clone(),
        };

        // Cache key information
        self.cache_key_info(&hsm_key).await?;

        info!("✅ StrongBox key generated successfully: {}", request.key_id);
        Ok(hsm_key)
    }

    /// Configure StrongBox parameters for key generation
    ///
    /// Converts a BearDog key generation request into Android-specific parameters.
    fn configure_strongbox_parameters(
        &self,
        request: &GenerateKeyRequest,
    ) -> BearDogResult<AndroidKeyParams> {
        let mut params = AndroidKeyParams::new();

        // Set algorithm and key size
        match &request.key_type {
            KeyType::EccP256 => {
                params.set_algorithm(AndroidKeyAlgorithm::Ec);
                params.set_curve(AndroidEcCurve::P256);
            }
            KeyType::EccP384 => {
                params.set_algorithm(AndroidKeyAlgorithm::Ec);
                params.set_curve(AndroidEcCurve::P384);
            }
            KeyType::EccP521 => {
                params.set_algorithm(AndroidKeyAlgorithm::Ec);
                params.set_curve(AndroidEcCurve::P521);
            }
            KeyType::Rsa { key_size } => {
                params.set_algorithm(AndroidKeyAlgorithm::Rsa);
                params.set_key_size(*key_size);
            }
            KeyType::Aes128 => {
                params.set_algorithm(AndroidKeyAlgorithm::Aes);
                params.set_key_size(128);
            }
            KeyType::Aes192 => {
                params.set_algorithm(AndroidKeyAlgorithm::Aes);
                params.set_key_size(192);
            }
            KeyType::Aes256 => {
                params.set_algorithm(AndroidKeyAlgorithm::Aes);
                params.set_key_size(256);
            }
            KeyType::ChaCha20 => {
                return Err(BearDogError::UnsupportedKeyType {
                    key_type: request.key_type.clone(),
                    hsm_type: "android_strongbox".to_string(),
                });
            }
            KeyType::Ed25519 => {
                return Err(BearDogError::UnsupportedKeyType {
                    key_type: request.key_type.clone(),
                    hsm_type: "android_strongbox".to_string(),
                });
            }
            KeyType::X25519 => {
                return Err(BearDogError::UnsupportedKeyType {
                    key_type: request.key_type.clone(),
                    hsm_type: "android_strongbox".to_string(),
                });
            }
            KeyType::Custom(_) => {
                return Err(BearDogError::UnsupportedKeyType {
                    key_type: request.key_type.clone(),
                    hsm_type: "android_strongbox".to_string(),
                });
            }
        }

        // Set key purposes
        let mut purposes = Vec::new();
        if request.usage_policy.can_encrypt {
            purposes.push(AndroidKeyPurpose::Encrypt);
        }
        if request.usage_policy.can_decrypt {
            purposes.push(AndroidKeyPurpose::Decrypt);
        }
        if request.usage_policy.can_sign {
            purposes.push(AndroidKeyPurpose::Sign);
        }
        if request.usage_policy.can_verify {
            purposes.push(AndroidKeyPurpose::Verify);
        }
        if request.usage_policy.can_wrap {
            purposes.push(AndroidKeyPurpose::Wrap);
        }
        if request.usage_policy.can_unwrap {
            purposes.push(AndroidKeyPurpose::Unwrap);
        }
        params.set_purposes(purposes);

        // Set StrongBox requirement
        params.set_strongbox_required(true);

        // Set user authentication requirement
        params.set_user_authentication_required(request.usage_policy.user_presence_required);

        // Set key validity
        if let Some(expires_at) = request.metadata.expires_at {
            params.set_key_validity_end(expires_at);
        }

        // Set attestation challenge if needed
        if request.attestation_challenge.is_some() {
            let challenge = self.generate_attestation_challenge()?;
            params.set_attestation_challenge(challenge);
        }

        Ok(params)
    }

    /// Generate an attestation challenge
    fn generate_attestation_challenge(&self) -> BearDogResult<Vec<u8>> {
        self.attestation_service
            .challenge_generator
            .generate_challenge(32)
    }

    /// Generate key attestation
    ///
    /// Creates a hardware-backed attestation for the specified key.
    async fn generate_key_attestation(
        &self,
        key_id: &str,
        challenge: &[u8],
    ) -> BearDogResult<KeyAttestation> {
        info!("🔐 Generating key attestation for: {}", key_id);

        // Get certificate chain from keystore
        let certificate_chain = self.keystore.get_certificate_chain(key_id).await?;

        // Verify certificate chain
        let chain_valid = self
            .attestation_service
            .verify_certificate_chain(&certificate_chain, challenge)
            .await?;

        if !chain_valid {
            return Err(BearDogError::VerificationFailed {
                message: "Certificate chain verification failed".to_string(),
            });
        }

        // Create attestation data
        let attestation_data = self
            .attestation_service
            .create_attestation_data(key_id, &self.device_info, challenge)
            .await?;

        // Sign attestation data
        let signature = self
            .keystore
            .sign_attestation_data(key_id, &attestation_data)
            .await?;

        Ok(KeyAttestation {
            attestation_type: AttestationLevel::Hardware,
            certificate_chain,
            attestation_data,
            attestation_signature: signature,
            verified: true,
        })
    }

    /// Cache key information for performance optimization
    async fn cache_key_info(&self, hsm_key: &HsmKey) -> BearDogResult<()> {
        debug!("Caching key info for key: {}", hsm_key.id);
        
        let mut cache = self.key_cache.write().await;
        
        let cache_info = CachedKeyInfo {
            key_id: hsm_key.id.clone(),
            key_type: hsm_key.key_type.clone(),
            hsm_type: hsm_key.hsm_type.clone(),
            strongbox_backed: true,
            attestation_verified: hsm_key.attestation.as_ref().map(|a| a.verified).unwrap_or(false),
            last_used: Utc::now(),
            usage_count: 0,
            created_at: hsm_key.created_at,
            usage_policy: hsm_key.metadata.usage_policy.clone(),
            health_status: KeyHealthStatus::Healthy,
        };
        
        cache.insert(hsm_key.id.clone(), cache_info);
        
        Ok(())
    }

    /// Update key usage statistics
    async fn update_key_usage(&self, key_id: &str) -> BearDogResult<()> {
        let mut cache = self.key_cache.write().await;
        if let Some(cache_info) = cache.get_mut(key_id) {
            cache_info.last_used = Utc::now();
            cache_info.usage_count += 1;
        }
        Ok(())
    }

    /// Validate key exists and is accessible
    async fn validate_key_access(&self, key_id: &str) -> BearDogResult<()> {
        // Check if key exists in cache
        let cache = self.key_cache.read().await;
        if !cache.contains_key(key_id) {
            return Err(BearDogError::KeyNotFound {
                key_id: key_id.to_string(),
            });
        }
        Ok(())
    }
}

#[async_trait]
impl HsmProvider for AndroidStrongBoxHsm {
    async fn initialize(&self, _config: HsmConfig) -> BearDogResult<()> {
        info!("🔐 Initializing Android StrongBox HSM Provider");

        // Test keystore access
        self.keystore.test_keystore_access().await?;

        // Initialize attestation service
        self.attestation_service.initialize().await?;

        // Start health monitoring
        self.health_monitor.start_monitoring().await?;

        info!("✅ Android StrongBox HSM Provider initialized");
        Ok(())
    }

    async fn generate_key(&self, request: GenerateKeyRequest) -> BearDogResult<HsmKey> {
        self.generate_strongbox_key(&request).await
    }

    async fn import_key(&self, _key_data: &[u8], _metadata: KeyMetadata) -> BearDogResult<HsmKey> {
        // StrongBox keys cannot be imported - they must be generated in hardware
        Err(BearDogError::UnsupportedOperation {
            operation: "Key import".to_string(),
            hsm_type: "Android StrongBox".to_string(),
            reason: "StrongBox keys must be generated in hardware".to_string(),
        })
    }

    async fn encrypt(&self, key_id: &str, plaintext: &[u8]) -> BearDogResult<Vec<u8>> {
        self.validate_key_access(key_id).await?;
        self.update_key_usage(key_id).await?;

        info!("🔐 Encrypting with StrongBox key: {}", key_id);
        let ciphertext = self.keystore.encrypt(key_id, plaintext).await?;
        info!("✅ Encryption completed successfully");
        Ok(ciphertext)
    }

    async fn decrypt(&self, key_id: &str, ciphertext: &[u8]) -> BearDogResult<Vec<u8>> {
        self.validate_key_access(key_id).await?;
        self.update_key_usage(key_id).await?;

        info!("🔐 Decrypting with StrongBox key: {}", key_id);
        let plaintext = self.keystore.decrypt(key_id, ciphertext).await?;
        info!("✅ Decryption completed successfully");
        Ok(plaintext)
    }

    async fn sign(&self, key_id: &str, data: &[u8]) -> BearDogResult<Vec<u8>> {
        self.validate_key_access(key_id).await?;
        self.update_key_usage(key_id).await?;

        info!("🔐 Signing with StrongBox key: {}", key_id);
        let signature = self.keystore.sign(key_id, data).await?;
        info!("✅ Signing completed successfully");
        Ok(signature)
    }

    async fn verify(&self, key_id: &str, data: &[u8], signature: &[u8]) -> BearDogResult<bool> {
        self.validate_key_access(key_id).await?;
        self.update_key_usage(key_id).await?;

        info!("🔐 Verifying signature with StrongBox key: {}", key_id);
        let valid = self.keystore.verify(key_id, data, signature).await?;
        info!("✅ Signature verification completed: {}", valid);
        Ok(valid)
    }

    async fn derive_key(
        &self,
        _master_key_id: &str,
        _derivation_data: &[u8],
    ) -> BearDogResult<HsmKey> {
        // Key derivation not directly supported in Android StrongBox
        Err(BearDogError::UnsupportedOperation {
            operation: "Key derivation".to_string(),
            hsm_type: "Android StrongBox".to_string(),
            reason: "Key derivation not supported in StrongBox".to_string(),
        })
    }

    async fn get_info(&self) -> BearDogResult<HsmInfo> {
        debug!("Getting Android StrongBox HSM info");
        
        let device_info = AndroidDeviceInfo::detect().await?;
        
        Ok(HsmInfo {
            hsm_type: HsmTier::SmartphoneHsm {
                device_type: SmartphoneType::Android {
                    manufacturer: device_info.manufacturer.clone(),
                    model: device_info.model.clone(),
                    android_version: device_info.android_version.clone(),
                    strongbox_version: device_info.strongbox_version.clone(),
                },
                secure_enclave: SecureEnclaveType::AndroidStrongBox {
                    implementation: device_info.strongbox_implementation(),
                    hardware_backed: device_info.hardware_backed(),
                    key_attestation: device_info.key_attestation_supported(),
                },
                attestation_level: AttestationLevel::Hardware,
                user_presence_required: false,
            },
            vendor: device_info.manufacturer.clone(),
            model: device_info.model.clone(),
            version: device_info.android_version.clone(),
            capabilities: vec![
                HsmCapability::KeyGeneration,
                HsmCapability::Signing,
                HsmCapability::Verification,
                HsmCapability::Encryption,
                HsmCapability::Decryption,
                HsmCapability::KeyAttestation,
                HsmCapability::TamperDetection,
                HsmCapability::BiometricAuthentication,
                HsmCapability::UserPresenceValidation,
            ],
            supported_algorithms: vec![
                Algorithm::EccP256,
                Algorithm::EccP384,
                Algorithm::EccP521,
                Algorithm::RsaSha256,
                Algorithm::RsaSha384,
                Algorithm::RsaSha512,
                Algorithm::Aes256Gcm,
                Algorithm::EcdsaSha256,
                Algorithm::EcdsaSha384,
                Algorithm::EcdsaSha512,
            ],
            max_key_size: Some(4096),
            certification: Some("StrongBox Keymaster".to_string()),
            tamper_resistance: TamperResistanceLevel::Hardware,
        })
    }

    async fn list_keys(&self) -> BearDogResult<Vec<HsmKeyInfo>> {
        debug!("Listing Android StrongBox keys");
        
        let mut keys = Vec::new();
        
        // Get keys from cache
        let cache = self.key_cache.read().await;
        for (key_id, cache_info) in cache.iter() {
            let key_info = HsmKeyInfo {
                key_id: cache_info.key_id.clone(),
                key_type: cache_info.key_type.clone(),
                hsm_type: cache_info.hsm_type.clone(),
                created_at: cache_info.last_used, // Use last_used as approximation
                usage_policy: cache_info.usage_policy.clone(),
                health_status: cache_info.health_status.clone(),
            };
            keys.push(key_info);
        }
        
        Ok(keys)
    }

    async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        self.validate_key_access(key_id).await?;

        info!("🗑️ Deleting StrongBox key: {}", key_id);
        self.keystore.delete_key(key_id).await?;

        // Remove from cache
        let mut cache = self.key_cache.write().await;
        cache.remove(key_id);

        info!("✅ StrongBox key deleted successfully: {}", key_id);
        Ok(())
    }

    async fn backup(&self) -> BearDogResult<Option<Vec<u8>>> {
        // StrongBox keys cannot be backed up - they are hardware-bound
        warn!("⚠️ StrongBox keys cannot be backed up - they are hardware-bound");
        Ok(None)
    }

    async fn restore(&self, _backup_data: &[u8]) -> BearDogResult<()> {
        // StrongBox keys cannot be restored - they are hardware-bound
        Err(BearDogError::UnsupportedOperation {
            operation: "Key restore".to_string(),
            hsm_type: "Android StrongBox".to_string(),
            reason: "StrongBox keys cannot be restored - they are hardware-bound".to_string(),
        })
    }

    async fn health_check(&self) -> BearDogResult<HsmHealthStatus> {
        self.health_monitor.get_health_status().await
    }
} 