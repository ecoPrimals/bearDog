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


//! Chaos Testing Models and Data Structures
//!
//! All data structures, configuration types, enums, and result models
//! for the chaos testing framework.

use serde::{Deserialize, Serialize};
use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicBool, AtomicU64},
        Arc, Mutex,
    },
    time::{Duration, Instant, SystemTime},
};
use tokio::sync::RwLock;
use std::collections::HashMap;

/// Chaos testing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosTestConfig {
    /// Maximum fault duration in milliseconds
    pub max_fault_duration_ms: u64,
    
    /// Fault injection rate (0.0 to 1.0)
    pub fault_injection_rate: f32,
    
    /// Recovery timeout in milliseconds
    pub recovery_timeout_ms: u64,
    
    /// Concurrent fault limit
    pub max_concurrent_faults: u32,
    
    /// Metrics collection interval
    pub metrics_interval_ms: u64,
    
    /// Enable Byzantine fault testing
    pub enable_byzantine_faults: bool,
    
    /// System degradation thresholds
    pub degradation_thresholds: DegradationThresholds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DegradationThresholds {
    pub response_time_multiplier: f64,
    pub error_rate_threshold: f64,
    pub memory_usage_threshold: f64,
    pub cpu_usage_threshold: f64,
}

impl Default for ChaosTestConfig {
    fn default() -> Self {
        Self {
            max_fault_duration_ms: 30_000,
            fault_injection_rate: 0.1,
            recovery_timeout_ms: 60_000,
            max_concurrent_faults: 3,
            metrics_interval_ms: 1_000,
            enable_byzantine_faults: true,
            degradation_thresholds: DegradationThresholds {
                response_time_multiplier: 3.0,
                error_rate_threshold: 0.05,
                memory_usage_threshold: 0.8,
                cpu_usage_threshold: 0.9,
            },
        }
    }
}

/// Active fault tracking
#[derive(Debug, Clone)]
pub struct ActiveFault {
    pub id: String,
    pub fault_type: FaultType,
    pub start_time: Instant,
    pub duration: Duration,
    pub target_component: String,
    pub severity: FaultSeverity,
}

/// Types of faults that can be injected
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FaultType {
    /// Network-related faults
    NetworkPartition { 
        /// Affected network segments
        segments: Vec<String>,
        /// Partition duration
        duration_ms: u64,
    },
    NetworkLatency {
        /// Added latency in milliseconds
        latency_ms: u64,
        /// Packet loss percentage
        packet_loss: f32,
    },
    
    /// Component failure faults
    ComponentCrash {
        /// Component identifier
        component: String,
        /// Crash type (graceful, immediate, corrupt)
        crash_type: CrashType,
    },
    ComponentSlowdown {
        /// Component identifier  
        component: String,
        /// Slowdown multiplier
        slowdown_factor: f32,
    },
    
    /// Resource exhaustion faults
    MemoryExhaustion {
        /// Memory to allocate in MB
        memory_mb: u64,
        /// Whether to cause OOM
        cause_oom: bool,
    },
    CpuExhaustion {
        /// CPU utilization percentage
        cpu_percent: u32,
        /// Number of threads to spawn
        thread_count: u32,
    },
    DiskExhaustion {
        /// Disk space to fill in MB
        disk_mb: u64,
        /// Target filesystem
        filesystem: String,
    },
    
    /// Database and storage faults
    DatabaseTimeout {
        /// Query timeout in milliseconds
        timeout_ms: u64,
    },
    DatabaseCorruption {
        /// Tables to corrupt
        tables: Vec<String>,
        /// Corruption type
        corruption_type: CorruptionType,
    },
    
    /// Security-related faults
    AuthenticationFailure {
        /// Failure rate (0.0 to 1.0)
        failure_rate: f32,
    },
    CertificateExpiry {
        /// Certificates to expire
        certificates: Vec<String>,
    },
    
    /// Byzantine faults
    ByzantineBehavior {
        /// Type of Byzantine behavior
        behavior_type: ByzantineType,
        /// Affected nodes
        nodes: Vec<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CrashType {
    Graceful,
    Immediate,
    MemoryCorrupt,
    InfiniteLoop,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CorruptionType {
    RandomBytes,
    ZeroBytes,
    DuplicateRecords,
    MissingRecords,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ByzantineType {
    /// Send conflicting information
    ConflictingMessages,
    /// Selectively ignore messages
    SelectiveIgnore,
    /// Send delayed responses
    DelayedResponses,
    /// Send malformed data
    MalformedData,
    /// Act as a split-brain
    SplitBrain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FaultSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Fault event for historical tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaultEvent {
    pub id: String,
    pub fault_type: FaultType,
    pub start_time: SystemTime,
    pub end_time: Option<SystemTime>,
    pub target_component: String,
    pub severity: FaultSeverity,
    pub recovery_time_ms: Option<u64>,
    pub system_impact: SystemImpact,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemImpact {
    pub response_time_increase: f64,
    pub error_rate_increase: f64,
    pub throughput_decrease: f64,
    pub memory_usage_increase: f64,
    pub availability_decrease: f64,
}

impl Default for SystemImpact {
    fn default() -> Self {
        Self {
            response_time_increase: 1.0,
            error_rate_increase: 0.0,
            throughput_decrease: 0.0,
            memory_usage_increase: 0.0,
            availability_decrease: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryStatus {
    FullyRecovered,
    PartiallyRecovered,
    NotRecovered,
    UnknownState,
}

/// Chaos testing scenario
#[derive(Debug, Clone)]
pub struct ChaosScenario {
    pub name: String,
    pub description: String,
    pub faults: Vec<FaultType>,
    pub duration_ms: u64,
    pub success_criteria: SuccessCriteria,
}

#[derive(Debug, Clone)]
pub struct SuccessCriteria {
    pub max_recovery_time_ms: u64,
    pub max_error_rate: f64,
    pub min_availability: f64,
    pub max_response_time_degradation: f64,
}

/// Metrics collector for chaos testing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChaosMetrics {
    pub total_faults_injected: u64,
    pub successful_recoveries: u64,
    pub failed_recoveries: u64,
    pub average_recovery_time_ms: f64,
    pub peak_error_rate: f64,
    pub min_availability: f64,
    pub response_time_degradation: f64,
    pub throughput_impact: f64,
    pub memory_usage_peak: f64,
    pub cpu_usage_peak: f64,
    pub system_resilience_score: f64,
}

// Result types

#[derive(Debug, Clone)]
pub struct FaultResult {
    pub fault_id: String,
    pub fault_type: FaultType,
    pub target_component: String,
    pub injection_success: bool,
    pub impact_metrics: SystemImpact,
    pub recovery_time_ms: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct RecoveryResult {
    pub component: String,
    pub status: RecoveryStatus,
}

#[derive(Debug, Clone)]
pub struct ScenarioResult {
    pub scenario_name: String,
    pub fault_results: Vec<FaultResult>,
    pub recovery_results: Vec<RecoveryResult>,
    pub baseline_metrics: SystemImpact,
    pub post_chaos_metrics: SystemImpact,
    pub success: bool,
    pub duration_ms: u64,
}

#[derive(Debug, Clone)]
pub struct ChaosTestReport {
    pub scenario_results: Vec<ScenarioResult>,
    pub overall_resilience_score: f64,
    pub metrics: ChaosMetrics,
    pub recommendations: Vec<String>,
} 