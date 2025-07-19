//! Core initialization and health check tests

use super::common::*;
use beardog::errors::BearDogResult;
use beardog::core::HealthStatus;

#[tokio::test]
async fn test_beardog_initialization() -> BearDogResult<()> {
    let core = create_test_core().await?;

    // Test health check
    let health = core.health_check().await?;
    assert!(matches!(health.status, HealthStatus::Healthy));
    assert!(!health.components.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_core_initialization() -> BearDogResult<()> {
    let core = create_test_core().await?;
    
    // Verify core is properly initialized
    let health = core.health_check().await?;
    assert!(matches!(health.status, HealthStatus::Healthy));
    
    // Check that all expected components are present
    let component_names: Vec<_> = health.components.iter().map(|c| &c.name).collect();
    
    // Verify key components are initialized
    assert!(component_names.contains(&&"core".to_string()));
    
    Ok(())
}

#[tokio::test]
async fn test_config_validation() -> BearDogResult<()> {
    let config = create_test_config();
    
    // Test that configuration is valid
    assert!(!config.api.bind_address.is_empty());
    assert_eq!(config.database.url, ":memory:");
    assert!(config.threat_detection.enabled);
    
    Ok(())
}

#[tokio::test]
async fn test_config_serialization() -> BearDogResult<()> {
    let config = create_test_config();
    
    // Test that configuration can be serialized and deserialized
    let serialized = serde_json::to_string(&config)?;
    let _deserialized: beardog::config::core::BearDogConfig = serde_json::from_str(&serialized)?;
    
    Ok(())
}

#[tokio::test]
async fn test_error_types() {
    use beardog::errors::BearDogError;
    
    // Test error type creation and formatting
    let config_error = BearDogError::config("Test configuration error");
    assert!(format!("{:?}", config_error).contains("Test configuration error"));
    
    let internal_error = BearDogError::internal("Test internal error");
    assert!(format!("{:?}", internal_error).contains("Test internal error"));
}

#[tokio::test]
async fn test_module_imports() {
    // Test that all main modules can be imported without issues
    use beardog::config::*;
    use beardog::core::*;
    use beardog::security::*;
    use beardog::compliance::*;
    use beardog::audit::*;
    use beardog::threat::*;
    use beardog::workflows::*;
    
    // If we get here without compilation errors, the imports work
    assert!(true);
} 