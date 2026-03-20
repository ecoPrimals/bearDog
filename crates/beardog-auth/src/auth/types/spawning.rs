// SPDX-License-Identifier: AGPL-3.0-only

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use super::genetics::{BearDogGenetics, NodeCapability};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Runtime view of a spawned BearDog instance and its resource bindings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnedBearDog {
    /// Unique id for this spawn, distinct from genetics or parent ids.
    pub spawn_id: String,
    /// Originating primal or genome identifier responsible for the spawn.
    pub parent_id: String,
    /// The genetics value
    pub genetics: BearDogGenetics,
    /// The spawn purpose value
    pub spawn_purpose: SpawnPurpose,
    /// Collection of task assignment
    pub task_assignment: Vec<TaskType>,
    /// The resource limits value
    pub resource_limits: ResourceLimits,
    /// Wall-clock timestamp when the spawn was created.
    pub spawn_time: DateTime<Utc>,
    /// Optional hard stop after which the child should drain and exit.
    pub expected_lifetime: Option<DateTime<Utc>>,
    /// Current status of the current
    pub current_status: SpawnStatus,
    /// Arbitrary KPIs (CPU%, queue depth, …) keyed by metric name for observability.
    pub performance_metrics: HashMap<String, f64>,
    /// Mapping of trust relationships
    pub trust_relationships: HashMap<String, f64>,
    /// Whether `consensus_participation` is enabled
    pub consensus_participation: bool,
    /// Collection of ecosystem connections
    pub ecosystem_connections: Vec<String>,
}

/// Upper bounds enforced on a spawned instance to protect shared cluster capacity.
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
            memory_mb: 1024,
            cpu_percent: 50,
            disk_mb: 5120,
            network_mbps: 100,
            concurrent_connections: 1000,
        }
    }
}

impl ResourceLimits {
    /// Load limits from process environment via `std::env` (production entry point).
    #[must_use]
    pub fn from_env() -> Self {
        Self::from_env_with(|key| std::env::var(key).ok())
    }

    /// Load limits using a custom lookup (tests inject a map or closure; no global env).
    #[must_use]
    pub fn from_env_with(get: impl Fn(&str) -> Option<String>) -> Self {
        let parse_u64 =
            |key: &str, default: u64| get(key).and_then(|s| s.parse().ok()).unwrap_or(default);
        let parse_u8 =
            |key: &str, default: u8| get(key).and_then(|s| s.parse().ok()).unwrap_or(default);
        let parse_u32 =
            |key: &str, default: u32| get(key).and_then(|s| s.parse().ok()).unwrap_or(default);
        Self {
            memory_mb: parse_u64("BEARDOG_RESOURCE_MEMORY_MB", 1024),
            cpu_percent: parse_u8("BEARDOG_RESOURCE_CPU_PERCENT", 50),
            disk_mb: parse_u64("BEARDOG_RESOURCE_DISK_MB", 5120),
            network_mbps: parse_u32("BEARDOG_RESOURCE_NETWORK_MBPS", 100),
            concurrent_connections: parse_u32("BEARDOG_MAX_CONCURRENT_CONNECTIONS", 1000),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_limits_default() {
        let limits = ResourceLimits::default();

        assert_eq!(limits.memory_mb, 1024);
        assert_eq!(limits.cpu_percent, 50);
        assert_eq!(limits.disk_mb, 5120);
        assert_eq!(limits.network_mbps, 100);
        assert_eq!(limits.concurrent_connections, 1000);
    }

    #[test]
    fn test_resource_limits_from_env_with_map() {
        let mut map = HashMap::new();
        map.insert("BEARDOG_RESOURCE_MEMORY_MB".to_string(), "2048".to_string());
        map.insert("BEARDOG_RESOURCE_DISK_MB".to_string(), "10240".to_string());
        map.insert(
            "BEARDOG_MAX_CONCURRENT_CONNECTIONS".to_string(),
            "5000".to_string(),
        );

        let limits = ResourceLimits::from_env_with(|k| map.get(k).cloned());
        assert_eq!(limits.memory_mb, 2048);
        assert_eq!(limits.disk_mb, 10240);
        assert_eq!(limits.concurrent_connections, 5000);
    }

    #[test]
    fn test_resource_limits_invalid_env_uses_default() {
        let mut map = HashMap::new();
        map.insert(
            "BEARDOG_RESOURCE_MEMORY_MB".to_string(),
            "invalid".to_string(),
        );

        let limits = ResourceLimits::from_env_with(|k| map.get(k).cloned());
        assert_eq!(limits.memory_mb, 1024);
    }

    #[test]
    fn test_spawn_purpose_variants() {
        // Ensure all variants exist and are serializable
        let purposes = vec![
            SpawnPurpose::TaskExecution,
            SpawnPurpose::SecurityMonitoring,
            SpawnPurpose::DataProcessing,
            SpawnPurpose::NetworkOptimization,
            SpawnPurpose::ResourceManagement,
            SpawnPurpose::Experimentation,
        ];

        for purpose in purposes {
            let serialized = serde_json::to_string(&purpose);
            assert!(serialized.is_ok(), "Should serialize spawn purpose");
        }
    }

    #[test]
    fn test_task_type_variants() {
        let tasks = vec![
            TaskType::Cryptographic,
            TaskType::NetworkingTask,
            TaskType::DataAnalysis,
            TaskType::SecurityAudit,
            TaskType::ResourceOptimization,
            TaskType::EcosystemMaintenance,
        ];

        for task in tasks {
            let serialized = serde_json::to_string(&task);
            assert!(serialized.is_ok(), "Should serialize task type");
        }
    }

    #[test]
    fn test_spawn_status_variants() {
        let statuses = vec![
            SpawnStatus::Initializing,
            SpawnStatus::Active,
            SpawnStatus::Idle,
            SpawnStatus::Terminating,
            SpawnStatus::Terminated,
            SpawnStatus::Failed,
        ];

        for status in statuses {
            let serialized = serde_json::to_string(&status);
            assert!(serialized.is_ok(), "Should serialize spawn status");
        }
    }

    #[test]
    fn test_spawned_beardog_creation() {
        use chrono::Utc;
        use std::collections::HashMap;

        let spawn = SpawnedBearDog {
            spawn_id: "test-spawn".to_string(),
            parent_id: "parent-1".to_string(),
            genetics: BearDogGenetics::default(),
            spawn_purpose: SpawnPurpose::TaskExecution,
            task_assignment: vec![TaskType::Cryptographic],
            resource_limits: ResourceLimits::default(),
            spawn_time: Utc::now(),
            expected_lifetime: None,
            current_status: SpawnStatus::Initializing,
            performance_metrics: HashMap::new(),
            trust_relationships: HashMap::new(),
            consensus_participation: false,
            ecosystem_connections: vec![],
        };

        assert_eq!(spawn.spawn_id, "test-spawn");
        assert!(matches!(spawn.current_status, SpawnStatus::Initializing));
        assert_eq!(spawn.task_assignment.len(), 1);
    }

    #[test]
    fn test_spawn_request_creation() {
        let request = SpawnRequest {
            parent_genetics: vec![BearDogGenetics::default()],
            required_capabilities: vec![NodeCapability::SecurityAnalysis],
            target_environment: "production".to_string(),
        };

        assert_eq!(request.parent_genetics.len(), 1);
        assert_eq!(request.target_environment, "production");
        assert!(!request.required_capabilities.is_empty());
    }
}

/// Inputs required to evaluate whether a new child primal may be spawned.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnRequest {
    /// Collection of parent genetics
    pub parent_genetics: Vec<BearDogGenetics>,
    /// Collection of required capabilities
    pub required_capabilities: Vec<NodeCapability>,
    /// The target environment value
    pub target_environment: String,
}

/// Business or technical reason driving a spawn request (feeds policy/quotas).
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

/// Unit of work assigned to a spawned BearDog (drives scheduling and sandboxing).
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// High-level lifecycle flag for a [`SpawnedBearDog`].
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
