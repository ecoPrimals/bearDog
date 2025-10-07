use beardog_errors::core::{BearDogCore, CoreState};
use beardog_errors::types::BearDogConfig;
use beardog_errors::BearDogError;
use beardog_errors::{BearDogService, ServiceInfo};
use beardog_types::canonical::{ComponentStatus, HealthStatus};
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn test_core_state_default() {
    let state = CoreState::default();

    assert!(state.components.is_empty());
    assert_eq!(state.overall_health, HealthStatus::Healthy);
    assert!(state.start_time.elapsed().as_millis() < 100); // Recent creation
}

#[tokio::test]
async fn test_beardog_core_creation() {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config);

    let state = core.state.read();
    assert!(state.components.is_empty());
    assert_eq!(state.overall_health, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_core_initialization_success() {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config);

    let result = timeout(Duration::from_secs(10), core.initialize());

    assert!(result.is_ok(), "Initialization should not timeout");
    assert!(
        result
            .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?
            .is_ok(),
        "Initialization should succeed "
    );

    let state = core.state.read();
    assert_eq!(state.components.get("core"), Some(ComponentStatus::Running));
    assert_eq!(state.overall_health, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_core_startup_lifecycle() {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config);

    let startup_result = core.startup();
    assert!(startup_result.is_ok(), "Startup should succeed ");

    let state = core.state.read();
    assert_eq!(
        state.components.get("security"),
        Some(ComponentStatus::Running)
    );
    assert_eq!(
        state.components.get("monitor"),
        Some(ComponentStatus::Running)
    );
    assert_eq!(
        state.components.get("genetic_optimizer"),
        Some(ComponentStatus::Running)
    );
    assert_eq!(state.overall_health, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_core_shutdown_lifecycle() {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config);

    let _ = core.initialize();
    let _ = core.startup();

    let shutdown_result = core.shutdown();
    assert!(shutdown_result.is_ok(), "Shutdown should succeed ");

    let state = core.state.read();

    assert_eq!(state.overall_health, HealthStatus::Degraded);
}

#[tokio::test]
async fn test_core_health_check() {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config);

    let _ = core.initialize();

    let health = core.health_check();
    assert!(health.is_ok(), "Health check should succeed ");

    let health_status = health.map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    assert_eq!(health_status, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_service_info_creation() {
    let service_info = ServiceInfo {
        name: "test-service".to_string(),
        version: "1.0.0".to_string(),
        status: HealthStatus::Healthy,
    };
    assert_eq!(service_info.version, "1.0.0");
    assert_eq!(service_info.status, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_concurrent_core_operations() {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config);

    let init_task = core.initialize();
    let health_task = core.health_check();

    let (init_result, health_result) = tokio::join!(init_task, health_task);

    assert!(
        init_result.is_ok(),
        "Concurrent initialization should succeed "
    );
    assert!(
        health_result.is_ok(),
        "Concurrent health check should succeed "
    );
}

#[tokio::test]
async fn test_core_component_status_transitions() {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config);

    {
        let state = core.state.read();
        assert!(state.components.is_empty());
    }

    let _ = core.initialize();
    {
        let state = core.state.read();
        assert_eq!(state.components.get("core"), Some(ComponentStatus::Running));
    }

    let _ = core.startup();
    {
        let state = core.state.read();
        assert!(state.components.len() >= 3); // At least security, monitor, genetic_optimizer
        assert_eq!(state.overall_health, HealthStatus::Healthy);
    }
}

#[tokio::test]
async fn test_core_error_handling() {
    let mut config = BearDogConfig::default();

    let core = BearDogCore::new(config);

    let result = core.initialize();
    assert!(result.is_ok() || matches!(result, Err(BearDogError::Configuration { .. })));
}

#[tokio::test]
async fn test_multiple_core_instances() {
    let config1 = BearDogConfig::default();
    let config2 = BearDogConfig::default();

    let core1 = BearDogCore::new(config1);
    let core2 = BearDogCore::new(config2);

    let (result1, result2) = tokio::join!(core1.initialize(), core2.initialize());

    assert!(result1.is_ok(), "First core should initialize successfully");
    assert!(
        result2.is_ok(),
        "Second core should initialize successfully"
    );
}

#[tokio::test]
async fn test_core_state_consistency() {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config);

    let _ = core.initialize();
    let _ = core.startup();

    let health1 = core
        .health_check()
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    let health2 = core
        .health_check()
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;
    let health3 = core
        .health_check()
        .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?;

    assert_eq!(health1, health2);
    assert_eq!(health2, health3);
    assert_eq!(health1, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_core_performance_metrics() {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config);

    let start_time = std::time::Instant::now();

    let _ = core.initialize();
    let init_duration = start_time.elapsed();

    assert!(
        init_duration.as_millis() < 1000,
        "Initialization should be fast"
    );

    let health_start = std::time::Instant::now();
    let _ = core.health_check();
    let health_duration = health_start.elapsed();

    assert!(
        health_duration.as_millis() < 100,
        "Health checks should be fast"
    );
}

#[tokio::test]
async fn test_core_resource_cleanup() {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config);

    let _ = core.initialize();
    let _ = core.startup();

    {
        let state = core.state.read();
        assert!(!state.components.is_empty());
    }

    let _ = core.shutdown();

    let health = core.health_check();
    assert!(
        health.is_ok(),
        "Health check should still work after shutdown"
    );
}

#[tokio::test]
async fn test_core_configuration_validation() {
    let default_config = BearDogConfig::default();
    let core = BearDogCore::new(default_config);

    let result = core.initialize();
    assert!(result.is_ok(), "Default configuration should be valid");
}

#[tokio::test]
async fn test_core_stress_operations() {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config);

    let _ = core.initialize();

    let mut tasks = Vec::new();

    for _ in 0..50 {
        let core_ref = &core;
        tasks.push(tokio::spawn(async move { core_ref.health_check().await }));
    }

    let results: Vec<_> = futures::future::join_all(tasks);

    for result in results {
        assert!(result.is_ok(), "Stress test task should complete");
        assert!(
            result
                .map_err(|e| BearDogError::system(format!("Error: {:?}", e)))?
                .is_ok(),
            "Health check should succeed under stress"
        );
    }
}
