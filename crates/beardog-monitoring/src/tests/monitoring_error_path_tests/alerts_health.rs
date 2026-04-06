// SPDX-License-Identifier: AGPL-3.0-or-later

//! Error-path tests for health checks and alerting.

use super::common::*;

#[test]
fn test_health_check_unreachable_service() {
    let checker = HealthCheckService::new();
    let result = checker.check_service("unreachable-service");

    assert!(result.is_err(), "Should fail for unreachable service");
}

#[test]
fn test_health_check_degraded_service() {
    let checker = HealthCheckService::new();
    let result = checker.check_degraded_service();

    assert!(result.is_ok(), "Should succeed but indicate degraded");
    let status = result.unwrap();
    assert!(!status.healthy, "Should mark as unhealthy");
}

#[test]
fn test_alert_manager_with_invalid_recipients() {
    let manager = AlertManager::new();
    let result = manager.send_alert("critical", vec![]);

    assert!(result.is_err(), "Should fail with empty recipient list");
}

#[test]
fn test_alert_manager_rate_limiting() {
    let manager = AlertManager::with_rate_limit(2);

    let result1 = manager.send_alert("info", vec!["admin@test.com"]);
    let result2 = manager.send_alert("info", vec!["admin@test.com"]);
    let result3 = manager.send_alert("info", vec!["admin@test.com"]);

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_err(), "Should be rate limited on third alert");
}

#[test]
fn test_alert_deduplication() {
    let manager = AlertManager::with_deduplication(true);

    let result1 = manager.send_alert("error", vec!["admin@test.com"]);
    let result2 = manager.send_alert("error", vec!["admin@test.com"]);

    assert!(result1.is_ok());
    assert!(result2.is_err(), "Should deduplicate identical alert");
}

#[test]
fn test_health_check_partial_failure() {
    let checker = HealthCheckService::new();
    checker.register_component("db", true);
    checker.register_component("cache", false);
    checker.register_component("api", true);

    let result = checker.check_all();
    assert!(result.is_ok());

    let status = result.unwrap();
    assert!(!status.all_healthy, "Should indicate partial failure");
    assert_eq!(status.failed_components.len(), 1);
}

#[test]
fn test_alert_severity_escalation() {
    let manager = AlertManager::new();

    for _ in 0..5 {
        let _ = manager.send_alert("warning", vec!["admin@test.com"]);
    }

    let escalated = manager.check_escalation();
    assert!(escalated, "Should escalate after threshold warnings");
}

#[test]
fn test_health_check_timeout() {
    let checker = HealthCheckService::with_timeout(1);
    let result = checker.check_slow_service();

    assert!(result.is_err(), "Should timeout on slow service");
}

#[test]
fn test_alert_channel_full() {
    let manager = AlertManager::with_queue_size(2);

    let result1 = manager.send_alert("info", vec!["admin@test.com"]);
    let result2 = manager.send_alert("info", vec!["admin@test.com"]);
    let result3 = manager.send_alert("info", vec!["admin@test.com"]);

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_err(), "Should fail when queue is full");
}
