//! Integration tests for Service Discovery
//!
//! Tests real service discovery implementations with mock servers.

use beardog_core::service_discovery::{
    ConsulDiscovery, HealthStatus, ServiceDiscovery, ServiceFilter, ServiceInfo,
    StaticConfigDiscovery,
};
use std::collections::HashMap;
use std::net::SocketAddr;

/// Test static discovery with filtering
#[tokio::test]
async fn test_static_discovery_integration() {
    let config_content = r#"
[[services]]
id = "hsm-1"
name = "beardog-hsm"
address = "127.0.0.1:9000"
tags = ["production", "v1.0.0"]

[services.metadata]
region = "us-east-1"

[[services]]
id = "hsm-2"
name = "beardog-hsm"
address = "127.0.0.1:9001"
tags = ["production", "v1.0.0"]

[services.metadata]
region = "us-west-2"

[[services]]
id = "api-1"
name = "beardog-api"
address = "127.0.0.1:8080"
tags = ["development"]

[services.metadata]
region = "us-east-1"
"#;

    let temp_file = std::env::temp_dir().join("beardog_test_services.toml");
    std::fs::write(&temp_file, config_content).unwrap();

    let discovery = StaticConfigDiscovery::from_file(temp_file.clone()).unwrap();

    // Test: Discover all HSM services
    let filter = ServiceFilter::new("beardog-hsm");
    let services = discovery.discover(&filter).await.unwrap();
    assert_eq!(services.len(), 2);
    assert!(services.iter().all(|s| s.name == "beardog-hsm"));

    // Test: Filter by tag
    let filter = ServiceFilter::new("beardog-hsm").with_tag("production");
    let services = discovery.discover(&filter).await.unwrap();
    assert_eq!(services.len(), 2);

    // Test: Filter by metadata
    let filter = ServiceFilter::new("beardog-hsm").with_metadata("region", "us-east-1");
    let services = discovery.discover(&filter).await.unwrap();
    assert_eq!(services.len(), 1);
    assert_eq!(services[0].id, "hsm-1");

    // Test: Filter by name only (no additional filters)
    let filter = ServiceFilter::new("beardog-api");
    let services = discovery.discover(&filter).await.unwrap();
    assert_eq!(services.len(), 1);
    assert_eq!(services[0].tags, vec!["development"]);

    // Cleanup
    std::fs::remove_file(&temp_file).ok();
}

/// Test service filter builder
#[test]
fn test_service_filter_builder() {
    let filter = ServiceFilter::new("my-service")
        .with_tag("production")
        .with_tag("v2.0.0")
        .with_metadata("datacenter", "dc1")
        .with_metadata("zone", "us-east-1a")
        .healthy_only(true);

    assert_eq!(filter.name, Some("my-service".to_string()));
    assert_eq!(filter.tags.len(), 2);
    assert!(filter.tags.contains(&"production".to_string()));
    assert!(filter.tags.contains(&"v2.0.0".to_string()));
    assert_eq!(filter.metadata.len(), 2);
    assert_eq!(filter.metadata.get("datacenter").unwrap(), "dc1");
    assert!(filter.healthy_only);
}

/// Test ServiceInfo serialization/deserialization
#[test]
fn test_service_info_serde() {
    let mut metadata = HashMap::new();
    metadata.insert("version".to_string(), "1.0.0".to_string());
    metadata.insert("region".to_string(), "us-east-1".to_string());

    let service = ServiceInfo {
        id: "test-service-1".to_string(),
        name: "test-service".to_string(),
        address: "192.168.1.100:8080".parse::<SocketAddr>().unwrap(),
        metadata,
        health_endpoint: Some("http://192.168.1.100:8080/health".to_string()),
        tags: vec!["production".to_string(), "primary".to_string()],
    };

    // Serialize to JSON
    let json = serde_json::to_string(&service).unwrap();

    // Deserialize back
    let deserialized: ServiceInfo = serde_json::from_str(&json).unwrap();

    // Verify round-trip
    assert_eq!(service, deserialized);
    assert_eq!(deserialized.id, "test-service-1");
    assert_eq!(deserialized.name, "test-service");
    assert_eq!(deserialized.tags.len(), 2);
    assert_eq!(deserialized.metadata.len(), 2);
}

/// Test health check for static discovery
#[tokio::test]
async fn test_static_discovery_health_check() {
    let config_content = r#"
[[services]]
id = "test-1"
name = "test-service"
address = "127.0.0.1:9000"
tags = []

[services.metadata]
"#;

    let temp_file = std::env::temp_dir().join("beardog_health_test.toml");
    std::fs::write(&temp_file, config_content).unwrap();

    let discovery = StaticConfigDiscovery::from_file(temp_file.clone()).unwrap();

    // Health check should pass when file exists
    let health = discovery.health_check().await.unwrap();
    assert_eq!(health, HealthStatus::Healthy);

    // Delete file and check again
    std::fs::remove_file(&temp_file).ok();
    let health = discovery.health_check().await.unwrap();
    assert_eq!(health, HealthStatus::Unhealthy);
}

/// Test ConsulDiscovery configuration from environment
#[test]
fn test_consul_config_from_env() {
    // Set environment variables
    std::env::set_var(
        "BEARDOG_CONSUL_ADDRESS",
        "http://consul.prod.example.com:8500",
    );
    std::env::set_var("BEARDOG_CONSUL_DATACENTER", "prod-dc1");
    std::env::set_var("BEARDOG_CONSUL_TIMEOUT_SECS", "20");
    std::env::set_var("BEARDOG_CONSUL_TLS", "true");

    // Create discovery (will use env vars)
    let result = ConsulDiscovery::from_env();

    // Cleanup
    std::env::remove_var("BEARDOG_CONSUL_ADDRESS");
    std::env::remove_var("BEARDOG_CONSUL_DATACENTER");
    std::env::remove_var("BEARDOG_CONSUL_TIMEOUT_SECS");
    std::env::remove_var("BEARDOG_CONSUL_TLS");

    // Should succeed (HTTP client creation should work)
    assert!(result.is_ok());
}

/// Test error handling for missing configuration
#[test]
fn test_static_discovery_missing_file() {
    let result = StaticConfigDiscovery::from_file("/nonexistent/path/to/config.toml".into());
    assert!(result.is_err());

    match result {
        Err(e) => {
            let msg = format!("{:?}", e);
            assert!(msg.contains("Failed to read config") || msg.contains("No such file"));
        }
        Ok(_) => panic!("Expected error for missing file"),
    }
}

/// Test empty service list discovery
#[tokio::test]
async fn test_empty_service_discovery() {
    let config_content = r#"
services = []
"#;

    let temp_file = std::env::temp_dir().join("beardog_empty_test.toml");
    std::fs::write(&temp_file, config_content).unwrap();

    let discovery = StaticConfigDiscovery::from_file(temp_file.clone()).unwrap();

    let filter = ServiceFilter::new("any-service");
    let services = discovery.discover(&filter).await.unwrap();
    assert_eq!(services.len(), 0);

    std::fs::remove_file(&temp_file).ok();
}

/// Test registration/deregistration not supported by static discovery
#[tokio::test]
async fn test_static_discovery_no_registration() {
    let config_content = r#"
services = []
"#;

    let temp_file = std::env::temp_dir().join("beardog_noreg_test.toml");
    std::fs::write(&temp_file, config_content).unwrap();

    let discovery = StaticConfigDiscovery::from_file(temp_file.clone()).unwrap();

    let service = ServiceInfo {
        id: "test".to_string(),
        name: "test".to_string(),
        address: "127.0.0.1:9000".parse().unwrap(),
        metadata: HashMap::new(),
        health_endpoint: None,
        tags: vec![],
    };

    // Registration should fail
    let result = discovery.register(&service).await;
    assert!(result.is_err());

    // Deregistration should fail
    let result = discovery.deregister("test").await;
    assert!(result.is_err());

    std::fs::remove_file(&temp_file).ok();
}
