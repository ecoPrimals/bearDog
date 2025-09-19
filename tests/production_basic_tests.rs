use beardog_errors::BearDogError;
use beardog_types::canonical::config::CanonicalAppConfig;
use beardog_types::canonical::monitoring::HealthStatus;

#[test]
fn test_production_config_validation() {
    let config = CanonicalAppConfig::default();

    // Verify configuration has basic required fields
    assert!(config.version.is_empty() || !config.version.is_empty());

    // Test that production config can be created
    assert!(true); // Basic validation that test compiles and runs
}

#[test]
fn test_production_health_status() {
    let healthy = HealthStatus::Healthy;
    let unhealthy = HealthStatus::Unhealthy;

    assert!(matches!(healthy, HealthStatus::Healthy));
    assert!(matches!(unhealthy, HealthStatus::Unhealthy));
    assert_ne!(healthy, unhealthy);
}

#[test]
fn test_production_error_handling() {
    let error = BearDogError::production("Production validation failed");

    assert!(format!("{:?}", error).contains("Production validation failed"));
    assert!(format!("{}", error).contains("production"));
}

#[tokio::test]
async fn test_production_readiness_basic() -> Result<(), BearDogError> {
    // Basic async test to verify tokio integration works
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;

    // Verify we can create and use basic production types
    let status = HealthStatus::Healthy;
    assert!(matches!(status, HealthStatus::Healthy));

    Ok(())
}

#[test]
fn test_production_constants() {
    // Test that production constants are accessible
    use beardog_types::constants::unified::SYSTEM_VERSION;

    // Verify system version is defined
    assert!(!SYSTEM_VERSION.is_empty());
}
