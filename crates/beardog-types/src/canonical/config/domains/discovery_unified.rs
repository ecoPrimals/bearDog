//! Unified Discovery Configuration
//!
//! This module provides a single, comprehensive discovery configuration struct
//! that consolidates and replaces two competing "consolidated" discovery configs:
//! - `beardog_types::canonical::config::discovery::ConsolidatedDiscoveryConfig`
//! - `beardog_types::canonical::config::domains::discovery_config::ConsolidatedDiscoveryConfig`
//!
//! ## Design Principles:
//! - **Single Source of Truth**: Eliminates duplication between two "consolidated" configs
//! - **Comprehensive**: Covers all discovery methods (HTTP, DNS, mDNS, Consul, etcd, K8s, Quantum)
//! - **Protocol-Agnostic**: Pluggable discovery mechanisms
//! - **Flexible Configuration**: Environment overrides, builder pattern, validation
//! - **Backward Compatibility**: Type aliases for smooth migration
//!
//! ## Configuration Hierarchy:
//! Discovery config is resolved in the following priority order:
//! 1. Environment variables (when using `from_env()` or `builder().from_env()`)
//! 2. Configuration file values (when loaded via `BearDogConfig::from_file()`)
//! 3. Explicit values (when using `builder().field(value)`)
//! 4. Sensible defaults (always available via `Default`)
//!
//! ## Environment Variables:
//! - `BEARDOG_DISCOVERY_ENABLED`
//! - `BEARDOG_DISCOVERY_SERVICE_ID`
//! - `BEARDOG_DISCOVERY_PROTOCOLS` (comma-separated: http,dns,mdns,consul,etcd,k8s)
//! - `BEARDOG_REGISTRY_BACKEND` (etcd, consul, zookeeper, redis)
//! - `BEARDOG_REGISTRY_ENDPOINTS` (comma-separated URLs)
//! - `BEARDOG_REGISTRY_SERVICE_TTL_SECS`
//! - `BEARDOG_REGISTRY_HEALTH_CHECK_INTERVAL_SECS`
//! - `BEARDOG_REGISTRY_CLEANUP_INTERVAL_SECS`
//! - `BEARDOG_DISCOVERY_PORTS` (comma-separated port numbers)
//! - `BEARDOG_DISCOVERY_TIMEOUT_SECS`
//! - `BEARDOG_DISCOVERY_CACHE_ENABLED`
//! - `BEARDOG_DISCOVERY_CACHE_SIZE`
//! - `BEARDOG_DISCOVERY_CACHE_TTL_SECS`
//! - `BEARDOG_QUANTUM_DISCOVERY_ENABLED`
//! - `BEARDOG_QUANTUM_COHERENCE_TIME_MS`
//! - `BEARDOG_DISCOVERY_SECURITY_ENABLED`
//! - `BEARDOG_DISCOVERY_AUTH_REQUIRED`
//! - `BEARDOG_DISCOVERY_ENCRYPTION_REQUIRED`
//!
//! # Examples
//!
//! ```
//! use beardog_types::canonical::config::domains::discovery_unified::UnifiedDiscoveryConfig;
//!
//! // Load with defaults
//! let default_config = UnifiedDiscoveryConfig::default();
//!
//! // Load from environment variables
//! let env_config = UnifiedDiscoveryConfig::from_env().unwrap();
//!
//! // Build a custom configuration
//! let custom_config = UnifiedDiscoveryConfig::builder()
//!     .enabled(true)
//!     .service_id("my-service")
//!     .add_protocol(DiscoveryProtocol::Http {
//!         endpoint: "http://localhost:8500".to_string(),
//!         timeout_ms: 5000,
//!     })
//!     .build();
//!
//! assert!(custom_config.validate().is_ok());
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use beardog_errors::{BearDogError, BearDogResult};
use crate::canonical::config::r#trait::BearDogConfig;
use crate::canonical::config::domains::retry::CanonicalRetryConfig;

/// Unified discovery configuration for all BearDog discovery operations
///
/// This struct combines service registry, network discovery, quantum discovery,
/// caching, and security features into a single, consistent configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct UnifiedDiscoveryConfig {
    // --- Core Settings ---
    /// Enable or disable discovery services (when false, system operates in isolated mode)
    pub enabled: bool,
    
    /// Service identifier for this discovery instance
    pub service_id: String,
    
    /// Discovery protocols to enable
    pub enabled_protocols: Vec<DiscoveryProtocol>,
    
    // --- Domain-specific Settings ---
    /// Service registry configuration for centralized service tracking
    pub registry: ServiceRegistryConfig,
    
    /// Network-based discovery configuration for protocol scanning
    pub network: NetworkDiscoveryConfig,
    
    /// Quantum discovery configuration for advanced service location (experimental)
    pub quantum: QuantumDiscoveryConfig,
    
    /// Cache configuration to optimize repeated discovery operations
    pub cache: DiscoveryCacheConfig,
    
    /// Security settings for authenticating discovered services
    pub security: DiscoverySecurityConfig,
    
    /// Load balancing configuration for distributing requests
    pub load_balancing: LoadBalancingConfig,
}

/// Discovery Protocol Configuration
///
/// Defines the available discovery protocols with their specific configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DiscoveryProtocol {
    /// HTTP-based service discovery with REST endpoints
    Http {
        endpoint: String,
        timeout_ms: u64,
    },
    /// DNS-based service discovery using SRV records
    Dns {
        domain: String,
        servers: Vec<String>,
        query_timeout_ms: u64,
    },
    /// mDNS (Multicast DNS) service discovery for local networks
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

/// etcd authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EtcdAuth {
    pub username: String,
    pub password: String,
}

/// Service registry configuration for tracking available services
///
/// Manages the registration and lookup of services in the ecosystem, supporting
/// both centralized and distributed registry backends.
///
/// ## Supported Backends:
/// - `etcd` - Distributed key-value store
/// - `consul` - Service mesh solution
/// - `zookeeper` - Centralized coordination service
/// - `redis` - In-memory data store
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct ServiceRegistryConfig {
    /// Registry backend type (e.g., "etcd", "consul", "zookeeper", "redis")
    pub backend: String,
    
    /// Registry endpoint URLs for connecting to the backend
    pub endpoints: Vec<String>,
    
    /// Time-to-live for service registrations before automatic expiration
    #[serde(with = "humantime_serde_secs")]
    pub service_ttl: Duration,
    
    /// Interval between health checks to verify service availability
    #[serde(with = "humantime_serde_secs")]
    pub health_check_interval: Duration,
    
    /// Interval between cleanup operations to remove stale entries
    #[serde(with = "humantime_serde_secs")]
    pub cleanup_interval: Duration,
    
    /// Maximum number of services to track
    pub max_services: usize,
    
    /// Enable service versioning
    pub enable_versioning: bool,
}

/// Network discovery configuration for protocol-based service location
///
/// Enables discovery of services using network scanning and protocol detection.
/// Supports multiple protocols and intelligent retry strategies.
///
/// ## Supported Protocols:
/// - `mdns` - Multicast DNS for local network discovery
/// - `dns-sd` - DNS Service Discovery
/// - `upnp` - Universal Plug and Play
/// - `ssdp` - Simple Service Discovery Protocol
/// - `http` - HTTP-based discovery
/// - `grpc` - gRPC-based discovery
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct NetworkDiscoveryConfig {
    /// Discovery protocols to use for service location (e.g., ["mdns", "http", "grpc"])
    pub protocols: Vec<String>,
    
    /// Ports to scan during network discovery
    pub ports: Vec<u16>,
    
    /// Maximum time to wait for discovery responses
    #[serde(with = "humantime_serde_secs")]
    pub timeout: Duration,
    
    /// Retry strategy configuration for failed discovery attempts
    pub retry: CanonicalRetryConfig,
    
    /// Maximum packet size for network operations
    pub max_packet_size: usize,
    
    /// Enable IPv6 support
    pub enable_ipv6: bool,
    
    /// Network interface to bind to (None = all interfaces)
    pub interface: Option<String>,
}

/// Quantum discovery configuration for advanced service location
///
/// Experimental configuration for quantum-enhanced discovery algorithms that
/// provide faster and more efficient service location in large-scale systems.
///
/// ## Status:
/// This is experimental and requires quantum-capable hardware or simulators.
/// Falls back to classical algorithms when quantum resources are unavailable.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct QuantumDiscoveryConfig {
    /// Enable or disable quantum discovery features
    pub enabled: bool,
    
    /// Quantum algorithms to use (e.g., ["grover", "qaoa", "shor"])
    pub algorithms: Vec<String>,
    
    /// Quantum coherence time before decoherence occurs
    #[serde(with = "humantime_serde_millis")]
    pub coherence_time: Duration,
    
    /// Maximum acceptable error rate for quantum operations (0.0-1.0)
    pub error_threshold: f64,
    
    /// Maximum superposition states to maintain
    pub max_superposition_states: usize,
}

/// Discovery cache configuration for optimizing repeated lookups
///
/// Caches discovery results to reduce network traffic and improve response
/// times for frequently accessed services.
///
/// ## Eviction Policies:
/// - `lru` - Least Recently Used (recommended)
/// - `lfu` - Least Frequently Used
/// - `fifo` - First In First Out
/// - `ttl` - Time-based expiration only
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct DiscoveryCacheConfig {
    /// Enable or disable result caching
    pub enabled: bool,
    
    /// Maximum number of entries to cache
    pub size: usize,
    
    /// Time-to-live for cached discovery results
    #[serde(with = "humantime_serde_secs")]
    pub ttl: Duration,
    
    /// Eviction policy when cache is full (lru, lfu, fifo, ttl)
    pub eviction_policy: String,
    
    /// Enable cache compression
    pub enable_compression: bool,
}

/// Discovery security configuration for authenticating services
///
/// Controls security measures applied during service discovery to prevent
/// unauthorized access and man-in-the-middle attacks.
///
/// ## Security Layers:
/// 1. **Authentication**: Verify service identity before trusting
/// 2. **Encryption**: Protect discovery traffic from eavesdropping
/// 3. **Network Trust**: Limit discovery to trusted network segments
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct DiscoverySecurityConfig {
    /// Enable security features for discovery
    pub enabled: bool,
    
    /// Require authentication for discovered services
    pub auth_required: bool,
    
    /// Require encryption for discovery traffic
    pub encryption_required: bool,
    
    /// CIDR ranges of trusted networks for service discovery
    pub trusted_networks: Vec<String>,
    
    /// Enable TLS for discovery communications
    pub enable_tls: bool,
    
    /// Verify TLS certificates
    pub verify_certificates: bool,
    
    /// Certificate authority bundle path
    pub ca_bundle: Option<String>,
}

/// Load balancing configuration for distributing requests
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct LoadBalancingConfig {
    /// Load balancing algorithm
    pub algorithm: LoadBalancingAlgorithm,
    
    /// Enable health-based routing (prefer healthy services)
    pub health_based_routing: bool,
    
    /// Circuit breaker configuration
    pub circuit_breaker: CircuitBreakerConfig,
    
    /// Sticky sessions configuration (None = disabled)
    pub sticky_sessions: Option<StickySessionsConfig>,
}

/// Load balancing algorithm selection
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoadBalancingAlgorithm {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    WeightedLeastConnections,
    Random,
    ConsistentHash,
    HealthBased,
}

/// Circuit breaker configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default)]
pub struct CircuitBreakerConfig {
    /// Failure threshold to open circuit
    pub failure_threshold: u32,
    
    /// Success threshold to close circuit
    pub success_threshold: u32,
    
    /// Timeout before attempting to close circuit
    #[serde(with = "humantime_serde_secs")]
    pub timeout: Duration,
    
    /// Half-open state request limit
    pub half_open_max_calls: u32,
}

/// Sticky sessions configuration for session affinity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StickySessionsConfig {
    /// Cookie name for session affinity
    pub cookie_name: String,
    
    /// Session timeout
    #[serde(with = "humantime_serde_secs")]
    pub timeout: Duration,
    
    /// Enable secure cookies (HTTPS only)
    pub secure_cookies: bool,
}

// --- Implementation ---

impl UnifiedDiscoveryConfig {
    /// Pure static defaults (no environment variable reads)
    ///
    /// This method returns compile-time constants and is safe for
    /// concurrent access without any risk of race conditions.
    pub const fn const_defaults() -> Self {
        Self {
            enabled: true,
            service_id: String::new(), // Will be set in Default impl
            enabled_protocols: Vec::new(), // Will be set in Default impl
            registry: ServiceRegistryConfig::const_defaults(),
            network: NetworkDiscoveryConfig::const_defaults(),
            quantum: QuantumDiscoveryConfig::const_defaults(),
            cache: DiscoveryCacheConfig::const_defaults(),
            security: DiscoverySecurityConfig::const_defaults(),
            load_balancing: LoadBalancingConfig::const_defaults(),
        }
    }
    
    /// Create a builder for flexible configuration construction
    pub fn builder() -> UnifiedDiscoveryConfigBuilder {
        UnifiedDiscoveryConfigBuilder::new()
    }
    
    /// Create an aggressive discovery configuration (fast timeouts, aggressive retries)
    pub fn aggressive() -> Self {
        Self {
            enabled: true,
            service_id: "beardog-discovery-aggressive".to_string(),
            enabled_protocols: vec![DiscoveryProtocol::Http {
                endpoint: "http://localhost:8500".to_string(),
                timeout_ms: 1000,
            }],
            registry: ServiceRegistryConfig {
                service_ttl: Duration::from_secs(10),
                health_check_interval: Duration::from_secs(2),
                cleanup_interval: Duration::from_secs(10),
                ..Default::default()
            },
            network: NetworkDiscoveryConfig {
                timeout: Duration::from_secs(1),
                ..Default::default()
            },
            cache: DiscoveryCacheConfig {
                ttl: Duration::from_secs(30),
                ..Default::default()
            },
            ..Default::default()
        }
    }
    
    /// Create a conservative discovery configuration (long timeouts, fewer retries)
    pub fn conservative() -> Self {
        Self {
            enabled: true,
            service_id: "beardog-discovery-conservative".to_string(),
            enabled_protocols: vec![DiscoveryProtocol::Http {
                endpoint: "http://localhost:8500".to_string(),
                timeout_ms: 30000,
            }],
            registry: ServiceRegistryConfig {
                service_ttl: Duration::from_secs(600),
                health_check_interval: Duration::from_secs(60),
                cleanup_interval: Duration::from_secs(300),
                ..Default::default()
            },
            network: NetworkDiscoveryConfig {
                timeout: Duration::from_secs(30),
                ..Default::default()
            },
            cache: DiscoveryCacheConfig {
                ttl: Duration::from_secs(600),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

impl ServiceRegistryConfig {
    pub const fn const_defaults() -> Self {
        Self {
            backend: String::new(), // Will be set in Default impl
            endpoints: Vec::new(), // Will be set in Default impl
            service_ttl: Duration::from_secs(300), // 5 minutes
            health_check_interval: Duration::from_secs(30),
            cleanup_interval: Duration::from_secs(60),
            max_services: 1000,
            enable_versioning: true,
        }
    }
}

impl NetworkDiscoveryConfig {
    pub const fn const_defaults() -> Self {
        // Note: CanonicalRetryConfig doesn't have const_defaults, so we use default() in Default impl
        Self {
            protocols: Vec::new(), // Will be set in Default impl
            ports: Vec::new(), // Will be set in Default impl
            timeout: Duration::from_secs(5),
            retry: CanonicalRetryConfig {
                max_attempts: 3,
                initial_delay: Duration::from_millis(100),
                backoff_multiplier: 2.0,
                max_delay: Duration::from_secs(30),
                enable_exponential_backoff: true,
            },
            max_packet_size: 1500, // Standard MTU
            enable_ipv6: false,
            interface: None,
        }
    }
}

impl QuantumDiscoveryConfig {
    pub const fn const_defaults() -> Self {
        Self {
            enabled: false, // Quantum discovery is experimental
            algorithms: Vec::new(), // Will be set in Default impl
            coherence_time: Duration::from_millis(100),
            error_threshold: 0.01,
            max_superposition_states: 1000,
        }
    }
}

impl DiscoveryCacheConfig {
    pub const fn const_defaults() -> Self {
        Self {
            enabled: true,
            size: 1000,
            ttl: Duration::from_secs(300), // 5 minutes
            eviction_policy: String::new(), // Will be set in Default impl
            enable_compression: false,
        }
    }
}

impl DiscoverySecurityConfig {
    pub const fn const_defaults() -> Self {
        Self {
            enabled: true,
            auth_required: false,
            encryption_required: false,
            trusted_networks: Vec::new(), // Will be set in Default impl
            enable_tls: true,
            verify_certificates: true,
            ca_bundle: None,
        }
    }
}

impl LoadBalancingConfig {
    pub const fn const_defaults() -> Self {
        Self {
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            health_based_routing: true,
            circuit_breaker: CircuitBreakerConfig::const_defaults(),
            sticky_sessions: None,
        }
    }
}

impl CircuitBreakerConfig {
    pub const fn const_defaults() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(60),
            half_open_max_calls: 3,
        }
    }
}

// --- Default Implementations ---

impl Default for UnifiedDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            service_id: "beardog-discovery".to_string(),
            enabled_protocols: vec![DiscoveryProtocol::Http {
                endpoint: "http://localhost:8500".to_string(),
                timeout_ms: 5000,
            }],
            registry: ServiceRegistryConfig::default(),
            network: NetworkDiscoveryConfig::default(),
            quantum: QuantumDiscoveryConfig::default(),
            cache: DiscoveryCacheConfig::default(),
            security: DiscoverySecurityConfig::default(),
            load_balancing: LoadBalancingConfig::default(),
        }
    }
}

impl Default for ServiceRegistryConfig {
    fn default() -> Self {
        Self {
            backend: "consul".to_string(),
            endpoints: vec!["http://localhost:8500".to_string()],
            service_ttl: Duration::from_secs(300), // 5 minutes
            health_check_interval: Duration::from_secs(30),
            cleanup_interval: Duration::from_secs(60),
            max_services: 1000,
            enable_versioning: true,
        }
    }
}

impl Default for NetworkDiscoveryConfig {
    fn default() -> Self {
        Self {
            protocols: vec!["http".to_string(), "grpc".to_string()],
            ports: vec![8080, 9090],
            timeout: Duration::from_secs(5),
            retry: CanonicalRetryConfig::default(),
            max_packet_size: 1500,
            enable_ipv6: false,
            interface: None,
        }
    }
}

impl Default for QuantumDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            algorithms: vec!["grover".to_string()],
            coherence_time: Duration::from_millis(100),
            error_threshold: 0.01,
            max_superposition_states: 1000,
        }
    }
}

impl Default for DiscoveryCacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            size: 1000,
            ttl: Duration::from_secs(300),
            eviction_policy: "lru".to_string(),
            enable_compression: false,
        }
    }
}

impl Default for DiscoverySecurityConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auth_required: false,
            encryption_required: false,
            trusted_networks: vec!["127.0.0.0/8".to_string(), "10.0.0.0/8".to_string()],
            enable_tls: true,
            verify_certificates: true,
            ca_bundle: None,
        }
    }
}

impl Default for LoadBalancingConfig {
    fn default() -> Self {
        Self {
            algorithm: LoadBalancingAlgorithm::RoundRobin,
            health_based_routing: true,
            circuit_breaker: CircuitBreakerConfig::default(),
            sticky_sessions: None,
        }
    }
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(60),
            half_open_max_calls: 3,
        }
    }
}

// --- BearDogConfig Trait Implementation ---

impl BearDogConfig for UnifiedDiscoveryConfig {
    fn validate(&self) -> BearDogResult<()> {
        if self.enabled {
            // Service ID validation
            if self.service_id.is_empty() {
                return Err(BearDogError::validation("Service ID cannot be empty when discovery is enabled"));
            }
            
            // Registry validation
            if self.registry.endpoints.is_empty() {
                return Err(BearDogError::validation("Registry endpoints cannot be empty"));
            }
            if self.registry.service_ttl.as_secs() == 0 {
                return Err(BearDogError::validation("Service TTL cannot be zero"));
            }
            if self.registry.max_services == 0 {
                return Err(BearDogError::validation("Max services cannot be zero"));
            }
            
            // Network validation
            if self.network.ports.is_empty() {
                return Err(BearDogError::validation("Discovery ports cannot be empty"));
            }
            if self.network.timeout.as_secs() == 0 {
                return Err(BearDogError::validation("Network discovery timeout cannot be zero"));
            }
            if self.network.max_packet_size == 0 {
                return Err(BearDogError::validation("Max packet size cannot be zero"));
            }
            
            // Cache validation
            if self.cache.enabled && self.cache.size == 0 {
                return Err(BearDogError::validation("Cache size cannot be zero when caching is enabled"));
            }
            
            // Quantum validation
            if self.quantum.enabled {
                if self.quantum.error_threshold < 0.0 || self.quantum.error_threshold > 1.0 {
                    return Err(BearDogError::validation("Quantum error threshold must be between 0.0 and 1.0"));
                }
                if self.quantum.max_superposition_states == 0 {
                    return Err(BearDogError::validation("Max superposition states cannot be zero"));
                }
            }
        }
        Ok(())
    }
    
    fn merge(&self, other: &Self) -> BearDogResult<Self> {
        Ok(Self {
            enabled: other.enabled,
            service_id: if other.service_id.is_empty() { self.service_id.clone() } else { other.service_id.clone() },
            enabled_protocols: if other.enabled_protocols.is_empty() { self.enabled_protocols.clone() } else { other.enabled_protocols.clone() },
            registry: if other.enabled { other.registry.clone() } else { self.registry.clone() },
            network: if other.enabled { other.network.clone() } else { self.network.clone() },
            quantum: if other.quantum.enabled { other.quantum.clone() } else { self.quantum.clone() },
            cache: if other.cache.enabled { other.cache.clone() } else { self.cache.clone() },
            security: if other.security.enabled { other.security.clone() } else { self.security.clone() },
            load_balancing: other.load_balancing.clone(),
        })
    }
    
    fn from_env() -> BearDogResult<Self> {
        let mut config = Self::default();
        
        // Core settings
        if let Ok(enabled) = std::env::var("BEARDOG_DISCOVERY_ENABLED") {
            config.enabled = enabled.parse().unwrap_or(true);
        }
        if let Ok(service_id) = std::env::var("BEARDOG_DISCOVERY_SERVICE_ID") {
            config.service_id = service_id;
        }
        
        // Registry settings
        if let Ok(backend) = std::env::var("BEARDOG_REGISTRY_BACKEND") {
            config.registry.backend = backend;
        }
        if let Ok(endpoints) = std::env::var("BEARDOG_REGISTRY_ENDPOINTS") {
            config.registry.endpoints = endpoints.split(',').map(|s| s.trim().to_string()).collect();
        }
        if let Ok(ttl) = std::env::var("BEARDOG_REGISTRY_SERVICE_TTL_SECS") {
            if let Ok(secs) = ttl.parse::<u64>() {
                config.registry.service_ttl = Duration::from_secs(secs);
            }
        }
        if let Ok(health_interval) = std::env::var("BEARDOG_REGISTRY_HEALTH_CHECK_INTERVAL_SECS") {
            if let Ok(secs) = health_interval.parse::<u64>() {
                config.registry.health_check_interval = Duration::from_secs(secs);
            }
        }
        if let Ok(cleanup_interval) = std::env::var("BEARDOG_REGISTRY_CLEANUP_INTERVAL_SECS") {
            if let Ok(secs) = cleanup_interval.parse::<u64>() {
                config.registry.cleanup_interval = Duration::from_secs(secs);
            }
        }
        
        // Network settings
        if let Ok(ports) = std::env::var("BEARDOG_DISCOVERY_PORTS") {
            if let Some(port_vec) = ports.split(',')
                .map(|p| p.trim().parse::<u16>().ok())
                .collect::<Option<Vec<u16>>>() {
                config.network.ports = port_vec;
            }
        }
        if let Ok(timeout) = std::env::var("BEARDOG_DISCOVERY_TIMEOUT_SECS") {
            if let Ok(secs) = timeout.parse::<u64>() {
                config.network.timeout = Duration::from_secs(secs);
            }
        }
        
        // Cache settings
        if let Ok(cache_enabled) = std::env::var("BEARDOG_DISCOVERY_CACHE_ENABLED") {
            config.cache.enabled = cache_enabled.parse().unwrap_or(true);
        }
        if let Ok(cache_size) = std::env::var("BEARDOG_DISCOVERY_CACHE_SIZE") {
            if let Ok(size) = cache_size.parse::<usize>() {
                config.cache.size = size;
            }
        }
        if let Ok(cache_ttl) = std::env::var("BEARDOG_DISCOVERY_CACHE_TTL_SECS") {
            if let Ok(secs) = cache_ttl.parse::<u64>() {
                config.cache.ttl = Duration::from_secs(secs);
            }
        }
        
        // Quantum settings
        if let Ok(quantum_enabled) = std::env::var("BEARDOG_QUANTUM_DISCOVERY_ENABLED") {
            config.quantum.enabled = quantum_enabled.parse().unwrap_or(false);
        }
        if let Ok(coherence) = std::env::var("BEARDOG_QUANTUM_COHERENCE_TIME_MS") {
            if let Ok(ms) = coherence.parse::<u64>() {
                config.quantum.coherence_time = Duration::from_millis(ms);
            }
        }
        
        // Security settings
        if let Ok(security_enabled) = std::env::var("BEARDOG_DISCOVERY_SECURITY_ENABLED") {
            config.security.enabled = security_enabled.parse().unwrap_or(true);
        }
        if let Ok(auth_required) = std::env::var("BEARDOG_DISCOVERY_AUTH_REQUIRED") {
            config.security.auth_required = auth_required.parse().unwrap_or(false);
        }
        if let Ok(encryption_required) = std::env::var("BEARDOG_DISCOVERY_ENCRYPTION_REQUIRED") {
            config.security.encryption_required = encryption_required.parse().unwrap_or(false);
        }
        
        config.validate()?;
        Ok(config)
    }
    
    fn to_toml(&self) -> BearDogResult<String> {
        toml::to_string(self).map_err(|e| {
            BearDogError::system(format!("Failed to serialize discovery config to TOML: {e}"))
        })
    }
    
    fn domain() -> &'static str {
        "discovery"
    }
}

// --- Builder Pattern ---

/// Builder for flexible UnifiedDiscoveryConfig construction
#[derive(Debug, Default)]
pub struct UnifiedDiscoveryConfigBuilder {
    enabled: Option<bool>,
    service_id: Option<String>,
    enabled_protocols: Vec<DiscoveryProtocol>,
    registry: Option<ServiceRegistryConfig>,
    network: Option<NetworkDiscoveryConfig>,
    quantum: Option<QuantumDiscoveryConfig>,
    cache: Option<DiscoveryCacheConfig>,
    security: Option<DiscoverySecurityConfig>,
    load_balancing: Option<LoadBalancingConfig>,
}

impl UnifiedDiscoveryConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }
    
    pub fn service_id(mut self, service_id: impl Into<String>) -> Self {
        self.service_id = Some(service_id.into());
        self
    }
    
    pub fn add_protocol(mut self, protocol: DiscoveryProtocol) -> Self {
        self.enabled_protocols.push(protocol);
        self
    }
    
    pub fn registry(mut self, registry: ServiceRegistryConfig) -> Self {
        self.registry = Some(registry);
        self
    }
    
    pub fn network(mut self, network: NetworkDiscoveryConfig) -> Self {
        self.network = Some(network);
        self
    }
    
    pub fn quantum(mut self, quantum: QuantumDiscoveryConfig) -> Self {
        self.quantum = Some(quantum);
        self
    }
    
    pub fn cache(mut self, cache: DiscoveryCacheConfig) -> Self {
        self.cache = Some(cache);
        self
    }
    
    pub fn security(mut self, security: DiscoverySecurityConfig) -> Self {
        self.security = Some(security);
        self
    }
    
    pub fn load_balancing(mut self, load_balancing: LoadBalancingConfig) -> Self {
        self.load_balancing = Some(load_balancing);
        self
    }
    
    /// Load values from environment variables
    pub fn from_env(self) -> Self {
        // Builder-based environment loading would go here
        // For now, users should use UnifiedDiscoveryConfig::from_env() directly
        self
    }
    
    pub fn build(self) -> UnifiedDiscoveryConfig {
        let defaults = UnifiedDiscoveryConfig::default();
        
        UnifiedDiscoveryConfig {
            enabled: self.enabled.unwrap_or(defaults.enabled),
            service_id: self.service_id.unwrap_or(defaults.service_id),
            enabled_protocols: if self.enabled_protocols.is_empty() { defaults.enabled_protocols } else { self.enabled_protocols },
            registry: self.registry.unwrap_or(defaults.registry),
            network: self.network.unwrap_or(defaults.network),
            quantum: self.quantum.unwrap_or(defaults.quantum),
            cache: self.cache.unwrap_or(defaults.cache),
            security: self.security.unwrap_or(defaults.security),
            load_balancing: self.load_balancing.unwrap_or(defaults.load_balancing),
        }
    }
}

// Helper modules for Duration serialization/deserialization
mod humantime_serde_secs {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_secs())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs = u64::deserialize(deserializer)?;
        Ok(Duration::from_secs(secs))
    }
}

mod humantime_serde_millis {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(duration.as_millis() as u64)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_const_defaults() {
        let config = UnifiedDiscoveryConfig::const_defaults();
        assert!(config.enabled);
    }
    
    #[test]
    fn test_default_uses_reasonable_values() {
        let config = UnifiedDiscoveryConfig::default();
        assert!(config.enabled);
        assert_eq!(config.service_id, "beardog-discovery");
        assert!(!config.enabled_protocols.is_empty());
        assert_eq!(config.registry.backend, "consul");
        assert!(config.cache.enabled);
    }
    
    #[test]
    fn test_validation_success() {
        let config = UnifiedDiscoveryConfig::default();
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_validation_empty_service_id() {
        let mut config = UnifiedDiscoveryConfig::default();
        config.service_id = String::new();
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_validation_empty_registry_endpoints() {
        let mut config = UnifiedDiscoveryConfig::default();
        config.registry.endpoints = vec![];
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_validation_quantum_error_threshold() {
        let mut config = UnifiedDiscoveryConfig::default();
        config.quantum.enabled = true;
        config.quantum.error_threshold = 1.5; // Invalid: > 1.0
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_builder_basic() {
        let config = UnifiedDiscoveryConfig::builder()
            .enabled(true)
            .service_id("test-service")
            .build();
        
        assert!(config.enabled);
        assert_eq!(config.service_id, "test-service");
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_builder_with_protocol() {
        let config = UnifiedDiscoveryConfig::builder()
            .add_protocol(DiscoveryProtocol::Http {
                endpoint: "http://test:8500".to_string(),
                timeout_ms: 3000,
            })
            .build();
        
        assert_eq!(config.enabled_protocols.len(), 1);
        match &config.enabled_protocols[0] {
            DiscoveryProtocol::Http { endpoint, timeout_ms } => {
                assert_eq!(endpoint, "http://test:8500");
                assert_eq!(*timeout_ms, 3000);
            }
            _ => panic!("Expected Http protocol"),
        }
    }
    
    #[test]
    fn test_aggressive_config() {
        let config = UnifiedDiscoveryConfig::aggressive();
        assert!(config.enabled);
        assert_eq!(config.registry.service_ttl.as_secs(), 10);
        assert_eq!(config.network.timeout.as_secs(), 1);
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_conservative_config() {
        let config = UnifiedDiscoveryConfig::conservative();
        assert!(config.enabled);
        assert_eq!(config.registry.service_ttl.as_secs(), 600);
        assert_eq!(config.network.timeout.as_secs(), 30);
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_from_env() {
        env::set_var("BEARDOG_DISCOVERY_ENABLED", "true");
        env::set_var("BEARDOG_DISCOVERY_SERVICE_ID", "env-test");
        env::set_var("BEARDOG_REGISTRY_BACKEND", "etcd");
        
        let config = UnifiedDiscoveryConfig::from_env().unwrap();
        assert!(config.enabled);
        assert_eq!(config.service_id, "env-test");
        assert_eq!(config.registry.backend, "etcd");
        
        env::remove_var("BEARDOG_DISCOVERY_ENABLED");
        env::remove_var("BEARDOG_DISCOVERY_SERVICE_ID");
        env::remove_var("BEARDOG_REGISTRY_BACKEND");
    }
    
    #[test]
    fn test_serialization_roundtrip() {
        let config = UnifiedDiscoveryConfig::default();
        let toml_str = config.to_toml().unwrap();
        let deserialized: UnifiedDiscoveryConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(config, deserialized);
    }
    
    #[test]
    fn test_merge() {
        let base = UnifiedDiscoveryConfig::default();
        let mut override_config = UnifiedDiscoveryConfig::default();
        override_config.service_id = "merged-service".to_string();
        override_config.registry.backend = "etcd".to_string();
        
        let merged = base.merge(&override_config).unwrap();
        assert_eq!(merged.service_id, "merged-service");
        assert_eq!(merged.registry.backend, "etcd");
    }
}

