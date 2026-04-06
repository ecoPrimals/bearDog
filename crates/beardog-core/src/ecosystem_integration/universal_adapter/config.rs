// SPDX-License-Identifier: AGPL-3.0-or-later

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
            connection_timeout_ms: beardog_errors::process_env::var(
                "BEARDOG_ADAPTER_CONNECTION_TIMEOUT_MS",
            )
            .ok()
            .and_then(|t| t.parse().ok())
            .unwrap_or(30000), // 30 seconds default
            request_timeout_ms: beardog_errors::process_env::var(
                "BEARDOG_ADAPTER_REQUEST_TIMEOUT_MS",
            )
            .ok()
            .and_then(|t| t.parse().ok())
            .unwrap_or(10000), // 10 seconds default
            max_retries: 3,
            auto_reconnect: true,
            health_check_interval_secs: 30,
            custom_config: HashMap::new(),
        }
    }
}

/// Individual production features that can be enabled/disabled
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductionFeature {
    /// Full production mode (stricter validation, real crypto)
    ProductionMode,
    /// Enhanced structured logging with trace correlation
    EnhancedLogging,
    /// Prometheus/OpenTelemetry metrics collection
    MetricsCollection,
    /// Circuit breaker for failing downstream services
    CircuitBreaker,
    /// Rate limiting on incoming requests
    RateLimiting,
}

/// Production configuration using a feature-flag set
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionConfig {
    /// Set of enabled production features
    pub enabled_features: Vec<ProductionFeature>,
}

impl ProductionConfig {
    /// Check whether a specific feature is enabled
    #[must_use]
    pub fn is_enabled(&self, feature: ProductionFeature) -> bool {
        self.enabled_features.contains(&feature)
    }

    /// Check whether production mode is active
    #[must_use]
    pub fn production_mode(&self) -> bool {
        self.is_enabled(ProductionFeature::ProductionMode)
    }

    /// Check whether enhanced logging is active
    #[must_use]
    pub fn enhanced_logging(&self) -> bool {
        self.is_enabled(ProductionFeature::EnhancedLogging)
    }

    /// Check whether metrics collection is active
    #[must_use]
    pub fn metrics_enabled(&self) -> bool {
        self.is_enabled(ProductionFeature::MetricsCollection)
    }

    /// Check whether circuit breaker is active
    #[must_use]
    pub fn circuit_breaker_enabled(&self) -> bool {
        self.is_enabled(ProductionFeature::CircuitBreaker)
    }

    /// Check whether rate limiting is active
    #[must_use]
    pub fn rate_limiting_enabled(&self) -> bool {
        self.is_enabled(ProductionFeature::RateLimiting)
    }
}

impl Default for ProductionConfig {
    fn default() -> Self {
        Self {
            enabled_features: vec![
                ProductionFeature::ProductionMode,
                ProductionFeature::EnhancedLogging,
                ProductionFeature::MetricsCollection,
                ProductionFeature::CircuitBreaker,
                ProductionFeature::RateLimiting,
            ],
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
