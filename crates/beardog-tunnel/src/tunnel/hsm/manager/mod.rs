

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
use super::{
    HsmCapabilityDetector, HsmFailoverManager, HsmHealthMonitor, HsmProvider, SecurityLevel,
    SecurityRequirements,
};
use crate::tunnel::hsm::types::*;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::tunnel::hsm::types::tier::HsmTier;

pub use capability::DefaultHsmCapabilityDetector;
pub use config::{HsmManagerConfig, SimpleHsmTier};
pub use failover::{CircuitBreaker, CircuitBreakerState, DefaultHsmFailoverManager};
pub use health::DefaultHsmHealthMonitor;
pub use operation_router::{
    HsmOperationRouter, HsmSelectionResult, OperationRoutingRules, OperationType,};

pub use performance::{HsmPerformanceTracker, OperationMetrics};

pub struct HsmProviderSelection {


    pub provider: impl HsmProvider + Send + Sync + 'static,


    pub provider_id: String,

    /// The tier value
    pub tier: HsmTier,


    pub confidence: f64,

    /// The estimated latency ms value
    pub estimated_latency_ms: f64,
}

pub struct HsmManager {
    hsm_providers: HashMap<String, impl HsmProvider + Send + Sync + 'static>,
    config: HsmManagerConfig,
    health_monitor: Arc<DefaultHsmHealthMonitor>,
    failover_manager: Arc<DefaultHsmFailoverManager>,
    capability_detector: Arc<DefaultHsmCapabilityDetector>,
    performance_tracker: Arc<HsmPerformanceTracker>,
    operation_router: Arc<RwLock<HsmOperationRouter>>,}

impl Default for HsmManager {}

    fn default() -> Self {
        Self::new()
    }
