// SPDX-License-Identifier: AGPL-3.0-or-later

// Health Status Types and Configurations
//
// This module provides health status types and configurations for the BearDog ecosystem.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Overall system health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
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
    #[default]
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
    pub details: BTreeMap<String, String>,
}

/// Aggregated health for one logical service and its constituent checks.
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

#[cfg(test)]
mod tests {
    use super::*;

    // HealthStatus tests
    #[test]
    fn test_health_status_variants() {
        let statuses = [
            HealthStatus::Healthy,
            HealthStatus::Degraded,
            HealthStatus::Unhealthy,
            HealthStatus::Critical,
            HealthStatus::Unknown,
        ];

        assert_eq!(statuses.len(), 5);
    }

    #[test]
    fn test_health_status_default() {
        let status = HealthStatus::default();
        assert_eq!(status, HealthStatus::Unknown);
    }

    #[test]
    fn test_health_status_is_operational() {
        assert!(HealthStatus::Healthy.is_operational());
        assert!(HealthStatus::Degraded.is_operational());
        assert!(!HealthStatus::Unhealthy.is_operational());
        assert!(!HealthStatus::Critical.is_operational());
        assert!(!HealthStatus::Unknown.is_operational());
    }

    #[test]
    fn test_health_status_is_critical() {
        assert!(!HealthStatus::Healthy.is_critical());
        assert!(!HealthStatus::Degraded.is_critical());
        assert!(HealthStatus::Unhealthy.is_critical());
        assert!(HealthStatus::Critical.is_critical());
        assert!(!HealthStatus::Unknown.is_critical());
    }

    #[test]
    fn test_health_status_score() {
        assert_eq!(HealthStatus::Healthy.score(), 100);
        assert_eq!(HealthStatus::Degraded.score(), 75);
        assert_eq!(HealthStatus::Unhealthy.score(), 50);
        assert_eq!(HealthStatus::Critical.score(), 25);
        assert_eq!(HealthStatus::Unknown.score(), 0);
    }

    #[test]
    fn test_health_status_score_ordering() {
        assert!(HealthStatus::Healthy.score() > HealthStatus::Degraded.score());
        assert!(HealthStatus::Degraded.score() > HealthStatus::Unhealthy.score());
        assert!(HealthStatus::Unhealthy.score() > HealthStatus::Critical.score());
        assert!(HealthStatus::Critical.score() > HealthStatus::Unknown.score());
    }

    // HealthCheck tests
    #[test]
    fn test_health_check_creation() {
        let check = HealthCheck {
            name: "database".to_string(),
            status: HealthStatus::Healthy,
            message: Some("All connections active".to_string()),
            timestamp: Utc::now(),
            duration_ms: 45,
            details: BTreeMap::new(),
        };

        assert_eq!(check.name, "database");
        assert_eq!(check.status, HealthStatus::Healthy);
        assert!(check.message.is_some());
        assert_eq!(check.duration_ms, 45);
    }

    #[test]
    fn test_health_check_with_details() {
        let mut details = BTreeMap::new();
        details.insert("connection_pool".to_string(), "10/10".to_string());
        details.insert("response_time_ms".to_string(), "12".to_string());

        let check = HealthCheck {
            name: "api".to_string(),
            status: HealthStatus::Healthy,
            message: None,
            timestamp: Utc::now(),
            duration_ms: 12,
            details: details.clone(),
        };

        assert_eq!(check.details.len(), 2);
        assert_eq!(
            check.details.get("connection_pool"),
            Some(&"10/10".to_string())
        );
    }

    // ServiceHealth tests
    #[test]
    fn test_service_health_default() {
        let service = ServiceHealth::default();

        assert_eq!(service.service_name, "unknown");
        assert_eq!(service.status, HealthStatus::Unknown);
        assert!(service.checks.is_empty());
        assert_eq!(service.uptime_seconds, 0);
        assert_eq!(service.version, "0.0.0");
    }

    #[test]
    fn test_service_health_creation() {
        let service = ServiceHealth {
            service_name: "auth-service".to_string(),
            status: HealthStatus::Healthy,
            checks: vec![],
            last_updated: Utc::now(),
            uptime_seconds: 3600,
            version: "1.2.3".to_string(),
        };

        assert_eq!(service.service_name, "auth-service");
        assert_eq!(service.uptime_seconds, 3600);
        assert_eq!(service.version, "1.2.3");
    }

    #[test]
    fn test_service_health_with_checks() {
        let check1 = HealthCheck {
            name: "check1".to_string(),
            status: HealthStatus::Healthy,
            message: None,
            timestamp: Utc::now(),
            duration_ms: 10,
            details: BTreeMap::new(),
        };

        let check2 = HealthCheck {
            name: "check2".to_string(),
            status: HealthStatus::Degraded,
            message: Some("Slow response".to_string()),
            timestamp: Utc::now(),
            duration_ms: 150,
            details: BTreeMap::new(),
        };

        let service = ServiceHealth {
            service_name: "data-service".to_string(),
            status: HealthStatus::Degraded,
            checks: vec![check1, check2],
            last_updated: Utc::now(),
            uptime_seconds: 7200,
            version: "2.0.0".to_string(),
        };

        assert_eq!(service.checks.len(), 2);
        assert_eq!(service.status, HealthStatus::Degraded);
    }

    // HealthSummary tests
    #[test]
    fn test_health_summary_all_healthy() {
        let summary = HealthSummary {
            total_services: 5,
            healthy_services: 5,
            degraded_services: 0,
            unhealthy_services: 0,
            critical_services: 0,
            unknown_services: 0,
        };

        assert_eq!(summary.total_services, 5);
        assert_eq!(summary.healthy_services, 5);
        assert_eq!(
            summary.healthy_services
                + summary.degraded_services
                + summary.unhealthy_services
                + summary.critical_services
                + summary.unknown_services,
            5
        );
    }

    #[test]
    fn test_health_summary_mixed() {
        let summary = HealthSummary {
            total_services: 10,
            healthy_services: 6,
            degraded_services: 2,
            unhealthy_services: 1,
            critical_services: 1,
            unknown_services: 0,
        };

        assert_eq!(summary.total_services, 10);
        let operational = summary.healthy_services + summary.degraded_services;
        assert_eq!(operational, 8);
    }

    // SystemHealthReport tests
    #[test]
    fn test_system_health_report_creation() {
        let summary = HealthSummary {
            total_services: 3,
            healthy_services: 3,
            degraded_services: 0,
            unhealthy_services: 0,
            critical_services: 0,
            unknown_services: 0,
        };

        let report = SystemHealthReport {
            overall_status: HealthStatus::Healthy,
            services: vec![],
            timestamp: Utc::now(),
            summary,
        };

        assert_eq!(report.overall_status, HealthStatus::Healthy);
        assert_eq!(report.summary.total_services, 3);
    }

    #[test]
    fn test_system_health_report_with_services() {
        let service1 = ServiceHealth {
            service_name: "service1".to_string(),
            status: HealthStatus::Healthy,
            checks: vec![],
            last_updated: Utc::now(),
            uptime_seconds: 100,
            version: "1.0.0".to_string(),
        };

        let service2 = ServiceHealth {
            service_name: "service2".to_string(),
            status: HealthStatus::Degraded,
            checks: vec![],
            last_updated: Utc::now(),
            uptime_seconds: 200,
            version: "1.0.0".to_string(),
        };

        let summary = HealthSummary {
            total_services: 2,
            healthy_services: 1,
            degraded_services: 1,
            unhealthy_services: 0,
            critical_services: 0,
            unknown_services: 0,
        };

        let report = SystemHealthReport {
            overall_status: HealthStatus::Degraded,
            services: vec![service1, service2],
            timestamp: Utc::now(),
            summary,
        };

        assert_eq!(report.services.len(), 2);
        assert_eq!(report.overall_status, HealthStatus::Degraded);
    }

    #[test]
    fn test_health_check_serialization() {
        let check = HealthCheck {
            name: "test".to_string(),
            status: HealthStatus::Healthy,
            message: Some("OK".to_string()),
            timestamp: Utc::now(),
            duration_ms: 5,
            details: BTreeMap::new(),
        };

        let json = serde_json::to_string(&check);
        assert!(json.is_ok(), "Should be able to serialize HealthCheck");
    }

    #[test]
    fn test_service_health_serialization() {
        let service = ServiceHealth::default();
        let json = serde_json::to_string(&service);
        assert!(json.is_ok(), "Should be able to serialize ServiceHealth");
    }
}
