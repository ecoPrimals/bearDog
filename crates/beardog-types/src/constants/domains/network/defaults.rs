// SPDX-License-Identifier: AGPL-3.0-or-later

//! Default ports and compile-time network tuning from the config hierarchy.

use std::time::Duration;

/// Get default API port from configuration hierarchy
///
/// Respects: CLI > Env > File > Platform > Fallback (8080 in config)
///
/// Access via: `BEARDOG_CONFIG.network.api.port`
#[must_use]
pub fn default_api_port() -> u16 {
    use beardog_config::global::BEARDOG_CONFIG;
    BEARDOG_CONFIG.network.api.port
}

/// Get default metrics port from configuration hierarchy
///
/// Respects full config hierarchy with fallback to 9090 (in config)
///
/// Access via: `BEARDOG_CONFIG.network.ports.metrics_port`
#[must_use]
pub fn default_metrics_port() -> u16 {
    use beardog_config::global::BEARDOG_CONFIG;
    BEARDOG_CONFIG.network.ports.metrics_port
}

/// Get default health check port from configuration hierarchy
///
/// Currently uses API port from config (health endpoint on same port)
///
/// Access via: `BEARDOG_CONFIG.network.api.port`
#[must_use]
pub fn default_health_port() -> u16 {
    use beardog_config::global::BEARDOG_CONFIG;
    BEARDOG_CONFIG.network.api.port
}

/// Get default admin port from configuration hierarchy
///
/// Respects full config hierarchy with fallback to 8082 (in config)
///
/// Access via: `BEARDOG_CONFIG.network.admin.port`
#[must_use]
pub fn default_admin_port() -> u16 {
    use beardog_config::global::BEARDOG_CONFIG;
    BEARDOG_CONFIG.network.admin.port
}

/// Get default debug port from configuration hierarchy
///
/// Uses admin port + 1 as convention (now properly supported in `NetworkConfig`).
///
/// **Note**: `debug_port` is now available in `NetworkConfig` via `BEARDOG_DEBUG_PORT`.
/// Consider using `BEARDOG_CONFIG.network.service_ports.debug_port` directly.
#[must_use]
pub fn default_debug_port() -> u16 {
    std::env::var("BEARDOG_DEBUG_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or_else(|| {
            // Use admin port + 1 as convention (configurable)
            use beardog_config::global::BEARDOG_CONFIG;
            BEARDOG_CONFIG.network.admin.port + 1
        })
}

// ============================================================================
// TIMEOUT CONSTANTS (Compile-time defaults, not configuration)
// ============================================================================
// These are reasonable defaults that don't need runtime configuration.
// They can be overridden via TimeoutConfig if needed.

/// Timeout defaults (these are reasonable compile-time constants)
/// Default TCP connect timeout
pub const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);
/// Default socket read timeout
pub const DEFAULT_READ_TIMEOUT: Duration = Duration::from_secs(60);
/// Default socket write timeout
pub const DEFAULT_WRITE_TIMEOUT: Duration = Duration::from_secs(30);
/// Default idle connection timeout before close
pub const DEFAULT_IDLE_TIMEOUT: Duration = Duration::from_secs(300);
/// Default keep-alive probe interval / timeout
pub const DEFAULT_KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(60);

/// Buffer sizes (compile-time constants are appropriate here)
/// Default socket buffer size in bytes (64KB)
pub const DEFAULT_SOCKET_BUFFER_SIZE: usize = 65536;
/// Default send buffer size in bytes (32KB)
pub const DEFAULT_SEND_BUFFER_SIZE: usize = 32768;
/// Default receive buffer size in bytes (32KB)
pub const DEFAULT_RECEIVE_BUFFER_SIZE: usize = 32768;
/// Default listen backlog for accepting connections
pub const DEFAULT_BACKLOG_SIZE: u32 = 128;

/// Connection pool defaults (compile-time constants are appropriate here)
/// Default maximum number of concurrent connections
pub const DEFAULT_MAX_CONNECTIONS: usize = 1000;
/// Default minimum number of connections to maintain
pub const DEFAULT_MIN_CONNECTIONS: usize = 1;
/// Default size of the connection pool
pub const DEFAULT_CONNECTION_POOL_SIZE: usize = 10;
/// Default maximum number of idle connections to keep
pub const DEFAULT_MAX_IDLE_CONNECTIONS: usize = 5;

/// Protocol defaults (compile-time constants are appropriate here)
/// Default HTTP protocol version
pub const DEFAULT_HTTP_VERSION: &str = "HTTP/1.1";
/// Default TLS protocol version string for handshakes
pub const DEFAULT_TLS_VERSION: &str = "TLSv1.3";
/// Default `BearDog` protocol version
pub const DEFAULT_PROTOCOL_VERSION: &str = "2.0";
