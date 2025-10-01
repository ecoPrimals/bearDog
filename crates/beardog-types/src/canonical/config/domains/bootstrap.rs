//! # Bootstrap Configuration Domain
//!
//! This module consolidates all bootstrap-related configuration types for zero-knowledge
//! bootstrap operations, infant pattern discovery, and ecosystem self-discovery.
//!
//! ## Consolidated Configs
//!
//! This module unifies:
//! - `BootstrapConfig` from `beardog-core/src/zero_knowledge_bootstrap/`
//! - `InfantPatternConfig` from `beardog-core/src/zero_knowledge_bootstrap/infant_patterns.rs`
//! - Bootstrap-related discovery configurations
//!
//! ## Migration Status
//! - Phase 1: Created canonical location (October 2025)
//! - Phase 2: Migrate configs and deprecate old locations
//! - Phase 3: Update imports across codebase

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::canonical::config::r#trait::BearDogConfig;

/// **UNIFIED BOOTSTRAP CONFIGURATION** - Zero-knowledge bootstrap settings
///
/// This configuration consolidates all bootstrap-related settings for BearDog's
/// revolutionary zero-knowledge bootstrap system that discovers the ecosystem
/// without any hardcoded assumptions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnifiedBootstrapConfig {
    /// Core bootstrap settings
    pub core: CoreBootstrapConfig,
    
    /// Infant pattern discovery settings
    pub infant_patterns: InfantPatternConfig,
    
    /// Discovery protocol settings
    pub discovery: BootstrapDiscoveryConfig,
    
    /// Network listening settings
    pub network: BootstrapNetworkConfig,
    
    /// Performance and optimization settings
    pub performance: BootstrapPerformanceConfig,
}

/// Core bootstrap configuration settings
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoreBootstrapConfig {
    /// Discovery timeout in milliseconds
    pub discovery_timeout_ms: u64,
    
    /// Maximum number of discovery attempts
    pub max_discovery_attempts: u32,
    
    /// Minimum capabilities threshold before considering bootstrap successful
    pub min_capabilities_threshold: usize,
    
    /// Enable passive listening mode
    pub enable_passive_listening: bool,
    
    /// Bootstrap retry strategy
    pub retry_strategy: RetryStrategy,
}

impl Default for CoreBootstrapConfig {
    fn default() -> Self {
        Self {
            discovery_timeout_ms: 30000, // 30 seconds
            max_discovery_attempts: 5,
            min_capabilities_threshold: 3,
            enable_passive_listening: true,
            retry_strategy: RetryStrategy::Exponential,
        }
    }
}

/// Bootstrap retry strategy
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RetryStrategy {
    /// No retries
    None,
    /// Fixed delay between retries
    Fixed,
    /// Exponential backoff
    Exponential,
    /// Adaptive based on success rate
    Adaptive,
}

/// **INFANT PATTERN CONFIGURATION** - Zero-knowledge learning settings
///
/// Configuration for the infant pattern discovery system that learns
/// ecosystem patterns through observation without hardcoded knowledge.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InfantPatternConfig {
    /// Minimum observations before forming a pattern
    pub min_observations: u64,
    
    /// Confidence threshold for pattern acceptance (0.0-1.0)
    pub confidence_threshold: f64,
    
    /// Maximum age for patterns before re-evaluation
    pub pattern_max_age: Duration,
    
    /// Learning rate for confidence updates (0.0-1.0)
    pub learning_rate: f64,
    
    /// Enable continuous learning
    pub enable_continuous_learning: bool,
    
    /// Pattern consolidation interval
    pub consolidation_interval: Duration,
}

impl Default for InfantPatternConfig {
    fn default() -> Self {
        Self {
            min_observations: 5,
            confidence_threshold: 0.7,
            pattern_max_age: Duration::from_secs(3600), // 1 hour
            learning_rate: 0.1,
            enable_continuous_learning: true,
            consolidation_interval: Duration::from_secs(300), // 5 minutes
        }
    }
}

/// Bootstrap discovery protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BootstrapDiscoveryConfig {
    /// Discovery protocols to enable
    pub enabled_protocols: Vec<DiscoveryProtocol>,
    
    /// Protocol-specific timeouts
    pub protocol_timeouts: ProtocolTimeouts,
    
    /// Maximum parallel discovery operations
    pub max_parallel_discoveries: usize,
    
    /// Enable fallback protocols
    pub enable_fallback: bool,
}

impl Default for BootstrapDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled_protocols: vec![
                DiscoveryProtocol::MulticastDNS,
                DiscoveryProtocol::HttpDiscovery,
                DiscoveryProtocol::EnvironmentDiscovery,
            ],
            protocol_timeouts: ProtocolTimeouts::default(),
            max_parallel_discoveries: 10,
            enable_fallback: true,
        }
    }
}

/// Discovery protocol types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DiscoveryProtocol {
    /// Multicast DNS discovery
    MulticastDNS,
    /// HTTP-based discovery endpoints
    HttpDiscovery,
    /// Environment variable discovery
    EnvironmentDiscovery,
    /// Service mesh integration
    ServiceMeshDiscovery,
    /// Container orchestration discovery
    ContainerDiscovery,
    /// Cloud provider metadata
    CloudMetadataDiscovery,
}

/// Protocol-specific timeout configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProtocolTimeouts {
    /// mDNS discovery timeout
    pub mdns_timeout_ms: u64,
    /// HTTP discovery timeout
    pub http_timeout_ms: u64,
    /// Environment discovery timeout
    pub env_timeout_ms: u64,
    /// Service mesh timeout
    pub mesh_timeout_ms: u64,
    /// Container orchestration timeout
    pub container_timeout_ms: u64,
}

impl Default for ProtocolTimeouts {
    fn default() -> Self {
        Self {
            mdns_timeout_ms: 5000,
            http_timeout_ms: 10000,
            env_timeout_ms: 1000,
            mesh_timeout_ms: 15000,
            container_timeout_ms: 10000,
        }
    }
}

/// Bootstrap network configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BootstrapNetworkConfig {
    /// Network interface to listen on (defaults to all interfaces)
    pub listen_interface: String,
    
    /// Multicast group for mDNS
    pub multicast_group: String,
    
    /// Discovery port
    pub discovery_port: u16,
    
    /// Enable IPv6 discovery
    pub enable_ipv6: bool,
    
    /// Network buffer size
    pub buffer_size: usize,
}

impl Default for BootstrapNetworkConfig {
    fn default() -> Self {
        Self {
            listen_interface: "0.0.0.0".to_string(),
            multicast_group: "224.0.0.251".to_string(),
            discovery_port: 5353,
            enable_ipv6: true,
            buffer_size: 8192,
        }
    }
}

/// Bootstrap performance and optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BootstrapPerformanceConfig {
    /// Enable caching of discovery results
    pub enable_caching: bool,
    
    /// Cache duration
    pub cache_duration: Duration,
    
    /// Enable parallel discovery
    pub enable_parallel: bool,
    
    /// Worker thread count (0 = auto)
    pub worker_threads: usize,
    
    /// Enable metrics collection
    pub enable_metrics: bool,
}

impl Default for BootstrapPerformanceConfig {
    fn default() -> Self {
        Self {
            enable_caching: true,
            cache_duration: Duration::from_secs(300),
            enable_parallel: true,
            worker_threads: 0, // Auto-detect
            enable_metrics: true,
        }
    }
}

impl Default for UnifiedBootstrapConfig {
    fn default() -> Self {
        Self {
            core: CoreBootstrapConfig::default(),
            infant_patterns: InfantPatternConfig::default(),
            discovery: BootstrapDiscoveryConfig::default(),
            network: BootstrapNetworkConfig::default(),
            performance: BootstrapPerformanceConfig::default(),
        }
    }
}

impl BearDogConfig for UnifiedBootstrapConfig {
    fn validate(&self) -> BearDogResult<()> {
        // Validate core settings
        if self.core.discovery_timeout_ms == 0 {
            return Err(BearDogError::configuration(
                "Bootstrap discovery timeout must be greater than 0",
            ));
        }
        
        if self.core.max_discovery_attempts == 0 {
            return Err(BearDogError::configuration(
                "Max discovery attempts must be greater than 0",
            ));
        }
        
        // Validate infant pattern settings
        if !(0.0..=1.0).contains(&self.infant_patterns.confidence_threshold) {
            return Err(BearDogError::configuration(
                "Confidence threshold must be between 0.0 and 1.0",
            ));
        }
        
        if !(0.0..=1.0).contains(&self.infant_patterns.learning_rate) {
            return Err(BearDogError::configuration(
                "Learning rate must be between 0.0 and 1.0",
            ));
        }
        
        // Validate discovery settings
        if self.discovery.enabled_protocols.is_empty() {
            return Err(BearDogError::configuration(
                "At least one discovery protocol must be enabled",
            ));
        }
        
        Ok(())
    }
    
    fn apply_environment_overrides(&mut self, _environment: &str) -> BearDogResult<()> {
        // Environment-specific overrides can be implemented here
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_bootstrap_config() {
        let config = UnifiedBootstrapConfig::default();
        assert!(config.validate().is_ok());
    }
    
    #[test]
    fn test_invalid_confidence_threshold() {
        let mut config = UnifiedBootstrapConfig::default();
        config.infant_patterns.confidence_threshold = 1.5; // Invalid: > 1.0
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_zero_discovery_timeout() {
        let mut config = UnifiedBootstrapConfig::default();
        config.core.discovery_timeout_ms = 0;
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_empty_protocols() {
        let mut config = UnifiedBootstrapConfig::default();
        config.discovery.enabled_protocols.clear();
        assert!(config.validate().is_err());
    }
} 