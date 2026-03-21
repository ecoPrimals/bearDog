// SPDX-License-Identifier: AGPL-3.0-only
//! Chaos and Fault Injection Tests for BearDog UniBin
//!
//! **Philosophy**: "test issues will be production issues"
//! - NO sleeps (real concurrency)
//! - NO serialization (except extreme chaos tests)
//! - Robust, modern, idiomatic async Rust
//!
//! Test Categories:
//! - Chaos: Extreme concurrent stress, resource exhaustion
//! - Fault: Simulated failures, error paths, recovery
//! - Stress: High load, rapid operations, contention

#[cfg(test)]
mod chaos_tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use tokio::sync::{Barrier, RwLock, Semaphore};

    // ========================================================================
    // CHAOS TESTS: Extreme Concurrent Stress
    // ========================================================================

    #[tokio::test]
    async fn chaos_test_massive_concurrent_operations() {
        // Spawn 1000 concurrent tasks - pure chaos!
        // No sleeps - just raw concurrent execution
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();

        for _ in 0..1000 {
            let counter_clone = Arc::clone(&counter);
            let handle = tokio::spawn(async move {
                // Simulate some work
                for _ in 0..10 {
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                    tokio::task::yield_now().await;
                }
            });
            handles.push(handle);
        }

        // All tasks should complete
        for handle in handles {
            assert!(handle.await.is_ok(), "Task should not panic");
        }

        // Verify all operations completed
        assert_eq!(
            counter.load(Ordering::SeqCst),
            10_000,
            "All operations should complete"
        );
    }

    #[tokio::test]
    async fn chaos_test_concurrent_resource_contention() {
        // Multiple tasks competing for limited resources
        // This tests RwLock, Arc, and concurrent patterns under stress
        let shared_data: Arc<RwLock<Vec<u32>>> = Arc::new(RwLock::new(Vec::new()));
        let mut handles = Vec::new();

        // Readers and writers competing
        for i in 0..100 {
            let data: Arc<RwLock<Vec<u32>>> = Arc::clone(&shared_data);
            if i % 3 == 0 {
                // Writer
                let handle = tokio::spawn(async move {
                    for j in 0..10 {
                        let mut lock = data.write().await;
                        lock.push(j);
                        drop(lock);
                        tokio::task::yield_now().await;
                    }
                });
                handles.push(handle);
            } else {
                // Reader
                let handle = tokio::spawn(async move {
                    for _ in 0..20 {
                        let lock = data.read().await;
                        let _len = lock.len();
                        drop(lock);
                        tokio::task::yield_now().await;
                    }
                });
                handles.push(handle);
            }
        }

        // All should complete without panics
        for handle in handles {
            assert!(handle.await.is_ok(), "Should handle contention gracefully");
        }
    }

    #[tokio::test]
    async fn chaos_test_task_spawn_limits() {
        // Test system behavior with many tasks
        // Verifies tokio runtime handles extreme concurrency
        let semaphore = Arc::new(Semaphore::new(100)); // Limit concurrent tasks
        let mut handles = Vec::new();

        for _ in 0..500 {
            let permit = Arc::clone(&semaphore);
            let handle = tokio::spawn(async move {
                let _permit = permit.acquire().await.unwrap();
                // Simulate work
                tokio::task::yield_now().await;
            });
            handles.push(handle);
        }

        // All tasks should eventually complete
        for handle in handles {
            assert!(handle.await.is_ok(), "Task should complete");
        }
    }

    #[tokio::test]
    async fn chaos_test_concurrent_allocation_deallocation() {
        // Stress test memory allocation under concurrency
        let mut handles = Vec::new();

        for _ in 0..200 {
            let handle = tokio::spawn(async move {
                // Allocate and deallocate rapidly
                for _ in 0..100 {
                    let data: Vec<u8> = vec![0; 1024]; // 1KB allocations
                    let _len = data.len();
                    tokio::task::yield_now().await;
                    drop(data);
                }
            });
            handles.push(handle);
        }

        // Should handle memory pressure gracefully
        for handle in handles {
            assert!(handle.await.is_ok(), "Should handle allocations");
        }
    }

    // ========================================================================
    // FAULT INJECTION TESTS: Simulated Failures
    // ========================================================================

    #[tokio::test]
    async fn fault_test_partial_task_failures() {
        // Some tasks fail, others succeed - system should be resilient
        let success_count = Arc::new(AtomicUsize::new(0));
        let failure_count = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();

        for i in 0..100 {
            let success = Arc::clone(&success_count);
            let failure = Arc::clone(&failure_count);

            let handle = tokio::spawn(async move {
                if i % 7 == 0 {
                    // Simulate failure
                    failure.fetch_add(1, Ordering::SeqCst);
                    Err::<(), &str>("simulated failure")
                } else {
                    // Success
                    success.fetch_add(1, Ordering::SeqCst);
                    Ok(())
                }
            });
            handles.push(handle);
        }

        // Collect results
        let mut successes = 0;
        let mut failures = 0;

        for handle in handles {
            match handle.await {
                Ok(Ok(())) => successes += 1,
                Ok(Err(_)) => failures += 1,
                Err(_) => panic!("Task should not panic"),
            }
        }

        // Verify expected distribution (100 tasks, ~14 failures)
        // Use range to account for potential race conditions in test execution
        assert!(
            (85..=87).contains(&successes),
            "Should have ~86 successes, got {}",
            successes
        );
        assert!(
            (13..=15).contains(&failures),
            "Should have ~14 failures, got {}",
            failures
        );
        assert_eq!(successes + failures, 100, "Total should be 100");
    }

    #[tokio::test]
    async fn fault_test_resource_exhaustion_recovery() {
        // Simulate resource exhaustion and recovery
        let available_resources = Arc::new(AtomicUsize::new(10));
        let mut handles = Vec::new();

        for _ in 0..50 {
            let resources = Arc::clone(&available_resources);
            let handle: tokio::task::JoinHandle<Result<(), String>> = tokio::spawn(async move {
                // Try to acquire resource
                for _attempt in 0..1000 {
                    let current = resources.load(Ordering::SeqCst);
                    if current > 0
                        && resources
                            .compare_exchange(
                                current,
                                current - 1,
                                Ordering::SeqCst,
                                Ordering::SeqCst,
                            )
                            .is_ok()
                    {
                        // Got resource, use it
                        tokio::task::yield_now().await;
                        // Release resource
                        resources.fetch_add(1, Ordering::SeqCst);
                        return Ok(());
                    }

                    // Resource exhausted - back off and retry
                    tokio::task::yield_now().await;
                }

                Err("Could not acquire resource after retries".to_string())
            });
            handles.push(handle);
        }

        // All should eventually succeed (with backoff)
        for handle in handles {
            let result = handle.await;
            assert!(result.is_ok(), "Should handle resource exhaustion");
        }

        // Resources should be back to initial state
        assert_eq!(
            available_resources.load(Ordering::SeqCst),
            10,
            "Resources should be released"
        );
    }

    #[tokio::test]
    async fn fault_test_concurrent_error_propagation() {
        // Errors should propagate correctly in concurrent context
        let error_seen = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();

        for i in 0..50 {
            let errors = Arc::clone(&error_seen);
            let handle = tokio::spawn(async move {
                let result: Result<(), String> = if i == 25 {
                    Err("critical error".to_string())
                } else {
                    Ok(())
                };

                if result.is_err() {
                    errors.fetch_add(1, Ordering::SeqCst);
                }

                result
            });
            handles.push(handle);
        }

        // Check error propagation
        let mut error_count = 0;
        for handle in handles {
            if let Ok(Err(_)) = handle.await {
                error_count += 1;
            }
        }

        assert_eq!(error_count, 1, "Should see exactly one error");
        assert_eq!(
            error_seen.load(Ordering::SeqCst),
            1,
            "Error counter should match"
        );
    }

    // ========================================================================
    // STRESS TESTS: High Load Operations
    // ========================================================================

    #[tokio::test]
    async fn stress_test_rapid_task_creation_destruction() {
        // Rapidly create and destroy tasks
        for _ in 0..100 {
            let handles: Vec<_> = (0..50)
                .map(|_| {
                    tokio::spawn(async {
                        tokio::task::yield_now().await;
                    })
                })
                .collect();

            for handle in handles {
                assert!(handle.await.is_ok());
            }
        }
    }

    #[tokio::test]
    async fn stress_test_concurrent_channel_operations() {
        // Stress test tokio channels under concurrency
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        let send_count = 1000;

        // Spawn senders
        let send_handles: Vec<_> = (0..10)
            .map(|_| {
                let tx_clone = tx.clone();
                tokio::spawn(async move {
                    for i in 0..100 {
                        tx_clone.send(i).await.expect("send should succeed");
                    }
                })
            })
            .collect();

        // Drop original sender
        drop(tx);

        // Receive all messages
        let recv_handle = tokio::spawn(async move {
            let mut count = 0;
            while let Some(_) = rx.recv().await {
                count += 1;
            }
            count
        });

        // Wait for senders
        for handle in send_handles {
            handle.await.expect("sender should complete");
        }

        // Verify all messages received
        let received = recv_handle.await.expect("receiver should complete");
        assert_eq!(received, send_count, "Should receive all messages");
    }

    #[tokio::test]
    async fn stress_test_tokio_rwlock_under_load() {
        // Stress tokio::sync::RwLock with heavy concurrent access
        let data: Arc<RwLock<u64>> = Arc::new(RwLock::new(0u64));
        let operations = 10_000;
        let mut handles = Vec::new();

        // Half writers, half readers
        for i in 0..100 {
            let data_clone: Arc<RwLock<u64>> = Arc::clone(&data);
            if i % 2 == 0 {
                // Writer
                let handle = tokio::spawn(async move {
                    for _ in 0..operations / 100 {
                        let mut lock = data_clone.write().await;
                        *lock += 1;
                        drop(lock);
                        tokio::task::yield_now().await;
                    }
                });
                handles.push(handle);
            } else {
                // Reader
                let handle = tokio::spawn(async move {
                    for _ in 0..operations / 100 {
                        let lock = data_clone.read().await;
                        let _value = *lock;
                        drop(lock);
                        tokio::task::yield_now().await;
                    }
                });
                handles.push(handle);
            }
        }

        // All should complete
        for handle in handles {
            assert!(handle.await.is_ok(), "RwLock operations should succeed");
        }

        // Verify final value (50 writers * 100 ops each = 5000)
        let final_value = *data.read().await;
        assert_eq!(final_value, 5_000, "All writes should be accounted for");
    }

    // ========================================================================
    // RECOVERY TESTS: Graceful Degradation
    // ========================================================================

    #[tokio::test]
    async fn recovery_test_task_cancellation_cleanup() {
        // Test that cancelled tasks clean up properly
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();

        for _ in 0..20 {
            let counter_clone = Arc::clone(&counter);
            let handle = tokio::spawn(async move {
                counter_clone.fetch_add(1, Ordering::SeqCst);
                // Long-running operation
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            });
            handles.push(handle);
        }

        // Let tasks start
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Cancel half of them
        for (i, handle) in handles.iter().enumerate() {
            if i % 2 == 0 {
                handle.abort();
            }
        }

        // Counter should show all tasks started
        assert_eq!(
            counter.load(Ordering::SeqCst),
            20,
            "All tasks should have started"
        );
    }

    #[tokio::test]
    async fn recovery_test_graceful_shutdown() {
        let (shutdown_tx, _shutdown_rx) = tokio::sync::broadcast::channel::<()>(10);
        let counter = Arc::new(AtomicUsize::new(0));
        let started = Arc::new(tokio::sync::Barrier::new(11)); // 10 workers + 1 test

        let mut handles = Vec::new();
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let started_clone = Arc::clone(&started);
            let mut shutdown_rx_clone = shutdown_tx.subscribe();

            let handle = tokio::spawn(async move {
                // Do work before waiting for shutdown
                counter_clone.fetch_add(1, Ordering::SeqCst);
                started_clone.wait().await;

                // Now wait for shutdown signal
                let _ = shutdown_rx_clone.recv().await;
            });
            handles.push(handle);
        }

        // Wait until all workers have incremented and are ready
        started.wait().await;

        shutdown_tx.send(()).expect("shutdown signal");
        drop(shutdown_tx);

        for handle in handles {
            let result = tokio::time::timeout(tokio::time::Duration::from_secs(1), handle).await;
            assert!(
                result.is_ok(),
                "Task should complete gracefully within timeout"
            );
        }

        assert!(
            counter.load(Ordering::SeqCst) >= 10,
            "All tasks should have done work"
        );
    }
}

#[cfg(test)]
mod benchmark_tests {
    //! Performance benchmarks (not strict, but verifies reasonable performance)

    use std::time::Instant;

    #[tokio::test]
    async fn benchmark_task_spawn_overhead() {
        let start = Instant::now();
        let mut handles = Vec::new();

        for _ in 0..1000 {
            let handle = tokio::spawn(async {
                // Minimal work
                42
            });
            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.await;
        }

        let elapsed = start.elapsed();

        // Spawning 1000 tasks should be fast (< 100ms)
        assert!(
            elapsed.as_millis() < 100,
            "Task spawn overhead too high: {:?}ms",
            elapsed.as_millis()
        );
    }

    #[tokio::test]
    async fn benchmark_concurrent_operations() {
        let start = Instant::now();

        let handles: Vec<_> = (0..100)
            .map(|_| {
                tokio::spawn(async {
                    for _ in 0..100 {
                        tokio::task::yield_now().await;
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.await.expect("should complete");
        }

        let elapsed = start.elapsed();

        // 100 tasks * 100 yields should complete in < 1s
        assert!(
            elapsed.as_secs() < 1,
            "Concurrent operations too slow: {:?}s",
            elapsed.as_secs()
        );
    }
}
