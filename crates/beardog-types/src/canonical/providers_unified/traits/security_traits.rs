// **SECURITY PROVIDER TRAITS**
//
// Security-focused provider traits including HSM, authentication, and encryption capabilities.

use super::base_traits::UnifiedProvider;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

/// **SECURITY PROVIDER TRAIT** - For all security-related providers
///
/// Extends the base provider with security-specific functionality including
/// authentication, authorization, encryption, and key management.
pub trait UnifiedSecurityProvider: UnifiedProvider {
    /// Authenticate a request
    fn authenticate(
        &self,
        request: AuthenticationRequest,
    ) -> impl std::future::Future<Output = Result<AuthenticationResponse, BearDogError>> + Send;

    /// Authorize an operation
    fn authorize(
        &self,
        request: AuthorizationRequest,
    ) -> impl std::future::Future<Output = Result<AuthorizationResponse, BearDogError>> + Send;

    /// Encrypt data
    fn encrypt(
        &self,
        data: &[u8],
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Decrypt data
    fn decrypt(
        &self,
        data: &[u8],
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Sign data
    fn sign(
        &self,
        data: &[u8],
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Verify signature
    fn verify(
        &self,
        data: &[u8],
        signature: &[u8],
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send;

    /// Generate a random number
    fn generate_random(
        &self,
        length: usize,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Get security context
    fn security_context(&self) -> SecurityContext;
}

/// **HSM PROVIDER TRAIT** - For Hardware Security Module providers
///
/// secure storage, and hardware attestation.
pub trait UnifiedHsmProvider: UnifiedSecurityProvider {
    /// Generate a key in the HSM
    fn generate_key(
        &self,
        spec: KeyGenerationSpec,
    ) -> impl std::future::Future<Output = Result<KeyInfo, BearDogError>> + Send;

    /// Import a key into the HSM
    fn import_key(
        &self,
        key_data: &[u8],
        key_type: KeyType,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<KeyInfo, BearDogError>> + Send;

    /// Export a key from the HSM (if allowed)
    fn export_key(
        &self,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<Vec<u8>, BearDogError>> + Send;

    /// Delete a key from the HSM
    /// Removes key
    fn delete_key(
        &self,
        key_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// List all keys in the HSM
    fn list_keys(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<KeyInfo>, BearDogError>> + Send;

    fn device_info(
        &self,
    ) -> impl std::future::Future<Output = Result<HsmDeviceInfo, BearDogError>> + Send;

    fn attest(
        &self,
    ) -> impl std::future::Future<Output = Result<AttestationResponse, BearDogError>> + Send;

    /// Backup keys from HSM
    fn backup_keys(
        &self,
        spec: KeyBackupSpec,
    ) -> impl std::future::Future<Output = Result<BackupInfo, BearDogError>> + Send;
}

/// Authentication request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationRequest {
    /// User identifier
    pub user_id: String,
    /// Authentication credentials
    /// Mapping of credentials
    pub credentials: HashMap<String, String>,
    /// Authentication context
    /// The context value
    pub context: AuthenticationContext,
}

/// Authentication response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationResponse {
    /// Authentication success status
    /// Whether success is enabled
    pub success: bool,
    /// Optional user info
    pub user_info: Option<HashMap<String, String>>,
    /// Authentication token
    /// Optional token
    pub token: Option<String>,
    /// Token expiration time
    /// Optional expires at
    pub expires_at: Option<SystemTime>,
    /// Error message (if authentication failed)
    /// Optional error
    pub error: Option<String>,
}

/// Authorization request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    /// User identifier
    pub user_id: String,
    /// Requested operation
    /// The operation value
    pub operation: String,
    /// Resource being accessed
    /// The resource value
    pub resource: String,
    /// Additional context
    /// Mapping of context
    pub context: HashMap<String, String>,
}

/// Authorization response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResponse {
    /// Authorization granted status
    /// Whether granted is enabled
    pub granted: bool,
    /// Granted permissions
    /// Collection of permissions
    pub permissions: Vec<String>,
    /// Authorization expiration time
    /// Optional expires at
    pub expires_at: Option<SystemTime>,
    /// Denial reason (if authorization denied)
    /// Optional denial reason
    pub denial_reason: Option<String>,
}

/// Key generation specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyGenerationSpec {
    /// Key identifier
    pub key_id: String,
    /// Key type
    /// The key type value
    pub key_type: KeyType,
    /// Key size in bits
    /// Number of `key_size`
    pub key_size: u32,
    /// Key usage restrictions
    /// Collection of key usage
    pub key_usage: Vec<KeyUsage>,
    /// Whether key is extractable
    /// Whether extractable is enabled
    pub extractable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    /// Key identifier
    pub key_id: String,
    /// Key type
    /// The key type value
    pub key_type: KeyType,
    /// Key size in bits
    /// Number of `key_size`
    pub key_size: u32,
    /// Key usage restrictions
    /// Collection of key usage
    pub key_usage: Vec<KeyUsage>,
    /// Key creation time
    /// The created at value
    pub created_at: SystemTime,
    /// Whether key is extractable
    /// Whether extractable is enabled
    pub extractable: bool,
}

/// Key type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of key
pub enum KeyType {
    /// RSA key
    Rsa,
    /// Elliptic Curve key
    EllipticCurve,
    /// AES symmetric key
    Aes,
    /// `ChaCha20` symmetric key
    ChaCha20,
    /// Ed25519 signature key
    Ed25519,
    /// X25519 key exchange key
    X25519,
    /// Generic key type
    Generic,
    /// Custom key type
    Custom(String),
}

/// Key usage enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KeyUsage {
    /// Encryption operations
    Encrypt,
    /// Decryption operations
    Decrypt,
    /// Digital signature operations
    Sign,
    /// Signature verification operations
    Verify,
    /// Key derivation operations
    Derive,
    /// Key wrapping operations
    Wrap,
    /// Key unwrapping operations
    Unwrap,
}

/// Authentication context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationContext {
    /// Client IP address
    /// Optional client ip
    pub client_ip: Option<String>,
    /// User agent string
    /// Optional user agent
    pub user_agent: Option<String>,
    /// Session identifier
    pub session_id: Option<String>,
    /// Additional context data
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmDeviceInfo {
    /// Device manufacturer
    /// The manufacturer value
    pub manufacturer: String,
    /// Device model
    /// The model value
    pub model: String,
    /// Device serial number
    /// The serial number value
    pub serial_number: String,
    /// Firmware version
    /// The firmware version value
    pub firmware_version: String,
    /// Hardware version
    /// The hardware version value
    pub hardware_version: String,
    /// Supported algorithms
    /// Collection of supported algorithms
    pub supported_algorithms: Vec<String>,
    /// Device capabilities
    /// Collection of capabilities
    pub capabilities: Vec<String>,
    /// Device status
    /// Current status of the component
    pub status: String,
    /// Device certificate (if available)
    /// Optional certificate
    pub certificate: Option<Vec<u8>>,
    /// Device attestation data
    /// Optional attestation data
    pub attestation_data: Option<Vec<u8>>,
}

/// Attestation response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationResponse {
    /// Attestation success status
    /// Whether success is enabled
    pub success: bool,
    /// Attestation data
    /// Collection of attestation data
    pub attestation_data: Vec<u8>,
    /// Attestation signature
    /// Collection of signature
    pub signature: Vec<u8>,
    /// Attestation certificate chain
    /// Collection of certificate chain
    pub certificate_chain: Vec<Vec<u8>>,
    /// Attestation timestamp
    pub timestamp: SystemTime,
}

/// Key backup specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBackupSpec {
    /// Key IDs to backup
    pub key_ids: Vec<String>,
    /// Backup encryption key
    /// Optional backup key
    pub backup_key: Option<String>,
    pub format: String,
    /// Include metadata
    /// Whether `include_metadata` is enabled
    pub include_metadata: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    /// Backup identifier
    pub backup_id: String,
    /// Backup data
    /// Collection of backup data
    pub backup_data: Vec<u8>,
    /// Backup metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
    /// Backup timestamp
    /// The created at value
    pub created_at: SystemTime,
    /// Backup checksum
    /// The checksum value
    pub checksum: String,
}

/// Security context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContext {
    /// Security level
    /// The security level value
    pub security_level: String,
    /// Encryption algorithms
    /// Collection of encryption algorithms
    pub encryption_algorithms: Vec<String>,
    /// Signature algorithms
    /// Collection of signature algorithms
    pub signature_algorithms: Vec<String>,
    /// Key derivation functions
    /// Collection of key derivation functions
    pub key_derivation_functions: Vec<String>,
    /// Random number generators
    /// Collection of random generators
    pub random_generators: Vec<String>,
}
