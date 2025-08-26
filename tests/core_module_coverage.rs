

use beardog::config::*;
use beardog::core::*;
use beardog::error::*;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn test_beardog_config_validation() -> BearDogResult<()> {

    let mut config = BearDogConfig::default();

    assert!(config.validate().is_ok(), "Default config should be valid");

    config.network.port = 99999; // Invalid port
    assert!(
        config.validate().is_err(),
        "Invalid port should fail validation"
    );

    config = BearDogConfig::default();
    config.security.level = SecurityLevel::Maximum;
    assert!(
        config.validate().is_ok(),
        "Maximum security should be valid"
    );

    config.security.level = SecurityLevel::Standard;
    assert!(
        config.validate().is_ok(),
        "Standard security should be valid"
    );

    Ok(())
}

#[tokio::test]
async fn test_beardog_core_lifecycle() -> BearDogResult<()> {

    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;

    let health = core.health_check().await?;
    assert_eq!(health.status, HealthStatus::Starting);

    core.start().await?;

    let health_after_start = core.health_check().await?;
    assert!(matches!(
        health_after_start.status,
        HealthStatus::Healthy | HealthStatus::Degraded
    ));

    core.stop().await?;

    Ok(())
}

#[test]
fn test_error_type_coverage() {

    let errors = vec![
        BearDogError::configuration("test config error".to_string(),
        ),
        BearDogError::configuration("test db error".to_string(),
        ),
        BearDogError::encryption("test_operation".to_string(), "test encryption error".to_string(),
        ),
        BearDogError::authentication("test auth error".to_string(),
        ),
        BearDogError::invalid_input("test authz error".to_string(),
        ),
        BearDogError::network("test network error".to_string(),
        ),
        BearDogError::validation("test genetics error".to_string(),
        ),
        BearDogError::SpawnRejected {
            reason: "test spawn rejection".to_string(),
        },
        BearDogError::OperationTimeout {
            operation: "test-workflow".to_string(),
        },
        BearDogError::OperationTimeout {
            operation: "test-operation".to_string(),
        },
        BearDogError::Compliance {
            standard: "test-violation".to_string(),
            message: "test details".to_string(),
        },
        BearDogError::internal("test threat detection error".to_string(),
        ),
    ];

    for error in errors {
        let debug_str = format!("{error:?}");
        let display_str = format!("{error}");

        assert!(
            !debug_str.is_empty(),
            "Error debug string should not be empty"
        );
        assert!(
            !display_str.is_empty(),
            "Error display string should not be empty"
        );

        let std_error: &dyn std::error::Error = &error;
        assert!(!std_error.to_string().is_empty());
    }
}

#[tokio::test]
async fn test_component_status_management() -> BearDogResult<()> {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;

    core.update_component_status("test-component", true, None)
        .await?;
    core.update_component_status("test-component-2", false, Some("test error".to_string()))
        .await?;

    let health = core.health_check().await?;

    let component_names: Vec<&str> = health.components.iter().map(|c| c.name.as_str()).collect();

    println!("Component statuses: {component_names:?}");

    Ok(())
}

#[tokio::test]
async fn test_system_metrics_update() -> BearDogResult<()> {
    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;

    let test_metrics = SystemMetrics {
        memory_usage_bytes: 1024 * 1024, // 1MB
        cpu_usage_percent: 25.5,
        active_connections: 10,
        requests_per_second: 100.0,
        avg_response_time_ms: 50.0,
        error_rate_percent: 0.1,
    };

    core.update_metrics(test_metrics.clone()).await?;

    let health = core.health_check().await?;
    assert_eq!(health.metrics.memory_usage_bytes, 1024 * 1024);
    assert_eq!(health.metrics.cpu_usage_percent, 25.5);
    assert_eq!(health.metrics.active_connections, 10);

    Ok(())
}

#[test]
fn test_config_serialization() {

    let config = BearDogConfig::default();

    let json = serde_json::to_string(&config).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Should serialize to JSON", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Should serialize to JSON", e).to_string())
})?;
    let deserialized: BearDogConfig =
        serde_json::from_str(&json).map_err(|e| {
    tracing::error!("JSON parsing failed ({}): {}", "Should deserialize from JSON", e);
    beardog_errors::BearDogError::ValidationError(format_args!("JSON parsing error ({}): {}", "Should deserialize from JSON", e).to_string())
})?;

    assert_eq!(config.security.level, deserialized.security.level);
    assert_eq!(config.database.url, deserialized.database.url);
    assert_eq!(config.api.bind_address, deserialized.api.bind_address);
}

#[test]
fn test_config_defaults() {

    let config = BearDogConfig::default();

    assert_eq!(config.security.level, SecurityLevel::High);
    assert!(config.security.token_expiration_seconds > 0);
    assert!(config.security.max_failed_logins > 0);

    assert!(!config.database.url.is_empty());
    assert!(config.database.max_connections > 0);

    assert!(!config.network.host.is_empty());
    assert!(config.network.port > 0);

    assert!(!config.encryption.default_algorithm.is_empty());
    assert!(config.encryption.key_derivation_iterations > 0);

    assert!(!config.api.bind_address.is_empty());
}

#[tokio::test]
async fn test_concurrent_core_operations() -> BearDogResult<()> {

    let config = BearDogConfig::default();
    let core = BearDogCore::new(config).await?;

    let core_clone1 = core.clone();
    let core_clone2 = core.clone();
    let core_clone3 = core.clone();

    let (result1, result2, result3) = tokio::join!(
        core_clone1.health_check(),
        core_clone2.update_component_status("concurrent-1", true, None),
        core_clone3.update_component_status("concurrent-2", true, None)
    );

    assert!(result1.is_ok(), "Concurrent health check should succeed");
    assert!(
        result2.is_ok(),
        "Concurrent component update 1 should succeed"
    );
    assert!(
        result3.is_ok(),
        "Concurrent component update 2 should succeed"
    );

    Ok(())
}

#[tokio::test]
async fn test_timeout_handling() -> BearDogResult<()> {

    let config = BearDogConfig::default();

    let result = timeout(Duration::from_secs(10), BearDogCore::new(config)).await;
    assert!(result.is_ok(), "Core initialization should not timeout");

    let core = result.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})??;

    let health_result = timeout(Duration::from_secs(5), core.health_check()).await;
    assert!(health_result.is_ok(), "Health check should not timeout");

    Ok(())
}
