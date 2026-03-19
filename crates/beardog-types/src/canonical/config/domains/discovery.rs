// SPDX-License-Identifier: AGPL-3.0-only

//! # Discovery Configuration
//!
//! Canonical discovery configuration for all discovery types across the BearDog ecosystem.
//!
//! This module provides the base `DiscoveryConfig` struct that should be used by all crates.
//! Domain-specific discovery needs should extend this via composition, not by redefining
//! new discovery config structs.
//!
//! ## Architecture
//!
//! - **Base**: `DiscoveryConfig` - Common fields for all discovery types
//! - **Extensions**: Domain-specific wrappers (e.g., `HsmDiscoveryConfig`, `BiomeDiscoveryConfig`)
//!
//! ## Migration
//!
//! This consolidates 8 fragmented `DiscoveryConfig` definitions into a single canonical source.
//! See `DISCOVERY_CONFIG_CONSOLIDATION_PLAN.md` for details.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Canonical discovery configuration for all discovery types
///
/// This is the single source of truth for discovery configuration across BearDog.
/// All crates should import this instead of defining their own `DiscoveryConfig`.
///
/// ## Usage
///
/// ```rust
/// use beardog_types::canonical::config::domains::discovery::DiscoveryConfig;
/// use std::time::Duration;
///
/// let config = DiscoveryConfig::default();
/// assert_eq!(config.timeout, Duration::from_secs(5));
/// ```
///
/// ## Domain Extensions
///
/// If you need domain-specific fields:
/// - **HSM**: Use `beardog_types::canonical::hsm::discovery::HsmDiscoveryConfig`
/// - **Biome**: Use `beardog_types::canonical::biome::discovery::BiomeDiscoveryConfig`
///
/// Do NOT create new discovery config structs in your crate.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiscoveryConfig {
    // ═══════════════════════════════════════════════════════
    // Core Discovery
    // ═══════════════════════════════════════════════════════
    /// Enable discovery feature
    ///
    /// Set to `false` to completely disable discovery.
    pub enabled: bool,

    /// Discovery operation timeout
    ///
    /// Maximum time to wait for a single discovery operation to complete.
    pub timeout: Duration,

    /// Maximum discovery attempts
    ///
    /// Number of times to retry a failed discovery before giving up.
    pub max_attempts: u32,

    /// Maximum concurrent discovery operations
    ///
    /// Limits parallelism to prevent resource exhaustion.
    pub max_concurrent: usize,

    /// Discovery interval
    ///
    /// How often to perform discovery sweeps (e.g., for new services).
    pub discovery_interval: Duration,

    /// Discovery refresh interval
    ///
    /// How often to refresh known services to check health.
    pub refresh_interval: Duration,

    // ═══════════════════════════════════════════════════════
    // Caching
    // ═══════════════════════════════════════════════════════
    /// Enable discovery result caching
    ///
    /// Caching improves performance by avoiding redundant discoveries.
    pub cache_enabled: bool,

    /// Cache time-to-live (TTL)
    ///
    /// How long to keep discovery results in cache before invalidating.
    pub cache_ttl: Duration,

    // ═══════════════════════════════════════════════════════
    // Endpoints
    // ═══════════════════════════════════════════════════════
    /// Discovery service endpoints
    ///
    /// List of discovery servers, registries, or broadcast addresses.
    /// Examples:
    /// - `https://discovery.ecosystem.internal:8500` (Consul)
    /// - `https://capabilities.ecosystem.internal:443` (Custom registry)
    /// - `udp://239.255.255.250:1900` (SSDP multicast)
    pub endpoints: Vec<String>,

    // ═══════════════════════════════════════════════════════
    // Health & Registration
    // ═══════════════════════════════════════════════════════
    /// Health check interval
    ///
    /// How often to check if discovered services are still alive.
    pub health_check_interval: Duration,

    /// Enable automatic registration
    ///
    /// If `true`, automatically register this service with discovery servers.
    pub auto_register: bool,

    // ═══════════════════════════════════════════════════════
    // Metadata
    // ═══════════════════════════════════════════════════════
    /// Service metadata for registration
    ///
    /// Key-value pairs attached to this service when registering.
    /// Examples: version, region, capabilities, tags.
    pub service_metadata: HashMap<String, String>,

    // ═══════════════════════════════════════════════════════
    // Features
    // ═══════════════════════════════════════════════════════
    /// Enable predictive discovery
    ///
    /// Use ML/heuristics to predict which services will be needed and pre-discover them.
    pub predictive_enabled: bool,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(5),
            max_attempts: 3,
            max_concurrent: 10,
            discovery_interval: Duration::from_secs(30),
            refresh_interval: Duration::from_secs(60),
            cache_enabled: true,
            cache_ttl: Duration::from_secs(300), // 5 minutes
            endpoints: vec![],
            health_check_interval: Duration::from_secs(60),
            auto_register: true,
            service_metadata: HashMap::new(),
            predictive_enabled: false,
        }
    }
}

impl DiscoveryConfig {
    /// Create a new discovery configuration with custom timeout
    ///
    /// All other fields use defaults.
    pub fn with_timeout(timeout: Duration) -> Self {
        Self {
            timeout,
            ..Default::default()
        }
    }

    /// Create a new discovery configuration with custom endpoints
    ///
    /// All other fields use defaults.
    pub fn with_endpoints(endpoints: Vec<String>) -> Self {
        Self {
            endpoints,
            ..Default::default()
        }
    }

    /// Create a minimal configuration for testing
    ///
    /// - Disabled caching
    /// - Short timeouts
    /// - No auto-registration
    pub fn for_testing() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_millis(100),
            max_attempts: 1,
            max_concurrent: 1,
            discovery_interval: Duration::from_secs(1),
            refresh_interval: Duration::from_secs(5),
            cache_enabled: false,
            cache_ttl: Duration::from_secs(1),
            endpoints: vec![],
            health_check_interval: Duration::from_secs(1),
            auto_register: false,
            service_metadata: HashMap::new(),
            predictive_enabled: false,
        }
    }

    /// Load from environment variables
    ///
    /// Supported env vars:
    /// - `BEARDOG_DISCOVERY_ENABLED`: "true" or "false"
    /// - `BEARDOG_DISCOVERY_TIMEOUT_SECS`: timeout in seconds
    /// - `BEARDOG_DISCOVERY_MAX_ATTEMPTS`: max retry attempts
    /// - `BEARDOG_DISCOVERY_ENDPOINT`: primary endpoint (can be repeated)
    /// - `BEARDOG_DISCOVERY_ENDPOINTS`: comma-separated list
    /// - `BEARDOG_CAPABILITY_REGISTRY`: additional endpoint
    ///
    /// Falls back to defaults for any missing vars.
    pub fn from_env() -> Self {
        let mut config = Self::default();

        // Enabled
        if let Ok(val) = std::env::var("BEARDOG_DISCOVERY_ENABLED") {
            config.enabled = val.to_lowercase() == "true";
        }

        // Timeout
        if let Ok(val) = std::env::var("BEARDOG_DISCOVERY_TIMEOUT_SECS") {
            if let Ok(secs) = val.parse::<u64>() {
                config.timeout = Duration::from_secs(secs);
            }
        }

        // Max attempts
        if let Ok(val) = std::env::var("BEARDOG_DISCOVERY_MAX_ATTEMPTS") {
            if let Ok(attempts) = val.parse::<u32>() {
                config.max_attempts = attempts;
            }
        }

        // Cache enabled
        if let Ok(val) = std::env::var("BEARDOG_DISCOVERY_CACHE_ENABLED") {
            config.cache_enabled = val.to_lowercase() == "true";
        }

        // Endpoints
        let mut endpoints = vec![];

        // Single endpoint
        if let Ok(endpoint) = std::env::var("BEARDOG_DISCOVERY_ENDPOINT") {
            endpoints.push(endpoint);
        }

        // Capability registry
        if let Ok(endpoint) = std::env::var("BEARDOG_CAPABILITY_REGISTRY") {
            endpoints.push(endpoint);
        }

        // Comma-separated list
        if let Ok(list) = std::env::var("BEARDOG_DISCOVERY_ENDPOINTS") {
            for endpoint in list.split(',') {
                let trimmed = endpoint.trim();
                if !trimmed.is_empty() {
                    endpoints.push(trimmed.to_string());
                }
            }
        }

        if !endpoints.is_empty() {
            config.endpoints = endpoints;
        }

        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = DiscoveryConfig::default();
        assert!(config.enabled);
        assert_eq!(config.timeout, Duration::from_secs(5));
        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.max_concurrent, 10);
        assert!(config.cache_enabled);
        assert!(config.auto_register);
    }

    #[test]
    fn test_with_timeout() {
        let config = DiscoveryConfig::with_timeout(Duration::from_secs(10));
        assert_eq!(config.timeout, Duration::from_secs(10));
        assert_eq!(config.max_attempts, 3); // Other fields still default
    }

    #[test]
    fn test_with_endpoints() {
        let endpoints = vec!["http://test:8500".to_string()];
        let config = DiscoveryConfig::with_endpoints(endpoints.clone());
        assert_eq!(config.endpoints, endpoints);
    }

    #[test]
    fn test_for_testing() {
        let config = DiscoveryConfig::for_testing();
        assert!(config.enabled);
        assert_eq!(config.timeout, Duration::from_millis(100));
        assert_eq!(config.max_attempts, 1);
        assert!(!config.cache_enabled);
        assert!(!config.auto_register);
    }
}
