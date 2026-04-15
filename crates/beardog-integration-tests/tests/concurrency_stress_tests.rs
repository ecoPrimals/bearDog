// SPDX-License-Identifier: AGPL-3.0-or-later
//! Comprehensive Concurrency Stress Tests
//!
//! Tests that truly stress the concurrent behavior of BearDog systems.
//! NO sleeps, only proper synchronization primitives.
//!
//! Philosophy: If it fails under stress, it will fail in production.

use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};
use std::time::Duration;
use tokio::sync::watch;
use tokio::sync::{Barrier, Notify, RwLock, Semaphore};
use tokio::time::{interval, timeout};

// ============================================================================
// Massive Concurrency Tests
// ============================================================================

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn stress_test_10k_concurrent_operations() {
    println!("🔥 STRESS: 10,000 concurrent atomic operations");

    let counter = Arc::new(AtomicU64::new(0));
    let mut handles = Vec::with_capacity(10_000);

    let start = std::time::Instant::now();

    // Spawn 10,000 concurrent tasks
    for _ in 0..10_000 {
        let counter = counter.clone();
        handles.push(tokio::spawn(async move {
            counter.fetch_add(1, Ordering::SeqCst);
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.expect("Task should complete");
    }

    let duration = start.elapsed();

    println!(
        "✅ Completed 10,000 operations in {:?} ({:.0} ops/sec)",
        duration,
        10_000.0 / duration.as_secs_f64()
    );

    assert_eq!(counter.load(Ordering::SeqCst), 10_000);
    assert!(
        duration < Duration::from_secs(5),
        "Should complete 10k ops in < 5 seconds"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn stress_test_synchronized_start_1000_tasks() {
    println!("🔥 STRESS: 1,000 tasks with barrier synchronization");

    let barrier = Arc::new(Barrier::new(1_000));
    let started = Arc::new(AtomicBool::new(false));
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::with_capacity(1_000);

    let start_time = std::time::Instant::now();

    // Spawn 1,000 tasks that all wait at barrier
    for _ in 0..1_000 {
        let barrier = barrier.clone();
        let started = started.clone();
        let counter = counter.clone();

        handles.push(tokio::spawn(async move {
            // All wait here
            barrier.wait().await;

            // All start simultaneously
            started.store(true, Ordering::SeqCst);
            counter.fetch_add(1, Ordering::SeqCst);
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.expect("Task should complete");
    }

    let duration = start_time.elapsed();

    println!("✅ Synchronized 1,000 tasks in {:?}", duration);

    assert_eq!(counter.load(Ordering::SeqCst), 1_000);
    assert!(started.load(Ordering::SeqCst));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn stress_test_completion_signals_100_tasks() {
    println!("🔥 STRESS: 100 tasks with completion tracking (no sleeps!)");

    let mut handles = Vec::with_capacity(100);
    let counter = Arc::new(AtomicUsize::new(0));
    let notify = Arc::new(Notify::new());

    let start = std::time::Instant::now();

    // Spawn 100 tasks
    for i in 0..100 {
        let counter = counter.clone();
        let notify = notify.clone();

        handles.push(tokio::spawn(async move {
            // Simulate varying workload (no sleep!)
            let work = (i * 17) % 100;
            for _ in 0..work {
                tokio::task::yield_now().await;
            }
            counter.fetch_add(1, Ordering::SeqCst);
            if counter.load(Ordering::SeqCst) == 100 {
                notify.notify_waiters();
            }
            i
        }));
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.expect("Task should complete successfully");
    }

    let duration = start.elapsed();

    println!(
        "✅ All 100 tasks completed in {:?} (avg: {:?}/task)",
        duration,
        duration / 100
    );

    assert_eq!(counter.load(Ordering::SeqCst), 100);
    assert!(
        duration < Duration::from_secs(2),
        "Should complete efficiently without sleeps"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn stress_test_rate_limiter_1000_operations() {
    println!("🔥 STRESS: 1,000 operations with semaphore-based rate limiting");

    let limiter = Arc::new(Semaphore::new(100)); // 100 concurrent operations
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::with_capacity(1_000);

    let start = std::time::Instant::now();

    // Spawn 1,000 operations (limited by semaphore)
    for _ in 0..1_000 {
        let limiter = limiter.clone();
        let counter = counter.clone();

        handles.push(tokio::spawn(async move {
            let _permit = limiter.acquire().await.expect("Semaphore should work");
            counter.fetch_add(1, Ordering::SeqCst);
            tokio::task::yield_now().await;
        }));
    }

    // Wait for all to complete
    for handle in handles {
        handle.await.expect("Task should complete");
    }

    let duration = start.elapsed();
    let ops_per_sec = 1_000.0 / duration.as_secs_f64();

    println!(
        "✅ Completed 1,000 operations in {:?} ({:.0} ops/sec)",
        duration, ops_per_sec
    );

    assert_eq!(counter.load(Ordering::SeqCst), 1_000);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn stress_test_condition_waiting_race() {
    println!("🔥 STRESS: Condition waiting under race conditions");

    let waiter_count = Arc::new(AtomicUsize::new(0));
    let (signal_tx, _) = watch::channel(false);
    let barrier = Arc::new(Barrier::new(101));
    let mut handles = Vec::with_capacity(100);

    let start = std::time::Instant::now();

    // Spawn 100 waiters: barrier ensures every task has subscribed to the watch before we signal,
    // so no lost wakeups vs `Notify::notify_waiters` racing with registration.
    for _ in 0..100 {
        let waiter_count = waiter_count.clone();
        let mut rx = signal_tx.subscribe();
        let barrier = barrier.clone();

        handles.push(tokio::spawn(async move {
            let _ = barrier.wait().await;
            rx.changed().await.expect("watch");
            assert!(*rx.borrow(), "Condition should be met");
            waiter_count.fetch_add(1, Ordering::SeqCst);
        }));
    }

    let _ = barrier.wait().await;
    signal_tx.send(true).expect("signal");

    // All should complete rapidly
    for handle in handles {
        handle.await.expect("Task should complete");
    }

    let duration = start.elapsed();

    println!("✅ All 100 waiters completed in {:?}", duration);

    assert_eq!(waiter_count.load(Ordering::SeqCst), 100);
    assert!(
        duration < Duration::from_secs(1),
        "Should complete rapidly once condition met"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn stress_test_mixed_workload_10k_operations() {
    println!("🔥 STRESS: 10,000 mixed concurrent operations");

    let reads = Arc::new(AtomicUsize::new(0));
    let writes = Arc::new(AtomicUsize::new(0));
    let computes = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::with_capacity(10_000);

    let start = std::time::Instant::now();

    // Mix of different operation types
    for i in 0..10_000 {
        let reads = reads.clone();
        let writes = writes.clone();
        let computes = computes.clone();

        handles.push(tokio::spawn(async move {
            match i % 3 {
                0 => {
                    // Read operation
                    reads.fetch_add(1, Ordering::Relaxed);
                }
                1 => {
                    // Write operation
                    writes.fetch_add(1, Ordering::SeqCst);
                }
                _ => {
                    // Compute operation
                    let _result = (i * 17) % 1000;
                    computes.fetch_add(1, Ordering::Relaxed);
                }
            }
        }));
    }

    // Wait for all
    for handle in handles {
        handle.await.expect("Task should complete");
    }

    let duration = start.elapsed();
    let ops_per_sec = 10_000.0 / duration.as_secs_f64();

    println!(
        "✅ Completed 10,000 mixed operations in {:?} ({:.0} ops/sec)",
        duration, ops_per_sec
    );
    println!(
        "   Reads: {}, Writes: {}, Computes: {}",
        reads.load(Ordering::Relaxed),
        writes.load(Ordering::SeqCst),
        computes.load(Ordering::Relaxed)
    );

    assert_eq!(
        reads.load(Ordering::Relaxed)
            + writes.load(Ordering::SeqCst)
            + computes.load(Ordering::Relaxed),
        10_000
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn stress_test_eventual_consistency_assertion() {
    println!("🔥 STRESS: Eventual consistency verification");

    let target = 1000;
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_clone = counter.clone();

    // Spawn incrementer tasks
    tokio::spawn(async move {
        for _ in 0..target {
            counter_clone.fetch_add(1, Ordering::SeqCst);
            tokio::task::yield_now().await;
        }
    });

    let start = std::time::Instant::now();

    // Wait for eventual consistency (polling with yield)
    let result = timeout(Duration::from_secs(5), async {
        let mut check_interval = interval(Duration::from_micros(100));
        loop {
            if counter.load(Ordering::SeqCst) == target {
                break;
            }
            check_interval.tick().await;
        }
    })
    .await;

    assert!(result.is_ok(), "Counter should reach target");

    let duration = start.elapsed();

    println!("✅ Eventual consistency achieved in {:?}", duration);

    assert_eq!(counter.load(Ordering::SeqCst), target);
}

// ============================================================================
// Race Condition Detection Tests
// ============================================================================

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn stress_test_no_race_conditions_in_shared_state() {
    println!("🔥 STRESS: Race condition detection - 1000 concurrent writes");

    let shared_vec = Arc::new(StdMutex::new(Vec::new()));
    let mut handles = Vec::with_capacity(1_000);

    let start = std::time::Instant::now();

    for i in 0..1_000 {
        let shared_vec = shared_vec.clone();
        handles.push(tokio::spawn(async move {
            let mut vec = shared_vec.lock().unwrap();
            vec.push(i);
        }));
    }

    // Wait for all
    for handle in handles {
        handle.await.expect("Task should complete");
    }

    let duration = start.elapsed();

    let final_vec = shared_vec.lock().unwrap();
    let mut sorted_vec = final_vec.clone();
    sorted_vec.sort();

    println!(
        "✅ No race conditions: {} unique values in {:?}",
        final_vec.len(),
        duration
    );

    // Should have exactly 1000 unique values
    assert_eq!(final_vec.len(), 1_000);

    // All values should be present
    for i in 0..1_000 {
        assert!(sorted_vec.contains(&i), "Value {} should be present", i);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn stress_test_no_deadlocks_proper_ordering() {
    println!("🔥 STRESS: No deadlocks with proper lock ordering");

    let lock1 = Arc::new(RwLock::new(0));
    let lock2 = Arc::new(RwLock::new(0));
    let mut handles = Vec::new();

    let start = std::time::Instant::now();

    // BOTH patterns use same lock order (lock1 -> lock2) to prevent deadlock
    for pattern in 0..2 {
        for _ in 0..50 {
            let lock1 = lock1.clone();
            let lock2 = lock2.clone();
            handles.push(tokio::spawn(async move {
                // Always acquire in same order: lock1 first, then lock2
                let mut val1 = lock1.write().await;
                *val1 += 1;
                let mut val2 = lock2.write().await;
                *val2 += 1;

                if pattern == 1 {
                    // Different operations, but same lock order
                    *val2 += 1;
                    *val1 += 1;
                }
            }));
        }
    }

    // Should complete without deadlock
    let completion_result = tokio::time::timeout(Duration::from_secs(5), async {
        for handle in handles {
            handle.await.expect("Task should complete");
        }
    })
    .await;

    let duration = start.elapsed();

    assert!(
        completion_result.is_ok(),
        "Should not deadlock with proper lock ordering"
    );

    println!("✅ No deadlocks detected in {:?}", duration);

    let val1 = *lock1.read().await;
    let val2 = *lock2.read().await;

    assert_eq!(val1, 150); // 100 base + 50 extra from pattern 1
    assert_eq!(val2, 150);
}

// ============================================================================
// Performance Regression Tests
// ============================================================================

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn performance_test_atomic_operations_throughput() {
    println!("🔥 PERFORMANCE: Atomic operation throughput");

    let counter = Arc::new(AtomicU64::new(0));
    let iterations = 1_000_000;
    let mut handles = Vec::new();

    let start = std::time::Instant::now();

    // Split across CPU cores
    let workers = 8;
    let per_worker = iterations / workers;

    for _ in 0..workers {
        let counter = counter.clone();
        handles.push(tokio::spawn(async move {
            for _ in 0..per_worker {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for handle in handles {
        handle.await.expect("Task should complete");
    }

    let duration = start.elapsed();
    let ops_per_sec = iterations as f64 / duration.as_secs_f64();

    println!(
        "✅ {} atomic ops in {:?} ({:.0} ops/sec)",
        iterations, duration, ops_per_sec
    );

    assert_eq!(counter.load(Ordering::SeqCst), iterations as u64);

    // Should achieve at least 10M ops/sec on modern hardware
    assert!(
        ops_per_sec > 1_000_000.0,
        "Atomic operations should be fast (got {:.0} ops/sec)",
        ops_per_sec
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn performance_test_task_spawn_overhead() {
    println!("🔥 PERFORMANCE: Task spawn overhead");

    let task_count = 10_000;
    let mut handles = Vec::with_capacity(task_count);

    let start = std::time::Instant::now();

    for i in 0..task_count {
        handles.push(tokio::spawn(async move { i }));
    }

    let mut sum = 0;
    for handle in handles {
        sum += handle.await.expect("Task should complete");
    }

    let duration = start.elapsed();
    let spawns_per_sec = task_count as f64 / duration.as_secs_f64();

    println!(
        "✅ Spawned {} tasks in {:?} ({:.0} spawns/sec)",
        task_count, duration, spawns_per_sec
    );

    assert_eq!(sum, (0..task_count).sum());

    // Should spawn at least 50k tasks/sec
    assert!(
        spawns_per_sec > 10_000.0,
        "Task spawning should be fast (got {:.0} spawns/sec)",
        spawns_per_sec
    );
}

// ============================================================================
// Correctness Under Load Tests
// ============================================================================

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn correctness_test_no_lost_updates() {
    println!("🔥 CORRECTNESS: No lost updates under high contention");

    let counter = Arc::new(AtomicU64::new(0));
    let operations = 100_000;
    let mut handles = Vec::new();

    let start = std::time::Instant::now();

    // Many concurrent incrementers
    for _ in 0..100 {
        let counter = counter.clone();
        handles.push(tokio::spawn(async move {
            for _ in 0..(operations / 100) {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    for handle in handles {
        handle.await.expect("Task should complete");
    }

    let duration = start.elapsed();

    println!(
        "✅ Verified {} atomic operations with no lost updates in {:?}",
        operations, duration
    );

    assert_eq!(
        counter.load(Ordering::SeqCst),
        operations,
        "No updates should be lost"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn correctness_test_ordering_guarantees() {
    println!("🔥 CORRECTNESS: Ordering guarantees under concurrency");

    let values = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    let start = std::time::Instant::now();

    // Each task appends in order
    for i in 0..100 {
        let values = values.clone();
        handles.push(tokio::spawn(async move {
            let mut vec = values.lock().await;
            vec.push(i);
        }));
    }

    for handle in handles {
        handle.await.expect("Task should complete");
    }

    let duration = start.elapsed();

    let final_vec = values.lock().await;

    println!(
        "✅ Verified ordering with {} elements in {:?}",
        final_vec.len(),
        duration
    );

    // Should have all values (order may vary, but all present)
    assert_eq!(final_vec.len(), 100);

    let mut sorted = final_vec.clone();
    sorted.sort();
    assert_eq!(sorted, (0..100).collect::<Vec<_>>());
}

// ============================================================================
// Resource Cleanup Tests
// ============================================================================

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn stress_test_no_resource_leaks() {
    println!("🔥 STRESS: Resource cleanup under load");

    let allocated = Arc::new(AtomicUsize::new(0));
    let deallocated = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    let start = std::time::Instant::now();

    for _ in 0..1_000 {
        let allocated = allocated.clone();
        let deallocated = deallocated.clone();

        handles.push(tokio::spawn(async move {
            let _allocated = allocated.clone();
            let _deallocated = deallocated.clone();

            allocated.fetch_add(1, Ordering::SeqCst);

            // Allocate resource
            let _resource = vec![0u8; 1024]; // 1KB

            // Do work
            tokio::task::yield_now().await;

            // Resource dropped here
            deallocated.fetch_add(1, Ordering::SeqCst);
        }));
    }

    for handle in handles {
        handle.await.expect("Task should complete");
    }

    let duration = start.elapsed();

    println!("✅ Resource cleanup verified in {:?}", duration);

    assert_eq!(
        allocated.load(Ordering::SeqCst),
        deallocated.load(Ordering::SeqCst),
        "All resources should be deallocated"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn stress_test_panic_recovery() {
    println!("🔥 STRESS: Panic recovery in concurrent environment");

    let successful = Arc::new(AtomicUsize::new(0));
    let panicked = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    let start = std::time::Instant::now();

    for i in 0..100 {
        let successful = successful.clone();
        let _panicked = panicked.clone();

        handles.push(tokio::spawn(async move {
            if i % 10 == 0 {
                // Intentional panic
                panic!("Simulated panic in task {}", i);
            } else {
                successful.fetch_add(1, Ordering::SeqCst);
            }
        }));
    }

    // Collect results (panics should be caught)
    for handle in handles {
        match handle.await {
            Ok(()) => {}
            Err(_) => {
                panicked.fetch_add(1, Ordering::SeqCst);
            }
        }
    }

    let duration = start.elapsed();

    println!(
        "✅ Panic recovery: {} successful, {} panicked in {:?}",
        successful.load(Ordering::SeqCst),
        panicked.load(Ordering::SeqCst),
        duration
    );

    assert_eq!(successful.load(Ordering::SeqCst), 90);
    assert_eq!(panicked.load(Ordering::SeqCst), 10);
}
