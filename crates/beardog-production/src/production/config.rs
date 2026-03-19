// SPDX-License-Identifier: AGPL-3.0-only



// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

pub use beardog_types::canonical::config::production::{
    ProductionConfig, Environment, ClusterConfig, NodeConfig, NodeRole, NodeResources,
    ConsensusAlgorithm, LoadBalancingStrategy, BackupConfig, BackupStorage, BackupSchedule,
    MaintenanceConfig, MaintenanceWindow, CircuitBreakerConfig, HealthMonitoringConfig,
    ResourceLimitsConfig, ClusterNetworkConfig, FailoverConfig, RollbackConfig
};

pub struct ProductionConfigValidator;

impl ProductionConfigValidator {

/// Validate Config operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates config
    /// Validates config
    pub fn validate_config(config: &ProductionConfig) -> Result<(), BearDogError> {
        Self::validate_cluster_config(&config.cluster)?;
        Self::validate_backup_config(&config.backup)?;
        Self::validate_maintenance_config(&config.maintenance)?;
        Ok(())
    }

/// Validate Cluster Config operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates cluster_config
    /// Validates cluster_config
    pub fn validate_cluster_config(config: &ClusterConfig) -> Result<(), BearDogError> {
        if config.nodes.is_empty() {
            return Err(BearDogError::configuration("Cluster must have at least one node".to_string()));
        }

        for node in &config.nodes {
            Self::validate_node_config(node)?;
        }
        
        Ok(())
    }

/// Validate Node Config operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates node_config
    /// Validates node_config
    pub fn validate_node_config(config: &NodeConfig) -> Result<(), BearDogError> {
        if config.id.is_empty() {
            return Err(BearDogError::configuration("Node ID cannot be empty".to_string()));
        }
        
        if config.hostname.is_empty() {
            return Err(BearDogError::configuration("Node hostname cannot be empty".to_string()));
        }
        
        if config.port == 0 {
            return Err(BearDogError::configuration("Node port must be greater than 0".to_string()));
        }
        
        Ok(())
    }

/// Validate Backup Config operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates backup_config
    /// Validates backup_config
    pub fn validate_backup_config(config: &BackupConfig) -> Result<(), BearDogError> {
        if config.enabled && config.retention_days == 0 {
            return Err(BearDogError::configuration("Backup retention days must be greater than 0 when backups are enabled".to_string()));
        }
        
        Ok(())
    }

/// Validate Maintenance Config operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Validates maintenance_config
    /// Validates maintenance_config
    pub fn validate_maintenance_config(config: &MaintenanceConfig) -> Result<(), BearDogError> {
        for window in &config.maintenance_windows {
            if window.duration_minutes == 0 {
                return Err(BearDogError::configuration(ProductionConfig,
}

impl ProductionConfigBuilder {

/// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            config: ProductionConfig::default(),
        }
    }

/// Environment operation.
    pub fn environment(mut self, env: Environment) -> Self {
        self.config.environment = env;
        self
    }

/// Cluster operation.
    pub fn cluster(mut self, cluster: ClusterConfig) -> Self {
        self.config.cluster = cluster;
        self
    }

/// Backup operation.
    pub fn backup(mut self, backup: BackupConfig) -> Self {
        self.config.backup = backup;
        self
    }

/// Maintenance operation.
    pub fn maintenance(mut self, maintenance: MaintenanceConfig) -> Self {
        self.config.maintenance = maintenance;
        self
    }

/// Build operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Builds component
    /// Builds component
    pub fn build(self) -> Result<ProductionConfig, BearDogError> {
        ProductionConfigValidator::validate_config(&self.config)?;
        Ok(self.config)
    }
}

impl Default for ProductionConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_production_config_validation() {
        let config = ProductionConfig::default();
        assert!(ProductionConfigValidator::validate_config(&config).is_ok());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_production_config_builder() {
        let config = ProductionConfigBuilder::new()
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: core
            // TEST_PRIORITY: normal
            .environment(Environment::Development)
            .build();
        
        assert!(config.is_ok());
        let config = config.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
})?;
        assert_eq!(config.environment, Environment::Development);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_empty_cluster_validation() {
        let mut config = ProductionConfig::default();
        config.cluster.nodes.clear();
        
        let result = ProductionConfigValidator::validate_config(&config);
        assert!(result.is_err());
    }
}
