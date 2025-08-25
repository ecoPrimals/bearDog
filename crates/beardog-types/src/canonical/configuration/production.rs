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


/// # Canonical Production Configuration
///
/// **UNIFIED PRODUCTION CONFIGURATION** - Single source of truth for all production settings
/// This module consolidates production configuration from:
/// - beardog-production/src/production/config.rs::ProductionConfig
/// - beardog-deploy/src/lib.rs::DeploymentConfig
/// - Various scattered production and deployment settings across the ecosystem

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **CANONICAL PRODUCTION CONFIGURATION** - Main production deployment settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionConfig {
    /// Deployment environment (Development, Staging, Production, Testing)
    pub environment: Environment,
    /// Unique identifier for this deployment
    pub deployment_id: String,
    /// Unique identifier for this node in the cluster
    pub node_id: String,
    /// Optional cluster configuration for multi-node deployments
    pub cluster_config: Option<ClusterConfig>,
    /// Backup configuration and policies
    pub backup_config: BackupConfig,
    /// Maintenance window and update configuration
    pub maintenance_config: MaintenanceConfig,
    /// Circuit breaker configuration for fault tolerance
    pub circuit_breaker_config: CircuitBreakerConfig,
    /// Health monitoring configuration
    pub health_monitoring: HealthMonitoringConfig,
    /// Resource limits and constraints
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

/// **CANONICAL DEPLOYMENT ENVIRONMENT** - Environment type specification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Environment {
    /// Development environment - relaxed security, debug logging
    Development,
    /// Staging environment - production-like, but with test data
    Staging,
    /// Production environment - maximum security, performance optimized
    Production,
    /// Testing environment - for automated testing and validation
    Testing,
}

/// **CANONICAL CLUSTER CONFIGURATION** - Multi-node deployment settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// List of nodes in the cluster
    pub nodes: Vec<NodeConfig>,
    /// Consensus algorithm for cluster coordination
    pub consensus_algorithm: ConsensusAlgorithm,
    /// Load balancing strategy
    pub load_balancing: LoadBalancingStrategy,
    /// Cluster network configuration
    pub network_config: ClusterNetworkConfig,
    /// Failover configuration
    pub failover_config: FailoverConfig,
}

/// **CANONICAL NODE CONFIGURATION** - Individual node settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Node identifier
    pub id: String,
    /// Node hostname or IP address
    pub hostname: String,
    /// Node port for cluster communication
    pub port: u16,
    /// Node role in the cluster
    pub role: NodeRole,
    /// Node resource allocation
    pub resources: NodeResources,
    /// Node-specific configuration overrides
    pub config_overrides: HashMap<String, serde_json::Value>,
}

/// **CANONICAL BACKUP CONFIGURATION** - Backup and recovery settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Enable automated backups
    pub enabled: bool,
    /// Backup frequency
    pub frequency: BackupFrequency,
    /// Backup retention policy
    pub retention: BackupRetention,
    /// Backup storage location
    pub storage_location: String,
    /// Backup encryption settings
    pub encryption: BackupEncryption,
    /// Backup verification settings
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

/// **CANONICAL MAINTENANCE CONFIGURATION** - Maintenance window and update settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceConfig {
    /// Enable automated maintenance
    pub enabled: bool,
    /// Maintenance window schedule
    pub window: MaintenanceWindow,
    /// Update policy
    pub update_policy: UpdatePolicy,
    /// Rollback configuration
    pub rollback_config: RollbackConfig,
    /// Notification settings for maintenance
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

/// **CANONICAL CIRCUIT BREAKER CONFIGURATION** - Fault tolerance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Enable circuit breaker
    pub enabled: bool,
    /// Failure threshold to open circuit
    pub failure_threshold: u32,
    /// Success threshold to close circuit
    pub success_threshold: u32,
    /// Timeout duration in open state
    pub timeout: Duration,
    /// Half-open state retry interval
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

/// **CANONICAL HEALTH MONITORING CONFIGURATION** - Health check and monitoring settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMonitoringConfig {
    /// Enable health monitoring
    pub enabled: bool,
    /// Health check interval
    pub check_interval: Duration,
    /// Health check timeout
    pub check_timeout: Duration,
    /// Unhealthy threshold
    pub unhealthy_threshold: u32,
    /// Health check endpoints
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

/// **CANONICAL RESOURCE LIMITS CONFIGURATION** - Resource constraints and limits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimitsConfig {
    /// Maximum memory usage (in bytes)
    pub max_memory: u64,
    /// Maximum CPU usage (as percentage)
    pub max_cpu_percent: f64,
    /// Maximum disk usage (in bytes)
    pub max_disk: u64,
    /// Maximum network bandwidth (in bytes per second)
    pub max_network_bandwidth: u64,
    /// Maximum concurrent connections
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

// Supporting enums and structs

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