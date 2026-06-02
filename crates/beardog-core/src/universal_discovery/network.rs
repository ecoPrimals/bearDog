// SPDX-License-Identifier: AGPL-3.0-or-later

// Network Configuration Module
//
// This module contains network configuration, addressing, and communication settings.

use beardog_config::env_keys;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// The bind address value
    pub bind_address: SocketAddr,
    /// The multicast address value
    pub multicast_address: IpAddr,
    /// Multicast port
    /// Number of `multicast_port`
    pub multicast_port: u16,
    /// Discovery port range
    /// The discovery port range value
    pub discovery_port_range: (u16, u16),
    /// Maximum packet size
    /// Number of `max_packet_size`
    pub max_packet_size: usize,
    /// Connection timeout in milliseconds
    pub connection_timeout_ms: u64,
    /// Read timeout in milliseconds
    pub read_timeout_ms: u64,
    /// Write timeout in milliseconds
    pub write_timeout_ms: u64,
    /// Enable IPv6
    /// Whether `enable_ipv6` is enabled
    pub enable_ipv6: bool,
    /// Network interface to bind to
    /// Optional interface
    pub interface: Option<String>,
    /// TLS configuration
    pub tls_config: Option<TlsConfig>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        let beardog_network_config =
            beardog_types::canonical::config::network::NetworkConfig::default();

        Self {
            bind_address: beardog_errors::process_env::var(env_keys::ENV_DISCOVERY_BIND_ADDRESS)
                .unwrap_or_else(|_| {
                    format!(
                        "{}:{}",
                        beardog_network_config.default_host,
                        beardog_network_config.service_ports.api_port
                    )
                })
                .parse()
                .unwrap_or_else(|e| {
                    tracing::warn!("Failed to parse bind address, using fallback: {}", e);
                    SocketAddr::new(
                        IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED),
                        beardog_network_config.service_ports.api_port,
                    )
                }),
            multicast_address: beardog_errors::process_env::var(
                "BEARDOG_DISCOVERY_MULTICAST_ADDRESS",
            )
            .unwrap_or_else(|_| "224.0.0.251".to_string())
            .parse()
            .unwrap_or_else(|e| {
                tracing::warn!("Failed to parse multicast address, using fallback: {}", e);
                IpAddr::V4(std::net::Ipv4Addr::new(224, 0, 0, 251))
            }),
            multicast_port: beardog_errors::process_env::var(
                env_keys::ENV_DISCOVERY_MULTICAST_PORT,
            )
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(5353),
            discovery_port_range: {
                use beardog_config::global::BEARDOG_CONFIG;
                let start = beardog_errors::process_env::var(env_keys::ENV_DISCOVERY_PORT_START)
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or_else(|| BEARDOG_CONFIG.network.discovery.port);
                let end = beardog_errors::process_env::var(env_keys::ENV_DISCOVERY_PORT_END)
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or_else(|| BEARDOG_CONFIG.network.discovery.port + 9);
                (start, end)
            },
            max_packet_size: beardog_errors::process_env::var(
                env_keys::ENV_DISCOVERY_MAX_PACKET_SIZE,
            )
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(65536),
            connection_timeout_ms: beardog_errors::process_env::var(
                "BEARDOG_DISCOVERY_CONNECTION_TIMEOUT_MS",
            )
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(5000),
            read_timeout_ms: beardog_errors::process_env::var(
                env_keys::ENV_DISCOVERY_READ_TIMEOUT_MS,
            )
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000),
            write_timeout_ms: beardog_errors::process_env::var(
                "BEARDOG_DISCOVERY_WRITE_TIMEOUT_MS",
            )
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000),
            enable_ipv6: beardog_errors::process_env::var(env_keys::ENV_DISCOVERY_ENABLE_IPV6)
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true),
            interface: beardog_errors::process_env::var(env_keys::ENV_DISCOVERY_INTERFACE).ok(),
            tls_config: None,
        }
    }
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Certificate file path
    /// The cert file value
    pub cert_file: String,
    /// Private key file path
    /// The key file value
    pub key_file: String,
    /// CA certificate file path
    /// Optional ca file
    pub ca_file: Option<String>,
    /// Enable client certificate verification
    /// Whether `verify_client` is enabled
    pub verify_client: bool,
    /// TLS version minimum
    /// The min version value
    pub min_version: TlsVersion,
    /// Cipher suites
    /// Collection of cipher suites
    pub cipher_suites: Vec<String>,
}

/// TLS version
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TlsVersion {
    /// TLS version 1.2 (RFC 5246)
    TlsV1_2,
    /// TLS version 1.3 (RFC 8446) - recommended
    TlsV1_3,
}

/// Cache configuration
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Enable service discovery cache
    /// Whether `enable_cache` is enabled
    pub enable_cache: bool,
    /// Cache TTL in seconds
    /// Number of `cache_ttl_secs`
    pub cache_ttl_secs: u64,
    /// Maximum cache entries
    /// Number of `max_cache_entries`
    pub max_cache_entries: usize,
    /// Cache cleanup interval in seconds
    /// Number of `cleanup_interval_secs`
    pub cleanup_interval_secs: u64,
    /// Enable cache compression
    /// Whether `enable_compression` is enabled
    pub enable_compression: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enable_cache: true,
            cache_ttl_secs: 300,
            max_cache_entries: 10000,
            cleanup_interval_secs: 60,
            enable_compression: false,
        }
    }
}

/// Authentication sub-config (present = enabled)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Authentication method
    pub method: AuthenticationMethod,
    /// API keys for key-based auth
    pub api_keys: Vec<String>,
    /// JWT secret for token-based auth
    pub jwt_secret: Option<String>,
}

/// Rate-limiting sub-config (present = enabled)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Requests per minute
    pub rpm: u32,
}

/// Encryption sub-config (present = enabled)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    /// Algorithm identifier (e.g. `"AES-256-GCM"`)
    pub algorithm: String,
}

/// Security configuration for network service discovery
///
/// Uses `Option`-based toggling: `None` = disabled, `Some(cfg)` = enabled.
/// This avoids boolean flags and makes the type system enforce valid states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Authentication settings (`None` = auth disabled)
    pub auth: Option<AuthConfig>,
    /// Rate limiting settings (`None` = rate limiting disabled)
    pub rate_limit: Option<RateLimitConfig>,
    /// IP allowlist (`None` = allowlist disabled)
    pub ip_allowlist: Option<Vec<IpAddr>>,
    /// Encryption settings (`None` = encryption disabled)
    pub encryption: Option<EncryptionConfig>,
}

impl SecurityConfig {
    /// Backward-compatible accessors
    #[must_use]
    pub const fn enable_auth(&self) -> bool {
        self.auth.is_some()
    }

    /// Whether rate limiting is active
    #[must_use]
    pub const fn enable_rate_limiting(&self) -> bool {
        self.rate_limit.is_some()
    }

    /// Whether IP allowlist is active
    #[must_use]
    pub const fn enable_ip_allowlist(&self) -> bool {
        self.ip_allowlist.is_some()
    }

    /// Whether encryption is active
    #[must_use]
    pub const fn enable_encryption(&self) -> bool {
        self.encryption.is_some()
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            auth: None,
            rate_limit: Some(RateLimitConfig { rpm: 1000 }),
            ip_allowlist: None,
            encryption: None,
        }
    }
}

/// Authentication method for service access
///
/// Specifies how clients authenticate to discovered services,
/// from no authentication to certificate-based mutual TLS.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    /// No authentication required
    None,
    /// API key-based authentication
    ApiKey,
    /// JSON Web Token authentication
    JWT,
    /// OAuth 2.0 authentication
    OAuth2,
    /// Certificate-based authentication
    Certificate,
}

/// Network utilities
#[derive(Debug, Clone)]
pub struct NetworkUtils;

impl NetworkUtils {
    /// Get available network interfaces
    ///
    /// # Errors
    /// Returns an error if the system fails to enumerate network interfaces or if access is denied.
    pub fn get_network_interfaces() -> Result<Vec<NetworkInterface>, BearDogError> {
        enumerate_network_interfaces()
    }

    /// Check if port is available
    ///
    /// # Errors
    /// Returns an error if the network address binding check fails or if the socket address is invalid.
    pub async fn is_port_available(addr: &SocketAddr) -> Result<bool, BearDogError> {
        use tokio::net::TcpListener;
        match TcpListener::bind(addr).await {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Find available port in range
    ///
    /// # Errors
    /// Returns an error if address format parsing fails or if network operations encounter errors during port scanning.
    pub async fn find_available_port(start: u16, end: u16) -> Result<Option<u16>, BearDogError> {
        // Use environment variable or network config default
        use beardog_types::canonical::config::network::NetworkConfig;
        let _network_config = NetworkConfig::default();
        let bind_host = beardog_errors::process_env::var(env_keys::ENV_BIND_HOST)
            .unwrap_or_else(|_| NetworkConfig::default().default_host);

        for port in start..=end {
            let addr: SocketAddr = format!("{bind_host}:{port}").parse().map_err(|e| {
                BearDogError::system(format!("Invalid address format for port {port}: {e}"))
            })?;
            if Self::is_port_available(&addr).await? {
                return Ok(Some(port));
            }
        }
        Ok(None)
    }

    /// Validate network configuration
    ///
    /// # Errors
    /// Returns an error if the configuration has invalid port ranges, zero packet size, or other invalid network settings.
    pub fn validate_config(config: &NetworkConfig) -> Result<(), BearDogError> {
        if config.discovery_port_range.0 >= config.discovery_port_range.1 {
            return Err(BearDogError::Network {
                message: "Invalid port range: start port must be less than end port".to_string(),
                category: beardog_errors::NetworkErrorCategory::Configuration,
            });
        }

        if config.max_packet_size == 0 {
            return Err(BearDogError::Network {
                message: "Maximum packet size must be greater than 0".to_string(),
                category: beardog_errors::NetworkErrorCategory::Configuration,
            });
        }

        Ok(())
    }
}

/// Represents a network interface available for service discovery
///
/// Contains information about a physical or virtual network interface,
/// including its name, assigned IP addresses, and operational status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// Name of the network interface (e.g., "eth0", "wlan0")
    /// Name of the item
    pub name: String,
    /// List of IP addresses assigned to this interface
    /// Collection of addresses
    pub addresses: Vec<IpAddr>,
    /// Whether the interface is currently up and running
    /// Whether `is_up` is enabled
    pub is_up: bool,
    /// Whether this is a loopback interface
    /// Whether `is_loopback` is enabled
    pub is_loopback: bool,
    /// Maximum transmission unit size in bytes
    /// Optional mtu
    pub mtu: Option<u32>,
}

#[cfg(target_os = "linux")]
fn enumerate_network_interfaces() -> Result<Vec<NetworkInterface>, BearDogError> {
    use std::fs;
    use std::net::IpAddr;
    use std::path::Path;

    let net_dir = Path::new("/sys/class/net");
    let entries = fs::read_dir(net_dir).map_err(|e| {
        BearDogError::system(format!(
            "Failed to read network interfaces from {}: {e}",
            net_dir.display()
        ))
    })?;

    let mut interfaces = Vec::new();

    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };

        let is_loopback = name == "lo";
        let is_up = fs::read_to_string(path.join("operstate"))
            .map(|s| s.trim() == "up" || s.trim() == "unknown")
            .unwrap_or(false);

        let mtu = fs::read_to_string(path.join("mtu"))
            .ok()
            .and_then(|s| s.trim().parse().ok());

        let mut addresses = Vec::new();
        collect_interface_addresses(name, &mut addresses);

        if is_loopback && addresses.is_empty() {
            addresses.push(IpAddr::V4(std::net::Ipv4Addr::LOCALHOST));
            addresses.push(IpAddr::V6(std::net::Ipv6Addr::LOCALHOST));
        }

        interfaces.push(NetworkInterface {
            name: name.to_string(),
            addresses,
            is_up,
            is_loopback,
            mtu,
        });
    }

    Ok(interfaces)
}

#[cfg(target_os = "linux")]
fn collect_interface_addresses(name: &str, out: &mut Vec<std::net::IpAddr>) {
    use std::fs;
    use std::net::{IpAddr, Ipv4Addr};

    if let Ok(content) = fs::read_to_string("/proc/net/if_inet6") {
        for line in content.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 6 {
                continue;
            }
            let ifname = parts.last().copied().unwrap_or("");
            if ifname != name {
                continue;
            }
            if let Ok(v6) = parse_ipv6_hex(parts[0]) {
                out.push(IpAddr::V6(v6));
            }
        }
    }

    if let Ok(route) = fs::read_to_string("/proc/net/route") {
        for line in route.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 4 || parts[0] != name {
                continue;
            }
            if let Ok(ip) = parse_ipv4_hex(parts[1])
                && ip != Ipv4Addr::UNSPECIFIED
                && !out.contains(&IpAddr::V4(ip))
            {
                out.push(IpAddr::V4(ip));
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn parse_ipv4_hex(hex: &str) -> Result<std::net::Ipv4Addr, std::num::ParseIntError> {
    let val = u32::from_str_radix(hex, 16)?;
    Ok(std::net::Ipv4Addr::from(val.to_le_bytes()))
}

#[cfg(target_os = "linux")]
fn parse_ipv6_hex(hex: &str) -> Result<std::net::Ipv6Addr, std::num::ParseIntError> {
    if hex.len() != 32 {
        return match u8::from_str_radix("zz", 16) {
            Err(e) => Err(e),
            Ok(_) => unreachable!(),
        };
    }
    let mut bytes = [0u8; 16];
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16)?;
    }
    Ok(std::net::Ipv6Addr::from(bytes))
}

#[cfg(not(target_os = "linux"))]
fn enumerate_network_interfaces() -> Result<Vec<NetworkInterface>, BearDogError> {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    Ok(vec![NetworkInterface {
        name: "lo".to_string(),
        addresses: vec![
            IpAddr::V4(Ipv4Addr::LOCALHOST),
            IpAddr::V6(Ipv6Addr::LOCALHOST),
        ],
        is_up: true,
        is_loopback: true,
        mtu: None,
    }])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[test]
    fn network_config_default_serializes() {
        let n = NetworkConfig::default();
        let v = serde_json::to_value(&n).expect("serialize NetworkConfig");
        let _: NetworkConfig = serde_json::from_value(v).expect("deserialize NetworkConfig");
    }

    #[test]
    fn cache_config_default() {
        let c = CacheConfig::default();
        assert!(c.enable_cache);
        assert_eq!(c.cache_ttl_secs, 300);
    }

    #[test]
    fn tls_config_roundtrip() {
        let t = TlsConfig {
            cert_file: "/c.pem".into(),
            key_file: "/k.pem".into(),
            ca_file: Some("/ca.pem".into()),
            verify_client: true,
            min_version: TlsVersion::TlsV1_3,
            cipher_suites: vec!["TLS_AES_128_GCM_SHA256".into()],
        };
        let v = serde_json::to_value(&t).expect("serialize TlsConfig");
        let back: TlsConfig = serde_json::from_value(v).expect("deserialize");
        assert!(matches!(back.min_version, TlsVersion::TlsV1_3));
    }

    #[test]
    fn security_config_accessors_and_default() {
        let s = SecurityConfig::default();
        assert!(!s.enable_auth());
        assert!(s.enable_rate_limiting());
        assert!(!s.enable_ip_allowlist());
        assert!(!s.enable_encryption());

        let s2 = SecurityConfig {
            auth: Some(AuthConfig {
                method: AuthenticationMethod::ApiKey,
                api_keys: vec!["k".into()],
                jwt_secret: None,
            }),
            rate_limit: None,
            ip_allowlist: Some(vec![IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1))]),
            encryption: Some(EncryptionConfig {
                algorithm: "AES-256-GCM".into(),
            }),
        };
        assert!(s2.enable_auth());
        assert!(!s2.enable_rate_limiting());
        assert!(s2.enable_ip_allowlist());
        assert!(s2.enable_encryption());
    }

    #[test]
    fn validate_config_errors() {
        let bad_range = NetworkConfig {
            discovery_port_range: (100, 100),
            ..NetworkConfig::default()
        };
        assert!(NetworkUtils::validate_config(&bad_range).is_err());

        let mut bad_pkt = NetworkConfig::default();
        bad_pkt.max_packet_size = 0;
        assert!(NetworkUtils::validate_config(&bad_pkt).is_err());

        let ok = NetworkConfig::default();
        NetworkUtils::validate_config(&ok).expect("valid config");
    }

    #[test]
    fn network_utils_get_interfaces() {
        let r = NetworkUtils::get_network_interfaces();
        assert!(r.is_ok());
        let interfaces = r.expect("interfaces");
        assert!(
            !interfaces.is_empty(),
            "expected at least one network interface"
        );
        assert!(
            interfaces.iter().any(|i| i.is_loopback),
            "expected a loopback interface"
        );
    }

    #[tokio::test]
    async fn is_port_available_localhost() {
        let addr: SocketAddr = "127.0.0.1:0".parse().expect("parse addr");
        let avail = NetworkUtils::is_port_available(&addr)
            .await
            .expect("is_port_available");
        assert!(avail);
    }

    #[tokio::test]
    async fn find_available_port_in_tight_range() {
        let found = NetworkUtils::find_available_port(20000, 20005)
            .await
            .expect("scan");
        assert!(found.is_some());
    }
}
