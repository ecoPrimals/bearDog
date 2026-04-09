// SPDX-License-Identifier: AGPL-3.0-or-later

//! Health Monitoring Tests

use super::health::*;
use std::time::Duration;

use std::sync::Arc;

#[cfg(test)]
#[expect(
    clippy::module_inception,
    reason = "nested test module colocated with health monitor sources"
)]
mod health_monitor_tests {
    use super::*;

    /// Test health monitor creation
    #[test]
    fn test_health_monitor_creation() {
        let _monitor = HealthMonitor::new(Duration::from_secs(1));
        // Test passes if monitor creation succeeds
    }

    /// Test health status creation - healthy
    #[test]
    fn test_health_status_healthy() {
        let status = HsmHealthStatus::healthy();
        assert!(status.is_healthy);
        assert!(status.error_message.is_none());
    }

    /// Test health status creation - unhealthy
    #[test]
    fn test_health_status_unhealthy() {
        let error = "Provider connection lost".to_string();
        let status = HsmHealthStatus::unhealthy(error.clone());
        assert!(!status.is_healthy);
        assert_eq!(status.error_message, Some(error));
    }

    /// Test starting health monitoring
    #[tokio::test]
    async fn test_start_health_monitoring() {
        let monitor = HealthMonitor::new(Duration::from_millis(100));
        let result = monitor.start_monitoring().await;
        assert!(result.is_ok());
        monitor.stop_monitoring().await;
    }

    /// Test stopping health monitoring
    #[tokio::test]
    async fn test_stop_health_monitoring() {
        let monitor = HealthMonitor::new(Duration::from_millis(100));
        monitor.start_monitoring().await.unwrap();

        // Modern pattern: Immediate stop without artificial delay
        // The start_monitoring().await ensures the task is spawned
        monitor.stop_monitoring().await;
        // Test passes if stop_monitoring succeeds
    }

    /// Test updating health status
    #[tokio::test]
    async fn test_update_health_status() {
        let monitor = HealthMonitor::new(Duration::from_secs(1));

        let status = HsmHealthStatus::healthy();
        monitor
            .update_health_status("provider-1".to_string(), status)
            .await;

        let retrieved = monitor.get_health_status("provider-1").await;
        assert!(retrieved.is_some());
        assert!(retrieved.unwrap().is_healthy);
    }

    /// Test getting health status
    #[tokio::test]
    async fn test_get_health_status() {
        let monitor = HealthMonitor::new(Duration::from_secs(1));

        // Initially should be None
        let status = monitor.get_health_status("provider-1").await;
        assert!(status.is_none());

        // Add a status
        monitor
            .update_health_status("provider-1".to_string(), HsmHealthStatus::healthy())
            .await;

        // Should now exist
        let status = monitor.get_health_status("provider-1").await;
        assert!(status.is_some());
    }

    /// Test `is_healthy` check
    #[tokio::test]
    async fn test_is_healthy() {
        let monitor = HealthMonitor::new(Duration::from_secs(1));

        // Non-existent provider should be unhealthy
        assert!(!monitor.is_healthy("provider-1").await);

        // Add healthy provider
        monitor
            .update_health_status("provider-1".to_string(), HsmHealthStatus::healthy())
            .await;

        assert!(monitor.is_healthy("provider-1").await);

        // Make provider unhealthy
        monitor
            .update_health_status(
                "provider-1".to_string(),
                HsmHealthStatus::unhealthy("Error".to_string()),
            )
            .await;

        assert!(!monitor.is_healthy("provider-1").await);
    }

    /// Test getting all health statuses
    #[tokio::test]
    async fn test_get_all_health_statuses() {
        let monitor = HealthMonitor::new(Duration::from_secs(1));

        // Initially empty
        let all_statuses = monitor.get_all_health_statuses().await;
        assert!(all_statuses.is_empty());

        // Add multiple providers
        monitor
            .update_health_status("provider-1".to_string(), HsmHealthStatus::healthy())
            .await;

        monitor
            .update_health_status(
                "provider-2".to_string(),
                HsmHealthStatus::unhealthy("Error".to_string()),
            )
            .await;

        // Should have 2 providers
        let all_statuses = monitor.get_all_health_statuses().await;
        assert_eq!(all_statuses.len(), 2);
    }

    /// Test double start monitoring fails
    #[tokio::test]
    async fn test_double_start_monitoring_fails() {
        let monitor = HealthMonitor::new(Duration::from_millis(100));

        let result1 = monitor.start_monitoring().await;
        assert!(result1.is_ok());

        let result2 = monitor.start_monitoring().await;
        assert!(result2.is_err());

        monitor.stop_monitoring().await;
    }

    /// Test health status transition from healthy to unhealthy
    #[tokio::test]
    async fn test_health_status_transition_healthy_to_unhealthy() {
        let monitor = HealthMonitor::new(Duration::from_secs(1));

        // Start healthy
        monitor
            .update_health_status("provider-1".to_string(), HsmHealthStatus::healthy())
            .await;

        assert!(monitor.is_healthy("provider-1").await);

        // Transition to unhealthy
        monitor
            .update_health_status(
                "provider-1".to_string(),
                HsmHealthStatus::unhealthy("Connection lost".to_string()),
            )
            .await;

        assert!(!monitor.is_healthy("provider-1").await);

        let status = monitor.get_health_status("provider-1").await.unwrap();
        assert_eq!(status.error_message, Some("Connection lost".to_string()));
    }

    /// Test health status transition from unhealthy to healthy
    #[tokio::test]
    async fn test_health_status_transition_unhealthy_to_healthy() {
        let monitor = HealthMonitor::new(Duration::from_secs(1));

        // Start unhealthy
        monitor
            .update_health_status(
                "provider-1".to_string(),
                HsmHealthStatus::unhealthy("Error".to_string()),
            )
            .await;

        assert!(!monitor.is_healthy("provider-1").await);

        // Transition to healthy
        monitor
            .update_health_status("provider-1".to_string(), HsmHealthStatus::healthy())
            .await;

        assert!(monitor.is_healthy("provider-1").await);

        let status = monitor.get_health_status("provider-1").await.unwrap();
        assert!(status.error_message.is_none());
    }

    /// Test concurrent health status updates
    #[tokio::test]
    async fn test_concurrent_health_status_updates() {
        let monitor = Arc::new(HealthMonitor::new(Duration::from_secs(1)));

        let mut handles = vec![];

        for i in 0..5 {
            let monitor_clone = monitor.clone();
            let handle = tokio::spawn(async move {
                monitor_clone
                    .update_health_status(format!("provider-{}", i), HsmHealthStatus::healthy())
                    .await;
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.await.unwrap();
        }

        let all_statuses = monitor.get_all_health_statuses().await;
        assert_eq!(all_statuses.len(), 5);
    }

    /// Test health monitoring with custom interval
    #[tokio::test]
    async fn test_health_monitoring_custom_interval() {
        let monitor = HealthMonitor::new(Duration::from_millis(50));

        monitor.start_monitoring().await.unwrap();

        // Modern pattern: Test the behavior, not the timing
        // Verify monitoring can be stopped cleanly without waiting
        monitor.stop_monitoring().await;

        // Test passes if monitoring starts and stops successfully
    }
}
