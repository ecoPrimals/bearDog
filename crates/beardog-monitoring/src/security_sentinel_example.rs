use crate::security_sentinel::{SecuritySentinel, SecuritySentinelConfig};
use beardog_errors::BearDogError;
use std::collections::HashMap;

/// Example function demonstrating `SecuritySentinel` usage
///
/// # Errors
/// Returns an error if the security sentinel operations fail
pub async fn demonstrate_security_sentinel() -> Result<(), BearDogError> {
    // Create configuration
    let config = SecuritySentinelConfig {
        enabled: true,
        max_events: 1000,
        retention_hours: 24,
        log_events: true,
        alert_threshold: 5,
        real_time_monitoring: true,
        monitoring_interval_seconds: 60,
    };

    // Initialize security sentinel
    let sentinel = SecuritySentinel::new(config);

    // Start monitoring
    sentinel.start_monitoring()?;

    // Process some security events
    let mut event_data = HashMap::new();
    event_data.insert("source_ip", "192.168.1.100");
    event_data.insert("user_id", "user123");

    sentinel
        .process_security_event("auth_failure", event_data.clone())
        .await?;
    sentinel
        .process_security_event("access_violation", event_data.clone())
        .await?;
    sentinel
        .process_security_event("compliance_violation", event_data)
        .await?;

    // Get status report
    let report = sentinel.get_status_report().await?;
    println!("Security Status: {}", report.status);
    println!("Total Events: {}", report.stats.total_events);
    println!("Auth Failures: {}", report.stats.auth_failures);

    // Check if monitoring is active
    if sentinel.is_monitoring_active() {
        println!("Security monitoring is active");
    }

    // Stop monitoring
    sentinel.stop_monitoring()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_sentinel_basic() -> Result<(), BearDogError> {
        let sentinel = SecuritySentinel::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: monitoring
        // TEST_PRIORITY: normal

        sentinel.start_monitoring()?;

        let status = sentinel.get_status_report().await?;
        assert_eq!(status.status, "ACTIVE");

        sentinel.stop_monitoring()?;

        Ok(())
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: monitoring
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_security_event_processing() -> Result<(), BearDogError> {
        let sentinel = SecuritySentinel::default();

        let mut event_data = HashMap::new();
        event_data.insert("test", "data");

        sentinel
            .process_security_event("auth_failure", event_data)
            .await?;

        let stats = sentinel.get_statistics().await;
        assert!(stats.auth_failures > 0);

        Ok(())
    }
}
