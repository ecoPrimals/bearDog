

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

pub use beardog_types::canonical::configuration::production::{
    ProductionConfig, Environment, ClusterConfig, NodeConfig, NodeRole, NodeResources,
    ConsensusAlgorithm, LoadBalancingStrategy, BackupConfig, BackupStorage, BackupSchedule,
    MaintenanceConfig, MaintenanceWindow, CircuitBreakerConfig, HealthMonitoringConfig,
    ResourceLimitsConfig, ClusterNetworkConfig, FailoverConfig, RollbackConfig
};

pub struct ProductionConfigValidator;

impl ProductionConfigValidator {

    pub fn validate_config(config: &ProductionConfig) -> BearDogResult<()> {
        Self::validate_cluster_config(&config.cluster)?;
        Self::validate_backup_config(&config.backup)?;
        Self::validate_maintenance_config(&config.maintenance)?;
        Ok(())
    }

    pub fn validate_cluster_config(config: &ClusterConfig) -> BearDogResult<()> {
        if config.nodes.is_empty() {
            return Err(BearDogError::configuration("Cluster must have at least one node".to_string()));
        }

        for node in &config.nodes {
            Self::validate_node_config(node)?;
        }
        
        Ok(())
    }

    pub fn validate_node_config(config: &NodeConfig) -> BearDogResult<()> {
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

    pub fn validate_backup_config(config: &BackupConfig) -> BearDogResult<()> {
        if config.enabled && config.retention_days == 0 {
            return Err(BearDogError::configuration("Backup retention days must be greater than 0 when backups are enabled".to_string()));
        }
        
        Ok(())
    }

    pub fn validate_maintenance_config(config: &MaintenanceConfig) -> BearDogResult<()> {
        for window in &config.maintenance_windows {
            if window.duration_minutes == 0 {
                return Err(BearDogError::configuration("Maintenance window duration must be greater than 0".to_string()));
            }
        }
        
        Ok(())
    }
}

pub struct ProductionConfigBuilder {
    config: ProductionConfig,
}

impl ProductionConfigBuilder {

    pub fn new() -> Self {
        Self {
            config: ProductionConfig::default(),
        }
    }

    pub fn environment(mut self, env: Environment) -> Self {
        self.config.environment = env;
        self
    }

    pub fn cluster(mut self, cluster: ClusterConfig) -> Self {
        self.config.cluster = cluster;
        self
    }

    pub fn backup(mut self, backup: BackupConfig) -> Self {
        self.config.backup = backup;
        self
    }

    pub fn maintenance(mut self, maintenance: MaintenanceConfig) -> Self {
        self.config.maintenance = maintenance;
        self
    }

    pub fn build(self) -> BearDogResult<ProductionConfig> {
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

    #[test]
    fn test_production_config_builder() {
        let config = ProductionConfigBuilder::new()
            .environment(Environment::Development)
            .build();
        
        assert!(config.is_ok());
        let config = config.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        assert_eq!(config.environment, Environment::Development);
    }

    #[test]
    fn test_empty_cluster_validation() {
        let mut config = ProductionConfig::default();
        config.cluster.nodes.clear();
        
        let result = ProductionConfigValidator::validate_config(&config);
        assert!(result.is_err());
    }
}
