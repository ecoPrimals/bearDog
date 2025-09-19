use beardog_errors::ecosystem_simple::{SimpleEcosystemConfig, SimpleEcosystemManager};
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;

use beardog_adapters::universal::vendor_adapter::{UniversalVendorAdapter, UniversalVendorRequest};
use beardog_types::canonical::capabilities::CapabilityType;
use std::collections::HashMap;

#[tokio::test]
async fn test_ecosystem_manager_initialization() -> Result<(), BearDogError> {
    let config = SimpleEcosystemConfig::default();
    let mut manager = SimpleEcosystemManager::new(config);

    manager.initialize()?;

    let services = manager.get_all_services();
    assert!(!services.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_service_discovery() -> Result<(), BearDogError> {
    let config = SimpleEcosystemConfig {
        enabled_services: vec!["compute-service".to_string(),
        health_check_interval_ms: 5000,
        retry_attempts: 2,
    };

    let mut manager = SimpleEcosystemManager::new(config);
    manager.initialize()?;

    let toadstool_service = manager.get_service_status("compute-service");
    assert!(toadstool_service.is_some());

    if let Some(service) = toadstool_service {
        assert_eq!(service.service_name, "compute-service");
        assert!(!service.endpoint.is_empty());
        assert!(!service.capabilities.is_empty());
    }

    Ok(())
}

#[tokio::test]
async fn test_health_check_all_services() -> Result<(), BearDogError> {
    let config = SimpleEcosystemConfig::default();
    let mut manager = SimpleEcosystemManager::new(config);

    manager.initialize()?;

    let health_results = manager.health_check_all();
    assert!(!health_results.is_empty());

    for (service_name, health_status) in &health_results {
        assert!(!service_name.is_empty());
        assert!(matches!(
            health_status,
            HealthStatus::Healthy | HealthStatus::Degraded | HealthStatus::Unhealthy
        ));
    }

    Ok(())
}

#[tokio::test]
async fn test_service_execution() -> Result<(), BearDogError> {
    let config = SimpleEcosystemConfig::default();
    let mut manager = SimpleEcosystemManager::new(config);

    manager.initialize()?;

    let payload = serde_json::json!({"test": "data"});
    let result = manager
        .execute_on_service("compute-service", "test_operation", payload)
        ?;

    assert!(result.is_object());

    Ok(())
}

#[tokio::test]
async fn test_unknown_service_handling() {
    let config = SimpleEcosystemConfig::default();
    let mut manager = SimpleEcosystemManager::new(config);

    let result = manager.initialize();
    assert!(result.is_ok());

    let unknown_service = manager.get_service_status("unknown_service");
    assert!(unknown_service.is_none());
}

#[tokio::test]
async fn test_service_capabilities() -> Result<(), BearDogError> {
    let config = SimpleEcosystemConfig::default();
    let mut manager = SimpleEcosystemManager::new(config);

    manager.initialize()?;

    let adapter_config = create_universal_adapter_config();
    let adapter = UniversalVendorAdapter::new(adapter_config)?;

    let available_capabilities = adapter.discover_available_capabilities()?;

    for capability in &available_capabilities {
        let test_request = UniversalVendorRequest {
            request_id: uuid::Uuid::new_v4().to_string(),
            required_capability: capability.clone(),
            parameters: json!({"test": "capability_check"}).into(beardog_adapters::universal::vendor_adapter::RequestPriority::Normal,
            timeout_seconds: 30,
            security_context: SecurityContext::default(),
        };

        let result = adapter.route_capability_request(test_request);
        match result {
            Ok(_) => println!("✅ Capability {:?} is available", capability),
            Err(e) => println!("⚠️ Capability {:?} unavailable: {}", capability, e),
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_config_customization() -> Result<(), BearDogError> {
    let adapter_config = create_universal_adapter_config();
    let adapter = UniversalVendorAdapter::new(capability_providers, // Dynamic based on what's actually available
        discovery_timeout_ms: 2000,
        health_check_interval_ms: 10000,
        retry_attempts: 5,
    };

    let mut manager = SimpleEcosystemManager::new(custom_config);
    manager.initialize()?;

    let services = manager.get_all_services();

    for service_name in &custom_config.enabled_services {
        if let Some(service) = services.get(service_name) {
            assert!(
                !service.capabilities.is_empty(),
                "Service {} should have capabilities",
                service_name
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_service_endpoint_configuration() -> Result<(), BearDogError> {
    let config = SimpleEcosystemConfig::default();
    let mut manager = SimpleEcosystemManager::new(config);

    manager.initialize()?;

    for (service_name, service) in manager.get_all_services() {
        assert!(
            service.endpoint.starts_with("http://"),
            "Service {} should have HTTP endpoint, got: {}",
            service_name,
            service.endpoint
        );
        assert!(
            service.endpoint.contains(service_name),
            "Service {} endpoint should contain service name",
            service_name
        );
    }

    Ok(())
}

fn create_universal_adapter_config(
) -> beardog_adapters::universal::vendor_adapter::UniversalAdapterConfig {
    beardog_adapters::universal::vendor_adapter::UniversalAdapterConfig::default()
}
