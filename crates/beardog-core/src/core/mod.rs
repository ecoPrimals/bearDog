pub mod components;
pub mod lifecycle;
use crate::types::{BearDogConfig, BearDogSecurityProvider, GeneticOptimizer, SystemMonitor};
use beardog_errors::BearDogError;
use beardog_types::canonical::{ComponentStatus, HealthStatus};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub type SystemError = BearDogError;

#[derive(Debug)]
pub struct CoreState {
    pub components: HashMap<String, ComponentStatus>,
    pub overall_health: HealthStatus,
    pub start_time: std::time::Instant,
}

impl Default for CoreState {
    fn default() -> Self {
        Self {
            components: HashMap::new(),
            overall_health: HealthStatus::Healthy,
            start_time: std::time::Instant::now(),
        }
    }
}

#[derive(Debug)]
pub struct BearDogCore {
    pub config: BearDogConfig,
    pub state: Arc<RwLock<CoreState>>,
    pub security: BearDogSecurityProvider,
    pub monitor: SystemMonitor,
    pub genetic_optimizer: GeneticOptimizer,
}

impl BearDogCore {
    pub fn new(config: BearDogConfig) -> Self {
        Self {
            config,
            state: Arc::new(RwLock::new(CoreState::default())),
            security: BearDogSecurityProvider::new(),
            monitor: SystemMonitor::new().unwrap_or_default(),
            genetic_optimizer: GeneticOptimizer::new(),
        }
    }

    pub async fn initialize(&self) -> Result<(), SystemError> {
        info!("🚀 Initializing BearDog Core");

        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("core".to_string(), ComponentStatus::Starting);
        }

        // Initialize components
        self.monitor.start().await?;
        self.genetic_optimizer.initialize().await?;

        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("core".to_string(), ComponentStatus::Running);
            state.overall_health = HealthStatus::Healthy;
        }

        info!("✅ BearDog Core initialized successfully");
        Ok(())
    }

    // shutdown and health_check methods moved to lifecycle.rs
}
