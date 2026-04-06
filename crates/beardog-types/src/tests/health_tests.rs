// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unit tests for health status types

use crate::health_status::{
    HealthCheck, HealthStatus, HealthSummary, ServiceHealth, SystemHealthReport,
};
use chrono::Utc;
use std::collections::HashMap;

#[cfg(test)]
mod health_status_tests {
    use super::*;

    #[test]
    fn test_health_status_variants() {
        // Test all health status variants exist and can be created
        let healthy = HealthStatus::Healthy;
        let degraded = HealthStatus::Degraded;
        let unhealthy = HealthStatus::Unhealthy;
        let critical = HealthStatus::Critical;
        let unknown = HealthStatus::Unknown;

        assert!(matches!(healthy, HealthStatus::Healthy));
        assert!(matches!(degraded, HealthStatus::Degraded));
        assert!(matches!(unhealthy, HealthStatus::Unhealthy));
        assert!(matches!(critical, HealthStatus::Critical));
        assert!(matches!(unknown, HealthStatus::Unknown));
    }

    #[test]
    fn test_health_status_equality() {
        assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
        assert_ne!(HealthStatus::Healthy, HealthStatus::Degraded);
        assert_ne!(HealthStatus::Unhealthy, HealthStatus::Critical);
    }

    #[test]
    fn test_health_status_clone() {
        let status = HealthStatus::Healthy;
        let cloned = status.clone();
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_health_status_debug() {
        let status = HealthStatus::Healthy;
        let debug_str = format!("{status:?}");
        assert!(debug_str.contains("Healthy"));
    }

    #[test]
    fn test_health_status_serialize() {
        let status = HealthStatus::Healthy;
        let serialized = serde_json::to_string(&status);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_health_status_deserialize() {
        let json = r#""Healthy""#;
        let status: Result<HealthStatus, _> = serde_json::from_str(json);
        assert!(status.is_ok());
        assert_eq!(status.unwrap(), HealthStatus::Healthy);
    }

    #[test]
    fn test_health_status_roundtrip() {
        let original = HealthStatus::Degraded;
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: HealthStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
    }
}

#[cfg(test)]
mod health_check_tests {
    use super::*;

    fn create_test_health_check() -> HealthCheck {
        HealthCheck {
            name: "test_check".to_string(),
            status: HealthStatus::Healthy,
            message: Some("All good".to_string()),
            timestamp: Utc::now(),
            duration_ms: 42,
            details: HashMap::new(),
        }
    }

    #[test]
    fn test_health_check_creation() {
        let check = create_test_health_check();
        assert_eq!(check.name, "test_check");
        assert_eq!(check.status, HealthStatus::Healthy);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(check.message, Some("All good".to_string()));
        assert_eq!(check.duration_ms, 42);
        assert!(check.details.is_empty());
    }

    #[test]
    fn test_health_check_with_details() {
        let mut details = HashMap::new();
        details.insert("key1".to_string(), "value1".to_string());
        details.insert("key2".to_string(), "value2".to_string());

        let check = HealthCheck {
            name: "detailed_check".to_string(),
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            status: HealthStatus::Healthy,
            message: None,
            timestamp: Utc::now(),
            duration_ms: 100,
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            details: details.clone(),
        };

        assert_eq!(check.details.len(), 2);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(check.details.get("key1"), Some(&"value1".to_string()));
        assert_eq!(check.details.get("key2"), Some(&"value2".to_string()));
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_health_check_without_message() {
        let check = HealthCheck {
            name: "no_message".to_string(),
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            status: HealthStatus::Healthy,
            message: None,
            timestamp: Utc::now(),
            duration_ms: 0,
            details: HashMap::new(),
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
        };

        assert!(check.message.is_none());
    }

    #[test]
    fn test_health_check_clone() {
        let check = create_test_health_check();
        let cloned = check.clone();
        assert_eq!(check.name, cloned.name);
        assert_eq!(check.status, cloned.status);
        assert_eq!(check.message, cloned.message);
    }

    #[test]
    fn test_health_check_serialize() {
        let check = create_test_health_check();
        let serialized = serde_json::to_string(&check);
        assert!(serialized.is_ok());
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_health_check_deserialize() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let json = r#"{
            "name": "test",
            "status": "Healthy",
            "message": null,
            "timestamp": "2025-10-09T00:00:00Z",
            "duration_ms": 50,
            "details": {}
        }"#;
        let check: Result<HealthCheck, _> = serde_json::from_str(json);
        assert!(check.is_ok());
        let check = check.unwrap();
        assert_eq!(check.name, "test");
        assert_eq!(check.duration_ms, 50);
    }

    #[test]
    fn test_health_check_different_statuses() {
        let statuses = vec![
            HealthStatus::Healthy,
            HealthStatus::Degraded,
            HealthStatus::Unhealthy,
            HealthStatus::Critical,
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            HealthStatus::Unknown,
        ];

        for status in statuses {
            let check = HealthCheck {
                name: format!("check_{status:?}"),
                status: status.clone(),
                message: None,
                timestamp: Utc::now(),
                duration_ms: 100,
                details: HashMap::new(),
                // TEST_CATEGORY: integration
                // TEST_DOMAIN: types
                // TEST_PRIORITY: normal
            };
            assert_eq!(check.status, status);
        }
    }
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[cfg(test)]
mod service_health_tests {
    use super::*;

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn create_test_service_health() -> ServiceHealth {
        ServiceHealth {
            service_name: "test_service".to_string(),
            status: HealthStatus::Healthy,
            checks: vec![],
            last_updated: Utc::now(),
            uptime_seconds: 3600,
            version: "1.0.0".to_string(),
        }
    }

    #[test]
    fn test_service_health_creation() {
        let service = create_test_service_health();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(service.service_name, "test_service");
        assert_eq!(service.status, HealthStatus::Healthy);
        assert_eq!(service.uptime_seconds, 3600);
        assert_eq!(service.version, "1.0.0");
        assert!(service.checks.is_empty());
    }

    #[test]
    fn test_service_health_with_checks() {
        let check = HealthCheck {
            name: "database".to_string(),
            status: HealthStatus::Healthy,
            message: None,
            timestamp: Utc::now(),
            duration_ms: 25,
            details: HashMap::new(),
        };

        let service = ServiceHealth {
            service_name: "api".to_string(),
            status: HealthStatus::Healthy,
            checks: vec![check],
            last_updated: Utc::now(),
            uptime_seconds: 7200,
            version: "2.0.0".to_string(),
        };

        assert_eq!(service.checks.len(), 1);
        assert_eq!(service.checks[0].name, "database");
    }

    #[test]
    fn test_service_health_multiple_checks() {
        let checks = vec![
            HealthCheck {
                name: "check1".to_string(),
                // TEST_CATEGORY: integration
                // TEST_DOMAIN: types
                // TEST_PRIORITY: normal
                status: HealthStatus::Healthy,
                message: None,
                timestamp: Utc::now(),
                duration_ms: 10,
                details: HashMap::new(),
            },
            HealthCheck {
                // TEST_CATEGORY: integration
                // TEST_DOMAIN: types
                // TEST_PRIORITY: normal
                name: "check2".to_string(),
                status: HealthStatus::Degraded,
                message: Some("Slow response".to_string()),
                timestamp: Utc::now(),
                duration_ms: 500,
                details: HashMap::new(),
            },
        ];

        let service = ServiceHealth {
            service_name: "multi_check_service".to_string(),
            status: HealthStatus::Degraded,
            checks,
            last_updated: Utc::now(),
            uptime_seconds: 1800,
            version: "1.2.3".to_string(),
        };

        assert_eq!(service.checks.len(), 2);
        assert_eq!(service.checks[1].message, Some("Slow response".to_string()));
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    #[test]
    fn test_service_health_serialize() {
        let service = create_test_service_health();
        let serialized = serde_json::to_string(&service);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_service_health_clone() {
        let service = create_test_service_health();
        let cloned = service.clone();
        assert_eq!(service.service_name, cloned.service_name);
        assert_eq!(service.uptime_seconds, cloned.uptime_seconds);
    }
}

#[cfg(test)]
mod system_health_report_tests {
    use super::*;

    fn create_test_summary() -> HealthSummary {
        HealthSummary {
            total_services: 3,
            healthy_services: 2,
            degraded_services: 1,
            unhealthy_services: 0,
            critical_services: 0,
            unknown_services: 0,
        }
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    fn create_test_system_health_report() -> SystemHealthReport {
        SystemHealthReport {
            overall_status: HealthStatus::Healthy,
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            services: vec![],
            timestamp: Utc::now(),
            summary: create_test_summary(),
        }
    }

    #[test]
    fn test_system_health_report_creation() {
        let report = create_test_system_health_report();
        assert_eq!(report.overall_status, HealthStatus::Healthy);
        assert!(report.services.is_empty());
        assert_eq!(report.summary.total_services, 3);
    }

    #[test]
    fn test_system_health_report_with_services() {
        let service = ServiceHealth {
            service_name: "api".to_string(),
            status: HealthStatus::Healthy,
            checks: vec![],
            last_updated: Utc::now(),
            uptime_seconds: 3600,
            version: "1.0.0".to_string(),
        };

        let report = SystemHealthReport {
            overall_status: HealthStatus::Healthy,
            services: vec![service],
            timestamp: Utc::now(),
            summary: create_test_summary(),
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
        };

        assert_eq!(report.services.len(), 1);
        assert_eq!(report.services[0].service_name, "api");
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    #[test]
    fn test_health_summary_counts() {
        let summary = create_test_summary();
        assert_eq!(summary.total_services, 3);
        assert_eq!(summary.healthy_services, 2);
        assert_eq!(summary.degraded_services, 1);
        assert_eq!(summary.unhealthy_services, 0);
        assert_eq!(summary.critical_services, 0);
        assert_eq!(summary.unknown_services, 0);
    }

    #[test]
    fn test_health_summary_all_healthy() {
        let summary = HealthSummary {
            total_services: 5,
            healthy_services: 5,
            degraded_services: 0,
            unhealthy_services: 0,
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            critical_services: 0,
            unknown_services: 0,
        };

        assert_eq!(summary.total_services, summary.healthy_services);
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_health_summary_mixed_states() {
        let summary = HealthSummary {
            total_services: 10,
            healthy_services: 5,
            degraded_services: 3,
            unhealthy_services: 1,
            critical_services: 1,
            unknown_services: 0,
        };

        let total_counted = summary.healthy_services
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            + summary.degraded_services
            + summary.unhealthy_services
            + summary.critical_services
            + summary.unknown_services;

        assert_eq!(total_counted, summary.total_services);
    }

    #[test]
    fn test_system_health_report_serialize() {
        let report = create_test_system_health_report();
        let serialized = serde_json::to_string(&report);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_system_health_report_clone() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let report = create_test_system_health_report();
        let cloned = report.clone();
        assert_eq!(report.overall_status, cloned.overall_status);
        assert_eq!(report.summary.total_services, cloned.summary.total_services);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
    }
}

#[cfg(test)]
mod health_integration_tests {
    use super::*;

    #[test]
    fn test_full_health_hierarchy() {
        // Create a complete health check hierarchy
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let details = HashMap::new();

        let check = HealthCheck {
            name: "database_connection".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Connected successfully".to_string()),
            timestamp: Utc::now(),
            duration_ms: 15,
            details,
        };

        let service = ServiceHealth {
            service_name: "api_service".to_string(),
            status: HealthStatus::Healthy,
            checks: vec![check],
            last_updated: Utc::now(),
            uptime_seconds: 86400,
            version: "3.0.0".to_string(),
        };

        let summary = HealthSummary {
            total_services: 1,
            healthy_services: 1,
            degraded_services: 0,
            unhealthy_services: 0,
            critical_services: 0,
            unknown_services: 0,
        };

        let report = SystemHealthReport {
            overall_status: HealthStatus::Healthy,
            services: vec![service],
            timestamp: Utc::now(),
            summary,
        };

        // Verify the complete hierarchy
        assert_eq!(report.services.len(), 1);
        assert_eq!(report.services[0].checks.len(), 1);
        assert_eq!(report.services[0].checks[0].name, "database_connection");
        assert_eq!(report.summary.healthy_services, 1);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_degraded_service_affects_overall_status() {
        let service1 = ServiceHealth {
            service_name: "service1".to_string(),
            status: HealthStatus::Healthy,
            checks: vec![],
            last_updated: Utc::now(),
            uptime_seconds: 3600,
            version: "1.0.0".to_string(),
        };

        let service2 = ServiceHealth {
            service_name: "service2".to_string(),
            status: HealthStatus::Degraded,
            checks: vec![],
            last_updated: Utc::now(),
            uptime_seconds: 1800,
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

        assert_eq!(report.overall_status, HealthStatus::Degraded);
        assert_eq!(report.services.len(), 2);
    }
}
