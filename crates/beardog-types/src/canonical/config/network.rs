// Network configuration types for BearDog ecosystem
// Centralizes all network-related configuration to eliminate hardcoded values

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Default service host (replaces hardcoded "localhost")
    /// The default host value
    pub default_host: String,
    /// Service-specific port mappings
    /// The service ports value
    pub service_ports: ServicePorts,
    /// Timeout configurations
    pub timeouts: NetworkTimeouts,
    /// Service endpoints
    /// The endpoints value
    pub endpoints: ServiceEndpoints,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            default_host: default_service_host(),
            service_ports: ServicePorts::default(),
            timeouts: NetworkTimeouts::default(),
            endpoints: ServiceEndpoints::default(),
        }
    }
}

/// Default service host (configurable via `BEARDOG_SERVICE_HOST` environment variable)
pub fn default_service_host() -> String {
    std::env::var("BEARDOG_SERVICE_HOST").unwrap_or_else(|_| {
        std::env::var("BEARDOG_HOST").unwrap_or_else(|_| "localhost".to_string())
    })
}

/// Service port configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePorts {
    /// Main API port
    /// Number of `api_port`
    pub api_port: u16,
    /// Admin interface port
    /// Number of `admin_port`
    pub admin_port: u16,
    /// Metrics port
    /// Number of `metrics_port`
    pub metrics_port: u16,
    /// WebSocket port
    /// Number of `websocket_port`
    pub websocket_port: u16,
    /// Health check port
    /// Number of `health_port`
    pub health_port: u16,
}

impl Default for ServicePorts {
    fn default() -> Self {
        Self {
            api_port: std::env::var("BEARDOG_API_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            admin_port: std::env::var("BEARDOG_ADMIN_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8081),
            metrics_port: std::env::var("BEARDOG_METRICS_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(9090),
            websocket_port: std::env::var("BEARDOG_WS_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8082),
            health_port: std::env::var("BEARDOG_HEALTH_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8083),
        }
    }
}

/// Network timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTimeouts {
    /// Request timeout in milliseconds
    pub request_timeout_ms: u64,
    /// Connection timeout in milliseconds
    pub connection_timeout_ms: u64,
    /// Keep-alive timeout in seconds
    pub keepalive_timeout_s: u64,
}

impl Default for NetworkTimeouts {
    fn default() -> Self {
        Self {
            request_timeout_ms: std::env::var("BEARDOG_REQUEST_TIMEOUT_MS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30000),
            connection_timeout_ms: std::env::var("BEARDOG_CONNECTION_TIMEOUT_MS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(5000),
            keepalive_timeout_s: std::env::var("BEARDOG_KEEPALIVE_TIMEOUT_S")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(60),
        }
    }
}

/// Service endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoints {
    /// The capabilities url value
    pub capabilities_url: String,
    /// Health check endpoint URL
    /// The health url value
    pub health_url: String,
    /// Metrics endpoint URL
    /// The metrics url value
    pub metrics_url: String,
    /// Admin interface URL
    /// The admin url value
    pub admin_url: String,
    /// WebSocket endpoint URL
    /// The websocket url value
    pub websocket_url: String,
}

impl Default for ServiceEndpoints {
    fn default() -> Self {
        let host = default_service_host();
        let ports = ServicePorts::default();

        Self {
            capabilities_url: std::env::var("BEARDOG_CAPABILITIES_URL").unwrap_or_else(|_| {
                format!("http://{}:{}/api/v1/capabilities", host, ports.api_port)
            }),
            health_url: std::env::var("BEARDOG_HEALTH_URL")
                .unwrap_or_else(|_| format!("http://{}:{}/health", host, ports.health_port)),
            metrics_url: std::env::var("BEARDOG_METRICS_URL")
                .unwrap_or_else(|_| format!("http://{}:{}/metrics", host, ports.metrics_port)),
            admin_url: std::env::var("BEARDOG_ADMIN_URL")
                .unwrap_or_else(|_| format!("http://{}:{}/admin", host, ports.admin_port)),
            websocket_url: std::env::var("BEARDOG_WEBSOCKET_URL")
                .unwrap_or_else(|_| format!("ws://{}:{}/ws", host, ports.websocket_port)),
        }
    }
}

impl NetworkConfig {
    /// Create network config with custom host
    #[must_use]
    /// Creates instance with host
    pub fn with_host(host: &str) -> Self {
        Self {
            default_host: host.to_string(),
            endpoints: ServiceEndpoints::with_host(host),
            ..Self::default()
        }
    }

    #[must_use]
    /// Gets `service_url`
    /// Gets `service_url`
    pub fn get_service_url(&self, service: &str, path: &str) -> String {
        let port = match service {
            "admin" => self.service_ports.admin_port,
            "metrics" => self.service_ports.metrics_port,
            "health" => self.service_ports.health_port,
            "websocket" => self.service_ports.websocket_port,
            _ => self.service_ports.api_port,
        };

        let protocol = if service == "websocket" { "ws" } else { "http" };
        format!("{}://{}:{}{}", protocol, self.default_host, port, path)
    }
}

impl ServiceEndpoints {
    /// Create endpoints with custom host
    #[must_use]
    /// Creates instance with host
    pub fn with_host(host: &str) -> Self {
        let ports = ServicePorts::default();

        Self {
            capabilities_url: format!("http://{}:{}/api/v1/capabilities", host, ports.api_port),
            health_url: format!("http://{}:{}/health", host, ports.health_port),
            metrics_url: format!("http://{}:{}/metrics", host, ports.metrics_port),
            admin_url: format!("http://{}:{}/admin", host, ports.admin_port),
            websocket_url: format!("ws://{}:{}/ws", host, ports.websocket_port),
        }
    }
}

// Legacy compatibility types
/// Canonical network configuration - consolidates all network-related configs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CanonicalNetworkConfig {
    /// Network enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Bind address
    /// The bind address value
    pub bind_address: String,
    /// Port
    /// Number of port
    pub port: u16,
    /// Connection timeout
    pub timeout: Duration,
    /// Maximum connections
    /// Number of `max_connections`
    pub max_connections: usize,
    /// TLS configuration
    /// The tls value
    pub tls: TlsConfig,
    /// Circuit breaker configuration
    /// The circuit breaker value
    pub circuit_breaker: CircuitBreakerConfig,
    /// Load balancing configuration
    /// The load balancing value
    pub load_balancing: LoadBalancingConfig,
    /// Rate limiting configuration
    /// The rate limiting value
    pub rate_limiting: super::domains::network::RateLimitConfig,
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TlsConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Cert Path
    /// Optional cert path
    pub cert_path: Option<String>,
    /// Key Path
    /// Optional key path
    pub key_path: Option<String>,
    /// Ca Path
    /// Optional ca path
    pub ca_path: Option<String>,
}

/// Circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CircuitBreakerConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Failure Threshold
    /// Number of `failure_threshold`
    pub failure_threshold: u32,
    /// Timeout
    pub timeout: Duration,
}

/// Load balancing configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoadBalancingConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Strategy
    /// The strategy value
    pub strategy: LoadBalancingStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub enum LoadBalancingStrategy {
    /// Round-robin distribution strategy
    #[default]
    /// Represents round robin variant
    RoundRobin,
    /// `LeastConnections` variant
    LeastConnections,
    /// Random variant
    Random,
}

/// Rate limiting configuration (DEPRECATED - use canonical domains/network)
///
/// **MIGRATION**: Use `super::domains::network::RateLimitConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use super::domains::network::RateLimitConfig instead"
)]
pub type RateLimitConfig = super::domains::network::RateLimitConfig;

// Compatibility aliases REMOVED:
// - EndpointConfig was deprecated (3.2.0) - Use CanonicalNetworkConfig directly
