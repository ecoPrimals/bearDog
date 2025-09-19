// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::{
    config::CloudIntegrationConfig,
    connection::CloudConnectionPool,
    metrics::CloudMetrics,
    providers::{CloudIntegration, CloudProvider},
};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub struct EnterpriseCloudManager {
    providers: HashMap<CloudProvider, Box<dyn CloudIntegration>>,

    config: CloudIntegrationConfig,

    connection_pool: Arc<RwLock<CloudConnectionPool>>,

    metrics: Arc<RwLock<CloudMetrics>>,
}

impl EnterpriseCloudManager {
    /// New operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(config: CloudIntegrationConfig) -> Result<Self, BearDogError> {
        info!("🌩️ Initializing enterprise cloud manager");

        Ok(Self {
            providers: HashMap::with_capacity(16),
            config,
            connection_pool: Arc::new(RwLock::new(CloudConnectionPool::new())),
            metrics: Arc::new(RwLock::new(CloudMetrics::new())),
        })
    }

    /// Connect All operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn connect_all(&self) -> Result<(), BearDogError> {
        info!("🔗 Connecting to all configured cloud providers");

        Ok(())
    }

    /// Health Check operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    pub fn health_check(&self) -> Result<bool, BearDogError> {
        Ok(true)
    }

    /// Get Metrics operation.
    /// Gets metrics
    /// Gets metrics
    pub fn get_metrics(&self) -> CloudMetrics {
        self.metrics.read().clone()
    }

    /// Get Metrics Ref operation.
    /// Gets metrics_ref
    /// Returns reference to get metrics
    pub fn get_metrics_ref(&self) -> tokio::sync::RwLockReadGuard<'_, CloudMetrics> {
        self.metrics.read()
    }
}
