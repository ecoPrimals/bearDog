//! Production Health Monitoring Tests
//!
//! Tests for system health monitoring, component health checks,
//! health endpoints, liveness/readiness probes, and alert systems.

use beardog::production::*;

/// Test production health monitoring systems
pub async fn test_health_monitoring(prod_manager: &mut ProductionManager) {
    println!("💓 Testing health monitoring systems...");

    // Test system health monitoring
    let system_health = prod_manager
        .get_system_health_status()
        .await
        .expect("System health check should succeed");

    assert_eq!(
        system_health.overall_status,
        HealthStatus::Healthy,
        "System should be healthy"
    );
    assert!(
        system_health.cpu_utilization >= 0.0 && system_health.cpu_utilization <= 1.0,
        "CPU utilization should be valid percentage"
    );
    assert!(
        system_health.memory_utilization >= 0.0 && system_health.memory_utilization <= 1.0,
        "Memory utilization should be valid percentage"
    );
    assert!(
        system_health.disk_utilization >= 0.0 && system_health.disk_utilization <= 1.0,
        "Disk utilization should be valid percentage"
    );
    assert!(
        system_health.network_connectivity,
        "Network should be connected"
    );

    // Test component health monitoring
    let component_health = prod_manager
        .get_component_health_status()
        .await
        .expect("Component health check should succeed");

    let required_components = vec![
        "core_engine",
        "encryption_engine",
        "compliance_engine",
        "audit_engine",
        "monitoring_engine",
        "security_provider",
    ];

    for component in required_components {
        assert!(
            component_health.components.contains_key(component),
            "Component {} should be monitored",
            component
        );

        let status = &component_health.components[component];
        assert!(
            matches!(
                status.health,
                ComponentHealth::Healthy | ComponentHealth::Warning
            ),
            "Component {} should be healthy or warning",
            component
        );
        assert!(
            status.uptime_seconds > 0,
            "Component {} should have uptime",
            component
        );
    }

    // Test health check endpoint
    let health_endpoint = prod_manager
        .test_health_endpoint()
        .await
        .expect("Health endpoint test should succeed");

    assert_eq!(
        health_endpoint.status_code, 200,
        "Health endpoint should return 200"
    );
    assert!(
        health_endpoint.response_time_ms < 1000,
        "Health check should be fast"
    );
    assert!(
        health_endpoint.response_valid,
        "Health response should be valid JSON"
    );

    // Test liveness and readiness probes
    let liveness_probe = prod_manager
        .test_liveness_probe()
        .await
        .expect("Liveness probe should succeed");

    assert!(liveness_probe.is_alive, "System should be alive");
    assert!(liveness_probe.core_responding, "Core should be responding");
    assert!(
        liveness_probe.critical_processes_running,
        "Critical processes should be running"
    );

    let readiness_probe = prod_manager
        .test_readiness_probe()
        .await
        .expect("Readiness probe should succeed");

    assert!(readiness_probe.is_ready, "System should be ready");
    assert!(
        readiness_probe.accepting_requests,
        "Should be accepting requests"
    );
    assert!(
        readiness_probe.dependencies_available,
        "Dependencies should be available"
    );

    // Test health monitoring alerts
    let monitoring_config = MonitoringConfiguration {
        cpu_threshold: 0.8,
        memory_threshold: 0.85,
        disk_threshold: 0.9,
        response_time_threshold_ms: 5000,
        error_rate_threshold: 0.01,
        alert_cooldown_seconds: 300,
    };

    let alert_system = prod_manager
        .configure_health_alerts(&monitoring_config)
        .await
        .expect("Alert configuration should succeed");

    assert!(
        alert_system.alerts_configured,
        "Health alerts should be configured"
    );
    assert!(
        alert_system.notification_channels.len() > 0,
        "Should have notification channels"
    );
    assert!(
        alert_system.escalation_procedures_defined,
        "Escalation should be defined"
    );
}

#[tokio::test]
async fn test_health_monitoring_standalone() {
    use beardog::core::*;
    use std::sync::Arc;
    
    let config = BearDogConfig::production();
    let core = Arc::new(
        BearDogCore::new(config)
            .await
            .expect("Core initialization failed"),
    );

    let mut production_manager = ProductionManager::new(core.clone())
        .await
        .expect("Production manager creation failed");
        
    test_health_monitoring(&mut production_manager).await;
} 