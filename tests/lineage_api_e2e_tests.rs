//! End-to-End Tests for Lineage API
//!
//! These tests verify the complete lineage API flow including:
//! - Genesis creation
//! - Child spawning
//! - Proof generation and verification
//! - Same-family checks
//! - Error handling

use std::sync::Arc;

use beardog_genetics::EcosystemGeneticEngine;
use beardog_tunnel::{
    tunnel::hsm::manager::HsmManager, BearDogApiServer, BearDogApiServerConfig,
    BeardogBtspProvider,
};
use reqwest::Client;
use serde_json::{json, Value};
use tokio::time::{sleep, Duration};

/// Helper to start a test server
async fn start_test_server(
) -> Result<(tokio::task::JoinHandle<()>, String), Box<dyn std::error::Error>> {
    // Use a fixed port range for testing to avoid conflicts
    static PORT_COUNTER: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(19000);
    let port = PORT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    
    let addr: std::net::SocketAddr = format!("127.0.0.1:{}", port).parse()?;
    let base_url = format!("http://{}", addr);

    // Create server
    let config = BearDogApiServerConfig {
        bind_addr: addr,
        enable_cors: true,
        version: "test".to_string(),
    };

    let hsm = Arc::new(HsmManager::auto_initialize().await?);
    let genetics = Arc::new(EcosystemGeneticEngine::new()?);
    let provider = Arc::new(BeardogBtspProvider::new(hsm, genetics).await?);
    let server = BearDogApiServer::new(config, provider).await?;
    
    // Start server in background
    let handle = tokio::spawn(async move {
        let _ = server.serve().await;
    });

    // Give server time to start and bind
    sleep(Duration::from_millis(300)).await;

    Ok((handle, base_url))
}

#[tokio::test]
async fn test_lineage_api_full_flow() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // 1. Create genesis lineage
    let create_resp = client
        .post(&format!("{}/api/v1/lineage/create", base_url))
        .json(&json!({ "service_type": "tower" }))
        .send()
        .await?;

    assert_eq!(create_resp.status(), 200);
    let create_body: Value = create_resp.json().await?;
    let genesis_lineage_id = create_body["data"]["lineage_id"]
        .as_str()
        .expect("lineage_id should be present")
        .to_string();

    assert!(genesis_lineage_id.starts_with("lineage:tower:"));

    // 2. Spawn child lineage
    let spawn_resp = client
        .post(&format!("{}/api/v1/lineage/spawn", base_url))
        .json(&json!({
            "parent_lineage": genesis_lineage_id,
            "service_type": "songbird"
        }))
        .send()
        .await?;

    assert_eq!(spawn_resp.status(), 200);
    let spawn_body: Value = spawn_resp.json().await?;
    let child_lineage_id = spawn_body["data"]["lineage_id"]
        .as_str()
        .expect("child lineage_id should be present")
        .to_string();
    let child_proof = &spawn_body["data"]["proof"];

    assert!(child_lineage_id.starts_with("lineage:songbird:"));

    // 3. Verify the child proof
    let verify_resp = client
        .post(&format!("{}/api/v1/lineage/proof/verify", base_url))
        .json(&json!({ "proof": child_proof }))
        .send()
        .await?;

    assert_eq!(verify_resp.status(), 200);
    let verify_body: Value = verify_resp.json().await?;
    assert_eq!(verify_body["data"]["valid"], true);
    assert_eq!(verify_body["data"]["same_genesis"], true);

    // 4. Check same family
    let same_family_resp = client
        .post(&format!("{}/api/v1/lineage/same_family", base_url))
        .json(&json!({
            "lineage_a": genesis_lineage_id,
            "lineage_b": child_lineage_id
        }))
        .send()
        .await?;

    assert_eq!(same_family_resp.status(), 200);
    let same_family_body: Value = same_family_resp.json().await?;
    assert_eq!(same_family_body["data"]["same_family"], true);

    // 5. Get current lineage
    let current_resp = client
        .get(&format!("{}/api/v1/lineage/current", base_url))
        .send()
        .await?;

    assert_eq!(current_resp.status(), 200);
    let current_body: Value = current_resp.json().await?;
    assert_eq!(
        current_body["data"]["lineage_id"],
        child_lineage_id,
        "Current lineage should be the last created"
    );

    Ok(())
}

#[tokio::test]
async fn test_lineage_api_cross_tower_same_genesis() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // 1. Create Tower A genesis
    let tower_a_resp = client
        .post(&format!("{}/api/v1/lineage/create", base_url))
        .json(&json!({ "service_type": "tower-a" }))
        .send()
        .await?;

    assert_eq!(tower_a_resp.status(), 200);
    let tower_a_body: Value = tower_a_resp.json().await?;
    let tower_a_lineage = tower_a_body["data"]["lineage_id"]
        .as_str()
        .expect("tower_a lineage_id should be present")
        .to_string();

    // 2. Spawn child from Tower A
    let child_a_resp = client
        .post(&format!("{}/api/v1/lineage/spawn", base_url))
        .json(&json!({
            "parent_lineage": tower_a_lineage,
            "service_type": "songbird-a"
        }))
        .send()
        .await?;

    assert_eq!(child_a_resp.status(), 200);
    let child_a_body: Value = child_a_resp.json().await?;
    let child_a_proof = &child_a_body["data"]["proof"];

    // 3. Verify child A proof - should have same_genesis=true (current lineage)
    let verify_a_resp = client
        .post(&format!("{}/api/v1/lineage/proof/verify", base_url))
        .json(&json!({ "proof": child_a_proof }))
        .send()
        .await?;

    assert_eq!(verify_a_resp.status(), 200);
    let verify_a_body: Value = verify_a_resp.json().await?;
    assert_eq!(verify_a_body["data"]["valid"], true, "Child A proof should be valid");
    assert_eq!(verify_a_body["data"]["same_genesis"], true, "Child A should have same_genesis=true");

    // 4. Create Tower B genesis (different genesis!)
    let tower_b_resp = client
        .post(&format!("{}/api/v1/lineage/create", base_url))
        .json(&json!({ "service_type": "tower-b" }))
        .send()
        .await?;

    assert_eq!(tower_b_resp.status(), 200);
    let tower_b_body: Value = tower_b_resp.json().await?;
    let tower_b_lineage = tower_b_body["data"]["lineage_id"]
        .as_str()
        .expect("tower_b lineage_id should be present")
        .to_string();

    // 5. Spawn child from Tower B
    let child_b_resp = client
        .post(&format!("{}/api/v1/lineage/spawn", base_url))
        .json(&json!({
            "parent_lineage": tower_b_lineage,
            "service_type": "songbird-b"
        }))
        .send()
        .await?;

    assert_eq!(child_b_resp.status(), 200);
    let child_b_body: Value = child_b_resp.json().await?;
    let child_b_proof = &child_b_body["data"]["proof"];

    // 6. Verify child A proof again - should now have same_genesis=false (different current lineage)
    let verify_a_again_resp = client
        .post(&format!("{}/api/v1/lineage/proof/verify", base_url))
        .json(&json!({ "proof": child_a_proof }))
        .send()
        .await?;

    assert_eq!(verify_a_again_resp.status(), 200);
    let verify_a_again_body: Value = verify_a_again_resp.json().await?;
    assert_eq!(verify_a_again_body["data"]["valid"], true, "Child A proof should still be valid");
    assert_eq!(
        verify_a_again_body["data"]["same_genesis"], false,
        "Child A should now have same_genesis=false (current is Tower B)"
    );

    // 7. Verify child B proof - should have same_genesis=true (current lineage)
    let verify_b_resp = client
        .post(&format!("{}/api/v1/lineage/proof/verify", base_url))
        .json(&json!({ "proof": child_b_proof }))
        .send()
        .await?;

    assert_eq!(verify_b_resp.status(), 200);
    let verify_b_body: Value = verify_b_resp.json().await?;
    assert_eq!(verify_b_body["data"]["valid"], true, "Child B proof should be valid");
    assert_eq!(verify_b_body["data"]["same_genesis"], true, "Child B should have same_genesis=true");

    // 8. Check same_family - should return false for cross-tower
    let same_family_resp = client
        .post(&format!("{}/api/v1/lineage/same_family", base_url))
        .json(&json!({
            "lineage_a": tower_a_lineage,
            "lineage_b": tower_b_lineage
        }))
        .send()
        .await?;

    assert_eq!(same_family_resp.status(), 200);
    let same_family_body: Value = same_family_resp.json().await?;
    assert_eq!(
        same_family_body["data"]["same_family"], false,
        "Cross-tower lineages should not be same_family"
    );

    Ok(())
}

#[tokio::test]
async fn test_lineage_api_concurrent_creates() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Create multiple lineages concurrently
    let mut handles = vec![];
    for i in 0..10 {
        let client = client.clone();
        let base_url = base_url.clone();
        handles.push(tokio::spawn(async move {
            let resp = client
                .post(&format!("{}/api/v1/lineage/create", base_url))
                .json(&json!({ "service_type": format!("service-{}", i) }))
                .send()
                .await
                .expect("Failed to send request");
            assert_eq!(resp.status(), 200);
            let body: Value = resp.json().await.expect("Failed to parse JSON");
            body["data"]["lineage_id"]
                .as_str()
                .unwrap()
                .to_string()
        }));
    }

    let lineage_ids: Vec<String> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.expect("Task panicked"))
        .collect();

    // All should be unique
    let unique_count = lineage_ids.iter().collect::<std::collections::HashSet<_>>().len();
    assert_eq!(unique_count, 10, "All lineage IDs should be unique");

    Ok(())
}

#[tokio::test]
async fn test_lineage_api_invalid_parent() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Try to spawn with non-existent parent
    let spawn_resp = client
        .post(&format!("{}/api/v1/lineage/spawn", base_url))
        .json(&json!({
            "parent_lineage": "lineage:fake:123:abc:xyz",
            "service_type": "songbird"
        }))
        .send()
        .await?;

    assert_eq!(spawn_resp.status(), 400);

    Ok(())
}

#[tokio::test]
async fn test_lineage_api_malformed_requests() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Missing service_type
    let resp = client
        .post(&format!("{}/api/v1/lineage/create", base_url))
        .json(&json!({}))
        .send()
        .await?;
    assert_eq!(resp.status(), 422); // Unprocessable Entity for validation errors

    // Invalid JSON
    let resp = client
        .post(&format!("{}/api/v1/lineage/create", base_url))
        .header("Content-Type", "application/json")
        .body("{not json}")
        .send()
        .await?;
    // Axum returns 415 (Unsupported Media Type) for unparseable JSON
    assert!(resp.status() == 400 || resp.status() == 415 || resp.status() == 422);

    Ok(())
}

#[tokio::test]
async fn test_lineage_api_deep_hierarchy() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Create a 5-level hierarchy
    let mut current_lineage = String::new();

    for depth in 0..5 {
        let service_type = format!("level-{}", depth);

        let resp = if depth == 0 {
            // Genesis
            client
                .post(&format!("{}/api/v1/lineage/create", base_url))
                .json(&json!({ "service_type": service_type }))
                .send()
                .await?
        } else {
            // Child
            client
                .post(&format!("{}/api/v1/lineage/spawn", base_url))
                .json(&json!({
                    "parent_lineage": current_lineage,
                    "service_type": service_type
                }))
                .send()
                .await?
        };

        assert_eq!(resp.status(), 200);
        let body: Value = resp.json().await?;
        current_lineage = body["data"]["lineage_id"]
            .as_str()
            .unwrap()
            .to_string();
    }

    // Final lineage should still be verifiable
    let sign_resp = client
        .post(&format!("{}/api/v1/lineage/sign", base_url))
        .json(&json!({ "lineage_id": current_lineage }))
        .send()
        .await?;

    assert_eq!(sign_resp.status(), 200);
    let proof = sign_resp.json::<Value>().await?;

    let verify_resp = client
        .post(&format!("{}/api/v1/lineage/proof/verify", base_url))
        .json(&json!({ "proof": proof["data"] }))
        .send()
        .await?;

    assert_eq!(verify_resp.status(), 200);
    let verify_body: Value = verify_resp.json().await?;
    assert_eq!(verify_body["data"]["valid"], true);

    Ok(())
}

#[tokio::test]
async fn test_lineage_api_health_check() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    let health_resp = client
        .get(&format!("{}/health", base_url))
        .send()
        .await?;

    assert_eq!(health_resp.status(), 200);
    let body: Value = health_resp.json().await?;
    assert_eq!(body["status"], "healthy");
    assert!(body["capabilities"]
        .as_array()
        .unwrap()
        .iter()
        .any(|c| c == "lineage"));

    Ok(())
}

#[tokio::test]
async fn test_lineage_api_stress_spawn() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Create genesis
    let create_resp = client
        .post(&format!("{}/api/v1/lineage/create", base_url))
        .json(&json!({ "service_type": "tower" }))
        .send()
        .await?;

    let genesis_id = create_resp.json::<Value>().await?["data"]["lineage_id"]
        .as_str()
        .unwrap()
        .to_string();

    // Spawn 50 children concurrently
    let mut handles = vec![];
    for i in 0..50 {
        let client = client.clone();
        let base_url = base_url.clone();
        let genesis_id = genesis_id.clone();

        handles.push(tokio::spawn(async move {
            let resp = client
                .post(&format!("{}/api/v1/lineage/spawn", base_url))
                .json(&json!({
                    "parent_lineage": genesis_id,
                    "service_type": format!("child-{}", i)
                }))
                .send()
                .await
                .expect("Failed to send request");

            assert_eq!(resp.status(), 200);
        }));
    }

    futures::future::join_all(handles).await;

    Ok(())
}

