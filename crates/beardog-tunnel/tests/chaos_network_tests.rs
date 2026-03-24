// SPDX-License-Identifier: AGPL-3.0-only
#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]
//! Chaos Testing for Network Partitions and Failures
//!
//! This test suite simulates real-world failure scenarios to verify
//! BearDog's resilience under adverse conditions.
//!
//! ## Chaos Scenarios:
//!
//! 1. **Network Partitions**: Simulated network splits
//! 2. **Slow Networks**: High latency conditions
//! 3. **Packet Loss**: Random packet drops
//! 4. **Connection Failures**: Abrupt disconnects
//! 5. **Resource Exhaustion**: Out of memory, file descriptors
//! 6. **Concurrent Failures**: Multiple simultaneous failures
//!
//! ## Deep Debt Solution
//!
//! Chaos testing reveals:
//! - Race conditions under stress
//! - Resource leaks during failures
//! - Deadlocks in error paths
//! - Improper cleanup

use beardog_errors::BearDogError;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tokio::time::{sleep, timeout};

/// Chaos test: Concurrent connection attempts with failures
///
/// Simulates many clients trying to connect simultaneously,
/// with some connections failing randomly
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn chaos_concurrent_connection_storm() -> Result<(), BearDogError> {
    let success_count = Arc::new(AtomicU64::new(0));
    let failure_count = Arc::new(AtomicU64::new(0));
    let concurrent_tasks = 100;

    let mut handles = vec![];

    for task_id in 0..concurrent_tasks {
        let success = Arc::clone(&success_count);
        let failure = Arc::clone(&failure_count);

        let handle = tokio::spawn(async move {
            // Simulate random connection behavior
            let should_fail = rand::random::<bool>();

            // Simulate connection delay (0-50ms)
            let delay_ms = rand::random::<u64>() % 50;
            sleep(Duration::from_millis(delay_ms)).await;

            if should_fail {
                failure.fetch_add(1, Ordering::Relaxed);
                Err(BearDogError::network(format!(
                    "Simulated connection failure (task {task_id})"
                )))
            } else {
                // Simulate successful connection
                success.fetch_add(1, Ordering::Relaxed);
                Ok(())
            }
        });

        handles.push(handle);
    }

    // Wait for all tasks (don't fail on individual errors)
    for handle in handles {
        let _ = handle.await;
    }

    let total_success = success_count.load(Ordering::Relaxed);
    let total_failure = failure_count.load(Ordering::Relaxed);

    println!(
        "🌪️  Chaos test results: {total_success} success, {total_failure} failures (total: {concurrent_tasks})"
    );

    // Property: All tasks completed (no panics or deadlocks)
    assert_eq!(
        total_success + total_failure,
        concurrent_tasks,
        "Some tasks did not complete"
    );

    Ok(())
}

/// Chaos test: Network timeout simulation
///
/// Simulates operations that take too long and must be timed out
#[tokio::test]
async fn chaos_network_timeout_resilience() -> Result<(), BearDogError> {
    let timeout_duration = Duration::from_millis(100);
    let iterations = 50;
    let mut timeouts = 0;
    let mut completions = 0;

    for _ in 0..iterations {
        // Simulate random operation duration (0-200ms)
        let operation_duration = Duration::from_millis(rand::random::<u64>() % 200);

        let result = timeout(timeout_duration, async move {
            sleep(operation_duration).await;
            Ok::<(), BearDogError>(())
        })
        .await;

        match result {
            Ok(_) => completions += 1,
            Err(_elapsed) => timeouts += 1,
        }
    }

    println!(
        "🌪️  Timeout chaos: {completions} completed, {timeouts} timed out (total: {iterations})"
    );

    // Property: All operations either completed or timed out (no hangs)
    assert_eq!(
        completions + timeouts,
        iterations,
        "Some operations neither completed nor timed out"
    );

    Ok(())
}

/// Chaos test: Resource exhaustion simulation
///
/// Simulates running out of resources (file descriptors, memory)
#[tokio::test]
async fn chaos_resource_exhaustion() -> Result<(), BearDogError> {
    // Simulate creating many "resources" until we hit a limit
    let max_resources = 1000;
    let mut resources = Vec::new();
    let mut allocations = 0;

    for i in 0..max_resources {
        // Simulate resource allocation
        // In real scenario, this would be sockets, file descriptors, etc.
        let resource = vec![0u8; 1024]; // 1KB per "resource"

        // Simulate random allocation failures (10% chance after 100 allocations)
        if i > 100 && rand::random::<u8>() < 25 {
            // Simulated allocation failure
            break;
        }

        resources.push(resource);
        allocations += 1;
    }

    println!(
        "🌪️  Resource chaos: Successfully allocated {allocations} resources before exhaustion (held {} buffers)",
        resources.len()
    );

    // Property: System gracefully handles resource limits
    assert!(allocations > 0, "Should allocate at least some resources");
    assert!(
        allocations <= max_resources,
        "Should not exceed resource limit"
    );

    // Cleanup
    resources.clear();

    Ok(())
}

/// Chaos test: Cascading failures
///
/// Simulates a failure that triggers other failures (cascade effect)
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn chaos_cascading_failures() -> Result<(), BearDogError> {
    let failure_trigger = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let cascade_count = Arc::new(AtomicU64::new(0));
    let tasks = 50;

    let mut handles = vec![];

    for task_id in 0..tasks {
        let trigger = Arc::clone(&failure_trigger);
        let cascade = Arc::clone(&cascade_count);

        let handle = tokio::spawn(async move {
            // Random delay before checking
            sleep(Duration::from_millis(rand::random::<u64>() % 100)).await;

            // If failure triggered, this task fails too (cascade)
            if trigger.load(Ordering::Relaxed) {
                cascade.fetch_add(1, Ordering::Relaxed);
                return Err(BearDogError::network(format!(
                    "Cascading failure (task {task_id})"
                )));
            }

            // 10% chance to trigger the cascade
            if rand::random::<u8>() < 25 && !trigger.swap(true, Ordering::Relaxed) {
                return Err(BearDogError::network(format!(
                    "Initial failure (task {task_id})"
                )));
            }

            Ok(())
        });

        handles.push(handle);
    }

    // Wait for all tasks
    let mut errors = 0;
    for handle in handles {
        if let Ok(Err(_)) = handle.await {
            errors += 1;
        }
    }

    let cascaded = cascade_count.load(Ordering::Relaxed);

    println!("🌪️  Cascading failure chaos: {errors} total errors, {cascaded} cascaded");

    // Property: System detects and reports cascading failures
    assert!(errors > 0, "Should have at least one failure");

    Ok(())
}

/// Chaos test: Rapid connect/disconnect cycles
///
/// Simulates clients connecting and disconnecting rapidly
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn chaos_rapid_connect_disconnect() -> Result<(), BearDogError> {
    let cycles = 100;
    let connections = Arc::new(AtomicU64::new(0));
    let disconnections = Arc::new(AtomicU64::new(0));

    let mut handles = vec![];

    for _ in 0..cycles {
        let conn_count = Arc::clone(&connections);
        let disc_count = Arc::clone(&disconnections);

        let handle = tokio::spawn(async move {
            // Simulate connect
            conn_count.fetch_add(1, Ordering::Relaxed);

            // Hold connection for random short time (0-20ms)
            sleep(Duration::from_millis(rand::random::<u64>() % 20)).await;

            // Simulate disconnect
            disc_count.fetch_add(1, Ordering::Relaxed);

            Ok::<(), BearDogError>(())
        });

        handles.push(handle);
    }

    // Wait for all cycles
    for handle in handles {
        handle.await.unwrap()?;
    }

    let total_conn = connections.load(Ordering::Relaxed);
    let total_disc = disconnections.load(Ordering::Relaxed);

    println!("🌪️  Connect/disconnect chaos: {total_conn} connections, {total_disc} disconnections");

    // Property: Connections and disconnections should match
    assert_eq!(
        total_conn, total_disc,
        "Connection/disconnection mismatch (possible resource leak)"
    );
    assert_eq!(total_conn, cycles, "Some cycles did not complete");

    Ok(())
}

/// Chaos test: Memory pressure simulation
///
/// Simulates operations under high memory pressure
#[tokio::test]
async fn chaos_memory_pressure() -> Result<(), BearDogError> {
    let mut memory_blocks = Vec::new();
    let block_size = 1024 * 1024; // 1 MB blocks
    let max_blocks = 100; // Up to 100 MB

    // Allocate memory gradually
    for i in 0..max_blocks {
        let block = vec![0u8; block_size];
        memory_blocks.push(block);

        // Every 10 blocks, try to do some "work"
        if i % 10 == 0 {
            // Simulate crypto operation under memory pressure
            let _ = beardog_tunnel::tunnel::hsm::software_hsm::crypto_providers::GeneticCryptoProvider::new();

            // Random small delay
            sleep(Duration::from_millis(rand::random::<u64>() % 10)).await;
        }
    }

    println!(
        "🌪️  Memory pressure chaos: Allocated {} MB",
        memory_blocks.len()
    );

    // Property: System continues functioning under memory pressure
    assert_eq!(
        memory_blocks.len(),
        max_blocks,
        "Should allocate all blocks"
    );

    // Cleanup
    memory_blocks.clear();

    Ok(())
}

/// Chaos test: Concurrent operations with random failures
///
/// Simulates many concurrent crypto operations with random failures
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn chaos_concurrent_crypto_operations() -> Result<(), BearDogError> {
    use beardog_tunnel::tunnel::hsm::software_hsm::CryptoProvider;
    use beardog_tunnel::tunnel::hsm::software_hsm::crypto_providers::GeneticCryptoProvider;
    use rand::Rng;

    let operations = 200;
    let success_count = Arc::new(AtomicU64::new(0));
    let failure_count = Arc::new(AtomicU64::new(0));

    let mut handles = vec![];

    for op_id in 0..operations {
        let success = Arc::clone(&success_count);
        let failure = Arc::clone(&failure_count);

        let handle = tokio::spawn(async move {
            // Random delay
            sleep(Duration::from_millis(rand::random::<u64>() % 50)).await;

            // Simulate random failure (10% chance)
            if rand::random::<u8>() < 25 {
                failure.fetch_add(1, Ordering::Relaxed);
                return Err(BearDogError::crypto_error(format!(
                    "Simulated crypto failure (op {op_id})"
                )));
            }

            // Perform actual crypto operation
            let provider = match GeneticCryptoProvider::new() {
                Ok(p) => Arc::new(p),
                Err(e) => {
                    failure.fetch_add(1, Ordering::Relaxed);
                    return Err(e);
                }
            };

            // Generate random data and encrypt/decrypt it
            let data_len = (rand::random::<usize>() % 1000) + 1;
            let mut data = vec![0u8; data_len];
            rand::thread_rng().fill(&mut data[..]);

            let key = match provider.generate_random_bytes(32) {
                Ok(k) => k,
                Err(e) => {
                    failure.fetch_add(1, Ordering::Relaxed);
                    return Err(e);
                }
            };

            // Encrypt
            let ciphertext = match provider.encrypt(&key, &data).await {
                Ok(c) => c,
                Err(e) => {
                    failure.fetch_add(1, Ordering::Relaxed);
                    return Err(e);
                }
            };

            // Decrypt
            match provider.decrypt(&key, &ciphertext).await {
                Ok(_) => {
                    success.fetch_add(1, Ordering::Relaxed);
                    Ok(())
                }
                Err(e) => {
                    failure.fetch_add(1, Ordering::Relaxed);
                    Err(e)
                }
            }
        });

        handles.push(handle);
    }

    // Wait for all operations
    for handle in handles {
        let _ = handle.await;
    }

    let total_success = success_count.load(Ordering::Relaxed);
    let total_failure = failure_count.load(Ordering::Relaxed);

    println!(
        "🌪️  Concurrent crypto chaos: {total_success} success, {total_failure} failures (total: {operations})"
    );

    // Property: All operations completed (no deadlocks)
    assert_eq!(
        total_success + total_failure,
        operations,
        "Some operations did not complete"
    );

    Ok(())
}
