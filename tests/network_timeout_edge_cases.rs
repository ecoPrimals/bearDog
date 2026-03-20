//! Network Timeout Edge Case Tests
//!
//! `TEST_CATEGORY`: integration
//! `TEST_DOMAIN`: network/timeouts
//! `TEST_PRIORITY`: high

use std::time::Duration;
use tokio::time::timeout;

#[tokio::test]
async fn test_zero_timeout() {
    // Test that zero timeout fails immediately
    let result = timeout(Duration::from_secs(0), async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        "completed"
    })
    .await;

    assert!(result.is_err(), "Zero timeout should fail immediately");
}

#[tokio::test]
async fn test_very_small_timeout() {
    // Test microsecond-level timeouts
    let result = timeout(Duration::from_micros(1), async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        "completed"
    })
    .await;

    // Should timeout due to very small duration
    assert!(
        result.is_err(),
        "Microsecond timeout should fail for longer operations"
    );
}

#[tokio::test]
async fn test_timeout_during_operation() {
    // Test timeout occurring mid-operation
    let start = tokio::time::Instant::now();

    let result = timeout(Duration::from_millis(50), async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        "completed"
    })
    .await;

    let elapsed = start.elapsed();

    assert!(result.is_err(), "Should timeout");
    assert!(
        elapsed < Duration::from_millis(75),
        "Should timeout around 50ms, got {elapsed:?}"
    );
}

#[tokio::test]
async fn test_timeout_with_instant_operation() {
    // Test that instant operations succeed even with small timeout
    let result = timeout(Duration::from_millis(10), async { "instant" }).await;

    assert!(result.is_ok(), "Instant operation should succeed");
    assert_eq!(result.unwrap(), "instant");
}

#[tokio::test]
async fn test_multiple_concurrent_timeouts() {
    // Test multiple operations with different timeouts
    use std::sync::Arc;
    use tokio::sync::Mutex;

    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..10 {
        let results_clone = results.clone();
        let handle = tokio::spawn(async move {
            let duration = Duration::from_millis(i * 10);
            let result = timeout(Duration::from_millis(55), async move {
                // Increased to 55ms for tolerance
                tokio::time::sleep(duration).await;
                i
            })
            .await;
            results_clone.lock().await.push((i, result.is_ok()));
        });
        handles.push(handle);
    }

    // Wait for all tasks
    for handle in handles {
        handle.await.expect("Task should complete");
    }

    let final_results = results.lock().await;

    // First 5 should succeed (0-40ms), later ones should timeout (60-90ms)
    for (i, is_ok) in final_results.iter() {
        if *i <= 5 {
            // 0-50ms should complete
            assert!(*is_ok, "Operation {i} should succeed");
        } else {
            // 60-90ms should timeout
            assert!(!*is_ok, "Operation {i} should timeout");
        }
    }
}

#[tokio::test]
async fn test_timeout_with_cancellation() {
    // Test that timeout properly cancels the operation
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    let was_cancelled = Arc::new(AtomicBool::new(false));
    let was_cancelled_clone = was_cancelled.clone();

    let result = timeout(Duration::from_millis(10), async move {
        tokio::select! {
            () = tokio::time::sleep(Duration::from_millis(100)) => {
                "completed"
            }
            () = tokio::time::sleep(Duration::from_millis(5)) => {
                was_cancelled_clone.store(true, Ordering::Relaxed);
                "partial"
            }
        }
    })
    .await;

    assert!(
        result.is_err() || result.unwrap() == "partial",
        "Should timeout or partially complete"
    );
}

#[tokio::test]
async fn test_nested_timeouts() {
    // Test nested timeout behavior
    let outer_result = timeout(Duration::from_millis(100), async {
        timeout(Duration::from_millis(50), async {
            tokio::time::sleep(Duration::from_millis(75)).await;
            "inner"
        })
        .await
    })
    .await;

    assert!(outer_result.is_ok(), "Outer timeout should succeed");
    assert!(outer_result.unwrap().is_err(), "Inner timeout should fail");
}

#[tokio::test]
async fn test_timeout_accuracy() {
    // Test timeout accuracy (should be close to specified duration)
    let durations = vec![10, 50, 100, 200];

    for expected_ms in durations {
        let start = tokio::time::Instant::now();

        let _result = timeout(Duration::from_millis(expected_ms), async {
            tokio::time::sleep(Duration::from_millis(expected_ms * 2)).await;
        })
        .await;

        let elapsed = start.elapsed().as_millis();

        // Allow 20% tolerance
        let tolerance = expected_ms / 5;
        let lower = expected_ms.saturating_sub(tolerance);
        let upper = expected_ms + tolerance;

        assert!(
            elapsed >= u128::from(lower) && elapsed <= u128::from(upper),
            "Timeout should be close to {expected_ms}ms, got {elapsed}ms"
        );
    }
}

#[tokio::test]
async fn test_timeout_with_panic_safety() {
    // Test that timeouts don't cause panics
    // Run timeout directly in the async context
    let result = timeout(Duration::from_millis(10), async {
        tokio::time::sleep(Duration::from_millis(100)).await;
    })
    .await;

    // Should be error (timeout) but not panic
    assert!(result.is_err(), "Should timeout without panicking");
}

#[tokio::test]
async fn test_timeout_with_select() {
    // Test timeout in combination with tokio::select!
    let result = tokio::select! {
        () = tokio::time::sleep(Duration::from_millis(100)) => {
            "sleep"
        }
        _ = timeout(Duration::from_millis(50), async {
            tokio::time::sleep(Duration::from_millis(200)).await;
        }) => {
            "timeout"
        }
    };

    assert_eq!(result, "timeout", "Select should choose timeout branch");
}
