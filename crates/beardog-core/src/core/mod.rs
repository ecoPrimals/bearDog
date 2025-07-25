//! BearDog Core Module
//!
//! Central coordination for all BearDog core functionality

pub mod components;
pub mod lifecycle;

use crate::types::{
    BearDogSecurityProvider, ComponentStatus, CoreState, GeneticOptimizer, HealthStatus,
    SystemMonitor,
};
use async_trait::async_trait;
use beardog_config::BearDogConfig;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_security::encryption::EncryptionEngine;
use beardog_workflows::workflows::InMemoryApprovalStore;
use beardog_workflows::workflows::InMemoryWorkflowStore;
use beardog_workflows::workflows::MultiPartyWorkflowEngine;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Core BearDog system coordinator
pub struct BearDogCore {
    pub state: Arc<RwLock<CoreState>>,
    pub config: BearDogConfig,
    pub security: Arc<BearDogSecurityProvider>,
    pub monitor: Arc<SystemMonitor>,
    pub genetic_optimizer: Arc<GeneticOptimizer>,
}

impl BearDogCore {
    /// Create new BearDog core instance
    pub async fn new(config: BearDogConfig) -> BearDogResult<Self> {
        let state = Arc::new(RwLock::new(CoreState::default()));
        let security = Arc::new(BearDogSecurityProvider::new()?);
        let monitor = Arc::new(SystemMonitor::new()?);
        let genetic_optimizer = Arc::new(GeneticOptimizer::new()?);

        Ok(Self {
            state,
            config,
            security,
            monitor,
            genetic_optimizer,
        })
    }

    /// Initialize all core components
    pub async fn initialize(&self) -> BearDogResult<()> {
        info!("🚀 Initializing BearDog Core");

        // Update state to show initialization
        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("core".to_string(), ComponentStatus::Starting);
        }

        // Initialize components
        self.security.initialize().await?;
        self.monitor.start().await?;
        self.genetic_optimizer.initialize().await?;

        // Update state to show running
        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("core".to_string(), ComponentStatus::Running);
            state.overall_health = HealthStatus::healthy();
        }

        info!("✅ BearDog Core initialized successfully");
        Ok(())
    }
}

// Re-export core types
pub use components::*;
pub use lifecycle::*;
