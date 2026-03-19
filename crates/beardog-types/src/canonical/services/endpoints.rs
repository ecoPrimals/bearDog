// SPDX-License-Identifier: AGPL-3.0-only

//! Unified Service Endpoint Definitions
//!
//! This module provides consolidated endpoint types for service communication,
//! replacing scattered endpoint definitions across the codebase.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Unified service endpoint definition
///
/// Consolidates endpoint types from:
/// - `ServiceEndpoint` from `beardog-types/src/services/`
/// - `ServiceEndpoints` from `beardog-core/src/ecosystem/primal_types.rs`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedServiceEndpoint {
    /// Endpoint name identifier
    pub name: String,
    
    /// The endpoint URL
    pub url: String,
    
    /// Communication protocol used
    pub protocol: EndpointProtocol,
    
    /// Supported HTTP methods or operations
    pub methods: Vec<String>,
    
    /// Whether authentication is required
    pub authentication_required: bool,
    
    /// Authentication configuration if required
    pub auth_config: Option<EndpointAuthConfig>,
    
    /// Security configuration for this endpoint
    pub security_config: Option<EndpointSecurityConfig>,
    
    /// Health check configuration
    pub health_check: Option<HealthCheckConfig>,
    
    /// Additional endpoint metadata
    pub metadata: HashMap<String, String>,
}

/// Supported endpoint protocols
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EndpointProtocol {
    /// Standard HTTP protocol
    Http,
    /// Secure HTTPS protocol
    Https,
    /// gRPC protocol
    Grpc,
    /// Secure gRPC with TLS
    GrpcTls,
    /// WebSocket protocol
    WebSocket,
    /// Secure WebSocket protocol
    WebSocketSecure,
    /// TCP socket
    Tcp,
    /// UDP socket
    Udp,
    /// Unix domain socket
    Unix,
    /// Custom protocol
    Custom(String),
}

/// Authentication configuration for endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointAuthConfig {
    /// Authentication type required
    pub auth_type: AuthenticationType,
    
    /// Required scopes or permissions
    pub required_scopes: Vec<String>,
    
    /// Token validation configuration
    pub token_config: Option<TokenConfig>,
    
    /// Mutual TLS configuration
    pub mtls_config: Option<MutualTlsConfig>,
}

/// Types of authentication supported
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthenticationType {
    /// No authentication required
    None,
    /// Bearer token authentication
    Bearer,
    /// Basic HTTP authentication
    Basic,
    /// API key authentication
    ApiKey,
    /// OAuth 2.0 authentication
    OAuth2,
    /// JWT token authentication
    Jwt,
    /// Mutual TLS authentication
    MutualTls,
    /// Custom authentication scheme
    Custom(String),
}

/// Token validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    /// Token validation endpoint
    pub validation_endpoint: Option<String>,
    
    /// Token issuer
    pub issuer: Option<String>,
    
    /// Token audience
    pub audience: Option<String>,
    
    /// Token expiration tolerance in seconds
    pub expiration_tolerance_seconds: Option<u64>,
}

/// Mutual TLS configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutualTlsConfig {
    /// Client certificate required
    pub require_client_cert: bool,
    
    /// Trusted CA certificates
    pub trusted_ca_certs: Vec<String>,
    
    /// Certificate revocation list
    pub crl_endpoints: Vec<String>,
}

/// Security configuration for endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointSecurityConfig {
    /// Require TLS encryption
    pub require_tls: bool,
    
    /// Minimum TLS version
    pub min_tls_version: Option<TlsVersion>,
    
    /// Allowed cipher suites
    pub allowed_ciphers: Vec<String>,
    
    /// Rate limiting configuration
    pub rate_limit: Option<RateLimitConfig>,
    
    /// IP allowlist
    pub ip_allowlist: Vec<String>,
    
    /// IP blocklist
    pub ip_blocklist: Vec<String>,
}

/// TLS version specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TlsVersion {
    /// TLS 1.0 (deprecated, not recommended)
    V1_0,
    /// TLS 1.1 (deprecated, not recommended)
    V1_1,
    /// TLS 1.2 (minimum recommended)
    V1_2,
    /// TLS 1.3 (preferred)
    V1_3,
}

/// Rate limiting configuration (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `super::super::config::domains::network::RateLimitConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use super::super::config::domains::network::RateLimitConfig instead"
)]
pub type RateLimitConfig = super::super::config::domains::network::RateLimitConfig;

/// Rate limiting scope (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `super::super::config::domains::network::RateLimitScope` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use super::super::config::domains::network::RateLimitScope instead"
)]
pub type RateLimitScope = super::super::config::domains::network::RateLimitScope;

/// Health check configuration for endpoints
///
/// **DEPRECATED**: Use `super::super::config::domains::network::monitoring::HealthCheckConfiguration` instead.
///
/// For specialized health checks, use the appropriate type from `canonical::monitoring::health`.
#[deprecated(
    since = "3.1.0",
    note = "Use canonical::config::domains::network::monitoring::HealthCheckConfiguration or canonical::monitoring::health types"
)]
pub type HealthCheckConfig = super::super::config::domains::network::monitoring::HealthCheckConfiguration;

impl Default for UnifiedServiceEndpoint {
    fn default() -> Self {
        Self {
            name: String::new(),
            url: String::new(),
            protocol: EndpointProtocol::Https,
            methods: vec!["GET".to_string()],
            authentication_required: true,
            auth_config: None,
            security_config: None,
            health_check: None,
            metadata: HashMap::new(),
        }
    }
}

impl UnifiedServiceEndpoint {
    /// Create a new HTTP endpoint
    pub fn http(name: String, url: String) -> Self {
        Self {
            name,
            url,
            protocol: EndpointProtocol::Http,
            ..Default::default()
        }
    }
    
    /// Create a new HTTPS endpoint
    pub fn https(name: String, url: String) -> Self {
        Self {
            name,
            url,
            protocol: EndpointProtocol::Https,
            ..Default::default()
        }
    }
    
    /// Create a new gRPC endpoint
    pub fn grpc(name: String, url: String) -> Self {
        Self {
            name,
            url,
            protocol: EndpointProtocol::GrpcTls,
            methods: vec!["CALL".to_string()],
            ..Default::default()
        }
    }
    
    /// Set supported HTTP methods
    pub fn with_methods(mut self, methods: Vec<String>) -> Self {
        self.methods = methods;
        self
    }
    
    /// Set authentication configuration
    pub fn with_auth(mut self, auth_config: EndpointAuthConfig) -> Self {
        self.authentication_required = true;
        self.auth_config = Some(auth_config);
        self
    }
    
    /// Set security configuration
    pub fn with_security(mut self, security_config: EndpointSecurityConfig) -> Self {
        self.security_config = Some(security_config);
        self
    }
    
    /// Set health check configuration
    pub fn with_health_check(mut self, health_check: HealthCheckConfig) -> Self {
        self.health_check = Some(health_check);
        self
    }
    
    /// Check if this endpoint is secure (uses TLS)
    pub fn is_secure(&self) -> bool {
        matches!(
            self.protocol,
            EndpointProtocol::Https
                | EndpointProtocol::GrpcTls
                | EndpointProtocol::WebSocketSecure
        )
    }
    
    /// Check if this endpoint supports a specific HTTP method
    pub fn supports_method(&self, method: &str) -> bool {
        self.methods.iter().any(|m| m.eq_ignore_ascii_case(method))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_creation() {
        let endpoint = UnifiedServiceEndpoint::https(
            "api".to_string(),
            "https://api.example.com".to_string(),
        );
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: types
         // TEST_PRIORITY: normal
        
        assert_eq!(endpoint.name, "api");
        assert_eq!(endpoint.url, "https://api.example.com");
        assert_eq!(endpoint.protocol, EndpointProtocol::Https);
        assert!(endpoint.is_secure());
    }
    
    #[test]
    fn test_endpoint_methods() {
        let endpoint = UnifiedServiceEndpoint::https(
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            "api".to_string(),
            "https://api.example.com".to_string(),
        ).with_methods(vec!["GET".to_string(), "POST".to_string()]);
        
        assert!(endpoint.supports_method("GET"));
        assert!(endpoint.supports_method("post")); // Case insensitive
        assert!(!endpoint.supports_method("DELETE"));
    }
    
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_grpc_endpoint() {
        let endpoint = UnifiedServiceEndpoint::grpc(
            "grpc-service".to_string(),
            "grpc://service.example.com:443".to_string(),
        );
        
        assert_eq!(endpoint.protocol, EndpointProtocol::GrpcTls);
        assert!(endpoint.supports_method("CALL"));
    }
} 