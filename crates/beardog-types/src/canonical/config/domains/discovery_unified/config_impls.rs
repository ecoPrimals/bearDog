// SPDX-License-Identifier: AGPL-3.0-or-later

//! [`UnifiedDiscoveryConfig`] inherent methods, defaults, and [`BearDogConfig`] impl.

use std::sync::Arc;
use std::time::Duration;

use crate::canonical::config::domains::retry::CanonicalRetryConfig;
use crate::canonical::config::r#trait::BearDogConfig;
use crate::constants::defaults;
use crate::constants::domains::network::ports::DEFAULT_CONSUL_HTTP_PORT;
use beardog_config::env_keys;
use beardog_errors::BearDogError;

use super::builder::UnifiedDiscoveryConfigBuilder;
use super::types::{
    CircuitBreakerConfig, DiscoveryCacheConfig, DiscoveryProtocol, DiscoverySecurityConfig,
    LoadBalancingAlgorithm, LoadBalancingConfig, NetworkDiscoveryConfig, QuantumDiscoveryConfig,
    ServiceRegistryConfig, UnifiedDiscoveryConfig,
};

impl UnifiedDiscoveryConfig {
    /// Pure static defaults (no environment variable reads)
    ///
    /// This method returns default values and is safe for
    /// concurrent access without any risk of race conditions.
    pub fn const_defaults() -> Self {
        Self {
            enabled: true,
            service_id: Arc::from(""),
            enabled_protocols: Vec::new(),
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
        let discovery_endpoint = std::env::var(env_keys::ENV_DISCOVERY_ENDPOINT)
            .or_else(|_| std::env::var(env_keys::ENV_DISCOVERY_ENDPOINT_UNPREFIXED))
            .unwrap_or_else(|_| {
                let host = std::env::var(env_keys::ENV_DISCOVERY_HOST_UNPREFIXED)
                    .unwrap_or_else(|_| env_keys::DEFAULT_DISCOVERY_HOST.to_string());
                let port = std::env::var(env_keys::ENV_DISCOVERY_PORT_UNPREFIXED)
                    .unwrap_or_else(|_| DEFAULT_CONSUL_HTTP_PORT.to_string());
                format!("http://{host}:{port}")
            });

        Self {
            enabled: true,
            service_id: Arc::from("beardog-discovery-aggressive"),
            enabled_protocols: vec![DiscoveryProtocol::Http {
                endpoint: discovery_endpoint,
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
        let discovery_endpoint = std::env::var(env_keys::ENV_DISCOVERY_ENDPOINT)
            .or_else(|_| std::env::var(env_keys::ENV_DISCOVERY_ENDPOINT_UNPREFIXED))
            .unwrap_or_else(|_| {
                let host = std::env::var(env_keys::ENV_DISCOVERY_HOST_UNPREFIXED)
                    .unwrap_or_else(|_| env_keys::DEFAULT_DISCOVERY_HOST.to_string());
                let port = std::env::var(env_keys::ENV_DISCOVERY_PORT_UNPREFIXED)
                    .unwrap_or_else(|_| DEFAULT_CONSUL_HTTP_PORT.to_string());
                format!("http://{host}:{port}")
            });

        Self {
            enabled: true,
            service_id: Arc::from("beardog-discovery-conservative"),
            enabled_protocols: vec![DiscoveryProtocol::Http {
                endpoint: discovery_endpoint,
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
    /// Static defaults for tests and const contexts (no environment reads).
    pub const fn const_defaults() -> Self {
        Self {
            backend: String::new(),
            endpoints: Vec::new(),
            service_ttl: Duration::from_secs(300), // 5 minutes
            health_check_interval: Duration::from_secs(30),
            cleanup_interval: Duration::from_secs(60),
            max_services: 1000,
            enable_versioning: true,
        }
    }
}

impl NetworkDiscoveryConfig {
    /// Static defaults for tests and const contexts (no environment reads).
    pub const fn const_defaults() -> Self {
        // Note: CanonicalRetryConfig doesn't have const_defaults, so we use default() in Default impl
        Self {
            protocols: Vec::new(), // Will be set in Default impl
            ports: Vec::new(),     // Will be set in Default impl
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
    /// Static defaults for tests and const contexts (no environment reads).
    pub const fn const_defaults() -> Self {
        Self {
            enabled: false,         // Quantum discovery is experimental
            algorithms: Vec::new(), // Will be set in Default impl
            coherence_time: Duration::from_millis(100),
            error_threshold: 0.01,
            max_superposition_states: 1000,
        }
    }
}

impl DiscoveryCacheConfig {
    /// Static defaults for tests (no environment reads); mirrors conservative cache sizing.
    pub fn const_defaults() -> Self {
        Self {
            enabled: true,
            size: 1000,
            ttl: Duration::from_secs(300), // 5 minutes
            eviction_policy: Arc::from(""),
            enable_compression: false,
        }
    }
}

impl DiscoverySecurityConfig {
    /// Static defaults for tests and const contexts (no environment reads).
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
    /// Static defaults for tests and const contexts (no environment reads).
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
    /// Static defaults for tests and const contexts (no environment reads).
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
        let discovery_endpoint = std::env::var(env_keys::ENV_DISCOVERY_ENDPOINT)
            .or_else(|_| std::env::var(env_keys::ENV_DISCOVERY_ENDPOINT_UNPREFIXED))
            .unwrap_or_else(|_| {
                let host = std::env::var(env_keys::ENV_DISCOVERY_HOST_UNPREFIXED)
                    .unwrap_or_else(|_| env_keys::DEFAULT_DISCOVERY_HOST.to_string());
                let port = std::env::var(env_keys::ENV_DISCOVERY_PORT_UNPREFIXED)
                    .unwrap_or_else(|_| DEFAULT_CONSUL_HTTP_PORT.to_string());
                format!("http://{host}:{port}")
            });

        Self {
            enabled: true,
            service_id: Arc::from("beardog-discovery"),
            enabled_protocols: vec![DiscoveryProtocol::Http {
                endpoint: discovery_endpoint,
                timeout_ms: defaults::DEFAULT_MAX_RESPONSE_TIME_MS,
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
        let registry_endpoint = std::env::var(env_keys::ENV_SERVICE_REGISTRY_ENDPOINT)
            .or_else(|_| std::env::var(env_keys::ENV_CONSUL_HTTP_ADDR))
            .unwrap_or_else(|_| {
                let host = std::env::var(env_keys::ENV_REGISTRY_HOST)
                    .unwrap_or_else(|_| env_keys::DEFAULT_REGISTRY_HOST.to_string());
                let port = std::env::var(env_keys::ENV_REGISTRY_PORT)
                    .unwrap_or_else(|_| DEFAULT_CONSUL_HTTP_PORT.to_string());
                format!("http://{host}:{port}")
            });

        Self {
            backend: "consul".to_string(),
            endpoints: vec![registry_endpoint],
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
        use beardog_config::domains::network_ports::{DEFAULT_API_PORT, DEFAULT_DISCOVERY_PORT};

        Self {
            protocols: vec!["http".to_string(), "grpc".to_string()],
            ports: vec![DEFAULT_API_PORT, DEFAULT_DISCOVERY_PORT],
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
            eviction_policy: Arc::from("lru"),
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

impl UnifiedDiscoveryConfig {
    /// Load from a custom environment provider (e.g. tests); production uses [`BearDogConfig::from_env`].
    ///
    /// # Errors
    ///
    /// Returns an error if the merged configuration fails [`BearDogConfig::validate`].
    pub fn from_env_provider(get: impl Fn(&str) -> Option<String>) -> Result<Self, BearDogError> {
        let mut config = Self::default();

        if let Some(enabled) = get(env_keys::ENV_DISCOVERY_ENABLED) {
            config.enabled = enabled.parse().unwrap_or(true);
        }
        if let Some(service_id) = get(env_keys::ENV_DISCOVERY_SERVICE_ID) {
            config.service_id = Arc::from(service_id.as_str());
        }

        if let Some(backend) = get(env_keys::ENV_REGISTRY_BACKEND) {
            config.registry.backend = backend;
        }
        if let Some(endpoints) = get(env_keys::ENV_REGISTRY_ENDPOINTS) {
            config.registry.endpoints =
                endpoints.split(',').map(|s| s.trim().to_string()).collect();
        }
        if let Some(ttl) = get(env_keys::ENV_REGISTRY_SERVICE_TTL_SECS)
            && let Ok(secs) = ttl.parse::<u64>()
        {
            config.registry.service_ttl = Duration::from_secs(secs);
        }
        if let Some(health_interval) = get(env_keys::ENV_REGISTRY_HEALTH_CHECK_INTERVAL_SECS)
            && let Ok(secs) = health_interval.parse::<u64>()
        {
            config.registry.health_check_interval = Duration::from_secs(secs);
        }
        if let Some(cleanup_interval) = get(env_keys::ENV_REGISTRY_CLEANUP_INTERVAL_SECS)
            && let Ok(secs) = cleanup_interval.parse::<u64>()
        {
            config.registry.cleanup_interval = Duration::from_secs(secs);
        }

        if let Some(ports) = get(env_keys::ENV_DISCOVERY_PORTS)
            && let Some(port_vec) = ports
                .split(',')
                .map(|p| p.trim().parse::<u16>().ok())
                .collect::<Option<Vec<u16>>>()
        {
            config.network.ports = port_vec;
        }
        if let Some(timeout) = get(env_keys::ENV_DISCOVERY_TIMEOUT_SECS)
            && let Ok(secs) = timeout.parse::<u64>()
        {
            config.network.timeout = Duration::from_secs(secs);
        }

        if let Some(cache_enabled) = get(env_keys::ENV_DISCOVERY_CACHE_ENABLED) {
            config.cache.enabled = cache_enabled.parse().unwrap_or(true);
        }
        if let Some(cache_size) = get(env_keys::ENV_DISCOVERY_CACHE_SIZE)
            && let Ok(size) = cache_size.parse::<usize>()
        {
            config.cache.size = size;
        }
        if let Some(cache_ttl) = get(env_keys::ENV_DISCOVERY_CACHE_TTL_SECS)
            && let Ok(secs) = cache_ttl.parse::<u64>()
        {
            config.cache.ttl = Duration::from_secs(secs);
        }

        if let Some(quantum_enabled) = get(env_keys::ENV_QUANTUM_DISCOVERY_ENABLED) {
            config.quantum.enabled = quantum_enabled.parse().unwrap_or(false);
        }
        if let Some(coherence) = get(env_keys::ENV_QUANTUM_COHERENCE_TIME_MS)
            && let Ok(ms) = coherence.parse::<u64>()
        {
            config.quantum.coherence_time = Duration::from_millis(ms);
        }

        if let Some(security_enabled) = get(env_keys::ENV_DISCOVERY_SECURITY_ENABLED) {
            config.security.enabled = security_enabled.parse().unwrap_or(true);
        }
        if let Some(auth_required) = get(env_keys::ENV_DISCOVERY_AUTH_REQUIRED) {
            config.security.auth_required = auth_required.parse().unwrap_or(false);
        }
        if let Some(encryption_required) = get(env_keys::ENV_DISCOVERY_ENCRYPTION_REQUIRED) {
            config.security.encryption_required = encryption_required.parse().unwrap_or(false);
        }

        config.validate()?;
        Ok(config)
    }
}

// --- BearDogConfig Trait Implementation ---

impl BearDogConfig for UnifiedDiscoveryConfig {
    fn validate(&self) -> Result<(), BearDogError> {
        if self.enabled {
            // Service ID validation
            if self.service_id.is_empty() {
                return Err(BearDogError::validation(
                    "Service ID cannot be empty when discovery is enabled",
                ));
            }

            // Registry validation
            if self.registry.endpoints.is_empty() {
                return Err(BearDogError::validation(
                    "Registry endpoints cannot be empty",
                ));
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
                return Err(BearDogError::validation(
                    "Network discovery timeout cannot be zero",
                ));
            }
            if self.network.max_packet_size == 0 {
                return Err(BearDogError::validation("Max packet size cannot be zero"));
            }

            // Cache validation
            if self.cache.enabled && self.cache.size == 0 {
                return Err(BearDogError::validation(
                    "Cache size cannot be zero when caching is enabled",
                ));
            }

            // Quantum validation
            if self.quantum.enabled {
                if self.quantum.error_threshold < 0.0 || self.quantum.error_threshold > 1.0 {
                    return Err(BearDogError::validation(
                        "Quantum error threshold must be between 0.0 and 1.0",
                    ));
                }
                if self.quantum.max_superposition_states == 0 {
                    return Err(BearDogError::validation(
                        "Max superposition states cannot be zero",
                    ));
                }
            }
        }
        Ok(())
    }

    fn merge(&self, other: &Self) -> Result<Self, BearDogError> {
        Ok(Self {
            enabled: other.enabled,
            service_id: if other.service_id.is_empty() {
                self.service_id.clone()
            } else {
                other.service_id.clone()
            },
            enabled_protocols: if other.enabled_protocols.is_empty() {
                self.enabled_protocols.clone()
            } else {
                other.enabled_protocols.clone()
            },
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
            load_balancing: other.load_balancing.clone(),
        })
    }

    fn from_env() -> Result<Self, BearDogError> {
        Self::from_env_provider(|k| std::env::var(k).ok())
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
