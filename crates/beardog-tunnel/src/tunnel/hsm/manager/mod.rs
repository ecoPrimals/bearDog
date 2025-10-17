// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod capability;
/// Configuration management
/// Configuration management
pub mod config;
pub mod failover;
pub mod health;
pub mod implementation;
pub mod operation_router;
pub mod performance;
use beardog_errors::BearDogError;
use beardog_types::hsm::{DefaultHsmFailoverManager, DefaultHsmHealthMonitor};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::tunnel::hsm::types::tier::HsmTier;

pub use capability::DefaultHsmCapabilityDetector;
pub use config::{HsmManagerConfig, SimpleHsmTier};
pub use failover::{CircuitBreaker, CircuitBreakerState};
// Note: DefaultHsmFailoverManager doesn't exist in failover module - commented out
// pub use failover::DefaultHsmFailoverManager;
// Note: DefaultHsmHealthMonitor doesn't exist in health module - commented out
// pub use health::DefaultHsmHealthMonitor;
pub use implementation::{DefaultHsmManager, HealthStatus, HsmProvider, KeyInfo, ProviderInfo};
pub use operation_router::{
    HsmOperationRouter, HsmSelectionResult, OperationRoutingRules, OperationType,
};

pub use performance::{HsmPerformanceTracker, OperationMetrics};

pub struct HsmProviderSelection {
    pub provider: Arc<dyn HsmProvider>,

    pub provider_id: String,

    /// The tier value
    pub tier: HsmTier,

    pub confidence: f64,

    /// The estimated latency ms value
    pub estimated_latency_ms: f64,
}

pub struct HsmManager {
    hsm_providers: HashMap<String, Arc<dyn HsmProvider>>,
    config: HsmManagerConfig,
    health_monitor: Arc<DefaultHsmHealthMonitor>,
    failover_manager: Arc<DefaultHsmFailoverManager>,
    capability_detector: Arc<DefaultHsmCapabilityDetector>,
    performance_tracker: Arc<HsmPerformanceTracker>,
    operation_router: Arc<RwLock<HsmOperationRouter>>,
}

impl Default for HsmManager {
    fn default() -> Self {
        Self::new()
    }
}

impl HsmManager {
    /// Create a new HSM manager
    pub fn new() -> Self {
        Self {
            hsm_providers: HashMap::new(),
            config: HsmManagerConfig::default(),
            health_monitor: Arc::new(DefaultHsmHealthMonitor::default()),
            failover_manager: Arc::new(DefaultHsmFailoverManager::default()),
            capability_detector: Arc::new(DefaultHsmCapabilityDetector::new().expect(
                "DefaultHsmCapabilityDetector::new() cannot fail - it only creates a HashMap",
            )),
            performance_tracker: Arc::new(HsmPerformanceTracker::default()),
            operation_router: Arc::new(RwLock::new(HsmOperationRouter::default())),
        }
    }

    /// Register an HSM provider
    ///
    /// # Errors
    /// Returns an error if registration fails
    pub fn register_hsm_provider(
        &mut self,
        tier: HsmTier,
        provider: Arc<dyn HsmProvider>,
    ) -> Result<(), BearDogError> {
        let tier_key = format!("{:?}", tier);
        self.hsm_providers.insert(tier_key, provider);
        Ok(())
    }

    /// Get routing metrics
    pub fn get_routing_metrics(&self) -> std::collections::HashMap<String, u64> {
        // Stub implementation
        std::collections::HashMap::new()
    }
}
