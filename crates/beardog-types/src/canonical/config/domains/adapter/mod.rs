// SPDX-License-Identifier: AGPL-3.0-only

//! # Unified Adapter Configuration Module
//!
//! This module provides unified configuration for all adapter-related functionality,
//! consolidating 20+ scattered adapter configs into a single, maintainable structure.
//!
//! ## Module Organization
//!
//! - [`core`] - Core adapter settings and types
//! - [`chain`] - Chain processing and workflow configuration
//! - [`service_mesh`] - Service mesh and handoff configuration
//! - [`vendor`] - Vendor-specific adapter configuration
//! - [`security`] - Security and monitoring configuration

pub mod chain;
pub mod core;
pub mod security;
pub mod service_mesh;
pub mod vendor;

// Re-export main types for convenience
pub use chain::{ChainConfig, RetryConfig, StepConfig};
pub use core::{AdapterType, AuthLevel, CoreAdapterConfig, OptimizationConfig};
pub use security::{AdapterMonitoringConfig, AdapterSecurityConfig};
pub use service_mesh::{
    HandoffRetryConfig, HealthMonitorConfig, MeshDiscoveryConfig, MeshSecurityConfig,
    ServiceMeshConfig,
};
pub use vendor::{CloudProviderConfig, KmsConfig, VendorConfig};

use crate::canonical::config::r#trait::{validation, BearDogConfig};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

// Re-export canonical DiscoveryConfig instead of defining locally
pub use super::discovery::DiscoveryConfig;

/// **UNIFIED ADAPTER CONFIGURATION** - Single source of truth for all adapter functionality
///
/// This configuration consolidates all adapter-related settings into a single,
/// well-organized structure with domain-specific sub-configurations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UnifiedAdapterConfig {
    /// Core adapter settings
    pub core: CoreAdapterConfig,

    /// Discovery and capability detection settings
    pub discovery: DiscoveryConfig,

    /// Chain processing and workflow settings
    pub chain: ChainConfig,

    /// Performance optimization settings
    pub optimization: OptimizationConfig,

    /// Service mesh and handoff settings
    pub service_mesh: ServiceMeshConfig,

    /// Vendor-specific adapter settings
    pub vendor: VendorConfig,

    /// Security settings for adapters
    pub security: AdapterSecurityConfig,

    /// Monitoring and health check settings
    pub monitoring: AdapterMonitoringConfig,
}

impl BearDogConfig for UnifiedAdapterConfig {
    fn validate(&self) -> Result<(), BearDogError> {
        // Validate core configuration
        validation::validate_non_empty_string(&self.core.adapter_id, "adapter_id")?;
        validation::validate_range(self.core.max_connections, 1, 10000, "max_connections")?;

        // Validate discovery configuration
        if !self.discovery.endpoints.is_empty() {
            for endpoint in &self.discovery.endpoints {
                validation::validate_url(endpoint, "discovery_endpoint")?;
            }
        }

        // Validate chain configuration
        validation::validate_range(self.chain.max_chain_length, 1, 1000, "max_chain_length")?;

        // Validate optimization configuration
        validation::validate_range(self.optimization.level, 1, 5, "optimization_level")?;

        // Validate retry configuration
        if self.chain.retry.backoff_multiplier <= 1.0 {
            return Err(BearDogError::configuration(
                "Backoff multiplier must be greater than 1.0",
            ));
        }

        validation::validate_range(self.chain.retry.jitter_factor, 0.0, 1.0, "jitter_factor")?;

        // Validate service mesh configuration
        if self.service_mesh.enabled {
            validation::validate_port(self.service_mesh.discovery.port, "mesh_discovery_port")?;
        }

        Ok(())
    }

    fn merge(&self, other: &Self) -> Result<Self, BearDogError> {
        let mut merged = self.clone();

        // Merge core config
        if other.core.adapter_id.as_ref() != "default-adapter" {
            merged.core.adapter_id.clone_from(&other.core.adapter_id);
        }
        merged.core.max_connections = other.core.max_connections;
        merged.core.connection_timeout = other.core.connection_timeout;
        merged.core.registry_enabled = other.core.registry_enabled;

        // Merge discovery config
        if !other.discovery.endpoints.is_empty() {
            merged
                .discovery
                .endpoints
                .clone_from(&other.discovery.endpoints);
        }
        merged.discovery.timeout = other.discovery.timeout;
        merged.discovery.cache_enabled = other.discovery.cache_enabled;

        // Merge other sections
        merged.optimization = other.optimization.clone();
        merged.security = other.security.clone();
        merged.monitoring = other.monitoring.clone();

        Ok(merged)
    }

    fn from_env() -> Result<Self, BearDogError> {
        let mut config = Self::default();

        // Load from environment variables with BEARDOG_ADAPTER_ prefix
        if let Ok(adapter_id) = std::env::var("BEARDOG_ADAPTER_ID") {
            config.core.adapter_id = Arc::from(adapter_id.as_str());
        }

        if let Ok(max_conn) = std::env::var("BEARDOG_ADAPTER_MAX_CONNECTIONS") {
            config.core.max_connections = max_conn
                .parse()
                .map_err(|_| BearDogError::configuration("Invalid max_connections value"))?;
        }

        if let Ok(timeout) = std::env::var("BEARDOG_ADAPTER_TIMEOUT_SECS") {
            let secs: u64 = timeout
                .parse()
                .map_err(|_| BearDogError::configuration("Invalid timeout value"))?;
            config.core.connection_timeout = Duration::from_secs(secs);
        }

        if let Ok(endpoints) = std::env::var("BEARDOG_ADAPTER_DISCOVERY_ENDPOINTS") {
            config.discovery.endpoints =
                endpoints.split(',').map(|s| s.trim().to_string()).collect();
        }

        config.validate()?;
        Ok(config)
    }

    fn to_toml(&self) -> Result<String, BearDogError> {
        #[cfg(feature = "config")]
        return toml::to_string_pretty(self).map_err(|e| {
            BearDogError::configuration(&format!("Failed to serialize to TOML: {e}"))
        });

        #[cfg(not(feature = "config"))]
        Err(BearDogError::configuration(
            "TOML support not enabled - enable 'config' feature",
        ))
    }

    fn domain() -> &'static str {
        "adapter"
    }

    fn apply_environment_overrides(&mut self, environment: &str) -> Result<(), BearDogError> {
        match environment {
            "development" => {
                self.security.auth_required = false;
                self.security.encryption_in_transit = false;
                self.monitoring.enabled = false;
                self.optimization.level = 1;
            }
            "production" => {
                self.security.auth_level = AuthLevel::MultiFactor;
                self.security.encryption_at_rest = true;
                self.optimization.level = 5;
                self.service_mesh.enabled = true;
                self.monitoring.performance_metrics = true;
            }
            "staging" => {
                self.security.auth_level = AuthLevel::Token;
                self.optimization.level = 3;
                self.monitoring.enabled = true;
            }
            _ => {
                // Unknown environment, use defaults
            }
        }
        Ok(())
    }
}

impl UnifiedAdapterConfig {
    /// Create a new adapter configuration with defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Create adapter configuration for development environment
    pub fn development() -> Result<Self, BearDogError> {
        let mut config = Self::default();
        config.apply_environment_overrides("development")?;
        Ok(config)
    }

    /// Create adapter configuration for production environment
    pub fn production() -> Result<Self, BearDogError> {
        let mut config = Self::default();
        config.apply_environment_overrides("production")?;
        Ok(config)
    }
}

/// Migration utilities for legacy adapter configurations
pub mod migration {
    use super::{DiscoveryConfig, Duration, OptimizationConfig, UnifiedAdapterConfig};
    use std::sync::Arc;

    /// Migrate from legacy `AdapterConfig` to `UnifiedAdapterConfig`
    pub fn migrate_legacy_adapter_config(
        adapter_id: &str,
        max_connections: usize,
        timeout_secs: u64,
    ) -> UnifiedAdapterConfig {
        let mut config = UnifiedAdapterConfig::default();
        config.core.adapter_id = Arc::from(adapter_id);
        config.core.max_connections = max_connections;
        config.core.connection_timeout = Duration::from_secs(timeout_secs);
        config
    }

    /// Migrate from legacy `DiscoveryConfig`
    pub fn migrate_legacy_discovery_config(
        endpoints: Vec<String>,
        timeout_ms: u64,
        cache_enabled: bool,
    ) -> DiscoveryConfig {
        DiscoveryConfig {
            enabled: true,
            timeout: Duration::from_millis(timeout_ms),
            max_attempts: 3,
            max_concurrent: 10,
            discovery_interval: Duration::from_secs(60),
            refresh_interval: Duration::from_secs(60),
            cache_enabled,
            cache_ttl: Duration::from_secs(300),
            endpoints,
            health_check_interval: Duration::from_secs(60),
            auto_register: true,
            service_metadata: std::collections::HashMap::new(),
            predictive_enabled: false,
        }
    }

    /// Migrate from legacy `OptimizationConfig`
    pub fn migrate_legacy_optimization_config(
        enabled: bool,
        level: u8,
        simd_enabled: bool,
    ) -> OptimizationConfig {
        OptimizationConfig {
            enabled,
            level,
            simd_enabled,
            ..Default::default()
        }
    }
}
