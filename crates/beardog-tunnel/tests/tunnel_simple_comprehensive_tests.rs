// SPDX-License-Identifier: AGPL-3.0-only
//! Simplified Comprehensive Tests for BearDog Tunnel
//!
//! Coverage expansion for beardog-tunnel crate - Phase 2
//! Focuses on SessionManager which is the primary public API

use beardog_tunnel::SessionManager;

// ============================================================================
// SessionManager Tests
// ============================================================================

#[tokio::test]
async fn test_session_manager_creation() {
    let manager = SessionManager::new();
    assert_eq!(manager.session_count().await, 0);
}

#[tokio::test]
async fn test_session_manager_multiple_instances() {
    let manager1 = SessionManager::new();
    let manager2 = SessionManager::new();

    assert_eq!(manager1.session_count().await, 0);
    assert_eq!(manager2.session_count().await, 0);
}

#[tokio::test]
async fn test_session_manager_debug() {
    let manager = SessionManager::new();
    let debug_str = format!("{:?}", manager);

    assert!(!debug_str.is_empty());
}

#[tokio::test]
async fn test_session_manager_sequential_operations() {
    let manager = SessionManager::new();

    for _ in 0..10 {
        let count = manager.session_count().await;
        assert_eq!(count, 0);
    }
}

#[tokio::test]
async fn test_concurrent_manager_creation() {
    let handles: Vec<_> = (0..10)
        .map(|_| {
            tokio::spawn(async {
                let manager = SessionManager::new();
                manager.session_count().await
            })
        })
        .collect();

    for handle in handles {
        let count = handle.await.expect("Task should complete");
        assert_eq!(count, 0);
    }
}

#[tokio::test]
async fn test_manager_lifecycle() {
    {
        let manager = SessionManager::new();
        assert_eq!(manager.session_count().await, 0);
    } // Manager dropped here

    // Create new manager after previous was dropped
    let manager2 = SessionManager::new();
    assert_eq!(manager2.session_count().await, 0);
}

#[tokio::test]
async fn test_parallel_manager_instances() {
    let handles: Vec<_> = (0..20)
        .map(|_| {
            tokio::spawn(async {
                let manager = SessionManager::new();
                let count = manager.session_count().await;
                assert_eq!(count, 0);
                manager
            })
        })
        .collect();

    for handle in handles {
        let _manager = handle.await.expect("Task should complete");
    }
}

#[tokio::test]
async fn test_manager_rapid_creation_and_drop() {
    for _ in 0..100 {
        let manager = SessionManager::new();
        let _ = manager.session_count().await;
        // Drop immediately
    }
}

#[tokio::test]
async fn test_manager_debug_multiple_times() {
    let manager = SessionManager::new();

    for _ in 0..10 {
        let debug_str = format!("{:?}", manager);
        assert!(!debug_str.is_empty());
    }
}

#[tokio::test]
async fn test_manager_count_consistency() {
    let manager = SessionManager::new();

    // Check count multiple times - should be consistent
    let count1 = manager.session_count().await;
    let count2 = manager.session_count().await;
    let count3 = manager.session_count().await;

    assert_eq!(count1, count2);
    assert_eq!(count2, count3);
}

#[tokio::test]
async fn test_manager_with_timeout() {
    let manager = SessionManager::new();

    let result =
        tokio::time::timeout(std::time::Duration::from_secs(1), manager.session_count()).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
