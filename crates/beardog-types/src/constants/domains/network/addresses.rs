// SPDX-License-Identifier: AGPL-3.0-or-later

//! Bind addresses, DNS, and multicast defaults.
//!
//! # Metrics and health bind discovery (tier hierarchy)
//!
//! Runtime resolution should follow this order (higher tiers override lower):
//!
//! 1. **Explicit bind env** — `BEARDOG_METRICS_BIND` / `BEARDOG_HEALTH_BIND` when set.
//! 2. **Constructed from address + port** — `BEARDOG_BIND_ADDRESS` (or equivalent) combined with
//!    ports from the configuration hierarchy (see [`super::defaults`] and `beardog-config`).
//! 3. **Config file / merged config** — `BEARDOG_CONFIG` and layered config sources.
//! 4. **Platform / compiled config defaults** — defaults shipped with the config crate.
//! 5. **Compile-time string fallbacks** — deprecated constants such as [`DEFAULT_METRICS_BIND`] and
//!    [`DEFAULT_HEALTH_BIND`] are *tier-5* last resorts for legacy call sites only. They must not be
//!    treated as primary sources; prefer env vars and config. Use [`default_metrics_bind_from_env`],
//!    [`default_health_bind_from_env`], or the config APIs first; reference these constants only when
//!    every other tier is unavailable (e.g. static data in old binaries).

use beardog_config::env_keys;

/// **Fallback** IPv4 loopback literal (IANA); prefer env/config (`BEARDOG_LOCALHOST_IPV4`, etc.).
pub const DEFAULT_LOCALHOST_IPV4_STR: &str = "127.0.0.1";
/// **Fallback** IPv6 loopback literal.
pub const DEFAULT_LOCALHOST_IPV6_STR: &str = "::1";
/// **Fallback** IPv4 “all interfaces” bind literal.
pub const DEFAULT_WILDCARD_IPV4_STR: &str = "0.0.0.0";
/// **Fallback** IPv6 unspecified bind literal.
pub const DEFAULT_WILDCARD_IPV6_STR: &str = "::";

/// Back-compat alias for [`DEFAULT_LOCALHOST_IPV4_STR`].
pub const LOCALHOST_IPV4: &str = DEFAULT_LOCALHOST_IPV4_STR;
/// Back-compat alias for [`DEFAULT_LOCALHOST_IPV6_STR`].
pub const LOCALHOST_IPV6: &str = DEFAULT_LOCALHOST_IPV6_STR;
/// Back-compat alias for [`DEFAULT_WILDCARD_IPV4_STR`].
pub const WILDCARD_IPV4: &str = DEFAULT_WILDCARD_IPV4_STR;
/// Back-compat alias for [`DEFAULT_WILDCARD_IPV6_STR`].
pub const WILDCARD_IPV6: &str = DEFAULT_WILDCARD_IPV6_STR;

/// Get default bind address from environment or fallback to 0.0.0.0
///
/// Checks `BEARDOG_BIND_ADDRESS` environment variable first.
#[must_use]
pub fn default_bind_address() -> String {
    WILDCARD_IPV4.to_string()
}

/// Bind address from `BEARDOG_BIND_ADDRESS`, falling back to [`WILDCARD_IPV4`].
#[must_use]
pub fn default_bind_address_from_env() -> String {
    std::env::var(env_keys::ENV_BIND_ADDRESS).unwrap_or_else(|_| WILDCARD_IPV4.to_string())
}

/// Get default API bind address from environment or construct from defaults
///
/// Checks `BEARDOG_API_BIND` environment variable first.
#[must_use]
pub fn default_api_bind() -> String {
    format!(
        "{}:{}",
        default_bind_address(),
        super::defaults::default_api_port()
    )
}

/// API bind from `BEARDOG_API_BIND`, or constructed from [`default_api_bind_from_env`] parts.
#[must_use]
pub fn default_api_bind_from_env() -> String {
    std::env::var(env_keys::ENV_API_BIND).unwrap_or_else(|_| {
        format!(
            "{}:{}",
            default_bind_address_from_env(),
            super::defaults::default_api_port()
        )
    })
}

/// Get default metrics bind address from environment or construct from defaults
///
/// Checks `BEARDOG_METRICS_BIND` environment variable first.
#[must_use]
pub fn default_metrics_bind() -> String {
    format!(
        "{}:{}",
        default_bind_address(),
        super::defaults::default_metrics_port()
    )
}

/// Metrics bind from `BEARDOG_METRICS_BIND`, or constructed from env-aware bind address.
#[must_use]
pub fn default_metrics_bind_from_env() -> String {
    std::env::var(env_keys::ENV_METRICS_BIND).unwrap_or_else(|_| {
        format!(
            "{}:{}",
            default_bind_address_from_env(),
            super::defaults::default_metrics_port()
        )
    })
}

/// Get default health check bind address from environment or construct from defaults
///
/// Checks `BEARDOG_HEALTH_BIND` environment variable first.
#[must_use]
pub fn default_health_bind() -> String {
    format!(
        "{}:{}",
        default_bind_address(),
        super::defaults::default_health_port()
    )
}

/// Health bind from `BEARDOG_HEALTH_BIND`, or constructed from env-aware bind address.
#[must_use]
pub fn default_health_bind_from_env() -> String {
    std::env::var(env_keys::ENV_HEALTH_BIND).unwrap_or_else(|_| {
        format!(
            "{}:{}",
            default_bind_address_from_env(),
            super::defaults::default_health_port()
        )
    })
}

/// Get multicast address from environment or fallback to 224.0.0.251
///
/// Checks `BEARDOG_MULTICAST_ADDRESS` environment variable first,
/// falling back to the mDNS multicast group (224.0.0.251).
#[must_use]
pub fn multicast_address() -> String {
    std::env::var(env_keys::ENV_MULTICAST_ADDRESS)
        .ok()
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| MDNS_MULTICAST_GROUP.to_string())
}

const MDNS_MULTICAST_GROUP: &str = "224.0.0.251";

/// Multicast address from `BEARDOG_MULTICAST_ADDRESS`, falling back to the default multicast group.
#[must_use]
pub fn multicast_address_from_env() -> String {
    multicast_address()
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
///
/// **Tier-5 fallback only** — literal `0.0.0.0:9190` when no env, config, or higher tier applies.
/// Primary sources: `BEARDOG_METRICS_BIND`, then address/port from env and [`super::defaults`].
pub const DEFAULT_METRICS_BIND: &str = "0.0.0.0:9190";
#[deprecated(
    since = "3.1.0",
    note = "Use default_health_bind() for environment-aware configuration"
)]
/// Deprecated: use [`default_health_bind`].
///
/// **Tier-5 fallback only** — literal `0.0.0.0:8081` when no env, config, or higher tier applies.
/// Primary sources: `BEARDOG_HEALTH_BIND`, then address/port from env and [`super::defaults`].
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
/// These are used when `BEARDOG_DNS_SERVERS` environment variable is not set.
/// Format: Google DNS (8.8.8.8, 8.8.4.4), Cloudflare DNS (1.1.1.1)
pub const FALLBACK_DNS_SERVERS: &[&str] = &["8.8.8.8", "8.8.4.4", "1.1.1.1"];

/// Get DNS servers from environment or use fallback
///
/// Reads from `BEARDOG_DNS_SERVERS` environment variable (comma-separated).
/// Falls back to well-known public DNS servers if not set.
pub fn dns_servers() -> Vec<String> {
    FALLBACK_DNS_SERVERS
        .iter()
        .map(|s| (*s).to_string())
        .collect()
}

/// DNS servers from `BEARDOG_DNS_SERVERS`, falling back to [`dns_servers`].
pub fn dns_servers_from_env() -> Vec<String> {
    std::env::var(env_keys::ENV_DNS_SERVERS).map_or_else(
        |_| dns_servers(),
        |s| s.split(',').map(str::trim).map(String::from).collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn localhost_and_wildcard_constants_match_expected_literals() {
        assert_eq!(DEFAULT_LOCALHOST_IPV4_STR, "127.0.0.1");
        assert_eq!(DEFAULT_LOCALHOST_IPV6_STR, "::1");
        assert_eq!(DEFAULT_WILDCARD_IPV4_STR, "0.0.0.0");
        assert_eq!(DEFAULT_WILDCARD_IPV6_STR, "::");
    }

    #[test]
    fn deprecated_aliases_match_modern_names() {
        assert_eq!(LOCALHOST_IPV4, DEFAULT_LOCALHOST_IPV4_STR);
        assert_eq!(LOCALHOST_IPV6, DEFAULT_LOCALHOST_IPV6_STR);
        assert_eq!(WILDCARD_IPV4, DEFAULT_WILDCARD_IPV4_STR);
        assert_eq!(WILDCARD_IPV6, DEFAULT_WILDCARD_IPV6_STR);
    }

    #[test]
    #[expect(
        deprecated,
        reason = "migration in progress — see CANONICAL_TYPE_MIGRATION_GUIDE"
    )]
    fn deprecated_bind_and_metrics_constants_exist() {
        assert_eq!(DEFAULT_BIND_ADDRESS, WILDCARD_IPV4);
        assert!(DEFAULT_METRICS_BIND.contains(':'));
        assert!(DEFAULT_HEALTH_BIND.contains(':'));
        assert_eq!(MULTICAST_ADDRESS, "224.0.0.251");
    }

    #[test]
    fn default_bind_address_helpers_consistent() {
        let b = default_bind_address();
        assert_eq!(b, WILDCARD_IPV4.to_string());
        let api = default_api_bind();
        assert!(api.contains(':'), "api bind should include port");
        let m = default_metrics_bind();
        assert!(m.contains(':'));
        let h = default_health_bind();
        assert!(h.contains(':'));
    }

    #[test]
    fn multicast_address_helpers() {
        assert_eq!(multicast_address(), "224.0.0.251".to_string());
        let from_env = multicast_address_from_env();
        assert!(
            !from_env.is_empty(),
            "multicast_address_from_env must yield a non-empty address string"
        );
    }

    #[test]
    fn dns_servers_fallback_list() {
        let s = dns_servers();
        assert_eq!(s.len(), FALLBACK_DNS_SERVERS.len());
        assert!(s.iter().any(|x| x == "8.8.8.8"));
    }

    #[test]
    fn broadcast_and_dns_port() {
        assert_eq!(BROADCAST_ADDRESS, "255.255.255.255");
        assert_eq!(DEFAULT_DNS_PORT, 53);
    }
}
