// Base Provider Traits - Foundation of Unified Provider System
//
// This module defines the core BaseProvider trait that all provider implementations
// must implement. It establishes the fundamental interface for provider lifecycle,
// health monitoring, capabilities, and ecosystem integration.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Note: SystemTime available for future timestamp needs

use super::{ProviderInfo, ProviderHealth, ProviderCapability, PerformanceMetrics};

/// Base trait that all BearDog providers must implement
/// 
/// This trait establishes the fundamental interface for all providers in the
/// BearDog ecosystem, ensuring consistent behavior for lifecycle management,
/// health monitoring, and capability discovery.
pub trait BaseProvider: Send + Sync {
    /// Get provider information
    fn provider_info(&self) -> ProviderInfo;

    /// Perform a health check on the provider
    fn health_check(&self) -> impl std::future::Future<Output = BearDogResult<ProviderHealth>> + Send;

    /// Get provider performance metrics
    fn metrics(&self) -> impl std::future::Future<Output = BearDogResult<PerformanceMetrics>> + Send;

    /// Get provider capabilities
    fn capabilities(&self) -> Vec<ProviderCapability>;

    /// Initialize the provider with configuration
    fn initialize(&mut self, config: ProviderConfiguration) -> impl std::future::Future<Output = BearDogResult<()>> + Send;

    /// Shutdown the provider gracefully
    fn shutdown(&mut self) -> impl std::future::Future<Output = BearDogResult<()>> + Send;

    /// Check if provider supports a specific capability
    fn supports_capability(&self, capability: &str) -> bool {
        self.capabilities().iter().any(|c| c.name == capability)
    }

    /// Get provider configuration schema
    fn configuration_schema(&self) -> ConfigurationSchema {
        ConfigurationSchema::default()
    }

    /// Validate provider configuration
    fn validate_configuration(&self, config: &ProviderConfiguration) -> BearDogResult<()> {
        // Default implementation - providers can override for custom validation
        if config.provider_id.is_empty() {
            return Err(BearDogError::business("Provider ID cannot be empty".to_string()));
        }
        Ok(())
    }
}

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
/// **MIGRATION**: Use `super::super::config::domains::system::LoggingConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use super::super::config::domains::system::LoggingConfig instead"
)]
pub type LoggingConfiguration = super::super::config::domains::system::LoggingConfig;

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

/// Performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfiguration {
    /// Connection pool settings
    pub connection_pool: ConnectionPoolConfiguration,
    
    /// Caching configuration
    pub caching: CachingConfiguration,
    
    /// Timeout settings
    pub timeouts: TimeoutConfiguration,
    
    /// Retry configuration
    pub retry: RetryConfiguration,
}

/// Connection pool configuration (DEPRECATED - use canonical config)
///
/// **MIGRATION**: Use `canonical::config::domains::network::ConnectionPoolConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use canonical::config::domains::network::ConnectionPoolConfig instead"
)]
pub type ConnectionPoolConfiguration = crate::canonical::config::domains::network::ConnectionPoolConfig;

/// Caching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachingConfiguration {
    /// Enable caching
    pub enabled: bool,
    
    /// Cache size limit
    pub max_size: u64,
    
    /// Cache TTL in seconds
    pub ttl: u64,
    
    /// Cache eviction policy
    pub eviction_policy: EvictionPolicy,
}

/// Cache eviction policies
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EvictionPolicy {
    Lru,
    Lfu,
    Fifo,
    Random,
    Custom(String),
}

/// Timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfiguration {
    /// Request timeout in seconds
    pub request_timeout: u64,
    
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    
    /// Read timeout in seconds
    pub read_timeout: u64,
    
    /// Write timeout in seconds
    pub write_timeout: u64,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfiguration {
    /// Maximum number of retries
    pub max_retries: u32,
    
    /// Base delay between retries in milliseconds
    pub base_delay_ms: u64,
    
    /// Maximum delay between retries in milliseconds
    pub max_delay_ms: u64,
    
    /// Backoff strategy
    pub backoff_strategy: BackoffStrategy,
}

/// Backoff strategies for retries
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BackoffStrategy {
    Fixed,
    Linear,
    Exponential,
    Jittered,
    Custom(String),
}

/// Configuration schema for providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationSchema {
    /// Schema parameters
    pub parameters: Vec<ConfigurationParameter>,
    
    /// Required parameters
    pub required: Vec<String>,
    
    /// Schema version
    pub version: String,
}

/// Configuration parameter definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationParameter {
    /// Parameter name
    pub name: String,
    
    /// Parameter type
    pub parameter_type: ParameterType,
    
    /// Parameter description
    pub description: Option<String>,
    
    /// Default value
    pub default_value: Option<ConfigurationValue>,
    
    /// Validation rules
    pub validation: Vec<ValidationRule>,
}

/// Parameter types for configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Integer,
    Float,
    Boolean,
    Array(Box<ParameterType>),
    Object(HashMap<String, ParameterType>),
    Custom(String),
}

/// Validation rules for parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    /// Rule type
    pub rule_type: ValidationRuleType,
    
    /// Rule parameters
    pub parameters: HashMap<String, ConfigurationValue>,
    
    /// Error message for validation failure
    pub error_message: String,
}

/// Types of validation rules
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationRuleType {
    Required,
    MinLength,
    MaxLength,
    Pattern,
    Range,
    Custom(String),
}

// Default implementations
impl Default for ProviderConfiguration {
    fn default() -> Self {
        Self {
            provider_id: String::new(),
            parameters: HashMap::new(),
            environment: EnvironmentSettings::default(),
            security: SecurityConfiguration::default(),
            performance: PerformanceConfiguration::default(),
            metadata: HashMap::new(),
        }
    }
}

impl Default for EnvironmentSettings {
    fn default() -> Self {
        Self {
            environment: "development".to_string(),
            variables: HashMap::new(),
            resource_limits: ResourceLimits::default(),
            logging: LoggingConfiguration::default(),
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_bytes: None,
            max_cpu_percent: None,
            max_disk_bytes: None,
            max_network_bps: None,
            max_concurrent_operations: Some(100),
        }
    }
}

impl Default for LoggingConfiguration {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "json".to_string(),
            structured: true,
            targets: vec!["stdout".to_string()],
        }
    }
}

impl Default for SecurityConfiguration {
    fn default() -> Self {
        Self {
            enable_tls: true,
            tls_cert_path: None,
            tls_key_path: None,
            authentication: AuthenticationConfiguration::default(),
            authorization: AuthorizationConfiguration::default(),
            encryption: EncryptionConfiguration::default(),
        }
    }
}

impl Default for AuthenticationConfiguration {
    fn default() -> Self {
        Self {
            method: AuthenticationMethod::ApiKey,
            parameters: HashMap::new(),
            session_timeout: 3600, // 1 hour
            enable_mfa: false,
        }
    }
}

impl Default for AuthorizationConfiguration {
    fn default() -> Self {
        Self {
            method: AuthorizationMethod::RoleBased,
            rbac: RbacConfiguration::default(),
            abac: AbacConfiguration::default(),
        }
    }
}

impl Default for RbacConfiguration {
    fn default() -> Self {
        Self {
            roles: Vec::new(),
            assignments: HashMap::new(),
            default_role: Some("user".to_string()),
        }
    }
}

impl Default for AbacConfiguration {
    fn default() -> Self {
        Self {
            attributes: Vec::new(),
            policies: Vec::new(),
            default_decision: PolicyDecision::Deny,
        }
    }
}

impl Default for EncryptionConfiguration {
    fn default() -> Self {
        Self {
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            key_management: KeyManagementConfiguration::default(),
            encrypt_at_rest: true,
            encrypt_in_transit: true,
        }
    }
}

impl Default for KeyManagementConfiguration {
    fn default() -> Self {
        Self {
            provider: KeyProvider::Local,
            rotation_interval: 86400, // 24 hours
            derivation: KeyDerivationConfiguration::default(),
        }
    }
}

impl Default for KeyDerivationConfiguration {
    fn default() -> Self {
        Self {
            algorithm: KeyDerivationAlgorithm::Pbkdf2,
            iterations: 100000,
            salt_length: 32,
        }
    }
}

impl Default for PerformanceConfiguration {
    fn default() -> Self {
        Self {
            connection_pool: ConnectionPoolConfiguration::default(),
            caching: CachingConfiguration::default(),
            timeouts: TimeoutConfiguration::default(),
            retry: RetryConfiguration::default(),
        }
    }
}

impl Default for ConnectionPoolConfiguration {
    fn default() -> Self {
        Self {
            min_size: 1,
            max_size: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SI as u64Z as u64E as u32,
            connection_timeout: 30,
            idle_timeout: 300,
        }
    }
}

impl Default for CachingConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            max_size: beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE as u64,
            ttl: 3600, // 1 hour
            eviction_policy: EvictionPolicy::Lru,
        }
    }
}

impl Default for TimeoutConfiguration {
    fn default() -> Self {
        Self {
            request_timeout: 30,
            connection_timeout: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE,
            read_timeout: 30,
            write_timeout: 30,
        }
    }
}

impl Default for RetryConfiguration {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE as u64,
            max_delay_ms: 10000,
            backoff_strategy: BackoffStrategy::Exponential,
        }
    }
}

impl Default for ConfigurationSchema {
    fn default() -> Self {
        Self {
            parameters: Vec::new(),
            required: Vec::new(),
            version: "1.0.0".to_string(),
        }
    }
} 