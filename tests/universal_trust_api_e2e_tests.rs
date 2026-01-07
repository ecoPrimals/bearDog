//! E2E Tests for Universal Trust API Integration
//!
//! These tests verify that BearDog can handle both:
//! 1. Universal Trust v1 format (generic, capability-based)
//! 2. Legacy format (backward compatible)
//!
//! These tests ensure seamless integration with Songbird and future primals.

use beardog_genetics::EcosystemGeneticEngine;
use beardog_tunnel::api::server::{BearDogApiServer, BearDogApiServerConfig};
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::tunnel::hsm::manager::HsmManager;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

/// Start test server with family seed
async fn start_test_server(
) -> Result<(tokio::task::JoinHandle<()>, String), Box<dyn std::error::Error>> {
    // Use a unique port for each test
    static PORT_COUNTER: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(25000);
    let port = PORT_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse()?;
    let base_url = format!("http://{}", addr);

    // Always clear env vars first to avoid test pollution
    env::remove_var("BEARDOG_FAMILY_SEED");

    // Set family seed for testing
    env::set_var("BEARDOG_FAMILY_SEED", "UniversalTestFamily");
    env::set_var("BEARDOG_HSM_MODE", "software");

    let config = BearDogApiServerConfig {
        bind_addr: addr,
        enable_cors: true,
        version: "test".to_string(),
    };

    let hsm = Arc::new(HsmManager::auto_initialize().await?);
    let genetics = Arc::new(EcosystemGeneticEngine::new()?);
    let provider = Arc::new(BeardogBtspProvider::new(hsm, genetics).await?);
    let server = BearDogApiServer::new(config, provider).await?;

    // Spawn server in background
    let handle = tokio::spawn(async move {
        if let Err(e) = server.serve().await {
            eprintln!("Server error: {}", e);
        }
    });

    // Wait for server to start
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;

    Ok((handle, base_url))
}

// ====================================================================================
// Universal Trust v1 Format Tests
// ====================================================================================

#[tokio::test]
async fn test_universal_trust_get_identity() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Get identity
    let resp = client
        .get(&format!("{}/api/v1/trust/identity", base_url))
        .send()
        .await?;

    assert_eq!(resp.status(), 200);

    let body: Value = resp.json().await?;
    println!(
        "Identity response: {}",
        serde_json::to_string_pretty(&body)?
    );

    // Verify structure (unwrapped - no "data" wrapper, HTTP 200 indicates success)
    assert!(body["encryption_tag"]
        .as_str()
        .unwrap()
        .starts_with("beardog:family:"));
    assert!(body["capabilities"].is_array());
    assert!(body["family_id"].is_string());
    assert!(body["identity_attestations"].is_array());

    // Verify attestations
    let attestations = body["identity_attestations"].as_array().unwrap();
    assert_eq!(attestations.len(), 1);
    assert_eq!(attestations[0]["provider_capability"], "security/identity");
    assert_eq!(attestations[0]["format"], "tag_list");
    assert!(attestations[0]["data"]["tags"].is_array());
    assert!(attestations[0]["data"]["family_id"].is_string());

    Ok(())
}

#[tokio::test]
async fn test_universal_trust_evaluate_same_family() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Get our identity first (unwrapped response)
    let identity_resp = client
        .get(&format!("{}/api/v1/trust/identity", base_url))
        .send()
        .await?;
    let identity: Value = identity_resp.json().await?;
    let our_family = identity["family_id"].as_str().unwrap();

    // Create universal trust evaluation request (same family)
    let request = json!({
        "request_format": "universal_trust_v1",
        "evaluator": {
            "peer_id": "tower2",
            "attestations": [{
                "provider_capability": "security/identity",
                "format": "tag_list",
                "data": {
                    "tags": [format!("beardog:family:{}:tower2", our_family), "btsp_enabled"],
                    "family_id": our_family
                }
            }]
        },
        "context": {
            "discovery_method": "udp_multicast",
            "first_seen_at": "2026-01-03T00:00:00Z",
            "endpoint": "https://192.168.1.134:8080",
            "capabilities": ["orchestration", "federation"]
        }
    });

    // Evaluate trust
    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&request)
        .send()
        .await?;

    assert_eq!(resp.status(), 200);

    let body: Value = resp.json().await?;
    println!(
        "Trust evaluation response (same family): {}",
        serde_json::to_string_pretty(&body)?
    );

    // Verify response (unwrapped - no "data" wrapper)
    assert_eq!(body["response_format"], "universal_trust_v1");
    assert_eq!(body["decision"], "auto_accept");
    assert_eq!(body["confidence"], 1.0);
    assert_eq!(body["reason_code"], "same_genetic_family");
    assert_eq!(body["metadata"]["same_family"], true);
    assert_eq!(body["metadata"]["family_id"], our_family);
    assert_eq!(body["metadata"]["provider"], "beardog");
    assert!(body["expires_at"].is_string());

    Ok(())
}

#[tokio::test]
async fn test_universal_trust_evaluate_different_family() -> Result<(), Box<dyn std::error::Error>>
{
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Get our identity (unwrapped response)
    let identity_resp = client
        .get(&format!("{}/api/v1/trust/identity", base_url))
        .send()
        .await?;
    let identity: Value = identity_resp.json().await?;
    let our_family = identity["family_id"].as_str().unwrap();

    // Create request with DIFFERENT family
    let different_family = "xyz_different";
    let request = json!({
        "request_format": "universal_trust_v1",
        "evaluator": {
            "peer_id": "tower3",
            "attestations": [{
                "provider_capability": "security/identity",
                "format": "tag_list",
                "data": {
                    "tags": [format!("beardog:family:{}:tower3", different_family)],
                    "family_id": different_family
                }
            }]
        },
        "context": {
            "discovery_method": "udp_multicast",
            "endpoint": "https://192.168.1.200:8080"
        }
    });

    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&request)
        .send()
        .await?;

    assert_eq!(resp.status(), 200);

    let body: Value = resp.json().await?;
    println!(
        "Trust evaluation response (different family): {}",
        serde_json::to_string_pretty(&body)?
    );

    // Verify response (unwrapped - no "data" wrapper)
    assert_eq!(body["response_format"], "universal_trust_v1");
    assert_eq!(body["decision"], "prompt_user");
    assert_eq!(body["confidence"], 0.5);
    assert_eq!(body["reason_code"], "different_genetic_family");
    assert_eq!(body["metadata"]["same_family"], false);
    assert_eq!(body["metadata"]["peer_family_id"], different_family);
    assert_eq!(body["metadata"]["our_family_id"], our_family);

    Ok(())
}

#[tokio::test]
async fn test_universal_trust_evaluate_no_lineage() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Create request with NO lineage attestations
    let request = json!({
        "request_format": "universal_trust_v1",
        "evaluator": {
            "peer_id": "legacy_peer",
            "attestations": []  // No attestations!
        },
        "context": {
            "discovery_method": "manual",
            "endpoint": "https://192.168.1.100:8080"
        }
    });

    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&request)
        .send()
        .await?;

    assert_eq!(resp.status(), 200);

    let body: Value = resp.json().await?;
    println!(
        "Trust evaluation response (no lineage): {}",
        serde_json::to_string_pretty(&body)?
    );

    // Verify response (unwrapped - no "data" wrapper)
    assert_eq!(body["response_format"], "universal_trust_v1");
    assert_eq!(body["decision"], "prompt_user");
    assert_eq!(body["confidence"], 0.3);
    assert_eq!(body["reason_code"], "peer_has_no_genetic_lineage");

    Ok(())
}

// ====================================================================================
// Backward Compatibility Tests (Legacy Format)
// ====================================================================================

#[tokio::test]
async fn test_legacy_trust_evaluate() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Get our identity (unwrapped response)
    let identity_resp = client
        .get(&format!("{}/api/v1/trust/identity", base_url))
        .send()
        .await?;
    let identity: Value = identity_resp.json().await?;
    let our_family = identity["family_id"].as_str().unwrap();

    // Create LEGACY format request
    let request = json!({
        "peer_id": "legacy_tower",
        "peer_tags": [format!("beardog:family:{}:legacy_tower", our_family), "btsp_enabled"],
        "connection_info": {"endpoint": "http://192.168.1.150:8080"},
        "context": {"discovery_method": "mdns"}
    });

    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&request)
        .send()
        .await?;

    assert_eq!(resp.status(), 200);

    let body: Value = resp.json().await?;
    println!(
        "Trust evaluation response (legacy format): {}",
        serde_json::to_string_pretty(&body)?
    );

    // Verify response (unwrapped - no "data" wrapper, no response_format for legacy)
    assert!(body["response_format"].is_null());
    assert_eq!(body["decision"], "auto_accept");
    assert_eq!(body["confidence"], 1.0);
    assert!(body["trust_level"].is_string());

    Ok(())
}

// ====================================================================================
// Error Handling Tests
// ====================================================================================

#[tokio::test]
async fn test_universal_trust_missing_evaluator() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Request with missing evaluator
    let request = json!({
        "request_format": "universal_trust_v1",
        "context": {}
    });

    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&request)
        .send()
        .await?;

    assert_eq!(resp.status(), 400);

    let body: Value = resp.json().await?;
    assert!(body["error"].as_str().unwrap().contains("evaluator"));

    Ok(())
}

#[tokio::test]
async fn test_universal_trust_unsupported_format() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    // Request with unsupported format
    let request = json!({
        "request_format": "future_format_v999",
        "evaluator": {
            "peer_id": "test",
            "attestations": []
        }
    });

    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&request)
        .send()
        .await?;

    assert_eq!(resp.status(), 400);

    let body: Value = resp.json().await?;
    assert!(body["error"].as_str().unwrap().contains("Unsupported"));

    Ok(())
}

// ====================================================================================
// Integration Tests (Simulating Songbird Integration)
// ====================================================================================

#[tokio::test]
async fn test_songbird_integration_flow() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    println!("\n=== Simulating Songbird Integration Flow ===\n");

    // Step 1: Songbird queries BearDog for identity on startup
    println!("Step 1: Songbird queries BearDog for identity");
    let identity_resp = client
        .get(&format!("{}/api/v1/trust/identity", base_url))
        .send()
        .await?;
    let identity: Value = identity_resp.json().await?;
    println!("Our identity: {}", identity["encryption_tag"]);
    println!(
        "Our attestations: {}",
        serde_json::to_string_pretty(&identity["identity_attestations"])?
    );

    let our_attestations = identity["identity_attestations"].clone();
    let our_family = identity["family_id"].as_str().unwrap();

    // Step 2: Songbird includes attestations in discovery announcement
    println!("\nStep 2: Songbird includes attestations in discovery");
    let discovery_announcement = json!({
        "peer_id": "songbird-tower1",
        "capabilities": ["orchestration", "federation"],
        "version": "3.0",
        "endpoint": "https://192.168.1.144:8080",
        "identity_attestations": our_attestations
    });
    println!(
        "Discovery announcement: {}",
        serde_json::to_string_pretty(&discovery_announcement)?
    );

    // Step 3: Peer tower discovers us and evaluates trust
    println!("\nStep 3: Peer discovers us and evaluates trust");
    let peer_request = json!({
        "request_format": "universal_trust_v1",
        "evaluator": {
            "peer_id": "songbird-tower1",
            "attestations": our_attestations
        },
        "context": {
            "discovery_method": "udp_multicast",
            "first_seen_at": "2026-01-03T00:00:00Z",
            "endpoint": "https://192.168.1.144:8080"
        }
    });

    let trust_resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&peer_request)
        .send()
        .await?;

    let trust_result: Value = trust_resp.json().await?;
    println!(
        "Trust evaluation: {}",
        serde_json::to_string_pretty(&trust_result)?
    );

    // Step 4: Verify auto-accept (same family) - unwrapped response
    println!("\nStep 4: Verify auto-accept decision");
    assert_eq!(trust_result["decision"], "auto_accept");
    assert_eq!(trust_result["confidence"], 1.0);
    println!("✅ Peer auto-accepted! Mesh connection formed.");

    Ok(())
}

#[tokio::test]
async fn test_cross_tower_integration() -> Result<(), Box<dyn std::error::Error>> {
    let (_handle, base_url) = start_test_server().await?;
    let client = Client::new();

    println!("\n=== Simulating Cross-Tower Integration ===\n");

    // Tower A (us) discovers Tower B (different family)
    let tower_b_request = json!({
        "request_format": "universal_trust_v1",
        "evaluator": {
            "peer_id": "tower-b",
            "attestations": [{
                "provider_capability": "security/identity",
                "format": "tag_list",
                "data": {
                    "tags": ["beardog:family:xyz:tower-b"],
                    "family_id": "xyz"
                }
            }]
        },
        "context": {
            "discovery_method": "udp_multicast",
            "endpoint": "https://192.168.2.100:8080"
        }
    });

    let resp = client
        .post(&format!("{}/api/v1/trust/evaluate", base_url))
        .json(&tower_b_request)
        .send()
        .await?;

    let result: Value = resp.json().await?;
    println!(
        "Cross-tower evaluation: {}",
        serde_json::to_string_pretty(&result)?
    );

    assert_eq!(result["decision"], "prompt_user");
    assert_eq!(result["reason_code"], "different_genetic_family");
    println!("✅ Different family detected - user prompt required");

    Ok(())
}
