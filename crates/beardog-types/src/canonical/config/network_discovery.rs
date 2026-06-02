// SPDX-License-Identifier: AGPL-3.0-or-later

//! Network Discovery Configuration
//! 
//! Eliminates hardcoded network endpoints by providing dynamic service discovery
//! and environment-based configuration for all network resources.

use beardog_config::env_keys;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Dynamic network discovery configuration
/// Replaces hardcoded localhost, IPs, and ports with configurable discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDiscoveryConfig {
    /// Service discovery method
    pub discovery_method: ServiceDiscoveryMethod,
    
    /// Environment-based endpoint configuration
    pub environment_endpoints: EnvironmentEndpoints,
    
    /// Fallback configuration for when discovery fails
    pub fallback_config: FallbackNetworkConfig,
    
    /// Network timeouts and retry policies
    pub network_policies: NetworkPolicyConfig,
    
    /// TLS and security configuration
    pub security_config: NetworkSecurityConfig,
}

/// Service discovery methods
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ServiceDiscoveryMethod {
    /// Environment variables (highest priority)
    Environment,
    /// DNS-based service discovery
    Dns,
    /// Kubernetes service discovery
    Kubernetes,
    /// Consul-based discovery
    Consul,
    /// mDNS/Bonjour discovery
    Mdns,
    /// Manual configuration
    Manual,
}

/// Environment-based endpoint configuration
/// Eliminates hardcoded URLs by using environment variables
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentEndpoints {
    /// Compute service endpoints
    pub compute: ServiceEndpointConfig,
    
    /// Storage service endpoints  
    pub storage: ServiceEndpointConfig,
    
    /// AI/Intelligence service endpoints
    pub intelligence: ServiceEndpointConfig,
    
    /// Service mesh endpoints
    pub mesh: ServiceEndpointConfig,
    
    /// Discovery service endpoints
    pub discovery: ServiceEndpointConfig,
    
    /// Database endpoints
    pub database: DatabaseEndpointConfig,
}

/// Service endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpointConfig {
    /// Primary endpoint (from environment or discovery)
    pub primary: Option<String>,
    
    /// Backup endpoints for failover
    pub backups: Vec<String>,
    
    /// Service-specific port mappings
    pub ports: ServicePortMapping,
    
    /// Health check endpoint
    pub health_check: Option<String>,
    
    /// Metrics endpoint
    pub metrics: Option<String>,
}

/// Service port mapping configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServicePortMapping {
    /// Main service port
    pub service: u16,
    
    /// Health check port
    pub health: u16,
    
    /// Metrics port  
    pub metrics: u16,
    
    /// Admin port
    pub admin: u16,
}

/// Database endpoint configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseEndpointConfig {
    /// Database URL (from environment)
    pub url: Option<String>,
    
    /// Connection pool configuration
    pub pool_config: DatabasePoolConfig,
    
    /// SSL/TLS configuration
    pub ssl_config: DatabaseSslConfig,
}

/// Database connection pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabasePoolConfig {
    /// Maximum connections
    pub max_connections: u32,
    
    /// Minimum connections
    pub min_connections: u32,
    
    /// Connection timeout
    pub connection_timeout: Duration,
    
    /// Idle timeout
    pub idle_timeout: Duration,
}

/// Database SSL configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSslConfig {
    /// Enable SSL
    pub enabled: bool,
    
    /// SSL mode (require, prefer, disable)
    pub mode: String,
    
    /// Certificate path
    pub cert_path: Option<String>,
    
    /// Key path
    pub key_path: Option<String>,
}

/// Fallback network configuration
/// Used when dynamic discovery fails
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackNetworkConfig {
    /// Default host (replaces hardcoded localhost)
    pub default_host: String,
    
    /// Default protocol
    pub default_protocol: String,
    
    /// Port ranges for different services
    pub port_ranges: HashMap<String, PortRange>,
    
    /// Enable localhost fallback for development
    pub enable_localhost_fallback: bool,
}

/// Port range configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortRange {
    /// Starting port
    pub start: u16,
    
    /// Ending port
    pub end: u16,
    
    /// Default port
    pub default: u16,
}

/// Network policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPolicyConfig {
    /// Connection timeout
    pub connection_timeout: Duration,
    
    /// Request timeout
    pub request_timeout: Duration,
    
    /// Retry policy
    pub retry_policy: RetryPolicyConfig,
    
    /// Circuit breaker configuration
    pub circuit_breaker: CircuitBreakerConfig,
}

/// Retry policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicyConfig {
    /// Maximum retries
    pub max_retries: u32,
    
    /// Initial delay
    pub initial_delay: Duration,
    
    /// Maximum delay
    pub max_delay: Duration,
    
    /// Backoff multiplier
    pub backoff_multiplier: f64,
}

/// Circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Failure threshold
    pub failure_threshold: u32,
    
    /// Success threshold
    pub success_threshold: u32,
    
    /// Timeout duration
    pub timeout: Duration,
}

/// Network security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurityConfig {
    /// Enable TLS
    pub tls_enabled: bool,
    
    /// Verify certificates
    pub verify_certificates: bool,
    
    /// Certificate paths
    pub cert_paths: CertificatePaths,
    
    /// Allowed cipher suites
    pub cipher_suites: Vec<String>,
    
    /// Minimum TLS version
    pub min_tls_version: String,
}

/// Certificate paths configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificatePaths {
    /// CA certificate path
    pub ca_cert: Option<String>,
    
    /// Client certificate path
    pub client_cert: Option<String>,
    
    /// Client key path
    pub client_key: Option<String>,
}

impl Default for NetworkDiscoveryConfig {
    fn default() -> Self {
        Self {
            discovery_method: ServiceDiscoveryMethod::Environment,
            environment_endpoints: EnvironmentEndpoints::default(),
            fallback_config: FallbackNetworkConfig::default(),
            network_policies: NetworkPolicyConfig::default(),
            security_config: NetworkSecurityConfig::default(),
        }
    }
}

impl Default for EnvironmentEndpoints {
    fn default() -> Self {
        Self {
            compute: ServiceEndpointConfig::compute(),
            storage: ServiceEndpointConfig::storage(),
            intelligence: ServiceEndpointConfig::intelligence(),
            mesh: ServiceEndpointConfig::mesh(),
            discovery: ServiceEndpointConfig::discovery(),
            database: DatabaseEndpointConfig::default(),
        }
    }
}

impl ServiceEndpointConfig {
    /// Create compute service endpoint configuration
    pub fn compute() -> Self {
        Self {
            primary: std::env::var(env_keys::ENV_COMPUTE_ENDPOINT).ok(),
            backups: Self::parse_backup_endpoints(env_keys::ENV_COMPUTE_BACKUPS),
            ports: ServicePortMapping {
                service: Self::parse_port(env_keys::ENV_COMPUTE_PORT, crate::constants::domains::network::defaults::default_api_port()),
                health: Self::parse_port(env_keys::ENV_COMPUTE_HEALTH_PORT, crate::constants::domains::network::defaults::default_health_port()),
                metrics: Self::parse_port(env_keys::ENV_COMPUTE_METRICS_PORT, crate::constants::domains::network::defaults::default_metrics_port()),
                admin: Self::parse_port(env_keys::ENV_COMPUTE_ADMIN_PORT, 8082),
            },
            health_check: std::env::var(env_keys::ENV_COMPUTE_HEALTH_ENDPOINT).ok(),
            metrics: std::env::var(env_keys::ENV_COMPUTE_METRICS_ENDPOINT).ok(),
        }
    }
    
    /// Create storage service endpoint configuration
    pub fn storage() -> Self {
        Self {
            primary: std::env::var(env_keys::ENV_STORAGE_ENDPOINT).ok(),
            backups: Self::parse_backup_endpoints(env_keys::ENV_STORAGE_BACKUPS),
            ports: ServicePortMapping {
                service: Self::parse_port(env_keys::ENV_STORAGE_PORT, 8084),
                health: Self::parse_port(env_keys::ENV_STORAGE_HEALTH_PORT, 8085),
                metrics: Self::parse_port(env_keys::ENV_STORAGE_METRICS_PORT, 9091),
                admin: Self::parse_port(env_keys::ENV_STORAGE_ADMIN_PORT, 8086),
            },
            health_check: std::env::var(env_keys::ENV_STORAGE_HEALTH_ENDPOINT).ok(),
            metrics: std::env::var(env_keys::ENV_STORAGE_METRICS_ENDPOINT).ok(),
        }
    }
    
    /// Create intelligence service endpoint configuration
    pub fn intelligence() -> Self {
        Self {
            primary: std::env::var(env_keys::ENV_AI_ENDPOINT).ok(),
            backups: Self::parse_backup_endpoints(env_keys::ENV_AI_BACKUPS),
            ports: ServicePortMapping {
                service: Self::parse_port(env_keys::ENV_AI_PORT, 8083),
                health: Self::parse_port(env_keys::ENV_AI_HEALTH_PORT, 8087),
                metrics: Self::parse_port(env_keys::ENV_AI_METRICS_PORT, 9092),
                admin: Self::parse_port(env_keys::ENV_AI_ADMIN_PORT, 8088),
            },
            health_check: std::env::var(env_keys::ENV_AI_HEALTH_ENDPOINT).ok(),
            metrics: std::env::var(env_keys::ENV_AI_METRICS_ENDPOINT).ok(),
        }
    }
    
    /// Create service mesh endpoint configuration
    pub fn mesh() -> Self {
        Self {
            primary: std::env::var(env_keys::ENV_MESH_ENDPOINT).ok(),
            backups: Self::parse_backup_endpoints(env_keys::ENV_MESH_BACKUPS),
            ports: ServicePortMapping {
                service: Self::parse_port(env_keys::ENV_MESH_PORT, 8082),
                health: Self::parse_port(env_keys::ENV_MESH_HEALTH_PORT, 8089),
                metrics: Self::parse_port(env_keys::ENV_MESH_METRICS_PORT, 9093),
                admin: Self::parse_port(env_keys::ENV_MESH_ADMIN_PORT, 8090),
            },
            health_check: std::env::var(env_keys::ENV_MESH_HEALTH_ENDPOINT).ok(),
            metrics: std::env::var(env_keys::ENV_MESH_METRICS_ENDPOINT).ok(),
        }
    }
    
    /// Create discovery service endpoint configuration
    pub fn discovery() -> Self {
        Self {
            primary: std::env::var(env_keys::ENV_DISCOVERY_ENDPOINT).ok(),
            backups: Self::parse_backup_endpoints(env_keys::ENV_DISCOVERY_BACKUPS),
            ports: ServicePortMapping {
                service: Self::parse_port(env_keys::ENV_DISCOVERY_PORT, crate::constants::domains::network::defaults::default_health_port()),
                health: Self::parse_port(env_keys::ENV_DISCOVERY_HEALTH_PORT, 8091),
                metrics: Self::parse_port(env_keys::ENV_DISCOVERY_METRICS_PORT, 9094),
                admin: Self::parse_port(env_keys::ENV_DISCOVERY_ADMIN_PORT, 8092),
            },
            health_check: std::env::var(env_keys::ENV_DISCOVERY_HEALTH_ENDPOINT).ok(),
            metrics: std::env::var(env_keys::ENV_DISCOVERY_METRICS_ENDPOINT).ok(),
        }
    }
    
    /// Parse port from environment variable with fallback
    fn parse_port(env_var: &str, default: u16) -> u16 {
        std::env::var(env_var)
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(default)
    }
    
    /// Parse backup endpoints from environment variable
    fn parse_backup_endpoints(env_var: &str) -> Vec<String> {
        std::env::var(env_var)
            .ok()
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default()
    }
    
    /// Get the effective endpoint URL
    pub fn get_endpoint_url(&self, fallback_host: &str) -> String {
        if let Some(primary) = &self.primary {
            primary.clone()
        } else {
            format!( http"://{}:{}", fallback_host, self.ports.service)
        }
    }
}

impl Default for DatabaseEndpointConfig {
    fn default() -> Self {
        Self {
            url: std::env::var(env_keys::ENV_DATABASE_URL)
                .or_else(|_| std::env::var(env_keys::ENV_DATABASE_URL_PREFIXED))
                .ok(),
            pool_config: DatabasePoolConfig::default(),
            ssl_config: DatabaseSslConfig::default(),
        }
    }
}

impl Default for DatabasePoolConfig {
    fn default() -> Self {
        Self {
            max_connections: std::env::var(env_keys::ENV_DB_MAX_CONNECTIONS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE),
            min_connections: std::env::var(env_keys::ENV_DB_MIN_CONNECTIONS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1),
            connection_timeout: Duration::from_secs(
                std::env::var(env_keys::ENV_DB_CONNECTION_TIMEOUT)
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30)
            ),
            idle_timeout: Duration::from_secs(
                std::env::var(env_keys::ENV_DB_IDLE_TIMEOUT)
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(600)
            ),
        }
    }
}

impl Default for DatabaseSslConfig {
    fn default() -> Self {
        Self {
            enabled: std::env::var(env_keys::ENV_DB_SSL_ENABLED)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(false),
            mode: std::env::var(env_keys::ENV_DB_SSL_MODE)
                .unwrap_or_else(|_|"prefer".to_string()),
            cert_path: std::env::var(env_keys::ENV_DB_SSL_CERT).ok(),
            key_path: std::env::var(env_keys::ENV_DB_SSL_KEY).ok(),
        }
    }
}

impl Default for FallbackNetworkConfig {
    fn default() -> Self {
        let mut port_ranges = HashMap::new();
        let api_port = crate::constants::domains::network::defaults::default_api_port();
        
        // Compute service port range
        port_ranges.insert("compute".to_string(), PortRange {
            start: api_port,
            end: std::env::var(env_keys::ENV_COMPUTE_PORT_END)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8089),
            default: api_port,
        });
        
        // Storage service port range
        port_ranges.insert("storage".to_string(), PortRange {
            start: std::env::var(env_keys::ENV_STORAGE_PORT_START)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8090),
            end: std::env::var(env_keys::ENV_STORAGE_PORT_END)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8099),
            default: std::env::var(env_keys::ENV_STORAGE_PORT)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8090),
        });
        
        // Intelligence service port range
        port_ranges.insert("intelligence".to_string(), PortRange {
            start: std::env::var(env_keys::ENV_INTELLIGENCE_PORT_START)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8100),
            end: std::env::var(env_keys::ENV_INTELLIGENCE_PORT_END)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8109),
            default: std::env::var(env_keys::ENV_INTELLIGENCE_PORT)
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8100),
        });
        
        Self {
            default_host: std::env::var(env_keys::ENV_DEFAULT_HOST)
                .unwrap_or_else(|_|beardog_types::constants::domains::network::config::default_service_host().to_string()),
            default_protocol: std::env::var(env_keys::ENV_DEFAULT_PROTOCOL)
                .unwrap_or_else(|_|"http".to_string()),
            port_ranges,
            enable_localhost_fallback: std::env::var(env_keys::ENV_ENABLE_LOCALHOST_FALLBACK)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
        }
    }
}

impl Default for NetworkPolicyConfig {
    fn default() -> Self {
        Self {
            connection_timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            retry_policy: RetryPolicyConfig::default(),
            circuit_breaker: CircuitBreakerConfig::default(),
        }
    }
}

impl Default for RetryPolicyConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: beardog_types::constants::domains::network::timeouts::DEFAULT_RETRY_DELAY,
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
        }
    }
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(60),
        }
    }
}

impl Default for NetworkSecurityConfig {
    fn default() -> Self {
        Self {
            tls_enabled: std::env::var(env_keys::ENV_TLS_ENABLED)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            verify_certificates: std::env::var(env_keys::ENV_VERIFY_CERTS)
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            cert_paths: CertificatePaths::default(),
            cipher_suites: vec![
                "TLS_AES_256_GCM_SHA384".to_string(),
                "TLS_CHACHA20_POLY1305_SHA256".to_string(),
                "TLS_AES_128_GCM_SHA256".to_string(),
            ],
            min_tls_version: "1.2".to_string(),
        }
    }
}

impl Default for CertificatePaths {
    fn default() -> Self {
        Self {
            ca_cert: std::env::var(env_keys::ENV_CA_CERT).ok(),
            client_cert: std::env::var(env_keys::ENV_CLIENT_CERT).ok(),
            client_key: std::env::var(env_keys::ENV_CLIENT_KEY).ok(),
        }
    }
}

impl NetworkDiscoveryConfig {
    /// Create a new network discovery configuration
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a production-ready configuration
    pub fn production() -> Self {
        let mut config = Self::default();
        config.fallback_config.enable_localhost_fallback = false;
        config.security_config.tls_enabled = true;
        config.security_config.verify_certificates = true;
        config.network_policies.connection_timeout = Duration::from_secs(beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u64);
        config.network_policies.request_timeout = Duration::from_secs(30);
        config
    }
    
    /// Create a development configuration with localhost fallbacks
    pub fn development() -> Self {
        let mut config = Self::default();
        config.fallback_config.enable_localhost_fallback = true;
        config.security_config.tls_enabled = false;
        config.security_config.verify_certificates = false;
        config
    }
    
    /// Validate the network discovery configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate port ranges don't overlap
        let mut used_ports = std::collections::HashSet::new();
        
        for (service, range) in &self.fallback_config.port_ranges {
            if range.start >= range.end {
                return Err(format!( Invalid" port range for service {}: {} >= {}", 
                    service, range.start, range.end));
            }
            
            if range.default < range.start || range.default > range.end {
                return Err(format!( Default" port {} for service {} is outside range {}-{}", 
                    range.default, service, range.start, range.end));
            }
            
            if used_ports.contains(&range.default) {
                return Err(format!( Port" {} is used by multiple services", range.default));
            }
            
            used_ports.insert(range.default);
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_discovery_config_default() {
        let config = NetworkDiscoveryConfig::default();
        assert_eq!(config.discovery_method, ServiceDiscoveryMethod::Environment);
        assert!(config.fallback_config.enable_localhost_fallback);
    }

    #[test]
    fn test_production_config() {
        let config = NetworkDiscoveryConfig::production();
        assert!(!config.fallback_config.enable_localhost_fallback);
        assert!(config.security_config.tls_enabled);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(config.security_config.verify_certificates);
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_development_config() {
        let config = NetworkDiscoveryConfig::development();
        assert!(config.fallback_config.enable_localhost_fallback);
        assert!(!config.security_config.tls_enabled);
    }
 // TEST_CATEGORY: unit
 // TEST_DOMAIN: types
 // TEST_PRIORITY: normal

    #[test]
    fn test_config_validation() {
        let config = NetworkDiscoveryConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(config.validate().is_ok());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_service_endpoint_fallback() {
        let compute_config = ServiceEndpointConfig::compute();
        let url = compute_config.get_endpoint_url("test-host");
        
        // Should use fallback since no environment variable is set
        assert!(url.contains("test-host"));
        let expected_port = crate::constants::domains::network::defaults::default_api_port();
        assert!(url.contains(&expected_port.to_string()));
    }
} 