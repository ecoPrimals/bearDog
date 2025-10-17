//! Component Manager Tests
//!
//! Comprehensive testing of `ComponentManager` functionality including:
//! - Component registration and tracking
//! - Status updates and retrieval
//! - Health monitoring
//! - System health aggregation

use super::super::components::ComponentManager;
use beardog_types::canonical::{ComponentStatus, HealthStatus};

#[tokio::test]
async fn test_component_manager_creation() {
    let manager = ComponentManager::new();

    // New manager should have no components
    let components = manager.get_all_components().await.unwrap();
    assert!(components.is_empty());
}

#[tokio::test]
async fn test_component_registration() {
    let manager = ComponentManager::new();

    // Register a component
    manager
        .register_component("security", ComponentStatus::Running)
        .await
        .unwrap();

    // Verify component is registered
    let status = manager.get_component_status("security").await.unwrap();
    assert_eq!(status, ComponentStatus::Running);
}

#[tokio::test]
async fn test_multiple_component_registration() {
    let manager = ComponentManager::new();

    // Register multiple components
    manager
        .register_component("security", ComponentStatus::Running)
        .await
        .unwrap();
    manager
        .register_component("auth", ComponentStatus::Running)
        .await
        .unwrap();
    manager
        .register_component("monitor", ComponentStatus::Starting)
        .await
        .unwrap();

    // Verify all components are registered
    let components = manager.get_all_components().await.unwrap();
    assert_eq!(components.len(), 3);
    assert!(components.contains_key("security"));
    assert!(components.contains_key("auth"));
    assert!(components.contains_key("monitor"));
}

#[tokio::test]
async fn test_component_status_update() {
    let manager = ComponentManager::new();

    // Register component
    manager
        .register_component("security", ComponentStatus::Starting)
        .await
        .unwrap();

    // Update status
    manager
        .update_component_status("security", ComponentStatus::Running)
        .await
        .unwrap();

    // Verify status changed
    let status = manager.get_component_status("security").await.unwrap();
    assert_eq!(status, ComponentStatus::Running);
}

#[tokio::test]
async fn test_component_status_update_nonexistent() {
    let manager = ComponentManager::new();

    // Try to update non-existent component
    let result = manager
        .update_component_status("nonexistent", ComponentStatus::Running)
        .await;

    // Should return error
    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_nonexistent_component() {
    let manager = ComponentManager::new();

    // Try to get non-existent component
    let result = manager.get_component_status("nonexistent").await;

    // Should return error
    assert!(result.is_err());
}

#[tokio::test]
async fn test_all_components_healthy_when_all_running() {
    let manager = ComponentManager::new();

    // Register all components as running
    manager
        .register_component("security", ComponentStatus::Running)
        .await
        .unwrap();
    manager
        .register_component("auth", ComponentStatus::Running)
        .await
        .unwrap();
    manager
        .register_component("monitor", ComponentStatus::Running)
        .await
        .unwrap();

    // All should be healthy
    let all_healthy = manager.all_components_healthy().await.unwrap();
    assert!(all_healthy);
}

#[tokio::test]
async fn test_all_components_healthy_with_failed_or_stopped_component() {
    let manager = ComponentManager::new();

    // Register components with one not running
    manager
        .register_component("security", ComponentStatus::Running)
        .await
        .unwrap();
    manager
        .register_component("auth", ComponentStatus::Stopping)
        .await
        .unwrap();
    manager
        .register_component("monitor", ComponentStatus::Running)
        .await
        .unwrap();

    // Should not all be healthy
    let all_healthy = manager.all_components_healthy().await.unwrap();
    assert!(!all_healthy);
}

#[tokio::test]
async fn test_all_components_healthy_with_failed_component() {
    let manager = ComponentManager::new();

    // Register components with one failed
    manager
        .register_component("security", ComponentStatus::Running)
        .await
        .unwrap();
    manager
        .register_component("auth", ComponentStatus::Failed)
        .await
        .unwrap();

    // Should not all be healthy
    let all_healthy = manager.all_components_healthy().await.unwrap();
    assert!(!all_healthy);
}

#[tokio::test]
async fn test_system_health_when_all_healthy() {
    let manager = ComponentManager::new();

    // Register all components as running
    manager
        .register_component("security", ComponentStatus::Running)
        .await
        .unwrap();
    manager
        .register_component("auth", ComponentStatus::Running)
        .await
        .unwrap();

    // System health should be Healthy
    let health = manager.get_system_health().await.unwrap();
    assert_eq!(health, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_system_health_when_not_all_running() {
    let manager = ComponentManager::new();

    // Register components with one not running
    manager
        .register_component("security", ComponentStatus::Running)
        .await
        .unwrap();
    manager
        .register_component("auth", ComponentStatus::Starting)
        .await
        .unwrap();

    // System health should be Degraded
    let health = manager.get_system_health().await.unwrap();
    assert_eq!(health, HealthStatus::Degraded);
}

#[tokio::test]
async fn test_component_manager_default() {
    let manager = ComponentManager::default();

    // Default should create empty manager
    let components = manager.get_all_components().await.unwrap();
    assert!(components.is_empty());
}

#[tokio::test]
async fn test_concurrent_component_access() {
    let manager = ComponentManager::new();

    // Register component
    manager
        .register_component("security", ComponentStatus::Running)
        .await
        .unwrap();

    // Spawn multiple concurrent reads
    let manager_clone1 = manager.clone();
    let manager_clone2 = manager.clone();

    let handle1 =
        tokio::spawn(async move { manager_clone1.get_component_status("security").await });

    let handle2 = tokio::spawn(async move { manager_clone2.get_all_components().await });

    // Both should succeed
    let status1 = handle1.await.unwrap().unwrap();
    let components2 = handle2.await.unwrap().unwrap();

    assert_eq!(status1, ComponentStatus::Running);
    assert_eq!(components2.len(), 1);
}

#[tokio::test]
async fn test_component_lifecycle_transitions() {
    let manager = ComponentManager::new();

    // Test full lifecycle: Starting -> Running -> Stopping -> Stopped
    manager
        .register_component("service", ComponentStatus::Starting)
        .await
        .unwrap();

    let status = manager.get_component_status("service").await.unwrap();
    assert_eq!(status, ComponentStatus::Starting);

    manager
        .update_component_status("service", ComponentStatus::Running)
        .await
        .unwrap();

    let status = manager.get_component_status("service").await.unwrap();
    assert_eq!(status, ComponentStatus::Running);

    manager
        .update_component_status("service", ComponentStatus::Stopping)
        .await
        .unwrap();

    let status = manager.get_component_status("service").await.unwrap();
    assert_eq!(status, ComponentStatus::Stopping);
}

#[tokio::test]
async fn test_empty_manager_is_healthy() {
    let manager = ComponentManager::new();

    // Empty manager should be considered healthy
    let all_healthy = manager.all_components_healthy().await.unwrap();
    assert!(all_healthy);

    let health = manager.get_system_health().await.unwrap();
    assert_eq!(health, HealthStatus::Healthy);
}
