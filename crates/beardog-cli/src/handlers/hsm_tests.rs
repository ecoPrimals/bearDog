// SPDX-License-Identifier: AGPL-3.0-or-later

// Comprehensive tests for HSM handler
// Following BearDog standards: concurrent, robust, idiomatic
// NO sleeps, NO serial tests - only truly concurrent tests

use super::hsm::{discover_hsms_agnostic, handle_hsm_discover, handle_hsm_list};

// ============================================================================
// AGNOSTIC HSM DISCOVERY TESTS (Concurrent, No Sleeps)
// ============================================================================

#[tokio::test]
async fn test_discover_hsms_agnostic_returns_ok() {
    // Universal discovery should always succeed (even if empty)
    let result = discover_hsms_agnostic().await;
    assert!(result.is_ok(), "Agnostic discovery should not error");
}

#[tokio::test]
async fn test_discover_hsms_agnostic_returns_vec() {
    let hsms = discover_hsms_agnostic()
        .await
        .expect("discovery should succeed");

    // Should return a vector (empty or populated)
    // Note: Vec::len() is always >= 0, this just verifies the call succeeds
    let _count = hsms.len();
}

#[tokio::test]
async fn test_discovered_hsms_have_required_fields() {
    let hsms = discover_hsms_agnostic()
        .await
        .expect("discovery should succeed");

    for hsm in &hsms {
        // Every discovered HSM must have these fields populated
        assert!(!hsm.id.is_empty(), "HSM ID should not be empty");
        assert!(!hsm.name.is_empty(), "HSM name should not be empty");
        assert!(!hsm.tier.is_empty(), "HSM tier should not be empty");
        assert!(!hsm.hsm_type.is_empty(), "HSM type should not be empty");
    }
}

#[tokio::test]
async fn test_hsm_ids_are_unique() {
    let hsms = discover_hsms_agnostic()
        .await
        .expect("discovery should succeed");

    let mut ids = std::collections::HashSet::new();
    for hsm in &hsms {
        assert!(
            ids.insert(hsm.id.clone()),
            "Duplicate HSM ID detected: {}",
            hsm.id
        );
    }
}

// ============================================================================
// CONCURRENT DISCOVERY TESTS (No Sleeps, Truly Parallel)
// ============================================================================

#[tokio::test]
async fn test_concurrent_hsm_discovery() {
    // Test that multiple concurrent discoveries work correctly
    // This is a real concurrency test - no sleeps, just parallel execution

    let handles: Vec<_> = (0..10)
        .map(|_| tokio::spawn(async move { discover_hsms_agnostic().await }))
        .collect();

    // All concurrent discoveries should succeed
    for handle in handles {
        let result = handle.await.expect("task should not panic");
        assert!(result.is_ok(), "Concurrent discovery should succeed");
    }
}

#[tokio::test]
async fn test_discovery_is_deterministic() {
    // Same discovery should return same results
    let result1 = discover_hsms_agnostic().await.expect("first discovery");
    let result2 = discover_hsms_agnostic().await.expect("second discovery");

    // Should find same number of HSMs
    assert_eq!(
        result1.len(),
        result2.len(),
        "Discovery should be deterministic"
    );

    // Should find same HSM IDs
    let ids1: std::collections::HashSet<_> = result1.iter().map(|h| &h.id).collect();
    let ids2: std::collections::HashSet<_> = result2.iter().map(|h| &h.id).collect();
    assert_eq!(ids1, ids2, "Should discover same HSMs");
}

// ============================================================================
// CLI HSM INFO TESTS (Data Structure Validation)
// ============================================================================

// CliHsmInfo tests removed - structure is defined in hsm_agnostic module
// Tests for that module belong in hsm_agnostic integration tests

// ============================================================================
// HANDLER FUNCTION TESTS (Integration)
// ============================================================================

#[tokio::test]
async fn test_handle_hsm_list_succeeds() {
    // Should always succeed (even if no HSMs found)
    let result = handle_hsm_list().await;
    assert!(result.is_ok(), "handle_hsm_list should not error");
}

#[tokio::test]
async fn test_handle_hsm_discover_verbose_succeeds() {
    let result = handle_hsm_discover(true).await;
    assert!(result.is_ok(), "handle_hsm_discover should not error");
}

#[tokio::test]
async fn test_handle_hsm_discover_non_verbose_succeeds() {
    let result = handle_hsm_discover(false).await;
    assert!(result.is_ok(), "handle_hsm_discover should not error");
}

// ============================================================================
// ROBUSTNESS TESTS (Concurrent Stress)
// ============================================================================

#[tokio::test]
async fn test_rapid_concurrent_discoveries() {
    // Stress test: 50 concurrent discoveries
    // NO sleeps - pure concurrent execution
    let handles: Vec<_> = (0..50)
        .map(|_| tokio::spawn(async move { discover_hsms_agnostic().await }))
        .collect();

    let mut success_count = 0;
    for handle in handles {
        if let Ok(Ok(_)) = handle.await {
            success_count += 1;
        }
    }

    // All should succeed
    assert_eq!(
        success_count, 50,
        "All concurrent discoveries should succeed"
    );
}

#[tokio::test]
async fn test_discovery_under_load() {
    // Test discovery while system is under load
    // Spawn background tasks to create contention
    let _background: Vec<_> = (0..20)
        .map(|_| {
            tokio::spawn(async move {
                for _ in 0..100 {
                    let () = tokio::task::yield_now().await;
                }
            })
        })
        .collect();

    // Discovery should still work under load
    let result = discover_hsms_agnostic().await;
    assert!(result.is_ok(), "Discovery should work under load");
}
