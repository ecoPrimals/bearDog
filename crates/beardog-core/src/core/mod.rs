

pub mod components;
pub mod lifecycle;
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::HealthStatus;
use beardog_types::config::unified::BearDogConfig; // Use canonical unified config
use beardog_errors::idiomatic::SystemResult;

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

pub struct BearDogCore {
    pub state: Arc<RwLock<CoreState>>,
    pub config: BearDogConfig,
    pub security: Arc<BearDogSecurityProvider>,
    pub monitor: Arc<SystemMonitor>,
    pub genetic_optimizer: Arc<GeneticOptimizer>,
}
impl BearDogCore {

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

    pub async fn initialize(&self) -> Result<(), SystemError> {
        info!("🚀 Initializing BearDog Core");

        {
            let mut state = self.state.write().await;
            state
                .components
                .insert("core".to_string(), ComponentStatus::Starting);
        }

        self.security.initialize().await?;
        self.monitor.start().await?;
        self.genetic_optimizer.initialize().await?;

                .insert("core".to_string(), ComponentStatus::Running);
            state.overall_health = HealthStatus::healthy();
        info!("✅ BearDog Core initialized successfully");
        Ok(())

