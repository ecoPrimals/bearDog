//! Discovery Configuration Module - DEPRECATED
//!
//! ⚠️ **DEPRECATED** (November 8, 2025)
//! 
//! This module is deprecated and will be removed in a future version.
//! Please use `discovery_unified` instead:
//! 
//! ```rust
//! // Old (deprecated):
//! use beardog_types::canonical::config::discovery::ConsolidatedDiscoveryConfig;
//! 
//! // New (recommended):
//! use beardog_types::canonical::config::domains::discovery_unified::UnifiedDiscoveryConfig;
//! ```
//!
//! See `DISCOVERY_CONFIG_MIGRATION_GUIDE.md` for migration instructions.
//!
//! ## Original Purpose
//! This module consolidated service discovery configuration types from across the codebase.
//! This functionality is now in `discovery_unified.rs` with enhanced features.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::net::{IpAddr, SocketAddr};
use std::time::Duration;

/// **CONSOLIDATED DISCOVERY CONFIGURATION** - DEPRECATED
///
/// ⚠️ **DEPRECATED**: Use `discovery_unified::UnifiedDiscoveryConfig` instead.
///
/// This type is deprecated and will be removed in a future version.
/// See `DISCOVERY_CONFIG_MIGRATION_GUIDE.md` for migration instructions.
#[deprecated(
    since = "3.1.0",
    note = "Use discovery_unified::UnifiedDiscoveryConfig instead. See DISCOVERY_CONFIG_MIGRATION_GUIDE.md"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedDiscoveryConfig {
    /// Service identifier
    pub service_id: String,
    /// Discovery protocols to enable
    pub enabled_protocols: Vec<DiscoveryProtocol>,
    /// Service registry configuration
    pub registry: ServiceRegistryConfig,
    /// Health monitoring configuration
    pub health: HealthCheckConfig,
    /// Load balancing configuration
    pub load_balancing: LoadBalancingConfig,
    /// Network configuration
    pub network: NetworkConfig,
    /// Cache configuration
    pub cache: CacheConfig,
    /// Security configuration
    pub security: SecurityConfig,
}

/// Discovery Protocol Configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiscoveryProtocol {
    /// HTTP-based service discovery with REST endpoints
    Http {
        endpoint: String,
        headers: HashMap<String, String>,
        timeout_ms: u64,
    },
    /// DNS-based service discovery using SRV records
    Dns {
        domain: String,
        servers: Vec<String>,
        query_timeout_ms: u64,
    },
    /// mDNS (Multicast DNS) service discovery
    Mdns {
        service_type: String,
        interface: String,
        timeout_ms: u64,
        continuous_monitoring: bool,
    },
    /// Consul-based service discovery and health checking
    Consul {
        address: String,
        datacenter: String,
        token: Option<String>,
    },
    /// etcd-based distributed service discovery
    Etcd {
        endpoints: Vec<String>,
        key_prefix: String,
        timeout_ms: u64,
        auth: Option<EtcdAuth>,
    },
    /// Kubernetes-native service discovery
    Kubernetes {
        namespace: String,
        label_selector: HashMap<String, String>,
        field_selector: HashMap<String, String>,
    },
}

impl Hash for DiscoveryProtocol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            DiscoveryProtocol::Http { endpoint, .. } => {
                "Http".hash(state);
                endpoint.hash(state);
            }
            DiscoveryProtocol::Dns { domain, servers, .. } => {
                "Dns".hash(state);
                domain.hash(state);
                servers.hash(state);
            }
            DiscoveryProtocol::Mdns { service_type, interface, .. } => {
                "Mdns".hash(state);
                service_type.hash(state);
                interface.hash(state);
            }
            DiscoveryProtocol::Consul { address, datacenter, .. } => {
                "Consul".hash(state);
                address.hash(state);
                datacenter.hash(state);
            }
            DiscoveryProtocol::Etcd { endpoints, key_prefix, .. } => {
                "Etcd".hash(state);
                endpoints.hash(state);
                key_prefix.hash(state);
            }
            DiscoveryProtocol::Kubernetes { namespace, .. } => {
                "Kubernetes".hash(state);
                namespace.hash(state);
            }
        }
    }
}

/// Service Registry Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRegistryConfig {
    /// Maximum number of services to track
    pub max_services: usize,
    /// Service TTL (time-to-live)
    pub service_ttl: Duration,
    /// Cleanup interval for stale services
    pub cleanup_interval: Duration,
    /// Enable service versioning
    pub enable_versioning: bool,
    /// Service metadata storage
    pub metadata_storage: MetadataStorageConfig,
}

/// Health Check Configuration
///
/// **DEPRECATED**: Use `super::domains::network::monitoring::HealthCheckConfiguration` instead.
#[deprecated(
    since = "3.1.0",
    note = "Use canonical::config::domains::network::monitoring::HealthCheckConfiguration instead"
)]
pub type HealthCheckConfig = super::domains::network::monitoring::HealthCheckConfiguration;

/// Load Balancing Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalancingConfig {
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    /// Health-based routing
    pub health_based_routing: bool,
    /// Circuit breaker configuration
    pub circuit_breaker: CircuitBreakerConfig,
    /// Retry configuration
    pub retry: RetryConfig,
    /// Sticky sessions configuration
    pub sticky_sessions: Option<StickySessionsConfig>,
}

/// Network Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Bind address for discovery service
    pub bind_address: SocketAddr,
    /// Multicast address for mDNS
    pub multicast_address: IpAddr,
    /// Multicast port
    pub multicast_port: u16,
    /// Discovery port range
    pub discovery_port_range: (u16, u16),
    /// Maximum packet size
    pub max_packet_size: usize,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Read timeout
    pub read_timeout: Duration,
    /// Write timeout
    pub write_timeout: Duration,
    /// Enable IPv6
    pub enable_ipv6: bool,
    /// Network interface to bind to
    pub interface: Option<String>,
    /// TLS configuration
    pub tls: Option<TlsConfig>,
}

/// Cache Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// Maximum number of cache entries
    pub max_entries: usize,
    /// Cache TTL (time-to-live)
    pub ttl: Duration,
    /// Cache eviction policy
    pub eviction_policy: EvictionPolicy,
    /// Enable cache compression
    pub enable_compression: bool,
    /// Cache persistence
    pub persistence: Option<CachePersistenceConfig>,
}

/// Security Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Enable TLS for discovery communications
    pub enable_tls: bool,
    /// Verify TLS certificates
    pub verify_certificates: bool,
    /// Certificate authority bundle
    pub ca_bundle: Option<String>,
    /// Client certificate for mutual TLS
    pub client_cert: Option<ClientCertConfig>,
    /// Authentication configuration
    pub authentication: Option<AuthenticationConfig>,
    /// Authorization configuration
    pub authorization: Option<AuthorizationConfig>,
}

// Supporting types and enums

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EtcdAuth {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataStorageConfig {
    /// Storage backend type
    pub backend: MetadataBackend,
    /// Storage capacity limits
    pub capacity_limits: CapacityLimits,
    /// Enable encryption at rest
    pub encrypt_at_rest: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetadataBackend {
    Memory,
    File { path: String },
    Database { connection_string: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityLimits {
    pub max_metadata_size_mb: usize,
    pub max_services: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthEndpoint {
    pub path: String,
    pub method: String,
    pub expected_status: u16,
    pub timeout: Duration,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    WeightedLeastConnections,
    Random,
    ConsistentHash,
    HealthBased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Failure threshold to open circuit
    pub failure_threshold: u32,
    /// Success threshold to close circuit
    pub success_threshold: u32,
    /// Timeout before attempting to close circuit
    pub timeout: Duration,
    /// Half-open state request limit
    pub half_open_max_calls: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// Maximum number of retries
    pub max_retries: u32,
    /// Base delay between retries
    pub base_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Backoff strategy
    pub backoff_strategy: BackoffStrategy,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BackoffStrategy {
    Fixed,
    Linear,
    Exponential,
    ExponentialWithJitter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickySessionsConfig {
    /// Cookie name for session affinity
    pub cookie_name: String,
    /// Session timeout
    pub timeout: Duration,
    /// Enable secure cookies
    pub secure_cookies: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsConfig {
    /// Certificate file path
    pub cert_file: String,
    /// Private key file path
    pub key_file: String,
    /// CA certificate file path
    pub ca_file: Option<String>,
    /// Verify peer certificates
    pub verify_peer: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EvictionPolicy {
    LRU,  // Least Recently Used
    LFU,  // Least Frequently Used
    FIFO, // First In, First Out
    TTL,  // Time To Live based
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePersistenceConfig {
    /// Persistence backend
    pub backend: PersistenceBackend,
    /// Persistence interval
    pub persist_interval: Duration,
    /// Enable compression for persisted data
    pub compress: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersistenceBackend {
    File { path: String },
    Redis { connection_string: String },
    Database { connection_string: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCertConfig {
    pub cert_file: String,
    pub key_file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    /// Authentication method
    pub method: AuthMethod,
    /// Token configuration
    pub token: Option<TokenConfig>,
    /// Certificate-based authentication
    pub certificate: Option<CertAuthConfig>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AuthMethod {
    None,
    Token,
    Certificate,
    OAuth2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    pub token: String,
    pub header_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertAuthConfig {
    pub client_cert: String,
    pub client_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationConfig {
    /// Authorization policy
    pub policy: AuthPolicy,
    /// Role-based access control
    pub rbac: Option<RbacConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthPolicy {
    AllowAll,
    DenyAll,
    RoleBased,
    Custom { rules: Vec<AuthRule> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RbacConfig {
    pub roles: HashMap<String, Vec<String>>,
    pub permissions: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRule {
    pub resource: String,
    pub action: String,
    pub effect: AuthEffect,
    pub conditions: HashMap<String, String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AuthEffect {
    Allow,
    Deny,
}

// Default implementations

impl Default for ConsolidatedDiscoveryConfig {
    fn default() -> Self {
        use super::runtime_config::RuntimeNetworkConfig;
        
        // Use environment-driven configuration instead of hardcoded localhost
        let config = RuntimeNetworkConfig::from_env();
        let endpoint = std::env::var("BEARDOG_DISCOVERY_ENDPOINT")
            .unwrap_or_else(|_| config.discovery_endpoint);
        
        Self {
            service_id: "beardog-discovery".to_string(),
            enabled_protocols: vec![
                DiscoveryProtocol::Http {
                    endpoint,
                    headers: HashMap::new(),
                    timeout_ms: beardog_types::constants::domains::network::defaults::DEFAULT_CONNECTION_TIMEOUT.as_millis() as u64,
                },
            ],
            registry: ServiceRegistryConfig::default(),
            health: HealthCheckConfig::default(),
            load_balancing: LoadBalancingConfig::default(),
            network: NetworkConfig::default(),
            cache: CacheConfig::default(),
            security: SecurityConfig::default(),
        }
    }
}

impl Default for ServiceRegistryConfig {
    fn default() -> Self {
        Self {
            max_services: beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE,
            service_ttl: Duration::from_secs(
                std::env::var("BEARDOG_SERVICE_REGISTRY_TTL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300)
            ),
            cleanup_interval: Duration::from_secs(
                std::env::var("BEARDOG_SERVICE_REGISTRY_CLEANUP_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60)
            ),
            enable_versioning: true,
            metadata_storage: MetadataStorageConfig::default(),
        }
    }
}

impl Default for MetadataStorageConfig {
    fn default() -> Self {
        Self {
            backend: MetadataBackend::Memory,
            capacity_limits: CapacityLimits {
                max_metadata_size_mb: 100,
                max_services: beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE,
            },
            encrypt_at_rest: false,
        }
    }
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(
                std::env::var("BEARDOG_DISCOVERY_HEALTH_CHECK_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30)
            ),
            check_timeout: Duration::from_secs(
                std::env::var("BEARDOG_DISCOVERY_HEALTH_CHECK_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(5)
            ),
            failure_threshold: 3,
            success_threshold: 2,
            enable_metrics: true,
            endpoints: vec![HealthEndpoint {
                path: "/health".to_string(),
                method: "GET".to_string(),
                expected_status: 200,
                timeout: Duration::from_secs(
                    std::env::var("BEARDOG_HEALTH_ENDPOINT_TIMEOUT_SECS")
                        .ok()
                        .and_then(|s| s.parse().ok())
                        .unwrap_or(5)
                ),
            }],
        }
    }
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            health_based_routing: true,
            circuit_breaker: CircuitBreakerConfig::default(),
            retry: RetryConfig::default(),
            sticky_sessions: None,
        }
    }
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(
                std::env::var("BEARDOG_DISCOVERY_CIRCUIT_BREAKER_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(60)
            ),
            half_open_max_calls: 3,
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: beardog_types::constants::domains::network::timeouts::DEFAULT_RETRY_DELAY,
            max_delay: Duration::from_secs(30),
            backoff_strategy: BackoffStrategy::ExponentialWithJitter,
        }
    }
}

impl Default for NetworkConfig {
    fn default() -> Self {
        use std::net::{Ipv4Addr, SocketAddr};
        let api_port = crate::constants::domains::network::defaults::default_api_port();
        Self {
            bind_address: SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                api_port,
            ),
            multicast_address: IpAddr::V4(Ipv4Addr::new(224, 0, 0, 251)),
            multicast_port: std::env::var("BEARDOG_MULTICAST_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(5353), // mDNS standard port
            discovery_port_range: (
                api_port,
                std::env::var("BEARDOG_DISCOVERY_PORT_RANGE_END")
                    .ok()
                    .and_then(|p| p.parse().ok())
                    .unwrap_or(8090),
            ),
            max_packet_size: std::env::var("BEARDOG_MAX_PACKET_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1500), // Standard MTU
            connection_timeout: Duration::from_secs(beardog_types::constants::domains::system::defaults::DEFAULT_POOL_SIZE as u64),
            read_timeout: Duration::from_secs(
                std::env::var("BEARDOG_DISCOVERY_READ_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30)
            ),
            write_timeout: Duration::from_secs(
                std::env::var("BEARDOG_DISCOVERY_WRITE_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30)
            ),
            enable_ipv6: false,
            interface: None,
            tls: None,
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_entries: beardog_types::constants::domains::system::defaults::DEFAULT_QUEUE_SIZE,
            ttl: Duration::from_secs(
                std::env::var("BEARDOG_DISCOVERY_CACHE_TTL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300)
            ),
            eviction_policy: EvictionPolicy::LRU,
            enable_compression: false,
            persistence: None,
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            enable_tls: true,
            verify_certificates: true,
            ca_bundle: None,
            client_cert: None,
            authentication: None,
            authorization: None,
        }
    }
} 