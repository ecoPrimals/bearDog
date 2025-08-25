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


/// # Canonical HSM Status
///
/// **SINGLE SOURCE OF TRUTH** for all HSM status and health-related types.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// use std::time::Duration; // Currently unused
use super::capabilities::{
    AdvancedFeatureCapabilities, ApiSupportCapabilities, KeyGenerationCapabilities,
    KeyManagementCapabilities, SecurityCapabilities,
};

/// **HSM HEALTH** - Overall HSM health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmHealth {
    /// Overall health status
    pub status: HsmHealthStatus,
    /// Last health check timestamp
    pub last_check: DateTime<Utc>,
    /// Health check details
    pub details: HashMap<String, serde_json::Value>,
    /// Performance metrics
    pub performance: HealthMetrics,
    /// Error information (if unhealthy)
    pub errors: Vec<String>,
}
/// **HSM HEALTH STATUS** - Health status enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[derive(Default)]
pub enum HsmHealthStatus {
    /// HSM is healthy and operational
    Healthy,
    /// HSM has minor issues but is functional
    Degraded,
    /// HSM has critical issues
    Unhealthy,
    /// HSM status cannot be determined
    #[default]
    Unknown,
}

/// **HSM STATUS** - Complete HSM status information


pub struct HsmStatus {
    /// HSM identifier
    pub hsm_id: String,
    /// Current health status
    pub health: HsmHealth,
    /// Connection status
    pub connected: bool,
    /// Last update timestamp
    pub last_updated: DateTime<Utc>,
    /// Additional status metadata
    pub metadata: HashMap<String, String>,
}

/// **HEALTH METRICS** - Performance and operational metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthMetrics {
    /// Operations per second
    pub ops_per_second: f64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Error rate percentage
    pub error_rate_percent: f64,
    /// Memory usage percentage
    pub memory_usage_percent: f64,
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Active connections
    pub active_connections: u32,
}

/// **HSM OPERATION RESULT** - Result of HSM operations
pub struct HsmOperationResult<T> {
    /// Operation success status
    pub success: bool,
    /// Result data (if successful)
    pub data: Option<T>,
    /// Error information (if failed)
    pub error: Option<HsmError>,
    /// Operation duration in milliseconds
    pub duration_ms: u64,
    /// Timestamp of operation
    pub timestamp: DateTime<Utc>,
}

/// **HSM ERROR** - HSM-specific error information
pub struct HsmError {
    /// Error code
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Error category
    pub category: ErrorCategory,
    /// Additional error details
    /// Suggested recovery actions
    pub recovery_actions: Vec<String>,
}

/// **ERROR CATEGORY** - Categories of HSM errors
pub enum ErrorCategory {
    /// Connection or communication errors
    Connection,
    /// Authentication or authorization errors
    Authentication,
    /// Key management errors
    KeyManagement,
    /// Cryptographic operation errors
    Cryptographic,
    /// Configuration errors
    Configuration,
    /// Hardware errors
    Hardware,
    /// Software errors
    Software,
    /// Unknown error category
    Unknown,
}

/// **HEALTH CHECK** - Health check configuration and results


pub struct HealthCheck {
    /// Type of health check
    pub check_type: HealthCheckType,
    /// Check configuration
    pub config: HealthCheckConfig,
    /// Last check result
    pub last_result: Option<HealthCheckResult>,
}

/// **HEALTH CHECK TYPE** - Types of health checks
pub enum HealthCheckType {
    /// Basic connectivity check
    Connectivity,
    /// Key operation test
    KeyOperation,
    /// Performance benchmark
    Performance,
    /// Security validation
    Security,
    /// Memory usage check
    Memory,
    /// Full diagnostic
    Diagnostic,
}

/// **HEALTH CHECK CONFIG** - Configuration for health checks


pub struct HealthCheckConfig {
    /// Check interval in seconds
    pub interval_seconds: u64,
    /// Timeout for check in seconds
    pub timeout_seconds: u64,
    /// Number of retries on failure
    pub retry_count: u32,
    /// Thresholds for health determination
    pub thresholds: HealthThresholds,
}

/// **HEALTH CHECK RESULT** - Result of a health check
pub struct HealthCheckResult {
    /// Check status
    /// Check timestamp
    /// Check duration in milliseconds
    /// Detailed results
    /// Any warnings or notes
    pub warnings: Vec<String>,
}

/// **HEALTH THRESHOLDS** - Thresholds for determining health status
pub struct HealthThresholds {
    /// Response time threshold for degraded status (ms)
    pub degraded_response_time_ms: u64,
    /// Response time threshold for unhealthy status (ms)
    pub unhealthy_response_time_ms: u64,
    /// Error rate threshold for degraded status (%)
    pub degraded_error_rate_percent: f64,
    /// Error rate threshold for unhealthy status (%)
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
    /// Software-based HSM
    /// Hardware Security Module
    /// Mobile HSM (Android StrongBox, iOS Secure Enclave)
    Mobile,
    /// Cloud HSM service
    Cloud,
    /// Hybrid configuration
    Hybrid,
}


pub struct HsmCapabilities {
    /// HSM vendor/manufacturer
    pub vendor: String,
    /// HSM model/product name
    pub model: String,
    /// Firmware version
    pub firmware_version: String,
    /// Supported cryptographic algorithms
    pub supported_algorithms: Vec<String>,
    /// Supported key types (simplified for compatibility)
    pub supported_key_types: Vec<String>,
    /// Maximum number of keys that can be stored
    pub max_keys: Option<u32>,
    /// Supported key operations
    pub supported_operations: Vec<String>,
    /// Hardware security features
    pub security_features: Vec<String>,
    /// Performance characteristics
    pub performance_metrics: HashMap<String, String>,
    /// Compliance certifications
    pub certifications: Vec<String>,
    /// Key generation capabilities
    pub key_generation: KeyGenerationCapabilities,
    /// Key management capabilities
    pub key_management: KeyManagementCapabilities,
    /// Advanced feature capabilities
    pub advanced_features: AdvancedFeatureCapabilities,
    /// API support capabilities
    pub api_support: ApiSupportCapabilities,
    /// Security capabilities
    pub security: SecurityCapabilities,
    // === UNIVERSAL EXTENSIONS ===
    /// Human entropy collection capabilities (NEW - universal support)
    pub human_entropy: crate::canonical::capabilities::HumanEntropyCapabilities,
    /// Performance capabilities (enhanced universal metrics)
    pub performance: crate::canonical::capabilities::PerformanceCapabilities,
    /// Compliance capabilities (enhanced universal compliance)
    pub compliance: crate::canonical::capabilities::ComplianceCapabilities,
    /// Vendor-specific capabilities (extensible for any HSM vendor)
    pub vendor_capabilities: HashMap<String, serde_json::Value>,
    /// Custom capabilities (fully extensible for future needs)
    pub custom_capabilities: HashMap<String, serde_json::Value>,
}
