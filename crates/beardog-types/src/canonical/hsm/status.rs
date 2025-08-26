

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::capabilities::{
    AdvancedFeatureCapabilities, ApiSupportCapabilities, KeyGenerationCapabilities,
    KeyManagementCapabilities, SecurityCapabilities,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHealth {

    pub status: HsmHealthStatus,

    pub last_check: DateTime<Utc>,

    pub details: HashMap<String, serde_json::Value>,

    pub performance: HealthMetrics,

    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[derive(Default)]
pub enum HsmHealthStatus {

    Healthy,

    Degraded,

    Unhealthy,

    #[default]
    Unknown,
}

pub struct HsmStatus {

    pub hsm_id: String,

    pub health: HsmHealth,

    pub connected: bool,

    pub last_updated: DateTime<Utc>,

    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {

    pub ops_per_second: f64,

    pub avg_response_time_ms: f64,

    pub error_rate_percent: f64,

    pub memory_usage_percent: f64,

    pub cpu_usage_percent: f64,

    pub active_connections: u32,
}

pub struct HsmOperationResult<T> {

    pub success: bool,

    pub data: Option<T>,

    pub error: Option<HsmError>,

    pub duration_ms: u64,

    pub timestamp: DateTime<Utc>,
}

pub struct HsmError {

    pub code: String,

    pub message: String,

    pub category: ErrorCategory,

    pub recovery_actions: Vec<String>,
}

pub enum ErrorCategory {

    Connection,

    Authentication,

    KeyManagement,

    Cryptographic,

    Configuration,

    Hardware,

    Software,

    Unknown,
}

pub struct HealthCheck {

    pub check_type: HealthCheckType,

    pub config: HealthCheckConfig,

    pub last_result: Option<HealthCheckResult>,
}

pub enum HealthCheckType {

    Connectivity,

    KeyOperation,

    Performance,

    Security,

    Memory,

    Diagnostic,
}

pub struct HealthCheckConfig {

    pub interval_seconds: u64,

    pub timeout_seconds: u64,

    pub retry_count: u32,

    pub thresholds: HealthThresholds,
}

pub struct HealthCheckResult {

    pub warnings: Vec<String>,
}

pub struct HealthThresholds {

    pub degraded_response_time_ms: u64,

    pub unhealthy_response_time_ms: u64,

    pub degraded_error_rate_percent: f64,

    pub unhealthy_error_rate_percent: f64,
}

impl Default for HealthMetrics {
    fn default() -> Self {
        Self {
            ops_per_second: 0.0,
            avg_response_time_ms: 0.0,
            error_rate_percent: 0.0,
            memory_usage_percent: 0.0,
            cpu_usage_percent: 0.0,
            active_connections: 0,
        }
    }
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            interval_seconds: 30,
            timeout_seconds: 10,
            retry_count: 3,
            thresholds: HealthThresholds::default(),
        }
    }
}

impl Default for HealthThresholds {
    fn default() -> Self {
        Self {
            degraded_response_time_ms: 1000,
            unhealthy_response_time_ms: 5000,
            degraded_error_rate_percent: 5.0,
            unhealthy_error_rate_percent: 15.0,
        }
    }
}
impl<T> Default for HsmOperationResult<T> {
    fn default() -> Self {
        Self {
            success: false,
            data: None,
            error: None,
            duration_ms: 0,
            timestamp: Utc::now(),
        }
    }
}

pub enum HsmTier {

    Mobile,

    Cloud,

    Hybrid,
}

pub struct HsmCapabilities {

    pub vendor: String,

    pub model: String,

    pub firmware_version: String,

    pub supported_algorithms: Vec<String>,

    pub supported_key_types: Vec<String>,

    pub max_keys: Option<u32>,

    pub supported_operations: Vec<String>,

    pub security_features: Vec<String>,

    pub performance_metrics: HashMap<String, String>,

    pub certifications: Vec<String>,

    pub key_generation: KeyGenerationCapabilities,

    pub key_management: KeyManagementCapabilities,

    pub advanced_features: AdvancedFeatureCapabilities,

    pub api_support: ApiSupportCapabilities,

    pub security: SecurityCapabilities,

    pub human_entropy: crate::canonical::capabilities::HumanEntropyCapabilities,

    pub performance: crate::canonical::capabilities::PerformanceCapabilities,

    pub compliance: crate::canonical::capabilities::ComplianceCapabilities,

    pub vendor_capabilities: HashMap<String, serde_json::Value>,

    pub custom_capabilities: HashMap<String, serde_json::Value>,
}
