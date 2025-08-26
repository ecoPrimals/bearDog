

use crate::security_sentinel::{SecuritySentinel, SecuritySentinelConfig};
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use tracing::{error, info};

pub async fn security_sentinel_demo() -> BearDogResult<()> {
    info!("🛡️ Starting BearDog Security Sentinel Demo");

    let sentinel = SecuritySentinel::new()?;

    sentinel.start_monitoring().await?;

    let mut event_data = HashMap::with_capacity(16);
    event_data.insert("source".to_string(), "demo".to_string());
    event_data.insert("severity".to_string(), "low".to_string());

    sentinel.process_event("authentication", event_data.clone()).await?;
    sentinel.process_event("authorization", event_data.clone()).await?;
    sentinel.process_event("data_access", event_data.clone()).await?;

    let status_report = sentinel.get_status_report().await?;
    info!("Security Status: {}", status_report.status);
    info!("Threat Level: {}", status_report.threat_level);

    let threat_report = sentinel.get_threat_landscape().await?;
    info!("Risk Score: {:.1}", threat_report.risk_score);
    info!("Threat Vectors: {:?}", threat_report.threat_vectors);

    let capabilities_report = sentinel.get_capabilities_health().await?;
    info!("Capabilities Health Score: {:.1}%", capabilities_report.health_score);

    let performance_metrics = sentinel.get_performance_metrics().await?;
    info!("Detection Accuracy: {:.1}%", performance_metrics.detection_accuracy);
    info!("Average Processing Time: {:.1}ms", performance_metrics.avg_processing_time_ms);

    sentinel.stop_monitoring().await?;

    info!("✅ Security Sentinel Demo completed successfully");
    Ok(())
}

pub async fn integrate_security_sentinel_startup() -> BearDogResult<SecuritySentinel> {
    info!("🚀 Integrating Security Sentinel into startup sequence");

    let mut config = SecuritySentinelConfig::default();
    config.monitoring_interval_seconds = 30; // More frequent monitoring
    config.enable_threat_intelligence = true;

    let sentinel = SecuritySentinel::new()?;

    sentinel.start_monitoring().await?;

    let initial_report = sentinel.get_status_report().await?;

    match initial_report.status.as_str() {
        "HEALTHY" => info!("✅ Startup Security Status: EXCELLENT"),
        "WARNING" => info!("⚠️ Startup Security Status: GOOD"),
        "CRITICAL" => error!("🚨 Startup Security Status: CRITICAL"),
        _ => info!("ℹ️ Startup Security Status: {}", initial_report.status),
    }

    info!("🛡️ Security Sentinel successfully integrated into startup");
    Ok(sentinel)
}

pub async fn handle_security_alerts_example(sentinel: &SecuritySentinel) -> BearDogResult<()> {
    info!("🚨 Demonstrating security alert handling");

    let mut high_risk_event = HashMap::with_capacity(16);
    high_risk_event.insert("source".to_string(), "external".to_string());
    high_risk_event.insert("severity".to_string(), "high".to_string());
    high_risk_event.insert("type".to_string(), "suspicious_activity".to_string());

    sentinel.process_event("suspicious_activity", high_risk_event).await?;

    let status_report = sentinel.get_status_report().await?;
    if status_report.threat_level != "LOW" {
        info!("⚠️ Elevated threat level detected: {}", status_report.threat_level);

        let threat_report = sentinel.get_threat_landscape().await?;
        for recommendation in &threat_report.recommendations {
            info!("📋 Recommendation: {}", recommendation);
        }
    }

    info!("✅ Security alert handling demonstration completed");
    Ok(())
}

pub async fn monitor_security_performance_example(sentinel: &SecuritySentinel) -> BearDogResult<()> {
    info!("📊 Demonstrating security performance monitoring");

    let metrics = sentinel.get_performance_metrics().await?;
    
    info!("Current Security Performance Metrics:");
    info!("  Events per second: {:.1}", metrics.events_per_second);
    info!("  Average processing time: {:.1}ms", metrics.avg_processing_time_ms);
    info!("  Detection accuracy: {:.1}%", metrics.detection_accuracy);
    info!("  False positive rate: {:.1}%", metrics.false_positive_rate);

    if metrics.avg_processing_time_ms > 100.0 {
        info!("⚠️ Processing time above threshold, consider optimization");
    }

    if metrics.detection_accuracy < 90.0 {
        info!("⚠️ Detection accuracy below threshold, review detection rules");
    }

    info!("✅ Security performance monitoring demonstration completed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_sentinel_demo() {
        let result = security_sentinel_demo().await;
        assert!(
            result.is_ok(),
            "Security Sentinel demo should complete successfully"
        );
    }

    #[tokio::test]
    async fn test_security_sentinel_startup_integration() {
        let result = integrate_security_sentinel_startup().await;
        assert!(
            result.is_ok(),
            "Security Sentinel startup integration should succeed"
        );
    }

    #[tokio::test]
    async fn test_security_alerts_handling() {
        let sentinel = SecuritySentinel::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Should create SecuritySentinel", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Should create SecuritySentinel", e).to_string())
})?;
        let result = handle_security_alerts_example(&sentinel).await;
        assert!(
            result.is_ok(),
            "Security alerts handling should complete successfully"
        );
    }

    #[tokio::test]
    async fn test_performance_monitoring() {
        let sentinel = SecuritySentinel::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Should create SecuritySentinel", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Should create SecuritySentinel", e).to_string())
})?;
        let result = monitor_security_performance_example(&sentinel).await;
        assert!(
            result.is_ok(),
            "Performance monitoring should complete successfully"
        );
    }
}
