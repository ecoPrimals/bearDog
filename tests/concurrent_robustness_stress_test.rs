//! Concurrent Robustness Stress Tests
//!
//! **Purpose**: Verify system behaves correctly under high concurrent load
//! **Philosophy**: Test issues ARE production issues - no sleeps, fully concurrent
//!
//! `TEST_CATEGORY`: stress
//! `TEST_DOMAIN`: `concurrent_safety`
//! `TEST_PRIORITY`: critical

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock, Semaphore};
use tokio::task;

/// Test 1000 concurrent read operations on shared state
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_concurrent_reads_1000_tasks() {
    let shared_data = Arc::new(RwLock::new(vec![1u64, 2, 3, 4, 5]));
    let read_count = Arc::new(AtomicU64::new(0));

    let mut handles = vec![];
    for _ in 0..1000 {
        let data = shared_data.clone();
        let counter = read_count.clone();

        let handle = task::spawn(async move {
            // Multiple reads per task
            for _ in 0..10 {
                let guard = data.read().await;
                let _sum: u64 = guard.iter().sum();
                counter.fetch_add(1, Ordering::Relaxed);
                drop(guard);
                task::yield_now().await; // Cooperative scheduling
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("Read task should complete");
    }

    // All 10,000 reads should complete
    assert_eq!(read_count.load(Ordering::Relaxed), 10_000);
}

/// Test concurrent read/write operations with proper synchronization
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_concurrent_read_write_contention() {
    let counter = Arc::new(RwLock::new(0u64));
    let read_sum = Arc::new(AtomicU64::new(0));
    let mut handles = vec![];

    // 100 writer tasks
    for _ in 0..100 {
        let counter_clone = counter.clone();
        let handle = task::spawn(async move {
            for _ in 0..10 {
                let mut val = counter_clone.write().await;
                *val += 1;
                task::yield_now().await;
            }
        });
        handles.push(handle);
    }

    // 500 reader tasks (more readers than writers)
    for _ in 0..500 {
        let counter_clone = counter.clone();
        let sum_clone = read_sum.clone();
        let handle = task::spawn(async move {
            for _ in 0..5 {
                let val = counter_clone.read().await;
                sum_clone.fetch_add(*val, Ordering::Relaxed);
                task::yield_now().await;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("Task should complete");
    }

    // Verify final counter value (100 writers * 10 increments)
    let final_value = *counter.read().await;
    assert_eq!(final_value, 1000, "All writes should be applied");
}

/// Test message passing under load (no sleeps, pure throughput)
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_channel_throughput_10k_messages() {
    use tokio::sync::mpsc;

    let (tx, mut rx) = mpsc::channel(100);
    let message_count = 10_000;
    let received = Arc::new(AtomicU64::new(0));

    // Sender tasks
    let sender_handles: Vec<_> = (0..10)
        .map(|task_id| {
            let tx = tx.clone();
            task::spawn(async move {
                for msg_id in 0..1000 {
                    tx.send((task_id, msg_id))
                        .await
                        .expect("Channel should not be closed");
                    task::yield_now().await;
                }
            })
        })
        .collect();

    // Receiver task
    let received_clone = received.clone();
    let receiver_handle = task::spawn(async move {
        while let Some((_task_id, _msg_id)) = rx.recv().await {
            received_clone.fetch_add(1, Ordering::Relaxed);
            // Simul ate processing
            let _work = (0..10).sum::<i32>();
        }
    });

    // Wait for all senders
    for handle in sender_handles {
        handle.await.expect("Sender should complete");
    }

    drop(tx); // Close channel to signal receiver
    receiver_handle.await.expect("Receiver should complete");

    assert_eq!(received.load(Ordering::Relaxed), message_count);
}

/// Test semaphore-based resource limiting
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_semaphore_resource_limiting() {
    let max_concurrent = 50;
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    let active_count = Arc::new(AtomicU64::new(0));
    let max_observed = Arc::new(AtomicU64::new(0));

    let mut handles = vec![];
    for _ in 0..1000 {
        let sem = semaphore.clone();
        let active = active_count.clone();
        let max_obs = max_observed.clone();

        let handle = task::spawn(async move {
            let _permit = sem.acquire().await.expect("Semaphore should not be closed");

            // Track concurrent tasks
            let current = active.fetch_add(1, Ordering::SeqCst) + 1;
            max_obs.fetch_max(current, Ordering::SeqCst);

            // Simulate work
            let _work = (0..100).map(|x| x * x).sum::<u64>();
            task::yield_now().await;

            active.fetch_sub(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("Task should complete");
    }

    // Should never exceed semaphore limit
    let max = max_observed.load(Ordering::Relaxed);
    assert!(
        max <= max_concurrent as u64,
        "Max concurrent should not exceed limit: {max} > {max_concurrent}"
    );
}

/// Test Arc/Mutex under contention (compare with `RwLock`)
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_mutex_contention_scalability() {
    let counter = Arc::new(Mutex::new(0u64));
    let iterations = 10_000;
    let tasks = 100;

    let mut handles = vec![];
    for _ in 0..tasks {
        let counter_clone = counter.clone();
        let handle = task::spawn(async move {
            for _ in 0..(iterations / tasks) {
                let mut val = counter_clone.lock().await;
                *val += 1;
                drop(val); // Explicit drop to release lock quickly
                task::yield_now().await;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.expect("Task should complete");
    }

    let final_value = *counter.lock().await;
    assert_eq!(final_value, iterations);
}

/// Test cancellation safety - tasks can be cancelled without corruption
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_cancellation_safety() {
    use tokio::sync::mpsc;
    use tokio::time::{timeout, Duration};

    let (tx, mut rx) = mpsc::channel(10);
    let completed = Arc::new(AtomicU64::new(0));

    // Start long-running task
    let completed_clone = completed.clone();
    let long_task = task::spawn(async move {
        for i in 0..1_000_000 {
            if tx.send(i).await.is_err() {
                break;
            }
            completed_clone.fetch_add(1, Ordering::Relaxed);
            if i % 100 == 0 {
                task::yield_now().await;
            }
        }
    });

    // Cancel after processing some messages
    let result = timeout(Duration::from_millis(10), long_task).await;
    assert!(result.is_err(), "Task should be cancelled");

    // Drain remaining messages
    let mut _received = 0;
    while rx.try_recv().is_ok() {
        _received += 1;
    }

    // Should have processed some messages before cancellation
    let processed = completed.load(Ordering::Relaxed);
    assert!(processed > 0, "Should have processed some messages");
    assert!(processed < 1_000_000, "Should not have completed all");
}

/// Test task spawn rate - system should handle rapid task creation
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn test_rapid_task_spawning() {
    let spawned = Arc::new(AtomicU64::new(0));
    let completed = Arc::new(AtomicU64::new(0));

    // Rapidly spawn 5000 tasks
    let handles: Vec<_> = (0..5000)
        .map(|i| {
            let spawned_clone = spawned.clone();
            let completed_clone = completed.clone();

            spawned_clone.fetch_add(1, Ordering::Relaxed);
            task::spawn(async move {
                // Minimal work per task
                let _result = i * i;
                task::yield_now().await;
                completed_clone.fetch_add(1, Ordering::Relaxed);
            })
        })
        .collect();

    // Wait for all to complete
    for handle in handles {
        handle.await.expect("Task should complete");
    }

    assert_eq!(spawned.load(Ordering::Relaxed), 5000);
    assert_eq!(completed.load(Ordering::Relaxed), 5000);
}

/// Test deadlock prevention - ensure lock ordering is correct
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_no_deadlocks_multiple_locks() {
    let lock_a = Arc::new(Mutex::new(0u64));
    let lock_b = Arc::new(Mutex::new(0u64));

    let mut handles = vec![];

    // Tasks that acquire locks in consistent order (A then B)
    for _ in 0..100 {
        let a = lock_a.clone();
        let b = lock_b.clone();

        let handle = task::spawn(async move {
            let mut val_a = a.lock().await;
            *val_a += 1;

            let mut val_b = b.lock().await;
            *val_b += 1;

            // Both locks held simultaneously (but consistent ordering prevents deadlock)
            drop(val_b);
            drop(val_a);

            task::yield_now().await;
        });
        handles.push(handle);
    }

    // All tasks should complete without deadlock
    for handle in handles {
        handle.await.expect("Task should complete without deadlock");
    }

    assert_eq!(*lock_a.lock().await, 100);
    assert_eq!(*lock_b.lock().await, 100);
}

/// Test panic isolation - one task panicking shouldn't affect others
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn test_panic_isolation() {
    let completed = Arc::new(AtomicU64::new(0));
    let panicked = Arc::new(AtomicU64::new(0));

    let mut handles = vec![];

    for i in 0..100 {
        let comp = completed.clone();
        let panic = panicked.clone();

        let handle = task::spawn(async move {
            if i == 50 {
                // One task panics
                panic.fetch_add(1, Ordering::Relaxed);
                panic!("Intentional panic for isolation test");
            }

            // Regular tasks complete normally
            let _work = i * i;
            comp.fetch_add(1, Ordering::Relaxed);
        });
        handles.push(handle);
    }

    let mut panic_count = 0;
    for handle in handles {
        if handle.await.is_err() {
            panic_count += 1;
        }
    }

    // Exactly one task should have panicked
    assert_eq!(panic_count, 1);
    // Other 99 tasks should have completed
    assert_eq!(completed.load(Ordering::Relaxed), 99);
}
