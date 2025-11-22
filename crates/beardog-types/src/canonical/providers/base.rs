// Base Provider Traits - Foundation of Unified Provider System
//
// This module defines the core BaseProvider trait that all provider implementations
// must implement. It establishes the fundamental interface for provider lifecycle,
// health monitoring, capabilities, and ecosystem integration.

use beardog_errors::BearDogError;
use beardog_errors::BearDogError;
use crate::canonical::traits::{CacheStrategy, TimeoutPolicy};
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
    fn health_check(&self) -> impl std::future::Future<Output = Result<ProviderHealth>> + Send;

    /// Get provider performance metrics
    fn metrics(&self) -> impl std::future::Future<Output = Result<PerformanceMetrics>> + Send;

    /// Get provider capabilities
    fn capabilities(&self) -> Vec<ProviderCapability>;

    /// Initialize the provider with configuration
    fn initialize(&mut self, config: ProviderConfiguration) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Shutdown the provider gracefully
    fn shutdown(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;

    /// Check if provider supports a specific capability
    fn supports_capability(&self, capability: &str) -> bool {
        self.capabilities().iter().any(|c| c.name == capability)
    }

    /// Get provider configuration schema
    fn configuration_schema(&self) -> ConfigurationSchema {
        ConfigurationSchema::default()
    }

    /// Validate provider configuration
    fn validate_configuration(&self, config: &ProviderConfiguration) -> Result<()> {
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

// Implement RetryStrategy trait for RetryConfiguration
impl crate::canonical::traits::RetryStrategy for RetryConfiguration {
    fn max_attempts(&self) -> u32 {
        self.max_retries.max(1) // Ensure at least 1 attempt
    }

    fn delay_for_attempt(&self, attempt: u32) -> std::time::Duration {
        if attempt == 0 {
            return std::time::Duration::ZERO;
        }

        let base_delay = std::time::Duration::from_millis(self.base_delay_ms);
        let max_delay = std::time::Duration::from_millis(self.max_delay_ms);

        let calculated_delay = match self.backoff_strategy {
            BackoffStrategy::Fixed => base_delay,
            BackoffStrategy::Linear => {
                let delay_ms = self.base_delay_ms * (attempt as u64);
                std::time::Duration::from_millis(delay_ms)
            }
            BackoffStrategy::Exponential => {
                let delay_ms = (self.base_delay_ms as f64 * 2.0_f64.powi((attempt - 1) as i32)) as u64;
                std::time::Duration::from_millis(delay_ms)
            }
            BackoffStrategy::Jittered => {
                // Jittered exponential backoff with deterministic jitter
                let base = (self.base_delay_ms as f64 * 2.0_f64.powi((attempt - 1) as i32)) as u64;
                let jitter_percent = (attempt % 10) as f64 * 0.01; // 0-10% variation
                let jitter_factor = 0.95 + jitter_percent; // 95-105%
                std::time::Duration::from_millis((base as f64 * jitter_factor) as u64)
            }
            BackoffStrategy::Custom(_) => {
                // Fallback to exponential for custom strategies
                let delay_ms = (self.base_delay_ms as f64 * 2.0_f64.powi((attempt - 1) as i32)) as u64;
                std::time::Duration::from_millis(delay_ms)
            }
        };

        // Cap at max_delay
        calculated_delay.min(max_delay)
    }

    fn backoff_multiplier(&self) -> f64 {
        match self.backoff_strategy {
            BackoffStrategy::Exponential | BackoffStrategy::Jittered => 2.0,
            BackoffStrategy::Linear => 1.0,
            BackoffStrategy::Fixed => 1.0,
            BackoffStrategy::Custom(_) => 2.0,
        }
    }
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
            session_timeout: std::env::var("BEARDOG_PROVIDER_SESSION_TIMEOUT_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3600), // 1 hour
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
            rotation_interval: std::env::var("BEARDOG_PROVIDER_KEY_ROTATION_INTERVAL_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(86400), // 24 hours
            derivation: KeyDerivationConfiguration::default(),
        }
    }
}

impl Default for KeyDerivationConfiguration {
    fn default() -> Self {
        Self {
            algorithm: KeyDerivationAlgorithm::Pbkdf2,
            iterations: std::env::var("BEARDOG_KEY_DERIVATION_ITERATIONS")
                .ok()
                .and_then(|i| i.parse().ok())
                .unwrap_or(100000), // PBKDF2 recommended iterations
            salt_length: std::env::var("BEARDOG_KEY_DERIVATION_SALT_LENGTH")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(32),
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
            min_size: std::env::var("BEARDOG_CONNECTION_POOL_MIN_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1),
            max_size: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u32,
            connection_timeout: std::env::var("BEARDOG_POOL_CONNECTION_TIMEOUT")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30), // 30 seconds default
            idle_timeout: std::env::var("BEARDOG_POOL_IDLE_TIMEOUT")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(300), // 5 minutes default
        }
    }
}

impl Default for CachingConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            max_size: beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE as u64,
            ttl: std::env::var("BEARDOG_CACHE_TTL_SECS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(3600), // 1 hour default
            eviction_policy: EvictionPolicy::Lru,
        }
    }
}

// Implement CacheStrategy trait for provider caching configuration
impl CacheStrategy for CachingConfiguration {
    fn max_entries(&self) -> usize {
        if !self.enabled {
            return 0;
        }
        self.max_size as usize
    }

    fn ttl(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.ttl)
    }

    fn eviction_policy(&self) -> crate::canonical::traits::cache::EvictionPolicy {
        use crate::canonical::traits::cache::EvictionPolicy as TraitPolicy;
        match self.eviction_policy {
            EvictionPolicy::Lru => TraitPolicy::Lru,
            EvictionPolicy::Lfu => TraitPolicy::Lfu,
            EvictionPolicy::Fifo => TraitPolicy::Fifo,
            EvictionPolicy::Random => TraitPolicy::Random,
            EvictionPolicy::Custom(_) => TraitPolicy::Lru, // Fallback to LRU
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn validate(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }
        if self.max_size == 0 {
            return Err("max_size must be > 0 when caching is enabled".to_string());
        }
        if self.ttl == 0 {
            return Err("TTL cannot be zero".to_string());
        }
        Ok(())
    }

    fn is_production_ready(&self) -> bool {
        if !self.enabled {
            return true;
        }
        self.max_size >= 100 && // At least 100 entries
        self.max_size <= 1_000_000 && // At most 1M entries
        self.ttl >= 60 && // At least 1 minute
        self.ttl <= 86400 && // At most 1 day
        self.validate().is_ok()
    }
}

impl Default for TimeoutConfiguration {
    fn default() -> Self {
        Self {
            request_timeout: std::env::var("BEARDOG_REQUEST_TIMEOUT_SECS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30), // 30 seconds default
            connection_timeout: beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE,
            read_timeout: std::env::var("BEARDOG_READ_TIMEOUT_SECS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30), // 30 seconds default
            write_timeout: std::env::var("BEARDOG_WRITE_TIMEOUT_SECS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30), // 30 seconds default
        }
    }
}

// Implement TimeoutPolicy trait for provider timeout configuration
impl TimeoutPolicy for TimeoutConfiguration {
    fn connection_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.connection_timeout)
    }

    fn operation_timeout(&self, operation: &str) -> std::time::Duration {
        let timeout_secs = match operation {
            "request" => self.request_timeout,
            "connect" | "connection" => self.connection_timeout,
            "read" => self.read_timeout,
            "write" => self.write_timeout,
            _ => self.request_timeout, // Default to request timeout
        };
        std::time::Duration::from_secs(timeout_secs)
    }

    fn should_timeout(&self, elapsed: std::time::Duration, operation: &str) -> bool {
        elapsed >= self.operation_timeout(operation)
    }

    fn global_timeout(&self) -> Option<std::time::Duration> {
        Some(std::time::Duration::from_secs(self.request_timeout))
    }

    fn read_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.read_timeout)
    }

    fn write_timeout(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.write_timeout)
    }

    fn idle_timeout(&self) -> Option<std::time::Duration> {
        None // Provider config doesn't have idle timeout
    }

    fn remaining_time(&self, elapsed: std::time::Duration, operation: &str) -> std::time::Duration {
        let timeout = self.operation_timeout(operation);
        timeout.saturating_sub(elapsed)
    }

    fn validate(&self) -> Result<(), String> {
        if self.connection_timeout == 0 {
            return Err("Connection timeout cannot be zero".to_string());
        }
        if self.request_timeout == 0 {
            return Err("Request timeout cannot be zero".to_string());
        }
        if self.read_timeout == 0 {
            return Err("Read timeout cannot be zero".to_string());
        }
        if self.write_timeout == 0 {
            return Err("Write timeout cannot be zero".to_string());
        }
        Ok(())
    }

    fn is_production_ready(&self) -> bool {
        self.connection_timeout >= 1 &&
        self.connection_timeout <= 60 &&
        self.request_timeout >= 5 &&
        self.read_timeout >= 5 &&
        self.write_timeout >= 5 &&
        self.validate().is_ok()
    }
}

impl Default for RetryConfiguration {
    fn default() -> Self {
        Self {
            max_retries: std::env::var("BEARDOG_MAX_RETRIES")
                .ok()
                .and_then(|r| r.parse().ok())
                .unwrap_or(3), // 3 retries default
            base_delay_ms: beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE as u64,
            max_delay_ms: std::env::var("BEARDOG_MAX_RETRY_DELAY_MS")
                .ok()
                .and_then(|d| d.parse().ok())
                .unwrap_or(10000), // 10 seconds default
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