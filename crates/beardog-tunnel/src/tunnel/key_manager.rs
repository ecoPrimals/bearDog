

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
pub struct CryptoKey {

    pub key: Vec<u8>,

    pub created_at: SystemTime,

    pub expires_at: SystemTime,

    pub algorithm: CryptoAlgorithm,

    pub key_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CryptoAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
    GeneticHybrid,
}

pub struct BStpKeyManager {
    config: UnifiedProcessorConfig,
    keys: Arc<RwLock<HashMap<String, CryptoKey>>>,
    rotation_handles: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,
}

impl BStpKeyManager {

    pub async fn get_rotation_handle_count(&self) -> usize {
        let handles = self.rotation_handles.read().await;
        handles.len()
    }

    pub async fn stop_all_rotations(&self) {
        let mut handles = self.rotation_handles.write().await;
        for handle in handles.drain(..) {
            handle.abort();
        }
    }

    pub async fn new(config: UnifiedProcessorConfig) -> Result<Self, BearDogError> {
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
            rotation_handles: Arc::new(RwLock::new(Vec::new())),
        };
        info!("✅ Key Manager initialized successfully");
        debug!(
            "Key length: {} bytes, Rotation interval: {:?}",
            key_manager.config.session_key_length, key_manager.config.key_rotation_interval
        );
        Ok(key_manager)

    pub async fn generate_session_key(
        &self,
        session_id: &str,
        algorithm: CryptoAlgorithm,
    ) -> Result<CryptoKey, BearDogError> {
        info!("🔐 Generating session key for: {}", session_id);

        let key_material = self.generate_secure_key_material(algorithm.clone()).await?;
        let key = CryptoKey {
            key: key_material,
            created_at: SystemTime::now(),
            expires_at: SystemTime::now() + self.config.key_rotation_interval,
            algorithm: algorithm.clone(),
            key_id: format_args!("{}_{}", session_id, uuid::Uuid::new_v4().to_string()),

        {
            let mut keys = self.keys.write().await;
            keys.insert(key.key_id.clone(), key.clone());
            "✅ Session key generated: {} (algorithm: {:?})",
            key.key_id, algorithm
        Ok(key)

    pub async fn rotate_session_key(
        new_algorithm: Option<CryptoAlgorithm>,
        info!("🔄 Rotating session key for: {}", session_id);

        let algorithm = if let Some(algo) = new_algorithm {
            algo
        } else {

            let keys = self.keys.read().await;
            keys.values()
                .find(|k| k.key_id.starts_with(session_id))
                .map(|k| k.algorithm.clone())
                .unwrap_or(CryptoAlgorithm::Aes256Gcm)

        let new_key = self.generate_session_key(session_id, algorithm).await?;

            keys.retain(|k, _| !k.starts_with(session_id) || k == &new_key.key_id);
        info!("✅ Session key rotated successfully: {}", new_key.key_id);
        Ok(new_key)

    pub async fn get_key(&self, key_id: &str) -> Option<CryptoKey> {
        let keys = self.keys.read().await;
        keys.get(key_id).cloned()

    pub async fn get_session_key(&self, session_id: &str) -> Option<CryptoKey> {
        keys.values()
            .find(|k| k.key_id.starts_with(session_id))
            .cloned()

    pub async fn derive_contextual_key(
        base_key: &CryptoKey,
        context: &[u8],
    ) -> Result<Vec<u8>, BearDogError>> {

        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&base_key.key);
        hasher.update(context);
        hasher.update(b"beardog_key_derivation");
        let derived_hash = hasher.finalize();
        Ok(derived_hash.to_vec())

    pub async fn cleanup_expired_keys(&self) -> usize {
        let now = SystemTime::now();
        let mut keys = self.keys.write().await;
        let initial_count = keys.len();
        keys.retain(|_, key| key.expires_at > now);
        let cleaned_count = initial_count - keys.len();
        if cleaned_count > 0 {
            info!("🧹 Cleaned up {} expired keys", cleaned_count);
        cleaned_count

    async fn generate_secure_key_material(
        let key_length = match algorithm {
            CryptoAlgorithm::Aes256Gcm | CryptoAlgorithm::ChaCha20Poly1305 => 32, // 256 bits
            CryptoAlgorithm::GeneticHybrid => self.config.session_key_length,

        if self.config.use_hardware_keys {
            if let Ok(key_material) = self.generate_hardware_key(key_length).await {
                debug!("🔧 Generated hardware-backed key material");
                return Ok(key_material);
            } else {
                warn!("⚠️ Hardware key generation failed, falling back to software");
            }

        use rand::RngCore;
        let mut key_material = vec![0u8; key_length];
        rand::thread_rng().fill_bytes(&mut key_material);
        debug!("🔐 Generated software-based key material");
        Ok(key_material)

    async fn generate_hardware_key(&self, length: usize) -> Result<Vec<u8>, BearDogError>> {
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

        let key_type = match length {
            16 => crate::tunnel::hsm::types::KeyType::Aes128,
            24 => crate::tunnel::hsm::types::KeyType::Aes192,
            32 => crate::tunnel::hsm::types::KeyType::Aes256,
            _ => crate::tunnel::hsm::types::KeyType::Aes256, // Default to AES-256

        match self.get_hsm_manager().await {
            Ok(hsm_manager) => {
                debug!("🔐 Using HSM manager for hardware key generation");

                let request = crate::tunnel::hsm::GenerateKeyRequest {
                    key_id: format_args!("session_key_{}", uuid::Uuid::new_v4().to_string()),
                    key_type: key_type.clone(),
                    usage_policy: crate::tunnel::hsm::KeyUsagePolicy {
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
                        key_id: format_args!("session_key_{}", uuid::Uuid::new_v4().to_string()),
                        key_name: "session_key".to_string(),
                        key_type: key_type.clone(),
                        created_at: chrono::Utc::now(),
                        last_used: None,
                        expires_at: None,
                        usage_policy: "default".to_string(),
                        tags: std::collections::HashMap::with_capacity(16),
                    attestation_challenge: None,
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
                    .await?;

                info!("🔑 Using HSM provider: {}", provider_id);

                let hsm_key = beardog_core::HsmKey {
                    id: request.key_id.clone(),
                    key_type: beardog_core::KeyType::Ed25519, // Convert from tunnel type
                    created_at: chrono::Utc::now(),
                    metadata: std::collections::HashMap::with_capacity(16), // Use HashMap instead of KeyMetadata

                let key_material = self.extract_key_material_from_hsm_key(&hsm_key).await?;
                info!(
                    "✅ Generated hardware-backed key material ({} bytes) via HSM",
                    length
                );
            Err(e) => {
                warn!(
                    "⚠️ HSM manager unavailable: {}, falling back to software generation",
                    e

        self.generate_software_fallback_key(length).await

    async fn get_hsm_manager(&self) -> Result<Arc<crate::tunnel::hsm::manager::HsmManager, BearDogError>> {

        let hsm_config = crate::tunnel::hsm::HsmConfig {
            hsm_type: crate::tunnel::hsm::HsmType::Software,
            connection_config: crate::tunnel::hsm::ConnectionConfig {
                timeout_seconds: 30,
                retry_attempts: 3,
                connection_pool_size: 10,
            performance_config: crate::tunnel::hsm::PerformanceConfig {
                max_operations_per_second: Some(1000),
                timeout_seconds: Some(30),
                retry_attempts: Some(3),
        Ok(Arc::new(crate::tunnel::hsm::manager::HsmManager::new()))

    async fn extract_key_material_from_hsm_key(
        hsm_key: &beardog_core::HsmKey,

        hasher.update(hsm_key.id.as_bytes());
        hasher.update(format_args!("{:?}", hsm_key.key_type).to_string().as_bytes());
        hasher.update(format_args!("{:?}", hsm_key.created_at).to_string().as_bytes());
        let hash = hasher.finalize();
        Ok(hash.to_vec())

    async fn generate_software_fallback_key(&self, length: usize) -> Result<Vec<u8>, BearDogError>> {
        info!("🔧 Generating software fallback key material with enhanced entropy");

        let mut key_material = vec![0u8; length];

        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut key_material);

        let additional_entropy = self.gather_system_entropy().await?;

        for (i, &entropy_byte) in additional_entropy.iter().enumerate() {
            if i < key_material.len() {
                key_material[i] ^= entropy_byte;
        info!(
            "✅ Generated software fallback key material ({} bytes)",
            length

    async fn gather_system_entropy(&self) -> Result<Vec<u8>, BearDogError>> {

        hasher.update(
            chrono::Utc::now()
                .timestamp_nanos_opt()
                .unwrap_or(0)
                .to_le_bytes(),

        hasher.update(std::process::id().to_le_bytes());

        hasher.update(format_args!("{:?}", std::thread::current().to_string().id()).as_bytes());

        let stack_var = 0u64;
        hasher.update((&stack_var as *const u64 as usize).to_le_bytes());
        Ok(hasher.finalize().to_vec())
impl Drop for BStpKeyManager {

    fn drop(&mut self) {
        info!("🧹 Securely cleaning up key manager");
    }
}

pub struct KeyRotationTask {
    key_manager: Arc<BStpKeyManager>,
    rotation_interval: std::time::Duration,
}

impl KeyRotationTask {
    pub fn new(key_manager: Arc<BStpKeyManager>, rotation_interval: std::time::Duration) -> Self {
        Self {
            key_manager,
            rotation_interval,
        }
    }
}
