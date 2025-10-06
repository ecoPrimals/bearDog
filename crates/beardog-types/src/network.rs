// Network configuration types for BearDog
// Provides network-related type definitions and configurations

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Network endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    /// Network address
    /// The address value
    pub address: String,
    /// Port number
    /// Number of port
    pub port: u16,
    /// Connection timeout in milliseconds
    pub timeout_ms: u64,
    /// Enable SSL/TLS
    /// Whether ssl is enabled
    pub ssl_enabled: bool,
}

impl Default for EndpointConfig {
    fn default() -> Self {
        Self {
            address: std::env::var("BEARDOG_DEFAULT_HOST")
                .unwrap_or_else(|_| "localhost".to_string()),
            port: std::env::var("BEARDOG_DEFAULT_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            timeout_ms: 5000,
            ssl_enabled: false,
        }
    }
}

/// Network protocol types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NetworkProtocol {
    /// HTTP protocol
    Http,
    /// HTTPS protocol
    Https,
    /// TCP protocol
    Tcp,
    /// UDP protocol
    Udp,
    /// WebSocket protocol
    WebSocket,
    /// gRPC protocol
    Grpc,
}

impl Default for NetworkProtocol {
    fn default() -> Self {
        Self::Http
    }
}

/// Network connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConnection {
    /// Network protocol
    /// The protocol value
    pub protocol: NetworkProtocol,
    /// Endpoint configuration
    pub config: EndpointConfig,
    /// Enable keep-alive
    /// Whether keep_alive is enabled
    pub keep_alive: bool,
}

impl Default for NetworkConnection {
    fn default() -> Self {
        Self {
            protocol: NetworkProtocol::default(),
            config: EndpointConfig::default(),
            keep_alive: true,
        }
    }
}

/// Network connection pool configuration (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `beardog_types::canonical::config::domains::network::ConnectionPoolConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use canonical::config::domains::network::ConnectionPoolConfig instead"
)]
pub type ConnectionPoolConfig = crate::canonical::config::domains::network::ConnectionPoolConfig;

/// Network security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurityConfig {
    /// Enable TLS
    /// Whether tls is enabled
    pub tls_enabled: bool,
    /// TLS version
    /// The tls version value
    pub tls_version: String,
    /// Certificate path
    /// Optional cert path
    pub cert_path: Option<String>,
    /// Private key path
    /// Optional key path
    pub key_path: Option<String>,
    /// CA certificate path
    /// Optional ca cert path
    pub ca_cert_path: Option<String>,
    /// Enable certificate verification
    /// Whether verify_certificates is enabled
    pub verify_certificates: bool,
}

impl Default for NetworkSecurityConfig {
    fn default() -> Self {
        Self {
            tls_enabled: true,
            tls_version: "1.3".to_string(),
            cert_path: None,
            key_path: None,
            ca_cert_path: None,
            verify_certificates: true,
        }
    }
}

/// Network rate limiting configuration (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `canonical::config::domains::network::RateLimitConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use canonical::config::domains::network::RateLimitConfig instead"
)]
pub type RateLimitConfig = crate::canonical::config::domains::network::RateLimitConfig;

/// Complete network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Connection configuration
    /// The connection value
    pub connection: NetworkConnection,
    /// Connection pool configuration
    /// The pool value
    pub pool: ConnectionPoolConfig,
    /// Security configuration
    /// The security value
    pub security: NetworkSecurityConfig,
    /// Rate limiting configuration
    /// The rate limit value
    pub rate_limit: RateLimitConfig,
    /// Request timeout
    pub request_timeout: Duration,
    /// Response timeout
    pub response_timeout: Duration,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            connection: NetworkConnection::default(),
            pool: ConnectionPoolConfig::default(),
            security: NetworkSecurityConfig::default(),
            rate_limit: RateLimitConfig::default(),
            request_timeout: Duration::from_secs(30),
            response_timeout: Duration::from_secs(60),
        }
    }
}

impl EndpointConfig {
    /// Create a new endpoint configuration
    /// Creates a new instance
    pub fn new(address: String, port: u16) -> Self {
        Self {
            address,
            port,
            ..Default::default()
        }
    }

    /// Create a secure endpoint configuration
    pub fn secure(address: String, port: u16) -> Self {
        Self {
            address,
            port,
            ssl_enabled: true,
            ..Default::default()
        }
    }

    /// Get the full address (address:port)
    pub fn full_address(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }

    /// Check if the endpoint is local
    /// Checks if local
    /// Checks if local
    pub fn is_local(&self) -> bool {
        matches!(self.address.as_str(), 
            "localhost" | "127.0.0.1" | "::1" | "0.0.0.0" | 
            "local" | "beardog.local" | "compute.local" | "storage.local"
        )
    }
}

impl NetworkConnection {
    /// Create a new network connection
    /// Creates a new instance
    pub fn new(protocol: NetworkProtocol, config: EndpointConfig) -> Self {
        Self {
            protocol,
            config,
            keep_alive: true,
        }
    }

    /// Create an HTTP connection
    pub fn http(address: String, port: u16) -> Self {
        Self::new(NetworkProtocol::Http, EndpointConfig::new(address, port))
    }

    /// Create an HTTPS connection
    pub fn https(address: String, port: u16) -> Self {
        Self::new(
            NetworkProtocol::Https,
            EndpointConfig::secure(address, port),
        )
    }

    /// Create a gRPC connection
    pub fn grpc(address: String, port: u16) -> Self {
        Self::new(NetworkProtocol::Grpc, EndpointConfig::new(address, port))
    }

    /// Get the connection URL
    pub fn url(&self) -> String {
        let scheme = match self.protocol {
            NetworkProtocol::Http => "http",
            NetworkProtocol::Https => "https",
            NetworkProtocol::WebSocket => "ws",
            NetworkProtocol::Grpc => "grpc",
            NetworkProtocol::Tcp => "tcp",
            NetworkProtocol::Udp => "udp",
        };
        format!("{}://{}", scheme, self.config.full_address())
    }
}

impl NetworkConfig {
    /// Create a new network configuration with environment-aware defaults
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            connection: NetworkConnection::http(
                std::env::var("BEARDOG_NETWORK_ADDRESS")
                    .or_else(|_| std::env::var("BEARDOG_HOST"))
                    .unwrap_or_else(|_| "localhost".to_string()),
                std::env::var("BEARDOG_PORT")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(8080),
            ),
            security: NetworkSecurityConfig {
                tls_enabled: true,
                verify_certificates: true,
                ..Default::default()
            },
            pool: ConnectionPoolConfig::default(),
            rate_limit: RateLimitConfig::default(),
            request_timeout: Duration::from_secs(30),
            response_timeout: Duration::from_secs(60),
        }
    }

    pub fn local() -> Self {
        Self {
            connection: NetworkConnection::http("localhost".to_string(), 8080),
            security: NetworkSecurityConfig {
                tls_enabled: false,
                verify_certificates: false,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn production() -> Self {
        Self {
            connection: NetworkConnection::https("0.0.0.0".to_string(), 8443),
            security: NetworkSecurityConfig {
                tls_enabled: true,
                verify_certificates: true,
                ..Default::default()
            },
            pool: ConnectionPoolConfig {
                max_connections: 100,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Validate the network configuration
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), String> {
        if self.connection.config.port == 0 {
            return Err("Port cannot be zero".to_string());
        }

        if self.connection.config.address.is_empty() {
            return Err("Address cannot be empty".to_string());
        }

        if self.pool.min_connections > self.pool.max_connections {
            return Err("Minimum connections cannot exceed maximum connections".to_string());
        }

        if self.security.tls_enabled {
            if self.security.cert_path.is_none() {
                return Err("Certificate path required when TLS is enabled".to_string());
            }
            if self.security.key_path.is_none() {
                return Err("Private key path required when TLS is enabled".to_string());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_config() {
        let config = EndpointConfig::new("example.com".to_string(), 8080);
        assert_eq!(config.full_address(), "example.com:8080");
        assert!(!config.is_local());

        let local_config = EndpointConfig::new("localhost".to_string(), 3000);
        assert!(local_config.is_local());
    }

    #[test]
    fn test_network_connection() {
        let conn = NetworkConnection::http("api.example.com".to_string(), 80);
        assert_eq!(conn.url(), "http://api.example.com:80");

        let secure_conn = NetworkConnection::https("api.example.com".to_string(), 443);
        assert_eq!(secure_conn.url(), "https://api.example.com:443");
    }

    #[test]
    fn test_network_config_validation() {
        let mut config = NetworkConfig::local();
        assert!(config.validate().is_ok());

        // Test invalid port
        config.connection.config.port = 0;
        assert!(config.validate().is_err());

        // Test empty address
        config.connection.config.port = 8080;
        config.connection.config.address = String::new();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_production_config() {
        let config = NetworkConfig::production();
        assert!(config.security.tls_enabled);
        assert_eq!(config.connection.config.port, 8443);
        assert_eq!(config.pool.max_connections, 100);
    }
}
