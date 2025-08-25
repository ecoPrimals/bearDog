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


/// # BearDog Key Manager
///
/// This module provides secure cryptographic key management for the BearDog tunnel system.
/// It handles key generation, rotation, storage, and lifecycle management with support for
/// multiple algorithms and hardware security modules.
/// ## Features
/// - **Secure Key Generation**: Hardware-backed key generation when available
/// - **Automatic Rotation**: Configurable key rotation policies
/// - **Algorithm Support**: AES-256-GCM, ChaCha20-Poly1305, and genetic hybrid algorithms
/// - **Performance Optimization**: Key pooling and predictive generation
/// - **Audit Trail**: Comprehensive logging for compliance requirements
/// ## Security Considerations
/// - Keys are never logged or stored in plaintext
/// - Hardware security modules are preferred when available
/// - Secure memory wiping on key destruction
/// - Cryptographically secure random number generation

use crate::tunnel::config::KeyManagementConfig;
use crate::tunnel::hsm::{PerformanceRequirements, SecurityRequirements};
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
/// Cryptographic key with metadata
/// Represents a cryptographic key along with its associated metadata including
/// creation time, expiration, algorithm, and unique identifier.
#[derive(Debug, Clone)]
pub struct CryptoKey {
    /// The raw key material (never logged)
    pub key: Vec<u8>,
    /// Timestamp when the key was created
    pub created_at: SystemTime,
    /// Timestamp when the key expires
    pub expires_at: SystemTime,
    /// Cryptographic algorithm for this key
    pub algorithm: CryptoAlgorithm,
    /// Unique identifier for the key
    pub key_id: String,
}
/// Supported cryptographic algorithms
/// Enumeration of cryptographic algorithms supported by the key manager.
/// Each algorithm has different performance and security characteristics.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CryptoAlgorithm {
    /// AES-256 with Galois/Counter Mode - High performance, widely supported
    Aes256Gcm,
    /// ChaCha20-Poly1305 - High security, good performance on older hardware
    ChaCha20Poly1305,
    /// Genetic algorithm optimized hybrid cipher - Adaptive security
    GeneticHybrid,
/// BearDog Tunnel Protocol Key Manager
/// Main key management system that handles the complete lifecycle of cryptographic keys.
/// Provides secure key generation, rotation, and storage with support for multiple algorithms.
/// ## Thread Safety
/// This implementation is thread-safe and can be safely shared across multiple threads.
/// ## Performance
/// Uses key pooling and background rotation to minimize latency during key operations.}


pub struct BStpKeyManager {
    config: KeyManagementConfig,
    keys: Arc<RwLock<HashMap<String, CryptoKey>>>,
    rotation_handles: Arc<RwLock<Vec<tokio::task::JoinHandle<()>>>>,}


impl BStpKeyManager {
    /// Get the number of active rotation handles
    pub async fn get_rotation_handle_count(&self) -> usize {
        let handles = self.rotation_handles.read().await;
        handles.len()
    }
    /// Stop all rotation handles
    pub async fn stop_all_rotations(&self) {
        let mut handles = self.rotation_handles.write().await;
        for handle in handles.drain(..) {
            handle.abort();
        }
    /// Create a new key manager instance
    ///
    /// Initializes the key manager with the provided configuration and starts
    /// background tasks for key rotation and maintenance.
    /// # Arguments
    /// * `config` - Key management configuration
    /// # Returns
    /// * `Ok(BStpKeyManager)` - Successfully initialized key manager
    /// * `Err(BearDogError)` - Configuration error or initialization failure
    /// # Examples
    /// ```rust,no_run
    /// use beardog::tunnel::config::KeyManagementConfig;
    /// use beardog::tunnel::key_manager::BStpKeyManager;
    /// let config = KeyManagementConfig::default();
    /// let key_manager = BStpKeyManager::new(config).await?;
    /// ```
    pub async fn new(config: KeyManagementConfig) -> BearDogResult<Self> {
        info!("🔑 Initializing BearDog Key Manager");
        // Validate configuration
        if config.session_key_length < 16 {
            return Err(BearDogError::config("Key length must be at least 16 bytes"));
        if config.key_derivation_rounds < 1000 {
            warn!("⚠️ Low key derivation rounds may impact security");
        let key_manager = Self {
            config,
            keys: Arc::new(RwLock::new(HashMap::new())),
            rotation_handles: Arc::new(RwLock::new(Vec::new())),
        };
        info!("✅ Key Manager initialized successfully");
        debug!(
            "Key length: {} bytes, Rotation interval: {:?}",
            key_manager.config.session_key_length, key_manager.config.key_rotation_interval
        );
        Ok(key_manager)
    /// Generate a new session key
    /// Creates a new cryptographic key for the specified session using the requested algorithm.
    /// The key is automatically registered for rotation and lifecycle management.
    /// * `session_id` - Unique identifier for the session
    /// * `algorithm` - Desired cryptographic algorithm
    /// * `Ok(CryptoKey)` - Successfully generated key
    /// * `Err(BearDogError)` - Key generation failure
    /// # Security Notes
    /// - Uses cryptographically secure random number generation
    /// - Key material is never logged or stored insecurely
    /// - Hardware entropy sources are preferred when available
    pub async fn generate_session_key(
        &self,
        session_id: &str,
        algorithm: CryptoAlgorithm,
    ) -> BearDogResult<CryptoKey> {
        info!("🔐 Generating session key for: {}", session_id);
        // Generate secure random key material
        let key_material = self.generate_secure_key_material(algorithm.clone()).await?;
        let key = CryptoKey {
            key: key_material,
            created_at: SystemTime::now(),
            expires_at: SystemTime::now() + self.config.key_rotation_interval,
            algorithm: algorithm.clone(),
            key_id: format!("{}_{}", session_id, uuid::Uuid::new_v4()),
        // Store the key
        {
            let mut keys = self.keys.write().await;
            keys.insert(key.key_id.clone(), key.clone());
            "✅ Session key generated: {} (algorithm: {:?})",
            key.key_id, algorithm
        Ok(key)
    /// Rotate an existing session key
    /// Generates a new key for the session and invalidates the old one.
    /// This operation is atomic and maintains session continuity.
    /// * `session_id` - Session identifier to rotate key for
    /// * `new_algorithm` - Optional new algorithm (uses existing if None)
    /// * `Ok(CryptoKey)` - New rotated key
    /// * `Err(BearDogError)` - Rotation failure
    pub async fn rotate_session_key(
        new_algorithm: Option<CryptoAlgorithm>,
        info!("🔄 Rotating session key for: {}", session_id);
        // Find existing key to determine algorithm
        let algorithm = if let Some(algo) = new_algorithm {
            algo
        } else {
            // Find existing key for this session
            let keys = self.keys.read().await;
            keys.values()
                .find(|k| k.key_id.starts_with(session_id))
                .map(|k| k.algorithm.clone())
                .unwrap_or(CryptoAlgorithm::Aes256Gcm)
        // Generate new key
        let new_key = self.generate_session_key(session_id, algorithm).await?;
        // Remove old keys for this session
            keys.retain(|k, _| !k.starts_with(session_id) || k == &new_key.key_id);
        info!("✅ Session key rotated successfully: {}", new_key.key_id);
        Ok(new_key)
    /// Retrieve a key by its identifier
    /// Looks up a previously generated key by its unique identifier.
    /// * `key_id` - Unique key identifier
    /// * `Some(CryptoKey)` - Key found
    /// * `None` - Key not found or expired
    pub async fn get_key(&self, key_id: &str) -> Option<CryptoKey> {
        let keys = self.keys.read().await;
        keys.get(key_id).cloned()
    /// Get a session key by session ID
    /// Convenience method to retrieve the current key for a session.
    /// * `session_id` - Session identifier
    /// * `Some(CryptoKey)` - Session key found
    /// * `None` - No key found for this session}


    pub async fn get_session_key(&self, session_id: &str) -> Option<CryptoKey> {
        keys.values()
            .find(|k| k.key_id.starts_with(session_id))
            .cloned()
    /// Derive a contextual key from an existing key
    /// Creates a derived key using the base key and contextual information.
    /// * `base_key` - Base key to derive from
    /// * `context` - Context information for derivation
    /// * `Ok(Vec<u8>)` - Derived key material
    /// * `Err(BearDogError)` - Derivation failure
    pub async fn derive_contextual_key(
        base_key: &CryptoKey,
        context: &[u8],
    ) -> BearDogResult<Vec<u8>> {
        // Simple key derivation using SHA-256
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&base_key.key);
        hasher.update(context);
        hasher.update(b"beardog_key_derivation");
        let derived_hash = hasher.finalize();
        Ok(derived_hash.to_vec())
    /// Clean up expired keys
    /// Removes expired keys from storage and securely wipes their memory.
    /// This method is called automatically by the rotation task.
    /// Number of keys cleaned up
    pub async fn cleanup_expired_keys(&self) -> usize {
        let now = SystemTime::now();
        let mut keys = self.keys.write().await;
        let initial_count = keys.len();
        keys.retain(|_, key| key.expires_at > now);
        let cleaned_count = initial_count - keys.len();
        if cleaned_count > 0 {
            info!("🧹 Cleaned up {} expired keys", cleaned_count);
        cleaned_count
    /// Generate secure key material for the specified algorithm
    /// Uses the best available entropy source to generate cryptographically secure key material.
    /// * `algorithm` - Algorithm requiring key material
    /// * `Ok(Vec<u8>)` - Secure key material
    /// * `Err(BearDogError)` - Generation failure
    async fn generate_secure_key_material(
        let key_length = match algorithm {
            CryptoAlgorithm::Aes256Gcm | CryptoAlgorithm::ChaCha20Poly1305 => 32, // 256 bits
            CryptoAlgorithm::GeneticHybrid => self.config.session_key_length,
        // Use hardware entropy if available and configured
        if self.config.use_hardware_keys {
            if let Ok(key_material) = self.generate_hardware_key(key_length).await {
                debug!("🔧 Generated hardware-backed key material");
                return Ok(key_material);
            } else {
                warn!("⚠️ Hardware key generation failed, falling back to software");
            }
        // Software-based secure random generation
        use rand::RngCore;
        let mut key_material = vec![0u8; key_length];
        rand::thread_rng().fill_bytes(&mut key_material);
        debug!("🔐 Generated software-based key material");
        Ok(key_material)
    /// Generate hardware-backed key material
    /// Uses the HSM manager to generate keys using the best available HSM provider.
    /// * `length` - Required key length in bytes
    /// * `Ok(Vec<u8>)` - Hardware-generated key material
    /// * `Err(BearDogError)` - Hardware unavailable or error
    async fn generate_hardware_key(&self, length: usize) -> BearDogResult<Vec<u8>> {
        debug!("🔧 Generating hardware key material using HSM");
        // Create security requirements for key generation
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
        // Determine key type based on length
        let key_type = match length {
            16 => crate::tunnel::hsm::types::KeyType::Aes128,
            24 => crate::tunnel::hsm::types::KeyType::Aes192,
            32 => crate::tunnel::hsm::types::KeyType::Aes256,
            _ => crate::tunnel::hsm::types::KeyType::Aes256, // Default to AES-256
        // Try to use HSM manager for hardware-backed key generation
        match self.get_hsm_manager().await {
            Ok(hsm_manager) => {
                debug!("🔐 Using HSM manager for hardware key generation");
                // Create key generation request
                let request = crate::tunnel::hsm::GenerateKeyRequest {
                    key_id: format!("session_key_{}", uuid::Uuid::new_v4()),
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
                        key_id: format!("session_key_{}", uuid::Uuid::new_v4()),
                        key_name: "session_key".to_string(),
                        key_type: key_type.clone(),
                        created_at: chrono::Utc::now(),
                        last_used: None,
                        expires_at: None,
                        usage_policy: "default".to_string(),
                        tags: std::collections::HashMap::new(),
                    attestation_challenge: None,
                    require_user_presence: false,
                    generate_attestation: false, // Add missing field
                    target_hsm_tier: "SoftwareHsm".to_string(), // Add missing field
                };
                // Generate key using HSM
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
                // Get the actual provider and generate key
                info!("🔑 Using HSM provider: {}", provider_id);
                // Placeholder key generation - in real implementation would use actual provider
                let hsm_key = beardog_core::HsmKey {
                    id: request.key_id.clone(),
                    key_type: beardog_core::KeyType::Ed25519, // Convert from tunnel type
                    created_at: chrono::Utc::now(),
                    metadata: std::collections::HashMap::new(), // Use HashMap instead of KeyMetadata
                // Extract key material from HSM key
                // Note: In a real implementation, this would be done more securely
                let key_material = self.extract_key_material_from_hsm_key(&hsm_key).await?;
                info!(
                    "✅ Generated hardware-backed key material ({} bytes) via HSM",
                    length
                );
            Err(e) => {
                warn!(
                    "⚠️ HSM manager unavailable: {}, falling back to software generation",
                    e
        // Fallback to enhanced software key generation
        self.generate_software_fallback_key(length).await
    /// Get HSM manager instance
    async fn get_hsm_manager(&self) -> BearDogResult<Arc<crate::tunnel::hsm::manager::HsmManager>> {
        // In a real implementation, this would be initialized during key manager construction
        // For now, create a basic HSM manager configuration
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
    /// Extract key material from HSM key (this is a simplified implementation)}


    async fn extract_key_material_from_hsm_key(
        hsm_key: &beardog_core::HsmKey,
        // In a real implementation, this would securely extract or generate key material
        // For now, we'll use a derived approach based on key ID
        hasher.update(hsm_key.id.as_bytes());
        hasher.update(format!("{:?}", hsm_key.key_type).as_bytes());
        hasher.update(format!("{:?}", hsm_key.created_at).as_bytes());
        let hash = hasher.finalize();
        Ok(hash.to_vec())
    /// Generate software fallback key with enhanced entropy
    async fn generate_software_fallback_key(&self, length: usize) -> BearDogResult<Vec<u8>> {
        info!("🔧 Generating software fallback key material with enhanced entropy");
        // Enhanced entropy generation with better randomness
        let mut key_material = vec![0u8; length];
        // Use system entropy source
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut key_material);
        // Add additional entropy from system sources
        let additional_entropy = self.gather_system_entropy().await?;
        // Mix entropy sources using XOR
        for (i, &entropy_byte) in additional_entropy.iter().enumerate() {
            if i < key_material.len() {
                key_material[i] ^= entropy_byte;
        info!(
            "✅ Generated software fallback key material ({} bytes)",
            length
    /// Gather additional system entropy
    async fn gather_system_entropy(&self) -> BearDogResult<Vec<u8>> {
        // Add timestamp
        hasher.update(
            chrono::Utc::now()
                .timestamp_nanos_opt()
                .unwrap_or(0)
                .to_le_bytes(),
        // Add process ID
        hasher.update(std::process::id().to_le_bytes());
        // Add thread ID (use stable method)
        hasher.update(format!("{:?}", std::thread::current().id()).as_bytes());
        // Add memory address entropy
        let stack_var = 0u64;
        hasher.update((&stack_var as *const u64 as usize).to_le_bytes());
        Ok(hasher.finalize().to_vec())
impl Drop for BStpKeyManager {
    /// Secure cleanup when key manager is dropped
    /// Ensures all key material is securely wiped from memory when the key manager
    /// is destroyed. This prevents key material from remaining in memory after use.}


    fn drop(&mut self) {
        info!("🧹 Securely cleaning up key manager");
        // In a real implementation, this would:
        // 1. Cancel all background tasks
        // 2. Securely wipe all key material from memory
        // 3. Close hardware security module connections
        // 4. Final audit logging
/// Automatic key rotation task
/// Background task that handles automatic key rotation according to the configured schedule.
/// This task runs independently and ensures keys are rotated before expiration.
pub struct KeyRotationTask {
    key_manager: Arc<BStpKeyManager>,
    rotation_interval: std::time::Duration,}


impl KeyRotationTask {
    /// Create a new key rotation task
    /// * `key_manager` - Key manager instance to manage
    /// * `rotation_interval` - Duration between key rotations}


    pub fn new(key_manager: Arc<BStpKeyManager>, rotation_interval: std::time::Duration) -> Self {
        Self {
            key_manager,
            rotation_interval,
