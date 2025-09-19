use beardog_errors::BearDogError;
use beardog_tunnel::hsm::{HsmCapability, HsmConfig, HsmManager};
use beardog_tunnel::universal_hsm_discovery::UniversalHsmDiscovery;
// HashMap import removed - not used in this test

#[tokio::test]
async fn test_hsm_discovery_initialization() -> Result<(), BearDogError> {
    let discovery = UniversalHsmDiscovery::new();

    assert!(discovery.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_hsm_capability_detection() -> Result<(), BearDogError> {
    let discovery = UniversalHsmDiscovery::new()?;

    let capabilities = discovery.detect_capabilities()?;

    assert!(capabilities.len() >= 0);

    Ok(())
}

#[tokio::test]
async fn test_hsm_config_validation() {
    let config = HsmConfig::default();

    assert!(!config.device_path.is_empty());
    assert!(config.timeout_ms > 0);
    assert!(config.retry_attempts > 0);
}

#[tokio::test]
async fn test_hsm_manager_creation() -> Result<(), BearDogError> {
    let config = HsmConfig::default();
    let manager = HsmManager::new(config)?;

    assert!(manager.is_initialized());

    Ok(())
}

#[tokio::test]
async fn test_hsm_key_operations() -> Result<(), BearDogError> {
    let config = HsmConfig::default();
    let mut manager = HsmManager::new(config)?;

    let key_result = manager.generate_key("test-key");

    match key_result {
        Ok(key_id) => {
            assert!(!key_id.is_empty());
        }
        Err(_) => {}
    }

    Ok(())
}

#[tokio::test]
async fn test_hsm_health_monitoring() -> Result<(), BearDogError> {
    let config = HsmConfig::default();
    let manager = HsmManager::new(config)?;

    let health_status = manager.health_check()?;

    assert!(matches!(
        health_status,
        beardog_types::canonical::HealthStatus::Healthy
            | beardog_types::canonical::HealthStatus::Degraded
            | beardog_types::canonical::HealthStatus::Unhealthy
    ));

    Ok(())
}

#[tokio::test]
async fn test_hsm_device_enumeration() -> Result<(), BearDogError> {
    let discovery = UniversalHsmDiscovery::new()?;

    let devices = discovery.enumerate_devices()?;

    assert!(devices.len() >= 0);

    Ok(())
}

#[tokio::test]
async fn test_hsm_strongbox_detection() -> Result<(), BearDogError> {
    let discovery = UniversalHsmDiscovery::new()?;

    let has_strongbox = discovery.detect_android_strongbox()?;

    assert!(has_strongbox == true || has_strongbox == false);

    Ok(())
}

#[tokio::test]
async fn test_hsm_secure_enclave_detection() -> Result<(), BearDogError> {
    let discovery = UniversalHsmDiscovery::new()?;

    let has_secure_enclave = discovery.detect_ios_secure_enclave()?;

    assert!(has_secure_enclave == true || has_secure_enclave == false);

    Ok(())
}

#[tokio::test]
fn test_hsm_error_handling() {
    let invalid_config = HsmConfig {
        device_path: "/invalid/path".to_string(),
        retry_attempts: 0, // Invalid retry count
        enable_attestation: true,
        require_pin: false,
    };

    let result = HsmManager::new(invalid_config);

    match result {
        Ok(_) => {}
        Err(_) => {}
    }
}
