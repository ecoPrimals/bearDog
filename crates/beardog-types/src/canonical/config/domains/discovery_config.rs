//! # Discovery Configuration Domain
//!
//! This module contains all service discovery-related configuration types, extracted from
//! the large `consolidated_domains.rs` file for better maintainability.

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::canonical::config::r#trait::BearDogConfig;

/// **CONSOLIDATED DISCOVERY CONFIGURATION** - Unifies all discovery configs
///
/// Consolidates: `DiscoveryConfig`, `ServiceRegistryConfig`, `QuantumDiscoveryConfig`, etc.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConsolidatedDiscoveryConfig {
    /// Enable discovery services
    pub enabled: bool,
    
    /// Service registry configuration
    pub registry: ServiceRegistryConfig,
    
    /// Network discovery configuration
    pub network: NetworkDiscoveryConfig,
    
    /// Quantum discovery configuration
    pub quantum: QuantumDiscoveryConfig,
    
    /// Cache configuration for discovery
    pub cache: DiscoveryCacheConfig,
    
    /// Security configuration for discovery
    pub security: DiscoverySecurityConfig,
}

/// Service registry configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServiceRegistryConfig {
    /// Registry backend type
    pub backend: String,
    
    /// Registry endpoints
    pub endpoints: Vec<String>,
    
    /// Service TTL
    pub service_ttl: Duration,
    
    /// Health check interval
    pub health_check_interval: Duration,
    
    /// Cleanup interval
    pub cleanup_interval: Duration,
}

/// Network discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkDiscoveryConfig {
    /// Discovery protocols
    pub protocols: Vec<String>,
    
    /// Discovery ports
    pub ports: Vec<u16>,
    
    /// Discovery timeout
    pub timeout: Duration,
    
    /// Retry configuration
    pub retry: RetryConfig,
}

/// Quantum discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuantumDiscoveryConfig {
    /// Enable quantum discovery
    pub enabled: bool,
    
    /// Quantum algorithms
    pub algorithms: Vec<String>,
    
    /// Quantum coherence time
    pub coherence_time: Duration,
    
    /// Error correction threshold
    pub error_threshold: f64,
}

/// Discovery cache configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiscoveryCacheConfig {
    /// Enable caching
    pub enabled: bool,
    
    /// Cache size
    pub size: usize,
    
    /// Cache TTL
    pub ttl: Duration,
    
    /// Eviction policy
    pub eviction_policy: String,
}

/// Discovery security configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiscoverySecurityConfig {
    /// Enable security
    pub enabled: bool,
    
    /// Authentication required
    pub auth_required: bool,
    
    /// Encryption required
    pub encryption_required: bool,
    
    /// Trusted networks
    pub trusted_networks: Vec<String>,
}

/// Retry configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetryConfig {
    /// Maximum retry attempts
    pub max_attempts: usize,
    
    /// Initial delay
    pub initial_delay: Duration,
    
    /// Backoff multiplier
    pub backoff_multiplier: f64,
    
    /// Maximum delay
    pub max_delay: Duration,
}

// Default implementations
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
        Self {
            backend: "consul".to_string(),
            endpoints: vec!["http://localhost:8500".to_string()],
            service_ttl: Duration::from_secs(30),
            health_check_interval: Duration::from_secs(10),
            cleanup_interval: Duration::from_secs(60),
        }
    }
}

impl Default for NetworkDiscoveryConfig {
    fn default() -> Self {
        Self {
            protocols: vec!["http".to_string(), "grpc".to_string()],
            ports: vec![8080, 9090],
            timeout: Duration::from_secs(5),
            retry: RetryConfig::default(),
        }
    }
}

impl Default for QuantumDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: false, // Quantum discovery is experimental
            algorithms: vec!["grover".to_string(), "shor".to_string()],
            coherence_time: Duration::from_millis(100),
            error_threshold: 0.01,
        }
    }
}

impl Default for DiscoveryCacheConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            size: 1000,
            ttl: Duration::from_secs(300), // 5 minutes
            eviction_policy: "lru".to_string(),
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

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            backoff_multiplier: 2.0,
            max_delay: Duration::from_secs(30),
        }
    }
}

// BearDogConfig implementation for ConsolidatedDiscoveryConfig
impl BearDogConfig for ConsolidatedDiscoveryConfig {
    fn validate(&self) -> BearDogResult<()> {
        if self.enabled {
            if self.registry.endpoints.is_empty() {
                return Err(BearDogError::validation("Registry endpoints cannot be empty"));
            }
            
            if self.registry.service_ttl.as_secs() == 0 {
                return Err(BearDogError::validation("Service TTL cannot be zero"));
            }
            
            if self.network.ports.is_empty() {
                return Err(BearDogError::validation("Discovery ports cannot be empty"));
            }
            
            if self.network.timeout.as_secs() == 0 {
                return Err(BearDogError::validation("Network discovery timeout cannot be zero"));
            }
            
            if self.cache.enabled && self.cache.size == 0 {
                return Err(BearDogError::validation("Cache size cannot be zero when caching is enabled"));
            }
            
            if self.quantum.enabled && (self.quantum.error_threshold < 0.0 || self.quantum.error_threshold > 1.0) {
                return Err(BearDogError::validation("Quantum error threshold must be between 0.0 and 1.0"));
            }
        }
        Ok(())
    }
    
    fn merge(&self, other: &Self) -> BearDogResult<Self> {
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
    
    fn from_env() -> BearDogResult<Self> {
        let mut config = Self::default();
        
        if let Ok(enabled) = std::env::var("BEARDOG_DISCOVERY_ENABLED") {
            config.enabled = enabled.parse().unwrap_or(true);
        }
        
        if let Ok(backend) = std::env::var("BEARDOG_DISCOVERY_BACKEND") {
            config.registry.backend = backend;
        }
        
        if let Ok(endpoints) = std::env::var("BEARDOG_DISCOVERY_ENDPOINTS") {
            config.registry.endpoints = endpoints.split(',').map(|s| s.trim().to_string()).collect();
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
    
    fn to_toml(&self) -> BearDogResult<String> {
        toml::to_string(self)
            .map_err(|e| BearDogError::system(format!("Failed to serialize discovery config to TOML: {e}")))
    }
    
    fn domain() -> &'static str {
        "discovery"
    }
} 