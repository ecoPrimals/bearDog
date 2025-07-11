//! Network and API configuration types
//! 
//! Contains all networking, API, TLS, CORS, and authentication configuration structures.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// Network and API configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Host to bind to
    pub host: String,
    /// Port to bind to
    pub port: u16,
    /// Enable TLS
    pub enable_tls: bool,
    /// TLS certificate path (if TLS enabled)
    pub tls_cert_path: Option<String>,
    /// TLS private key path (if TLS enabled)
    pub tls_key_path: Option<String>,
}

/// API server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    /// Bind address for the API server
    pub bind_address: String,
    /// TLS configuration
    pub tls: TlsConfig,
    /// Authentication configuration
    pub auth: AuthConfig,
    /// Rate limiting configuration
    pub rate_limiting: RateLimitConfig,
    /// CORS configuration
    pub cors: CorsConfig,
    /// Request timeout
    pub timeout: Duration,
}

/// TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Enable TLS
    pub enabled: bool,
    /// Certificate file path
    pub cert_path: Option<PathBuf>,
    /// Private key file path
    pub key_path: Option<PathBuf>,
    /// Minimum TLS version
    pub min_version: String,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// JWT secret key
    pub jwt_secret: String,
    /// JWT expiration time
    pub jwt_expiration: Duration,
    /// API key configuration
    pub api_keys: ApiKeyConfig,
}

/// API key configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyConfig {
    /// Enable API key authentication
    pub enabled: bool,
    /// API key header name
    pub header_name: String,
    /// API key length
    pub key_length: usize,
}

/// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    /// Enable rate limiting
    pub enabled: bool,
    /// Requests per minute
    pub requests_per_minute: u32,
    /// Burst size
    pub burst_size: u32,
}

/// CORS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorsConfig {
    /// Enable CORS
    pub enabled: bool,
    /// Allowed origins
    pub allowed_origins: Vec<String>,
    /// Allowed methods
    pub allowed_methods: Vec<String>,
    /// Allowed headers
    pub allowed_headers: Vec<String>,
}

/// Inter-service authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterServiceAuth {
    /// Shared secret or key
    pub secret: String,
    /// Authentication method
    pub method: AuthMethod,
}

/// Authentication method options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthMethod {
    SharedSecret,
    JWT,
    Mutual,
}

/// Retry configuration for network operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_attempts: u32,
    pub initial_delay_ms: u32,
    pub max_delay_ms: u32,
    pub backoff_multiplier: f32,
}

// Default implementations
impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            enable_tls: false,
            tls_cert_path: None,
            tls_key_path: None,
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1:3000".to_string(),
            tls: TlsConfig::default(),
            auth: AuthConfig::default(),
            rate_limiting: RateLimitConfig::default(),
            cors: CorsConfig::default(),
            timeout: Duration::from_secs(30),
        }
    }
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cert_path: None,
            key_path: None,
            min_version: "TLSv1.2".to_string(),
        }
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "change-me-in-production".to_string(),
            jwt_expiration: Duration::from_secs(3600), // 1 hour
            api_keys: ApiKeyConfig::default(),
        }
    }
}

impl Default for ApiKeyConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            header_name: "X-API-Key".to_string(),
            key_length: 32,
        }
    }
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            requests_per_minute: 60,
            burst_size: 10,
        }
    }
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            allowed_origins: vec!["*".to_string()],
            allowed_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
            allowed_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
        }
    }
} 