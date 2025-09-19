

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


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

/// New operation.
    /// Creates a new instance
    pub fn new(config: GlobalDeploymentConfig) -> Self {
        Self {
            deployment_manager: Arc::new(&deployment::DeploymentManager::new(config)),
            monitoring_system: Arc::new(&monitoring::GlobalHealthMonitor::new(config.monitoring)),
            load_balancer: Arc::new(&load_balancer::IntelligentLoadBalancer::new(config.load_balancing)),
            cdn_manager: Arc::new(&cdn::CDNManager::new(config.cdn)),
            config,
        }
    }

/// Initialize operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Initializes componentialize
    /// Initializes componentialize
    pub fn initialize(&self) -> Result<(), BearDogError> {

        self.deployment_manager.initialize()?;
        self.monitoring_system.start_monitoring()?;
        self.load_balancer.initialize()?;
        self.cdn_manager.initialize()?;
        
        Ok(())
    }

/// Deploy Globally operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn deploy_globally(&self) -> Result<(), BearDogError> {
        self.deployment_manager.deploy_all_regions()
    }

/// Get Global Health operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets global_health
    /// Gets global_health
    pub fn get_global_health(&self) -> Result<GlobalHealthStatus, BearDogError> {
        self.monitoring_system.get_global_health()
    }

/// Shutdown operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn shutdown(&self) -> Result<(), BearDogError> {
        self.cdn_manager.shutdown()?;
        self.load_balancer.shutdown()?;
        self.monitoring_system.shutdown()?;
        self.deployment_manager.shutdown()?;
        
        Ok(())
    }
} 
