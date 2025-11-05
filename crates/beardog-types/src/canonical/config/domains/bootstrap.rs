//! # Bootstrap Configuration Domain
//!
//! Configuration for BearDog's revolutionary **zero-knowledge bootstrap system** that
//! eliminates the 2^n hardcoding problem by discovering the ecosystem dynamically.
//!
//! ## Zero-Knowledge Bootstrap
//!
//! Traditional systems require hardcoding knowledge about every possible service.
//! BearDog uses **infant learning patterns** to:
//! - Start with zero ecosystem knowledge
//! - Discover services through observation
//! - Learn patterns without assumptions
//! - Scale to any ecosystem size with O(1) configuration
//!
//! ## Core Principle
//!
//! **"Each primal only knows itself and discovers others via the universal adapter"**
//!
//! This eliminates exponential configuration complexity (2^n) and replaces it with
//! constant configuration (O(1)) using universal patterns.
//!
//! ## Configuration Layers
//!
//! 1. **Core Bootstrap** - Timeouts, retries, thresholds
//! 2. **Infant Patterns** - Learning and observation settings
//! 3. **Discovery Protocols** - mDNS, HTTP, environment, service mesh
//! 4. **Network** - Listening interfaces and connectivity
//! 5. **Performance** - Caching, concurrency, optimization
//!
//! ## Quick Start
//!
//! ```rust
//! use beardog_types::canonical::config::domains::bootstrap::{
//!     UnifiedBootstrapConfig, CoreBootstrapConfig, RetryStrategy
//! };
//!
//! // Use defaults for most cases
//! let config = UnifiedBootstrapConfig::default();
//!
//! // Or customize for specific needs
//! let mut config = UnifiedBootstrapConfig::default();
//! config.core.discovery_timeout_ms = 60000; // 1 minute
//! config.core.max_discovery_attempts = 10;
//! config.core.min_capabilities_threshold = 5;
//! ```
//!
//! ## Consolidated Configs
//!
//! This module unifies:
//! - `BootstrapConfig` from `beardog-core/src/zero_knowledge_bootstrap/`
//! - `InfantPatternConfig` from infant patterns module
//! - Bootstrap-related discovery configurations
//!
//! ## Migration Status
//! - ✅ Phase 1: Created canonical location (October 2025)
//! - ✅ Phase 2: Migrated configs and deprecated old locations
//! - 🔄 Phase 3: Updating imports across codebase

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::canonical::config::r#trait::BearDogConfig;

/// Unified Bootstrap Configuration - Zero-Knowledge Bootstrap Settings
///
/// Complete configuration for BearDog's revolutionary zero-knowledge bootstrap system
/// that discovers the ecosystem without any hardcoded assumptions.
///
/// ## Architecture
///
/// The bootstrap process follows these phases:
/// 1. **Self-Discovery** - Learn own capabilities (core settings)
/// 2. **Pattern Observation** - Watch ecosystem behavior (infant patterns)
/// 3. **Service Discovery** - Find other primals (discovery protocols)
/// 4. **Network Listening** - Passive observation (network settings)
/// 5. **Optimization** - Performance tuning (performance settings)
///
/// ## Key Innovation
///
/// Eliminates the **2^n hardcoding problem**:
/// - Traditional: Need config for every possible service combination
/// - BearDog: O(1) config using universal patterns
///
/// ## Example
///
/// ```rust
/// use beardog_types::canonical::config::domains::bootstrap::{
///     UnifiedBootstrapConfig, CoreBootstrapConfig, RetryStrategy
/// };
///
/// // Quick start with sensible defaults
/// let config = UnifiedBootstrapConfig::default();
///
/// // Customize for production
/// let mut prod_config = UnifiedBootstrapConfig::default();
/// prod_config.core.discovery_timeout_ms = 60000; // Extended timeout
/// prod_config.core.max_discovery_attempts = 10;   // More retries
/// prod_config.core.min_capabilities_threshold = 5; // Require more capabilities
/// prod_config.core.enable_passive_listening = true;
/// prod_config.core.retry_strategy = RetryStrategy::Exponential;
///
/// // Use the config
/// println!("Timeout: {}ms", prod_config.core.discovery_timeout_ms);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnifiedBootstrapConfig {
    /// Core bootstrap settings (timeouts, retries, thresholds)
    pub core: CoreBootstrapConfig,

    /// Infant pattern discovery settings (learning, observation, confidence)
    pub infant_patterns: InfantPatternConfig,

    /// Discovery protocol settings (mDNS, HTTP, environment, service mesh)
    pub discovery: BootstrapDiscoveryConfig,

    /// Network listening settings (interfaces, ports, connectivity)
    pub network: BootstrapNetworkConfig,

    /// Performance and optimization settings (caching, concurrency, limits)
    pub performance: BootstrapPerformanceConfig,
}

/// Core Bootstrap Configuration - Essential Bootstrap Settings
///
/// Core settings that control the fundamental bootstrap behavior including
/// timeouts, retry logic, and success criteria.
///
/// ## Configuration Guidelines
///
/// - **Discovery Timeout**: How long to wait for responses (default: 30s)
/// - **Max Attempts**: Number of retry attempts (default: 5)
/// - **Capability Threshold**: Minimum capabilities to consider success (default: 3)
/// - **Passive Listening**: Enable continuous ecosystem monitoring (default: true)
/// - **Retry Strategy**: How to handle failures (default: exponential backoff)
///
/// ## Example
///
/// ```rust
/// use beardog_types::canonical::config::domains::bootstrap::{
///     CoreBootstrapConfig, RetryStrategy
/// };
///
/// // Production configuration
/// let config = CoreBootstrapConfig {
///     discovery_timeout_ms: 60000,        // 1 minute for slow networks
///     max_discovery_attempts: 10,         // More retries in prod
///     min_capabilities_threshold: 5,      // Require more capabilities
///     enable_passive_listening: true,     // Always listen
///     retry_strategy: RetryStrategy::Exponential,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoreBootstrapConfig {
    /// Discovery timeout in milliseconds (default: 30000ms = 30s)
    ///
    /// How long to wait for discovery responses before timing out.
    /// Increase for slow networks or large ecosystems.
    pub discovery_timeout_ms: u64,

    /// Maximum number of discovery attempts (default: 5)
    ///
    /// How many times to retry discovery before giving up.
    /// Increase for unreliable networks.
    pub max_discovery_attempts: u32,

    /// Minimum capabilities threshold (default: 3)
    ///
    /// Minimum number of discovered capabilities before considering
    /// bootstrap successful. Increase to ensure ecosystem readiness.
    pub min_capabilities_threshold: usize,

    /// Enable passive listening mode (default: true)
    ///
    /// When enabled, continues listening for new primals even after
    /// initial bootstrap completes. Recommended for production.
    pub enable_passive_listening: bool,

    /// Bootstrap retry strategy (default: Exponential)
    ///
    /// Strategy to use when discovery attempts fail.
    pub retry_strategy: RetryStrategy,
}

impl Default for CoreBootstrapConfig {
    fn default() -> Self {
        Self {
            discovery_timeout_ms: std::env::var("BEARDOG_DISCOVERY_TIMEOUT_MS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30000), // 30 seconds default
            max_discovery_attempts: std::env::var("BEARDOG_MAX_DISCOVERY_ATTEMPTS")
                .ok()
                .and_then(|a| a.parse().ok())
                .unwrap_or(5),
            min_capabilities_threshold: std::env::var("BEARDOG_MIN_CAPABILITIES")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(3),
            enable_passive_listening: true,
            retry_strategy: RetryStrategy::Exponential,
        }
    }
}

/// Bootstrap Retry Strategy - How to Handle Discovery Failures
///
/// Defines the strategy for retrying failed discovery attempts during bootstrap.
///
/// ## Strategies
///
/// - **None** - Don't retry, fail immediately (not recommended)
/// - **Fixed** - Fixed delay between retries (simple, predictable)
/// - **Exponential** - Exponential backoff (recommended, prevents flooding)
/// - **Adaptive** - Adapts based on success rate (smart, complex)
///
/// ## Recommendations
///
/// - **Development**: Use Fixed for predictable behavior
/// - **Production**: Use Exponential to handle transient failures gracefully
/// - **High-scale**: Use Adaptive to optimize for specific conditions
///
/// ## Example
///
/// ```rust
/// use beardog_types::canonical::config::domains::bootstrap::RetryStrategy;
///
/// // Production: Exponential backoff (1s, 2s, 4s, 8s, 16s)
/// let strategy = RetryStrategy::Exponential;
///
/// // Development: Fixed 2-second delay
/// let strategy = RetryStrategy::Fixed;
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RetryStrategy {
    /// No retries - fail immediately on first attempt
    ///
    /// Use only when failures are unacceptable or retries are handled
    /// at a higher level.
    None,

    /// Fixed delay between retries
    ///
    /// Simple strategy with consistent delay. Good for development
    /// and testing where predictable behavior is important.
    Fixed,

    /// Exponential backoff (recommended)
    ///
    /// Delays increase exponentially (1s, 2s, 4s, 8s, 16s, ...).
    /// Prevents overwhelming the network during transient failures.
    /// Best for production environments.
    Exponential,

    /// Adaptive strategy based on success rate
    ///
    /// Adjusts retry behavior based on historical success rates.
    /// More complex but optimizes for specific conditions.
    /// Good for high-scale deployments.
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
            pattern_max_age: Duration::from_secs(
                std::env::var("BEARDOG_PATTERN_MAX_AGE_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600), // 1 hour default
            ),
            learning_rate: 0.1,
            enable_continuous_learning: true,
            consolidation_interval: Duration::from_secs(
                std::env::var("BEARDOG_PATTERN_CONSOLIDATION_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300), // 5 minutes default
            ),
        }
    }
}

/// Bootstrap discovery protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
            mdns_timeout_ms: std::env::var("BEARDOG_MDNS_TIMEOUT_MS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(5000),
            http_timeout_ms: std::env::var("BEARDOG_HTTP_TIMEOUT_MS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(10000),
            env_timeout_ms: std::env::var("BEARDOG_ENV_TIMEOUT_MS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(1000),
            mesh_timeout_ms: std::env::var("BEARDOG_MESH_TIMEOUT_MS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(15000),
            container_timeout_ms: std::env::var("BEARDOG_CONTAINER_TIMEOUT_MS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(10000),
        }
    }
}

/// Bootstrap network configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
            listen_interface: crate::constants::domains::network::addresses::default_bind_address(),
            multicast_group: crate::constants::domains::network::addresses::multicast_address(),
            discovery_port: std::env::var("BEARDOG_BOOTSTRAP_DISCOVERY_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(5353), // mDNS standard port
            enable_ipv6: true,
            buffer_size: std::env::var("BEARDOG_BOOTSTRAP_BUFFER_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8192), // 8KB default buffer
        }
    }
}

/// Bootstrap performance and optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
            cache_duration: Duration::from_secs(
                std::env::var("BEARDOG_BOOTSTRAP_CACHE_DURATION_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(300),
            ),
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

    fn merge(&self, other: &Self) -> BearDogResult<Self> {
        // Simple merge: other takes precedence
        Ok(other.clone())
    }

    fn from_env() -> BearDogResult<Self> {
        // Return defaults for now; can be enhanced to read from env vars
        Ok(Self::default())
    }

    fn to_toml(&self) -> BearDogResult<String> {
        toml::to_string_pretty(self).map_err(|e| {
            BearDogError::configuration(&format!("Failed to serialize to TOML: {}", e))
        })
    }

    fn domain() -> &'static str {
        "bootstrap"
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
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        config.infant_patterns.confidence_threshold = 1.5; // Invalid: > 1.0
        assert!(config.validate().is_err());
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: important

    #[test]
    fn test_zero_discovery_timeout() {
        let mut config = UnifiedBootstrapConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        config.core.discovery_timeout_ms = 0;
        assert!(config.validate().is_err());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_empty_protocols() {
        let mut config = UnifiedBootstrapConfig::default();
        config.discovery.enabled_protocols.clear();
        assert!(config.validate().is_err());
    }
}
