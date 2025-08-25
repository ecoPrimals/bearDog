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


use crate::security_sentinel::{SecuritySentinel, SecuritySentinelConfig};
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use tracing::{error, info};

/// **CANONICAL SECURITY SENTINEL EXAMPLE** - Demonstration of security monitoring
/// This module provides examples of how to use the SecuritySentinel for comprehensive
/// security monitoring, threat detection, and compliance tracking.

/// Demonstrate security sentinel functionality
pub async fn security_sentinel_demo() -> BearDogResult<()> {
    info!("🛡️ Starting BearDog Security Sentinel Demo");

    // Create security sentinel with default configuration
    let sentinel = SecuritySentinel::new()?;

    // Start monitoring
    sentinel.start_monitoring().await?;

    // Simulate some security events
    let mut event_data = HashMap::new();
    event_data.insert("source".to_string(), "demo".to_string());
    event_data.insert("severity".to_string(), "low".to_string());

    // Process different types of security events
    sentinel.process_event("authentication", event_data.clone()).await?;
    sentinel.process_event("authorization", event_data.clone()).await?;
    sentinel.process_event("data_access", event_data.clone()).await?;

    // Get security status report
    let status_report = sentinel.get_status_report().await?;
    info!("Security Status: {}", status_report.status);
    info!("Threat Level: {}", status_report.threat_level);

    // Get threat landscape report
    let threat_report = sentinel.get_threat_landscape().await?;
    info!("Risk Score: {:.1}", threat_report.risk_score);
    info!("Threat Vectors: {:?}", threat_report.threat_vectors);

    // Get capabilities health report
    let capabilities_report = sentinel.get_capabilities_health().await?;
    info!("Capabilities Health Score: {:.1}%", capabilities_report.health_score);

    // Get performance metrics
    let performance_metrics = sentinel.get_performance_metrics().await?;
    info!("Detection Accuracy: {:.1}%", performance_metrics.detection_accuracy);
    info!("Average Processing Time: {:.1}ms", performance_metrics.avg_processing_time_ms);

    // Stop monitoring
    sentinel.stop_monitoring().await?;

    info!("✅ Security Sentinel Demo completed successfully");
    Ok(())
}

/// Integration example for startup security monitoring
pub async fn integrate_security_sentinel_startup() -> BearDogResult<SecuritySentinel> {
    info!("🚀 Integrating Security Sentinel into startup sequence");

    // Create security sentinel with custom configuration
    let mut config = SecuritySentinelConfig::default();
    config.monitoring_interval_seconds = 30; // More frequent monitoring
    config.enable_threat_intelligence = true;

    let sentinel = SecuritySentinel::new()?;

    // Start monitoring immediately
    sentinel.start_monitoring().await?;

    // Get initial security assessment
    let initial_report = sentinel.get_status_report().await?;
    
    // Log startup security status
    match initial_report.status.as_str() {
        "HEALTHY" => info!("✅ Startup Security Status: EXCELLENT"),
        "WARNING" => info!("⚠️ Startup Security Status: GOOD"),
        "CRITICAL" => error!("🚨 Startup Security Status: CRITICAL"),
        _ => info!("ℹ️ Startup Security Status: {}", initial_report.status),
    }

    info!("🛡️ Security Sentinel successfully integrated into startup");
    Ok(sentinel)
}

/// Example of handling security alerts
pub async fn handle_security_alerts_example(sentinel: &SecuritySentinel) -> BearDogResult<()> {
    info!("🚨 Demonstrating security alert handling");

    // Simulate high-risk security event
    let mut high_risk_event = HashMap::new();
    high_risk_event.insert("source".to_string(), "external".to_string());
    high_risk_event.insert("severity".to_string(), "high".to_string());
    high_risk_event.insert("type".to_string(), "suspicious_activity".to_string());

    // Process the high-risk event
    sentinel.process_event("suspicious_activity", high_risk_event).await?;

    // Check for any alerts generated
    let status_report = sentinel.get_status_report().await?;
    if status_report.threat_level != "LOW" {
        info!("⚠️ Elevated threat level detected: {}", status_report.threat_level);
        
        // Get detailed threat landscape
        let threat_report = sentinel.get_threat_landscape().await?;
        for recommendation in &threat_report.recommendations {
            info!("📋 Recommendation: {}", recommendation);
        }
    }

    info!("✅ Security alert handling demonstration completed");
    Ok(())
}

/// Example of monitoring security performance
pub async fn monitor_security_performance_example(sentinel: &SecuritySentinel) -> BearDogResult<()> {
    info!("📊 Demonstrating security performance monitoring");

    // Get current performance metrics
    let metrics = sentinel.get_performance_metrics().await?;
    
    info!("Current Security Performance Metrics:");
    info!("  Events per second: {:.1}", metrics.events_per_second);
    info!("  Average processing time: {:.1}ms", metrics.avg_processing_time_ms);
    info!("  Detection accuracy: {:.1}%", metrics.detection_accuracy);
    info!("  False positive rate: {:.1}%", metrics.false_positive_rate);

    // Check if performance is within acceptable thresholds
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
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Should create SecuritySentinel", e))
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
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "Should create SecuritySentinel", e))
})?;
        let result = monitor_security_performance_example(&sentinel).await;
        assert!(
            result.is_ok(),
            "Performance monitoring should complete successfully"
        );
    }
}
