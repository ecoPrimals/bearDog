

use beardog_errors::BearDogError;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod types;
pub mod deployment;
pub mod monitoring;
pub mod load_balancer;
pub mod cdn;

pub use types::*;
pub use deployment::*;
pub use monitoring::*;
pub use load_balancer::*;
pub use cdn::*;

pub struct GlobalEdgeManager {
    deployment_manager: Arc<deployment::DeploymentManager>,
    monitoring_system: Arc<monitoring::GlobalHealthMonitor>,
    load_balancer: Arc<load_balancer::IntelligentLoadBalancer>,
    cdn_manager: Arc<cdn::CDNManager>,
    config: GlobalDeploymentConfig,
}

impl GlobalEdgeManager {

    pub fn new(config: GlobalDeploymentConfig) -> Self {
        Self {
            deployment_manager: Arc::new(deployment::DeploymentManager::new(config.clone())),
            monitoring_system: Arc::new(monitoring::GlobalHealthMonitor::new(config.monitoring.clone())),
            load_balancer: Arc::new(load_balancer::IntelligentLoadBalancer::new(config.load_balancing.clone())),
            cdn_manager: Arc::new(cdn::CDNManager::new(config.cdn.clone())),
            config,
        }
    }

    pub async fn initialize(&self) -> Result<(), BearDogError> {

        self.deployment_manager.initialize().await?;
        self.monitoring_system.start_monitoring().await?;
        self.load_balancer.initialize().await?;
        self.cdn_manager.initialize().await?;
        
        Ok(())
    }

    pub async fn deploy_globally(&self) -> Result<(), BearDogError> {
        self.deployment_manager.deploy_all_regions().await
    }

    pub async fn get_global_health(&self) -> Result<GlobalHealthStatus, BearDogError> {
        self.monitoring_system.get_global_health().await
    }

    pub async fn shutdown(&self) -> Result<(), BearDogError> {
        self.cdn_manager.shutdown().await?;
        self.load_balancer.shutdown().await?;
        self.monitoring_system.shutdown().await?;
        self.deployment_manager.shutdown().await?;
        
        Ok(())
    }
} 