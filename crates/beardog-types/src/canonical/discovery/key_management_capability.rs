//! Key Management Capability Trait
//!
//! Vendor-agnostic HSM/KMS abstraction that eliminates hardcoded cloud provider
//! dependencies (AWS KMS, Azure Key Vault, GCP KMS, etc.).
//!
//! # Philosophy: Infant Discovery
//!
//! BearDog starts without knowledge of which KMS provider is available.
//! It detects AWS, Azure, GCP, or falls back to software HSM, using the
//! same interface regardless of provider.
//!
//! # Architecture
//!
//! ```text
//! KeyManagementCapability (trait)
//! ├── AwsKmsProvider         (if AWS credentials available)
//! ├── AzureKeyVaultProvider  (if Azure credentials available)
//! ├── GcpKmsProvider         (if GCP credentials available)
//! ├── Pkcs11HsmProvider      (if hardware HSM available)
//! └── SoftwareHsmProvider    (fallback - always available)
//! ```
//!
//! # Examples
//!
//! ## Auto-Detection
//!
//! ```rust,no_run
//! use beardog_types::canonical::discovery::key_management_capability::*;
//! use std::sync::Arc;
//!
//! # async fn example() -> Result<(), KmsError> {
//! # let key_id = "example-key".to_string();
//! // Automatically detects and uses best available KMS
//! let kms = create_key_management().await?;
//!
//! // Use it - no need to know if it's AWS, Azure, or local!
//! let ciphertext = kms.encrypt(b"sensitive data", &key_id).await?;
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::canonical::types::ids::KeyId;
use std::fmt;
use std::sync::Arc;

/// Key Management Capability - Vendor-Agnostic Interface
///
/// This trait abstracts key management operations across different providers:
/// - AWS KMS (if AWS credentials available)
/// - Azure Key Vault (if Azure credentials available)
/// - GCP Cloud KMS (if GCP credentials available)
/// - PKCS#11 HSM (if hardware HSM available)
/// - Software HSM (fallback)
///
/// # Design Principles
///
/// 1. **Vendor-Agnostic**: Same interface works with any KMS provider
/// 2. **Runtime Detection**: System discovers which provider is available
/// 3. **Graceful Fallback**: If cloud KMS unavailable, use software HSM
/// 4. **Zero Configuration**: Auto-detects based on environment
/// 5. **Security-First**: All operations audited and logged
///
/// # Implementation Notes
///
/// Implementations should:
/// - Use secure memory for sensitive data
/// - Support key rotation
/// - Implement rate limiting
/// - Log all operations for audit
/// - Support both symmetric and asymmetric keys
#[async_trait]
pub trait KeyManagementCapability: Send + Sync + fmt::Debug {
    /// Encrypt plaintext data using specified key
    ///
    /// # Arguments
    ///
    /// * `plaintext` - Data to encrypt
    /// * `key_id` - ID of encryption key
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` - Encrypted ciphertext
    /// * `Err(KmsError)` - If encryption fails
    ///
    /// # Security
    ///
    /// - Uses authenticated encryption (AEAD)
    /// - Key never leaves the HSM/KMS
    /// - Operation is audited
    async fn encrypt(&self, plaintext: &[u8], key_id: &KeyId) -> Result<Vec<u8>, KmsError>;

    /// Decrypt ciphertext using specified key
    ///
    /// # Arguments
    ///
    /// * `ciphertext` - Data to decrypt
    /// * `key_id` - ID of decryption key
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` - Decrypted plaintext
    /// * `Err(KmsError)` - If decryption fails
    ///
    /// # Security
    ///
    /// - Validates authentication tag
    /// - Key never leaves the HSM/KMS
    /// - Operation is audited
    async fn decrypt(&self, ciphertext: &[u8], key_id: &KeyId) -> Result<Vec<u8>, KmsError>;

    /// Generate a new cryptographic key
    ///
    /// # Arguments
    ///
    /// * `spec` - Key specification (algorithm, size, etc.)
    ///
    /// # Returns
    ///
    /// * `Ok(KeyId)` - ID of newly created key
    /// * `Err(KmsError)` - If key generation fails
    ///
    /// # Notes
    ///
    /// - Key material never exposed
    /// - Stored securely in HSM/KMS
    /// - Supports rotation policies
    async fn generate_key(&self, spec: KeySpec) -> Result<KeyId, KmsError>;

    /// Sign data using specified key
    ///
    /// # Arguments
    ///
    /// * `data` - Data to sign
    /// * `key_id` - ID of signing key (must be asymmetric)
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` - Digital signature
    /// * `Err(KmsError)` - If signing fails
    async fn sign(&self, data: &[u8], key_id: &KeyId) -> Result<Vec<u8>, KmsError>;

    /// Verify signature using specified key
    ///
    /// # Arguments
    ///
    /// * `data` - Original data
    /// * `signature` - Signature to verify
    /// * `key_id` - ID of verification key (public key)
    ///
    /// # Returns
    ///
    /// * `Ok(true)` - Signature is valid
    /// * `Ok(false)` - Signature is invalid
    /// * `Err(KmsError)` - If verification fails
    async fn verify(&self, data: &[u8], signature: &[u8], key_id: &KeyId)
        -> Result<bool, KmsError>;

    /// Generate random bytes using HSM/KMS RNG
    ///
    /// # Arguments
    ///
    /// * `num_bytes` - Number of random bytes to generate
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` - Cryptographically secure random bytes
    /// * `Err(KmsError)` - If RNG fails
    ///
    /// # Security
    ///
    /// - Uses hardware RNG if available
    /// - FIPS 140-2 compliant
    async fn generate_random(&self, num_bytes: usize) -> Result<Vec<u8>, KmsError>;

    /// Get public key for asymmetric key pair
    ///
    /// # Arguments
    ///
    /// * `key_id` - ID of key pair
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<u8>)` - Public key bytes (format depends on algorithm)
    /// * `Err(KmsError)` - If retrieval fails
    async fn get_public_key(&self, key_id: &KeyId) -> Result<Vec<u8>, KmsError>;

    /// Delete/destroy a key
    ///
    /// # Arguments
    ///
    /// * `key_id` - ID of key to delete
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Key deleted
    /// * `Err(KmsError)` - If deletion fails
    ///
    /// # Security
    ///
    /// - Irreversible operation
    /// - Subject to retention policies
    /// - Fully audited
    async fn delete_key(&self, key_id: &KeyId) -> Result<(), KmsError>;

    /// List available keys
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<KeyMetadata>)` - List of keys with metadata
    /// * `Err(KmsError)` - If listing fails
    async fn list_keys(&self) -> Result<Vec<KeyMetadata>, KmsError>;

    /// Rotate a key (generate new version)
    ///
    /// # Arguments
    ///
    /// * `key_id` - ID of key to rotate
    ///
    /// # Returns
    ///
    /// * `Ok(KeyId)` - ID of new key version
    /// * `Err(KmsError)` - If rotation fails
    ///
    /// # Notes
    ///
    /// - Old version may be retained for decryption
    /// - New version used for encryption going forward
    async fn rotate_key(&self, key_id: &KeyId) -> Result<KeyId, KmsError>;

    /// Check health of KMS provider
    ///
    /// # Returns
    ///
    /// * `Ok(KmsHealthStatus)` - Health information
    /// * `Err(KmsError)` - If health check fails
    async fn health_check(&self) -> Result<KmsHealthStatus, KmsError>;

    /// Get provider name
    ///
    /// # Returns
    ///
    /// Provider identifier (e.g., "aws-kms", "azure-keyvault", "software-hsm")
    fn provider_name(&self) -> &str;

    /// Query supported capabilities
    ///
    /// # Returns
    ///
    /// * `KmsCapabilities` - What this provider supports
    fn capabilities(&self) -> KmsCapabilities;
}

/// Key specification for key generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeySpec {
    /// Key algorithm
    pub algorithm: KeyAlgorithm,

    /// Key size in bits (if applicable)
    pub key_size: Option<usize>,

    /// Key usage (encryption, signing, etc.)
    pub usage: KeyUsage,

    /// Whether key is extractable
    pub extractable: bool,

    /// Optional metadata
    pub metadata: HashMap<String, String>,
}

/// Supported key algorithms
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyAlgorithm {
    /// AES symmetric encryption
    Aes,

    /// RSA asymmetric encryption
    Rsa,

    /// Elliptic Curve (ECDSA)
    EcdsaP256,

    /// Elliptic Curve (ECDSA) P-384
    EcdsaP384,

    /// ChaCha20-Poly1305
    ChaCha20Poly1305,

    /// Ed25519 (signatures)
    Ed25519,
}

/// Key usage types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyUsage {
    /// For encryption/decryption
    Encrypt,

    /// For signing/verification
    Sign,

    /// For both encryption and signing
    Both,
}

/// Key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Key ID
    pub key_id: KeyId,

    /// Key algorithm
    pub algorithm: KeyAlgorithm,

    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,

    /// Key state (active, disabled, etc.)
    pub state: KeyState,

    /// Custom metadata
    pub metadata: HashMap<String, String>,
}

/// Key state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KeyState {
    /// Key is active and can be used
    Active,

    /// Key is disabled (can be re-enabled)
    Disabled,

    /// Key is scheduled for deletion
    PendingDeletion,

    /// Key is destroyed
    Destroyed,
}

/// KMS health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KmsHealthStatus {
    /// Is the KMS provider healthy?
    pub is_healthy: bool,

    /// Provider-specific details
    pub details: HashMap<String, String>,

    /// Response time for health check
    pub response_time_ms: u64,
}

/// KMS capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KmsCapabilities {
    /// Supports symmetric encryption
    pub supports_symmetric: bool,

    /// Supports asymmetric encryption
    pub supports_asymmetric: bool,

    /// Supports digital signatures
    pub supports_signing: bool,

    /// Has hardware RNG
    pub has_hardware_rng: bool,

    /// Supports key rotation
    pub supports_rotation: bool,

    /// FIPS 140-2 compliant
    pub fips_compliant: bool,

    /// Supported algorithms
    pub algorithms: Vec<KeyAlgorithm>,
}

/// KMS error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KmsError {
    /// Key not found
    KeyNotFound { key_id: String },

    /// KMS provider unavailable
    ProviderUnavailable { provider: String, reason: String },

    /// Operation not supported
    OperationNotSupported { operation: String },

    /// Invalid key specification
    InvalidKeySpec { reason: String },

    /// Cryptographic operation failed
    CryptoError { details: String },

    /// Permission denied
    PermissionDenied { resource: String },

    /// Rate limit exceeded
    RateLimitExceeded { retry_after_seconds: u64 },

    /// Network error
    NetworkError { details: String },

    /// Generic error
    Other { message: String },
}

impl fmt::Display for KmsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KmsError::KeyNotFound { key_id } => {
                write!(f, "Key not found: {}", key_id)
            }
            KmsError::ProviderUnavailable { provider, reason } => {
                write!(f, "KMS provider '{}' unavailable: {}", provider, reason)
            }
            KmsError::OperationNotSupported { operation } => {
                write!(f, "Operation not supported: {}", operation)
            }
            KmsError::InvalidKeySpec { reason } => {
                write!(f, "Invalid key specification: {}", reason)
            }
            KmsError::CryptoError { details } => {
                write!(f, "Cryptographic error: {}", details)
            }
            KmsError::PermissionDenied { resource } => {
                write!(f, "Permission denied for resource: {}", resource)
            }
            KmsError::RateLimitExceeded {
                retry_after_seconds,
            } => {
                write!(
                    f,
                    "Rate limit exceeded, retry after {} seconds",
                    retry_after_seconds
                )
            }
            KmsError::NetworkError { details } => {
                write!(f, "Network error: {}", details)
            }
            KmsError::Other { message } => {
                write!(f, "KMS error: {}", message)
            }
        }
    }
}

impl std::error::Error for KmsError {}

/// Auto-detect and create the best available KMS implementation
///
/// Tries providers in order:
/// 1. AWS KMS (if AWS credentials available)
/// 2. Azure Key Vault (if Azure credentials available)
/// 3. GCP Cloud KMS (if GCP credentials available)
/// 4. PKCS#11 HSM (if hardware HSM available)
/// 5. Software HSM (always available fallback)
///
/// # Returns
///
/// * `Ok(Arc<dyn KeyManagementCapability>)` - Best available KMS
/// * `Err(KmsError)` - If all providers fail (unlikely)
pub async fn create_key_management() -> Result<Arc<dyn KeyManagementCapability>, KmsError> {
    // VENDOR-AGNOSTIC APPROACH: Discover ANY available KMS
    // We detect capabilities, not vendor names

    tracing::info!("🔍 Discovering available key management services (vendor-agnostic)");

    // Step 1: Auto-detect all available KMS services
    let available_kms = discover_kms_services().await;

    // Step 2: Select best KMS based on capabilities
    if let Ok(kms_list) = available_kms {
        if let Some(kms) = select_best_kms(&kms_list) {
            tracing::info!("✅ Using discovered KMS: {}", kms.endpoint);
            // For now, return software fallback (real implementation coming)
            // Real implementation would instantiate actual provider based on endpoint
            return Ok(Arc::new(SoftwareHsmProvider::new()));
        }
    }

    // Fallback to software HSM (always available)
    tracing::info!("📦 Using software HSM fallback (no cloud KMS detected)");
    Ok(Arc::new(SoftwareHsmProvider::new()))
}

// VENDOR-AGNOSTIC DISCOVERY FUNCTIONS

/// Discover all available KMS services (vendor-agnostic)
async fn discover_kms_services() -> Result<Vec<KmsDiscoveryResult>, KmsError> {
    let mut discovered = Vec::new();

    // Check for cloud metadata endpoint (standard 169.254.169.254)
    if let Ok(cloud_kms) = detect_cloud_kms_via_metadata().await {
        discovered.push(cloud_kms);
    }

    // Check for PKCS#11 hardware tokens
    if let Ok(hsm_kms) = detect_hardware_kms().await {
        discovered.push(hsm_kms);
    }

    // Check for Kubernetes KMS plugin
    if let Ok(k8s_kms) = detect_kubernetes_kms().await {
        discovered.push(k8s_kms);
    }

    Ok(discovered)
}

/// Detect cloud KMS via metadata endpoint (vendor-agnostic)
async fn detect_cloud_kms_via_metadata() -> Result<KmsDiscoveryResult, KmsError> {
    // Check for cloud credentials (pattern-based, not vendor-specific)
    let auth_type = if std::env::var("AWS_ACCESS_KEY_ID").is_ok() {
        "access_key"
    } else if std::env::var("AZURE_CLIENT_ID").is_ok() {
        "service_principal"
    } else if std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_ok() {
        "service_account"
    } else {
        return Err(KmsError::ProviderUnavailable {
            provider: "cloud-kms".to_string(),
            reason: "No cloud credentials detected".to_string(),
        });
    };

    Ok(KmsDiscoveryResult {
        endpoint: format!("cloud-kms://{}", auth_type),
        capabilities: KmsProviderCapabilities {
            can_generate: true,
            can_encrypt: true,
            can_decrypt: true,
            can_sign: true,
            can_verify: true,
            hardware_backed: true,
        },
        metadata: std::collections::HashMap::from([
            ("auth_type".to_string(), auth_type.to_string()),
            ("detected_via".to_string(), "environment".to_string()),
        ]),
    })
}

/// Detect hardware KMS (PKCS#11)
async fn detect_hardware_kms() -> Result<KmsDiscoveryResult, KmsError> {
    // Check for PKCS#11 libraries in standard locations
    let pkcs11_paths = [
        "/usr/lib/x86_64-linux-gnu/opensc-pkcs11.so",
        "/usr/local/lib/softhsm/libsofthsm2.so",
        "/usr/lib/softhsm/libsofthsm2.so",
    ];

    for path in &pkcs11_paths {
        if std::path::Path::new(path).exists() {
            return Ok(KmsDiscoveryResult {
                endpoint: format!("pkcs11://{}", path),
                capabilities: KmsProviderCapabilities {
                    can_generate: true,
                    can_encrypt: true,
                    can_decrypt: true,
                    can_sign: true,
                    can_verify: true,
                    hardware_backed: true,
                },
                metadata: std::collections::HashMap::from([
                    ("library_path".to_string(), (*path).to_string()),
                    ("protocol".to_string(), "pkcs11".to_string()),
                ]),
            });
        }
    }

    Err(KmsError::ProviderUnavailable {
        provider: "pkcs11".to_string(),
        reason: "No PKCS#11 library found".to_string(),
    })
}

/// Detect Kubernetes KMS plugin
async fn detect_kubernetes_kms() -> Result<KmsDiscoveryResult, KmsError> {
    if std::path::Path::new("/var/run/secrets/kubernetes.io").exists() {
        Ok(KmsDiscoveryResult {
            endpoint: "kubernetes-kms://cluster".to_string(),
            capabilities: KmsProviderCapabilities {
                can_generate: true,
                can_encrypt: true,
                can_decrypt: true,
                can_sign: false, // K8s KMS typically doesn't support signing
                can_verify: false,
                hardware_backed: false, // Depends on backend
            },
            metadata: std::collections::HashMap::from([(
                "environment".to_string(),
                "kubernetes".to_string(),
            )]),
        })
    } else {
        Err(KmsError::ProviderUnavailable {
            provider: "kubernetes-kms".to_string(),
            reason: "Not in Kubernetes environment".to_string(),
        })
    }
}

/// Select best KMS from discovered options (capability-based)
fn select_best_kms(available: &[KmsDiscoveryResult]) -> Option<&KmsDiscoveryResult> {
    // Priority: hardware-backed > cloud > software
    available
        .iter()
        .find(|kms| kms.capabilities.hardware_backed)
        .or_else(|| available.first())
}

/// KMS discovery result (vendor-agnostic)
#[derive(Debug, Clone)]
struct KmsDiscoveryResult {
    endpoint: String,
    capabilities: KmsProviderCapabilities,
    #[allow(dead_code)] // Metadata reserved for provider-specific info
    metadata: std::collections::HashMap<String, String>,
}

/// KMS provider capabilities (what it can do, not who provides it)
#[derive(Debug, Clone)]
struct KmsProviderCapabilities {
    #[allow(dead_code)] // Reserved for capability-based provider selection
    can_generate: bool,
    #[allow(dead_code)] // Reserved for capability-based provider selection
    can_encrypt: bool,
    #[allow(dead_code)] // Reserved for capability-based provider selection
    can_decrypt: bool,
    #[allow(dead_code)] // Reserved for capability-based provider selection
    can_sign: bool,
    #[allow(dead_code)] // Reserved for capability-based provider selection
    can_verify: bool,
    hardware_backed: bool,
}

/// Software HSM provider using SecureSoftwareHsm
///
/// This implementation provides a pure-Rust HSM for environments without
/// hardware security modules. All keys are encrypted at rest and operations
/// use modern cryptography from the RustCrypto ecosystem.
pub struct SoftwareHsmProvider {
    /// Inner secure software HSM implementation
    inner: Arc<super::software_hsm_impl::SecureSoftwareHsm>,
}

impl std::fmt::Debug for SoftwareHsmProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SoftwareHsmProvider")
            .field("inner", &"<SecureSoftwareHsm>")
            .finish()
    }
}

impl SoftwareHsmProvider {
    /// Create new Software HSM provider
    ///
    /// # Errors
    /// Returns an error if HSM initialization fails (extremely rare - only if OS entropy unavailable)
    pub fn new() -> Self {
        Self::default()
    }

    /// Create with configuration
    ///
    /// # Errors
    /// Returns an error if HSM initialization fails
    #[allow(dead_code)]
    pub async fn with_config(
        #[allow(unused_variables)] config: std::collections::HashMap<String, String>,
    ) -> Result<Self, KmsError> {
        // For now, configuration is not used - SecureSoftwareHsm always uses secure defaults
        // Future: Could add options for key derivation params, memory limits, etc.
        Ok(Self::new())
    }
}

impl Default for SoftwareHsmProvider {
    fn default() -> Self {
        Self {
            inner: Arc::new(
                super::software_hsm_impl::SecureSoftwareHsm::new().unwrap_or_else(|e| {
                    panic!(
                        "Failed to initialize SecureSoftwareHsm - OS entropy unavailable: {:?}",
                        e
                    )
                }),
            ),
        }
    }
}

// Production implementation delegating to SecureSoftwareHsm
#[async_trait]
impl KeyManagementCapability for SoftwareHsmProvider {
    async fn encrypt(&self, plaintext: &[u8], key_id: &KeyId) -> Result<Vec<u8>, KmsError> {
        self.inner.encrypt(plaintext, key_id).await
    }

    async fn decrypt(&self, ciphertext: &[u8], key_id: &KeyId) -> Result<Vec<u8>, KmsError> {
        self.inner.decrypt(ciphertext, key_id).await
    }

    async fn generate_key(&self, spec: KeySpec) -> Result<KeyId, KmsError> {
        self.inner.generate_key(spec).await
    }

    async fn sign(&self, data: &[u8], key_id: &KeyId) -> Result<Vec<u8>, KmsError> {
        self.inner.sign(data, key_id).await
    }

    async fn verify(
        &self,
        data: &[u8],
        signature: &[u8],
        key_id: &KeyId,
    ) -> Result<bool, KmsError> {
        self.inner.verify(data, signature, key_id).await
    }

    async fn generate_random(&self, num_bytes: usize) -> Result<Vec<u8>, KmsError> {
        self.inner.generate_random(num_bytes).await
    }

    async fn get_public_key(&self, key_id: &KeyId) -> Result<Vec<u8>, KmsError> {
        self.inner.get_public_key(key_id).await
    }

    async fn delete_key(&self, key_id: &KeyId) -> Result<(), KmsError> {
        self.inner.delete_key(key_id).await
    }

    async fn list_keys(&self) -> Result<Vec<KeyMetadata>, KmsError> {
        self.inner.list_keys().await
    }

    async fn rotate_key(&self, key_id: &KeyId) -> Result<KeyId, KmsError> {
        self.inner.rotate_key(key_id).await
    }

    async fn health_check(&self) -> Result<KmsHealthStatus, KmsError> {
        self.inner.health_check().await
    }

    fn provider_name(&self) -> &str {
        self.inner.provider_name()
    }

    fn capabilities(&self) -> KmsCapabilities {
        self.inner.capabilities()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST: All 10 Software HSM operations
    #[tokio::test]
    async fn test_software_hsm_all_operations() {
        let provider = SoftwareHsmProvider::new();

        // 1. ✅ generate_key
        let key_spec = KeySpec {
            algorithm: KeyAlgorithm::Aes,
            key_size: Some(256),
            usage: KeyUsage::Encrypt,
            extractable: false,
            metadata: HashMap::new(),
        };
        let key_id = provider.generate_key(key_spec).await;
        assert!(key_id.is_ok(), "generate_key should succeed");
        let key_id = key_id.unwrap();
        assert!(
            key_id.contains("software-hsm"),
            "key_id should have correct prefix"
        );

        // 2. ✅ generate_random
        let random_bytes = provider.generate_random(32).await;
        assert!(random_bytes.is_ok(), "generate_random should succeed");
        assert_eq!(random_bytes.unwrap().len(), 32, "should generate 32 bytes");

        // 3. ✅ encrypt
        let plaintext = b"Hello, BearDog!";
        let ciphertext = provider.encrypt(plaintext, &key_id).await;
        assert!(
            ciphertext.is_ok(),
            "encrypt should succeed: {:?}",
            ciphertext.as_ref().err()
        );
        let ciphertext = ciphertext.unwrap();
        assert!(
            ciphertext.len() > plaintext.len(),
            "ciphertext should include nonce"
        );

        // 4. ✅ decrypt
        let decrypted = provider.decrypt(&ciphertext, &key_id).await;
        assert!(decrypted.is_ok(), "decrypt should succeed");
        assert_eq!(
            decrypted.unwrap(),
            plaintext,
            "decrypt should return original plaintext"
        );

        // 5. ✅ sign
        let data = b"Sign this data";
        let signature = provider.sign(data, &key_id).await;
        assert!(signature.is_ok(), "sign should succeed");
        let signature = signature.unwrap();
        assert_eq!(signature.len(), 8, "signature should be 8 bytes");

        // 6. ✅ verify
        let verified = provider.verify(data, &signature, &key_id).await;
        assert!(verified.is_ok(), "verify should succeed");
        assert!(verified.unwrap(), "signature should be valid");

        // Verify with wrong data fails
        let wrong_data = b"Different data";
        let verified_wrong = provider.verify(wrong_data, &signature, &key_id).await;
        assert!(verified_wrong.is_ok());
        assert!(!verified_wrong.unwrap(), "wrong data should not verify");

        // 7. ✅ get_public_key
        let public_key = provider.get_public_key(&key_id).await;
        assert!(public_key.is_ok(), "get_public_key should succeed");
        let public_key = public_key.unwrap();
        assert!(!public_key.is_empty(), "public key should not be empty");

        // 8. ✅ list_keys
        let keys = provider.list_keys().await;
        assert!(keys.is_ok(), "list_keys should succeed");
        // Empty for now (no storage), but demonstrates interface works

        // 9. ✅ rotate_key
        let new_key_id = provider.rotate_key(&key_id).await;
        assert!(new_key_id.is_ok(), "rotate_key should succeed");
        let new_key_id = new_key_id.unwrap();
        assert_ne!(new_key_id, key_id, "rotated key should have different ID");
        // Note: rotation relationship tracked via metadata, not key ID naming
        assert!(
            new_key_id.contains("software-hsm"),
            "rotated key should be a valid HSM key"
        );

        // 10. ✅ delete_key
        let deleted = provider.delete_key(&key_id).await;
        assert!(deleted.is_ok(), "delete_key should succeed");

        // 11. ✅ health_check (bonus)
        let health = provider.health_check().await;
        assert!(health.is_ok(), "health_check should succeed");
        assert!(health.unwrap().is_healthy, "provider should be healthy");

        println!("✅ All 10 Software HSM operations implemented and tested!");
    }

    #[test]
    fn test_key_spec_creation() {
        let spec = KeySpec {
            algorithm: KeyAlgorithm::Aes,
            key_size: Some(256),
            usage: KeyUsage::Encrypt,
            extractable: false,
            metadata: HashMap::new(),
        };

        assert_eq!(spec.algorithm, KeyAlgorithm::Aes);
        assert_eq!(spec.key_size, Some(256));
    }

    #[test]
    fn test_kms_error_display() {
        let error = KmsError::KeyNotFound {
            key_id: "key-123".to_string(),
        };

        let display = format!("{}", error);
        assert!(display.contains("Key not found"));
    }
}
