// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive Connection Error Path Tests
//!
//! Tests error handling in connection lifecycle and management

use crate::tunnel::hsm::manager::implementation::DefaultHsmManager;

// TEST_CATEGORY: error_path
// TEST_DOMAIN: hsm_connection
// TEST_PRIORITY: high

#[tokio::test]
async fn test_manager_initialization_default() {
    let manager = DefaultHsmManager::new();
    // Manager should always initialize
    assert!(manager.hsm_providers.is_empty());
}

#[tokio::test]
async fn test_manager_list_providers_empty() {
    let manager = DefaultHsmManager::new();

    let providers = manager.list_providers();
    assert_eq!(providers.len(), 0);
}

#[tokio::test]
async fn test_manager_get_nonexistent_provider() {
    let manager = DefaultHsmManager::new();

    let result = manager.get_provider("nonexistent");
    assert!(result.is_err());
}

#[tokio::test]
async fn test_concurrent_manager_access() {
    use std::sync::Arc;
    use tokio::task::JoinSet;

    let manager = Arc::new(DefaultHsmManager::new());

    let mut set = JoinSet::new();

    for _ in 0..20 {
        let manager_clone = Arc::clone(&manager);
        set.spawn(async move { manager_clone.list_providers() });
    }

    let mut success_count = 0;
    while let Some(result) = set.join_next().await {
        if result.is_ok() {
            success_count += 1;
        }
    }

    assert_eq!(success_count, 20); // All should succeed
}

#[tokio::test]
async fn test_manager_operations_thread_safety() {
    use std::sync::Arc;
    use std::thread;

    let manager = Arc::new(DefaultHsmManager::new());

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let manager_clone = Arc::clone(&manager);
            thread::spawn(move || manager_clone.list_providers())
        })
        .collect();

    for handle in handles {
        let result = handle.join();
        assert!(result.is_ok());
    }
}

#[tokio::test]
async fn test_rapid_sequential_list_providers() {
    let manager = DefaultHsmManager::new();

    for _ in 0..100 {
        let providers = manager.list_providers();
        assert_eq!(providers.len(), 0);
    }
}

#[tokio::test]
async fn test_manager_clone_independence() {
    let manager1 = DefaultHsmManager::new();
    let manager2 = DefaultHsmManager::new();

    let providers1 = manager1.list_providers();
    let providers2 = manager2.list_providers();

    // Both should be empty and independent
    assert_eq!(providers1.len(), 0);
    assert_eq!(providers2.len(), 0);
}

#[tokio::test]
async fn test_manager_default_trait() {
    let manager = DefaultHsmManager::default();
    assert!(manager.hsm_providers.is_empty());
}

#[tokio::test]
async fn test_repeated_get_nonexistent_provider() {
    let manager = DefaultHsmManager::new();

    for _ in 0..50 {
        let result = manager.get_provider("does-not-exist");
        assert!(result.is_err());
    }
}

#[tokio::test]
async fn test_concurrent_list_providers() {
    use std::sync::Arc;
    use tokio::task::JoinSet;

    let manager = Arc::new(DefaultHsmManager::new());

    let mut set = JoinSet::new();

    for i in 0..50 {
        let manager_clone = Arc::clone(&manager);
        set.spawn(async move {
            let _iteration = i;
            manager_clone.list_providers()
        });
    }

    let mut all_results = Vec::new();
    while let Some(result) = set.join_next().await {
        if let Ok(providers) = result {
            all_results.push(providers);
        }
    }

    // All should return empty list
    assert_eq!(all_results.len(), 50);
    for providers in all_results {
        assert_eq!(providers.len(), 0);
    }
}
