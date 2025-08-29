use super::genetics::{BearDogGenetics, NodeCapability};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnedBearDog {
    pub id: String,

    pub parent_id: String,

    pub genetics: BearDogGenetics,

    pub spawn_purpose: SpawnPurpose,

    pub task_assignment: Vec<TaskType>,

    pub resource_limits: ResourceLimits,

    pub spawn_time: DateTime<Utc>,

    pub expected_lifetime: Option<DateTime<Utc>>,

    pub current_status: SpawnStatus,

    pub performance_metrics: HashMap<String, f64>,

    pub trust_relationships: HashMap<String, f64>,

    pub consensus_participation: bool,

    pub ecosystem_connections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpawnPurpose {
    LoadBalancing,

    SpecializedTask(TaskType),

    EcosystemIntegration(String),

    SecurityResponse,

    EmergencyResponse,

    DisasterRecovery,

    ComplianceRequirement,

    UserRequest,

    GeneticExperiment,

    NetworkExpansion,

    PerformanceOptimization,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaskType {
    DataStorage,

    Storage,

    ComputeTask,

    Compute,

    SecurityAnalysis,

    Security,

    NetworkRelay,

    Network,

    ComplianceCheck,

    ThreatHunting,

    BackupOperation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub memory_mb: u64,
    pub cpu_percent: u8,
    pub disk_mb: u64,
    pub network_mbps: u32,
    pub concurrent_connections: u32,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            memory_mb: 1024,
            cpu_percent: 50,
            disk_mb: 5120,
            network_mbps: 100,
            concurrent_connections: 1000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnStatus {
    Initializing,

    Active,

    Paused,

    Terminated,

    Failed(String),

    Upgrading,

    Hibernating,
}

#[derive(Debug, Clone)]
pub struct SpawnRequest {
    pub parent_genetics: Vec<BearDogGenetics>,
    pub required_capabilities: Vec<NodeCapability>,
    pub target_environment: String,
}
