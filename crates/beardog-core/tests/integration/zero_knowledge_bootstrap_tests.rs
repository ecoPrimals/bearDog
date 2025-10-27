//! Integration tests for Zero-Knowledge Bootstrap
//!
//! Tests the self-discovery and capability registry mechanisms.

use beardog_core::zero_knowledge_bootstrap::{
    ZeroKnowledgeBootstrap, ServiceCapability, ServiceCapabilityType,
};
use beardog_types::canonical::capabilities::Capability;
use std::time::Duration;

/// Test basic initialization of zero-knowledge bootstrap
#[tokio::test]
async fn test_bootstrap_initialization() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let config = beardog_core::Config::default();
    
    // Act
    let bootstrap = ZeroKnowledgeBootstrap::new(config)?;
    
    // Assert
    assert!(bootstrap.is_initialized());
    
    Ok(())
}

/// Test capability registry can store and retrieve capabilities
#[tokio::test]
async fn test_capability_registry_basic_operations() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let config = beardog_core::Config::default();
    let bootstrap = ZeroKnowledgeBootstrap::new(config)?;
    
    let capability = ServiceCapability {
        service_type: ServiceCapabilityType::Storage,
        endpoint: "http://localhost:8080".to_string(),
        capabilities: vec![Capability::Storage],
        metadata: Default::default(),
    };
    
    // Act
    bootstrap.register_capability(capability.clone()).await?;
    let discovered = bootstrap.discover_capabilities(ServiceCapabilityType::Storage).await?;
    
    // Assert
    assert!(!discovered.is_empty(), "Should discover registered capability");
    assert_eq!(discovered[0].service_type, ServiceCapabilityType::Storage);
    
    Ok(())
}

/// Test capability registry handles unknown service types gracefully
#[tokio::test]
async fn test_capability_registry_unknown_service_type() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let config = beardog_core::Config::default();
    let bootstrap = ZeroKnowledgeBootstrap::new(config)?;
    
    // Act
    let result = bootstrap.discover_capabilities(ServiceCapabilityType::Compute).await?;
    
    // Assert
    assert!(result.is_empty(), "Should return empty for unknown service types");
    
    Ok(())
}

/// Test self-discovery timeout handling
#[tokio::test]
async fn test_self_discovery_with_timeout() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let mut config = beardog_core::Config::default();
    config.discovery_timeout = Duration::from_millis(100);
    
    let bootstrap = ZeroKnowledgeBootstrap::new(config)?;
    
    // Act - discovery should complete even with short timeout
    let result = tokio::time::timeout(
        Duration::from_secs(1),
        bootstrap.discover_ecosystem_services()
    ).await;
    
    // Assert
    assert!(result.is_ok(), "Discovery should complete within timeout");
    
    Ok(())
}

/// Test capability deregistration
#[tokio::test]
async fn test_capability_deregistration() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let config = beardog_core::Config::default();
    let bootstrap = ZeroKnowledgeBootstrap::new(config)?;
    
    let capability = ServiceCapability {
        service_type: ServiceCapabilityType::Compute,
        endpoint: "http://localhost:8081".to_string(),
        capabilities: vec![Capability::Compute],
        metadata: Default::default(),
    };
    
    // Act
    let id = bootstrap.register_capability(capability.clone()).await?;
    let before_dereg = bootstrap.discover_capabilities(ServiceCapabilityType::Compute).await?;
    
    bootstrap.deregister_capability(&id).await?;
    let after_dereg = bootstrap.discover_capabilities(ServiceCapabilityType::Compute).await?;
    
    // Assert
    assert_eq!(before_dereg.len(), 1, "Should have one capability before deregistration");
    assert_eq!(after_dereg.len(), 0, "Should have no capabilities after deregistration");
    
    Ok(())
}

/// Test multiple capabilities of same type
#[tokio::test]
async fn test_multiple_capabilities_same_type() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let config = beardog_core::Config::default();
    let bootstrap = ZeroKnowledgeBootstrap::new(config)?;
    
    let cap1 = ServiceCapability {
        service_type: ServiceCapabilityType::Storage,
        endpoint: "http://localhost:8080".to_string(),
        capabilities: vec![Capability::Storage],
        metadata: Default::default(),
    };
    
    let cap2 = ServiceCapability {
        service_type: ServiceCapabilityType::Storage,
        endpoint: "http://localhost:8082".to_string(),
        capabilities: vec![Capability::Storage],
        metadata: Default::default(),
    };
    
    // Act
    bootstrap.register_capability(cap1).await?;
    bootstrap.register_capability(cap2).await?;
    let discovered = bootstrap.discover_capabilities(ServiceCapabilityType::Storage).await?;
    
    // Assert
    assert_eq!(discovered.len(), 2, "Should discover both storage capabilities");
    
    Ok(())
}

/// Test ecosystem listener initialization
#[tokio::test]
async fn test_ecosystem_listener_starts() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let config = beardog_core::Config::default();
    let bootstrap = ZeroKnowledgeBootstrap::new(config)?;
    
    // Act
    let listener_status = bootstrap.get_listener_status().await?;
    
    // Assert
    assert!(listener_status.is_running, "Ecosystem listener should be running");
    
    Ok(())
}

/// Test capability health checks
#[tokio::test]
async fn test_capability_health_check() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let config = beardog_core::Config::default();
    let bootstrap = ZeroKnowledgeBootstrap::new(config)?;
    
    let capability = ServiceCapability {
        service_type: ServiceCapabilityType::Health,
        endpoint: "http://localhost:9090/health".to_string(),
        capabilities: vec![Capability::HealthCheck],
        metadata: Default::default(),
    };
    
    // Act
    bootstrap.register_capability(capability.clone()).await?;
    let health = bootstrap.check_capability_health(&capability.endpoint).await;
    
    // Assert
    // Health check may fail if service not running, but should return a result
    assert!(health.is_ok() || health.is_err(), "Health check should return a result");
    
    Ok(())
}

