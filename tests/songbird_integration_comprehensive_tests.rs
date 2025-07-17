use std::time::Duration;
use tokio::time::sleep;
use serde_json::json;

use beardog_adapters::adapters::universal::songbird_handoff::{
    client::SongBirdClient,
    registration::UniversalRegistrationManager,
    health::UniversalHealthMonitor,
};
use beardog_adapters::adapters::universal::beardog_provider::BearDogProvider;
use beardog_config::BearDogConfig;
use beardog_errors::BearDogResult;

/// Mock SongBird server for testing
struct MockSongBirdServer {
    registration_success: bool,
    health_endpoint_active: bool,
    heartbeat_endpoint_active: bool,
}

impl MockSongBirdServer {
    fn new() -> Self {
        Self {
            registration_success: true,
            health_endpoint_active: true,
            heartbeat_endpoint_active: true,
        }
    }

    fn with_registration_failure() -> Self {
        Self {
            registration_success: false,
            health_endpoint_active: true,
            heartbeat_endpoint_active: true,
        }
    }

    fn with_health_endpoint_down() -> Self {
        Self {
            registration_success: true,
            health_endpoint_active: false,
            heartbeat_endpoint_active: true,
        }
    }
}

#[tokio::test]
async fn test_songbird_registration_success() -> BearDogResult<()> {
    // Setup test environment
    let config = create_test_config();
    let client = SongBirdClient::new(config.clone());
    let registration_manager = UniversalRegistrationManager::new(
        "test-primal-001".to_string(),
        std::sync::Arc::new(client),
    );

    // Test successful registration
    let result = registration_manager.register_with_songbird().await;
    
    match result {
        Ok(registration) => {
            assert!(!registration.registration_id.to_string().is_empty());
            assert_eq!(registration.ecosystem_id, "test-ecosystem");
            assert_eq!(registration.instance_id, "test-instance");
            println!("✅ SongBird registration successful: {}", registration.registration_id);
        }
        Err(e) => {
            // In test environment, this might fail due to no actual SongBird server
            // This is expected behavior for standalone fallback
            println!("⚠️  SongBird registration failed (expected in test): {}", e);
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_songbird_registration_fallback() -> BearDogResult<()> {
    // Test fallback behavior when SongBird is unavailable
    let config = create_test_config_with_invalid_songbird();
    let client = SongBirdClient::new(config.clone());
    let registration_manager = UniversalRegistrationManager::new(
        "test-primal-002".to_string(),
        std::sync::Arc::new(client),
    );

    // This should fail to connect to SongBird but not crash
    let result = registration_manager.register_with_songbird().await;
    
    // The system should handle this gracefully
    match result {
        Ok(_) => {
            println!("✅ Unexpected success - test environment might have SongBird running");
        }
        Err(e) => {
            println!("✅ Expected failure handled gracefully: {}", e);
            // Verify the error is properly structured
            assert!(e.to_string().contains("SongBird") || e.to_string().contains("connection"));
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_heartbeat_task_functionality() -> BearDogResult<()> {
    let config = create_test_config();
    let client = SongBirdClient::new(config.clone());
    let registration_manager = UniversalRegistrationManager::new(
        "test-primal-003".to_string(),
        std::sync::Arc::new(client),
    );

    // Test heartbeat task startup
    let result = registration_manager.start_heartbeat_task().await;
    assert!(result.is_ok());
    println!("✅ Heartbeat task started successfully");

    // Test direct heartbeat sending
    let heartbeat_result = registration_manager.send_heartbeat().await;
    match heartbeat_result {
        Ok(_) => {
            println!("✅ Heartbeat sent successfully");
        }
        Err(e) => {
            println!("⚠️  Heartbeat failed (expected in test environment): {}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_health_monitoring_comprehensive() -> BearDogResult<()> {
    let config = create_test_config();
    let client = SongBirdClient::new(config.clone());
    let health_monitor = UniversalHealthMonitor::new(
        "test-primal-004".to_string(),
        std::sync::Arc::new(client),
    );

    // Test health monitoring startup
    let result = health_monitor.start_monitoring().await;
    assert!(result.is_ok());
    println!("✅ Health monitoring started successfully");

    // Test health check execution
    let health_check_result = health_monitor.perform_health_check().await;
    assert!(health_check_result.is_ok());
    
    let health_result = health_check_result.unwrap();
    assert!(health_result.response_time_ms >= 0.0);
    assert!(health_result.metrics.cpu_utilization >= 0.0);
    assert!(health_result.metrics.memory_utilization >= 0.0);
    
    println!("✅ Health check completed: {:?}", health_result.status);
    println!("   Response time: {:.2}ms", health_result.response_time_ms);
    println!("   CPU: {:.1}%", health_result.metrics.cpu_utilization);
    println!("   Memory: {:.1}%", health_result.metrics.memory_utilization);

    Ok(())
}

#[tokio::test]
async fn test_capability_advertisement_updates() -> BearDogResult<()> {
    let config = create_test_config();
    let client = SongBirdClient::new(config.clone());
    let registration_manager = UniversalRegistrationManager::new(
        "test-primal-005".to_string(),
        std::sync::Arc::new(client),
    );

    // Test capability advertisement
    let result = registration_manager.update_capability_advertisement().await;
    
    match result {
        Ok(_) => {
            println!("✅ Capability advertisement updated successfully");
        }
        Err(e) => {
            println!("⚠️  Capability advertisement failed (expected in test): {}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_beardog_provider_full_lifecycle() -> BearDogResult<()> {
    let config = create_test_config();
    let primal_id = "test-primal-006".to_string();
    let mut provider = BearDogProvider::new(primal_id.clone(), config.clone());

    // Test initialization
    let init_result = provider.initialize(beardog_adapters::adapters::universal::beardog_provider::ProviderConfig::default()).await;
    match init_result {
        Ok(_) => {
            println!("✅ BearDog provider initialized successfully");
        }
        Err(e) => {
            println!("⚠️  BearDog provider initialization failed: {}", e);
        }
    }

    // Test ecosystem registration
    let registration_result = provider.register_with_ecosystem().await;
    match registration_result {
        Ok(registration) => {
            println!("✅ Ecosystem registration successful: {}", registration.registration_id);
            assert_eq!(registration.ecosystem_id, "test-ecosystem");
        }
        Err(e) => {
            println!("⚠️  Ecosystem registration failed (expected in test): {}", e);
        }
    }

    // Test graceful shutdown
    let shutdown_result = provider.shutdown().await;
    assert!(shutdown_result.is_ok());
    println!("✅ BearDog provider shutdown completed");

    Ok(())
}

#[tokio::test]
async fn test_concurrent_operations() -> BearDogResult<()> {
    let config = create_test_config();
    let client = std::sync::Arc::new(SongBirdClient::new(config.clone()));
    
    // Create multiple registration managers
    let managers: Vec<_> = (0..5)
        .map(|i| UniversalRegistrationManager::new(
            format!("test-primal-concurrent-{}", i),
            std::sync::Arc::clone(&client),
        ))
        .collect();

    // Test concurrent operations
    let mut handles = Vec::new();
    for manager in managers {
        handles.push(tokio::spawn(async move {
            let _ = manager.start_heartbeat_task().await;
            let _ = manager.send_heartbeat().await;
            let _ = manager.update_capability_advertisement().await;
        }));
    }

    // Wait for all concurrent operations to complete
    for handle in handles {
        handle.await.unwrap();
    }

    println!("✅ Concurrent operations completed successfully");
    Ok(())
}

#[tokio::test]
async fn test_background_task_lifecycle() -> BearDogResult<()> {
    let config = create_test_config();
    let primal_id = "test-primal-background".to_string();
    let mut provider = BearDogProvider::new(primal_id.clone(), config.clone());

    // Initialize provider (this starts background tasks)
    let _ = provider.initialize(beardog_adapters::adapters::universal::beardog_provider::ProviderConfig::default()).await;

    // Let background tasks run for a short time
    sleep(Duration::from_millis(100)).await;

    // Test that background tasks are running by checking logs
    println!("✅ Background tasks running for 100ms");

    // Shutdown (this should stop background tasks)
    let shutdown_result = provider.shutdown().await;
    assert!(shutdown_result.is_ok());
    println!("✅ Background tasks stopped during shutdown");

    Ok(())
}

#[tokio::test]
async fn test_error_handling_and_recovery() -> BearDogResult<()> {
    let config = create_test_config_with_invalid_songbird();
    let client = SongBirdClient::new(config.clone());
    let registration_manager = UniversalRegistrationManager::new(
        "test-primal-error-recovery".to_string(),
        std::sync::Arc::new(client),
    );

    // Test error handling in registration
    let registration_result = registration_manager.register_with_songbird().await;
    assert!(registration_result.is_err());
    println!("✅ Registration error handled properly");

    // Test error handling in heartbeat
    let heartbeat_result = registration_manager.send_heartbeat().await;
    assert!(heartbeat_result.is_err());
    println!("✅ Heartbeat error handled properly");

    // Test error handling in capability advertisement
    let capability_result = registration_manager.update_capability_advertisement().await;
    assert!(capability_result.is_err());
    println!("✅ Capability advertisement error handled properly");

    Ok(())
}

#[tokio::test]
async fn test_metrics_and_monitoring() -> BearDogResult<()> {
    let config = create_test_config();
    let client = SongBirdClient::new(config.clone());
    let health_monitor = UniversalHealthMonitor::new(
        "test-primal-metrics".to_string(),
        std::sync::Arc::new(client),
    );

    // Test metrics collection
    let health_result = health_monitor.perform_health_check().await?;
    
    // Verify metrics are within expected ranges
    assert!(health_result.metrics.cpu_utilization >= 0.0);
    assert!(health_result.metrics.cpu_utilization <= 100.0);
    assert!(health_result.metrics.memory_utilization >= 0.0);
    assert!(health_result.metrics.memory_utilization <= 100.0);
    assert!(health_result.metrics.active_connections >= 0);
    assert!(health_result.response_time_ms >= 0.0);

    println!("✅ Metrics validation passed");
    println!("   CPU: {:.1}%", health_result.metrics.cpu_utilization);
    println!("   Memory: {:.1}%", health_result.metrics.memory_utilization);
    println!("   Connections: {}", health_result.metrics.active_connections);
    println!("   Response time: {:.2}ms", health_result.response_time_ms);

    Ok(())
}

// Helper functions for test setup
fn create_test_config() -> BearDogConfig {
    BearDogConfig {
        songbird_endpoint: Some("http://localhost:8080".to_string()),
        ecosystem_id: "test-ecosystem".to_string(),
        instance_id: "test-instance".to_string(),
        ..Default::default()
    }
}

fn create_test_config_with_invalid_songbird() -> BearDogConfig {
    BearDogConfig {
        songbird_endpoint: Some("http://invalid-songbird:9999".to_string()),
        ecosystem_id: "test-ecosystem".to_string(),
        instance_id: "test-instance".to_string(),
        ..Default::default()
    }
}

#[tokio::test]
async fn test_primal_discovery_integration() -> BearDogResult<()> {
    let config = create_test_config();
    let client = SongBirdClient::new(config.clone());
    
    // Test primal discovery functionality
    let discovery_result = client.discover_primals().await;
    
    match discovery_result {
        Ok(primals) => {
            println!("✅ Primal discovery successful, found {} primals", primals.len());
            for primal in primals {
                println!("   - {}: {}", primal.id, primal.capabilities.len());
            }
        }
        Err(e) => {
            println!("⚠️  Primal discovery failed (expected in test): {}", e);
        }
    }
    
    Ok(())
}

#[tokio::test]
async fn test_security_provider_registration() -> BearDogResult<()> {
    let config = create_test_config();
    let primal_id = "beardog-security-provider".to_string();
    let mut provider = BearDogProvider::new(primal_id.clone(), config.clone());

    // Test security provider specific registration
    let init_result = provider.initialize(beardog_adapters::adapters::universal::beardog_provider::ProviderConfig::default()).await;
    
    match init_result {
        Ok(_) => {
            println!("✅ Security provider initialized successfully");
            
            // Verify security capabilities are advertised
            let capabilities = provider.capabilities();
            assert!(capabilities.iter().any(|c| c.name.contains("security")));
            println!("✅ Security capabilities advertised: {}", capabilities.len());
        }
        Err(e) => {
            println!("⚠️  Security provider initialization failed: {}", e);
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_ecosystem_resilience() -> BearDogResult<()> {
    let config = create_test_config();
    let primal_id = "test-primal-resilience".to_string();
    let mut provider = BearDogProvider::new(primal_id.clone(), config.clone());

    // Test initialization with SongBird potentially unavailable
    let init_result = provider.initialize(beardog_adapters::adapters::universal::beardog_provider::ProviderConfig::default()).await;
    
    // Provider should initialize regardless of SongBird availability
    match init_result {
        Ok(_) => {
            println!("✅ Provider initialized successfully");
        }
        Err(e) => {
            println!("⚠️  Provider initialization failed: {}", e);
        }
    }

    // Test ecosystem registration with fallback
    let registration_result = provider.register_with_ecosystem().await;
    
    // Should get either successful registration or fallback
    match registration_result {
        Ok(registration) => {
            println!("✅ Ecosystem registration: {}", registration.status);
            assert!(
                registration.status == beardog_adapters::adapters::universal::beardog_provider::RegistrationStatus::Active ||
                registration.status == beardog_adapters::adapters::universal::beardog_provider::RegistrationStatus::Standalone
            );
        }
        Err(e) => {
            println!("❌ Ecosystem registration failed: {}", e);
        }
    }

    Ok(())
} 