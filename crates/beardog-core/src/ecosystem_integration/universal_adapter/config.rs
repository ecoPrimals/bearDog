// Configuration for Universal Adapter System

use super::types::ProtocolType;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Universal adapter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalAdapterConfig {
    /// Adapter identifier
    pub adapter_id: String,
    /// Adapter name
    /// Name of the adapter
    pub adapter_name: String,
    /// Supported protocols
    /// Collection of supported protocols
    pub supported_protocols: Vec<ProtocolType>,
    /// Connection timeout in milliseconds
    pub connection_timeout_ms: u64,
    /// Request timeout in milliseconds
    pub request_timeout_ms: u64,
    /// Maximum retry attempts
    /// Number of `max_retries`
    pub max_retries: u32,
    /// Enable automatic reconnection
    /// Whether `auto_reconnect` is enabled
    pub auto_reconnect: bool,
    /// Health check interval in seconds
    /// Number of `health_check_interval_secs`
    pub health_check_interval_secs: u64,
    /// Custom configuration parameters
    pub custom_config: HashMap<String, serde_json::Value>,
}

impl Default for UniversalAdapterConfig {
    fn default() -> Self {
        Self {
            adapter_id: "universal-adapter".to_string(),
            adapter_name: "Universal Ecosystem Adapter".to_string(),
            supported_protocols: vec![
                ProtocolType::Http,
                ProtocolType::WebSocket,
                ProtocolType::Grpc,
            ],
            connection_timeout_ms: 30000,
            request_timeout_ms: 10000,
            max_retries: 3,
            auto_reconnect: true,
            health_check_interval_secs: 30,
            custom_config: HashMap::new(),
        }
    }
}

/// Production configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ProductionConfig {
    /// Enable production features
    /// Whether `production_mode` is enabled
    pub production_mode: bool,
    /// Enhanced logging
    /// Whether `enhanced_logging` is enabled
    pub enhanced_logging: bool,
    /// Metrics collection
    /// Whether metrics is enabled
    pub metrics_enabled: bool,
    /// Circuit breaker enabled
    /// Whether `circuit_breaker` is enabled
    pub circuit_breaker_enabled: bool,
    /// Rate limiting enabled
    /// Whether `rate_limiting` is enabled
    pub rate_limiting_enabled: bool,
}

impl Default for ProductionConfig {
    fn default() -> Self {
        Self {
            production_mode: true,
            enhanced_logging: true,
            metrics_enabled: true,
            circuit_breaker_enabled: true,
            rate_limiting_enabled: true,
        }
    }
}

/// Connection pool configuration (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `beardog_types::canonical::config::domains::network::ConnectionPoolConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use beardog_types::canonical::config::domains::network::ConnectionPoolConfig instead"
)]
pub type PoolConfig = beardog_types::canonical::config::domains::network::ConnectionPoolConfig;
