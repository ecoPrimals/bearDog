// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::genetics::{BearDogGenetics, NodeCapability};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnedBearDog {
    pub spawn_id: String,
    pub parent_id: String,
    /// The genetics value
    pub genetics: BearDogGenetics,
    /// The spawn purpose value
    pub spawn_purpose: SpawnPurpose,
    /// Collection of task assignment
    pub task_assignment: Vec<TaskType>,
    /// The resource limits value
    pub resource_limits: ResourceLimits,
    pub spawn_time: DateTime<Utc>,
    pub expected_lifetime: Option<DateTime<Utc>>,
    /// Current status of the current
    pub current_status: SpawnStatus,
    pub performance_metrics: HashMap<String, f64>,
    /// Mapping of trust relationships
    pub trust_relationships: HashMap<String, f64>,
    /// Whether `consensus_participation` is enabled
    pub consensus_participation: bool,
    /// Collection of ecosystem connections
    pub ecosystem_connections: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    /// Number of `memory_mb`
    pub memory_mb: u64,
    /// Number of `cpu_percent`
    pub cpu_percent: u8,
    /// Number of `disk_mb`
    pub disk_mb: u64,
    /// Number of `network_mbps`
    pub network_mbps: u32,
    /// Number of `concurrent_connections`
    pub concurrent_connections: u32,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            memory_mb: std::env::var("BEARDOG_RESOURCE_MEMORY_MB")
                .ok()
                .and_then(|m| m.parse().ok())
                .unwrap_or(1024), // 1GB default
            cpu_percent: 50,
            disk_mb: std::env::var("BEARDOG_RESOURCE_DISK_MB")
                .ok()
                .and_then(|d| d.parse().ok())
                .unwrap_or(5120), // 5GB default
            network_mbps: 100,
            concurrent_connections: std::env::var("BEARDOG_MAX_CONCURRENT_CONNECTIONS")
                .ok()
                .and_then(|c| c.parse().ok())
                .unwrap_or(1000), // 1000 connections default
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnRequest {
    /// Collection of parent genetics
    pub parent_genetics: Vec<BearDogGenetics>,
    /// Collection of required capabilities
    pub required_capabilities: Vec<NodeCapability>,
    /// The target environment value
    pub target_environment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnPurpose {
    /// Represents task execution variant
    TaskExecution,
    /// Currently securitymonitoring
    SecurityMonitoring,
    /// Currently dataprocessing
    DataProcessing,
    /// Represents network optimization variant
    NetworkOptimization,
    /// Represents resource management variant
    ResourceManagement,
    /// Represents experimentation variant
    Experimentation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of task
pub enum TaskType {
    /// Represents cryptographic variant
    Cryptographic,
    /// Represents networking task variant
    NetworkingTask,
    /// Represents data analysis variant
    DataAnalysis,
    /// Represents security audit variant
    SecurityAudit,
    /// Represents resource optimization variant
    ResourceOptimization,
    /// Represents ecosystem maintenance variant
    EcosystemMaintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpawnStatus {
    /// Currently initializing
    Initializing,
    /// Active or enabled state
    Active,
    /// Represents idle variant
    Idle,
    /// Currently terminating
    Terminating,
    /// State indicating terminated
    Terminated,
    /// Error or failure state
    Failed,
}
