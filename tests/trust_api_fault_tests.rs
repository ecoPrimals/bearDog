//! Fault Injection Tests for Universal Trust API
//!
//! These tests verify system behavior under various fault conditions:
//! - Network failures
//! - Malformed requests
//! - Resource exhaustion
//! - Concurrent stress

use beardog_genetics::EcosystemGeneticEngine;
use beardog_tunnel::api::server::{BearDogApiServer, BearDogApiServerConfig};
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::timeout;

/// Helper to start a test server
async fn start_test_server(
) -> Result<(tokio::task::JoinHandle<()>, String), Box<dyn std::error::Error>> {
    static PORT_COUNTER: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(26000);
    let port = PORT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    env::remove_var("BEARDOG_FAMILY_SEED");
    env::set_var("BEARDOG_FAMILY_SEED", "FaultTestFamily");
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
// Malformed Request Tests
// ====================================================================================

#[tokio::test]
async fn test_fault_malformed_json() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Send invalid JSON
    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .header("Content-Type", "application/json")
        .body("{invalid json")
        .send()
        .await?;

    assert_eq!(resp.status(), 400);
    Ok(())
}

#[tokio::test]
async fn test_fault_missing_required_fields() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Universal format without evaluator
    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&json!({
            "request_format": "universal_trust_v1",
            "context": {}
        }))
        .send()
        .await?;

    assert_eq!(resp.status(), 400);

    // Legacy format without peer_id
    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&json!({
            "peer_tags": ["test"]
        }))
        .send()
        .await?;

    assert_eq!(resp.status(), 400);

    Ok(())
}

#[tokio::test]
async fn test_fault_wrong_content_type() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Send JSON with wrong content type
    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .header("Content-Type", "text/plain")
        .body(r#"{"request_format": "universal_trust_v1"}"#)
        .send()
        .await?;

    // Should be rejected (415 Unsupported Media Type or 400)
    assert!(resp.status().is_client_error());
    Ok(())
}

#[tokio::test]
async fn test_fault_empty_request_body() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .header("Content-Type", "application/json")
        .body("")
        .send()
        .await?;

    assert!(resp.status().is_client_error());
    Ok(())
}

#[tokio::test]
async fn test_fault_unsupported_request_format() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&json!({
            "request_format": "future_format_v999",
            "evaluator": {
                "peer_id": "test",
                "attestations": []
            }
        }))
        .send()
        .await?;

    assert_eq!(resp.status(), 400);
    let body: Value = resp.json().await?;
    assert!(body["error"].as_str().unwrap().contains("Unsupported"));

    Ok(())
}

// ====================================================================================
// Resource Exhaustion Tests
// ====================================================================================

#[tokio::test]
async fn test_fault_extremely_large_request() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Create a very large request
    let large_tags: Vec<String> = (0..10000).map(|i| format!("tag_{}", i)).collect();

    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&json!({
            "request_format": "universal_trust_v1",
            "evaluator": {
                "peer_id": "test",
                "attestations": [{
                    "provider_capability": "security/identity",
                    "format": "tag_list",
                    "data": {
                        "tags": large_tags
                    }
                }]
            }
        }))
        .send()
        .await?;

    // Should handle gracefully (either accept or reject based on limits)
    assert!(resp.status().is_success() || resp.status().is_client_error());

    Ok(())
}

#[tokio::test]
async fn test_fault_deeply_nested_json() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Create deeply nested JSON
    let mut nested = json!({"value": "leaf"});
    for _ in 0..100 {
        nested = json!({"nested": nested});
    }

    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&json!({
            "request_format": "universal_trust_v1",
            "evaluator": {
                "peer_id": "test",
                "attestations": [{
                    "provider_capability": "security/identity",
                    "format": "nested",
                    "data": nested
                }]
            }
        }))
        .send()
        .await?;

    // Should handle gracefully
    assert!(resp.status().is_success() || resp.status().is_client_error());

    Ok(())
}

#[tokio::test]
async fn test_fault_rapid_fire_requests() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Send 100 requests rapidly
    let mut handles = vec![];
    for i in 0..100 {
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

    // All should complete within reasonable time
    let results = timeout(Duration::from_secs(10), async {
        let mut successes = 0;
        for handle in handles {
            if let Ok(Ok(resp)) = handle.await {
                if resp.status().is_success() {
                    successes += 1;
                }
            }
        }
        successes
    })
    .await?;

    // Most should succeed
    assert!(results > 90, "Only {} requests succeeded", results);

    Ok(())
}

// ====================================================================================
// Network Fault Tests
// ====================================================================================

#[tokio::test]
async fn test_fault_connection_timeout() -> Result<(), Box<dyn std::error::Error>> {
    // Try to connect to non-existent server
    let client = Client::builder()
        .timeout(Duration::from_millis(100))
        .build()?;

    let result = client
        .get("http://127.0.0.1:9999/api/v1/trust/identity")
        .send()
        .await;

    // Should fail with connection error
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_fault_request_timeout() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;

    // Use extremely short timeout
    let client = Client::builder().timeout(Duration::from_nanos(1)).build()?;

    let result = client
        .get(&format!("{}/api/v1/trust/identity", base_url))
        .send()
        .await;

    // Might timeout or succeed if fast enough
    // Either outcome is acceptable, we're testing robustness
    let _ = result;

    Ok(())
}

// ====================================================================================
// Concurrent Stress Tests
// ====================================================================================

#[tokio::test]
async fn test_fault_concurrent_different_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Concurrent requests to different endpoints
    let mut handles = vec![];

    for i in 0..50 {
        let client = client.clone();
        let url = base_url.clone();

        // Alternate between identity and evaluate
        let handle = if i % 2 == 0 {
            tokio::spawn(async move {
                client
                    .get(&format!("{}/api/v1/trust/identity", url))
                    .send()
                    .await
            })
        } else {
            tokio::spawn(async move {
                client
                    .post(&format!("{}/api/v1/trust/evaluate", url))
                    .json(&json!({
                        "request_format": "universal_trust_v1",
                        "evaluator": {
                            "peer_id": "test",
                            "attestations": []
                        }
                    }))
                    .send()
                    .await
            })
        };
        handles.push(handle);
    }

    let mut successes = 0;
    for handle in handles {
        if let Ok(Ok(resp)) = handle.await {
            if resp.status().is_success() {
                successes += 1;
            }
        }
    }

    // Most should succeed
    assert!(successes > 45, "Only {} requests succeeded", successes);

    Ok(())
}

#[tokio::test]
async fn test_fault_mixed_valid_invalid_requests() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    let mut handles = vec![];

    for i in 0..50 {
        let client = client.clone();
        let url = base_url.clone();

        let handle = tokio::spawn(async move {
            // Half valid, half invalid
            if i % 2 == 0 {
                // Valid request
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
            } else {
                // Invalid request (missing evaluator)
                client
                    .post(&format!("{}/api/v1/trust/evaluate", url))
                    .json(&json!({
                        "request_format": "universal_trust_v1"
                    }))
                    .send()
                    .await
            }
        });
        handles.push(handle);
    }

    let mut valid_successes = 0;
    let mut invalid_errors = 0;

    for (i, handle) in handles.into_iter().enumerate() {
        if let Ok(Ok(resp)) = handle.await {
            if i % 2 == 0 {
                // Valid request
                if resp.status().is_success() {
                    valid_successes += 1;
                }
            } else {
                // Invalid request
                if resp.status().is_client_error() {
                    invalid_errors += 1;
                }
            }
        }
    }

    // Valid requests should succeed
    assert!(
        valid_successes > 20,
        "Only {} valid requests succeeded",
        valid_successes
    );
    // Invalid requests should error
    assert!(
        invalid_errors > 20,
        "Only {} invalid requests errored",
        invalid_errors
    );

    Ok(())
}

// ====================================================================================
// Edge Case Tests
// ====================================================================================

#[tokio::test]
async fn test_fault_special_characters_in_ids() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    let special_ids = vec![
        "peer@example.com",
        "peer#123",
        "peer$special",
        "peer%encoded",
        "peer&and",
        "peer'quote",
        "peer<script>",
        "../../../etc/passwd",
        "DROP TABLE peers;",
    ];

    for peer_id in special_ids {
        let resp = client
            .post(&format!("{}/api/v1/trust/evaluate", base_url))
            .json(&json!({
                "request_format": "universal_trust_v1",
                "evaluator": {
                    "peer_id": peer_id,
                    "attestations": []
                }
            }))
            .send()
            .await?;

        // Should handle gracefully (accept as-is or sanitize)
        assert!(
            resp.status().is_success() || resp.status().is_client_error(),
            "Failed for peer_id: {}",
            peer_id
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_fault_null_values_in_json() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&json!({
            "request_format": "universal_trust_v1",
            "evaluator": {
                "peer_id": null,
                "attestations": null
            }
        }))
        .send()
        .await?;

    // Should reject with validation error
    assert!(resp.status().is_client_error());

    Ok(())
}

#[tokio::test]
async fn test_fault_unicode_in_data() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&json!({
            "request_format": "universal_trust_v1",
            "evaluator": {
                "peer_id": "测试节点-🐻",
                "attestations": [{
                    "provider_capability": "security/identity",
                    "format": "tag_list",
                    "data": {
                        "tags": ["beardog:family:家族:节点"],
                        "family_id": "家族"
                    }
                }]
            }
        }))
        .send()
        .await?;

    // Should handle Unicode gracefully
    assert!(resp.status().is_success());

    Ok(())
}

#[tokio::test]
async fn test_fault_extremely_long_strings() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    let long_id = "a".repeat(10000);

    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&json!({
            "request_format": "universal_trust_v1",
            "evaluator": {
                "peer_id": long_id,
                "attestations": []
            }
        }))
        .send()
        .await?;

    // Should handle gracefully (either accept or reject based on limits)
    assert!(resp.status().is_success() || resp.status().is_client_error());

    Ok(())
}

// ====================================================================================
// Recovery Tests
// ====================================================================================

#[tokio::test]
async fn test_fault_recovery_after_bad_request() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Send bad request
    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .header("Content-Type", "application/json")
        .body("{invalid")
        .send()
        .await?;
    assert!(resp.status().is_client_error());

    // Follow with valid request
    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&json!({
            "request_format": "universal_trust_v1",
            "evaluator": {
                "peer_id": "test",
                "attestations": []
            }
        }))
        .send()
        .await?;

    // Should succeed (server recovered)
    assert!(resp.status().is_success());

    Ok(())
}

#[tokio::test]
async fn test_fault_multiple_rapid_recoveries() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    for _ in 0..10 {
        // Bad request
        let _ = client
            .post(&format!("{}/api/v1/trust/evaluate", base_url))
            .body("{invalid")
            .send()
            .await;

        // Good request
        let resp = client
            .get(&format!("{}/api/v1/trust/identity", base_url))
            .send()
            .await?;

        // Should keep working
        assert!(resp.status().is_success());
    }

    Ok(())
}
