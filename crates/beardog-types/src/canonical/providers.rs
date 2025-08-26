

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderType {
    Security,
    Hsm,
    Storage,
    Network,
    Compute,
    AI,
    Monitoring,
    Compliance,
    Generic,
}
impl Default for ProviderType {
    fn default() -> Self {
        Self::Generic
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProviderStatus {
    Active,
    Inactive,
    Degraded,
    Failed,
    Maintenance,
    Error,
    Unknown,
}

impl Default for ProviderStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealth {
    pub status: ProviderStatus,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub error_count: u32,
    pub success_rate: f64,
    pub response_time_ms: Option<u64>,
    pub details: Option<String>,
}

impl Default for ProviderHealth {
    fn default() -> Self {
        Self {
            status: ProviderStatus::Unknown,
            last_check: chrono::Utc::now(),
            error_count: 0,
            success_rate: 0.0,
            response_time_ms: None,
            details: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapability {
    pub name: String,
    pub version: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderRegistryEntry {
    pub provider_type: ProviderType,
    pub capabilities: Vec<ProviderCapability>,
}

/// Unified canonical provider configuration
/// Consolidates all provider configuration patterns across the codebase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    // Core identification
    pub provider_id: String,
    pub provider_type: ProviderType,
    pub endpoint: Option<String>,
    
    // Standardized timing configuration
    pub timeouts: TimeoutConfig,
    pub retries: RetryConfig,
    pub circuit_breaker: CircuitBreakerConfig,
    
    // Health monitoring
    pub health_check_interval: Duration,
    
    // Operational settings
    pub enabled: bool,
    pub priority: u32,
    
    // Flexible configuration parameters
    pub config_params: HashMap<String, serde_json::Value>,
    pub metadata: HashMap<String, String>,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            provider_id: "default".to_string(),
            provider_type: ProviderType::Generic,
            endpoint: None,
            timeouts: TimeoutConfig::default(),
            retries: RetryConfig::default(),
            circuit_breaker: CircuitBreakerConfig::default(),
            health_check_interval: Duration::from_secs(60),
            enabled: true,
            priority: 100,
            config_params: HashMap::with_capacity(16),
            metadata: HashMap::with_capacity(8),
        }
    }
}

impl ProviderConfig {
    /// Create a new provider config with basic settings
    pub fn new(provider_id: String, provider_type: ProviderType) -> Self {
        Self {
            provider_id,
            provider_type,
            ..Default::default()
        }
    }
    
    /// Set endpoint for the provider
    pub fn with_endpoint(mut self, endpoint: String) -> Self {
        self.endpoint = Some(endpoint);
        self
    }
    
    /// Set timeout configuration
    pub fn with_timeouts(mut self, timeouts: TimeoutConfig) -> Self {
        self.timeouts = timeouts;
        self
    }
    
    /// Set retry configuration
    pub fn with_retries(mut self, retries: RetryConfig) -> Self {
        self.retries = retries;
        self
    }
    
    /// Set circuit breaker configuration
    pub fn with_circuit_breaker(mut self, circuit_breaker: CircuitBreakerConfig) -> Self {
        self.circuit_breaker = circuit_breaker;
        self
    }
    
    /// Add configuration parameter
    pub fn with_config_param<T: serde::Serialize>(mut self, key: String, value: T) -> Self {
        if let Ok(json_value) = serde_json::to_value(value) {
            self.config_params.insert(key, json_value);
        }
        self
    }
    
    /// Add metadata entry
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

// Legacy compatibility types - to be removed after migration
#[deprecated(note = "Use ProviderConfig instead")]
pub type LegacyProviderConfig = ProviderConfig;

#[deprecated(note = "Use ProviderStatus instead")]
pub type LegacyProviderStatus = ProviderStatus;

/// Consolidated Universal Adapter Configuration
/// Replaces all fragmented UniversalAdapterConfig definitions across crates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalAdapterConfig {
    // Core adapter settings
    pub adapter_id: Option<String>,
    pub adapter_type: String,
    pub enabled: bool,
    
    // Standardized timing configuration
    pub timeouts: TimeoutConfig,
    pub retries: RetryConfig,
    pub circuit_breaker: CircuitBreakerConfig,
    
    // Connection limits
    pub max_concurrent_connections: u32,
    pub max_concurrent_operations: u32,
    
    // Health monitoring
    pub health_check_interval: Duration,
    
    // Discovery and monitoring
    pub auto_discovery: bool,
    pub discovery_interval_seconds: u64,
    pub enable_monitoring: bool,
    pub enable_health_checks: bool,
    
    // Type system and protocol configuration
    pub auto_discover_types: bool,
    pub type_system_configs: HashMap<String, serde_json::Value>,
    pub protocol_configs: HashMap<String, serde_json::Value>,
    
    // Target configuration
    pub target_primal: Option<TargetPrimalConfig>,
    
    // Additional metadata
    pub metadata: HashMap<String, String>,
}

impl Default for UniversalAdapterConfig {
    fn default() -> Self {
        Self {
            adapter_id: None,
            adapter_type: "universal".to_string(),
            enabled: true,
            timeouts: TimeoutConfig::default(),
            retries: RetryConfig::default(),
            circuit_breaker: CircuitBreakerConfig::default(),
            max_concurrent_connections: 100,
            max_concurrent_operations: 100,
            health_check_interval: Duration::from_secs(60),
            auto_discovery: true,
            discovery_interval_seconds: 300,
            enable_monitoring: true,
            enable_health_checks: true,
            auto_discover_types: true,
            type_system_configs: HashMap::with_capacity(16),
            protocol_configs: HashMap::with_capacity(16),
            target_primal: None,
            metadata: HashMap::with_capacity(8),
        }
    }
}

impl UniversalAdapterConfig {
    /// Create a new adapter config with basic settings
    pub fn new(adapter_type: String) -> Self {
        Self {
            adapter_type,
            ..Default::default()
        }
    }
    
    /// Set target primal configuration
    pub fn with_target_primal(mut self, target: TargetPrimalConfig) -> Self {
        self.target_primal = Some(target);
        self
    }
    
    /// Set timeout configuration
    pub fn with_timeouts(mut self, timeouts: TimeoutConfig) -> Self {
        self.timeouts = timeouts;
        self
    }
    
    /// Set retry configuration
    pub fn with_retries(mut self, retries: RetryConfig) -> Self {
        self.retries = retries;
        self
    }
    
    /// Set circuit breaker configuration
    pub fn with_circuit_breaker(mut self, circuit_breaker: CircuitBreakerConfig) -> Self {
        self.circuit_breaker = circuit_breaker;
        self
    }
    
    /// Set discovery configuration
    pub fn with_discovery(mut self, auto_discover: bool, interval_seconds: u64) -> Self {
        self.auto_discovery = auto_discover;
        self.discovery_interval_seconds = interval_seconds;
        self
    }
}

// Legacy compatibility - to be removed after migration
#[deprecated(note = "Use UniversalAdapterConfig instead")]
pub type LegacyUniversalAdapterConfig = UniversalAdapterConfig;

// External services configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalServicesConfig {
    pub songbird_endpoint: Option<String>,
    pub storage_endpoint: Option<String>,
    pub auth_endpoint: Option<String>,
    pub monitoring_endpoint: Option<String>,
}

impl Default for ExternalServicesConfig {
    fn default() -> Self {
        Self {
            songbird_endpoint: None,
            storage_endpoint: None,
            auth_endpoint: None,
            monitoring_endpoint: None,
        }
    }
}

// HSM-specific types
use super::hsm::KeyType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmKeyInfo {
    pub key_id: String,
    pub key_type: KeyType,
    pub metadata: super::hsm::KeyMetadata,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub usage_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHardwareStatus {
    pub available: bool,
    pub temperature: Option<f64>,
    pub free_memory: Option<u64>,
    pub uptime_seconds: Option<u64>,
    pub error_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmInfo {
    pub instance_id: String,
    pub vendor: String,
    pub model: String,
    pub firmware_version: String,
    pub api_version: String,
    pub supported_algorithms: Vec<String>,
    pub max_key_count: u32,
    pub current_key_count: u32,
    pub certification: Option<String>,
    pub tamper_resistant: bool,
}

// Standardized timeout and retry configuration
// Used consistently across all provider and adapter configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeoutConfig {
    pub connect_timeout: Duration,
    pub request_timeout: Duration,
    pub read_timeout: Duration,
    pub write_timeout: Duration,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            connect_timeout: Duration::from_secs(10),
            request_timeout: Duration::from_secs(30),
            read_timeout: Duration::from_secs(30),
            write_timeout: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
    pub jitter_enabled: bool,
    pub retryable_errors: Vec<String>,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter_enabled: true,
            retryable_errors: vec![
                "connection_timeout".to_string(),
                "read_timeout".to_string(),
                "temporary_failure".to_string(),
            ],
        }
    }
}

impl RetryConfig {
    /// Calculate delay for a specific attempt (0-indexed)
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        let base_delay = self.initial_delay.as_millis() as f64;
        let delay_ms = base_delay * self.backoff_multiplier.powi(attempt as i32);
        let delay_ms = delay_ms.min(self.max_delay.as_millis() as f64);
        
        let final_delay = if self.jitter_enabled {
            // Add up to 25% jitter
            let jitter = rand::random::<f64>() * 0.25;
            delay_ms * (1.0 + jitter)
        } else {
            delay_ms
        };
        
        Duration::from_millis(final_delay as u64)
    }
    
    /// Check if an error is retryable
    pub fn is_retryable(&self, error: &str) -> bool {
        self.retryable_errors.iter().any(|e| error.contains(e))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    pub enabled: bool,
    pub failure_threshold: u32,
    pub success_threshold: u32,
    pub timeout: Duration,
    pub half_open_max_calls: u32,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            failure_threshold: 5,
            success_threshold: 2,
            timeout: Duration::from_secs(60),
            half_open_max_calls: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetPrimalConfig {
    pub id: String,
    pub name: String,
    pub endpoint: Option<String>,
    pub authentication: Option<HashMap<String, serde_json::Value>>,
}
