// SPDX-License-Identifier: AGPL-3.0-only

//! Bind addresses, DNS, and multicast defaults.

/// Localhost addresses (these are universal constants)
/// IPv4 localhost address
pub const LOCALHOST_IPV4: &str = "127.0.0.1";
/// IPv6 localhost address
pub const LOCALHOST_IPV6: &str = "::1";
/// IPv4 wildcard address (bind to all interfaces)
pub const WILDCARD_IPV4: &str = "0.0.0.0";
/// IPv6 wildcard address (bind to all interfaces)
pub const WILDCARD_IPV6: &str = "::";

/// Get default bind address from environment or fallback to 0.0.0.0
///
/// Checks `BEARDOG_BIND_ADDRESS` environment variable first.
#[must_use]
pub fn default_bind_address() -> String {
    std::env::var("BEARDOG_BIND_ADDRESS").unwrap_or_else(|_| WILDCARD_IPV4.to_string())
}

/// Get default API bind address from environment or construct from defaults
///
/// Checks `BEARDOG_API_BIND` environment variable first.
#[must_use]
pub fn default_api_bind() -> String {
    std::env::var("BEARDOG_API_BIND").unwrap_or_else(|_| {
        format!(
            "{}:{}",
            default_bind_address(),
            super::defaults::default_api_port()
        )
    })
}

/// Get default metrics bind address from environment or construct from defaults
///
/// Checks `BEARDOG_METRICS_BIND` environment variable first.
#[must_use]
pub fn default_metrics_bind() -> String {
    std::env::var("BEARDOG_METRICS_BIND").unwrap_or_else(|_| {
        format!(
            "{}:{}",
            default_bind_address(),
            super::defaults::default_metrics_port()
        )
    })
}

/// Get default health check bind address from environment or construct from defaults
///
/// Checks `BEARDOG_HEALTH_BIND` environment variable first.
#[must_use]
pub fn default_health_bind() -> String {
    std::env::var("BEARDOG_HEALTH_BIND").unwrap_or_else(|_| {
        format!(
            "{}:{}",
            default_bind_address(),
            super::defaults::default_health_port()
        )
    })
}

/// Get multicast address from environment or fallback to 224.0.0.251
///
/// Checks `BEARDOG_MULTICAST_ADDRESS` environment variable first.
#[must_use]
pub fn multicast_address() -> String {
    std::env::var("BEARDOG_MULTICAST_ADDRESS").unwrap_or_else(|_| "224.0.0.251".to_string())
}

// Legacy const exports for backward compatibility (deprecated)
#[deprecated(
    since = "3.1.0",
    note = "Use default_bind_address() for environment-aware configuration"
)]
/// Deprecated: use [`default_bind_address`].
pub const DEFAULT_BIND_ADDRESS: &str = WILDCARD_IPV4;
// ✅ REMOVED: DEFAULT_API_BIND - Use default_api_bind() function instead
// ✅ REMOVED: DEFAULT_METRICS_BIND - Use default_metrics_bind() function instead
#[deprecated(
    since = "3.1.0",
    note = "Use default_metrics_bind() for environment-aware configuration"
)]
/// Deprecated: use [`default_metrics_bind`].
pub const DEFAULT_METRICS_BIND: &str = "0.0.0.0:9090";
#[deprecated(
    since = "3.1.0",
    note = "Use default_health_bind() for environment-aware configuration"
)]
/// Deprecated: use [`default_health_bind`].
pub const DEFAULT_HEALTH_BIND: &str = "0.0.0.0:8081";
#[deprecated(
    since = "3.1.0",
    note = "Use multicast_address() for environment-aware configuration"
)]
/// Deprecated: use [`multicast_address`].
pub const MULTICAST_ADDRESS: &str = "224.0.0.251";

/// Broadcast address (universal constant)
/// Configuration constant: broadcast address
pub const BROADCAST_ADDRESS: &str = "255.255.255.255";

/// DNS settings (these can remain as compile-time constants)
pub const DEFAULT_DNS_PORT: u16 = 53;

/// Fallback DNS servers (well-known public DNS)
/// These are used when BEARDOG_DNS_SERVERS environment variable is not set.
/// Format: Google DNS (8.8.8.8, 8.8.4.4), Cloudflare DNS (1.1.1.1)
pub const FALLBACK_DNS_SERVERS: &[&str] = &["8.8.8.8", "8.8.4.4", "1.1.1.1"];

/// Get DNS servers from environment or use fallback
///
/// Reads from `BEARDOG_DNS_SERVERS` environment variable (comma-separated).
/// Falls back to well-known public DNS servers if not set.
pub fn dns_servers() -> Vec<String> {
    std::env::var("BEARDOG_DNS_SERVERS").map_or_else(
        |_| {
            FALLBACK_DNS_SERVERS
                .iter()
                .map(|s| (*s).to_string())
                .collect()
        },
        |s| s.split(',').map(str::trim).map(String::from).collect(),
    )
}
