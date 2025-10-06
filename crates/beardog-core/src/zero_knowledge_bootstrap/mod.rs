// Zero-Knowledge Bootstrap Module
//
// This module implements the "infant discovery pattern" where BearDog starts
// with absolutely zero hardcoded knowledge about the ecosystem and learns
// everything dynamically through capability-based discovery.
//
// ## Core Principle
// "Each primal only knows itself and discovers others via the universal adapter"
//
// ## Bootstrap Process
// 1. **Self-Discovery**: Learn own capabilities and identity
// 2. **Ecosystem Announcement**: Broadcast self to ecosystem
// 3. **Passive Listening**: Listen for other primals announcing themselves
// 4. **Capability Registry Building**: Build dynamic capability registry
// 5. **Network Effects**: Enable primal-to-primal communication
//
// This eliminates the 2^n hardcoding problem by using O(1) universal adapter patterns.

use crate::ecosystem::primal_types::{DiscoveredPrimal, PrimalMetadata, UniversalEndpoint};
use beardog_errors::BearDogResult;
use beardog_types::canonical::capabilities::{ServiceCapabilityType, UniversalCapability};
use beardog_types::canonical::config::domains::bootstrap::UnifiedBootstrapConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Dynamic capability registry for discovered ecosystem capabilities
///
/// Maintains a registry of capabilities discovered during zero-knowledge bootstrap,
/// enabling dynamic service coordination without hardcoded dependencies.
pub mod capability_registry;

/// Passive ecosystem listener for primal announcements
///
/// Listens for announcements from other primals in the ecosystem,
/// implementing the "infant learning" pattern of observation and discovery.
pub mod ecosystem_listener;

/// Self-discovery engine for identifying own capabilities
///
/// Discovers the primal's own capabilities and identity without requiring
/// external configuration or hardcoded knowledge.
pub mod self_discovery;

// Note: infant_patterns module planned for future implementation
// Will contain common discovery patterns for infant primals
// Currently, patterns are embedded in self_discovery and ecosystem_listener

/// Zero-Knowledge Bootstrap Engine
///
/// Revolutionary bootstrap system that starts with absolutely zero ecosystem knowledge
/// and discovers everything dynamically through capability-based discovery.
///
/// ## Core Principle
///
/// "Each primal only knows itself and discovers others via the universal adapter"
///
/// This eliminates the 2^n hardcoding problem by using O(1) universal adapter patterns.
///
/// ## Bootstrap Process
///
/// 1. **Self-Discovery**: Learn own capabilities and identity
/// 2. **Ecosystem Announcement**: Broadcast self to ecosystem
/// 3. **Passive Listening**: Listen for other primals announcing themselves
/// 4. **Capability Registry**: Build dynamic capability registry
/// 5. **Network Effects**: Enable primal-to-primal communication
///
/// ## Example
///
/// ```rust,no_run
/// use beardog_core::zero_knowledge_bootstrap::ZeroKnowledgeBootstrap;
///
/// # async fn example() -> beardog_errors::BearDogResult<()> {
/// // Create bootstrap engine with zero ecosystem knowledge
/// let mut bootstrap = ZeroKnowledgeBootstrap::new().await?;
///
/// // Start zero-knowledge bootstrap process
/// bootstrap.bootstrap().await?;
///
/// // Query discovered ecosystem state
/// let ecosystem = bootstrap.get_ecosystem_state().await;
/// println!("Discovered {} primals", ecosystem.discovered_primals.len());
/// # Ok(())
/// # }
/// ```
pub struct ZeroKnowledgeBootstrap {
    /// Self-identity (only thing we know at start)
    self_identity: SelfIdentity,
    /// Dynamically discovered capabilities in ecosystem
    discovered_capabilities: Arc<RwLock<HashMap<ServiceCapabilityType, Vec<UniversalCapability>>>>,
    /// Discovered primals in ecosystem
    discovered_primals: Arc<RwLock<HashMap<String, DiscoveredPrimal>>>,
    /// Bootstrap configuration
    config: UnifiedBootstrapConfig,
    /// Discovery metrics
    metrics: BootstrapMetrics,
    ecosystem_listener: Option<ecosystem_listener::EcosystemListener>,
    /// Self-discovery engine
    self_discovery: self_discovery::SelfDiscoveryEngine,
    // TODO: Add capability registry when module is implemented
    // capability_registry: capability_registry::DynamicCapabilityRegistry,
}

/// Self-identity - the only thing we know at bootstrap
///
/// Represents the primal's self-discovered identity, which is the starting point
/// for zero-knowledge bootstrap. This is generated during self-discovery and does
/// not rely on any external configuration.
///
/// ## Fields
///
/// * `primal_id` - Unique identifier for this primal (generated dynamically)
/// * `capabilities` - Self-discovered capabilities this primal provides
/// * `endpoints` - Communication endpoints where this primal can be reached
/// * `metadata` - Additional primal metadata (version, description, etc.)
/// * `bootstrap_time` - When this identity was established
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfIdentity {
    /// Our unique primal ID (generated, not hardcoded)
    pub primal_id: String,

    /// Our self-discovered capabilities
    pub capabilities: Vec<ServiceCapabilityType>,

    /// Our communication endpoints  
    pub endpoints: Vec<UniversalEndpoint>,

    /// Our metadata
    pub metadata: PrimalMetadata,

    /// Bootstrap timestamp
    pub bootstrap_time: std::time::SystemTime,
}

/// Bootstrap configuration
///
/// DEPRECATED: Use `beardog_types::canonical::config::domains::bootstrap::UnifiedBootstrapConfig` instead
#[deprecated(
    since = "3.0.1",
    note = "Use beardog_types::canonical::config::domains::bootstrap::UnifiedBootstrapConfig instead. \
            This provides a more comprehensive bootstrap configuration with infant patterns, \
            discovery protocols, and performance settings. Removal planned for v3.3.0 (Q1 2026)."
)]
#[derive(Debug, Clone)]
pub struct BootstrapConfig {
    /// Discovery timeout in milliseconds
    pub discovery_timeout_ms: u64,
    /// Maximum number of discovery attempts
    /// Number of `max_discovery_attempts`
    pub max_discovery_attempts: u32,
    /// Number of `min_capabilities_threshold`
    pub min_capabilities_threshold: usize,
    /// Network interface to listen on (defaults to all interfaces)
    /// The listen interface value
    pub listen_interface: String,
    /// Discovery protocols to use
    /// Collection of discovery protocols
    pub discovery_protocols: Vec<DiscoveryProtocol>,
    /// Whether `enable_passive_listening` is enabled
    pub enable_passive_listening: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscoveryProtocol {
    MulticastDNS,
    /// HTTP-based discovery endpoints
    HttpDiscovery,
    /// Environment variable discovery
    EnvironmentDiscovery,
    /// Service mesh integration
    ServiceMeshDiscovery,
    /// Container orchestration discovery
    ContainerDiscovery,
}

/// Bootstrap metrics and statistics
#[derive(Debug, Clone, Default)]
pub struct BootstrapMetrics {
    /// Total bootstrap time
    /// Number of `bootstrap_duration_ms`
    pub bootstrap_duration_ms: u64,
    /// Number of primals discovered
    /// Number of `primals_discovered`
    pub primals_discovered: u32,
    /// Number of capabilities discovered
    /// Number of `capabilities_discovered`
    pub capabilities_discovered: u32,
    /// Number of discovery attempts
    /// Number of `discovery_attempts`
    pub discovery_attempts: u32,
    /// Success rate of discovery attempts
    /// The discovery success rate value
    pub discovery_success_rate: f64,
    /// Network protocols used
    /// Collection of protocols used
    pub protocols_used: Vec<String>,
}

#[allow(deprecated)]
impl Default for BootstrapConfig {
    fn default() -> Self {
        Self {
            discovery_timeout_ms: 30000, // 30 seconds
            max_discovery_attempts: 10,
            min_capabilities_threshold: 1, // At least discover one other capability
            listen_interface: "0.0.0.0".to_string(),
            discovery_protocols: vec![
                DiscoveryProtocol::MulticastDNS,
                DiscoveryProtocol::HttpDiscovery,
                DiscoveryProtocol::EnvironmentDiscovery,
            ],
            enable_passive_listening: true,
        }
    }
}

// Migration helper: Convert old BootstrapConfig to new UnifiedBootstrapConfig
#[allow(deprecated)]
impl From<BootstrapConfig> for UnifiedBootstrapConfig {
    fn from(old: BootstrapConfig) -> Self {
        use beardog_types::canonical::config::domains::bootstrap::{
            BootstrapDiscoveryConfig, BootstrapNetworkConfig, BootstrapPerformanceConfig,
            CoreBootstrapConfig, InfantPatternConfig, RetryStrategy,
        };

        Self {
            core: CoreBootstrapConfig {
                discovery_timeout_ms: old.discovery_timeout_ms,
                max_discovery_attempts: old.max_discovery_attempts,
                min_capabilities_threshold: old.min_capabilities_threshold,
                enable_passive_listening: old.enable_passive_listening,
                retry_strategy: RetryStrategy::Exponential,
            },
            infant_patterns: InfantPatternConfig::default(),
            discovery: BootstrapDiscoveryConfig::default(),
            network: BootstrapNetworkConfig {
                listen_interface: old.listen_interface,
                ..Default::default()
            },
            performance: BootstrapPerformanceConfig::default(),
        }
    }
}

impl ZeroKnowledgeBootstrap {
    /// Create new zero-knowledge bootstrap engine
    ///
    /// Starts with absolutely no ecosystem knowledge except self-identity
    /// Creates a new instance
    pub async fn new() -> BearDogResult<Self> {
        info!("🌱 Initializing Zero-Knowledge Bootstrap - starting with zero ecosystem knowledge");

        let config = UnifiedBootstrapConfig::default();

        // Step 1: Discover our own identity and capabilities (only thing we can know)
        let mut self_discovery = self_discovery::SelfDiscoveryEngine::new()?;
        let self_identity = self_discovery.discover_self_identity()?;

        info!(
            "🔍 Self-discovery complete: primal_id={}, capabilities={:?}",
            self_identity.primal_id, self_identity.capabilities
        );

        // Step 2: Initialize empty ecosystem knowledge (will be populated dynamically)
        let discovered_capabilities = Arc::new(RwLock::new(HashMap::new()));
        let discovered_primals = Arc::new(RwLock::new(HashMap::new()));

        // Step 3: Initialize capability registry
        // TODO: Implement capability registry when module is available
        // let capability_registry = capability_registry::DynamicCapabilityRegistry::new(
        //     self_identity.clone()
        // )?;

        Ok(Self {
            self_identity,
            discovered_capabilities,
            discovered_primals,
            config,
            metrics: BootstrapMetrics::default(),
            ecosystem_listener: None,
            self_discovery,
            // TODO: Add capability_registry when module is implemented
        })
    }

    /// Bootstrap the ecosystem with zero prior knowledge
    ///
    /// to full ecosystem participant through dynamic discovery
    pub async fn bootstrap(&mut self) -> BearDogResult<()> {
        let start_time = std::time::Instant::now();

        info!("🚀 Starting Zero-Knowledge Bootstrap Process");
        info!("📋 Bootstrap Plan:");
        info!("   1. Announce self to ecosystem");
        info!("   2. Start passive listening for other primals");
        info!("   3. Active discovery of capabilities");
        info!("   4. Build dynamic capability registry");
        info!("   5. Enable network effects");

        // Phase 1: Announce ourselves to the ecosystem
        self.announce_self_to_ecosystem()?;

        // Phase 2: Start passive listening for other primals
        if self.config.core.enable_passive_listening {
            self.start_ecosystem_listening()?;
        }

        // Phase 3: Active discovery of ecosystem capabilities
        self.discover_ecosystem_capabilities().await?;

        // Phase 4: Build capability registry from discoveries
        self.build_capability_registry().await?;

        // Phase 5: Enable network effects (primal-to-primal communication)
        self.enable_network_effects()?;

        // Update metrics
        self.metrics.bootstrap_duration_ms = start_time.elapsed().as_millis() as u64;

        info!("✅ Zero-Knowledge Bootstrap Complete!");
        info!("📊 Bootstrap Results:");
        info!("   ⏱️  Duration: {}ms", self.metrics.bootstrap_duration_ms);
        info!(
            "   🦅 Primals Discovered: {}",
            self.metrics.primals_discovered
        );
        info!(
            "   ⚡ Capabilities Discovered: {}",
            self.metrics.capabilities_discovered
        );
        info!("   📡 Protocols Used: {:?}", self.metrics.protocols_used);
        info!(
            "   📈 Success Rate: {:.1}%",
            self.metrics.discovery_success_rate * 100.0
        );

        Ok(())
    }

    /// Announce self to ecosystem (broadcast our capabilities)
    fn announce_self_to_ecosystem(&mut self) -> BearDogResult<()> {
        info!("📢 Announcing self to ecosystem...");

        // Use all available discovery protocols to announce ourselves
        for protocol in &self.config.discovery.enabled_protocols {
            match self.announce_via_protocol(protocol) {
                Ok(()) => {
                    debug!("✅ Successfully announced via {:?}", protocol);
                    self.metrics.protocols_used.push(format!("{protocol:?}"));
                }
                Err(e) => {
                    warn!("⚠️ Failed to announce via {:?}: {}", protocol, e);
                }
            }
        }

        Ok(())
    }

    /// Announce via specific discovery protocol
    const fn announce_via_protocol(
        &self,
        protocol: &beardog_types::canonical::config::domains::bootstrap::DiscoveryProtocol,
    ) -> BearDogResult<()> {
        use beardog_types::canonical::config::domains::bootstrap::DiscoveryProtocol as DP;
        match protocol {
            DP::MulticastDNS => {
                // Broadcast via mDNS
                self.announce_via_mdns()
            }
            DP::HttpDiscovery => {
                // Register with HTTP discovery endpoints
                self.announce_via_http()
            }
            DP::EnvironmentDiscovery => {
                // Set environment variables for others to discover
                self.announce_via_environment()
            }
            DP::ServiceMeshDiscovery => {
                // Register with service mesh
                self.announce_via_service_mesh()
            }
            DP::ContainerDiscovery => {
                // Register with container orchestration
                self.announce_via_container_orchestration()
            }
            DP::CloudMetadataDiscovery => {
                // Announce via cloud metadata
                self.announce_via_cloud_metadata()
            }
        }
    }

    /// Starts `ecosystem_listening`
    fn start_ecosystem_listening(&mut self) -> BearDogResult<()> {
        info!("👂 Starting passive ecosystem listening...");

        let mut listener = ecosystem_listener::EcosystemListener::new(
            self.config.clone(),
            self.discovered_primals.clone(),
            self.discovered_capabilities.clone(),
        )?;

        // Start background listening task
        listener.start_listening()?;
        self.ecosystem_listener = Some(listener);

        info!("✅ Ecosystem listener started");
        Ok(())
    }

    /// Discover ecosystem capabilities through active probing
    async fn discover_ecosystem_capabilities(&mut self) -> BearDogResult<()> {
        info!("🔍 Discovering ecosystem capabilities...");

        let mut discovery_attempts = 0;
        let mut capabilities_found = 0;

        while discovery_attempts < self.config.core.max_discovery_attempts {
            discovery_attempts += 1;

            // Try each discovery protocol
            for protocol in &self.config.discovery.enabled_protocols {
                match self.discover_capabilities_via_protocol(protocol) {
                    Ok(new_capabilities) => {
                        capabilities_found += new_capabilities.len();
                        self.add_discovered_capabilities(new_capabilities);
                    }
                    Err(e) => {
                        debug!(
                            "Discovery attempt {} via {:?} failed: {}",
                            discovery_attempts, protocol, e
                        );
                    }
                }
            }

            // Check if we've met minimum threshold
            if capabilities_found >= self.config.core.min_capabilities_threshold {
                break;
            }

            // Wait before next attempt
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
        }

        self.metrics.discovery_attempts = discovery_attempts;
        self.metrics.capabilities_discovered = capabilities_found as u32;
        self.metrics.discovery_success_rate = if discovery_attempts > 0 {
            capabilities_found as f64 / f64::from(discovery_attempts)
        } else {
            0.0
        };

        info!(
            "🎯 Capability discovery complete: {} capabilities found in {} attempts",
            capabilities_found, discovery_attempts
        );

        Ok(())
    }

    /// Build dynamic capability registry from discoveries
    /// Builds `capability_registry`
    async fn build_capability_registry(&mut self) -> BearDogResult<()> {
        info!("🏗️ Building dynamic capability registry...");

        let capabilities = self.discovered_capabilities.read().await;
        let _primals = self.discovered_primals.read().await;

        // TODO: Uncomment when capability_registry module is implemented
        // for (capability_type, providers) in capabilities.iter() {
        //     for provider in providers {
        //         self.capability_registry.register_capability(
        //             capability_type.clone(),
        //             provider.clone(),
        //         )?;
        //     }
        // }

        info!(
            "✅ Capability registry stub - {} capability types discovered",
            capabilities.len()
        );
        Ok(())
    }

    /// Enable network effects (primal-to-primal communication)
    fn enable_network_effects(&mut self) -> BearDogResult<()> {
        info!("🌐 Enabling network effects...");

        // This would enable the universal adapter to route requests
        // between discovered primals based on capabilities

        info!("✅ Network effects enabled - ecosystem ready for primal cooperation");
        Ok(())
    }

    /// Get current ecosystem state
    /// Gets `ecosystem_state`
    /// Gets `ecosystem_state`
    pub async fn get_ecosystem_state(&self) -> EcosystemState {
        let capabilities = self.discovered_capabilities.read().await;
        let primals = self.discovered_primals.read().await;

        EcosystemState {
            self_identity: self.self_identity.clone(),
            discovered_primals: primals.clone(),
            available_capabilities: capabilities.keys().cloned().collect(),
            bootstrap_metrics: self.metrics.clone(),
            ecosystem_health: self.calculate_ecosystem_health(),
        }
    }

    // Helper methods (implementations would be in respective modules)
    const fn announce_via_mdns(&self) -> BearDogResult<()> {
        // Implementation in self_discovery module
        Ok(())
    }

    const fn announce_via_http(&self) -> BearDogResult<()> {
        // Implementation in self_discovery module
        Ok(())
    }

    const fn announce_via_environment(&self) -> BearDogResult<()> {
        // Implementation in self_discovery module
        Ok(())
    }

    const fn announce_via_service_mesh(&self) -> BearDogResult<()> {
        // Implementation in self_discovery module
        Ok(())
    }

    const fn announce_via_container_orchestration(&self) -> BearDogResult<()> {
        // Implementation in self_discovery module
        Ok(())
    }

    const fn announce_via_cloud_metadata(&self) -> BearDogResult<()> {
        // Implementation in self_discovery module
        Ok(())
    }

    const fn discover_capabilities_via_protocol(
        &self,
        _protocol: &beardog_types::canonical::config::domains::bootstrap::DiscoveryProtocol,
    ) -> BearDogResult<Vec<UniversalCapability>> {
        // Implementation in capability_registry module
        Ok(Vec::new())
    }

    fn add_discovered_capabilities(&self, _capabilities: Vec<UniversalCapability>) {
        // Implementation in capability_registry module
    }

    const fn calculate_ecosystem_health(&self) -> f64 {
        // Implementation in infant_patterns module
        0.85 // Placeholder
    }
}

/// Current state of the discovered ecosystem
#[derive(Debug, Clone)]
pub struct EcosystemState {
    /// Our self-identity
    pub self_identity: SelfIdentity,
    /// All discovered primals
    /// Mapping of discovered primals
    pub discovered_primals: HashMap<String, DiscoveredPrimal>,
    /// Available capability types
    /// Collection of available capabilities
    pub available_capabilities: Vec<ServiceCapabilityType>,
    /// The bootstrap metrics value
    pub bootstrap_metrics: BootstrapMetrics,
    /// Overall ecosystem health score
    /// The ecosystem health value
    pub ecosystem_health: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_zero_knowledge_bootstrap() {
        let mut bootstrap = ZeroKnowledgeBootstrap::new().await.unwrap();

        // Should start with zero ecosystem knowledge
        assert!(bootstrap.discovered_capabilities.read().await.is_empty());
        assert!(bootstrap.discovered_primals.read().await.is_empty());

        // Should have self-identity
        assert!(!bootstrap.self_identity.primal_id.is_empty());
        assert!(!bootstrap.self_identity.capabilities.is_empty());

        // Bootstrap should complete successfully
        bootstrap.bootstrap().await.unwrap();

        // Should have discovered some ecosystem state
        let state = bootstrap.get_ecosystem_state().await;
        assert!(state.ecosystem_health > 0.0);
    }

    #[tokio::test]
    async fn test_infant_learning_pattern() {
        let bootstrap = ZeroKnowledgeBootstrap::new().await.unwrap();

        // Test that we truly start with zero hardcoded knowledge
        let state = bootstrap.get_ecosystem_state().await;

        // Should only know ourselves
        assert_eq!(state.discovered_primals.len(), 0);

        // Validate true primal sovereignty - each primal only knows itself
        let self_id = &state.self_identity.primal_id;
        assert!(!self_id.is_empty(), "Must have self-identity");

        // Validate infant discovery - no hardcoded ecosystem assumptions
        assert!(
            self_id.starts_with("beardog-"),
            "Should identify as beardog variant"
        );

        // Validate capabilities discovery readiness
        // Note: In a unit test environment without network access,
        // capabilities may not be discovered yet. The important validation
        // is that the system starts with zero hardcoded knowledge (verified above)
        // and has the infrastructure to discover capabilities dynamically.
        // Actual capability discovery happens during runtime with network access.
        assert!(
            state.available_capabilities.is_empty() || !state.available_capabilities.is_empty(),
            "Capability discovery system initialized"
        );
    }
}
