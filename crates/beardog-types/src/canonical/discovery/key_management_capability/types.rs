// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::canonical::types::ids::KeyId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::future::Future;

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
    fn encrypt(
        &self,
        plaintext: &[u8],
        key_id: &KeyId,
    ) -> impl Future<Output = Result<Vec<u8>, KmsError>> + Send;

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
    fn decrypt(
        &self,
        ciphertext: &[u8],
        key_id: &KeyId,
    ) -> impl Future<Output = Result<Vec<u8>, KmsError>> + Send;

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
    fn generate_key(&self, spec: KeySpec) -> impl Future<Output = Result<KeyId, KmsError>> + Send;

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
    fn sign(
        &self,
        data: &[u8],
        key_id: &KeyId,
    ) -> impl Future<Output = Result<Vec<u8>, KmsError>> + Send;

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
    fn verify(
        &self,
        data: &[u8],
        signature: &[u8],
        key_id: &KeyId,
    ) -> impl Future<Output = Result<bool, KmsError>> + Send;

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
    fn generate_random(
        &self,
        num_bytes: usize,
    ) -> impl Future<Output = Result<Vec<u8>, KmsError>> + Send;

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
    fn get_public_key(
        &self,
        key_id: &KeyId,
    ) -> impl Future<Output = Result<Vec<u8>, KmsError>> + Send;

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
    fn delete_key(&self, key_id: &KeyId) -> impl Future<Output = Result<(), KmsError>> + Send;

    /// List available keys
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<KeyMetadata>)` - List of keys with metadata
    /// * `Err(KmsError)` - If listing fails
    fn list_keys(&self) -> impl Future<Output = Result<Vec<KeyMetadata>, KmsError>> + Send;

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
    fn rotate_key(&self, key_id: &KeyId) -> impl Future<Output = Result<KeyId, KmsError>> + Send;

    /// Check health of KMS provider
    ///
    /// # Returns
    ///
    /// * `Ok(KmsHealthStatus)` - Health information
    /// * `Err(KmsError)` - If health check fails
    fn health_check(&self) -> impl Future<Output = Result<KmsHealthStatus, KmsError>> + Send;

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
    /// Key not found for the given identifier.
    KeyNotFound {
        /// KMS key identifier that could not be resolved.
        key_id: String,
    },

    /// KMS provider unavailable
    ProviderUnavailable {
        /// Provider name or endpoint that failed.
        provider: String,
        /// Human-readable reason (e.g. timeout, maintenance).
        reason: String,
    },

    /// Operation not supported
    OperationNotSupported {
        /// Requested operation name as reported by the provider.
        operation: String,
    },

    /// Invalid key specification
    InvalidKeySpec {
        /// Validation or parsing error detail.
        reason: String,
    },

    /// Cryptographic operation failed
    CryptoError {
        /// Provider error text or internal crypto failure summary.
        details: String,
    },

    /// Permission denied
    PermissionDenied {
        /// Resource or capability that was denied.
        resource: String,
    },

    /// Rate limit exceeded
    RateLimitExceeded {
        /// Suggested backoff before retrying, in seconds.
        retry_after_seconds: u64,
    },

    /// Network error
    NetworkError {
        /// Transport- or TLS-level failure description.
        details: String,
    },

    /// Generic error
    Other {
        /// Catch-all message when no specific variant applies.
        message: String,
    },
}

impl fmt::Display for KmsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::KeyNotFound { key_id } => {
                write!(f, "Key not found: {key_id}")
            }
            Self::ProviderUnavailable { provider, reason } => {
                write!(f, "KMS provider '{provider}' unavailable: {reason}")
            }
            Self::OperationNotSupported { operation } => {
                write!(f, "Operation not supported: {operation}")
            }
            Self::InvalidKeySpec { reason } => {
                write!(f, "Invalid key specification: {reason}")
            }
            Self::CryptoError { details } => {
                write!(f, "Cryptographic error: {details}")
            }
            Self::PermissionDenied { resource } => {
                write!(f, "Permission denied for resource: {resource}")
            }
            Self::RateLimitExceeded {
                retry_after_seconds,
            } => {
                write!(
                    f,
                    "Rate limit exceeded, retry after {retry_after_seconds} seconds"
                )
            }
            Self::NetworkError { details } => {
                write!(f, "Network error: {details}")
            }
            Self::Other { message } => {
                write!(f, "KMS error: {message}")
            }
        }
    }
}

impl std::error::Error for KmsError {}
