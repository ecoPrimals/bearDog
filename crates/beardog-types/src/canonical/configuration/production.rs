

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionConfig {

    pub environment: Environment,

    pub deployment_id: String,

    pub node_id: String,

    pub cluster_config: Option<ClusterConfig>,

    pub backup_config: BackupConfig,

    pub maintenance_config: MaintenanceConfig,

    pub circuit_breaker_config: CircuitBreakerConfig,

    pub health_monitoring: HealthMonitoringConfig,

    pub resource_limits: ResourceLimitsConfig,
}

impl Default for ProductionConfig {
    fn default() -> Self {
        Self {
            environment: Environment::Development,
            deployment_id: "default-deployment".to_string(),
            node_id: "node-001".to_string(),
            cluster_config: None,
            backup_config: BackupConfig::default(),
            maintenance_config: MaintenanceConfig::default(),
            circuit_breaker_config: CircuitBreakerConfig::default(),
            health_monitoring: HealthMonitoringConfig::default(),
            resource_limits: ResourceLimitsConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Environment {

    Development,

    Staging,

    Production,

    Testing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {

    pub nodes: Vec<NodeConfig>,

    pub consensus_algorithm: ConsensusAlgorithm,

    pub load_balancing: LoadBalancingStrategy,

    pub network_config: ClusterNetworkConfig,

    pub failover_config: FailoverConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {

    pub id: String,

    pub hostname: String,

    pub port: u16,

    pub role: NodeRole,

    pub resources: NodeResources,

    pub config_overrides: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {

    pub enabled: bool,

    pub frequency: BackupFrequency,

    pub retention: BackupRetention,

    pub storage_location: String,

    pub encryption: BackupEncryption,

    pub verification: BackupVerification,
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            frequency: BackupFrequency::Daily,
            retention: BackupRetention::default(),
            storage_location: "/var/backups/beardog".to_string(),
            encryption: BackupEncryption::default(),
            verification: BackupVerification::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceConfig {

    pub enabled: bool,

    pub window: MaintenanceWindow,

    pub update_policy: UpdatePolicy,

    pub rollback_config: RollbackConfig,

    pub notifications: MaintenanceNotifications,
}

impl Default for MaintenanceConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            window: MaintenanceWindow::default(),
            update_policy: UpdatePolicy::default(),
            rollback_config: RollbackConfig::default(),
            notifications: MaintenanceNotifications::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {

    pub enabled: bool,

    pub failure_threshold: u32,

    pub success_threshold: u32,

    pub timeout: Duration,

    pub retry_interval: Duration,
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            failure_threshold: 5,
            success_threshold: 3,
            timeout: Duration::from_secs(60),
            retry_interval: Duration::from_secs(30),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitoringConfig {

    pub enabled: bool,

    pub check_interval: Duration,

    pub check_timeout: Duration,

    pub unhealthy_threshold: u32,

    pub endpoints: Vec<String>,
}

impl Default for HealthMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: Duration::from_secs(30),
            check_timeout: Duration::from_secs(10),
            unhealthy_threshold: 3,
            endpoints: vec!["/health".to_string()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimitsConfig {

    pub max_memory: u64,

    pub max_cpu_percent: f64,

    pub max_disk: u64,

    pub max_network_bandwidth: u64,

    pub max_connections: u32,
}

impl Default for ResourceLimitsConfig {
    fn default() -> Self {
        Self {
            max_memory: 2_147_483_648, // 2GB
            max_cpu_percent: 80.0,
            max_disk: 10_737_418_240, // 10GB
            max_network_bandwidth: 104_857_600, // 100MB/s
            max_connections: 1000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsensusAlgorithm {
    Raft,
    Pbft,
    PoS,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastConnections,
    WeightedRoundRobin,
    IpHash,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeRole {
    Leader,
    Follower,
    Observer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeResources {
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub disk_gb: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterNetworkConfig {
    pub cluster_port: u16,
    pub discovery_port: u16,
    pub encryption_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailoverConfig {
    pub enabled: bool,
    pub detection_timeout: Duration,
    pub failover_timeout: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BackupFrequency {
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRetention {
    pub daily_backups: u32,
    pub weekly_backups: u32,
    pub monthly_backups: u32,
}

impl Default for BackupRetention {
    fn default() -> Self {
        Self {
            daily_backups: 7,
            weekly_backups: 4,
            monthly_backups: 12,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupEncryption {
    pub enabled: bool,
    pub algorithm: String,
    pub key_rotation_days: u32,
}

impl Default for BackupEncryption {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: "AES-256-GCM".to_string(),
            key_rotation_days: 90,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupVerification {
    pub enabled: bool,
    pub verification_frequency: BackupFrequency,
    pub integrity_checks: bool,
}

impl Default for BackupVerification {
    fn default() -> Self {
        Self {
            enabled: true,
            verification_frequency: BackupFrequency::Weekly,
            integrity_checks: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    pub day_of_week: u8, // 0 = Sunday
    pub start_hour: u8,   // 24-hour format
    pub duration_hours: u8,
}

impl Default for MaintenanceWindow {
    fn default() -> Self {
        Self {
            day_of_week: 0, // Sunday
            start_hour: 2,  // 2 AM
            duration_hours: 4,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePolicy {
    pub auto_update: bool,
    pub update_channel: UpdateChannel,
    pub rollback_on_failure: bool,
}

impl Default for UpdatePolicy {
    fn default() -> Self {
        Self {
            auto_update: false,
            update_channel: UpdateChannel::Stable,
            rollback_on_failure: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpdateChannel {
    Stable,
    Beta,
    Alpha,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackConfig {
    pub enabled: bool,
    pub max_rollback_attempts: u32,
    pub rollback_timeout: Duration,
}

impl Default for RollbackConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_rollback_attempts: 3,
            rollback_timeout: Duration::from_secs(300),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceNotifications {
    pub enabled: bool,
    pub email_recipients: Vec<String>,
    pub webhook_urls: Vec<String>,
    pub advance_notice_hours: u32,
}

impl Default for MaintenanceNotifications {
    fn default() -> Self {
        Self {
            enabled: true,
            email_recipients: Vec::new(),
            webhook_urls: Vec::new(),
            advance_notice_hours: 24,
        }
    }
} 