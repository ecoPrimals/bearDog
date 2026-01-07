//! Chaos Engineering Tests for Universal Trust API
//!
//! These tests verify system resilience under extreme conditions:
//! - Simultaneous multi-client load
//! - Random failure injection
//! - State corruption scenarios
//! - Resource contention

use beardog_genetics::EcosystemGeneticEngine;
use beardog_tunnel::api::server::{BearDogApiServer, BearDogApiServerConfig};
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

/// Helper to start a test server
async fn start_test_server(
) -> Result<(tokio::task::JoinHandle<()>, String), Box<dyn std::error::Error>> {
    static PORT_COUNTER: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(27000);
    let port = PORT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    env::remove_var("BEARDOG_FAMILY_SEED");
    env::set_var("BEARDOG_FAMILY_SEED", "ChaosTestFamily");
    env::set_var("BEARDOG_HSM_MODE", "software");

    let addr = format!("127.0.0.1:{}", port).parse()?;
    let base_url = format!("http://{}", addr);

    let config = BearDogApiServerConfig {
        bind_addr: addr,
        enable_cors: true,
        version: "test".to_string(),
    };

    let hsm = Arc::new(HsmManager::auto_initialize().await?);
    let genetics = Arc::new(EcosystemGeneticEngine::new()?);
    let provider = Arc::new(BeardogBtspProvider::new(hsm, genetics).await?);
    let server = BearDogApiServer::new(config, provider).await?;

    let handle = tokio::spawn(async move {
        if let Err(e) = server.serve().await {
            eprintln!("Server error: {}", e);
        }
    });

    tokio::time::sleep(Duration::from_millis(300)).await;

    Ok((handle, base_url))
}

// ====================================================================================
// Chaos Test: Massive Concurrent Load
// ====================================================================================

#[tokio::test]
#[ignore] // Run with: cargo test --test trust_api_chaos_tests -- --ignored
async fn test_chaos_massive_concurrent_load() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Arc::new(Client::new());

    let num_clients = 1000;
    let requests_per_client = 10;

    let success_count = Arc::new(AtomicU64::new(0));
    let error_count = Arc::new(AtomicU64::new(0));

    let mut handles = vec![];

    for client_id in 0..num_clients {
        let client = client.clone();
        let url = base_url.clone();
        let success_count = success_count.clone();
        let error_count = error_count.clone();

        let handle = tokio::spawn(async move {
            for req_id in 0..requests_per_client {
                let result = client
                    .post(&format!("{}/api/v1/trust/evaluate", url))
                    .json(&json!({
                        "request_format": "universal_trust_v1",
                        "evaluator": {
                            "peer_id": format!("client_{}_{}", client_id, req_id),
                            "attestations": []
                        }
                    }))
                    .send()
                    .await;

                match result {
                    Ok(resp) if resp.status().is_success() => {
                        success_count.fetch_add(1, Ordering::Relaxed);
                    }
                    _ => {
                        error_count.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        });
        handles.push(handle);
    }

    // Wait for all to complete (with timeout)
    let total_requests = (num_clients * requests_per_client) as u64;
    let completed = timeout(Duration::from_secs(30), async {
        for handle in handles {
            let _ = handle.await;
        }
    })
    .await
    .is_ok();

    assert!(completed, "Test did not complete in time");

    let successes = success_count.load(Ordering::Relaxed);
    let errors = error_count.load(Ordering::Relaxed);

    println!(
        "Massive load test: {} successes, {} errors out of {} requests",
        successes, errors, total_requests
    );

    // At least 90% should succeed
    let success_rate = (successes as f64) / (total_requests as f64);
    assert!(
        success_rate > 0.9,
        "Success rate {} is below 90%",
        success_rate
    );

    Ok(())
}

// ====================================================================================
// Chaos Test: Random Request Types
// ====================================================================================

#[tokio::test]
#[ignore]
async fn test_chaos_random_request_types() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Arc::new(Client::new());

    let num_requests = 500;
    let success_count = Arc::new(AtomicU64::new(0));

    let mut handles = vec![];

    for i in 0..num_requests {
        let client = client.clone();
        let url = base_url.clone();
        let success_count = success_count.clone();

        let handle = tokio::spawn(async move {
            // Randomly choose request type
            let request_type = i % 5;

            let result = match request_type {
                0 => {
                    // Universal format, valid
                    client
                        .post(&format!("{}/api/v1/trust/evaluate", url))
                        .json(&json!({
                            "request_format": "universal_trust_v1",
                            "evaluator": {
                                "peer_id": format!("peer_{}", i),
                                "attestations": []
                            }
                        }))
                        .send()
                        .await
                }
                1 => {
                    // Universal format, with attestations
                    client
                        .post(&format!("{}/api/v1/trust/evaluate", url))
                        .json(&json!({
                            "request_format": "universal_trust_v1",
                            "evaluator": {
                                "peer_id": format!("peer_{}", i),
                                "attestations": [{
                                    "provider_capability": "security/identity",
                                    "format": "tag_list",
                                    "data": {"tags": [format!("tag_{}", i)]}
                                }]
                            }
                        }))
                        .send()
                        .await
                }
                2 => {
                    // Legacy format
                    client
                        .post(&format!("{}/api/v1/trust/evaluate", url))
                        .json(&json!({
                            "peer_id": format!("peer_{}", i),
                            "peer_tags": [format!("tag_{}", i)]
                        }))
                        .send()
                        .await
                }
                3 => {
                    // Identity endpoint
                    client
                        .get(&format!("{}/api/v1/trust/identity", url))
                        .send()
                        .await
                }
                _ => {
                    // Invalid request (should error gracefully)
                    client
                        .post(&format!("{}/api/v1/trust/evaluate", url))
                        .json(&json!({
                            "invalid_field": "test"
                        }))
                        .send()
                        .await
                }
            };

            if let Ok(resp) = result {
                // Count both success and expected errors
                if resp.status().is_success() || resp.status().is_client_error() {
                    success_count.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
        handles.push(handle);
    }

    let completed = timeout(Duration::from_secs(30), async {
        for handle in handles {
            let _ = handle.await;
        }
    })
    .await
    .is_ok();

    assert!(completed, "Test did not complete in time");

    let handled = success_count.load(Ordering::Relaxed);
    let handle_rate = (handled as f64) / (num_requests as f64);

    println!(
        "Random request test: {} handled out of {} requests",
        handled, num_requests
    );

    // Server should handle all requests gracefully (success or proper error)
    assert!(
        handle_rate > 0.95,
        "Only {} of requests handled",
        handle_rate
    );

    Ok(())
}

// ====================================================================================
// Chaos Test: Burst Traffic
// ====================================================================================

#[tokio::test]
#[ignore]
async fn test_chaos_burst_traffic() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Arc::new(Client::new());

    let num_bursts = 10;
    let requests_per_burst = 100;

    for burst in 0..num_bursts {
        println!("Starting burst {}/{}", burst + 1, num_bursts);

        let mut handles = vec![];

        // Send all requests simultaneously
        for i in 0..requests_per_burst {
            let client = client.clone();
            let url = base_url.clone();

            let handle = tokio::spawn(async move {
                client
                    .post(&format!("{}/api/v1/trust/evaluate", url))
                    .json(&json!({
                        "request_format": "universal_trust_v1",
                        "evaluator": {
                            "peer_id": format!("peer_{}", i),
                            "attestations": []
                        }
                    }))
                    .send()
                    .await
            });
            handles.push(handle);
        }

        // Wait for burst to complete
        let mut successes = 0;
        for handle in handles {
            if let Ok(Ok(resp)) = handle.await {
                if resp.status().is_success() {
                    successes += 1;
                }
            }
        }

        println!("Burst {} completed: {} successes", burst + 1, successes);

        // Most should succeed
        assert!(
            successes > requests_per_burst * 9 / 10,
            "Burst {} had only {} successes",
            burst + 1,
            successes
        );

        // Small delay between bursts
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}

// ====================================================================================
// Chaos Test: Mixed Read/Write Operations
// ====================================================================================

#[tokio::test]
#[ignore]
async fn test_chaos_mixed_operations() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Arc::new(Client::new());

    let num_operations = 1000;
    let read_count = Arc::new(AtomicU64::new(0));
    let write_count = Arc::new(AtomicU64::new(0));

    let mut handles = vec![];

    for i in 0..num_operations {
        let client = client.clone();
        let url = base_url.clone();
        let read_count = read_count.clone();
        let write_count = write_count.clone();

        let handle = tokio::spawn(async move {
            // 70% reads (identity), 30% writes (evaluate)
            if i % 10 < 7 {
                // Read
                if let Ok(resp) = client
                    .get(&format!("{}/api/v1/trust/identity", url))
                    .send()
                    .await
                {
                    if resp.status().is_success() {
                        read_count.fetch_add(1, Ordering::Relaxed);
                    }
                }
            } else {
                // Write
                if let Ok(resp) = client
                    .post(&format!("{}/api/v1/trust/evaluate", url))
                    .json(&json!({
                        "request_format": "universal_trust_v1",
                        "evaluator": {
                            "peer_id": format!("peer_{}", i),
                            "attestations": []
                        }
                    }))
                    .send()
                    .await
                {
                    if resp.status().is_success() {
                        write_count.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        });
        handles.push(handle);
    }

    let completed = timeout(Duration::from_secs(30), async {
        for handle in handles {
            let _ = handle.await;
        }
    })
    .await
    .is_ok();

    assert!(completed, "Test did not complete in time");

    let reads = read_count.load(Ordering::Relaxed);
    let writes = write_count.load(Ordering::Relaxed);

    println!(
        "Mixed operations: {} reads, {} writes out of {} operations",
        reads, writes, num_operations
    );

    // Both should have high success rates
    assert!(reads > 600, "Only {} reads succeeded", reads);
    assert!(writes > 250, "Only {} writes succeeded", writes);

    Ok(())
}

// ====================================================================================
// Chaos Test: Varying Payload Sizes
// ====================================================================================

#[tokio::test]
#[ignore]
async fn test_chaos_varying_payload_sizes() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    let sizes = vec![1, 10, 100, 1000, 5000];

    for size in sizes {
        println!("Testing payload size: {} tags", size);

        let tags: Vec<String> = (0..size).map(|i| format!("tag_{}", i)).collect();

        let resp = client
            .post(&format!("{}/api/v1/trust/evaluate", base_url))
            .json(&json!({
                "request_format": "universal_trust_v1",
                "evaluator": {
                    "peer_id": "test",
                    "attestations": [{
                        "provider_capability": "security/identity",
                        "format": "tag_list",
                        "data": {"tags": tags}
                    }]
                }
            }))
            .send()
            .await?;

        // Should handle gracefully
        assert!(
            resp.status().is_success() || resp.status().is_client_error(),
            "Failed for size {}",
            size
        );
    }

    Ok(())
}

// ====================================================================================
// Chaos Test: Long Running Stability
// ====================================================================================

#[tokio::test]
#[ignore]
async fn test_chaos_long_running_stability() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Arc::new(Client::new());

    let duration = Duration::from_secs(60); // 1 minute
    let request_interval = Duration::from_millis(10);

    let start = std::time::Instant::now();
    let mut request_count = 0;
    let mut success_count = 0;

    while start.elapsed() < duration {
        let resp = client
            .post(&format!("{}/api/v1/trust/evaluate", base_url))
            .json(&json!({
                "request_format": "universal_trust_v1",
                "evaluator": {
                    "peer_id": format!("peer_{}", request_count),
                    "attestations": []
                }
            }))
            .send()
            .await?;

        request_count += 1;
        if resp.status().is_success() {
            success_count += 1;
        }

        tokio::time::sleep(request_interval).await;
    }

    let success_rate = (success_count as f64) / (request_count as f64);

    println!(
        "Long running test: {} successes out of {} requests over {:?}",
        success_count, request_count, duration
    );

    // Should maintain high success rate over time
    assert!(
        success_rate > 0.95,
        "Success rate {} degraded over time",
        success_rate
    );

    Ok(())
}

// ====================================================================================
// Chaos Test: Random Delays
// ====================================================================================

#[tokio::test]
#[ignore]
async fn test_chaos_random_delays() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Arc::new(Client::new());

    let num_requests = 200;
    let success_count = Arc::new(AtomicU64::new(0));

    let mut handles = vec![];

    for i in 0..num_requests {
        let client = client.clone();
        let url = base_url.clone();
        let success_count = success_count.clone();

        let handle = tokio::spawn(async move {
            // Random delay before request (0-100ms)
            let delay = Duration::from_millis((i * 37) % 100);
            tokio::time::sleep(delay).await;

            if let Ok(resp) = client
                .post(&format!("{}/api/v1/trust/evaluate", url))
                .json(&json!({
                    "request_format": "universal_trust_v1",
                    "evaluator": {
                        "peer_id": format!("peer_{}", i),
                        "attestations": []
                    }
                }))
                .send()
                .await
            {
                if resp.status().is_success() {
                    success_count.fetch_add(1, Ordering::Relaxed);
                }
            }
        });
        handles.push(handle);
    }

    let completed = timeout(Duration::from_secs(30), async {
        for handle in handles {
            let _ = handle.await;
        }
    })
    .await
    .is_ok();

    assert!(completed, "Test did not complete in time");

    let successes = success_count.load(Ordering::Relaxed);
    let success_rate = (successes as f64) / (num_requests as f64);

    println!(
        "Random delay test: {} successes out of {} requests",
        successes, num_requests
    );

    assert!(
        success_rate > 0.95,
        "Success rate {} is too low",
        success_rate
    );

    Ok(())
}

// ====================================================================================
// Chaos Test: Concurrent Format Mix
// ====================================================================================

#[tokio::test]
#[ignore]
async fn test_chaos_concurrent_format_mix() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Arc::new(Client::new());

    let num_requests = 500;
    let universal_count = Arc::new(AtomicU64::new(0));
    let legacy_count = Arc::new(AtomicU64::new(0));

    let mut handles = vec![];

    for i in 0..num_requests {
        let client = client.clone();
        let url = base_url.clone();
        let universal_count = universal_count.clone();
        let legacy_count = legacy_count.clone();

        let handle = tokio::spawn(async move {
            if i % 2 == 0 {
                // Universal format
                if let Ok(resp) = client
                    .post(&format!("{}/api/v1/trust/evaluate", url))
                    .json(&json!({
                        "request_format": "universal_trust_v1",
                        "evaluator": {
                            "peer_id": format!("peer_{}", i),
                            "attestations": []
                        }
                    }))
                    .send()
                    .await
                {
                    if resp.status().is_success() {
                        universal_count.fetch_add(1, Ordering::Relaxed);
                    }
                }
            } else {
                // Legacy format
                if let Ok(resp) = client
                    .post(&format!("{}/api/v1/trust/evaluate", url))
                    .json(&json!({
                        "peer_id": format!("peer_{}", i),
                        "peer_tags": [format!("tag_{}", i)]
                    }))
                    .send()
                    .await
                {
                    if resp.status().is_success() {
                        legacy_count.fetch_add(1, Ordering::Relaxed);
                    }
                }
            }
        });
        handles.push(handle);
    }

    let completed = timeout(Duration::from_secs(30), async {
        for handle in handles {
            let _ = handle.await;
        }
    })
    .await
    .is_ok();

    assert!(completed, "Test did not complete in time");

    let universal = universal_count.load(Ordering::Relaxed);
    let legacy = legacy_count.load(Ordering::Relaxed);

    println!(
        "Format mix test: {} universal, {} legacy out of {} requests",
        universal, legacy, num_requests
    );

    // Both formats should work concurrently
    assert!(universal > 230, "Only {} universal succeeded", universal);
    assert!(legacy > 230, "Only {} legacy succeeded", legacy);

    Ok(())
}
