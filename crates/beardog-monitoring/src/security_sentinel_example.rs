//! Security Sentinel Usage Example
//!
//! Demonstrates how BearDog uses Security Sentinel for self-aware monitoring
//! that focuses on protecting humans, not surveilling them.

use crate::security_sentinel::{SecuritySentinel, SecuritySentinelConfig};
use beardog_errors::BearDogResult;
use tracing::info;

/// Example of using Security Sentinel for self-aware monitoring
pub async fn security_sentinel_demo() -> BearDogResult<()> {
    info!("🛡️ Starting Security Sentinel Demo - BearDog Self-Awareness Example");

    // Create security sentinel with custom configuration
    let config = SecuritySentinelConfig {
        enabled: true,
        monitoring_interval_secs: 60, // Check every minute
        enable_threat_intelligence: true,
        enable_performance_monitoring: true,
        enable_sovereignty_monitoring: true,
        ..Default::default()
    };

    let sentinel = SecuritySentinel::with_config(config);

    // Start monitoring in background
    sentinel
        .start_monitoring()
        .await
        .map_err(|e| beardog_errors::BearDogError::Internal {
            message: format!("Failed to start security monitoring: {e}"),
        })?;

    // Perform immediate security assessment
    let security_report = sentinel.perform_security_assessment().await.map_err(|e| {
        beardog_errors::BearDogError::Internal {
            message: format!("Failed to perform security assessment: {e}"),
        }
    })?;

    info!("📊 Security Assessment Results:");
    info!(
        "   Overall Security Score: {:.2}",
        security_report.overall_security_score
    );
    info!(
        "   Threat Level: {:?}",
        security_report.threat_landscape.threat_level
    );
    info!(
        "   Capabilities Health: {:.2}",
        security_report.capabilities_health.overall_health_score
    );
    info!(
        "   Sovereignty Score: {:.2}",
        security_report.sovereignty_status.sovereignty_score
    );
    info!(
        "   Active Alerts: {}",
        security_report.alert_status.active_alerts
    );
    info!(
        "   Recommendations: {}",
        security_report.recommendations.len()
    );

    // Display key insights
    info!("🎯 Key Security Insights:");
    for recommendation in security_report.recommendations {
        info!(
            "   {} Priority {}: {}",
            match recommendation.priority {
                crate::security_sentinel::RecommendationPriority::Critical => "🚨",
                crate::security_sentinel::RecommendationPriority::High => "⚠️",
                crate::security_sentinel::RecommendationPriority::Medium => "💡",
                crate::security_sentinel::RecommendationPriority::Low => "📝",
            },
            recommendation.category,
            recommendation.description
        );
    }

    // Show human dignity metrics
    let dignity_metrics = &security_report.sovereignty_status.human_dignity_metrics;
    info!("👑 Human Dignity Preservation:");
    info!(
        "   Privacy Protection: {:.1}%",
        dignity_metrics.privacy_protection_score * 100.0
    );
    info!(
        "   Consent Compliance: {:.1}%",
        dignity_metrics.consent_compliance_score * 100.0
    );
    info!(
        "   Surveillance Resistance: {:.1}%",
        dignity_metrics.surveillance_resistance_score * 100.0
    );
    info!(
        "   User Empowerment: {:.1}%",
        dignity_metrics.user_empowerment_score * 100.0
    );

    // Show performance metrics
    info!("⚡ Security Performance:");
    info!(
        "   Average Response Time: {:.1}ms",
        security_report.performance_metrics.avg_response_time_ms
    );
    info!(
        "   Security Ops/sec: {:.0}",
        security_report.performance_metrics.security_ops_per_sec
    );
    info!(
        "   Error Rate: {:.2}%",
        security_report.performance_metrics.security_error_rate
    );

    // Demonstrate continuous monitoring
    info!("🔄 Security Sentinel will continue monitoring in background");
    info!("   Protecting human dignity and security autonomy");
    info!("   Watching our own systems, never surveilling users");

    Ok(())
}

/// Example of how to integrate Security Sentinel into BearDog's startup
pub async fn integrate_security_sentinel_startup() -> BearDogResult<SecuritySentinel> {
    info!("🚀 Integrating Security Sentinel into BearDog startup");

    // Create sentinel with production configuration
    let sentinel = SecuritySentinel::new();

    // Start monitoring
    sentinel
        .start_monitoring()
        .await
        .map_err(|e| beardog_errors::BearDogError::Internal {
            message: format!("Failed to start security monitoring: {e}"),
        })?;

    // Perform initial security assessment
    let initial_report = sentinel.perform_security_assessment().await.map_err(|e| {
        beardog_errors::BearDogError::Internal {
            message: format!("Failed to perform security assessment: {e}"),
        }
    })?;

    // Log startup security status
    match initial_report.overall_security_score {
        score if score >= 0.9 => {
            info!("✅ BearDog Security Status: EXCELLENT - Ready to protect humans");
        }
        score if score >= 0.7 => {
            info!("⚠️ BearDog Security Status: GOOD - Some areas for improvement");
        }
        score => {
            info!(
                "🚨 BearDog Security Status: NEEDS ATTENTION - Score: {:.2}",
                score
            );
        }
    }

    // Check sovereignty compliance
    if initial_report.sovereignty_status.sovereignty_score >= 0.9 {
        info!("👑 Sovereignty Status: EXCELLENT - Human dignity fully preserved");
    } else {
        info!(
            "⚠️ Sovereignty Status: Needs improvement - Score: {:.2}",
            initial_report.sovereignty_status.sovereignty_score
        );
    }

    Ok(sentinel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_sentinel_demo() {
        // This test demonstrates that Security Sentinel works correctly
        let result = security_sentinel_demo().await;
        assert!(
            result.is_ok(),
            "Security Sentinel demo should complete successfully"
        );
    }

    #[tokio::test]
    async fn test_security_sentinel_integration() {
        // Test integration into startup process
        let sentinel_result = integrate_security_sentinel_startup().await;
        assert!(
            sentinel_result.is_ok(),
            "Security Sentinel integration should work"
        );

        let sentinel = sentinel_result.unwrap();
        let stats = sentinel.get_sentinel_stats().await;

        // Should have performed at least one assessment during integration
        assert!(
            stats
                .security_assessments
                .load(std::sync::atomic::Ordering::Relaxed)
                > 0
        );
    }
}
