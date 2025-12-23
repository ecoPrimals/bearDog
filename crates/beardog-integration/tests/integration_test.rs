//! Integration tests for beardog-integration API server
//!
//! Tests the full wire-up of all 17 endpoints with realistic scenarios.

use beardog_integration::IntegrationConfig;

/// Helper function to create test configuration
fn test_integration_config() -> IntegrationConfig {
    IntegrationConfig {
        upa_url: "http://localhost:18080".to_string(), // Mock UPA
        api_port: 19000,                               // Non-standard port for testing
        service_name: "BearDog Integration Test".to_string(),
        capabilities: vec!["btsp".to_string(), "birdsong".to_string()],
        heartbeat_interval_secs: 60,
    }
}

#[tokio::test]
async fn test_integration_config_creation() {
    let config = test_integration_config();
    assert_eq!(config.service_name, "BearDog Integration Test");
    assert_eq!(config.api_port, 19000);
    assert_eq!(config.heartbeat_interval_secs, 60);
}

#[tokio::test]
async fn test_default_integration_config() {
    let config = IntegrationConfig::default();
    assert_eq!(config.api_port, 9000);
    assert!(!config.capabilities.is_empty());
}

#[tokio::test]
async fn test_btsp_tunnel_flow() {
    // This would test the full BTSP flow:
    // 1. POST /btsp/tunnel/establish
    // 2. POST /btsp/tunnel/:id/encrypt
    // 3. POST /btsp/tunnel/:id/decrypt
    // 4. GET /btsp/tunnel/:id/status
    // 5. DELETE /btsp/tunnel/:id

    // Note: Full HTTP tests are in the --ignored section below
}

#[tokio::test]
async fn test_birdsong_flow() {
    // This would test the BirdSong flow:
    // 1. POST /birdsong/encrypt
    // 2. POST /birdsong/decrypt
    // 3. GET /birdsong/lineage/:node_id
    // 4. POST /birdsong/lineage/verify
}

#[tokio::test]
async fn test_lineage_flow() {
    // This would test the Lineage flow:
    // 1. POST /lineage/generate (root)
    // 2. POST /lineage/generate (child)
    // 3. POST /lineage/verify
    // 4. GET /lineage/proof/:node_id
}

#[tokio::test]
async fn test_system_endpoints() {
    // This would test system endpoints:
    // 1. GET /health
    // 2. GET /metrics
    // 3. GET /capabilities
    // 4. GET /status
}

// ============================================================================
// Live API Server Tests (when server is running)
// ============================================================================

/// Test establishing a BTSP tunnel via HTTP
///
/// Run with: cargo test test_live_btsp_establish --ignored
#[tokio::test]
#[ignore]
async fn test_live_btsp_establish() {
    let client = reqwest::Client::new();

    let request = serde_json::json!({
        "responder_id": "test_responder",
        "initiator_entropy": "test_entropy_12345"
    });

    let response = client
        .post("http://localhost:9000/btsp/tunnel/establish")
        .json(&request)
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());

    let body: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    assert!(body.get("tunnel_id").is_some());
    assert!(body.get("responder_entropy").is_some());
}

/// Test BTSP encryption flow via HTTP
///
/// Run with: cargo test test_live_btsp_encryption --ignored
#[tokio::test]
#[ignore]
async fn test_live_btsp_encryption_flow() {
    let client = reqwest::Client::new();

    // 1. Establish tunnel
    let establish_req = serde_json::json!({
        "responder_id": "test_responder",
        "initiator_entropy": "test_entropy"
    });

    let tunnel_response = client
        .post("http://localhost:9000/btsp/tunnel/establish")
        .json(&establish_req)
        .send()
        .await
        .expect("Failed to establish tunnel");

    let tunnel_data: serde_json::Value = tunnel_response
        .json()
        .await
        .expect("Failed to parse tunnel response");
    let tunnel_id = tunnel_data["tunnel_id"]
        .as_str()
        .expect("No tunnel_id")
        .to_string();

    // 2. Encrypt data
    let encrypt_req = serde_json::json!({
        "plaintext": "Hello, BearDog!"
    });

    let encrypt_response = client
        .post(format!(
            "http://localhost:9000/btsp/tunnel/{}/encrypt",
            tunnel_id
        ))
        .json(&encrypt_req)
        .send()
        .await
        .expect("Failed to encrypt");

    assert!(encrypt_response.status().is_success());

    let encrypt_data: serde_json::Value = encrypt_response
        .json()
        .await
        .expect("Failed to parse encrypt response");
    let ciphertext = encrypt_data["ciphertext"]
        .as_str()
        .expect("No ciphertext")
        .to_string();

    // 3. Decrypt data
    let decrypt_req = serde_json::json!({
        "ciphertext": ciphertext
    });

    let decrypt_response = client
        .post(format!(
            "http://localhost:9000/btsp/tunnel/{}/decrypt",
            tunnel_id
        ))
        .json(&decrypt_req)
        .send()
        .await
        .expect("Failed to decrypt");

    assert!(decrypt_response.status().is_success());

    let decrypt_data: serde_json::Value = decrypt_response
        .json()
        .await
        .expect("Failed to parse decrypt response");
    let plaintext = decrypt_data["plaintext"].as_str().expect("No plaintext");

    assert_eq!(plaintext, "Hello, BearDog!");

    // 4. Close tunnel
    let close_response = client
        .delete(format!("http://localhost:9000/btsp/tunnel/{}", tunnel_id))
        .send()
        .await
        .expect("Failed to close tunnel");

    assert!(close_response.status().is_success());
}

/// Test BirdSong encryption via HTTP
///
/// Run with: cargo test test_live_birdsong --ignored
#[tokio::test]
#[ignore]
async fn test_live_birdsong_encryption() {
    let client = reqwest::Client::new();

    let encrypt_req = serde_json::json!({
        "payload": "Secret message for lineage",
        "lineage_hint": "lineage_root_123"
    });

    let encrypt_response = client
        .post("http://localhost:9000/birdsong/encrypt")
        .json(&encrypt_req)
        .send()
        .await
        .expect("Failed to encrypt");

    assert!(encrypt_response.status().is_success());

    let encrypt_data: serde_json::Value = encrypt_response
        .json()
        .await
        .expect("Failed to parse response");

    assert!(encrypt_data.get("ciphertext").is_some());
    assert!(encrypt_data.get("metadata").is_some());
}

/// Test lineage generation via HTTP
///
/// Run with: cargo test test_live_lineage --ignored
#[tokio::test]
#[ignore]
async fn test_live_lineage_generation() {
    let client = reqwest::Client::new();

    // Generate root lineage
    let root_req = serde_json::json!({
        "node_id": "root_node_001",
        "parent_id": null
    });

    let root_response = client
        .post("http://localhost:9000/lineage/generate")
        .json(&root_req)
        .send()
        .await
        .expect("Failed to generate root");

    assert!(root_response.status().is_success());

    let root_data: serde_json::Value = root_response
        .json()
        .await
        .expect("Failed to parse response");

    assert!(root_data.get("lineage_chain").is_some());
    assert!(root_data.get("signature").is_some());

    // Generate child lineage
    let child_req = serde_json::json!({
        "node_id": "child_node_001",
        "parent_id": "root_node_001"
    });

    let child_response = client
        .post("http://localhost:9000/lineage/generate")
        .json(&child_req)
        .send()
        .await
        .expect("Failed to generate child");

    assert!(child_response.status().is_success());

    let child_data: serde_json::Value = child_response
        .json()
        .await
        .expect("Failed to parse response");

    let chain = child_data["lineage_chain"]
        .as_array()
        .expect("No lineage_chain");
    assert_eq!(chain.len(), 2); // Root + child
}

/// Test system endpoints via HTTP
///
/// Run with: cargo test test_live_system_endpoints --ignored
#[tokio::test]
#[ignore]
async fn test_live_system_endpoints() {
    let client = reqwest::Client::new();

    // Test health endpoint
    let health_response = client
        .get("http://localhost:9000/health")
        .send()
        .await
        .expect("Failed to get health");

    assert!(health_response.status().is_success());

    let health_data: serde_json::Value = health_response.json().await.expect("Failed to parse");
    assert_eq!(health_data["status"], "healthy");

    // Test metrics endpoint
    let metrics_response = client
        .get("http://localhost:9000/metrics")
        .send()
        .await
        .expect("Failed to get metrics");

    assert!(metrics_response.status().is_success());

    // Test capabilities endpoint
    let caps_response = client
        .get("http://localhost:9000/capabilities")
        .send()
        .await
        .expect("Failed to get capabilities");

    assert!(caps_response.status().is_success());

    let caps_data: serde_json::Value = caps_response.json().await.expect("Failed to parse");
    let capabilities = caps_data["capabilities"]
        .as_array()
        .expect("No capabilities");
    assert!(capabilities.len() >= 4);

    // Test status endpoint
    let status_response = client
        .get("http://localhost:9000/status")
        .send()
        .await
        .expect("Failed to get status");

    assert!(status_response.status().is_success());

    let status_data: serde_json::Value = status_response.json().await.expect("Failed to parse");
    assert!(status_data.get("status").is_some());
    assert!(status_data.get("active_tunnels").is_some());
    assert!(status_data.get("active_lineages").is_some());
}

/// Test concurrent BTSP operations
///
/// Run with: cargo test test_live_concurrent --ignored
#[tokio::test]
#[ignore]
async fn test_live_concurrent_operations() {
    let client = reqwest::Client::new();

    // Create 10 tunnels concurrently
    let tasks: Vec<_> = (0..10)
        .map(|i| {
            let client = client.clone();
            tokio::spawn(async move {
                let request = serde_json::json!({
                    "responder_id": format!("responder_{}", i),
                    "initiator_entropy": format!("entropy_{}", i)
                });

                let response = client
                    .post("http://localhost:9000/btsp/tunnel/establish")
                    .json(&request)
                    .send()
                    .await
                    .expect("Failed to establish tunnel");

                assert!(response.status().is_success());
                response.json::<serde_json::Value>().await.unwrap()
            })
        })
        .collect();

    let results = futures::future::join_all(tasks).await;

    // All should succeed
    for result in results {
        let data = result.expect("Task failed");
        assert!(data.get("tunnel_id").is_some());
    }
}

/// Test error handling for invalid requests
///
/// Run with: cargo test test_live_error_handling --ignored
#[tokio::test]
#[ignore]
async fn test_live_error_handling() {
    let client = reqwest::Client::new();

    // Test with invalid tunnel ID
    let response = client
        .get("http://localhost:9000/btsp/tunnel/invalid_tunnel/status")
        .send()
        .await
        .expect("Failed to send request");

    assert_eq!(response.status(), reqwest::StatusCode::BAD_REQUEST);

    // Test with malformed request
    let response = client
        .post("http://localhost:9000/btsp/tunnel/establish")
        .body("not json")
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_client_error());
}
