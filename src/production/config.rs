//! Production configuration management
//!
//! This module provides configuration structures and types for production
//! deployments, including environment settings, cluster configuration,
//! backup policies, and operational parameters.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Production deployment configuration
///
/// Central configuration for production deployments, including environment settings,
/// cluster configuration, backup policies, and operational procedures.
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
}

/// Deployment environment type
///
/// Specifies the type of environment where BearDog is deployed, affecting
/// configuration defaults, security policies, and operational procedures.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// Cluster configuration for multi-node deployments
///
/// Defines the configuration for distributed BearDog deployments across
/// multiple nodes, including consensus algorithms and networking settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterConfig {
    /// List of nodes in the cluster
    pub nodes: Vec<NodeConfig>,
    /// Consensus algorithm for cluster coordination
    pub consensus_algorithm: ConsensusAlgorithm,
    /// Interval between heartbeat messages
    pub heartbeat_interval: Duration,
    /// Timeout for leader election
    pub election_timeout: Duration,
}

/// Individual node configuration within a cluster
///
/// Configuration for a single node in a BearDog cluster, including
/// network settings, role, and operational parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    /// Unique identifier for this node
    pub node_id: String,
    /// Network address for this node
    pub address: String,
    /// Network port for this node
    pub port: u16,
    /// Weight for load balancing decisions
    pub weight: u32,
    /// Role of this node in the cluster
    pub role: NodeRole,
    /// Whether this node is currently active
    pub active: bool,
}

/// Node role within a cluster
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeRole {
    /// Leader node - coordinates cluster operations
    Leader,
    /// Follower node - follows leader decisions
    Follower,
    /// Candidate node - competing for leadership
    Candidate,
    /// Observer node - passive monitoring only
    Observer,
}

/// Consensus algorithm for cluster coordination
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusAlgorithm {
    /// Raft consensus algorithm
    Raft,
    /// Practical Byzantine Fault Tolerance
    PBFT,
    /// Proof of Work (for testing only)
    PoW,
    /// Proof of Stake (for testing only)
    PoS,
}

/// Backup configuration and policies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Whether automated backups are enabled
    pub enabled: bool,
    /// Backup schedule as cron expression
    pub schedule: String,
    /// Number of days to retain backups
    pub retention_days: u32,
    /// Storage backend for backups
    pub storage_backend: BackupStorage,
    /// Whether backup encryption is enabled
    pub encryption_enabled: bool,
    /// Whether backup compression is enabled
    pub compression_enabled: bool,
}

/// Storage backend for backups
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BackupStorage {
    /// Store backups on local filesystem
    LocalFilesystem {
        /// Local filesystem path for backups
        path: String,
    },
    /// Store backups in Amazon S3
    S3 {
        /// S3 bucket name
        bucket: String,
        /// AWS region
        region: String,
    },
    /// Store backups in Google Cloud Storage
    GCS {
        /// GCS bucket name
        bucket: String,
    },
    /// Store backups in Azure Blob Storage
    Azure {
        /// Azure container name
        container: String,
    },
}

/// Maintenance window and update configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceConfig {
    /// Scheduled maintenance windows
    pub maintenance_windows: Vec<MaintenanceWindow>,
    /// Whether automatic updates are enabled
    pub auto_update: bool,
    /// Whether rollback is enabled for failed updates
    pub rollback_enabled: bool,
    /// Whether canary deployments are used
    pub canary_deployment: bool,
    /// Whether blue-green deployments are used
    pub blue_green_deployment: bool,
}

/// Scheduled maintenance window
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    /// Day of the week (Monday, Tuesday, etc.)
    pub day_of_week: String,
    /// Start time (HH:MM format)
    pub start_time: String,
    /// Duration of maintenance window in minutes
    pub duration_minutes: u32,
    /// Timezone for the maintenance window
    pub timezone: String,
}

/// Circuit breaker configuration for fault tolerance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    /// Number of failures before opening circuit
    pub failure_threshold: u32,
    /// Timeout before attempting recovery
    pub recovery_timeout: Duration,
    /// Minimum request volume before evaluating circuit state
    pub request_volume_threshold: u32,
    /// Error percentage threshold for opening circuit
    pub error_threshold_percentage: f64,
}

impl Default for ProductionConfig {
    fn default() -> Self {
        Self {
            environment: Environment::Development,
            deployment_id: "beardog-dev-001".to_string(),
            node_id: "node-001".to_string(),
            cluster_config: None,
            backup_config: BackupConfig::default(),
            maintenance_config: MaintenanceConfig::default(),
            circuit_breaker_config: CircuitBreakerConfig::default(),
        }
    }
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            schedule: "0 2 * * *".to_string(), // Daily at 2 AM
            retention_days: 7,
            storage_backend: BackupStorage::LocalFilesystem {
                path: "/tmp/beardog/backups".to_string(),
            },
            encryption_enabled: true,
            compression_enabled: true,
        }
    }
}

impl Default for MaintenanceConfig {
    fn default() -> Self {
        Self {
            maintenance_windows: vec![MaintenanceWindow {
                day_of_week: "Sunday".to_string(),
                start_time: "02:00".to_string(),
                duration_minutes: 60,
                timezone: "UTC".to_string(),
            }],
            auto_update: false,
            rollback_enabled: true,
            canary_deployment: false,
            blue_green_deployment: false,
        }
    }
}

impl Default for CircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            recovery_timeout: Duration::from_secs(30),
            request_volume_threshold: 20,
            error_threshold_percentage: 50.0,
        }
    }
}

impl Environment {
    /// Check if this is a production environment
    pub fn is_production(&self) -> bool {
        matches!(self, Environment::Production)
    }

    /// Check if this is a development environment
    pub fn is_development(&self) -> bool {
        matches!(self, Environment::Development)
    }

    /// Check if this is a staging environment
    pub fn is_staging(&self) -> bool {
        matches!(self, Environment::Staging)
    }

    /// Check if this is a testing environment
    pub fn is_testing(&self) -> bool {
        matches!(self, Environment::Testing)
    }

    /// Get the default log level for this environment
    pub fn default_log_level(&self) -> &'static str {
        match self {
            Environment::Development => "debug",
            Environment::Testing => "debug",
            Environment::Staging => "info",
            Environment::Production => "warn",
        }
    }

    /// Get the default metrics retention period for this environment
    pub fn metrics_retention_days(&self) -> u32 {
        match self {
            Environment::Development => 7,
            Environment::Testing => 3,
            Environment::Staging => 30,
            Environment::Production => 90,
        }
    }
}

impl NodeConfig {
    /// Create a new node configuration
    pub fn new(node_id: String, address: String, port: u16) -> Self {
        Self {
            node_id,
            address,
            port,
            weight: 100,
            role: NodeRole::Follower,
            active: true,
        }
    }

    /// Check if this node is a leader
    pub fn is_leader(&self) -> bool {
        matches!(self.role, NodeRole::Leader)
    }

    /// Check if this node is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Get the full address (host:port) for this node
    pub fn full_address(&self) -> String {
        format!("{}:{}", self.address, self.port)
    }
}

impl ClusterConfig {
    /// Create a new cluster configuration
    pub fn new(nodes: Vec<NodeConfig>) -> Self {
        Self {
            nodes,
            consensus_algorithm: ConsensusAlgorithm::Raft,
            heartbeat_interval: Duration::from_secs(5),
            election_timeout: Duration::from_secs(10),
        }
    }

    /// Get all active nodes in the cluster
    pub fn active_nodes(&self) -> Vec<&NodeConfig> {
        self.nodes.iter().filter(|node| node.active).collect()
    }

    /// Get the leader node if one exists
    pub fn leader_node(&self) -> Option<&NodeConfig> {
        self.nodes.iter().find(|node| node.is_leader())
    }

    /// Get the total weight of all active nodes
    pub fn total_weight(&self) -> u32 {
        self.active_nodes().iter().map(|node| node.weight).sum()
    }
}

impl BackupConfig {
    /// Create a production backup configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            schedule: "0 2 * * *".to_string(), // Daily at 2 AM
            retention_days: 30,
            storage_backend: BackupStorage::S3 {
                bucket: "beardog-production-backups".to_string(),
                region: "us-east-1".to_string(),
            },
            encryption_enabled: true,
            compression_enabled: true,
        }
    }

    /// Create a development backup configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            schedule: "0 2 * * 0".to_string(), // Weekly on Sunday at 2 AM
            retention_days: 7,
            storage_backend: BackupStorage::LocalFilesystem {
                path: "/tmp/beardog/backups".to_string(),
            },
            encryption_enabled: false,
            compression_enabled: true,
        }
    }

    /// Check if backups are enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl MaintenanceConfig {
    /// Create a production maintenance configuration
    pub fn production() -> Self {
        Self {
            maintenance_windows: vec![MaintenanceWindow {
                day_of_week: "Sunday".to_string(),
                start_time: "02:00".to_string(),
                duration_minutes: 120,
                timezone: "UTC".to_string(),
            }],
            auto_update: false,
            rollback_enabled: true,
            canary_deployment: true,
            blue_green_deployment: true,
        }
    }

    /// Create a development maintenance configuration
    pub fn development() -> Self {
        Self {
            maintenance_windows: vec![],
            auto_update: true,
            rollback_enabled: true,
            canary_deployment: false,
            blue_green_deployment: false,
        }
    }

    /// Check if auto-updates are enabled
    pub fn auto_updates_enabled(&self) -> bool {
        self.auto_update
    }

    /// Check if there are any maintenance windows defined
    pub fn has_maintenance_windows(&self) -> bool {
        !self.maintenance_windows.is_empty()
    }
}

impl CircuitBreakerConfig {
    /// Create a production circuit breaker configuration
    pub fn production() -> Self {
        Self {
            failure_threshold: 3,
            recovery_timeout: Duration::from_secs(60),
            request_volume_threshold: 10,
            error_threshold_percentage: 25.0,
        }
    }

    /// Create a development circuit breaker configuration
    pub fn development() -> Self {
        Self {
            failure_threshold: 10,
            recovery_timeout: Duration::from_secs(10),
            request_volume_threshold: 5,
            error_threshold_percentage: 75.0,
        }
    }

    /// Check if the circuit breaker is configured for high sensitivity
    pub fn is_high_sensitivity(&self) -> bool {
        self.failure_threshold <= 3 && self.error_threshold_percentage <= 25.0
    }
} 