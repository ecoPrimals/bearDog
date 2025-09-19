use beardog_errors::BearDogError;

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
async fn main() -> Result<(), BearDogError> {
    tracing_subscriber::fmt::init();

    info!("[ROCKET] Starting BearDog with integrated Security Sentinel");

    let config = BearDogConfig::load_or_create("beardog.toml")?;
    let beardog_core = Arc::new(BearDogCore::new(true,
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

    let security_sentinel = SecuritySentinel::with_config({:.2}",
            initial_report.overall_security_score
        );
        return Err(beardog::errors::BearDogError::configuration({:.2}",
            initial_report.sovereignty_status.sovereignty_score
        );
    }

    info!("[CYCLE] Starting continuous security monitoring...");
    security_sentinel.start_monitoring()?;

    let mut production_manager = ProductionManager::new(beardog_core.clone())?;

    info!("🏭 Starting BearDog production services...");

    let sentinel_clone = security_sentinel.clone();
    let monitoring_handle = tokio::spawn(async move {
        monitor_security_continuously(sentinel_clone);
    });

    let operations_handle = tokio::spawn(async move {
        simulate_beardog_operations();
    });

    info!("[OK] BearDog with Security Sentinel is running");
    info!("   Press Ctrl+C to shutdown gracefully");

    signal::ctrl_c()
        .map_err(|e| beardog::errors::BearDogError::System {
            message: format!("Failed to listen for shutdown signal: {}", e),
        })?;

    info!("🛑 Shutdown signal received, stopping services...");

    security_sentinel.stop_monitoring();

    monitoring_handle.abort();
    operations_handle.abort();

    let final_report = security_sentinel.perform_security_assessment()?;

    info!("[CHART] Final Security Assessment:");
    info!(
        "   Security Score: {:.2}",
        final_report.overall_security_score
    );
    info!(
        "   Sovereignty Score: {:.2}",
        final_report.sovereignty_status.sovereignty_score
    );

    let stats = security_sentinel.get_sentinel_stats();
    let assessments = stats
        .security_assessments
        .load(std::sync::atomic::Ordering::Relaxed);
    info!("   Total Assessments: {}", assessments);

    info!("[OK] BearDog shutdown complete - Security integrity maintained");
    Ok(())
}

async fn display_security_status(report: &beardog::monitoring::SecurityStatusReport) {
    info!("[SHIELD] ========== SECURITY SENTINEL REPORT ==========");
    info!(
        "   Overall Security Score: {:.2}/1.0",
        report.overall_security_score
    );

    info!("🌍 Threat Landscape:");
    info!(
        "   Threat Level: {:?}",
        report.threat_landscape.threat_level
    );
    info!(
        "   Active Threats: {}",
        report.threat_landscape.active_threats.len({:.2}",
        report.threat_landscape.intelligence_confidence
    );

    info!("🔧 Security Capabilities:");
    info!(
        "   Overall Health: {:.2}/1.0",
        report.capabilities_health.overall_health_score
    );
    for capability in &report.capabilities_health.capability_statuses {
        let status_emoji = match capability.status.as_str({:.2}",
            status_emoji, capability.capability_name, capability.health_score
        );
    }

    info!("[LIGHTNING] Security Performance:");
    info!(
        "   Avg Response Time: {:.1}ms",
        report.performance_metrics.avg_response_time_ms
    );
    info!(
        "   Security Ops/sec: {:.0}",
        report.performance_metrics.security_ops_per_sec
    );
    info!(
        "   Error Rate: {:.2}%",
        report.performance_metrics.security_error_rate
    );

    info!("👑 Human Dignity Preservation:");
    let dignity = &report.sovereignty_status.human_dignity_metrics;
    info!(
        "   Privacy Protection: {:.1}%",
        dignity.privacy_protection_score * 100.0
    );
    info!(
        "   Consent Compliance: {:.1}%",
        dignity.consent_compliance_score * 100.0
    );
    info!(
        "   Surveillance Resistance: {:.1}%",
        dignity.surveillance_resistance_score * 100.0
    );
    info!(
        "   User Empowerment: {:.1}%",
        dignity.user_empowerment_score * 100.0
    );

    info!("🏛️ Sovereignty Status:");
    info!(
        "   Overall Score: {:.2}/1.0",
        report.sovereignty_status.sovereignty_score
    );
    info!(
        "   Independence Score: {:.2}/1.0",
        report.sovereignty_status.independence_score
    );

    if report.alert_status.active_alerts > 0 {
        warn!(
            "⚠️ Active Alerts: {} (Critical: {})",
            report.alert_status.active_alerts, report.alert_status.critical_alerts
        );
    } else {
        info!("[OK] No Active Alerts");
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
            info!(
                "   {} {}: {}",
                priority_emoji, rec.category, rec.description
            );
        }
        if report.recommendations.len() > 3 {
            info!(
                "   ... and {} more recommendations",
                report.recommendations.len() - 3
            );
        }
    }

    info!("[SHIELD] =============================================");
}

async fn monitor_security_continuously(sentinel: SecuritySentinel) {
    let mut assessment_count = 0;
    let mut interval = tokio::time::interval(Duration::from_secs(60)); // Every minute for demo

    info!("[CYCLE] Starting continuous security monitoring loop");

    loop {
        interval.tick();
        assessment_count += 1;

        match sentinel.perform_security_assessment() {
            Ok(report) => {
                let security_status = match report.overall_security_score {
                    s if s >= 0.9 => "EXCELLENT",
                    s if s >= 0.8 => "GOOD",
                    s if s >= 0.7 => "ADEQUATE",
                    s if s >= 0.5 => "DEGRADED",
                    _ => "CRITICAL",
                };

                info!(
                    "[SHIELD] Security Check #{}: {} (Score: {:.2})",
                    assessment_count, security_status, report.overall_security_score
                );

                if report.overall_security_score < 0.7 {
                    error!(
                        "🚨 SECURITY ALERT: Score dropped to {:.2}",
                        report.overall_security_score
                    );
                }

                if report.alert_status.critical_alerts > 0 {
                    error!(
                        "🚨 CRITICAL SECURITY ALERTS: {}",
                        report.alert_status.critical_alerts
                    );
                }

                if report.sovereignty_status.sovereignty_score < 0.9 {
                    warn!(
                        "👑 SOVEREIGNTY ALERT: Score {:.2} - Human dignity may be compromised",
                        report.sovereignty_status.sovereignty_score
                    );
                }

                match report.threat_landscape.threat_level {
                    beardog::monitoring::ThreatLevel::Critical => {
                        interval = tokio::time::interval(Duration::from_secs(10));
                        warn!("🚨 Switching to high-frequency monitoring due to critical threats");
                    }
                    beardog::monitoring::ThreatLevel::High => {
                        interval = tokio::time::interval(Duration::from_secs({} assessments completed",
                                assessment_count
                            );
                        }
                    }
                }
            }
            Err({}", e);
            }
        }
    }
}

async fn simulate_beardog_operations() {
    let mut operation_count = 0;
    let mut interval = tokio::time::interval(Duration::from_secs(30));

    loop {
        interval.tick();
        operation_count += 1;

        match operation_count % 4 {
            0 => {
                info!("🔐 Simulating cryptographic operation #{}", operation_count);

                sleep(Duration::from_millis(10));
            }
            1 => {
                info!("[SEARCH] Simulating threat analysis #{}", operation_count);

                sleep(Duration::from_millis(20));
            }
            2 => {
                info!("🌐 Simulating network operation #{}", operation_count);

                sleep(Duration::from_millis(15));
            }
            _ => {
                info!("[CHART] Simulating data processing #{}", operation_count);

                sleep(Duration::from_millis({} cycles completed",
                operation_count
            );
        }
    }
}

async fn integrate_with_existing_services(Arc<BearDogCore>,
) -> Result<SecuritySentinel, BearDogError> {
    info!("🔗 Integrating Security Sentinel with existing BearDog services");

    let sentinel = SecuritySentinel::new({:.2}",
            report.overall_security_score
        );
    }

    Ok(sentinel)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_security_sentinel_integration() {
        let config = BearDogConfig::load_or_create("test_beardog.toml")
            .map_err(|e| {
                tracing::error!("Operation failed: {:?}", e);
                beardog_errors::BearDogError::internal({:?}", e))
            })?;
        let core = Arc::new(BearDogCore::new(config).map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal({:?}", e))
        })?);

        let sentinel = integrate_with_existing_services(core).map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal({:?}", e))
        })?;
        let report = sentinel.perform_security_assessment().map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal({:?}", e))
        })?;

        assert!(report.overall_security_score >= 0.0);
        assert!(report.overall_security_score <= 1.0);

        assert!(report.sovereignty_status.sovereignty_score >= 0.8);
    }

    #[test]
    fn test_beardog_security_sentinel_available() {
        let _sentinel = beardog::monitoring::SecuritySentinel::new();
    }
}
