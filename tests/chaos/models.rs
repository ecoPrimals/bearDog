

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosTestConfig {

    pub max_fault_duration_ms: u64,

    pub fault_injection_rate: f32,

    pub recovery_timeout_ms: u64,

    pub max_concurrent_faults: u32,

    pub metrics_interval_ms: u64,

    pub enable_byzantine_faults: bool,

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

#[derive(Debug, Clone)]
pub struct ActiveFault {
    pub id: String,
    pub fault_type: FaultType,
    pub start_time: Instant,
    pub duration: Duration,
    pub target_component: String,
    pub severity: FaultSeverity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FaultType {

    NetworkPartition { 

        segments: Vec<String>,

        duration_ms: u64,
    },
    NetworkLatency {

        latency_ms: u64,

        packet_loss: f32,
    },

    ComponentCrash {

        component: String,

        crash_type: CrashType,
    },
    ComponentSlowdown {

        component: String,

        slowdown_factor: f32,
    },

    MemoryExhaustion {

        memory_mb: u64,

        cause_oom: bool,
    },
    CpuExhaustion {

        cpu_percent: u32,

        thread_count: u32,
    },
    DiskExhaustion {

        disk_mb: u64,

        filesystem: String,
    },

    DatabaseTimeout {

        timeout_ms: u64,
    },
    DatabaseCorruption {

        tables: Vec<String>,

        corruption_type: CorruptionType,
    },

    AuthenticationFailure {

        failure_rate: f32,
    },
    CertificateExpiry {

        certificates: Vec<String>,
    },

    ByzantineBehavior {

        behavior_type: ByzantineType,

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

    ConflictingMessages,

    SelectiveIgnore,

    DelayedResponses,

    MalformedData,

    SplitBrain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FaultSeverity {
    Low,
    Medium,
    High,
    Critical,
}

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