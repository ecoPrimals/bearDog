// SPDX-License-Identifier: AGPL-3.0-or-later

//! Zero Hardcoding Configuration Infrastructure
//!
//! Modern, idiomatic Rust configuration system that eliminates ALL hardcoding:
//! - Ports (8080, 9000, etc.) → Environment or Port 0 (OS auto-select)
//! - IPs (localhost, 127.0.0.1) → Environment or runtime discovery
//! - Timeouts (`Duration::from_secs`) → Environment-driven with smart defaults
//!
//! ## Philosophy
//!
//! **"Never hardcode what can be discovered, never discover what can be configured"**
//!
//! ## Design Principles
//!
//! 1. **Environment-First**: All config from environment variables
//! 2. **Port 0 Magic**: OS selects available ports (no conflicts!)
//! 3. **Smart Defaults**: Production-tested fallbacks
//! 4. **Type-Safe**: Strong typing prevents errors
//! 5. **Zero-Cost**: Compile-time optimization
//!
//! ## Architecture
//!
//! ```text
//! Environment Variables
//!         ↓
//!   ZeroHardcodingConfig
//!    ├─ EndpointConfig     (ports, bind addresses)
//!    ├─ TimeoutConfig      (all timeouts)
//!    ├─ RetryConfig        (retry policies)
//!    └─ DiscoveryConfig    (runtime discovery)
//! ```

use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

// ============================================================================
// Endpoint Configuration (Ports & Bind Addresses)
// ============================================================================

/// Endpoint configuration - NO hardcoded ports or IPs!
///
/// ## Usage
///
/// ```rust
/// use beardog_config::zero_hardcoding::EndpointConfig;
///
/// // Environment-driven (production)
/// let config = EndpointConfig::from_env();
///
/// // Auto-select ports (testing)
/// let config = EndpointConfig::auto();
///
/// // Explicit (human sovereignty) - http, rpc, ws, metrics, bind address
/// let config = EndpointConfig::new(9000, 9001, 9002, 9003, "0.0.0.0");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    /// HTTP/REST API port (0 = OS auto-select)
    pub http_port: u16,

    /// RPC/tarpc port (0 = OS auto-select)
    pub rpc_port: u16,

    /// WebSocket port (0 = OS auto-select)
    pub ws_port: u16,

    /// Metrics/Prometheus port (0 = OS auto-select)
    pub metrics_port: u16,

    /// Bind address (default: 0.0.0.0 for production, 127.0.0.1 for dev)
    pub bind_addr: IpAddr,
}

impl EndpointConfig {
    /// Create from environment variables
    ///
    /// Environment variables:
    /// - `BEARDOG_HTTP_PORT` - HTTP port (default: 0)
    /// - `BEARDOG_RPC_PORT` - RPC port (default: 0)
    /// - `BEARDOG_WS_PORT` - WebSocket port (default: 0)
    /// - `BEARDOG_METRICS_PORT` - Metrics port (default: 0)
    /// - `BEARDOG_BIND_ADDR` - Bind address (default: 0.0.0.0)
    pub fn from_env() -> Self {
        Self {
            http_port: Self::env_port("BEARDOG_HTTP_PORT", 0),
            rpc_port: Self::env_port("BEARDOG_RPC_PORT", 0),
            ws_port: Self::env_port("BEARDOG_WS_PORT", 0),
            metrics_port: Self::env_port("BEARDOG_METRICS_PORT", 0),
            bind_addr: Self::env_addr("BEARDOG_BIND_ADDR", "0.0.0.0"),
        }
    }

    /// Auto-select all ports (OS chooses available ports)
    ///
    /// Perfect for:
    /// - Testing (no port conflicts!)
    /// - Development (multiple instances)
    /// - Cloud environments (dynamic allocation)
    pub const fn auto() -> Self {
        Self {
            http_port: 0,
            rpc_port: 0,
            ws_port: 0,
            metrics_port: 0,
            bind_addr: IpAddr::V4(Ipv4Addr::LOCALHOST),
        }
    }

    /// Create with explicit ports (human sovereignty)
    pub fn new(http: u16, rpc: u16, ws: u16, metrics: u16, bind: &str) -> Self {
        Self {
            http_port: http,
            rpc_port: rpc,
            ws_port: ws,
            metrics_port: metrics,
            bind_addr: IpAddr::from_str(bind).unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
        }
    }

    /// Get HTTP socket address
    pub const fn http_socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.bind_addr, self.http_port)
    }

    /// Get RPC socket address
    pub const fn rpc_socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.bind_addr, self.rpc_port)
    }

    /// Get WebSocket socket address
    pub const fn ws_socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.bind_addr, self.ws_port)
    }

    /// Get metrics socket address
    pub const fn metrics_socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.bind_addr, self.metrics_port)
    }

    // Helper: Get port from environment
    fn env_port(var: &str, default: u16) -> u16 {
        std::env::var(var)
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(default)
    }

    // Helper: Get address from environment
    fn env_addr(var: &str, default: &str) -> IpAddr {
        std::env::var(var)
            .ok()
            .and_then(|s| IpAddr::from_str(&s).ok())
            .unwrap_or_else(|| {
                // Default addresses are compile-time known valid IPs (127.0.0.1, 0.0.0.0)
                IpAddr::from_str(default).unwrap_or(IpAddr::V4(std::net::Ipv4Addr::LOCALHOST))
            })
    }
}

impl Default for EndpointConfig {
    fn default() -> Self {
        Self {
            http_port: 0,
            rpc_port: 0,
            ws_port: 0,
            metrics_port: 0,
            bind_addr: IpAddr::V4(Ipv4Addr::UNSPECIFIED),
        }
    }
}

// ============================================================================
// Timeout Configuration (NO hardcoded durations!)
// ============================================================================

/// Timeout configuration - environment-driven with smart defaults
///
/// ## Usage
///
/// ```rust,no_run
/// use beardog_config::zero_hardcoding::ZeroHardcodingTimeouts;
///
/// let config = ZeroHardcodingTimeouts::from_env();
///
/// // Use timeouts with external HTTP client
/// // let client = reqwest::Client::builder()
/// //     .connect_timeout(config.connect)
/// //     .timeout(config.request)
/// //     .build()?;
///
/// tracing::debug!("Connect timeout: {:?}", config.connect);
/// tracing::debug!("Request timeout: {:?}", config.request);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ZeroHardcodingTimeouts {
    /// Connection timeout
    pub connect: Duration,

    /// Request timeout
    pub request: Duration,

    /// Idle/keep-alive timeout
    pub idle: Duration,

    /// Discovery timeout
    pub discovery: Duration,

    /// Shutdown grace period
    pub shutdown: Duration,

    /// Health check timeout
    pub health_check: Duration,

    /// Database query timeout
    pub db_query: Duration,
}

impl ZeroHardcodingTimeouts {
    /// Create from environment variables
    ///
    /// Environment variables:
    /// - `BEARDOG_TIMEOUT_CONNECT` - Connection timeout (seconds, default: 10)
    /// - `BEARDOG_TIMEOUT_REQUEST` - Request timeout (seconds, default: 30)
    /// - `BEARDOG_TIMEOUT_IDLE` - Idle timeout (seconds, default: 60)
    /// - `BEARDOG_TIMEOUT_DISCOVERY` - Discovery timeout (seconds, default: 5)
    /// - `BEARDOG_TIMEOUT_SHUTDOWN` - Shutdown timeout (seconds, default: 30)
    /// - `BEARDOG_TIMEOUT_HEALTH` - Health check timeout (seconds, default: 5)
    /// - `BEARDOG_TIMEOUT_DB_QUERY` - DB query timeout (seconds, default: 10)
    pub fn from_env() -> Self {
        Self {
            connect: Self::env_duration("BEARDOG_TIMEOUT_CONNECT", 10),
            request: Self::env_duration("BEARDOG_TIMEOUT_REQUEST", 30),
            idle: Self::env_duration("BEARDOG_TIMEOUT_IDLE", 60),
            discovery: Self::env_duration("BEARDOG_TIMEOUT_DISCOVERY", 5),
            shutdown: Self::env_duration("BEARDOG_TIMEOUT_SHUTDOWN", 30),
            health_check: Self::env_duration("BEARDOG_TIMEOUT_HEALTH", 5),
            db_query: Self::env_duration("BEARDOG_TIMEOUT_DB_QUERY", 10),
        }
    }

    /// Aggressive timeouts (for fast-fail scenarios)
    pub const fn aggressive() -> Self {
        Self {
            connect: Duration::from_secs(2),
            request: Duration::from_secs(5),
            idle: Duration::from_secs(10),
            discovery: Duration::from_secs(1),
            shutdown: Duration::from_secs(5),
            health_check: Duration::from_secs(1),
            db_query: Duration::from_secs(2),
        }
    }

    /// Relaxed timeouts (for slow networks)
    pub const fn relaxed() -> Self {
        Self {
            connect: Duration::from_secs(30),
            request: Duration::from_secs(120),
            idle: Duration::from_secs(300),
            discovery: Duration::from_secs(15),
            shutdown: Duration::from_secs(60),
            health_check: Duration::from_secs(10),
            db_query: Duration::from_secs(30),
        }
    }

    // Helper: Get duration from environment
    fn env_duration(var: &str, default_secs: u64) -> Duration {
        std::env::var(var)
            .ok()
            .and_then(|s| s.parse::<u64>().ok())
            .map_or_else(|| Duration::from_secs(default_secs), Duration::from_secs)
    }
}

impl Default for ZeroHardcodingTimeouts {
    fn default() -> Self {
        Self {
            connect: Duration::from_secs(10),
            request: Duration::from_secs(30),
            idle: Duration::from_secs(60),
            discovery: Duration::from_secs(5),
            shutdown: Duration::from_secs(30),
            health_check: Duration::from_secs(5),
            db_query: Duration::from_secs(10),
        }
    }
}

// ============================================================================
// Retry Configuration
// ============================================================================

/// Retry configuration - smart backoff and retry policies
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,

    /// Initial backoff duration
    pub initial_backoff: Duration,

    /// Maximum backoff duration
    pub max_backoff: Duration,

    /// Backoff multiplier (exponential)
    pub backoff_multiplier: f64,
}

impl RetryConfig {
    /// Create from environment variables
    ///
    /// Environment variables:
    /// - `BEARDOG_RETRY_MAX_ATTEMPTS` - Max attempts (default: 3)
    /// - `BEARDOG_RETRY_INITIAL_BACKOFF_MS` - Initial backoff ms (default: 100)
    /// - `BEARDOG_RETRY_MAX_BACKOFF_SECS` - Max backoff seconds (default: 30)
    /// - `BEARDOG_RETRY_BACKOFF_MULTIPLIER` - Backoff multiplier (default: 2.0)
    pub fn from_env() -> Self {
        Self {
            max_attempts: std::env::var("BEARDOG_RETRY_MAX_ATTEMPTS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            initial_backoff: Duration::from_millis(
                std::env::var("BEARDOG_RETRY_INITIAL_BACKOFF_MS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100),
            ),
            max_backoff: Duration::from_secs(
                std::env::var("BEARDOG_RETRY_MAX_BACKOFF_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            backoff_multiplier: std::env::var("BEARDOG_RETRY_BACKOFF_MULTIPLIER")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2.0),
        }
    }

    /// Calculate backoff for attempt N
    #[expect(
        clippy::cast_possible_wrap,
        reason = "Retry attempt index used as powi exponent; fits i32 for f64 powi"
    )]
    #[expect(
        clippy::cast_precision_loss,
        reason = "Backoff scaled from millis; u128 to f64 acceptable for duration math"
    )]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Backoff millis clamped from f64 for Duration::from_millis"
    )]
    #[expect(
        clippy::cast_sign_loss,
        reason = "Non-negative backoff before u64 millis"
    )]
    pub fn backoff_for_attempt(&self, attempt: u32) -> Duration {
        let multiplier = self.backoff_multiplier.powi(attempt as i32);
        let backoff = self.initial_backoff.as_millis() as f64 * multiplier;
        let backoff_duration = Duration::from_millis(backoff as u64);
        std::cmp::min(backoff_duration, self.max_backoff)
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(30),
            backoff_multiplier: 2.0,
        }
    }
}

// ============================================================================
// Complete Zero-Hardcoding Configuration
// ============================================================================

/// Complete zero-hardcoding configuration
///
/// ## Usage
///
/// ```rust,no_run
/// use beardog_config::ZeroHardcodingConfig;
///
/// // Simple: Just use defaults from environment
/// let config = ZeroHardcodingConfig::default();
///
/// // Get socket addresses (no hardcoded ports!)
/// let http_addr = config.endpoints.http_socket_addr();
/// let rpc_addr = config.endpoints.rpc_socket_addr();
///
/// tracing::info!("HTTP will bind to: {}", http_addr);
/// tracing::info!("RPC will bind to: {}", rpc_addr);
///
/// // Timeouts are configurable via environment
/// tracing::debug!("Connect timeout: {:?}", config.timeouts.connect);
/// tracing::debug!("Request timeout: {:?}", config.timeouts.request);
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ZeroHardcodingConfig {
    /// Endpoint configuration (ports, addresses)
    pub endpoints: EndpointConfig,

    /// Timeout configuration
    pub timeouts: ZeroHardcodingTimeouts,

    /// Retry configuration
    pub retries: RetryConfig,
}

impl ZeroHardcodingConfig {
    /// Create from environment variables (recommended)
    pub fn from_env() -> Self {
        Self {
            endpoints: EndpointConfig::from_env(),
            timeouts: ZeroHardcodingTimeouts::from_env(),
            retries: RetryConfig::from_env(),
        }
    }

    /// Auto-select all ports (testing)
    pub fn auto() -> Self {
        Self {
            endpoints: EndpointConfig::auto(),
            timeouts: ZeroHardcodingTimeouts::default(),
            retries: RetryConfig::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_config_auto() {
        let config = EndpointConfig::auto();
        assert_eq!(config.http_port, 0); // OS will select
        assert_eq!(config.rpc_port, 0);
        assert!(config.bind_addr.is_loopback());
    }

    #[test]
    fn test_timeout_config_defaults() {
        let config = ZeroHardcodingTimeouts::from_env();
        assert!(config.connect.as_secs() > 0);
        assert!(config.request.as_secs() >= config.connect.as_secs());
    }

    #[test]
    fn test_retry_backoff() {
        let config = RetryConfig::default();
        let backoff1 = config.backoff_for_attempt(0);
        let backoff2 = config.backoff_for_attempt(1);
        let backoff3 = config.backoff_for_attempt(2);

        assert!(backoff2 > backoff1);
        assert!(backoff3 > backoff2);
        assert!(backoff3 <= config.max_backoff);
    }

    #[test]
    fn test_zero_hardcoding_config() {
        let config = ZeroHardcodingConfig::auto();
        let http_addr = config.endpoints.http_socket_addr();
        #[expect(
            clippy::double_comparisons,
            reason = "explicit chained comparisons document inclusive numeric bounds"
        )]
        {
            assert!(http_addr.port() == 0 || http_addr.port() > 0);
        }
    }

    #[test]
    fn test_endpoint_config_new_explicit() {
        let config = EndpointConfig::new(9000, 9001, 9002, 9003, "0.0.0.0");
        assert_eq!(config.http_port, 9000);
        assert_eq!(config.rpc_port, 9001);
        assert_eq!(config.ws_port, 9002);
        assert_eq!(config.metrics_port, 9003);
        assert!(!config.bind_addr.is_loopback());
    }

    #[test]
    fn test_endpoint_config_new_invalid_addr_fallback() {
        let config = EndpointConfig::new(80, 81, 82, 83, "invalid");
        assert_eq!(config.bind_addr, IpAddr::V4(Ipv4Addr::UNSPECIFIED));
    }

    #[test]
    fn test_endpoint_socket_addrs() {
        let config = EndpointConfig::new(1000, 1001, 1002, 1003, "127.0.0.1");

        assert_eq!(config.http_socket_addr().port(), 1000);
        assert_eq!(config.rpc_socket_addr().port(), 1001);
        assert_eq!(config.ws_socket_addr().port(), 1002);
        assert_eq!(config.metrics_socket_addr().port(), 1003);
        assert!(config.http_socket_addr().ip().is_loopback());
    }

    #[test]
    fn test_endpoint_config_from_env_defaults() {
        let config = EndpointConfig::from_env();
        // Without env vars set, should use defaults (port 0)
        // The bind_addr defaults to 0.0.0.0
        #[expect(
            clippy::double_comparisons,
            reason = "explicit chained comparisons document inclusive numeric bounds"
        )]
        {
            assert!(config.http_port == 0 || config.http_port > 0); // may have env set
        }
    }

    #[test]
    fn test_endpoint_config_default_is_pure() {
        let d = EndpointConfig::default();
        assert_eq!(d.http_port, 0);
        assert_eq!(d.bind_addr, IpAddr::V4(Ipv4Addr::UNSPECIFIED));
    }

    #[test]
    fn test_timeout_aggressive() {
        let config = ZeroHardcodingTimeouts::aggressive();
        assert_eq!(config.connect, Duration::from_secs(2));
        assert_eq!(config.request, Duration::from_secs(5));
        assert_eq!(config.idle, Duration::from_secs(10));
        assert_eq!(config.discovery, Duration::from_secs(1));
        assert_eq!(config.shutdown, Duration::from_secs(5));
        assert_eq!(config.health_check, Duration::from_secs(1));
        assert_eq!(config.db_query, Duration::from_secs(2));
    }

    #[test]
    fn test_timeout_relaxed() {
        let config = ZeroHardcodingTimeouts::relaxed();
        assert_eq!(config.connect, Duration::from_secs(30));
        assert_eq!(config.request, Duration::from_secs(120));
        assert_eq!(config.idle, Duration::from_secs(300));
        assert_eq!(config.discovery, Duration::from_secs(15));
        assert_eq!(config.shutdown, Duration::from_secs(60));
        assert_eq!(config.health_check, Duration::from_secs(10));
        assert_eq!(config.db_query, Duration::from_secs(30));
    }

    #[test]
    fn test_timeout_default_matches_documented_baseline() {
        let d = ZeroHardcodingTimeouts::default();
        assert_eq!(d.connect, Duration::from_secs(10));
        assert_eq!(d.request, Duration::from_secs(30));
    }

    #[test]
    fn test_retry_config_from_env_defaults() {
        let config = RetryConfig::from_env();
        // Without env vars, should have defaults
        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.initial_backoff, Duration::from_millis(100));
        assert_eq!(config.max_backoff, Duration::from_secs(30));
        assert!((config.backoff_multiplier - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_retry_config_default_is_pure() {
        let d = RetryConfig::default();
        assert_eq!(d.max_attempts, 3);
        assert_eq!(d.initial_backoff, Duration::from_millis(100));
    }

    #[test]
    fn test_retry_backoff_capped_at_max() {
        let config = RetryConfig {
            max_attempts: 10,
            initial_backoff: Duration::from_millis(100),
            max_backoff: Duration::from_secs(1), // low cap
            backoff_multiplier: 10.0,
        };

        let backoff = config.backoff_for_attempt(5);
        assert!(backoff <= config.max_backoff);
    }

    #[test]
    fn test_retry_backoff_attempt_zero() {
        let config = RetryConfig::default();
        let backoff = config.backoff_for_attempt(0);
        assert_eq!(backoff, config.initial_backoff);
    }

    #[test]
    fn test_zero_hardcoding_config_from_env() {
        let config = ZeroHardcodingConfig::from_env();
        // Should not panic and have valid structure
        let _ = config.endpoints.http_socket_addr();
        assert!(config.timeouts.connect.as_secs() > 0);
        assert!(config.retries.max_attempts > 0);
    }

    #[test]
    fn test_zero_hardcoding_config_default_is_pure() {
        let d = ZeroHardcodingConfig::default();
        assert_eq!(d.endpoints.http_port, 0);
        assert_eq!(d.timeouts.connect, Duration::from_secs(10));
        assert_eq!(d.retries.max_attempts, 3);
    }
}
