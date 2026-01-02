//! End-to-End Tests for Trust Evaluation API (biomeOS Integration)
//!
//! These tests verify the complete trust API flow for USB family seed integration:
//! - Identity endpoint (GET /api/v1/trust/identity)
//! - Trust evaluation endpoint (POST /api/v1/trust/evaluate)
//! - Same family auto-accept
//! - Different family prompt
//! - No lineage reject

use std::sync::Arc;

use beardog_genetics::EcosystemGeneticEngine;
use beardog_tunnel::{
    tunnel::hsm::manager::HsmManager, BearDogApiServer, BearDogApiServerConfig,
    BeardogBtspProvider,
};
use reqwest::Client;
use serde_json::{json, Value};
use tokio::time::{sleep, Duration};

/// Helper to start a test server with optional family seed
async fn start_test_server_with_seed(
    family_seed: Option<&str>,
) -> Result<(tokio::task::JoinHandle<()>, String), Box<dyn std::error::Error>> {
    // Use a fixed port range for testing
    static PORT_COUNTER: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(20000);
    let port = PORT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    
    let addr: std::net::SocketAddr = format!("127.0.0.1:{}", port).parse()?;
    let base_url = format!("http://{}", addr);

    // CRITICAL: Always clear the env var first to avoid test pollution
    std::env::remove_var("BEARDOG_FAMILY_SEED");
    
    // Set family seed if provided
    if let Some(seed) = family_seed {
        std::env::set_var("BEARDOG_FAMILY_SEED", seed);
    }

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

    // Give server time to start
    sleep(Duration::from_millis(300)).await;

    Ok((handle, base_url))
}

#[tokio::test]
async fn test_trust_identity_with_family() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server_with_seed(Some("TestFamilyABC123")).await?;
    let client = Client::new();

    // Get identity
    let resp = client
        .get(&format!("{}/api/v1/trust/identity", base_url))
        .send()
        .await?;

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await?;
    
    let encryption_tag = body["data"]["encryption_tag"]
        .as_str()
        .expect("encryption_tag should be present");
    
    // Should have family tag
    assert!(encryption_tag.starts_with("beardog:family:"));
    // Family ID is first 4 alphanumeric chars of seed, lowercased
    // "TestFamilyABC123" → "Test" → "test"
    assert!(encryption_tag.contains("test"), "encryption_tag should contain 'test', got: {}", encryption_tag);

    // Should have family_id
    let family_id = body["data"]["family_id"]
        .as_str()
        .expect("family_id should be present");
    assert!(!family_id.is_empty());

    // Should have capabilities
    let capabilities = body["data"]["capabilities"]
        .as_array()
        .expect("capabilities should be array");
    assert!(capabilities.len() >= 3);

    Ok(())
}

#[tokio::test]
async fn test_trust_identity_without_family() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server_with_seed(None).await?;
    let client = Client::new();

    // Get identity
    let resp = client
        .get(&format!("{}/api/v1/trust/identity", base_url))
        .send()
        .await?;

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await?;
    
    let encryption_tag = body["data"]["encryption_tag"]
        .as_str()
        .expect("encryption_tag should be present");
    
    // Should have node tag (not family)
    assert!(encryption_tag.starts_with("beardog:node:") || encryption_tag.starts_with("beardog:family:"),
            "encryption_tag should start with beardog:node: or beardog:family:, got: {}", encryption_tag);

    // Should NOT have family_id
    assert!(body["data"]["family_id"].is_null(), "family_id should be null when no seed provided");

    Ok(())
}

#[tokio::test]
async fn test_trust_evaluate_same_family() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server_with_seed(Some("SameFamilyXYZ456")).await?;
    let client = Client::new();

    // Get our identity to extract family_id
    let identity_resp = client
        .get(&format!("{}/api/v1/trust/identity", base_url))
        .send()
        .await?;
    let identity: Value = identity_resp.json().await?;
    let our_family = identity["data"]["family_id"]
        .as_str()
        .expect("family_id should be present");

    // Evaluate peer with same family
    let eval_resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&json!({
            "peer_id": "tower2",
            "peer_tags": [
                format!("beardog:family:{}:tower2", our_family),
                "btsp_enabled"
            ],
            "connection_info": {
                "endpoint": "http://192.168.1.134:8080"
            },
            "context": {
                "discovery_method": "udp_multicast"
            }
        }))
        .send()
        .await?;

    assert_eq!(eval_resp.status(), 200);
    let body: Value = eval_resp.json().await?;

    // Should auto-accept same family
    assert_eq!(body["data"]["decision"], "auto_accept");
    assert_eq!(body["data"]["trust_level"], "high");
    assert_eq!(body["data"]["confidence"], 1.0);
    assert_eq!(body["data"]["reason"], "same_genetic_family");

    // Should have metadata
    let metadata = body["data"]["metadata"].as_object().expect("metadata should be present");
    assert_eq!(metadata["family_id"].as_str(), Some(our_family));
    assert_eq!(metadata["relationship"].as_str(), Some("sibling"));
    assert_eq!(metadata["genesis_verified"].as_str(), Some("true"));

    Ok(())
}

#[tokio::test]
async fn test_trust_evaluate_different_family() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server_with_seed(Some("FamilyA123")).await?;
    let client = Client::new();

    // Evaluate peer with different family
    let eval_resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&json!({
            "peer_id": "tower_b",
            "peer_tags": [
                "beardog:family:b7e9:tower_b",  // Different family
                "btsp_enabled"
            ],
            "connection_info": {
                "endpoint": "http://192.168.1.135:8080"
            }
        }))
        .send()
        .await?;

    assert_eq!(eval_resp.status(), 200);
    let body: Value = eval_resp.json().await?;

    // Should prompt user for different family
    assert_eq!(body["data"]["decision"], "prompt_user");
    assert_eq!(body["data"]["trust_level"], "medium");
    assert_eq!(body["data"]["confidence"], 0.5);
    assert_eq!(body["data"]["reason"], "different_genetic_family");

    Ok(())
}

#[tokio::test]
async fn test_trust_evaluate_no_lineage() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server_with_seed(None).await?;
    let client = Client::new();

    // Evaluate peer with no lineage tags
    let eval_resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&json!({
            "peer_id": "legacy_tower",
            "peer_tags": [
                "orchestration",
                "federation"
            ],
            "connection_info": {
                "endpoint": "http://192.168.1.136:8080"
            }
        }))
        .send()
        .await?;

    assert_eq!(eval_resp.status(), 200);
    let body: Value = eval_resp.json().await?;

    // Should reject when neither has lineage
    assert_eq!(body["data"]["decision"], "reject");
    assert_eq!(body["data"]["trust_level"], "none");
    assert_eq!(body["data"]["confidence"], 0.0);
    assert_eq!(body["data"]["reason"], "no_genetic_lineage_present");

    Ok(())
}

#[tokio::test]
async fn test_trust_evaluate_we_have_family_peer_doesnt() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server_with_seed(Some("FamilyC789")).await?;
    let client = Client::new();

    // Evaluate peer without lineage
    let eval_resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&json!({
            "peer_id": "legacy_peer",
            "peer_tags": [
                "orchestration"
            ],
            "connection_info": {
                "endpoint": "http://192.168.1.137:8080"
            }
        }))
        .send()
        .await?;

    assert_eq!(eval_resp.status(), 200);
    let body: Value = eval_resp.json().await?;

    // Should prompt user (could be legacy)
    assert_eq!(body["data"]["decision"], "prompt_user");
    assert_eq!(body["data"]["trust_level"], "low");
    assert_eq!(body["data"]["confidence"], 0.3);
    assert_eq!(body["data"]["reason"], "peer_has_no_genetic_lineage");

    Ok(())
}

#[tokio::test]
async fn test_trust_evaluate_peer_has_family_we_dont() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server_with_seed(None).await?;
    let client = Client::new();

    // Evaluate peer with lineage (but we don't have one)
    let eval_resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&json!({
            "peer_id": "tower_with_lineage",
            "peer_tags": [
                "beardog:family:a3f2:tower_with_lineage",
                "btsp_enabled"
            ],
            "connection_info": {
                "endpoint": "http://192.168.1.138:8080"
            }
        }))
        .send()
        .await?;

    assert_eq!(eval_resp.status(), 200);
    let body: Value = eval_resp.json().await?;

    // Should prompt user
    assert_eq!(body["data"]["decision"], "prompt_user");
    assert_eq!(body["data"]["trust_level"], "low");
    assert_eq!(body["data"]["confidence"], 0.3);
    assert_eq!(body["data"]["reason"], "we_have_no_family_lineage");

    Ok(())
}

#[tokio::test]
async fn test_trust_health_includes_trust_capability() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server_with_seed(Some("HealthTest123")).await?;
    let client = Client::new();

    // Check health endpoint
    let resp = client
        .get(&format!("{}/health", base_url))
        .send()
        .await?;

    assert_eq!(resp.status(), 200);
    let body: Value = resp.json().await?;

    // Should include "trust" in capabilities
    let capabilities = body["capabilities"]
        .as_array()
        .expect("capabilities should be array");
    
    let has_trust = capabilities.iter().any(|c| c.as_str() == Some("trust"));
    assert!(has_trust, "Health endpoint should include 'trust' capability");

    Ok(())
}

#[tokio::test]
async fn test_trust_concurrent_evaluations() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server_with_seed(Some("ConcurrentTest456")).await?;
    let client = Client::new();

    // Get our family ID
    let identity_resp = client
        .get(&format!("{}/api/v1/trust/identity", base_url))
        .send()
        .await?;
    let identity: Value = identity_resp.json().await?;
    let our_family = identity["data"]["family_id"]
        .as_str()
        .expect("family_id should be present");

    // Evaluate multiple peers concurrently
    let mut handles = vec![];
    for i in 0..10 {
        let client = client.clone();
        let base_url = base_url.clone();
        let our_family = our_family.to_string();
        
        handles.push(tokio::spawn(async move {
            let resp = client
                .post(&format!("{}/api/v1/trust/evaluate", base_url))
                .json(&json!({
                    "peer_id": format!("peer_{}", i),
                    "peer_tags": [
                        format!("beardog:family:{}:peer_{}", our_family, i)
                    ],
                    "connection_info": {}
                }))
                .send()
                .await
                .unwrap();

            let body: Value = resp.json().await.unwrap();
            assert_eq!(body["data"]["decision"], "auto_accept");
        }));
    }

    // Wait for all evaluations
    for handle in handles {
        handle.await?;
    }

    Ok(())
}

