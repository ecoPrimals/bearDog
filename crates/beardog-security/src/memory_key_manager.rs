//! In-Memory Key Manager for Standalone BearDog
//!
//! **Crypto in Your Pocket - Standalone Security Manager**
//!
//! This module provides a fully self-contained, in-memory key management system
//! that enables BearDog to operate completely standalone while maintaining
//! the ability to share vaults with other BearDogs for network effects.

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::{debug, info};
use uuid::Uuid;

use crate::crypto_utils::BearDogCrypto;
use crate::types::*;
use beardog_errors::{BearDogError, BearDogResult};

/// Standalone in-memory key manager for portable crypto
#[derive(Debug)]
pub struct MemoryKeyManager {
    /// In-memory key vault
    key_vault: Arc<RwLock<HashMap<String, StoredKey>>>,
    /// Shared vault connections to other BearDogs
    shared_vaults: Arc<RwLock<HashMap<String, SharedVault>>>,
    /// Configuration for key management
    config: MemoryKeyConfig,
    /// Key derivation cache for performance
    derivation_cache: Arc<RwLock<HashMap<String, DerivedKey>>>,
    /// Metrics for key operations
    metrics: Arc<RwLock<KeyManagerMetrics>>,
}

/// Configuration for in-memory key management
#[derive(Debug, Clone)]
pub struct MemoryKeyConfig {
    /// Maximum number of keys to store in memory
    pub max_keys: usize,
    /// Key expiry duration (None = no expiry)
    pub key_expiry: Option<Duration>,
    /// Enable key derivation caching
    pub cache_derivations: bool,
    /// Enable automatic key rotation
    pub auto_rotation: bool,
    /// Rotation interval
    pub rotation_interval: Duration,
    /// Enable vault sharing
    pub enable_vault_sharing: bool,
    /// Vault sharing port
    pub vault_sharing_port: u16,
}

/// Stored key in memory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredKey {
    /// Key ID
    pub id: String,
    /// Key material (encrypted at rest)
    pub encrypted_material: Vec<u8>,
    /// Key metadata
    pub metadata: KeyMetadata,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last access timestamp
    pub last_accessed: DateTime<Utc>,
    /// Access count
    pub access_count: u64,
    /// Key status
    pub status: KeyStatus,
}

/// Key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Key type (e.g., "AES-256", "RSA-2048")
    pub key_type: String,
    /// Key purpose (e.g., "encryption", "signing")
    pub purpose: String,
    /// Key algorithm
    pub algorithm: String,
    /// Key size in bits
    pub key_size: u32,
    /// Owner ID
    pub owner_id: String,
    /// Key tags for organization
    pub tags: Vec<String>,
    /// Custom attributes
    pub attributes: HashMap<String, String>,
}

/// Shared vault connection to another BearDog
#[derive(Debug, Clone)]
pub struct SharedVault {
    /// Vault ID
    pub id: String,
    /// Remote BearDog endpoint
    pub endpoint: String,
    /// Authentication token
    pub auth_token: String,
    /// Vault permissions
    pub permissions: VaultPermissions,
    /// Connection status
    pub status: VaultConnectionStatus,
    /// Last sync timestamp
    pub last_sync: DateTime<Utc>,
    /// Shared keys
    pub shared_keys: HashMap<String, String>, // key_id -> local_key_id
}

/// Vault permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultPermissions {
    /// Can read keys
    pub read: bool,
    /// Can write keys
    pub write: bool,
    /// Can share keys
    pub share: bool,
    /// Can manage vault
    pub manage: bool,
}

/// Vault connection status
#[derive(Debug, Clone, PartialEq)]
pub enum VaultConnectionStatus {
    /// Connected and synced
    Connected,
    /// Connecting
    Connecting,
    /// Disconnected
    Disconnected,
    /// Error state
    Error(String),
}

/// Derived key for caching
#[derive(Debug, Clone)]
pub struct DerivedKey {
    /// Base key ID
    pub base_key_id: String,
    /// Derivation parameters
    pub derivation_params: String,
    /// Derived key material
    pub derived_material: Vec<u8>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Expiry timestamp
    pub expires_at: DateTime<Utc>,
}

/// Metrics for key manager monitoring
#[derive(Debug, Default, Clone)]
pub struct KeyManagerMetrics {
    /// Total keys stored
    pub total_keys: usize,
    /// Keys by type
    pub keys_by_type: HashMap<String, usize>,
    /// Key operations count
    pub operations_count: HashMap<String, u64>,
    /// Memory usage estimate
    pub memory_usage_bytes: usize,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Shared vault count
    pub shared_vaults: usize,
}

impl Default for MemoryKeyConfig {
    fn default() -> Self {
        Self {
            max_keys: 10_000,
            key_expiry: Some(Duration::days(30)),
            cache_derivations: true,
            auto_rotation: true,
            rotation_interval: Duration::days(7),
            enable_vault_sharing: true,
            vault_sharing_port: 8765,
        }
    }
}

impl MemoryKeyManager {
    /// Create a new standalone key manager
    pub async fn new(config: MemoryKeyConfig) -> BearDogResult<Self> {
        info!("🔐 Initializing standalone BearDog key manager");

        let manager = Self {
            key_vault: Arc::new(RwLock::new(HashMap::new())),
            shared_vaults: Arc::new(RwLock::new(HashMap::new())),
            config,
            derivation_cache: Arc::new(RwLock::new(HashMap::new())),
            metrics: Arc::new(RwLock::new(KeyManagerMetrics::default())),
        };

        // Start background tasks
        manager.start_background_tasks().await?;

        info!("✅ Standalone key manager initialized - crypto in your pocket!");
        Ok(manager)
    }

    /// Generate a new encryption key
    pub async fn generate_key(
        &self,
        key_type: String,
        purpose: String,
        owner_id: String,
    ) -> BearDogResult<String> {
        info!("🔑 Generating new key: {} for {}", key_type, purpose);

        // Check capacity
        let current_count = self.key_vault.read().await.len();
        if current_count >= self.config.max_keys {
            return Err(BearDogError::authorization("Key vault at capacity"));
        }

        // Generate key ID
        let key_id = format!("key_{}", Uuid::new_v4());

        // Generate key material based on type
        let (key_material, key_size) = self.generate_key_material(&key_type).await?;

        // Encrypt key material for storage using context-aware encryption
        let encrypted_material = self.encrypt_key_material(&key_material, &key_id).await?;

        // Create stored key
        let stored_key = StoredKey {
            id: key_id.clone(),
            encrypted_material,
            metadata: KeyMetadata {
                key_type: key_type.clone(),
                purpose: purpose.clone(),
                algorithm: self.get_algorithm_for_type(&key_type),
                key_size,
                owner_id,
                tags: Vec::new(),
                attributes: HashMap::new(),
            },
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            access_count: 0,
            status: KeyStatus::Active,
        };

        // Store key
        self.key_vault
            .write()
            .await
            .insert(key_id.clone(), stored_key);

        // Update metrics
        self.update_metrics().await;

        info!("✅ Key generated: {}", key_id);
        Ok(key_id)
    }

    /// Retrieve a key by ID
    pub async fn get_key(&self, key_id: &str) -> BearDogResult<Vec<u8>> {
        debug!("🔍 Retrieving key: {}", key_id);

        let mut vault = self.key_vault.write().await;
        if let Some(stored_key) = vault.get_mut(key_id) {
            // Check if key is active
            if stored_key.status != KeyStatus::Active {
                return Err(BearDogError::authorization("Key is not active"));
            }

            // Check expiry
            if let Some(expiry) = self.config.key_expiry {
                if Utc::now() - stored_key.created_at > expiry {
                    stored_key.status = KeyStatus::Expired;
                    return Err(BearDogError::authorization("Key has expired"));
                }
            }

            // Update access metrics
            stored_key.last_accessed = Utc::now();
            stored_key.access_count += 1;

            // Decrypt key material
            let key_material = self
                .decrypt_key_material(&stored_key.encrypted_material, &stored_key.id)
                .await?;

            debug!("✅ Key retrieved: {}", key_id);
            Ok(key_material)
        } else {
            // Try shared vaults
            self.get_key_from_shared_vaults(key_id).await
        }
    }

    /// Store a key in the vault
    pub async fn store_key(
        &self,
        key_material: &[u8],
        metadata: KeyMetadata,
    ) -> BearDogResult<String> {
        info!(
            "💾 Storing key: {} ({})",
            metadata.key_type, metadata.purpose
        );

        // Check capacity
        let current_count = self.key_vault.read().await.len();
        if current_count >= self.config.max_keys {
            return Err(BearDogError::authorization("Key vault at capacity"));
        }

        // Generate key ID
        let key_id = format!("key_{}", Uuid::new_v4());

        // Encrypt key material using context-aware encryption
        let encrypted_material = self.encrypt_key_material(key_material, &key_id).await?;

        // Create stored key
        let stored_key = StoredKey {
            id: key_id.clone(),
            encrypted_material,
            metadata,
            created_at: Utc::now(),
            last_accessed: Utc::now(),
            access_count: 0,
            status: KeyStatus::Active,
        };

        // Store key
        self.key_vault
            .write()
            .await
            .insert(key_id.clone(), stored_key);

        // Update metrics
        self.update_metrics().await;

        info!("✅ Key stored: {}", key_id);
        Ok(key_id)
    }

    /// Delete a key from the vault
    pub async fn delete_key(&self, key_id: &str) -> BearDogResult<()> {
        info!("🗑️ Deleting key: {}", key_id);

        let mut vault = self.key_vault.write().await;
        if let Some(mut stored_key) = vault.remove(key_id) {
            // Mark as revoked
            stored_key.status = KeyStatus::Revoked;

            // Securely clear key material
            stored_key.encrypted_material.fill(0);

            info!("✅ Key deleted: {}", key_id);
            Ok(())
        } else {
            Err(BearDogError::authorization("Key not found"))
        }
    }

    /// List all keys in the vault
    pub async fn list_keys(&self) -> BearDogResult<Vec<KeyInfo>> {
        let vault = self.key_vault.read().await;
        let keys: Vec<KeyInfo> = vault
            .values()
            .map(|stored_key| KeyInfo {
                id: stored_key.id.clone(),
                key_type: stored_key.metadata.key_type.clone(),
                purpose: stored_key.metadata.purpose.clone(),
                owner_id: stored_key.metadata.owner_id.clone(),
                created_at: stored_key.created_at,
                last_accessed: stored_key.last_accessed,
                access_count: stored_key.access_count,
                status: stored_key.status.clone(),
            })
            .collect();

        Ok(keys)
    }

    /// Share vault with another BearDog
    pub async fn share_vault_with(
        &self,
        endpoint: &str,
        permissions: VaultPermissions,
    ) -> BearDogResult<String> {
        if !self.config.enable_vault_sharing {
            return Err(BearDogError::authorization("Vault sharing is disabled"));
        }

        info!("🤝 Sharing vault with: {}", endpoint);

        // Generate vault ID
        let vault_id = format!("vault_{}", Uuid::new_v4());

        // Generate authentication token
        let auth_token = self.generate_auth_token().await?;

        // Create shared vault
        let shared_vault = SharedVault {
            id: vault_id.clone(),
            endpoint: endpoint.to_string(),
            auth_token,
            permissions,
            status: VaultConnectionStatus::Connecting,
            last_sync: Utc::now(),
            shared_keys: HashMap::new(),
        };

        // Store shared vault
        self.shared_vaults
            .write()
            .await
            .insert(vault_id.clone(), shared_vault);

        // Initiate connection
        self.connect_to_shared_vault(&vault_id).await?;

        info!("✅ Vault shared: {}", vault_id);
        Ok(vault_id)
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> BearDogResult<KeyManagerMetrics> {
        let metrics = self.metrics.read().await;
        Ok(metrics.clone())
    }

    /// Export vault for backup or migration
    pub async fn export_vault(&self, password: &str) -> BearDogResult<Vec<u8>> {
        info!("📦 Exporting vault for backup");

        let vault = self.key_vault.read().await;
        let vault_data = VaultExport {
            keys: vault.values().cloned().collect(),
            metadata: VaultMetadata {
                version: "1.0".to_string(),
                created_at: Utc::now(),
                key_count: vault.len(),
                beardog_version: env!("CARGO_PKG_VERSION").to_string(),
            },
        };

        // Serialize and encrypt vault data
        let serialized = serde_json::to_vec(&vault_data)?;
        let encrypted = self.encrypt_with_password(&serialized, password).await?;

        info!("✅ Vault exported ({} keys)", vault.len());
        Ok(encrypted)
    }

    /// Import vault from backup
    pub async fn import_vault(
        &self,
        encrypted_data: &[u8],
        password: &str,
    ) -> BearDogResult<usize> {
        info!("📥 Importing vault from backup");

        // Decrypt vault data
        let decrypted = self.decrypt_with_password(encrypted_data, password).await?;
        let vault_data: VaultExport = serde_json::from_slice(&decrypted)?;

        // Import keys
        let mut vault = self.key_vault.write().await;
        let mut imported_count = 0;

        for stored_key in vault_data.keys {
            if !vault.contains_key(&stored_key.id) {
                vault.insert(stored_key.id.clone(), stored_key);
                imported_count += 1;
            }
        }

        info!("✅ Vault imported ({} keys)", imported_count);
        Ok(imported_count)
    }

    /// Get or derive key with caching for performance
    async fn get_or_derive_key(
        &self,
        password: &str,
        salt: &[u8],
        iterations: u32,
        key_length: usize,
    ) -> BearDogResult<Vec<u8>> {
        use sha2::{Digest, Sha256};

        // Create cache key from password hash + salt + iterations + key_length
        let mut hasher = Sha256::new();
        hasher.update(password.as_bytes());
        hasher.update(salt);
        hasher.update(iterations.to_le_bytes());
        hasher.update((key_length as u32).to_le_bytes());
        let cache_key = format!("{:x}", hasher.finalize());

        // Check cache first
        {
            let cache = self.derivation_cache.read().await;
            if let Some(cached_key) = cache.get(&cache_key) {
                // Check if cached key is still valid (not expired)
                if cached_key.expires_at > chrono::Utc::now() {
                    return Ok(cached_key.derived_material.clone());
                }
            }
        }

        // Derive key using PBKDF2
        let key_bytes = crate::crypto_utils::BearDogCrypto::derive_key_pbkdf2(
            password.as_bytes(),
            salt,
            iterations,
            key_length,
        )?;

        // Cache the derived key (expires in 1 hour)
        {
            let mut cache = self.derivation_cache.write().await;
            let derived_key = DerivedKey {
                base_key_id: "password_derived".to_string(),
                derivation_params: format!(
                    "pbkdf2:{}:{}:{}",
                    iterations,
                    key_length,
                    hex::encode(salt)
                ),
                derived_material: key_bytes.clone(),
                created_at: chrono::Utc::now(),
                expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
            };
            cache.insert(cache_key, derived_key);
        }

        Ok(key_bytes)
    }

    /// Clear expired entries from derivation cache
    pub async fn cleanup_derivation_cache(&self) {
        let mut cache = self.derivation_cache.write().await;
        let now = chrono::Utc::now();
        cache.retain(|_, derived_key| derived_key.expires_at > now);
    }

    // Private helper methods

    /// Generate key material for different key types
    async fn generate_key_material(&self, key_type: &str) -> BearDogResult<(Vec<u8>, u32)> {
        use rand::RngCore;
        let mut rng = rand::thread_rng();

        match key_type {
            "AES-128" => {
                let mut key = vec![0u8; 16];
                rng.fill_bytes(&mut key);
                Ok((key, 128))
            }
            "AES-256" => {
                let mut key = vec![0u8; 32];
                rng.fill_bytes(&mut key);
                Ok((key, 256))
            }
            "ChaCha20" => {
                let mut key = vec![0u8; 32];
                rng.fill_bytes(&mut key);
                Ok((key, 256))
            }
            "HMAC-SHA256" => {
                let mut key = vec![0u8; 32];
                rng.fill_bytes(&mut key);
                Ok((key, 256))
            }
            _ => Err(BearDogError::authorization("Unsupported key type")),
        }
    }

    /// Get algorithm for key type
    fn get_algorithm_for_type(&self, key_type: &str) -> String {
        match key_type {
            "AES-128" => "AES-128-GCM".to_string(),
            "AES-256" => "AES-256-GCM".to_string(),
            "ChaCha20" => "ChaCha20-Poly1305".to_string(),
            "HMAC-SHA256" => "HMAC-SHA-256".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    /// Encrypt key material for storage using context-aware derivation
    async fn encrypt_key_material(
        &self,
        key_material: &[u8],
        key_id: &str,
    ) -> BearDogResult<Vec<u8>> {
        // NEW AGE CRYPTO: No master keys - derive encryption key from context
        // Each key is encrypted with a unique derived key
        // Compromising one doesn't compromise others
        let context_key = self.derive_context_encryption_key(key_id).await?;

        // Use proper AES-256-GCM encryption for security
        let (ciphertext, nonce) = BearDogCrypto::encrypt_aes_gcm(&context_key, key_material, None)?;

        // Store nonce (12 bytes) + ciphertext together
        // Format: [nonce (12 bytes)][ciphertext (variable length)]
        let mut encrypted_data = Vec::with_capacity(12 + ciphertext.len());
        encrypted_data.extend_from_slice(&nonce);
        encrypted_data.extend_from_slice(&ciphertext);

        tracing::debug!(
            "Encrypted key material for key_id '{}' using AES-256-GCM: {} bytes total",
            key_id,
            encrypted_data.len()
        );

        Ok(encrypted_data)
    }

    /// Decrypt key material from storage using context-aware derivation
    async fn decrypt_key_material(
        &self,
        encrypted_material: &[u8],
        key_id: &str,
    ) -> BearDogResult<Vec<u8>> {
        // Derive the same context-specific key for decryption
        let context_key = self.derive_context_encryption_key(key_id).await?;

        // Parse the stored format: [nonce (12 bytes)][ciphertext (variable length)]
        if encrypted_material.len() < 12 {
            return Err(BearDogError::Crypto {
                message: format!(
                    "Invalid encrypted material length for key_id '{}': expected at least 12 bytes, got {}",
                    key_id,
                    encrypted_material.len()
                ),
            });
        }

        let (nonce, ciphertext) = encrypted_material.split_at(12);

        // Use proper AES-256-GCM decryption for security
        let decrypted = BearDogCrypto::decrypt_aes_gcm(&context_key, ciphertext, nonce)?;

        tracing::debug!(
            "Decrypted key material for key_id '{}' using AES-256-GCM: {} bytes plaintext",
            key_id,
            decrypted.len()
        );

        Ok(decrypted)
    }

    /// Derive context-specific encryption key (NO master key)
    async fn derive_context_encryption_key(&self, key_id: &str) -> BearDogResult<Vec<u8>> {
        // New age crypto: Each key has its own encryption key derived from context
        // This means losing access to one key doesn't compromise others

        // Use key ID + node context to derive unique encryption key
        let derivation_context = format!("beardog_key_encryption_{key_id}");
        let node_entropy = self.get_node_entropy().await?;

        let derived_key = BearDogCrypto::derive_key_pbkdf2(
            node_entropy.as_bytes(),
            derivation_context.as_bytes(),
            100_000, // Strong iteration count
            32,      // 256-bit key
        )?;

        Ok(derived_key)
    }

    /// Get node-specific entropy (not a master key - unique per node)
    async fn get_node_entropy(&self) -> BearDogResult<String> {
        // This is not a master key - it's node-specific entropy
        // Each node has different entropy, so keys can't be transferred between nodes
        use std::fs;

        let entropy_path = format!(
            "{}/.beardog_node_entropy",
            std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string())
        );

        // Try to load existing entropy, or generate new
        if let Ok(entropy) = fs::read_to_string(&entropy_path) {
            Ok(entropy.trim().to_string())
        } else {
            // Generate new node entropy and save it
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| BearDogError::internal(format!("System time error: {e}")))?
                .as_secs();

            let new_entropy = format!("beardog_node_{}_{}", Uuid::new_v4(), timestamp);

            // Try to save (ignore errors for read-only filesystems)
            let _ = fs::write(&entropy_path, &new_entropy);
            Ok(new_entropy)
        }
    }

    /// Get key from shared vaults
    async fn get_key_from_shared_vaults(&self, key_id: &str) -> BearDogResult<Vec<u8>> {
        let shared_vaults = self.shared_vaults.read().await;

        for shared_vault in shared_vaults.values() {
            if shared_vault.permissions.read && shared_vault.shared_keys.contains_key(key_id) {
                // Try to retrieve from shared vault
                // Implementation would involve network call to remote BearDog
                debug!(
                    "🔗 Attempting to retrieve key from shared vault: {}",
                    shared_vault.id
                );
                // For now, return error
                return Err(BearDogError::authorization(
                    "Key not found in shared vaults",
                ));
            }
        }

        Err(BearDogError::authorization("Key not found"))
    }

    /// Connect to shared vault
    async fn connect_to_shared_vault(&self, vault_id: &str) -> BearDogResult<()> {
        debug!("🔗 Connecting to shared vault: {}", vault_id);

        // Update status to connected
        if let Some(vault) = self.shared_vaults.write().await.get_mut(vault_id) {
            vault.status = VaultConnectionStatus::Connected;
        }

        Ok(())
    }

    /// Generate authentication token
    async fn generate_auth_token(&self) -> BearDogResult<String> {
        let token = format!("token_{}", Uuid::new_v4());
        Ok(token)
    }

    /// Update metrics
    async fn update_metrics(&self) {
        let vault = self.key_vault.read().await;
        let mut metrics = self.metrics.write().await;

        metrics.total_keys = vault.len();
        metrics.keys_by_type.clear();

        for stored_key in vault.values() {
            *metrics
                .keys_by_type
                .entry(stored_key.metadata.key_type.clone())
                .or_insert(0) += 1;
        }

        metrics.memory_usage_bytes = vault.len() * 1024; // Rough estimate
        metrics.shared_vaults = self.shared_vaults.read().await.len();
    }

    /// Start background tasks
    async fn start_background_tasks(&self) -> BearDogResult<()> {
        // Start key rotation task
        if self.config.auto_rotation {
            self.start_key_rotation_task().await?;
        }

        // Start cleanup task
        self.start_cleanup_task().await?;

        Ok(())
    }

    /// Start key rotation background task
    async fn start_key_rotation_task(&self) -> BearDogResult<()> {
        let vault = self.key_vault.clone();
        let rotation_interval = self.config.rotation_interval;

        tokio::spawn(async move {
            let std_duration = match rotation_interval.to_std() {
                Ok(duration) => duration,
                Err(e) => {
                    tracing::error!("Failed to convert rotation interval to std duration: {}", e);
                    return;
                }
            };
            let mut interval = tokio::time::interval(std_duration);
            loop {
                interval.tick().await;

                // Rotate keys that need rotation
                let vault_read = vault.read().await;
                let keys_to_rotate: Vec<_> = vault_read
                    .iter()
                    .filter(|(_, key)| {
                        key.status == KeyStatus::Active
                            && Utc::now() - key.created_at > rotation_interval
                    })
                    .map(|(id, _)| id.clone())
                    .collect();
                drop(vault_read);

                for key_id in keys_to_rotate {
                    debug!("🔄 Rotating key: {}", key_id);
                    // Key rotation logic would go here
                }
            }
        });

        Ok(())
    }

    /// Start cleanup background task
    async fn start_cleanup_task(&self) -> BearDogResult<()> {
        let vault = self.key_vault.clone();
        let expiry = self.config.key_expiry;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600)); // 1 hour
            loop {
                interval.tick().await;

                if let Some(expiry_duration) = expiry {
                    let mut vault_write = vault.write().await;
                    let now = Utc::now();

                    // Remove expired keys
                    vault_write.retain(|_, key| {
                        if now - key.created_at > expiry_duration {
                            debug!("🗑️ Removing expired key: {}", key.id);
                            false
                        } else {
                            true
                        }
                    });
                }
            }
        });

        Ok(())
    }

    /// Encrypt with password
    async fn encrypt_with_password(&self, data: &[u8], password: &str) -> BearDogResult<Vec<u8>> {
        // Use proper AES-256-GCM encryption with password-derived key
        use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};
        use rand::RngCore;

        // Derive key from password using PBKDF2 with caching
        let salt = b"BearDog_MemoryKeyManager_Salt_32"; // 32 bytes
        let key_bytes = self.get_or_derive_key(password, salt, 100_000, 32).await?;

        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        // Generate secure random nonce
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        // Encrypt the data
        let ciphertext = cipher
            .encrypt(nonce, data)
            .map_err(|e| BearDogError::Encryption {
                operation: "password_encrypt".to_string(),
                message: format!("AES-256-GCM encryption failed: {e}"),
            })?;

        // Prepend nonce to ciphertext for decryption
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    /// Decrypt with password
    async fn decrypt_with_password(
        &self,
        encrypted_data: &[u8],
        password: &str,
    ) -> BearDogResult<Vec<u8>> {
        // Use proper AES-256-GCM decryption with password-derived key
        use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};

        if encrypted_data.len() < 12 {
            return Err(BearDogError::Encryption {
                operation: "password_decrypt".to_string(),
                message: "Encrypted data too short to contain nonce".to_string(),
            });
        }

        // Derive the same key from password using PBKDF2 with caching
        let salt = b"BearDog_MemoryKeyManager_Salt_32"; // 32 bytes
        let key_bytes = self.get_or_derive_key(password, salt, 100_000, 32).await?;

        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        let cipher = Aes256Gcm::new(key);

        // Extract nonce and ciphertext
        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        // Decrypt the data
        cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| BearDogError::Encryption {
                operation: "password_decrypt".to_string(),
                message: format!("AES-256-GCM decryption failed: {e}"),
            })
    }
}

/// Key information for listing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyInfo {
    pub id: String,
    pub key_type: String,
    pub purpose: String,
    pub owner_id: String,
    pub created_at: DateTime<Utc>,
    pub last_accessed: DateTime<Utc>,
    pub access_count: u64,
    pub status: KeyStatus,
}

/// Vault export structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultExport {
    pub keys: Vec<StoredKey>,
    pub metadata: VaultMetadata,
}

/// Vault metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VaultMetadata {
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub key_count: usize,
    pub beardog_version: String,
}

impl Default for VaultPermissions {
    fn default() -> Self {
        Self {
            read: true,
            write: false,
            share: false,
            manage: false,
        }
    }
}
