//! # Discovery Configuration Domain - DEPRECATED
//!
//! ⚠️ **DEPRECATED** (November 8, 2025)
//!
//! This module is deprecated and will be removed in a future version.
//! Please use `discovery_unified` instead:
//!
//! ```rust
//! // Old (deprecated):
//! use beardog_types::canonical::config::domains::discovery_config::ConsolidatedDiscoveryConfig;
//!
//! // New (recommended):
//! use beardog_types::canonical::config::domains::discovery_unified::UnifiedDiscoveryConfig;
//! ```
//!
//! See `DISCOVERY_CONFIG_MIGRATION_GUIDE.md` for migration instructions.
//!
//! ## Original Purpose
//! This module contained service discovery-related configuration types, extracted from
//! the large `consolidated_domains.rs` file for better maintainability.

// Allow deprecated warnings in this module - these types are intentionally kept for backward compatibility
#![allow(deprecated)]

use crate::canonical::traits::CacheStrategy;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::canonical::config::r#trait::BearDogConfig;

/// Consolidated discovery configuration for the BearDog ecosystem - DEPRECATED
///
/// ⚠️ **DEPRECATED**: Use `discovery_unified::UnifiedDiscoveryConfig` instead.
///
/// This type is deprecated and will be removed in a future version.
/// The new `UnifiedDiscoveryConfig` provides all the same features plus:
/// - Enhanced protocol support (HTTP, DNS, mDNS, Consul, etcd, K8s)
/// - Load balancing with circuit breaker
/// - Builder pattern for flexible construction
/// - Better validation and error messages
///
/// See `DISCOVERY_CONFIG_MIGRATION_GUIDE.md` for migration instructions.
///
/// # Migration Example
///
/// ```rust
/// // Old (deprecated):
/// use beardog_types::canonical::config::domains::discovery_config::ConsolidatedDiscoveryConfig;
/// let config = ConsolidatedDiscoveryConfig::default();
///
/// // New (recommended):
/// use beardog_types::canonical::config::domains::discovery_unified::UnifiedDiscoveryConfig;
/// let config = UnifiedDiscoveryConfig::default();
/// ```
#[allow(deprecated)] // Allow internal uses for backward compatibility
#[deprecated(
    since = "3.1.0",
    note = "Use discovery_unified::UnifiedDiscoveryConfig instead. See DISCOVERY_CONFIG_MIGRATION_GUIDE.md"
)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConsolidatedDiscoveryConfig {
    /// Enable or disable discovery services
    ///
    /// When `false`, the system operates in isolated mode without
    /// attempting to discover other services.
    pub enabled: bool,

    /// Service registry configuration for centralized service tracking
    pub registry: ServiceRegistryConfig,

    /// Network-based discovery configuration for protocol scanning
    pub network: NetworkDiscoveryConfig,

    /// Quantum discovery configuration for advanced service location
    pub quantum: QuantumDiscoveryConfig,

    /// Cache configuration to optimize repeated discovery operations
    pub cache: DiscoveryCacheConfig,

    /// Security settings for authenticating discovered services
    pub security: DiscoverySecurityConfig,
}

/// Service registry configuration for tracking available services
///
/// Manages the registration and lookup of services in the ecosystem, supporting
/// both centralized and distributed registry backends.
///
/// # Supported Backends
///
/// - `etcd` - Distributed key-value store
/// - `consul` - Service mesh solution
/// - `zookeeper` - Centralized coordination service
/// - `redis` - In-memory data store
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::domains::discovery_config::ServiceRegistryConfig;
/// use std::time::Duration;
///
/// let config = ServiceRegistryConfig {
///     backend: "etcd".to_string(),
///     endpoints: vec![std::env::var("ETCD_ENDPOINT")
///         .unwrap_or_else(|_| "http://etcd.ecosystem.internal:2379".to_string())],
///     service_ttl: Duration::from_secs(30),
///     health_check_interval: Duration::from_secs(5),
///     cleanup_interval: Duration::from_secs(60),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServiceRegistryConfig {
    /// Registry backend type (e.g., "etcd", "consul", "zookeeper")
    pub backend: String,

    /// Registry endpoint URLs for connecting to the backend
    pub endpoints: Vec<String>,

    /// Time-to-live for service registrations before automatic expiration
    pub service_ttl: Duration,

    /// Interval between health checks to verify service availability
    pub health_check_interval: Duration,

    /// Interval between cleanup operations to remove stale entries
    pub cleanup_interval: Duration,
}

/// Network discovery configuration for protocol-based service location
///
/// Enables discovery of services using network scanning and protocol detection.
/// Supports multiple protocols and intelligent retry strategies.
///
/// # Supported Protocols
///
/// - `mdns` - Multicast DNS for local network discovery
/// - `dns-sd` - DNS Service Discovery
/// - `upnp` - Universal Plug and Play
/// - `ssdp` - Simple Service Discovery Protocol
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::domains::discovery_config::{
///     NetworkDiscoveryConfig, RetryConfig
/// };
/// use std::time::Duration;
///
/// let config = NetworkDiscoveryConfig {
///     protocols: vec!["mdns".to_string(), "dns-sd".to_string()],
///     ports: vec![5353, 80, 443],
///     timeout: Duration::from_secs(5),
///     retry: RetryConfig::default(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkDiscoveryConfig {
    /// Discovery protocols to use for service location
    pub protocols: Vec<String>,

    /// Ports to scan during network discovery
    pub ports: Vec<u16>,

    /// Maximum time to wait for discovery responses
    pub timeout: Duration,

    /// Retry strategy configuration for failed discovery attempts
    pub retry: RetryConfig,
}

/// Quantum discovery configuration for advanced service location
///
/// Experimental configuration for quantum-enhanced discovery algorithms that
/// provide faster and more efficient service location in large-scale systems.
///
/// # Status
///
/// This is experimental and requires quantum-capable hardware or simulators.
/// Falls back to classical algorithms when quantum resources are unavailable.
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::domains::discovery_config::QuantumDiscoveryConfig;
/// use std::time::Duration;
///
/// let config = QuantumDiscoveryConfig {
///     enabled: false,  // Disabled by default
///     algorithms: vec!["grover".to_string()],
///     coherence_time: Duration::from_millis(100),
///     error_threshold: 0.01,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuantumDiscoveryConfig {
    /// Enable or disable quantum discovery features
    pub enabled: bool,

    /// Quantum algorithms to use (e.g., "grover", "qaoa")
    pub algorithms: Vec<String>,

    /// Quantum coherence time before decoherence occurs
    pub coherence_time: Duration,

    /// Maximum acceptable error rate for quantum operations (0.0-1.0)
    pub error_threshold: f64,
}

/// Discovery cache configuration for optimizing repeated lookups
///
/// Caches discovery results to reduce network traffic and improve response
/// times for frequently accessed services.
///
/// # Eviction Policies
///
/// - `lru` - Least Recently Used (recommended)
/// - `lfu` - Least Frequently Used
/// - `fifo` - First In First Out
/// - `ttl` - Time-based expiration only
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::domains::discovery_config::DiscoveryCacheConfig;
/// use std::time::Duration;
///
/// let config = DiscoveryCacheConfig {
///     enabled: true,
///     size: 1000,  // Cache up to 1000 entries
///     ttl: Duration::from_secs(300),  // 5 minutes
///     eviction_policy: "lru".to_string(),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveryCacheConfig {
    /// Enable or disable result caching
    pub enabled: bool,

    /// Maximum number of entries to cache
    pub size: usize,

    /// Time-to-live for cached discovery results
    pub ttl: Duration,

    /// Eviction policy when cache is full (lru, lfu, fifo, ttl)
    pub eviction_policy: String,
}

/// Discovery security configuration for authenticating services
///
/// Controls security measures applied during service discovery to prevent
/// unauthorized access and man-in-the-middle attacks.
///
/// # Security Layers
///
/// 1. **Authentication**: Verify service identity before trusting
/// 2. **Encryption**: Protect discovery traffic from eavesdropping
/// 3. **Network Trust**: Limit discovery to trusted network segments
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::domains::discovery_config::DiscoverySecurityConfig;
///
/// // Production configuration
/// let config = DiscoverySecurityConfig {
///     enabled: true,
///     auth_required: true,
///     encryption_required: true,
///     trusted_networks: vec![
///         "10.0.0.0/8".to_string(),
///         "192.168.0.0/16".to_string(),
///     ],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoverySecurityConfig {
    /// Enable security features for discovery
    pub enabled: bool,

    /// Require authentication for discovered services
    pub auth_required: bool,

    /// Require encryption for discovery traffic
    pub encryption_required: bool,

    /// CIDR ranges of trusted networks for service discovery
    pub trusted_networks: Vec<String>,
}

/// Retry configuration for handling transient failures
///
/// Implements exponential backoff with configurable parameters to handle
/// temporary network issues or service unavailability gracefully.
///
/// # Retry Strategy
///
/// The delay between retries follows exponential backoff:
/// ```text
/// delay = min(initial_delay * (backoff_multiplier ^ attempt), max_delay)
/// ```
///
/// # Examples
///
/// ```rust
/// use beardog_types::canonical::config::domains::discovery_config::RetryConfig;
/// use std::time::Duration;
///
/// // Standard configuration: 3 retries with 2x backoff
/// let config = RetryConfig {
///     max_attempts: 3,
///     initial_delay: Duration::from_millis(100),
///     backoff_multiplier: 2.0,
///     max_delay: Duration::from_secs(30),
///     enable_exponential_backoff: true,
/// };
/// // Retry delays: 100ms, 200ms, 400ms
/// ```
// MIGRATED: Now using canonical RetryConfig from domains/retry.rs
// Old definition (replaced Nov 8, 2025):
// pub struct RetryConfig {
//     pub max_attempts: usize,
//     pub initial_delay: Duration,
//     pub backoff_multiplier: f64,
//     pub max_delay: Duration,
// }
pub use crate::canonical::config::domains::retry::CanonicalRetryConfig as RetryConfig;

// Default implementations
#[allow(deprecated)] // Allow implementation for backward compatibility
impl Default for ConsolidatedDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            registry: ServiceRegistryConfig::default(),
            network: NetworkDiscoveryConfig::default(),
            quantum: QuantumDiscoveryConfig::default(),
            cache: DiscoveryCacheConfig::default(),
            security: DiscoverySecurityConfig::default(),
        }
    }
}

impl Default for ServiceRegistryConfig {
    fn default() -> Self {
        use super::super::network::NetworkConfig;
        let network_config = NetworkConfig::default();

        let default_backend =
            std::env::var("BEARDOG_REGISTRY_BACKEND").unwrap_or_else(|_| "consul".to_string());

        let default_endpoints = std::env::var("BEARDOG_REGISTRY_ENDPOINTS").map_or_else(
            |_| vec![format!("http://{}:8500", network_config.default_host)],
            |s| s.split(',').map(|e| e.trim().to_string()).collect(),
        );

        let service_ttl_secs = std::env::var("BEARDOG_REGISTRY_SERVICE_TTL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);

        let health_check_interval_secs =
            std::env::var("BEARDOG_REGISTRY_HEALTH_CHECK_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10);

        let cleanup_interval_secs = std::env::var("BEARDOG_REGISTRY_CLEANUP_INTERVAL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60);

        Self {
            backend: default_backend,
            endpoints: default_endpoints,
            service_ttl: Duration::from_secs(service_ttl_secs),
            health_check_interval: Duration::from_secs(health_check_interval_secs),
            cleanup_interval: Duration::from_secs(cleanup_interval_secs),
        }
    }
}

impl Default for NetworkDiscoveryConfig {
    fn default() -> Self {
        let default_ports = std::env::var("BEARDOG_DISCOVERY_PORTS")
            .ok()
            .and_then(|s| {
                s.split(',')
                    .map(|p| p.trim().parse::<u16>().ok())
                    .collect::<Option<Vec<u16>>>()
            })
            .unwrap_or_else(|| vec![8080, 9090]);

        let timeout_secs = std::env::var("BEARDOG_DISCOVERY_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);

        Self {
            protocols: vec!["http".to_string(), "grpc".to_string()],
            ports: default_ports,
            timeout: Duration::from_secs(timeout_secs),
            retry: RetryConfig::default(),
        }
    }
}

impl Default for QuantumDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: false, // Quantum discovery is experimental
            algorithms: vec!["grover".to_string(), "shor".to_string()],
            coherence_time: Duration::from_millis(
                std::env::var("BEARDOG_QUANTUM_COHERENCE_TIME_MS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100),
            ),
            error_threshold: std::env::var("BEARDOG_DISCOVERY_ERROR_THRESHOLD")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.01),
        }
    }
}

// Implement CacheStrategy trait for discovery cache configuration
impl CacheStrategy for DiscoveryCacheConfig {
    fn max_entries(&self) -> usize {
        if !self.enabled {
            return 0;
        }
        self.size
    }

    fn ttl(&self) -> Duration {
        self.ttl
    }

    fn eviction_policy(&self) -> crate::canonical::traits::cache::EvictionPolicy {
        use crate::canonical::traits::cache::EvictionPolicy as TraitPolicy;
        // Parse string policy
        match self.eviction_policy.to_lowercase().as_str() {
            "lru" => TraitPolicy::Lru,
            "lfu" => TraitPolicy::Lfu,
            "fifo" => TraitPolicy::Fifo,
            "random" => TraitPolicy::Random,
            "ttl" => TraitPolicy::Ttl,
            _ => TraitPolicy::Lru, // Default fallback
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn validate(&self) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }
        if self.size == 0 {
            return Err("Cache size must be > 0 when enabled".to_string());
        }
        if self.ttl.is_zero() {
            return Err("TTL cannot be zero".to_string());
        }
        // Validate eviction policy string
        let policy_lower = self.eviction_policy.to_lowercase();
        if !["lru", "lfu", "fifo", "random", "ttl"].contains(&policy_lower.as_str()) {
            eprintln!(
                "WARNING: Unknown eviction policy '{}', defaulting to LRU",
                self.eviction_policy
            );
        }
        Ok(())
    }

    fn is_production_ready(&self) -> bool {
        use crate::constants::domains::validation::{MAX_CACHE_TTL_SECS, MIN_CACHE_SIZE};

        if !self.enabled {
            return true;
        }
        self.size >= MIN_CACHE_SIZE
            && self.size <= 100_000
            && self.ttl >= Duration::from_secs(60)
            && self.ttl <= Duration::from_secs(MAX_CACHE_TTL_SECS)
            && self.validate().is_ok()
    }
}

impl Default for DiscoveryCacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            size: std::env::var("BEARDOG_DISCOVERY_CACHE_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
            ttl: Duration::from_secs(
                std::env::var("BEARDOG_DISCOVERY_CACHE_TTL_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(300),
            ), // 5 minutes default
            eviction_policy: std::env::var("BEARDOG_DISCOVERY_CACHE_EVICTION_POLICY")
                .unwrap_or_else(|_| "lru".to_string()),
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
        }
    }
}

// REMOVED: Custom Default implementation for RetryConfig (Nov 8, 2025)
// RetryConfig is now an alias for CanonicalRetryConfig, which has its own Default impl.
// If environment variable overrides are needed, use a custom constructor function instead.
//
// Old implementation removed to fix conflict:
// impl Default for RetryConfig {
//     fn default() -> Self {
//         Self {
//             max_attempts: std::env::var("BEARDOG_DISCOVERY_RETRY_MAX_ATTEMPTS")
//                 .ok().and_then(|s| s.parse().ok()).unwrap_or(3),
//             ...
//         }
//     }
// }
//
// If environment variable overrides are needed, add a helper function:
// pub fn retry_config_from_env() -> RetryConfig { ... }

// BearDogConfig implementation for ConsolidatedDiscoveryConfig
#[allow(deprecated)] // Allow implementation for backward compatibility
impl BearDogConfig for ConsolidatedDiscoveryConfig {
    fn validate(&self) -> Result<(), BearDogError> {
        if self.enabled {
            if self.registry.endpoints.is_empty() {
                return Err(BearDogError::validation(
                    "Registry endpoints cannot be empty",
                ));
            }

            if self.registry.service_ttl.as_secs() == 0 {
                return Err(BearDogError::validation("Service TTL cannot be zero"));
            }

            if self.network.ports.is_empty() {
                return Err(BearDogError::validation("Discovery ports cannot be empty"));
            }

            if self.network.timeout.as_secs() == 0 {
                return Err(BearDogError::validation(
                    "Network discovery timeout cannot be zero",
                ));
            }

            if self.cache.enabled && self.cache.size == 0 {
                return Err(BearDogError::validation(
                    "Cache size cannot be zero when caching is enabled",
                ));
            }

            if self.quantum.enabled
                && (self.quantum.error_threshold < 0.0 || self.quantum.error_threshold > 1.0)
            {
                return Err(BearDogError::validation(
                    "Quantum error threshold must be between 0.0 and 1.0",
                ));
            }
        }
        Ok(())
    }

    fn merge(&self, other: &Self) -> Result<Self, BearDogError> {
        Ok(Self {
            enabled: other.enabled,
            registry: if other.enabled {
                other.registry.clone()
            } else {
                self.registry.clone()
            },
            network: if other.enabled {
                other.network.clone()
            } else {
                self.network.clone()
            },
            quantum: if other.quantum.enabled {
                other.quantum.clone()
            } else {
                self.quantum.clone()
            },
            cache: if other.cache.enabled {
                other.cache.clone()
            } else {
                self.cache.clone()
            },
            security: if other.security.enabled {
                other.security.clone()
            } else {
                self.security.clone()
            },
        })
    }

    fn from_env() -> Result<Self, BearDogError> {
        let mut config = Self::default();

        if let Ok(enabled) = std::env::var("BEARDOG_DISCOVERY_ENABLED") {
            config.enabled = enabled.parse().unwrap_or(true);
        }

        if let Ok(backend) = std::env::var("BEARDOG_DISCOVERY_BACKEND") {
            config.registry.backend = backend;
        }

        if let Ok(endpoints) = std::env::var("BEARDOG_DISCOVERY_ENDPOINTS") {
            config.registry.endpoints =
                endpoints.split(',').map(|s| s.trim().to_string()).collect();
        }

        if let Ok(ttl) = std::env::var("BEARDOG_DISCOVERY_TTL") {
            if let Ok(secs) = ttl.parse::<u64>() {
                config.registry.service_ttl = Duration::from_secs(secs);
            }
        }

        if let Ok(cache_enabled) = std::env::var("BEARDOG_DISCOVERY_CACHE_ENABLED") {
            config.cache.enabled = cache_enabled.parse().unwrap_or(true);
        }

        if let Ok(cache_size) = std::env::var("BEARDOG_DISCOVERY_CACHE_SIZE") {
            config.cache.size = cache_size.parse().unwrap_or(1000);
        }

        config.validate()?;
        Ok(config)
    }

    fn to_toml(&self) -> Result<String, BearDogError> {
        toml::to_string(self).map_err(|e| {
            BearDogError::system(format!("Failed to serialize discovery config to TOML: {e}"))
        })
    }

    fn domain() -> &'static str {
        "discovery"
    }
}
