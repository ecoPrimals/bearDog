// SPDX-License-Identifier: AGPL-3.0-or-later

//! Core provider configuration: wire format, environment, security, and key management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::performance::PerformanceConfiguration;

/// Configuration for provider initialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfiguration {
    /// Provider identifier
    pub provider_id: String,
    
    /// Provider-specific configuration parameters
    pub parameters: HashMap<String, ConfigurationValue>,
    
    /// Environment-specific settings
    pub environment: EnvironmentSettings,
    
    /// Security configuration
    pub security: SecurityConfiguration,
    
    /// Performance tuning parameters
    pub performance: PerformanceConfiguration,
    
    /// Metadata for the provider
    pub metadata: HashMap<String, String>,
}

/// Configuration value that can hold different types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigurationValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<ConfigurationValue>),
    Object(HashMap<String, ConfigurationValue>),
}

/// Environment-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentSettings {
    /// Environment name (development, staging, production)
    pub environment: String,
    
    /// Environment variables
    pub variables: HashMap<String, String>,
    
    /// Resource limits
    pub resource_limits: ResourceLimits,
    
    /// Logging configuration
    pub logging: LoggingConfiguration,
}

/// Resource limits for provider operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory usage in bytes
    pub max_memory_bytes: Option<u64>,
    
    /// Maximum CPU usage percentage
    pub max_cpu_percent: Option<f64>,
    
    /// Maximum disk usage in bytes
    pub max_disk_bytes: Option<u64>,
    
    /// Maximum network bandwidth in bytes per second
    pub max_network_bps: Option<u64>,
    
    /// Maximum number of concurrent operations
    pub max_concurrent_operations: Option<u32>,
}

/// Logging configuration (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `crate::canonical::config::domains::system::LoggingConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use crate::canonical::config::domains::system::LoggingConfig instead"
)]
pub type LoggingConfiguration = crate::canonical::config::domains::system::LoggingConfig;

/// Security configuration for providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfiguration {
    /// Enable TLS for communications
    pub enable_tls: bool,
    
    /// TLS certificate path
    pub tls_cert_path: Option<String>,
    
    /// TLS private key path
    pub tls_key_path: Option<String>,
    
    /// Authentication configuration
    pub authentication: AuthenticationConfiguration,
    
    /// Authorization configuration
    pub authorization: AuthorizationConfiguration,
    
    /// Encryption settings
    pub encryption: EncryptionConfiguration,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfiguration {
    /// Authentication method
    pub method: AuthenticationMethod,
    
    /// Authentication parameters
    pub parameters: HashMap<String, String>,
    
    /// Session timeout in seconds
    pub session_timeout: u64,
    
    /// Enable multi-factor authentication
    pub enable_mfa: bool,
}

/// Authentication methods
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    None,
    ApiKey,
    BearerToken,
    OAuth2,
    MutualTls,
    Custom(String),
}

/// Authorization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationConfiguration {
    /// Authorization method
    pub method: AuthorizationMethod,
    
    /// Role-based access control settings
    pub rbac: RbacConfiguration,
    
    /// Attribute-based access control settings
    pub abac: AbacConfiguration,
}

/// Authorization methods
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthorizationMethod {
    None,
    RoleBased,
    AttributeBased,
    PolicyBased,
    Custom(String),
}

/// Role-based access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbacConfiguration {
    /// Available roles
    pub roles: Vec<Role>,
    
    /// Role assignments
    pub assignments: HashMap<String, Vec<String>>,
    
    /// Default role for new users
    pub default_role: Option<String>,
}

/// Role definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// Role name
    pub name: String,
    
    /// Role description
    pub description: Option<String>,
    
    /// Permissions granted by this role
    pub permissions: Vec<String>,
    
    /// Role metadata
    pub metadata: HashMap<String, String>,
}

/// Attribute-based access control configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbacConfiguration {
    /// Available attributes
    pub attributes: Vec<Attribute>,
    
    /// Access policies
    pub policies: Vec<AccessPolicy>,
    
    /// Default policy decision
    pub default_decision: PolicyDecision,
}

/// Attribute definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    /// Attribute name
    pub name: String,
    
    /// Attribute type
    pub attribute_type: AttributeType,
    
    /// Attribute description
    pub description: Option<String>,
}

/// Attribute types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttributeType {
    String,
    Integer,
    Boolean,
    List(Box<AttributeType>),
    Custom(String),
}

/// Access policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPolicy {
    /// Policy name
    pub name: String,
    
    /// Policy rules
    pub rules: Vec<PolicyRule>,
    
    /// Policy decision
    pub decision: PolicyDecision,
}

/// Policy rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    /// Attribute name
    pub attribute: String,
    
    /// Comparison operator
    pub operator: ComparisonOperator,
    
    /// Expected value
    pub value: ConfigurationValue,
}

/// Comparison operators for policy rules
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComparisonOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    NotContains,
    In,
    NotIn,
}

/// Policy decision
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PolicyDecision {
    Allow,
    Deny,
    Conditional(Vec<String>),
}

/// Encryption configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfiguration {
    /// Encryption algorithm
    pub algorithm: EncryptionAlgorithm,
    
    /// Key management settings
    pub key_management: KeyManagementConfiguration,
    
    /// Enable encryption at rest
    pub encrypt_at_rest: bool,
    
    /// Enable encryption in transit
    pub encrypt_in_transit: bool,
}

/// Encryption algorithms
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EncryptionAlgorithm {
    Aes256Gcm,
    ChaCha20Poly1305,
    Aes256Cbc,
    Custom(String),
}

/// Key management configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyManagementConfiguration {
    /// Key provider
    pub provider: KeyProvider,
    
    /// Key rotation interval in seconds
    pub rotation_interval: u64,
    
    /// Key derivation settings
    pub derivation: KeyDerivationConfiguration,
}

/// Key providers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KeyProvider {
    Local,
    HardwareSecurityModule,
    CloudKms(String),
    Custom(String),
}

/// Key derivation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDerivationConfiguration {
    /// Derivation algorithm
    pub algorithm: KeyDerivationAlgorithm,
    
    /// Number of iterations
    pub iterations: u32,
    
    /// Salt length in bytes
    pub salt_length: u32,
}

/// Key derivation algorithms
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum KeyDerivationAlgorithm {
    Pbkdf2,
    Scrypt,
    Argon2,
    Custom(String),
}

