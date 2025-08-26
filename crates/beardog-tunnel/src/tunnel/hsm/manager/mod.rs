// PHASE 5 MODERNIZED: Comprehensive Arc<dyn> elimination
// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # HSM Manager
///
/// This module provides the central HSM management system that orchestrates between
/// different HSM providers (smartphone, software, hardware) with intelligent tier
/// selection, automatic failover, and performance optimization.
/// ## Architecture
/// ```text
/// ┌─────────────────────────────────────────────────────────────────┐
/// │                       HSM Manager                              │
/// │  ┌─────────────────┐  ┌─────────────────┐  ┌─────────────────┐ │
/// │  │ Tier Selection  │  │ Health Monitor  │  │ Failover Manager│ │
/// │  │ - Requirements  │  │ - Health Checks │  │ - Retry Logic   │ │
/// │  │ - Capabilities  │  │ - Metrics       │  │ - Fallback      │ │
/// │  │ - Performance   │  │ - Alerting      │  │ - Load Balance  │ │
/// │  └─────────────────┘  └─────────────────┘  └─────────────────┘ │
/// └─────────────────────┬───────────────────────────────────────────┘
///                       │
///       ┌───────────────┼───────────────┐
///       │               │               │
/// ┌─────▼─────┐  ┌─────▼─────┐  ┌─────▼─────┐
/// │Smartphone │  │ Software  │  │ Hardware  │
/// │    HSM    │  │    HSM    │  │    HSM    │
/// │  (T1)     │  │   (T2)    │  │   (T3)    │
/// └───────────┘  └───────────┘  └───────────┘
/// ```

pub mod capability;
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
use beardog_errors::{BearDogError, BearDogResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
// Import required types
use crate::tunnel::hsm::types::tier::HsmTier;
// Re-export types from submodules
pub use capability::DefaultHsmCapabilityDetector;
pub use config::{HsmManagerConfig, SimpleHsmTier};
pub use failover::{CircuitBreaker, CircuitBreakerState, DefaultHsmFailoverManager};
pub use health::DefaultHsmHealthMonitor;
pub use operation_router::{
    HsmOperationRouter, HsmSelectionResult, OperationRoutingRules, OperationType,};


pub use performance::{HsmPerformanceTracker, OperationMetrics};
/// HSM provider selection result
#[derive(Clone)]
pub struct HsmProviderSelection {
    /// The selected HSM provider instance
    pub provider: impl HsmProvider + Send + Sync + 'static,
    /// Unique identifier for the provider
    pub provider_id: String,
    /// Security tier of the provider
    pub tier: HsmTier,
    /// Confidence score for the selection (0.0 to 1.0)
    pub confidence: f64,
    /// Estimated latency in milliseconds for operations
    pub estimated_latency_ms: f64,
}
/// HSM Manager - Central orchestrator for HSM providers
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
