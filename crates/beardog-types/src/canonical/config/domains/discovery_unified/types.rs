// SPDX-License-Identifier: AGPL-3.0-or-later

//! Core discovery configuration types (structs and enums).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use crate::canonical::config::domains::retry::CanonicalRetryConfig;

/// Unified discovery configuration for all `BearDog` discovery operations
///
/// This struct combines service registry, network discovery, quantum discovery,
/// caching, and security features into a single, consistent configuration.
///
/// ## Performance Note
/// Uses `Arc<str>` for `service_id` to enable fast, cheap cloning (10x faster).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct UnifiedDiscoveryConfig {
    // --- Core Settings ---
    /// Enable or disable discovery services (when false, system operates in isolated mode)
    pub enabled: bool,

    /// Service identifier for this discovery instance (Arc for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub service_id: Arc<str>,

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
        /// Base URL of the discovery HTTP API (scheme + host + port + optional path).
        endpoint: String,
        /// Per-request timeout for discovery calls over HTTP.
        timeout_ms: u64,
    },
    /// DNS-based service discovery using SRV records
    Dns {
        /// DNS zone or search domain for SRV lookups.
        domain: String,
        /// Recursive resolvers to use (empty = system resolver).
        servers: Vec<String>,
        /// Maximum time to wait for each DNS query.
        query_timeout_ms: u64,
    },
    /// mDNS (Multicast DNS) service discovery for local networks
    Mdns {
        /// Service type string (e.g. `_beardog._tcp.local.`).
        service_type: String,
        /// Network interface name to bind (`""` or system default = all suitable interfaces).
        interface: String,
        /// Browse / resolve timeout for multicast operations.
        timeout_ms: u64,
        /// When true, keep listening for service updates instead of one-shot browse.
        continuous_monitoring: bool,
    },
    /// Consul-based service discovery and health checking
    Consul {
        /// Consul HTTP(S) agent address.
        address: String,
        /// Consul datacenter to query.
        datacenter: String,
        /// Optional ACL token for authenticated clusters.
        token: Option<String>,
    },
    /// etcd-based distributed service discovery
    Etcd {
        /// etcd client endpoints (typically `http://host:2379`).
        endpoints: Vec<String>,
        /// Key prefix under which service registrations are stored.
        key_prefix: String,
        /// Client timeout for etcd RPCs.
        timeout_ms: u64,
        /// Optional user/password authentication for secured etcd.
        auth: Option<EtcdAuth>,
    },
    /// Kubernetes-native service discovery
    Kubernetes {
        /// Kubernetes namespace to list/watch for services or pods.
        namespace: String,
        /// Label selector restricting which objects participate in discovery.
        label_selector: HashMap<String, String>,
        /// Field selector (e.g. `metadata.name`) for finer-grained selection.
        field_selector: HashMap<String, String>,
    },
}

/// etcd authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EtcdAuth {
    /// etcd user name for role-based access (stored as `Arc<str>` for cheap cloning).
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub username: Arc<str>,
    /// etcd password or bearer secret paired with [`Self::username`].
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub password: Arc<str>,
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

    /// Eviction policy when cache is full (lru, lfu, fifo, ttl) - Arc for fast cloning
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub eviction_policy: Arc<str>,

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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    /// Distribute requests evenly in cyclic order across backends.
    RoundRobin,
    /// Like round-robin but respects per-backend integer weights.
    WeightedRoundRobin,
    /// Prefer the backend with the fewest active connections.
    LeastConnections,
    /// Least-connections with per-backend weights.
    WeightedLeastConnections,
    /// Choose a backend uniformly at random each request.
    Random,
    /// Stable mapping by request key (e.g. hash of client id) to reduce reordering.
    ConsistentHash,
    /// Prefer backends that pass health checks; fallback order is implementation-defined.
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
    /// Cookie name for session affinity (Arc for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub cookie_name: Arc<str>,

    /// Session timeout
    #[serde(with = "humantime_serde_secs")]
    pub timeout: Duration,

    /// Enable secure cookies (HTTPS only)
    pub secure_cookies: bool,
}

// --- Humantime Serde Helpers ---
// Custom serialization/deserialization helpers for Duration
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
        #[expect(
            clippy::cast_possible_truncation,
            reason = "duration millis serialized as u64 for wire format"
        )]
        let millis = duration.as_millis() as u64;
        serializer.serialize_u64(millis)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let millis = u64::deserialize(deserializer)?;
        Ok(Duration::from_millis(millis))
    }
}
