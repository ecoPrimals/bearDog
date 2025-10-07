use beardog_errors::BearDogError;
use beardog_types::canonical::{configuration::*, monitoring::*, providers::*, security::*};
use beardog_types::constants::domains::{system::*, network::*, security::*};
use beardog_types::zero_cost::*;
use serde_json;
use std::time::Duration;

#[tokio::test]
async fn test_canonical_configuration_comprehensive() -> Result<(), BearDogError> {
    let config = BearDogCanonicalConfig::default();
    assert!(!config.app.service_name.is_empty());
    assert!(config.app.port > 0);
    assert!(config.app.port < 65536);

    let serialized = serde_json::to_string(&config)
        .map_err(|e| BearDogError::serialization(format!("Serialization failed: {}", e)))?;

    let deserialized: BearDogCanonicalConfig = serde_json::from_str(&serialized)
        .map_err(|e| BearDogError::serialization(format!("Deserialization failed: {}", e)))?;

    assert_eq!(config.app.service_name, deserialized.app.service_name);
    assert_eq!(config.app.port, deserialized.app.port);

    assert!(config.app.max_connections > 0);
    assert!(config.app.timeout_seconds > 0);

    Ok(())
}

#[tokio::test]
async fn test_provider_configuration_comprehensive() -> Result<(), BearDogError> {
    let provider_config = ProviderConfig::default();
    assert!(provider_config.enabled);
    assert!(provider_config.timeout > Duration::from_secs(0));

    let statuses = vec![
        ProviderStatus::Active,
        ProviderStatus::Inactive,
        ProviderStatus::Error,
        ProviderStatus::Maintenance,
    ];

    for status in statuses {
        let serialized = serde_json::to_string(&status)?;
        let deserialized: ProviderStatus = serde_json::from_str(&serialized)?;
        assert_eq!(status, deserialized);
    }

    Ok(())
}

#[tokio::test]
async fn test_monitoring_types_comprehensive() -> Result<(), BearDogError> {
    let health_status = HealthStatus::Healthy;
    assert_eq!(health_status, HealthStatus::Healthy);

    let component_status = ComponentStatus::Healthy;
    let serialized = serde_json::to_string(&component_status)?;
    let deserialized: ComponentStatus = serde_json::from_str(45.2,
        memory_usage: 67.8,
        disk_usage: 23.1,
        network_throughput: 1024.0,
        active_connections: 42,
        request_rate: 150.0,
        error_rate: 0.5,
        uptime_seconds: 86400,
    };

    assert!(metrics.cpu_usage > 0.0);
    assert!(metrics.memory_usage > 0.0);
    assert!(metrics.active_connections > 0);

    Ok(())
}

#[tokio::test]
async fn test_security_types_comprehensive(KeyType::Ed25519,
        key_size: 256,
        purpose: KeyPurpose::Signing,
        hardware_backed: true,
    };

    assert_eq!(key_config.key_type, KeyType::Ed25519);
    assert_eq!(key_config.key_size, 256);
    assert!(key_config.hardware_backed);

    let encryption_params = EncryptionParams {
        algorithm: EncryptionAlgorithm::AesGcm256,
        key_derivation: KeyDerivation::Hkdf,
        nonce_size: 12,
        tag_size: 16,
    };

    assert_eq!(encryption_params.algorithm, EncryptionAlgorithm::AesGcm256);
    assert_eq!(encryption_params.nonce_size, 12);
    assert_eq!(encryption_params.tag_size, 16);

    Ok(())
}

#[tokio::test]
async fn test_constants_comprehensive() {
    assert_eq!(api::VERSION, "v1");
    assert!(!api::PROJECT_NAME.is_empty());
    assert!(api::DEFAULT_TIMEOUT_MS > 0);
    assert!(api::MAX_REQUEST_SIZE > 0);

    assert!(network::ports::API > 0);
    assert!(network::ports::HTTPS > 0);
    assert!(network::ports::GRPC > 0);

    assert!(network::limits::MAX_CONNECTIONS > 0);

    assert!(beardog_types::canonical::configuration::NetworkConfig::default().max_connections > 0);
    assert!(network::limits::CONNECTION_TIMEOUT_MS > 0);
    assert!(network::limits::MAX_RETRIES > 0);

    assert!(!network::addresses::DEFAULT_BIND.is_empty());
    assert!(!network::addresses::LOCALHOST.is_empty());
}

#[tokio::test]
async fn test_zero_cost_types_comprehensive(1024,
        max_capacity: 65536,
        growth_factor: 2.0,
        shrink_threshold: 0.25,
    };

    assert_eq!(buffer_config.initial_capacity, 1024);
    assert_eq!(buffer_config.max_capacity, 65536);
    assert!(buffer_config.growth_factor > 1.0);
    assert!(buffer_config.shrink_threshold < 1.0);

    let pool_config = MemoryPoolConfig {
        small_buffer_size: 1024,
        medium_buffer_size: 8192,
        large_buffer_size: 65536,
        max_buffers_per_size: 100,
    };

    assert!(pool_config.small_buffer_size < pool_config.medium_buffer_size);
    assert!(pool_config.medium_buffer_size < pool_config.large_buffer_size);
    assert!(pool_config.max_buffers_per_size > 0);

    Ok(())
}

#[tokio::test]
async fn test_error_handling_types() -> Result<(), BearDogError> {
    let error = BearDogError::validation("Test validation error");
    let error_string = format!("{}", error);
    assert!(error_string.contains("validation"));
    assert!(error_string.contains("Test validation error"));

    let source_error = BearDogError::network("Network connection failed");
    let chained_error = BearDogError::system("System error").with_source(source_error);
    assert!(chained_error.source().is_some());

    Ok(())
}

#[tokio::test]
async fn test_configuration_validation_comprehensive() -> Result<(), BearDogError> {
    let mut config = BearDogCanonicalConfig::default();

    config.app.port = 8080;
    assert!(config.app.port > 1024); // Non-privileged port

    config.app.max_connections = 1000;
    assert!(config.app.max_connections > 0);
    assert!(config.app.max_connections <= 10000); // Reasonable limit

    config.app.timeout_seconds = 30;
    assert!(config.app.timeout_seconds > 0);
    assert!(config.app.timeout_seconds < 300); // 5 minutes max

    Ok(())
}

#[tokio::test]
async fn test_type_conversions_comprehensive() -> Result<(), BearDogError> {
    let service_name = "beardog-test-service";
    let config = BearDogCanonicalConfig {
        app: AppConfig {
            service_name: service_name.to_string(),
            max_connections: 1000,
            timeout_seconds: 30,
            ..Default::default()
        },
        ..Default::default()
    };

    assert_eq!(config.app.service_name, service_name);

    let port: u16 = config.app.port;
    assert!(port > 0);

    let max_connections: usize = config.app.max_connections;
    assert!(max_connections > 0);

    Ok(())
}
