// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builder pattern for `UnifiedDiscoveryConfig`
//!
//! This module provides a flexible builder for constructing `UnifiedDiscoveryConfig`
//! with support for environment variable overrides and custom configuration.

use super::{
    DiscoveryCacheConfig, DiscoveryProtocol, DiscoverySecurityConfig, LoadBalancingConfig,
    NetworkDiscoveryConfig, QuantumDiscoveryConfig, ServiceRegistryConfig, UnifiedDiscoveryConfig,
};
use std::sync::Arc;

/// Builder for flexible [`UnifiedDiscoveryConfig`] construction
#[derive(Debug, Default)]
pub struct UnifiedDiscoveryConfigBuilder {
    /// Overrides [`UnifiedDiscoveryConfig::enabled`] when set.
    enabled: Option<bool>,
    /// Overrides [`UnifiedDiscoveryConfig::service_id`] when set (converted to `Arc<str>` on build).
    service_id: Option<String>,
    /// Replaces default [`UnifiedDiscoveryConfig::enabled_protocols`] when non-empty.
    enabled_protocols: Vec<DiscoveryProtocol>,
    /// Overrides the registry subsection when set.
    registry: Option<ServiceRegistryConfig>,
    /// Overrides the network discovery subsection when set.
    network: Option<NetworkDiscoveryConfig>,
    /// Overrides quantum (experimental) discovery when set.
    quantum: Option<QuantumDiscoveryConfig>,
    /// Overrides discovery result cache settings when set.
    cache: Option<DiscoveryCacheConfig>,
    /// Overrides discovery TLS/auth policy when set.
    security: Option<DiscoverySecurityConfig>,
    /// Overrides client-side load balancing for resolved instances when set.
    load_balancing: Option<LoadBalancingConfig>,
}

impl UnifiedDiscoveryConfigBuilder {
    /// Creates an empty builder; call `build()` to merge with [`UnifiedDiscoveryConfig::default`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets whether discovery subsystems are active for this primal.
    pub const fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = Some(enabled);
        self
    }

    /// Sets the logical service id announced to peers and registries.
    pub fn service_id(mut self, service_id: impl Into<String>) -> Self {
        self.service_id = Some(service_id.into());
        self
    }

    /// Appends a protocol-specific discovery configuration (HTTP, DNS, mDNS, etc.).
    pub fn add_protocol(mut self, protocol: DiscoveryProtocol) -> Self {
        self.enabled_protocols.push(protocol);
        self
    }

    /// Sets centralized registry (Consul, etcd, …) connection parameters.
    pub fn registry(mut self, registry: ServiceRegistryConfig) -> Self {
        self.registry = Some(registry);
        self
    }

    /// Sets passive/active network scan and retry behavior.
    pub fn network(mut self, network: NetworkDiscoveryConfig) -> Self {
        self.network = Some(network);
        self
    }

    /// Sets experimental quantum-assisted discovery parameters.
    pub fn quantum(mut self, quantum: QuantumDiscoveryConfig) -> Self {
        self.quantum = Some(quantum);
        self
    }

    /// Sets TTL, size, and eviction for the discovery cache.
    pub fn cache(mut self, cache: DiscoveryCacheConfig) -> Self {
        self.cache = Some(cache);
        self
    }

    /// Sets authentication and TLS requirements for discovery traffic.
    pub fn security(mut self, security: DiscoverySecurityConfig) -> Self {
        self.security = Some(security);
        self
    }

    /// Sets how traffic is spread across discovered healthy endpoints.
    pub fn load_balancing(mut self, load_balancing: LoadBalancingConfig) -> Self {
        self.load_balancing = Some(load_balancing);
        self
    }

    /// Load values from environment variables
    pub const fn from_env(self) -> Self {
        // Builder-based environment loading would go here
        // For now, users should use UnifiedDiscoveryConfig::from_env() directly
        self
    }

    /// Consumes the builder, applying each `Some` field over [`UnifiedDiscoveryConfig::default`].
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

#[cfg(test)]
mod tests {
    use super::UnifiedDiscoveryConfigBuilder;
    use crate::canonical::config::domains::discovery_unified::{
        DiscoveryCacheConfig, DiscoveryProtocol, DiscoverySecurityConfig, LoadBalancingConfig,
        NetworkDiscoveryConfig, QuantumDiscoveryConfig, ServiceRegistryConfig,
        UnifiedDiscoveryConfig,
    };
    use std::sync::Arc;

    #[test]
    fn builder_new_and_from_env_are_noops_on_fields() {
        let b = UnifiedDiscoveryConfigBuilder::new().from_env();
        let c = b.build();
        let d = UnifiedDiscoveryConfig::default();
        assert_eq!(c.enabled, d.enabled);
    }

    #[test]
    fn builder_overrides_all_sections() {
        let reg = ServiceRegistryConfig::default();
        let net = NetworkDiscoveryConfig::default();
        let q = QuantumDiscoveryConfig::default();
        let cache = DiscoveryCacheConfig::default();
        let sec = DiscoverySecurityConfig::default();
        let lb = LoadBalancingConfig::default();

        let c = UnifiedDiscoveryConfig::builder()
            .enabled(false)
            .service_id("svc-x")
            .add_protocol(DiscoveryProtocol::Http {
                endpoint: "http://h".into(),
                timeout_ms: 100,
            })
            .registry(reg.clone())
            .network(net.clone())
            .quantum(q.clone())
            .cache(cache.clone())
            .security(sec.clone())
            .load_balancing(lb.clone())
            .build();

        assert!(!c.enabled);
        assert_eq!(c.service_id.as_ref(), "svc-x");
        assert_eq!(c.enabled_protocols.len(), 1);
        assert_eq!(c.registry, reg);
        assert_eq!(c.network, net);
        assert_eq!(c.quantum, q);
        assert_eq!(c.cache, cache);
        assert_eq!(c.security, sec);
        assert_eq!(c.load_balancing, lb);
    }

    #[test]
    fn builder_empty_protocols_use_defaults() {
        let c = UnifiedDiscoveryConfigBuilder::new()
            .service_id("only-id")
            .build();
        let d = UnifiedDiscoveryConfig::default();
        assert_eq!(c.enabled_protocols, d.enabled_protocols);
        assert_eq!(c.service_id, Arc::from("only-id"));
    }
}
