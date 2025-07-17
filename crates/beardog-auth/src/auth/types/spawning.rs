//! Spawning system types for BearDog instances
//!
//! This module contains all types related to spawning new BearDog instances,
//! including spawn requests, spawned instances, purposes, and resource limits.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::genetics::{BearDogGenetics, NodeCapability};

/// Spawned BearDog instance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnedBearDog {
    /// Unique identifier for the spawned BearDog
    pub id: String,
    /// ID of the parent BearDog that spawned this instance
    pub parent_id: String,
    /// Genetic information for this BearDog
    pub genetics: BearDogGenetics,
    /// Purpose for spawning this BearDog
    pub spawn_purpose: SpawnPurpose,
    /// Tasks assigned to this BearDog
    pub task_assignment: Vec<TaskType>,
    /// Resource limits for this BearDog
    pub resource_limits: ResourceLimits,
    /// When this BearDog was spawned
    pub spawn_time: DateTime<Utc>,
    /// Expected lifetime of this BearDog
    pub expected_lifetime: Option<DateTime<Utc>>,
    /// Current status of this BearDog
    pub current_status: SpawnStatus,
    /// Performance metrics for this BearDog
    pub performance_metrics: HashMap<String, f64>,
    /// Trust relationships with other nodes
    pub trust_relationships: HashMap<String, f64>,
    /// Whether this BearDog participates in consensus
    pub consensus_participation: bool,
    /// Connections to other ecosystem services
    pub ecosystem_connections: Vec<String>,
}

/// Purpose for spawning new BearDog instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnPurpose {
    /// Spawned for load balancing purposes
    LoadBalancing,
    /// Spawned for a specialized task
    SpecializedTask(TaskType),
    /// Spawned for ecosystem integration
    EcosystemIntegration(String),
    /// Spawned for security response
    SecurityResponse,
    /// Spawned for emergency response
    EmergencyResponse,
    /// Spawned for disaster recovery
    DisasterRecovery,
    /// Spawned to meet compliance requirements
    ComplianceRequirement,
    /// Spawned due to user request
    UserRequest,
    /// Spawned for genetic experimentation
    GeneticExperiment,
    /// Spawned for network expansion
    NetworkExpansion,
    /// Spawned for performance optimization
    PerformanceOptimization,
}

/// Task types for BearDog operations
///
/// Defines the different types of tasks that can be assigned
/// to BearDog nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaskType {
    /// Data storage tasks
    DataStorage,
    /// Legacy storage alias
    Storage,
    /// Computation tasks
    ComputeTask,
    /// Legacy compute alias
    Compute,
    /// Security analysis tasks
    SecurityAnalysis,
    /// Legacy security alias
    Security,
    /// Network relay tasks
    NetworkRelay,
    /// Legacy network alias
    Network,
    /// Compliance checking tasks
    ComplianceCheck,
    /// Threat hunting tasks
    ThreatHunting,
    /// Backup operation tasks
    BackupOperation,
    /// Disaster recovery tasks
    DisasterRecovery,
}

/// Resource limits for spawned instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Maximum memory usage in megabytes
    pub max_memory_mb: u64,
    /// Maximum CPU usage percentage
    pub max_cpu_percent: u8,
    /// Maximum disk usage in megabytes
    pub max_disk_mb: u64,
    /// Maximum network bandwidth in megabits per second
    pub max_network_mbps: u32,
    /// Maximum number of concurrent connections
    pub max_concurrent_connections: u32,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_mb: 1024,
            max_cpu_percent: 50,
            max_disk_mb: 5120,
            max_network_mbps: 100,
            max_concurrent_connections: 1000,
        }
    }
}

/// Status of spawned BearDog instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnStatus {
    /// BearDog is initializing
    Initializing,
    /// BearDog is active and running
    Active,
    /// BearDog is paused
    Paused,
    /// BearDog has been terminated
    Terminated,
    /// BearDog has failed with an error
    Failed(String),
    /// BearDog is upgrading
    Upgrading,
    /// BearDog is hibernating
    Hibernating,
}

/// Spawn request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnRequest {
    /// Parent genetics to inherit from
    pub parent_genetics: Vec<BearDogGenetics>,
    /// Purpose for spawning
    pub spawn_purpose: SpawnPurpose,
    /// Required capabilities for spawned instance
    pub required_capabilities: Vec<NodeCapability>,
    /// Resource limits for spawned instance
    pub resource_limits: ResourceLimits,
    /// Target environment for spawning
    pub target_environment: String,
}
