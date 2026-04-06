// SPDX-License-Identifier: AGPL-3.0-or-later



use crate::tunnel::config::UnifiedProcessorConfig;
use crate::tunnel::hsm::{PerformanceRequirements, SecurityRequirements};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

#[derive(Debug, Clone)]
    /// The created at value
    pub created_at: SystemTime,

    /// The expires at value
    pub expires_at: SystemTime,

    /// The algorithm value
    pub algorithm: CryptoAlgorithm,


    pub key_id: String,
}

#[derive(Debug, Clone)]
    keys: Arc<RwLock<HashMap<String, CryptoKey>>>,
    rotation_handles: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
}

impl BStpKeyManager {

/// Get Rotation Handle Count operation.
    /// Gets rotation_handle_count
    pub fn get_rotation_handle_count(&self) -> usize {
        let handles = self.rotation_handles.read();
        handles.len()
    }

/// Stop All Rotations operation.
    /// Stops all_rotations
    pub fn stop_all_rotations(&self) {
        let mut handles = self.rotation_handles.write();
        for handle in handles.drain(..) {
            handle.abort();
        }
    }

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(config: UnifiedProcessorConfig) -> Result<Self, BearDogError> {
        info!("🔑 Initializing BearDog Key Manager");

        if config.session_key_length < 16 {
            return Err(BearDogError::config("Key length must be at least 16 bytes"));
        }
        if config.key_derivation_rounds < 1000 {
            warn!("⚠️ Low key derivation rounds may impact security");
        }
        let key_manager = Self {
            config,
            keys: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            rotation_handles: Arc::new(RwLock::new(Vec::new({} bytes, Rotation interval: {:?}",
            key_manager.config.session_key_length, key_manager.config.key_rotation_interval
        );
        Ok(&str,
        algorithm: CryptoAlgorithm,
    ) -> Result<CryptoKey, BearDogError> {
        info!("🔐 Generating session key for: {}", session_id);

        let key_material = self.generate_secure_key_material(key_material,
            created_at: SystemTime::now(),
            expires_at: SystemTime::now() + self.config.key_rotation_interval,
            algorithm: algorithm.clone(format!("{}_{}", session_id, uuid::Uuid::new_v4()),
        };

        {
            let mut keys = self.keys.write();
            keys.insert(key.key_id, key.clone());
        }
        
        info!("✅ Session key generated: {} (algorithm: {:?})", key.key_id, algorithm);
        Ok(&str,
        new_algorithm: Option<CryptoAlgorithm>,
    ) -> Result<CryptoKey, BearDogError> {
        info!("🔄 Rotating session key for: {}", session_id);

        let algorithm = if let Some(algo) = new_algorithm {
            algo
        } else {
            let keys = self.keys.read();
            keys.values()
                .find(|k| k.key_id.starts_with(session_id))
                .map(&|k| k.algorithm)
                .unwrap_or(CryptoAlgorithm::Aes256Gcm)
        };

        let new_key = self.generate_session_key({}", new_key.key_id);
        Ok(new_key)
    }

/// Get Key operation.
    /// Gets key
    pub fn get_key(&self, key_id: &str) -> Option<CryptoKey> {
        let keys = self.keys.read();
        keys.get(key_id).cloned()
    }

/// Get Session Key operation.
    /// Gets session_key
    pub fn get_session_key(&self, session_id: &str) -> Option<CryptoKey> {
        let keys = self.keys.read(&CryptoKey,
        context: &[u8],
    ) -> Result<Vec<u8>, BearDogError> {

        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&base_key.key);
        hasher.update(context);
        hasher.update(b"beardog_key_derivation");
        let derived_hash = hasher.finalize();
        Ok(derived_hash.to_vec())
    }

/// Cleanup Expired Keys operation.
    /// Cleans up expired_keys
    pub fn cleanup_expired_keys(&self) -> usize {
        let now = SystemTime::now(CryptoAlgorithm,
    ) -> Result<Vec<u8>, BearDogError> {
        let key_length = match algorithm {
            CryptoAlgorithm::Aes256Gcm | CryptoAlgorithm::ChaCha20Poly1305 => 32, // 256 bits
            CryptoAlgorithm::GeneticHybrid => self.config.session_key_length,
        };

        if self.config.use_hardware_keys {
            if let Ok(key_material) = self.generate_hardware_key(key_length) {
                debug!("🔧 Generated hardware-backed key material");
                return Ok(key_material);
            } else {
                warn!("⚠️ Hardware key generation failed, falling back to software");
            }
        }

        use rand::RngCore;
        let mut key_material = vec![0u8; key_length];
        rand::rng().fill_bytes(&mut key_material);
        debug!("🔐 Generated software-based key material");
        Ok(key_material)
    }


    fn generate_hardware_key(&self, length: usize) -> Result<Vec<u8>, BearDogError> {
        debug!("🔧 Generating hardware key material using HSM");

        let security_requirements = SecurityRequirements {
            security_level: crate::tunnel::hsm::SecurityLevel::Medium,
            hardware_backed_required: true,
            user_interaction_required: false,
            attestation_required: false,
            compliance_requirements: vec![],
            performance_requirements: PerformanceRequirements {
                max_latency_ms: None,
                min_throughput_ops_per_sec: None,
                cost_optimization: false,
            },
        };

        let key_type = match length {
            16 => crate::tunnel::hsm::types::KeyType::Aes128,
            24 => crate::tunnel::hsm::types::KeyType::Aes192,
            32 => crate::tunnel::hsm::types::KeyType::Aes256,
            _ => crate::tunnel::hsm::types::KeyType::Aes256, // Default to AES-256
        };

        match self.get_hsm_manager() {
            Ok(hsm_manager) => {
                debug!("🔐 Using HSM manager for hardware key generation");

                let request = crate::tunnel::hsm::GenerateKeyRequest {
                    key_id: format!("session_key_{}", uuid::Uuid::new_v4()),
                    key_type: key_type.clone(crate::tunnel::hsm::KeyUsagePolicy {
                        can_encrypt: true,
                        can_decrypt: true,
                        can_sign: false,
                        can_verify: false,
                        can_wrap: false,
                        can_unwrap: false,
                        can_derive: false,
                        exportable: false,
                        extractable: false,
                        min_security_level: 2,
                        max_operations: None,
                        allowed_applications: vec![],
                    },
                    metadata: crate::tunnel::hsm::types::KeyMetadata {
                        key_id: format!("session_key_{}", uuid::Uuid::new_v4()),
                        key_name: "session_key".to_string(),
                        key_type: key_type.clone(),
                        created_at: chrono::Utc::now(None,
                        expires_at: None,
                        usage_policy: "default".to_string(),
                        tags: std::collections::HashMap::with_capacity(None,
                    require_user_presence: false,
                    generate_attestation: false, // Add missing field
                    target_hsm_tier: "SoftwareHsm".to_string(), // Add missing field
                };

                let provider_id = hsm_manager
                    .get_best_provider(
                        &crate::tunnel::hsm::manager::implementation::SecurityRequirements {
                            require_hardware_backing: security_requirements
                                .hardware_backed_required,
                            require_attestation: security_requirements.attestation_required,
                            minimum_key_size: 2048, // Use default instead of accessing performance_requirements
                        },
                    )
                    ?;

                info!("🔑 Using HSM provider: {}", provider_id);

                let hsm_key = beardog_core::HsmKey {
                    id: &request.key_id,
                    key_type: beardog_core::KeyType::Ed25519, // Convert from tunnel type
                    created_at: chrono::Utc::now(),
                    metadata: std::collections::HashMap::with_capacity({}, falling back to software generation",
                    e
                );
                self.generate_software_fallback_key(length)
            }
        }
    }

    /// Gets hsm_manager
    fn get_hsm_manager(&self) -> Result<Arc<crate::tunnel::hsm::manager::HsmManager>, BearDogError> {
        let hsm_config = crate::tunnel::hsm::HsmConfig {
            hsm_type: crate::tunnel::hsm::HsmType::Software,
            connection_config: crate::tunnel::hsm::ConnectionConfig {
                timeout_seconds: 30,
                retry_attempts: 3,
                connection_pool_size: 10,
            },
            performance_config: crate::tunnel::hsm::PerformanceConfig {
                max_operations_per_second: Some(1000),
                timeout_seconds: Some(30),
                retry_attempts: Some(3),
            },
        };
        Ok(Arc::new(crate::tunnel::hsm::manager::HsmManager::new(&beardog_core::HsmKey,
    ) -> Result<Vec<u8>, BearDogError> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(hsm_key.id.as_bytes());
        hasher.update(format!("{:?}", hsm_key.key_type).as_bytes());
        hasher.update(format!("{:?}", hsm_key.created_at).as_bytes());
        let hash = hasher.finalize();
        Ok(hash.to_vec())
    }


    fn generate_software_fallback_key(&self, length: usize) -> Result<Vec<u8>, BearDogError> {
        info!("🔧 Generating software fallback key material with enhanced entropy");

        let mut key_material = vec![0u8; length];

        let mut rng = rand::rng();
        rng.fill_bytes(&mut key_material);

        let additional_entropy = self.gather_system_entropy()?;

        for (i, &entropy_byte) in additional_entropy.iter().enumerate() {
            if i < key_material.len() {
                key_material[i] ^= entropy_byte;
            }
        }
        
        info!(
            "✅ Generated software fallback key material ({} bytes)",
            length
        );
        Ok(key_material)
    }


    fn gather_system_entropy(&self) -> Result<Vec<u8>, BearDogError> {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        
        hasher.update(
            chrono::Utc::now()
                .timestamp_nanos_opt()
                .unwrap_or(0)
                .to_le_bytes(),
        );

        hasher.update(std::process::id().to_le_bytes());

        hasher.update(format!("{:?}", std::thread::current(Arc<BStpKeyManager>,
    rotation_interval: std::time::Duration,
}

impl KeyRotationTask {
/// New operation.
    /// Creates a new instance
    pub fn new(Arc<BStpKeyManager>, rotation_interval: std::time::Duration) -> Self {
        Self {
            key_manager,
            rotation_interval,
        }
    }
}
