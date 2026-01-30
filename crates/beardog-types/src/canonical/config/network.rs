//! Network Configuration
//!
//! Centralized network configuration for the BearDog ecosystem eliminating hardcoded values.
//!
//! # Overview
//!
//! This module provides comprehensive network configuration including:
//! - Service host and port configuration
//! - Timeout management for connections and requests
//! - Service endpoint URL construction
//! - TLS/SSL configuration
//! - Load balancing and circuit breaker settings
//!
//! # Quick Start
//!
//! ```rust
//! use beardog_types::canonical::config::network::NetworkConfig;
//!
//! // Create network configuration with defaults
//! let config = NetworkConfig::default();
//!
//! // Access service ports
//! assert_eq!(config.service_ports.api_port, 8080);
//! assert_eq!(config.service_ports.admin_port, 9091);
//!
//! // Build service URLs
//! let health_url = config.get_service_url("health", "/status");
//! ```
//!
//! # Custom Configuration
//!
//! ```rust
//! use beardog_types::canonical::config::network::NetworkConfig;
//!
//! // Configure for specific host
//! let config = NetworkConfig::with_host("api.example.com");
//!
//! // Get service-specific URLs
//! let api_url = config.get_service_url("api", "/v1/users");
//! let ws_url = config.get_service_url("websocket", "/stream");
//! ```
//!
//! # Environment Variables
//!
//! Configuration can be controlled via environment variables:
//! - `BEARDOG_SERVICE_HOST` / `BEARDOG_HOST` - Default host
//! - `BEARDOG_API_PORT` - API service port (default: 8080)
//! - `BEARDOG_ADMIN_PORT` - Admin interface port (default: 8081)
//! - `BEARDOG_METRICS_PORT` - Metrics port (default: 9090)
//! - `BEARDOG_WS_PORT` - WebSocket port (default: 8082)
//! - `BEARDOG_HEALTH_PORT` - Health check port (default: 8083)
//! - `BEARDOG_REQUEST_TIMEOUT_MS` - Request timeout (default: 30000)
//! - `BEARDOG_CONNECTION_TIMEOUT_MS` - Connection timeout (default: 5000)
//! - `BEARDOG_KEEPALIVE_TIMEOUT_S` - Keep-alive timeout (default: 60)

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Network configuration for BearDog services
///
/// Comprehensive network settings including hosts, ports, timeouts, and endpoints.
///
/// # Fields
///
/// * `default_host` - Default service host (configurable via `BEARDOG_SERVICE_HOST`)
/// * `service_ports` - Port configuration for all services
/// * `timeouts` - Network timeout settings
/// * `endpoints` - Pre-configured service endpoint URLs
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::network::NetworkConfig;
///
/// let config = NetworkConfig::default();
/// println!("API running on port {}", config.service_ports.api_port);
/// ```
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
///
/// **Deprecated**: Use `BEARDOG_CONFIG.network.api.bind_address` directly instead.
/// This function will be removed in v0.11.0.
#[deprecated(
    since = "0.10.0",
    note = "Use BEARDOG_CONFIG.network.api.bind_address directly"
)]
pub fn default_service_host() -> String {
    std::env::var("BEARDOG_SERVICE_HOST").unwrap_or_else(|_| {
        std::env::var("BEARDOG_HOST").unwrap_or_else(|_| {
            // Use config system instead of hardcoded constant
            use beardog_config::global::BEARDOG_CONFIG;
            BEARDOG_CONFIG.network.api.bind_address.to_string()
        })
    })
}

/// Service port configuration
///
/// Port assignments for all BearDog services. Ports can be customized via
/// environment variables for flexible deployment.
///
/// # Default Ports
///
/// * `api_port` - 8080 (main API)
/// * `admin_port` - 9091 (admin interface)
/// * `metrics_port` - 9100 (Prometheus metrics)
/// * `websocket_port` - 8082 (WebSocket connections)
/// * `health_port` - 8083 (health checks)
/// * `compute_port` - 8001 (compute service)
/// * `mesh_port` - 8002 (service mesh)
/// * `ai_port` - 8003 (AI service)
/// * `storage_port` - 8004 (storage service)
/// * `security_port` - 8005 (security service)
/// * `database_port` - 5432 (database)
/// * `grafana_port` - 3000 (Grafana)
/// * `jaeger_port` - 14268 (Jaeger)
/// * `discovery_port` - 8080 (discovery service)
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::network::ServicePorts;
///
/// let ports = ServicePorts::default();
/// assert_eq!(ports.api_port, 8080);
/// assert_eq!(ports.metrics_port, 9100);
/// ```
///
/// # Environment Configuration
///
/// Override defaults with environment variables:
/// ```bash
/// export BEARDOG_API_PORT=9000
/// export BEARDOG_ADMIN_PORT=9001
/// export BEARDOG_COMPUTE_PORT=8001
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePorts {
    /// Main API port
    /// Number of `api_port`
    pub api_port: u16,
    /// Admin interface port
    /// Number of `admin_port`
    pub admin_port: u16,
    /// Debug/diagnostics port
    /// Number of `debug_port`
    pub debug_port: u16,
    /// Metrics port
    /// Number of `metrics_port`
    pub metrics_port: u16,
    /// WebSocket port
    /// Number of `websocket_port`
    pub websocket_port: u16,
    /// Health check port
    /// Number of `health_port`
    pub health_port: u16,
    /// Compute service port
    pub compute_port: u16,
    /// Service mesh port
    pub mesh_port: u16,
    /// AI service port
    pub ai_port: u16,
    /// Storage service port
    pub storage_port: u16,
    /// Security service port
    pub security_port: u16,
    /// Database port
    pub database_port: u16,
    /// Grafana port
    pub grafana_port: u16,
    /// Jaeger port
    pub jaeger_port: u16,
    /// Discovery service port
    pub discovery_port: u16,
}

impl Default for ServicePorts {
    fn default() -> Self {
        use beardog_config::domains::network_ports::{
            DEFAULT_ADMIN_PORT, DEFAULT_AI_PORT, DEFAULT_API_PORT, DEFAULT_COMPUTE_PORT,
            DEFAULT_DATABASE_PORT, DEFAULT_DEBUG_PORT, DEFAULT_DISCOVERY_PORT, DEFAULT_GRAFANA_PORT,
            DEFAULT_HEALTH_PORT, DEFAULT_JAEGER_PORT, DEFAULT_MESH_PORT, DEFAULT_METRICS_PORT,
            DEFAULT_SECURITY_PORT, DEFAULT_STORAGE_PORT, DEFAULT_WEBSOCKET_PORT,
        };

        Self {
            api_port: std::env::var("BEARDOG_API_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_API_PORT),
            admin_port: std::env::var("BEARDOG_ADMIN_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_ADMIN_PORT),
            debug_port: std::env::var("BEARDOG_DEBUG_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_DEBUG_PORT),
            metrics_port: std::env::var("BEARDOG_METRICS_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_METRICS_PORT),
            websocket_port: std::env::var("BEARDOG_WS_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_WEBSOCKET_PORT),
            health_port: std::env::var("BEARDOG_HEALTH_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_HEALTH_PORT),
            compute_port: std::env::var("BEARDOG_COMPUTE_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_COMPUTE_PORT),
            mesh_port: std::env::var("BEARDOG_MESH_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_MESH_PORT),
            ai_port: std::env::var("BEARDOG_AI_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_AI_PORT),
            storage_port: std::env::var("BEARDOG_STORAGE_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_STORAGE_PORT),
            security_port: std::env::var("BEARDOG_SECURITY_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_SECURITY_PORT),
            database_port: std::env::var("BEARDOG_DATABASE_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_DATABASE_PORT),
            grafana_port: std::env::var("BEARDOG_GRAFANA_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_GRAFANA_PORT),
            jaeger_port: std::env::var("BEARDOG_JAEGER_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_JAEGER_PORT),
            discovery_port: std::env::var("BEARDOG_DISCOVERY_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_DISCOVERY_PORT),
        }
    }
}

/// Network timeout configuration
///
/// Configurable timeouts for network operations to prevent hung connections
/// and ensure responsive behavior.
///
/// # Fields
///
/// * `request_timeout_ms` - Maximum time for a request to complete (default: 30000ms / 30s)
/// * `connection_timeout_ms` - Maximum time to establish a connection (default: 5000ms / 5s)
/// * `keepalive_timeout_s` - How long to keep idle connections alive (default: 60s)
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::network::NetworkTimeouts;
///
/// let timeouts = NetworkTimeouts::default();
/// assert_eq!(timeouts.request_timeout_ms, 30000);  // 30 seconds
/// assert_eq!(timeouts.connection_timeout_ms, 5000);  // 5 seconds
/// ```
///
/// # Performance Tuning
///
/// - Short `connection_timeout_ms` (< 5s): Fail fast on connection issues
/// - Long `request_timeout_ms` (> 30s): For slow operations (large uploads, etc.)
/// - `keepalive_timeout_s`: Balance between connection reuse and resource usage
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
// Re-export from domains::network (consolidated Nov 7, 2025)
pub use super::domains::network::RateLimitConfig;

// Compatibility aliases REMOVED:
// - EndpointConfig was deprecated (3.2.0) - Use CanonicalNetworkConfig directly
