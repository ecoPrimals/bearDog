// BearDog core health check tests

use beardog_core::BearDogConfig;
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;

#[tokio::test]
async fn test_config_default_creation() {
    let config = BearDogConfig::default();
    // If this compiles and runs, default config works
    drop(config);
}

#[tokio::test]
async fn test_health_status_healthy() {
    let status = HealthStatus::Healthy;
    assert_eq!(status, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_health_status_degraded() {
    let status = HealthStatus::Degraded;
    assert_eq!(status, HealthStatus::Degraded);
}

#[tokio::test]
async fn test_health_status_unhealthy() {
    let status = HealthStatus::Unhealthy;
    assert_eq!(status, HealthStatus::Unhealthy);
}

#[tokio::test]
async fn test_health_status_comparison() {
    assert_ne!(HealthStatus::Healthy, HealthStatus::Degraded);
    assert_ne!(HealthStatus::Degraded, HealthStatus::Unhealthy);
    assert_eq!(HealthStatus::Healthy, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_health_status_clone() {
    let status = HealthStatus::Healthy;
    let cloned = status;
    assert_eq!(status, cloned);
}

#[tokio::test]
async fn test_config_clone() {
    let config1 = BearDogConfig::default();
    let config2 = config1.clone();
    // Both configs should be equal
    drop(config1);
    drop(config2);
}

#[test]
fn test_error_system_sync() {
    let error = BearDogError::system("sync test".to_string());
    assert!(format!("{:?}", error).contains("sync test"));
}

#[test]
fn test_error_validation_sync() {
    let error = BearDogError::validation("validation test");
    assert!(format!("{:?}", error).contains("validation test"));
}

#[test]
fn test_error_security_sync() {
    let error = BearDogError::security("security test".to_string());
    assert!(format!("{:?}", error).contains("security test"));
}

#[test]
fn test_error_network_sync() {
    let error = BearDogError::network("network test".to_string());
    assert!(format!("{:?}", error).contains("network test"));
}

#[test]
fn test_error_configuration_sync() {
    let error = BearDogError::configuration("config test");
    assert!(format!("{:?}", error).contains("config test"));
}

#[test]
fn test_error_not_found_sync() {
    let error = BearDogError::not_found("not found test".to_string());
    assert!(format!("{:?}", error).contains("not found"));
}

#[test]
fn test_error_unauthorized_sync() {
    let error = BearDogError::unauthorized("unauthorized test".to_string());
    assert!(format!("{:?}", error).contains("unauthorized"));
}

#[test]
fn test_error_invalid_input_sync() {
    let error = BearDogError::invalid_input("invalid input test");
    assert!(format!("{:?}", error).contains("invalid input"));
}

#[test]
fn test_error_unavailable_sync() {
    let error = BearDogError::unavailable("unavailable test".to_string());
    assert!(format!("{:?}", error).contains("unavailable"));
}

#[test]
fn test_error_internal_sync() {
    let error = BearDogError::internal("internal test".to_string());
    assert!(format!("{:?}", error).contains("internal"));
}

#[test]
fn test_error_business_sync() {
    let error = BearDogError::business("business test".to_string());
    assert!(format!("{:?}", error).contains("business"));
}

#[test]
fn test_error_api_sync() {
    let error = BearDogError::api("api test".to_string());
    assert!(format!("{:?}", error).contains("api"));
}

#[test]
fn test_error_workflow_sync() {
    let error = BearDogError::workflow("workflow test".to_string());
    assert!(format!("{:?}", error).contains("workflow"));
}
