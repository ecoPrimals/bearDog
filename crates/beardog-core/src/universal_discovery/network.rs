// Network Configuration Module
//
// This module contains network configuration, addressing, and communication settings.

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
            bind_address: std::env::var("BEARDOG_DISCOVERY_BIND_ADDRESS")
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
                    SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED), 8080)
                }),
            multicast_address: std::env::var("BEARDOG_DISCOVERY_MULTICAST_ADDRESS")
                .unwrap_or_else(|_| "224.0.0.251".to_string())
                .parse()
                .unwrap_or_else(|e| {
                    tracing::warn!("Failed to parse multicast address, using fallback: {}", e);
                    IpAddr::V4(std::net::Ipv4Addr::new(224, 0, 0, 251))
                }),
            multicast_port: std::env::var("BEARDOG_DISCOVERY_MULTICAST_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(5353),
            discovery_port_range: {
                use beardog_config::global::BEARDOG_CONFIG;
                let start = std::env::var("BEARDOG_DISCOVERY_PORT_START")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or_else(|| BEARDOG_CONFIG.network.discovery.port);
                let end = std::env::var("BEARDOG_DISCOVERY_PORT_END")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or_else(|| BEARDOG_CONFIG.network.discovery.port + 9);
                (start, end)
            },
            max_packet_size: std::env::var("BEARDOG_DISCOVERY_MAX_PACKET_SIZE")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(65536),
            connection_timeout_ms: std::env::var("BEARDOG_DISCOVERY_CONNECTION_TIMEOUT_MS")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(5000),
            read_timeout_ms: std::env::var("BEARDOG_DISCOVERY_READ_TIMEOUT_MS")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            write_timeout_ms: std::env::var("BEARDOG_DISCOVERY_WRITE_TIMEOUT_MS")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            enable_ipv6: std::env::var("BEARDOG_DISCOVERY_ENABLE_IPV6")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true),
            interface: std::env::var("BEARDOG_DISCOVERY_INTERFACE").ok(),
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

/// Security configuration for network service discovery
///
/// Defines authentication, authorization, and encryption settings for secure
/// communication between services in the discovery system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable authentication
    /// Whether `enable_auth` is enabled
    pub enable_auth: bool,
    /// The auth method value
    pub auth_method: AuthenticationMethod,
    /// Collection of api keys
    pub api_keys: Vec<String>,
    /// Optional jwt secret
    pub jwt_secret: Option<String>,
    /// Enable rate limiting
    /// Whether `enable_rate_limiting` is enabled
    pub enable_rate_limiting: bool,
    /// Rate limit: requests per minute
    /// Number of `rate_limit_rpm`
    pub rate_limit_rpm: u32,
    /// Enable IP allowlisting
    /// Whether `enable_ip_allowlist` is enabled
    pub enable_ip_allowlist: bool,
    /// Allowed IP addresses
    /// Collection of allowed ips
    pub allowed_ips: Vec<IpAddr>,
    /// Whether `enable_encryption` is enabled
    pub enable_encryption: bool,
    /// Encryption algorithm
    /// The encryption algorithm value
    pub encryption_algorithm: String,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_auth: false,
            auth_method: AuthenticationMethod::ApiKey,
            api_keys: vec![],
            jwt_secret: None,
            enable_rate_limiting: true,
            rate_limit_rpm: 1000,
            enable_ip_allowlist: false,
            allowed_ips: vec![],
            enable_encryption: false,
            encryption_algorithm: "AES-256-GCM".to_string(),
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
    pub const fn get_network_interfaces() -> Result<Vec<NetworkInterface>, BearDogError> {
        // Implementation would enumerate network interfaces
        Ok(vec![])
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
        let bind_host = std::env::var("BEARDOG_BIND_HOST")
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
