// Connection Configuration
//
// Provider connection management, pooling, and authentication configuration.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    /// Connection timeout
    pub timeout: Duration,

    /// Connection retry attempts
    /// Number of `retry_attempts`
    pub retry_attempts: u32,

    /// Retry delay between attempts
    /// The retry delay value
    pub retry_delay: Duration,

    /// Keep-alive configuration
    /// The keep alive value
    pub keep_alive: KeepAliveConfig,

    /// Connection pool configuration
    ///
    /// Uses the canonical `network::ConnectionPoolConfig` from `config::domains::network`.
    pub pool: crate::canonical::config::domains::network::ConnectionPoolConfig,

    /// TLS configuration
    /// Optional tls
    pub tls: Option<TlsConfig>,

    /// Authentication configuration
    /// Optional auth
    pub auth: Option<ConnectionAuthConfig>,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            retry_attempts: 3,
            retry_delay: Duration::from_millis(1000),
            keep_alive: KeepAliveConfig::default(),
            pool: crate::canonical::config::domains::network::ConnectionPoolConfig::default(),
            tls: None,
            auth: None,
        }
    }
}

/// Keep-alive configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeepAliveConfig {
    /// Keep-alive enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Keep-alive interval
    /// The interval value
    pub interval: Duration,

    /// Keep-alive timeout
    pub timeout: Duration,
}

impl Default for KeepAliveConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Duration::from_secs(60),
            timeout: Duration::from_secs(10),
        }
    }
}

/// Connection pool configuration (DEPRECATED - use canonical config)
///
/// **MIGRATION**: Use `canonical::config::domains::network::ConnectionPoolConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use canonical::config::domains::network::ConnectionPoolConfig instead"
)]
pub type ConnectionPoolConfig = crate::canonical::config::domains::network::ConnectionPoolConfig;

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// TLS enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// TLS version
    /// The version value
    pub version: TlsVersion,

    /// Certificate path
    /// Optional cert path
    pub cert_path: Option<String>,

    /// Private key path
    /// Optional key path
    pub key_path: Option<String>,

    /// CA certificate path
    /// Optional ca path
    pub ca_path: Option<String>,

    /// Verify certificates
    /// Whether `verify_certs` is enabled
    pub verify_certs: bool,

    /// Verify hostname
    /// Name of the `verify_hostitem`
    pub verify_hostname: bool,

    /// Allowed cipher suites
    /// Collection of cipher suites
    pub cipher_suites: Vec<String>,
}

/// TLS version specification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TlsVersion {
    /// TLS version 1.2
    V1_2,
    /// TLS version 1.3
    V1_3,
}

/// Connection authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionAuthConfig {
    /// Authentication type
    /// The auth type value
    pub auth_type: AuthType,

    /// Name of the useritem
    pub username: Option<String>,

    /// Optional password
    pub password: Option<String>,

    /// Optional token
    pub token: Option<String>,

    /// Certificate authentication
    /// Optional certificate
    pub certificate: Option<CertificateAuth>,
}

/// Authentication types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
/// Types of auth
pub enum AuthType {
    /// None variant
    None,
    /// Basic variant
    Basic,
    /// Bearer variant
    Bearer,
    /// Certificate variant
    Certificate,
    /// Custom authentication type
    Custom(String),
}

/// Certificate-based authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateAuth {
    /// Client certificate path
    /// The cert path value
    pub cert_path: String,

    /// Private key path
    /// The key path value
    pub key_path: String,

    /// Key password
    /// Optional key password
    pub key_password: Option<String>,
}
