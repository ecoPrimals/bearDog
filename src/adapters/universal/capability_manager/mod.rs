//! Comprehensive Capability Management System
//!
//! **Advanced capability monitoring, merging, and discovery**
//!
//! This system enhances the basic capability advertisement with:
//! - Real-time capability monitoring and updates
//! - Genetic spawning capability merging
//! - Emergent capability discovery
//! - Advanced capability matching algorithms
//! - Dynamic dependency resolution

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::interval;
use tracing::{debug, info, warn};

// Re-export all public types and traits
pub use self::config::*;
pub use self::dependency::*;
pub use self::emergent::*;
pub use self::genetic::*;
pub use self::matching::*;
pub use self::monitoring::*;

use super::registry::CapabilityRegistry;
use super::traits::*;
use crate::BearDogResult;

// Module declarations
pub mod config;
pub mod dependency;
pub mod emergent;
pub mod genetic;
pub mod matching;
pub mod monitoring;

/// Comprehensive Capability Manager
///
/// Extends the basic capability registry with advanced features for
/// ecosystem-wide capability orchestration and genetic spawning integration.
pub struct CapabilityManager {
    /// Basic capability registry
    registry: Arc<CapabilityRegistry>,

    /// Real-time capability monitoring
    capability_monitors: Arc<RwLock<HashMap<String, CapabilityMonitor>>>,

    /// Genetic spawning capability tracker
    genetic_capabilities: Arc<RwLock<HashMap<String, GeneticCapabilityProfile>>>,

    /// Emergent capability discovery engine
    discovery_engine: Arc<EmergentCapabilityEngine>,

    /// Advanced capability matcher
    matcher: Arc<AdvancedCapabilityMatcher>,

    /// Dependency resolver
    dependency_resolver: Arc<DependencyResolver>,

    /// Manager configuration
    config: CapabilityManagerConfig,
}

impl CapabilityManager {
    /// Create a new comprehensive capability manager
    pub async fn new(
        registry: Arc<CapabilityRegistry>,
        config: CapabilityManagerConfig,
    ) -> BearDogResult<Self> {
        info!("🚀 Initializing Comprehensive Capability Manager");

        let manager = Self {
            registry,
            capability_monitors: Arc::new(RwLock::new(HashMap::new())),
            genetic_capabilities: Arc::new(RwLock::new(HashMap::new())),
            discovery_engine: Arc::new(EmergentCapabilityEngine::new().await?),
            matcher: Arc::new(AdvancedCapabilityMatcher::new().await?),
            dependency_resolver: Arc::new(DependencyResolver::new().await?),
            config,
        };

        // Start monitoring task
        manager.start_monitoring_task().await?;

        info!("✅ Comprehensive Capability Manager initialized successfully");
        Ok(manager)
    }

    /// Start background monitoring task
    async fn start_monitoring_task(&self) -> BearDogResult<()> {
        let monitors = Arc::clone(&self.capability_monitors);
        let registry = Arc::clone(&self.registry);
        let config = self.config.clone();

        tokio::spawn(async move {
            let mut interval = interval(config.monitoring_interval);
            loop {
                interval.tick().await;
                if let Err(e) = Self::run_monitoring_cycle(&monitors, &registry, &config).await {
                    warn!("⚠️  Monitoring cycle failed: {}", e);
                }
            }
        });

        Ok(())
    }

    /// Run a single monitoring cycle
    async fn run_monitoring_cycle(
        _monitors: &Arc<RwLock<HashMap<String, CapabilityMonitor>>>,
        _registry: &Arc<CapabilityRegistry>,
        _config: &CapabilityManagerConfig,
    ) -> BearDogResult<()> {
        // TODO: Implement full monitoring cycle
        // For now, just log that we're running
        debug!("🔄 Running capability monitoring cycle");
        Ok(())
    }

    /// Get comprehensive capability monitoring status
    pub async fn get_monitoring_status(&self) -> BearDogResult<HashMap<String, CapabilityMonitor>> {
        Ok(self.capability_monitors.read().await.clone())
    }

    /// Get genetic capability tracking data
    pub async fn get_genetic_capabilities(
        &self,
    ) -> BearDogResult<HashMap<String, GeneticCapabilityProfile>> {
        Ok(self.genetic_capabilities.read().await.clone())
    }

    /// Get emergent capabilities
    pub async fn get_emergent_capabilities(
        &self,
    ) -> BearDogResult<HashMap<String, EmergentCapability>> {
        Ok(self
            .discovery_engine
            .emergent_capabilities
            .read()
            .await
            .clone())
    }
} 