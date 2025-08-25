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


//! BearDog Security Sentinel Integration Demo
//!
//! This example demonstrates how to integrate the Security Sentinel
//! into a real BearDog deployment for self-aware security monitoring.

use beardog::{
    config::BearDogConfig,
    core::BearDogCore,
    errors::BearDogResult,
    monitoring::{SecuritySentinel, SecuritySentinelConfig},
    production::ProductionManager,
};
use std::sync::Arc;
use tokio::signal;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> BearDogResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🚀 Starting BearDog with integrated Security Sentinel");

    // Initialize BearDog core system
    let config = BearDogConfig::load_or_create("beardog.toml").await?;
    let beardog_core = Arc::new(BearDogCore::new(config).await?);

    // Initialize Security Sentinel with production configuration
    let sentinel_config = SecuritySentinelConfig {
        enabled: true,
        monitoring_interval_secs: 30,
        enable_threat_intelligence: true,
        enable_performance_monitoring: true,
        enable_sovereignty_monitoring: true,
        alert_thresholds: beardog::monitoring::AlertThresholds {
            min_security_posture_score: 0.7,
            max_threat_exposure_level: beardog::monitoring::ThreatLevel::Medium,
            min_capability_health_score: 0.8,
            max_security_response_time_ms: 1000,
            min_sovereignty_score: 0.9,
        },
        data_retention_hours: 24,
    };

    let security_sentinel = SecuritySentinel::with_config(sentinel_config);

    // Perform initial security assessment
    info!("🛡️ Performing initial security assessment...");
    let initial_report = security_sentinel.perform_security_assessment().await?;

    // Display startup security status
    display_security_status(&initial_report).await;

    // Verify security meets minimum requirements
    if initial_report.overall_security_score < 0.7 {
        error!("🚨 Security score too low for production deployment: {:.2}", 
               initial_report.overall_security_score);
        return Err(beardog::errors::BearDogError::configuration("Security posture insufficient for production".to_string(),
        ));
    }

    // Verify human dignity compliance
    if initial_report.sovereignty_status.sovereignty_score < 0.9 {
        warn!("⚠️ Sovereignty score below recommended threshold: {:.2}", 
              initial_report.sovereignty_status.sovereignty_score);
    }

    // Start continuous security monitoring
    info!("🔄 Starting continuous security monitoring...");
    security_sentinel.start_monitoring().await?;

    // Initialize production manager (which will use the core)
    let mut production_manager = ProductionManager::new(beardog_core.clone()).await?;

    // Start BearDog production services
    info!("🏭 Starting BearDog production services...");
    
    // In a production system, this would start the full BearDog service
    // For this demo, we'll simulate some operations and monitor security
    
    // Spawn the main service loop
    let sentinel_clone = security_sentinel.clone();
    let monitoring_handle = tokio::spawn(async move {
        monitor_security_continuously(sentinel_clone).await;
    });

    // Simulate BearDog operations
    let operations_handle = tokio::spawn(async move {
        simulate_beardog_operations().await;
    });

    // Wait for shutdown signal
    info!("✅ BearDog with Security Sentinel is running");
    info!("   Press Ctrl+C to shutdown gracefully");

    // Wait for SIGINT (Ctrl+C)
    signal::ctrl_c().await.map_err(|e| beardog::errors::BearDogError::System {
        message: format!("Failed to listen for shutdown signal: {}", e),
    })?;

    info!("🛑 Shutdown signal received, stopping services...");

    // Stop security monitoring
    security_sentinel.stop_monitoring().await;
    
    // Cancel background tasks
    monitoring_handle.abort();
    operations_handle.abort();

    // Final security assessment before shutdown
    let final_report = security_sentinel.perform_security_assessment().await?;
    
    info!("📊 Final Security Assessment:");
    info!("   Security Score: {:.2}", final_report.overall_security_score);
    info!("   Sovereignty Score: {:.2}", final_report.sovereignty_status.sovereignty_score);
    
    let stats = security_sentinel.get_sentinel_stats().await;
    let assessments = stats.security_assessments.load(std::sync::atomic::Ordering::Relaxed);
    info!("   Total Assessments: {}", assessments);

    info!("✅ BearDog shutdown complete - Security integrity maintained");
    Ok(())
}

/// Display comprehensive security status at startup
async fn display_security_status(report: &beardog::monitoring::SecurityStatusReport) {
    info!("🛡️ ========== SECURITY SENTINEL REPORT ==========");
    info!("   Overall Security Score: {:.2}/1.0", report.overall_security_score);
    
    // Threat landscape
    info!("🌍 Threat Landscape:");
    info!("   Threat Level: {:?}", report.threat_landscape.threat_level);
    info!("   Active Threats: {}", report.threat_landscape.active_threats.len());
    info!("   Intelligence Confidence: {:.2}", report.threat_landscape.intelligence_confidence);

    // Capabilities health
    info!("🔧 Security Capabilities:");
    info!("   Overall Health: {:.2}/1.0", report.capabilities_health.overall_health_score);
    for capability in &report.capabilities_health.capability_statuses {
        let status_emoji = match capability.status.as_str() {
            "healthy" => "✅",
            "degraded" => "⚠️",
            _ => "❌",
        };
        info!("   {} {}: {:.2}", status_emoji, capability.capability_name, capability.health_score);
    }

    // Performance metrics
    info!("⚡ Security Performance:");
    info!("   Avg Response Time: {:.1}ms", report.performance_metrics.avg_response_time_ms);
    info!("   Security Ops/sec: {:.0}", report.performance_metrics.security_ops_per_sec);
    info!("   Error Rate: {:.2}%", report.performance_metrics.security_error_rate);

    // Human dignity metrics
    info!("👑 Human Dignity Preservation:");
    let dignity = &report.sovereignty_status.human_dignity_metrics;
    info!("   Privacy Protection: {:.1}%", dignity.privacy_protection_score * 100.0);
    info!("   Consent Compliance: {:.1}%", dignity.consent_compliance_score * 100.0);
    info!("   Surveillance Resistance: {:.1}%", dignity.surveillance_resistance_score * 100.0);
    info!("   User Empowerment: {:.1}%", dignity.user_empowerment_score * 100.0);

    // Sovereignty status
    info!("🏛️ Sovereignty Status:");
    info!("   Overall Score: {:.2}/1.0", report.sovereignty_status.sovereignty_score);
    info!("   Independence Score: {:.2}/1.0", report.sovereignty_status.independence_score);

    // Alerts and recommendations
    if report.alert_status.active_alerts > 0 {
        warn!("⚠️ Active Alerts: {} (Critical: {})", 
              report.alert_status.active_alerts, report.alert_status.critical_alerts);
    } else {
        info!("✅ No Active Alerts");
    }

    if !report.recommendations.is_empty() {
        info!("💡 Security Recommendations:");
        for (i, rec) in report.recommendations.iter().take(3).enumerate() {
            let priority_emoji = match rec.priority {
                beardog::monitoring::RecommendationPriority::Critical => "🚨",
                beardog::monitoring::RecommendationPriority::High => "⚠️",
                beardog::monitoring::RecommendationPriority::Medium => "💡",
                beardog::monitoring::RecommendationPriority::Low => "📝",
            };
            info!("   {} {}: {}", priority_emoji, rec.category, rec.description);
        }
        if report.recommendations.len() > 3 {
            info!("   ... and {} more recommendations", report.recommendations.len() - 3);
        }
    }

    info!("🛡️ =============================================");
}

/// Continuously monitor security and react to changes
async fn monitor_security_continuously(sentinel: SecuritySentinel) {
    let mut assessment_count = 0;
    let mut interval = tokio::time::interval(Duration::from_secs(60)); // Every minute for demo
    
    info!("🔄 Starting continuous security monitoring loop");
    
    loop {
        interval.tick().await;
        assessment_count += 1;
        
        match sentinel.perform_security_assessment().await {
            Ok(report) => {
                let security_status = match report.overall_security_score {
                    s if s >= 0.9 => "EXCELLENT",
                    s if s >= 0.8 => "GOOD", 
                    s if s >= 0.7 => "ADEQUATE",
                    s if s >= 0.5 => "DEGRADED",
                    _ => "CRITICAL"
                };

                info!("🛡️ Security Check #{}: {} (Score: {:.2})", 
                      assessment_count, security_status, report.overall_security_score);

                // React to security changes
                if report.overall_security_score < 0.7 {
                    error!("🚨 SECURITY ALERT: Score dropped to {:.2}", report.overall_security_score);
                    
                    // In production, this might trigger:
                    // - Automated incident response
                    // - Enhanced monitoring
                    // - Administrator notifications
                    // - Service isolation
                }

                if report.alert_status.critical_alerts > 0 {
                    error!("🚨 CRITICAL SECURITY ALERTS: {}", report.alert_status.critical_alerts);
                }

                // Monitor sovereignty compliance
                if report.sovereignty_status.sovereignty_score < 0.9 {
                    warn!("👑 SOVEREIGNTY ALERT: Score {:.2} - Human dignity may be compromised", 
                          report.sovereignty_status.sovereignty_score);
                }

                // Adaptive monitoring frequency based on threat level
                match report.threat_landscape.threat_level {
                    beardog::monitoring::ThreatLevel::Critical => {
                        // Monitor every 10 seconds under critical threat
                        interval = tokio::time::interval(Duration::from_secs(10));
                        warn!("🚨 Switching to high-frequency monitoring due to critical threats");
                    },
                    beardog::monitoring::ThreatLevel::High => {
                        // Monitor every 30 seconds under high threat
                        interval = tokio::time::interval(Duration::from_secs(30));
                        info!("⚠️ Increased monitoring frequency due to high threat level");
                    },
                    _ => {
                        // Normal monitoring frequency
                        if assessment_count % 5 == 0 {
                            info!("✅ Security monitoring: {} assessments completed", assessment_count);
                        }
                    }
                }
            },
            Err(e) => {
                error!("❌ Security assessment failed: {}", e);
                // In production, this might trigger failsafe mode
            }
        }
    }
}

/// Simulate BearDog operations for demonstration
async fn simulate_beardog_operations() {
    let mut operation_count = 0;
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    
    loop {
        interval.tick().await;
        operation_count += 1;
        
        // Simulate various BearDog operations
        match operation_count % 4 {
            0 => {
                info!("🔐 Simulating cryptographic operation #{}", operation_count);
                // Simulate crypto work
                sleep(Duration::from_millis(10)).await;
            },
            1 => {
                info!("🔍 Simulating threat analysis #{}", operation_count);
                // Simulate threat detection
                sleep(Duration::from_millis(20)).await;
            },
            2 => {
                info!("🌐 Simulating network operation #{}", operation_count);
                // Simulate network communication
                sleep(Duration::from_millis(15)).await;
            },
            _ => {
                info!("📊 Simulating data processing #{}", operation_count);
                // Simulate data processing
                sleep(Duration::from_millis(25)).await;
            }
        }
        
        if operation_count % 10 == 0 {
            info!("🔄 BearDog operations: {} cycles completed", operation_count);
        }
    }
}

/// Example of how to integrate Security Sentinel into existing BearDog services
#[allow(dead_code)]
async fn integrate_with_existing_services(core: Arc<BearDogCore>) -> BearDogResult<SecuritySentinel> {
    info!("🔗 Integrating Security Sentinel with existing BearDog services");

    // Create Security Sentinel with configuration from BearDog core
    let sentinel = SecuritySentinel::new();
    
    // Start monitoring
    sentinel.start_monitoring().await?;
    
    // Perform initial assessment
    let report = sentinel.perform_security_assessment().await?;
    
    // Integrate with BearDog's existing monitoring/alerting
    if report.overall_security_score < 0.8 {
        // In production, this would integrate with existing alert systems
        warn!("🚨 Security score below production threshold: {:.2}", report.overall_security_score);
    }
    
    // Return configured sentinel for use by other services
    Ok(sentinel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_sentinel_integration() {
        // Test that Security Sentinel integrates properly with BearDog
        let config = BearDogConfig::load_or_create("test_beardog.toml").await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        let core = Arc::new(BearDogCore::new(config).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?);
        
        let sentinel = integrate_with_existing_services(core).await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        let report = sentinel.perform_security_assessment().await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})?;
        
        assert!(report.overall_security_score >= 0.0);
        assert!(report.overall_security_score <= 1.0);
        
        // Should maintain high sovereignty standards
        assert!(report.sovereignty_status.sovereignty_score >= 0.8);
    }

    #[test]
    fn test_beardog_security_sentinel_available() {
        // Test that Security Sentinel is properly exported from BearDog
        let _sentinel = beardog::monitoring::SecuritySentinel::new();
        // If this compiles, the integration is working
    }
} 