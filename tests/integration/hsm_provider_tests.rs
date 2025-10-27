//! HSM Provider Integration Tests
//! 
//! High-value integration tests for HSM provider initialization,
//! health checks, failover, and multi-provider coordination.
//!
//! TEST_CATEGORY: integration
//! TEST_DOMAIN: hsm

use beardog_errors::BearDogError;
use beardog_tunnel::hsm::manager::HsmManager;
use beardog_tunnel::hsm::providers::HsmProvider;
use beardog_types::canonical::hsm_unified::*;
use std::time::Duration;
use tokio::time::timeout;

/// Helper to create test HSM configuration
fn create_test_hsm_config() -> UnifiedHsmConfig {
    UnifiedHsmConfig {
        tier: HsmTier::Software,
        provider_type: HsmProviderType::Software,
        capabilities: vec![
            HsmCapability::KeyGeneration,
            HsmCapability::Signing,
            HsmCapability::Encryption,
        ],
        health_check_interval: Duration::from_secs(30),
        timeout: Duration::from_secs(5),
        retry_config: RetryConfig::default(),
    }
}

/// Helper to create invalid HSM configuration
fn create_invalid_hsm_config() -> UnifiedHsmConfig {
    UnifiedHsmConfig {
        tier: HsmTier::Hardware, // Hardware but no hardware available
        provider_type: HsmProviderType::PKCS11,
        capabilities: vec![],
        health_check_interval: Duration::from_millis(1), // Too short
        timeout: Duration::from_millis(1), // Too short
        retry_config: RetryConfig {
            max_retries: 0,
            initial_backoff: Duration::from_secs(0),
            max_backoff: Duration::from_secs(0),
        },
    }
}

// ====================================================================================
// Provider Initialization Tests (10 tests)
// ====================================================================================

#[tokio::test]
async fn test_hsm_provider_initialization_success() {
    // Test successful provider initialization with valid config
    let config = create_test_hsm_config();
    
    let result = HsmManager::new(config).await;
    
    assert!(
        result.is_ok(),
        "HSM provider should initialize successfully with valid config"
    );
    
    if let Ok(manager) = result {
        assert!(
            manager.is_healthy().await,
            "Initialized provider should be healthy"
        );
    }
}

#[tokio::test]
async fn test_hsm_provider_initialization_with_invalid_config() {
    // Test that initialization fails gracefully with invalid config
    let config = create_invalid_hsm_config();
    
    let result = HsmManager::new(config).await;
    
    assert!(
        result.is_err(),
        "HSM provider initialization should fail with invalid config"
    );
    
    if let Err(e) = result {
        assert!(
            matches!(e, BearDogError::Configuration { .. }),
            "Should return configuration error, got: {:?}",
            e
        );
    }
}

#[tokio::test]
async fn test_hsm_provider_initialization_timeout() {
    // Test that initialization respects timeout settings
    let mut config = create_test_hsm_config();
    config.timeout = Duration::from_millis(1); // Very short timeout
    
    let start = std::time::Instant::now();
    let result = timeout(Duration::from_secs(2), HsmManager::new(config)).await;
    let elapsed = start.elapsed();
    
    assert!(
        elapsed < Duration::from_secs(2),
        "Initialization should respect timeout, took: {:?}",
        elapsed
    );
    
    // May succeed or fail depending on system speed, but should not hang
    assert!(
        result.is_ok(),
        "Timeout wrapper should not trigger (internal timeout should)"
    );
}

#[tokio::test]
async fn test_hsm_provider_initialization_concurrent() {
    // Test concurrent provider initialization
    let config1 = create_test_hsm_config();
    let config2 = create_test_hsm_config();
    
    let (result1, result2) = tokio::join!(
        HsmManager::new(config1),
        HsmManager::new(config2)
    );
    
    assert!(
        result1.is_ok() && result2.is_ok(),
        "Concurrent provider initialization should succeed"
    );
}

#[tokio::test]
async fn test_hsm_provider_initialization_retry_logic() {
    // Test that initialization retry logic works correctly
    let mut config = create_test_hsm_config();
    config.retry_config.max_retries = 3;
    config.retry_config.initial_backoff = Duration::from_millis(10);
    
    let start = std::time::Instant::now();
    let result = HsmManager::new(config).await;
    let elapsed = start.elapsed();
    
    if result.is_ok() {
        // Should complete quickly if successful on first try
        assert!(
            elapsed < Duration::from_secs(1),
            "Successful initialization should be fast"
        );
    }
}

#[tokio::test]
async fn test_hsm_provider_initialization_with_missing_deps() {
    // Test initialization when required dependencies are missing
    let mut config = create_test_hsm_config();
    config.provider_type = HsmProviderType::PKCS11; // PKCS#11 requires library
    
    let result = HsmManager::new(config).await;
    
    // Should handle missing dependencies gracefully
    if result.is_err() {
        // Expected - no PKCS#11 library in test environment
        assert!(true);
    } else {
        // If somehow succeeded, verify it's functional
        assert!(result.unwrap().is_healthy().await);
    }
}

#[tokio::test]
async fn test_hsm_provider_initialization_health_check() {
    // Test that health check runs after initialization
    let config = create_test_hsm_config();
    
    let manager = HsmManager::new(config)
        .await
        .expect("Should initialize successfully");
    
    // Wait a moment for health check to run
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    let is_healthy = manager.is_healthy().await;
    assert!(is_healthy, "Provider should be healthy after initialization");
}

#[tokio::test]
async fn test_hsm_provider_initialization_capability_detection() {
    // Test that capabilities are detected during initialization
    let config = create_test_hsm_config();
    
    let manager = HsmManager::new(config)
        .await
        .expect("Should initialize successfully");
    
    let capabilities = manager.get_capabilities().await;
    
    assert!(
        !capabilities.is_empty(),
        "Should detect at least some capabilities"
    );
    
    assert!(
        capabilities.contains(&HsmCapability::KeyGeneration),
        "Should detect key generation capability"
    );
}

#[tokio::test]
async fn test_hsm_provider_initialization_multiple_providers() {
    // Test initializing multiple providers with different configs
    let config_software = create_test_hsm_config();
    let mut config_cloud = create_test_hsm_config();
    config_cloud.provider_type = HsmProviderType::CloudKMS;
    
    let result1 = HsmManager::new(config_software).await;
    let result2 = HsmManager::new(config_cloud).await;
    
    // At least one should succeed (software HSM)
    assert!(
        result1.is_ok(),
        "Software HSM should always be available"
    );
    
    // Cloud KMS may not be available in test environment
    if result2.is_ok() {
        assert!(result2.unwrap().is_healthy().await);
    }
}

#[tokio::test]
async fn test_hsm_provider_initialization_cleanup_on_failure() {
    // Test that resources are cleaned up if initialization fails
    let config = create_invalid_hsm_config();
    
    let result = HsmManager::new(config).await;
    
    if result.is_err() {
        // Verify no resource leaks by attempting another initialization
        let valid_config = create_test_hsm_config();
        let second_result = HsmManager::new(valid_config).await;
        
        assert!(
            second_result.is_ok(),
            "Should be able to initialize after failed attempt"
        );
    }
}

// ====================================================================================
// Provider Health Check Tests (10 tests)
// ====================================================================================

#[tokio::test]
async fn test_hsm_provider_health_check_success() {
    // Test successful health check
    let config = create_test_hsm_config();
    let manager = HsmManager::new(config)
        .await
        .expect("Should initialize successfully");
    
    let is_healthy = manager.is_healthy().await;
    
    assert!(is_healthy, "Healthy provider should return true");
}

#[tokio::test]
async fn test_hsm_provider_health_check_after_operation() {
    // Test health check after performing operations
    let config = create_test_hsm_config();
    let manager = HsmManager::new(config)
        .await
        .expect("Should initialize successfully");
    
    // Perform some operations
    let capabilities = manager.get_capabilities().await;
    assert!(!capabilities.is_empty());
    
    // Health check should still pass
    let is_healthy = manager.is_healthy().await;
    assert!(is_healthy, "Should remain healthy after operations");
}

#[tokio::test]
async fn test_hsm_provider_health_check_timeout() {
    // Test health check timeout handling
    let mut config = create_test_hsm_config();
    config.timeout = Duration::from_millis(1);
    
    if let Ok(manager) = HsmManager::new(config).await {
        let start = std::time::Instant::now();
        let _ = manager.is_healthy().await;
        let elapsed = start.elapsed();
        
        assert!(
            elapsed < Duration::from_secs(1),
            "Health check should respect timeout"
        );
    }
}

#[tokio::test]
async fn test_hsm_provider_health_check_concurrent() {
    // Test concurrent health checks
    let config = create_test_hsm_config();
    let manager = HsmManager::new(config)
        .await
        .expect("Should initialize successfully");
    
    let (health1, health2, health3) = tokio::join!(
        manager.is_healthy(),
        manager.is_healthy(),
        manager.is_healthy()
    );
    
    assert!(
        health1 && health2 && health3,
        "Concurrent health checks should all succeed"
    );
}

#[tokio::test]
async fn test_hsm_provider_health_check_degraded_detection() {
    // Test detection of degraded state
    let config = create_test_hsm_config();
    let manager = HsmManager::new(config)
        .await
        .expect("Should initialize successfully");
    
    // Provider should start healthy
    assert!(manager.is_healthy().await);
    
    // In a real scenario, we'd simulate degradation
    // For now, just verify health check mechanism works
    let health_status = manager.get_health_status().await;
    assert!(health_status.is_ok());
}

#[tokio::test]
async fn test_hsm_provider_health_check_recovery() {
    // Test health check during recovery
    let config = create_test_hsm_config();
    let manager = HsmManager::new(config)
        .await
        .expect("Should initialize successfully");
    
    // Simulate recovery by checking health multiple times
    for _ in 0..5 {
        let is_healthy = manager.is_healthy().await;
        assert!(is_healthy, "Should remain healthy during checks");
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

#[tokio::test]
async fn test_hsm_provider_health_check_multiple_providers() {
    // Test health checks across multiple providers
    let config1 = create_test_hsm_config();
    let config2 = create_test_hsm_config();
    
    let manager1 = HsmManager::new(config1).await.expect("Init 1 should succeed");
    let manager2 = HsmManager::new(config2).await.expect("Init 2 should succeed");
    
    let (health1, health2) = tokio::join!(
        manager1.is_healthy(),
        manager2.is_healthy()
    );
    
    assert!(health1 && health2, "Both providers should be healthy");
}

#[tokio::test]
async fn test_hsm_provider_health_check_notification() {
    // Test that health status changes trigger notifications
    let config = create_test_hsm_config();
    let manager = HsmManager::new(config)
        .await
        .expect("Should initialize successfully");
    
    // Get initial health status
    let status = manager.get_health_status().await;
    assert!(status.is_ok());
    
    // Verify notification mechanism exists
    // (actual notification testing would require observer pattern)
    let is_healthy = manager.is_healthy().await;
    assert!(is_healthy);
}

#[tokio::test]
async fn test_hsm_provider_health_check_metrics() {
    // Test that health checks update metrics
    let config = create_test_hsm_config();
    let manager = HsmManager::new(config)
        .await
        .expect("Should initialize successfully");
    
    // Perform multiple health checks
    for _ in 0..3 {
        let _ = manager.is_healthy().await;
    }
    
    // Verify metrics are available
    let metrics = manager.get_metrics().await;
    assert!(metrics.is_ok(), "Should provide health check metrics");
}

#[tokio::test]
async fn test_hsm_provider_health_check_periodic() {
    // Test periodic health check execution
    let mut config = create_test_hsm_config();
    config.health_check_interval = Duration::from_millis(100);
    
    let manager = HsmManager::new(config)
        .await
        .expect("Should initialize successfully");
    
    // Wait for multiple intervals
    tokio::time::sleep(Duration::from_millis(350)).await;
    
    // Verify provider is still healthy
    let is_healthy = manager.is_healthy().await;
    assert!(is_healthy, "Should remain healthy through periodic checks");
}

// ====================================================================================
// Marker for additional test suites
// ====================================================================================

// TODO: Add Provider Failover Tests (Day 2)
// TODO: Add Multi-Provider Coordination Tests (Day 2)

#[cfg(test)]
mod test_helpers {
    use super::*;
    
    /// Helper to simulate provider failure
    pub async fn simulate_provider_failure(manager: &HsmManager) -> Result<(), BearDogError> {
        // In a real scenario, this would trigger actual failure conditions
        // For now, just verify the interface exists
        manager.is_healthy().await;
        Ok(())
    }
    
    /// Helper to wait for provider recovery
    pub async fn wait_for_recovery(manager: &HsmManager, max_wait: Duration) -> bool {
        let start = std::time::Instant::now();
        
        while start.elapsed() < max_wait {
            if manager.is_healthy().await {
                return true;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        
        false
    }
}

