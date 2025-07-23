//! Universal NestGate Integration Types
//!
//! Universal data structures and types for NestGate integration that can be used
//! by any ecosystem component (BearDog, SongBird, ToadStool, biomeOS, etc.)

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

// Import the new expiry types from beardog-security
use beardog_security::types::{
    EntropyAdjustmentConfig, EntropyBasedExpiry, GeneticRenewalConfig, KeyExpiryPolicy,
    KeyExpiryStatus,
};

/// Integration with BearDog's entropy hierarchy system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyIntegration {
    /// Entropy tier (1=machine, 2=supervised, 3=human)  
    pub entropy_tier: u8,
    /// Quality score from entropy hierarchy (0.0-1.0)
    pub entropy_quality: f64,
    /// Human entropy source type (if applicable)
    pub human_source_type: Option<String>,
    /// Whether this key uses self-sovereign entropy
    pub is_self_sovereign: bool,
    /// Entropy source identifier for tracking
    pub entropy_source_id: Option<String>,
    /// Genetic traits inherited from entropy
    pub genetic_traits: HashMap<String, String>,
    /// Link to entropy hierarchy manager
    pub entropy_manager_ref: Option<String>,
}

impl EntropyIntegration {
    /// Create entropy integration from entropy hierarchy data
    pub fn from_entropy_hierarchy(
        entropy_tier: u8,
        entropy_quality: f64,
        human_source_type: Option<String>,
        is_self_sovereign: bool,
    ) -> Self {
        Self {
            entropy_tier,
            entropy_quality,
            human_source_type,
            is_self_sovereign,
            entropy_source_id: None,
            genetic_traits: HashMap::new(),
            entropy_manager_ref: None,
        }
    }

    /// Calculate expiry duration based on entropy characteristics and configuration
    pub fn calculate_expiry_duration(
        &self,
        base_policy: &KeyExpiryPolicy,
        entropy_config: &EntropyAdjustmentConfig,
    ) -> Duration {
        let entropy_expiry = EntropyBasedExpiry {
            entropy_quality: self.entropy_quality,
            entropy_tier: self.entropy_tier,
            human_source_type: self.human_source_type.clone(),
            is_self_sovereign: self.is_self_sovereign,
        };

        entropy_expiry.calculate_expiry_duration(base_policy, entropy_config)
    }

    /// Determine if genetic renewal should be enabled for this key based on configuration
    pub fn should_enable_genetic_renewal(&self, entropy_config: &EntropyAdjustmentConfig) -> bool {
        let entropy_expiry = EntropyBasedExpiry {
            entropy_quality: self.entropy_quality,
            entropy_tier: self.entropy_tier,
            human_source_type: self.human_source_type.clone(),
            is_self_sovereign: self.is_self_sovereign,
        };

        entropy_expiry.should_enable_genetic_renewal(entropy_config)
    }
}

/// Universal result type for NestGate operations
pub type NestGateResult<T> = Result<T, NestGateError>;

/// Error types for NestGate adapter operations
#[derive(Debug, thiserror::Error)]
pub enum NestGateError {
    /// Configuration-related errors
    #[error("Configuration error: {0}")]
    Configuration(String),
    /// Authentication and authorization errors
    #[error("Authentication error: {0}")]
    Authentication(String),
    /// ZFS filesystem operation errors
    #[error("ZFS operation error: {0}")]
    ZfsOperation(String),
    /// Policy enforcement and validation errors
    #[error("Policy violation: {0}")]
    PolicyViolation(String),
    /// Audit logging and compliance errors
    #[error("Audit error: {0}")]
    Audit(String),
    /// Key management and cryptographic errors
    #[error("Key management error: {0}")]
    KeyManagement(String),
    /// File system operation errors
    #[error("File operation error: {0}")]
    FileOperation(String),
    /// Network communication errors
    #[error("Network error: {0}")]
    Network(String),
    /// Data serialization and deserialization errors
    #[error("Serialization error: {0}")]
    Serialization(String),
    /// Internal system errors
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Universal primal provider trait for NestGate integration
#[async_trait::async_trait]
pub trait PrimalProvider: Send + Sync {
    /// Get the primal provider name
    fn name(&self) -> &str;

    /// Get primal provider capabilities
    fn capabilities(&self) -> Vec<String>;

    /// Check if provider is healthy
    async fn health_check(&self) -> NestGateResult<HealthStatus>;

    /// Get provider configuration
    fn config(&self) -> &dyn std::any::Any;
}

/// Universal health status for any primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Overall health status
    pub healthy: bool,
    /// Status message
    pub message: String,
    /// Detailed component statuses
    pub components: HashMap<String, ComponentHealth>,
    /// Last check timestamp
    pub last_check: DateTime<Utc>,
}

/// Component health details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Component is healthy
    pub healthy: bool,
    /// Component status message
    pub status: String,
    /// Component metrics
    pub metrics: HashMap<String, String>,
}

/// Universal NestGate configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestGateConfig {
    /// Enable NestGate integration
    pub enabled: bool,
    /// NestGate API endpoint
    pub api_endpoint: String,
    /// Primal provider name (e.g., "beardog", "songbird", "toadstool")
    pub provider_name: String,
    /// Authentication configuration
    pub auth: AuthConfig,
    /// ZFS configuration
    pub zfs: ZfsConfig,
    /// Policy configuration
    pub policies: PolicyConfig,
    /// Audit configuration
    pub audit: AuditConfig,
    /// Universal capabilities
    pub capabilities: Vec<String>,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Authentication method
    pub method: AuthMethod,
    /// API key
    pub api_key: String,
    /// Client certificate path
    pub client_cert_path: Option<PathBuf>,
    /// Client key path
    pub client_key_path: Option<PathBuf>,
    /// CA certificate path
    pub ca_cert_path: Option<PathBuf>,
    /// Token refresh interval in seconds
    pub token_refresh_interval: u64,
}

/// Authentication methods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthMethod {
    /// API key authentication
    ApiKey,
    /// Certificate-based authentication
    Certificate,
    /// OAuth2 authentication
    OAuth2,
    /// Ed25519 cryptographic authentication
    Ed25519,
}

/// ZFS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZfsConfig {
    /// Default encryption algorithm
    pub default_algorithm: String,
    /// Key wrap algorithm
    pub wrap_algorithm: String,
    /// ZFS pool name
    pub pool_name: String,
    /// Dataset prefix
    pub dataset_prefix: String,
    /// Compression algorithm
    pub compression: String,
    /// Deduplication enabled
    pub deduplication: bool,
    /// Record size
    pub record_size: String,
}

/// Policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyConfig {
    /// Enabled policies
    pub enabled_policies: Vec<String>,
    /// Default access level
    pub default_access_level: AccessLevel,
    /// Require approval for operations
    pub require_approval: Vec<String>,
    /// Policy refresh interval in seconds
    pub refresh_interval: u64,
    /// External policy provider
    pub external_provider: Option<String>,
}

/// Audit configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditConfig {
    /// Enable audit logging
    pub enabled: bool,
    /// Audit retention period in days
    pub retention_days: u32,
    /// Log all operations
    pub log_all_operations: bool,
    /// Audit storage backend
    pub storage_backend: AuditStorageBackend,
    /// Audit encryption enabled
    pub encrypt_logs: bool,
}

/// Audit storage backends
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuditStorageBackend {
    /// Local file storage
    Local,
    /// Database storage
    Database,
    /// Remote syslog
    Syslog,
    /// Cloud storage
    Cloud,
}

/// Access levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccessLevel {
    /// No access
    None,
    /// Read-only access
    ReadOnly,
    /// Read-write access
    ReadWrite,
    /// Full admin access
    Admin,
}

/// Universal encryption key structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionKey {
    /// Unique key identifier
    pub key_id: String,
    /// Key type (e.g., "AES256", "RSA2048")
    pub key_type: String,
    /// Algorithm name (e.g., "AES-256-GCM")
    pub algorithm: String,
    /// Encrypted key material
    pub key_material: Vec<u8>,
    /// Key creation timestamp
    pub created_at: DateTime<Utc>,
    /// Key expiration timestamp
    pub expires_at: Option<DateTime<Utc>>,
    /// Key metadata
    pub metadata: HashMap<String, String>,
    /// Key purpose
    pub purpose: String,
    /// Key status
    pub status: KeyStatus,
}

/// Key status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KeyStatus {
    /// Key is active
    Active,
    /// Key is expired
    Expired,
    /// Key is revoked
    Revoked,
    /// Key is pending activation
    Pending,
}

/// Context-aware encryption key (NEW AGE CRYPTO - replaces master keys)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestGateContextKey {
    /// Key ID (includes context information)
    pub id: String,
    /// Owner ID (primal provider ID)  
    pub owner_id: String,
    /// Context for this key (what it's used for)
    pub context: String,
    /// Encryption algorithm
    pub algorithm: String,
    /// Key creation timestamp
    pub created_at: DateTime<Utc>,
    /// Key material (encrypted, context-specific)
    pub key_material: Vec<u8>,
    /// Key metadata
    pub metadata: HashMap<String, String>,
    /// Key derivation information
    pub derivation_info: KeyDerivationInfo,
    /// Key scope constraints (what this key can/cannot do)
    pub scope_constraints: Vec<String>,
    /// Configurable expiry policy (can be permanent if authorized)
    pub expiry_policy: KeyExpiryPolicy,
    /// Current expiry status
    pub expiry_status: KeyExpiryStatus,
    /// Integration with entropy hierarchy
    pub entropy_integration: Option<EntropyIntegration>,
    /// Genetic renewal configuration
    pub genetic_renewal: Option<GeneticRenewalConfig>,
}



/// Key derivation information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDerivationInfo {
    /// Key derivation function
    pub kdf: String,
    /// Salt value
    pub salt: Vec<u8>,
    /// Iteration count
    pub iterations: u32,
    /// Key length
    pub key_length: u32,
}

/// Wrapped key structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WrappedKey {
    /// Wrapped key data
    pub wrapped_data: Vec<u8>,
    /// Wrapping key ID
    pub wrapping_key_id: String,
    /// Wrapping algorithm
    pub algorithm: String,
    /// Key metadata
    pub metadata: HashMap<String, String>,
    /// Integrity check value
    pub integrity_check: Vec<u8>,
}

/// Key rotation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationResult {
    /// Workflow ID for approval process
    pub workflow_id: String,
    /// Current status
    pub status: String,
    /// Estimated completion time
    pub estimated_completion: Option<DateTime<Utc>>,
    /// Rotation metadata
    pub metadata: HashMap<String, String>,
}

/// Universal file operation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperationRequest {
    /// Operation type
    pub operation: FileOperation,
    /// Source path
    pub source_path: PathBuf,
    /// Destination path (for copy/move operations)
    pub destination_path: Option<PathBuf>,
    /// User ID performing the operation
    pub user_id: String,
    /// Primal provider ID
    pub provider_id: String,
    /// Operation metadata
    pub metadata: HashMap<String, String>,
    /// Operation priority
    pub priority: OperationPriority,
}

/// File operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FileOperation {
    /// Read file
    Read,
    /// Write file
    Write,
    /// Copy file
    Copy,
    /// Move file
    Move,
    /// Delete file
    Delete,
    /// Create directory
    CreateDirectory,
    /// List directory
    ListDirectory,
    /// Compress file
    Compress,
    /// Decompress file
    Decompress,
    /// Encrypt file
    Encrypt,
    /// Decrypt file
    Decrypt,
    /// Get file attributes
    GetAttributes,
    /// Set file attributes
    SetAttributes,
}

/// Operation priority
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperationPriority {
    /// Low priority
    Low,
    /// Normal priority
    Normal,
    /// High priority
    High,
    /// Critical priority
    Critical,
}

/// File operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOperationResult {
    /// Operation ID
    pub operation_id: String,
    /// Success status
    pub success: bool,
    /// Error message (if failed)
    pub error_message: Option<String>,
    /// Audit trail entry ID
    pub audit_entry_id: String,
    /// Operation timestamp
    pub timestamp: DateTime<Utc>,
    /// Result metadata
    pub metadata: HashMap<String, String>,
}

/// Universal audit event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestGateAuditEvent {
    /// Event ID
    pub id: String,
    /// Event type
    pub event_type: String,
    /// User ID
    pub user_id: String,
    /// Primal provider ID
    pub provider_id: String,
    /// Resource affected
    pub resource: String,
    /// Operation performed
    pub operation: String,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Event metadata
    pub metadata: HashMap<String, String>,
    /// Result of the operation
    pub result: OperationResult,
    /// Event severity
    pub severity: EventSeverity,
}

/// Operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationResult {
    /// Operation succeeded
    Success,
    /// Operation failed
    Failed { error: String },
    /// Operation denied by policy
    Denied { reason: String },
    /// Operation pending approval
    Pending { workflow_id: String },
}

/// Event severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EventSeverity {
    /// Informational event
    Info,
    /// Warning event
    Warning,
    /// Error event
    Error,
    /// Critical event
    Critical,
}

/// Universal access policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    /// Policy ID
    pub id: String,
    /// Policy name
    pub name: String,
    /// Policy description
    pub description: String,
    /// Policy rules
    pub rules: Vec<PolicyRule>,
    /// Policy enabled
    pub enabled: bool,
    /// Policy priority
    pub priority: u32,
    /// Policy creation timestamp
    pub created_at: DateTime<Utc>,
    /// Policy modification timestamp
    pub modified_at: DateTime<Utc>,
}

/// Policy rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// Rule ID
    pub id: String,
    /// User/group pattern
    pub subject: String,
    /// Resource pattern
    pub resource: String,
    /// Allowed operations
    pub operations: Vec<FileOperation>,
    /// Access level
    pub access_level: AccessLevel,
    /// Time-based restrictions
    pub time_restrictions: Option<TimeRestriction>,
    /// Conditions
    pub conditions: Vec<PolicyCondition>,
}

/// Time-based restrictions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRestriction {
    /// Start time (hour of day)
    pub start_hour: u8,
    /// End time (hour of day)
    pub end_hour: u8,
    /// Allowed days of week (0=Sunday, 6=Saturday)
    pub allowed_days: Vec<u8>,
    /// Timezone
    pub timezone: String,
}

/// Policy condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCondition {
    /// Condition type
    pub condition_type: ConditionType,
    /// Condition value
    pub value: String,
    /// Operator
    pub operator: ConditionOperator,
}

/// Condition types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionType {
    /// IP address condition
    IpAddress,
    /// User agent condition
    UserAgent,
    /// File size condition
    FileSize,
    /// File type condition
    FileType,
    /// Custom condition
    Custom(String),
}

/// Condition operators
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConditionOperator {
    /// Equal to
    Equal,
    /// Not equal to
    NotEqual,
    /// Greater than
    GreaterThan,
    /// Less than
    LessThan,
    /// Contains
    Contains,
    /// Regex match
    Regex,
}

/// Policy check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyCheckResult {
    /// Access allowed
    pub allowed: bool,
    /// Reason for decision
    pub reason: String,
    /// Access level granted
    pub access_level: AccessLevel,
    /// Matching policy ID
    pub policy_id: Option<String>,
    /// Matching rule ID
    pub rule_id: Option<String>,
}

/// Default implementations
impl Default for NestGateConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            api_endpoint: "https://nestgate.local:8443".to_string(),
            provider_name: "universal".to_string(),
            auth: AuthConfig::default(),
            zfs: ZfsConfig::default(),
            policies: PolicyConfig::default(),
            audit: AuditConfig::default(),
            capabilities: vec![
                "file_operations".to_string(),
                "key_management".to_string(),
                "zfs_integration".to_string(),
                "policy_enforcement".to_string(),
                "audit_logging".to_string(),
            ],
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            method: AuthMethod::ApiKey,
            api_key: String::new(),
            client_cert_path: None,
            client_key_path: None,
            ca_cert_path: None,
            token_refresh_interval: 3600,
        }
    }
}

impl Default for ZfsConfig {
    fn default() -> Self {
        Self {
            default_algorithm: "AES-256-GCM".to_string(),
            wrap_algorithm: "AES-256-KW".to_string(),
            pool_name: "secure_pool".to_string(),
            dataset_prefix: "secure".to_string(),
            compression: "lz4".to_string(),
            deduplication: true,
            record_size: "128K".to_string(),
        }
    }
}

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            enabled_policies: vec!["default_policy".to_string()],
            default_access_level: AccessLevel::ReadOnly,
            require_approval: vec![],
            refresh_interval: 300,
            external_provider: None,
        }
    }
}

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            retention_days: 90,
            log_all_operations: true,
            storage_backend: AuditStorageBackend::Local,
            encrypt_logs: true,
        }
    }
}
