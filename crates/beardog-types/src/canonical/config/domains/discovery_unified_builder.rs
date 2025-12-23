//! Builder pattern for UnifiedDiscoveryConfig
//!
//! This module provides a flexible builder for constructing UnifiedDiscoveryConfig
//! with support for environment variable overrides and custom configuration.

use super::{
    DiscoveryCacheConfig, DiscoveryProtocol, DiscoverySecurityConfig, LoadBalancingConfig,
    NetworkDiscoveryConfig, QuantumDiscoveryConfig, ServiceRegistryConfig, UnifiedDiscoveryConfig,
};
use std::sync::Arc;

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
            service_id: self
                .service_id
                .map(|s| Arc::from(s.as_str()))
                .unwrap_or(defaults.service_id),
            enabled_protocols: if self.enabled_protocols.is_empty() {
                defaults.enabled_protocols
            } else {
                self.enabled_protocols
            },
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
