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


/// Production Configuration Module - CANONICAL MODERNIZATION COMPLETE ✅
///
/// **CONFIGURATION UNIFICATION ACHIEVED**
/// This module now uses canonical configuration types from beardog_types::canonical::configuration::production
/// All duplicate configuration structs have been eliminated and replaced with canonical types.
///
/// ## Migration Benefits:
/// - ✅ **Eliminates 5+ duplicate configuration structs** (ClusterConfig, NodeConfig, BackupConfig, etc.)
/// - ✅ **Single source of truth** for all production configurations
/// - ✅ **Reduced maintenance burden** - configurations defined once
/// - ✅ **Better consistency** across the codebase

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ✅ CANONICAL CONFIGURATION IMPORTS - No more duplicates
pub use beardog_types::canonical::configuration::production::{
    ProductionConfig, Environment, ClusterConfig, NodeConfig, NodeRole, NodeResources,
    ConsensusAlgorithm, LoadBalancingStrategy, BackupConfig, BackupStorage, BackupSchedule,
    MaintenanceConfig, MaintenanceWindow, CircuitBreakerConfig, HealthMonitoringConfig,
    ResourceLimitsConfig, ClusterNetworkConfig, FailoverConfig, RollbackConfig
};

/// **PRODUCTION CONFIGURATION VALIDATION** - Ensures all configurations are valid
pub struct ProductionConfigValidator;

impl ProductionConfigValidator {
    /// Validate complete production configuration
    pub fn validate_config(config: &ProductionConfig) -> BearDogResult<()> {
        Self::validate_cluster_config(&config.cluster)?;
        Self::validate_backup_config(&config.backup)?;
        Self::validate_maintenance_config(&config.maintenance)?;
        Ok(())
    }

    /// Validate cluster configuration
    pub fn validate_cluster_config(config: &ClusterConfig) -> BearDogResult<()> {
        if config.nodes.is_empty() {
            return Err(BearDogError::configuration("Cluster must have at least one node".to_string()));
        }
        
        // Validate each node
        for node in &config.nodes {
            Self::validate_node_config(node)?;
        }
        
        Ok(())
    }

    /// Validate individual node configuration
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

    /// Validate backup configuration
    pub fn validate_backup_config(config: &BackupConfig) -> BearDogResult<()> {
        if config.enabled && config.retention_days == 0 {
            return Err(BearDogError::configuration("Backup retention days must be greater than 0 when backups are enabled".to_string()));
        }
        
        Ok(())
    }

    /// Validate maintenance configuration
    pub fn validate_maintenance_config(config: &MaintenanceConfig) -> BearDogResult<()> {
        for window in &config.maintenance_windows {
            if window.duration_minutes == 0 {
                return Err(BearDogError::configuration("Maintenance window duration must be greater than 0".to_string()));
            }
        }
        
        Ok(())
    }
}

/// **PRODUCTION CONFIGURATION BUILDER** - Helps build valid configurations
pub struct ProductionConfigBuilder {
    config: ProductionConfig,
}

impl ProductionConfigBuilder {
    /// Create new builder with defaults
    pub fn new() -> Self {
        Self {
            config: ProductionConfig::default(),
        }
    }

    /// Set environment
    pub fn environment(mut self, env: Environment) -> Self {
        self.config.environment = env;
        self
    }

    /// Set cluster configuration
    pub fn cluster(mut self, cluster: ClusterConfig) -> Self {
        self.config.cluster = cluster;
        self
    }

    /// Set backup configuration
    pub fn backup(mut self, backup: BackupConfig) -> Self {
        self.config.backup = backup;
        self
    }

    /// Set maintenance configuration
    pub fn maintenance(mut self, maintenance: MaintenanceConfig) -> Self {
        self.config.maintenance = maintenance;
        self
    }

    /// Build and validate configuration
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
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
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
