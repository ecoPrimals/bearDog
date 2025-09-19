// Health Status Types and Configurations
//
// This module provides health status types and configurations for the BearDog ecosystem.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Overall system health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HealthStatus {
    /// All systems operational
    Healthy,
    /// Some degradation but functional
    Degraded,
    /// System has issues but partially functional
    Unhealthy,
    /// System is not functional
    Critical,
    /// Status unknown or unreachable
    Unknown,
}

/// Health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    /// Name
    /// Name of the item
    pub name: String,
    /// Status
    /// Current status of the component
    pub status: HealthStatus,
    /// Message
    /// Optional message
    pub message: Option<String>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Duration Ms
    /// Number of `duration_ms`
    pub duration_ms: u64,
    /// Details
    /// Mapping of details
    pub details: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    /// Service Name
    /// Name of the service
    pub service_name: String,
    /// Status
    /// Current status of the component
    pub status: HealthStatus,
    /// Checks
    /// Collection of checks
    pub checks: Vec<HealthCheck>,
    /// Last Updated
    /// The last updated value
    pub last_updated: DateTime<Utc>,
    /// Uptime Seconds
    pub uptime_seconds: u64,
    /// Version
    /// The version value
    pub version: String,
}

/// System-wide health report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealthReport {
    /// Overall Status
    /// Current status of the overall
    pub overall_status: HealthStatus,
    /// Services
    /// Collection of services
    pub services: Vec<ServiceHealth>,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
    /// Summary
    /// The summary value
    pub summary: HealthSummary,
}

/// Health summary statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthSummary {
    /// Total Services
    /// Number of `total_services`
    pub total_services: usize,
    /// Healthy Services
    /// Number of `healthy_services`
    pub healthy_services: usize,
    /// Degraded Services
    /// Number of `degraded_services`
    pub degraded_services: usize,
    /// Unhealthy Services
    /// Number of `unhealthy_services`
    pub unhealthy_services: usize,
    /// Critical Services
    /// Number of `critical_services`
    pub critical_services: usize,
    /// Unknown Services
    /// Number of `unknown_services`
    pub unknown_services: usize,
}

impl Default for HealthStatus {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for ServiceHealth {
    fn default() -> Self {
        Self {
            service_name: "unknown".to_string(),
            status: HealthStatus::Unknown,
            checks: vec![],
            last_updated: Utc::now(),
            uptime_seconds: 0,
            version: "0.0.0".to_string(),
        }
    }
}

impl HealthStatus {
    /// Check if the health status indicates operational capability
    #[must_use]
    pub const fn is_operational(&self) -> bool {
        matches!(self, Self::Healthy | Self::Degraded)
    }

    /// Check if the health status is critical
    #[must_use]
    pub const fn is_critical(&self) -> bool {
        matches!(self, Self::Critical | Self::Unhealthy)
    }

    /// Get numeric health score (0-100)
    #[must_use]
    pub const fn score(&self) -> u8 {
        match self {
            Self::Healthy => 100,
            Self::Degraded => 75,
            Self::Unhealthy => 50,
            Self::Critical => 25,
            Self::Unknown => 0,
        }
    }
}

// Re-export canonical health types when available
// Note: Using unified canonical monitoring for health functionality
pub use crate::canonical::monitoring_unified::*;
