//! Comprehensive Tests for Security Sentinel System
//!
//! Tests for all components of the Security Sentinel to ensure
//! BearDog's self-aware monitoring works correctly.

use super::*;
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_security_sentinel_creation_and_configuration() {
    // Test default creation
    let sentinel_default = SecuritySentinel::new();
    assert!(sentinel_default.config.enabled);
    assert_eq!(sentinel_default.config.monitoring_interval_secs, 30);

    // Test custom configuration
    let config = SecuritySentinelConfig {
        enabled: true,
        monitoring_interval_secs: 60,
        enable_threat_intelligence: true,
        enable_performance_monitoring: true,
        enable_sovereignty_monitoring: true,
        alert_thresholds: AlertThresholds {
            min_security_posture_score: 0.8,
            max_threat_exposure_level: ThreatLevel::Medium,
            min_capability_health_score: 0.85,
            max_security_response_time_ms: 500,
            min_sovereignty_score: 0.95,
        },
        data_retention_hours: 48,
    };

    let sentinel_custom = SecuritySentinel::with_config(config.clone());
    assert_eq!(sentinel_custom.config.monitoring_interval_secs, 60);
    assert_eq!(
        sentinel_custom
            .config
            .alert_thresholds
            .min_security_posture_score,
        0.8
    );
    assert_eq!(sentinel_custom.config.data_retention_hours, 48);
}

#[tokio::test]
async fn test_security_posture_assessment() {
    let sentinel = SecuritySentinel::new();
    let report = sentinel.perform_security_assessment().await.unwrap();

    // Validate security posture report structure
    assert!(report.overall_security_score >= 0.0);
    assert!(report.overall_security_score <= 1.0);
    assert!(!report.recommendations.is_empty() || report.overall_security_score > 0.8);

    // Validate threat landscape
    assert!(report.threat_landscape.intelligence_confidence >= 0.0);
    assert!(report.threat_landscape.intelligence_confidence <= 1.0);

    // Validate capabilities health
    assert!(report.capabilities_health.overall_health_score >= 0.0);
    assert!(report.capabilities_health.overall_health_score <= 1.0);
    assert!(!report.capabilities_health.capability_statuses.is_empty());

    // Validate sovereignty status
    assert!(report.sovereignty_status.sovereignty_score >= 0.0);
    assert!(report.sovereignty_status.sovereignty_score <= 1.0);
    assert!(!report.sovereignty_status.autonomy_indicators.is_empty());

    // Validate human dignity metrics
    let dignity = &report.sovereignty_status.human_dignity_metrics;
    assert!(dignity.privacy_protection_score >= 0.0 && dignity.privacy_protection_score <= 1.0);
    assert!(dignity.consent_compliance_score >= 0.0 && dignity.consent_compliance_score <= 1.0);
    assert!(
        dignity.surveillance_resistance_score >= 0.0
            && dignity.surveillance_resistance_score <= 1.0
    );
    assert!(dignity.user_empowerment_score >= 0.0 && dignity.user_empowerment_score <= 1.0);

    println!(
        "✅ Security Assessment Complete - Score: {:.2}",
        report.overall_security_score
    );
}

#[tokio::test]
async fn test_individual_sentinel_components() {
    // Test Security Posture Monitor
    let posture_monitor = SecurityPostureMonitor::new();
    let posture_report = posture_monitor.assess_security_posture().await.unwrap();

    assert!(posture_report.overall_posture_score >= 0.0);
    assert!(posture_report.overall_posture_score <= 1.0);
    assert!(!posture_report
        .cryptographic_capabilities
        .indicators
        .is_empty());
    assert!(!posture_report.authentication_systems.indicators.is_empty());

    // Test Threat Landscape Intelligence
    let threat_intelligence = ThreatLandscapeIntelligence::new();
    let threat_report = threat_intelligence.assess_threat_landscape().await;

    assert!(threat_report.intelligence_confidence >= 0.0);
    assert!(threat_report.intelligence_confidence <= 1.0);

    // Test Security Capability Monitor
    let capability_monitor = SecurityCapabilityMonitor::new();
    let capability_report = capability_monitor.assess_capabilities().await;

    assert!(capability_report.overall_health_score >= 0.0);
    assert!(capability_report.overall_health_score <= 1.0);
    assert!(!capability_report.capability_statuses.is_empty());

    // Test Performance Sentinel
    let performance_sentinel = PerformanceSentinel::new();
    let performance_metrics = performance_sentinel.gather_performance_metrics().await;

    assert!(performance_metrics.avg_response_time_ms >= 0.0);
    assert!(performance_metrics.security_ops_per_sec >= 0.0);
    assert!(performance_metrics.security_error_rate >= 0.0);

    // Test Sovereignty Health Monitor
    let sovereignty_monitor = SovereigntyHealthMonitor::new();
    let sovereignty_report = sovereignty_monitor.assess_sovereignty().await;

    assert!(sovereignty_report.sovereignty_score >= 0.0);
    assert!(sovereignty_report.sovereignty_score <= 1.0);
    assert!(!sovereignty_report.autonomy_indicators.is_empty());

    println!("✅ All individual components tested successfully");
}

#[tokio::test]
async fn test_security_sentinel_monitoring_loop() {
    let config = SecuritySentinelConfig {
        enabled: true,
        monitoring_interval_secs: 1, // Fast interval for testing
        ..Default::default()
    };

    let sentinel = SecuritySentinel::with_config(config);

    // Test that monitoring can be started and stopped
    let result = sentinel.start_monitoring().await;
    assert!(result.is_ok(), "Should be able to start monitoring");

    // Check that monitoring is active
    let stats = sentinel.get_sentinel_stats().await;
    assert!(stats
        .monitoring_active
        .load(std::sync::atomic::Ordering::Relaxed));

    // Perform manual assessments to simulate what the monitoring loop does
    let report1 = sentinel.perform_security_assessment().await.unwrap();
    let report2 = sentinel.perform_security_assessment().await.unwrap();

    // Both reports should be valid
    assert!(report1.overall_security_score >= 0.0 && report1.overall_security_score <= 1.0);
    assert!(report2.overall_security_score >= 0.0 && report2.overall_security_score <= 1.0);

    // Check that multiple assessments were recorded
    let final_stats = sentinel.get_sentinel_stats().await;
    let assessments = final_stats
        .security_assessments
        .load(std::sync::atomic::Ordering::Relaxed);
    assert!(
        assessments >= 2,
        "Should have performed at least 2 assessments, got {assessments}",
    );

    // Stop monitoring
    sentinel.stop_monitoring().await;

    // After stopping, monitoring should be inactive
    let stopped_stats = sentinel.get_sentinel_stats().await;
    assert!(!stopped_stats
        .monitoring_active
        .load(std::sync::atomic::Ordering::Relaxed));

    println!("✅ Monitoring loop tested successfully - {assessments} assessments performed",);
}

#[tokio::test]
async fn test_human_dignity_metrics_thresholds() {
    let sentinel = SecuritySentinel::new();
    let report = sentinel.perform_security_assessment().await.unwrap();

    let dignity_metrics = &report.sovereignty_status.human_dignity_metrics;

    // BearDog should maintain high human dignity standards
    assert!(
        dignity_metrics.privacy_protection_score >= 0.9,
        "Privacy protection below threshold: {:.2}",
        dignity_metrics.privacy_protection_score
    );

    assert!(
        dignity_metrics.consent_compliance_score >= 0.9,
        "Consent compliance below threshold: {:.2}",
        dignity_metrics.consent_compliance_score
    );

    assert!(
        dignity_metrics.surveillance_resistance_score >= 0.9,
        "Surveillance resistance below threshold: {:.2}",
        dignity_metrics.surveillance_resistance_score
    );

    assert!(
        dignity_metrics.user_empowerment_score >= 0.9,
        "User empowerment below threshold: {:.2}",
        dignity_metrics.user_empowerment_score
    );

    println!("✅ Human dignity metrics exceed minimum thresholds");
    println!(
        "   Privacy: {:.1}%",
        dignity_metrics.privacy_protection_score * 100.0
    );
    println!(
        "   Consent: {:.1}%",
        dignity_metrics.consent_compliance_score * 100.0
    );
    println!(
        "   Surveillance Resistance: {:.1}%",
        dignity_metrics.surveillance_resistance_score * 100.0
    );
    println!(
        "   User Empowerment: {:.1}%",
        dignity_metrics.user_empowerment_score * 100.0
    );
}

#[tokio::test]
async fn test_alert_generation() {
    // Create sentinel with strict thresholds to trigger alerts
    let config = SecuritySentinelConfig {
        enabled: true,
        alert_thresholds: AlertThresholds {
            min_security_posture_score: 0.99,            // Very high threshold
            max_threat_exposure_level: ThreatLevel::Low, // Very low threshold
            min_capability_health_score: 0.99,
            max_security_response_time_ms: 1, // Very strict response time
            min_sovereignty_score: 0.99,
        },
        ..Default::default()
    };

    let sentinel = SecuritySentinel::with_config(config);
    let report = sentinel.perform_security_assessment().await.unwrap();

    // With such strict thresholds, we should have some alerts
    println!("Alert Status:");
    println!("   Active Alerts: {}", report.alert_status.active_alerts);
    println!(
        "   Critical Alerts: {}",
        report.alert_status.critical_alerts
    );

    // Verify alert structure is valid
    assert!(!report.alert_status.alert_trends.is_empty());

    println!("✅ Alert generation system tested");
}

#[tokio::test]
async fn test_performance_metrics_consistency() {
    let sentinel = SecuritySentinel::new();

    // Perform multiple assessments
    let report1 = sentinel.perform_security_assessment().await.unwrap();
    sleep(Duration::from_millis(100)).await;
    let report2 = sentinel.perform_security_assessment().await.unwrap();

    // Performance metrics should be reasonably consistent
    let perf1 = &report1.performance_metrics;
    let perf2 = &report2.performance_metrics;

    // Response times should be similar (within reasonable variance)
    let response_time_diff = (perf1.avg_response_time_ms - perf2.avg_response_time_ms).abs();
    assert!(
        response_time_diff < 100.0,
        "Response time variance too high: {response_time_diff:.1}ms",
    );

    // Operations per second should be similar
    let ops_diff = (perf1.security_ops_per_sec - perf2.security_ops_per_sec).abs();
    assert!(
        ops_diff < 1000.0,
        "Operations variance too high: {ops_diff:.1}",
    );

    println!("✅ Performance metrics consistency verified");
    println!("   Response Time Variance: {response_time_diff:.1}ms");
    println!("   Ops/sec Variance: {ops_diff:.1}");
}

#[tokio::test]
async fn test_threat_level_assessment() {
    let sentinel = SecuritySentinel::new();
    let report = sentinel.perform_security_assessment().await.unwrap();

    let threat_landscape = &report.threat_landscape;

    // Validate threat level is reasonable
    match threat_landscape.threat_level {
        ThreatLevel::Critical => {
            assert!(
                !threat_landscape.active_threats.is_empty(),
                "Critical threat level should have active threats"
            );
        }
        ThreatLevel::High => {
            assert!(
                !threat_landscape.active_threats.is_empty(),
                "High threat level should have threats"
            );
        }
        ThreatLevel::Medium | ThreatLevel::Low => {
            // These are acceptable for normal operation
        }
    }

    // Validate threat indicators
    for threat in &threat_landscape.active_threats {
        assert!(threat.confidence >= 0.0 && threat.confidence <= 1.0);
        assert!(!threat.threat_type.is_empty());
        assert!(!threat.description.is_empty());
    }

    println!(
        "✅ Threat level assessment validated - Level: {:?}",
        threat_landscape.threat_level
    );
}

#[tokio::test]
async fn test_security_sentinel_statistics() {
    let sentinel = SecuritySentinel::new();

    // Perform several operations
    sentinel.perform_security_assessment().await.unwrap();
    sentinel.perform_security_assessment().await.unwrap();
    sentinel.perform_security_assessment().await.unwrap();

    let stats = sentinel.get_sentinel_stats().await;

    // Should have performed 3 assessments
    let assessments = stats
        .security_assessments
        .load(std::sync::atomic::Ordering::Relaxed);
    assert_eq!(assessments, 3, "Expected 3 assessments, got {assessments}",);

    // Other stats should be reasonable
    let threats_analyzed = stats
        .threats_analyzed
        .load(std::sync::atomic::Ordering::Relaxed);
    let capabilities_monitored = stats
        .capabilities_monitored
        .load(std::sync::atomic::Ordering::Relaxed);

    println!("✅ Security Sentinel Statistics:");
    println!("   Security Assessments: {assessments}");
    println!("   Threats Analyzed: {threats_analyzed}");
    println!("   Capabilities Monitored: {capabilities_monitored}");
}

#[tokio::test]
async fn test_sovereignty_compliance() {
    let sentinel = SecuritySentinel::new();
    let report = sentinel.perform_security_assessment().await.unwrap();

    // BearDog must maintain sovereignty compliance
    assert!(
        report.sovereignty_status.sovereignty_score >= 0.8,
        "Sovereignty score below minimum threshold: {:.2}",
        report.sovereignty_status.sovereignty_score
    );

    // Check autonomy indicators
    for indicator in &report.sovereignty_status.autonomy_indicators {
        assert!(
            indicator.score >= 0.7,
            "Autonomy indicator '{}' below threshold: {:.2}",
            indicator.indicator_name,
            indicator.score
        );
    }

    // Independence score should be high
    assert!(
        report.sovereignty_status.independence_score >= 0.8,
        "Independence score below threshold: {:.2}",
        report.sovereignty_status.independence_score
    );

    println!(
        "✅ Sovereignty compliance verified - Score: {:.2}",
        report.sovereignty_status.sovereignty_score
    );
}

#[tokio::test]
async fn test_security_recommendations_generation() {
    let sentinel = SecuritySentinel::new();
    let report = sentinel.perform_security_assessment().await.unwrap();

    // Validate recommendation structure
    for recommendation in &report.recommendations {
        assert!(!recommendation.category.is_empty());
        assert!(!recommendation.description.is_empty());
        assert!(!recommendation.impact.is_empty());
        assert!(!recommendation.effort.is_empty());

        // Priority should be valid
        match recommendation.priority {
            RecommendationPriority::Critical
            | RecommendationPriority::High
            | RecommendationPriority::Medium
            | RecommendationPriority::Low => {
                // Valid priorities
            }
        }
    }

    println!(
        "✅ Security recommendations validated - {} recommendations generated",
        report.recommendations.len()
    );

    for (i, rec) in report.recommendations.iter().enumerate() {
        println!("   {}: {} - {}", i + 1, rec.category, rec.description);
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::security_sentinel_example::{
        integrate_security_sentinel_startup, security_sentinel_demo,
    };

    #[tokio::test]
    async fn test_security_sentinel_demo() {
        let result = security_sentinel_demo().await;
        assert!(
            result.is_ok(),
            "Security Sentinel demo should complete successfully"
        );
        println!("✅ Security Sentinel demo test passed");
    }

    #[tokio::test]
    async fn test_security_sentinel_integration() {
        let sentinel_result = integrate_security_sentinel_startup().await;
        assert!(
            sentinel_result.is_ok(),
            "Security Sentinel integration should work"
        );

        let sentinel = sentinel_result.unwrap();
        let stats = sentinel.get_sentinel_stats().await;

        // Should have performed at least one assessment during integration
        let assessments = stats
            .security_assessments
            .load(std::sync::atomic::Ordering::Relaxed);
        assert!(
            assessments > 0,
            "Integration should perform at least one assessment"
        );

        println!("✅ Security Sentinel integration test passed - {assessments} assessments",);
    }
}

// Performance and load testing
#[cfg(test)]
mod performance_tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn test_security_assessment_performance() {
        let sentinel = SecuritySentinel::new();

        let start = Instant::now();
        let report = sentinel.perform_security_assessment().await.unwrap();
        let duration = start.elapsed();

        // Security assessment should complete quickly (under 100ms for tests)
        assert!(
            duration.as_millis() < 100,
            "Security assessment took too long: {}ms",
            duration.as_millis()
        );

        // Should still produce valid results
        assert!(report.overall_security_score >= 0.0);
        assert!(report.overall_security_score <= 1.0);

        println!(
            "✅ Security assessment performance test passed - {}ms",
            duration.as_millis()
        );
    }

    #[tokio::test]
    async fn test_concurrent_assessments() {
        let sentinel = SecuritySentinel::new();

        // Run multiple assessments concurrently
        let mut handles = vec![];

        for i in 0..10 {
            let sentinel_clone = sentinel.clone();
            let handle = tokio::spawn(async move {
                let report = sentinel_clone.perform_security_assessment().await.unwrap();
                (i, report.overall_security_score)
            });
            handles.push(handle);
        }

        // Wait for all assessments to complete
        let mut results = vec![];
        for handle in handles {
            results.push(handle.await.unwrap());
        }

        assert_eq!(
            results.len(),
            10,
            "All concurrent assessments should complete"
        );

        // All results should be valid
        for (i, score) in results {
            assert!(
                (0.0..=1.0).contains(&score),
                "Assessment {i} invalid score: {score:.2}",
            );
        }

        println!("✅ Concurrent assessments test passed - 10 assessments completed");
    }
}
