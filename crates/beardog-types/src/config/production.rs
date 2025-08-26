

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedProductionConfig {
    pub disaster_recovery: DisasterRecoveryConfig,
    pub failover: FailoverConfig,
    pub cluster: ClusterConfig,
    pub maintenance: MaintenanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterRecoveryConfig {
    pub enabled: bool,
    pub backup_interval: Duration,
    pub recovery_time_objective: Duration,
    pub recovery_point_objective: Duration,
}

impl Default for DisasterRecoveryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            backup_interval: Duration::from_secs(3600), // 1 hour
            recovery_time_objective: Duration::from_secs(1800), // 30 minutes
            recovery_point_objective: Duration::from_secs(300), // 5 minutes
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    pub enabled: bool,
    pub failover_timeout: Duration,
    pub health_check_interval: Duration,
    pub max_failover_attempts: u32,
}

impl Default for FailoverConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            failover_timeout: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            max_failover_attempts: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    pub enabled: bool,
    pub cluster_name: String,
    pub node_count: u32,
    pub consensus_timeout: Duration,
}

impl Default for ClusterConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            cluster_name: "beardog-cluster".to_string(),
            node_count: 3,
            consensus_timeout: Duration::from_secs(10),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceConfig {
    pub maintenance_window: String,
    pub auto_maintenance: bool,
    pub backup_before_maintenance: bool,
    pub maintenance_timeout: Duration,
}

impl Default for MaintenanceConfig {
    fn default() -> Self {
        Self {
            maintenance_window: "02:00-04:00".to_string(),
            auto_maintenance: false,
            backup_before_maintenance: true,
            maintenance_timeout: Duration::from_secs(7200), // 2 hours
        }
    }
}
