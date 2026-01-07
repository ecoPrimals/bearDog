//! E2E Tests for Capability-Based IPC
//!
//! Tests full integration scenarios:
//! - Federation discovery flows
//! - Multi-primal compatibility
//! - Environment-driven configuration
//! - Concurrent access patterns
//! - Protocol negotiation

use beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer;
use beardog_tunnel::btsp_provider::BeardogBtspProvider;
use beardog_tunnel::tunnel::hsm::HsmManager;
use beardog_genetics::ecosystem_evolution::engine::EcosystemGeneticEngine;
use std::sync::Arc;
use tokio::net::UnixStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tempfile::tempdir;
use serde_json::json;

/// Helper to create test server with specific family/node
async fn create_test_server(
    family: &str,
    node: &str,
) -> (Arc<UnixSocketIpcServer>, tokio::task::JoinHandle<()>, std::path::PathBuf) {
    let dir = tempdir().unwrap();
    let socket_path = dir.path().join(format!("beardog-{}-{}.sock", family, node));
    
    // Set environment
    std::env::set_var("FAMILY_ID", family);
    std::env::set_var("NODE_ID", node);
    std::env::set_var("BEARDOG_HSM_MODE", "software");
    
    // Create HSM manager in software mode for testing
    let hsm = Arc::new(
        HsmManager::auto_initialize()
            .await
            .expect("Failed to initialize HSM")
    );
    
    // Create genetics engine
    let genetics = Arc::new(
        EcosystemGeneticEngine::new()
            .expect("Failed to create genetics engine")
    );
    
    // Create provider
    let btsp_provider = Arc::new(
        BeardogBtspProvider::new(hsm, genetics)
            .await
            .expect("Failed to create BTSP provider")
    );
    
    let server = Arc::new(
        UnixSocketIpcServer::new(&socket_path, btsp_provider)
            .await
            .expect("Failed to create server")
    );
    
    let server_clone: Arc<UnixSocketIpcServer> = Arc::clone(&server);
    let socket_path_clone = socket_path.clone();
    let handle = tokio::spawn(async move {
        let _ = server_clone.start().await;
    });
    
    // Wait for socket file to be created (with timeout)
    let mut attempts = 0;
    while !socket_path.exists() && attempts < 50 {
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        attempts += 1;
    }
    
    if !socket_path.exists() {
        panic!("Socket file not created after 500ms: {}", socket_path.display());
    }
    
    // Additional small delay to ensure server is ready
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    
    std::env::remove_var("BEARDOG_HSM_MODE");
    
    (server, handle, socket_path_clone)
}

/// Helper to send JSON-RPC request and get response
async fn send_jsonrpc_request(
    socket_path: &std::path::Path,
    method: &str,
    params: Option<serde_json::Value>,
    id: i32,
) -> Result<serde_json::Value, String> {
    let mut stream = UnixStream::connect(socket_path)
        .await
        .map_err(|e| format!("Connection failed: {}", e))?;
    
    let request = if let Some(p) = params {
        json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": p,
            "id": id
        })
    } else {
        json!({
            "jsonrpc": "2.0",
            "method": method,
            "id": id
        })
    };
    
    let request_str = format!("{}\n", serde_json::to_string(&request).unwrap());
    stream.write_all(request_str.as_bytes())
        .await
        .map_err(|e| format!("Write failed: {}", e))?;
    
    let mut response = String::new();
    stream.read_to_string(&mut response)
        .await
        .map_err(|e| format!("Read failed: {}", e))?;
    
    serde_json::from_str(&response)
        .map_err(|e| format!("Parse failed: {}", e))
}

// ========================================================================
// E2E: Federation Discovery Flow
// ========================================================================

#[tokio::test]
async fn e2e_dual_tower_federation() {
    // Setup: Two towers in same family
    let (_server1, _handle1, socket1) = create_test_server("nat0", "tower1").await;
    let (_server2, _handle2, socket2) = create_test_server("nat0", "tower2").await;
    
    // Tower 1 discovers itself
    let response = send_jsonrpc_request(&socket1, "identity", None, 1)
        .await
        .unwrap();
    assert_eq!(response["result"]["family"], "nat0");
    assert_eq!(response["result"]["node"], "tower1");
    
    // Tower 2 discovers itself
    let response = send_jsonrpc_request(&socket2, "identity", None, 2)
        .await
        .unwrap();
    assert_eq!(response["result"]["family"], "nat0");
    assert_eq!(response["result"]["node"], "tower2");
    
    // Tower 1 evaluates Tower 2 (same family)
    let response = send_jsonrpc_request(
        &socket1,
        "trust.evaluate_peer",
        Some(json!({
            "peer_id": "tower2",
            "peer_family": "nat0"
        })),
        3
    ).await.unwrap();
    assert_eq!(response["result"]["trust_level"], 1);
    assert_eq!(response["result"]["reason"], "same_genetic_family");
    
    // Tower 2 evaluates Tower 1 (same family)
    let response = send_jsonrpc_request(
        &socket2,
        "trust.evaluate_peer",
        Some(json!({
            "peer_id": "tower1",
            "peer_family": "nat0"
        })),
        4
    ).await.unwrap();
    assert_eq!(response["result"]["trust_level"], 1);
    assert_eq!(response["result"]["reason"], "same_genetic_family");
}

#[tokio::test]
async fn e2e_cross_family_evaluation() {
    // Setup: Two towers in different families
    let (_server1, _handle1, socket1) = create_test_server("family-a", "tower-alpha").await;
    let (_server2, _handle2, socket2) = create_test_server("family-b", "tower-beta").await;
    
    // Tower Alpha evaluates Tower Beta (different family)
    let response = send_jsonrpc_request(
        &socket1,
        "trust.evaluate_peer",
        Some(json!({
            "peer_id": "tower-beta",
            "peer_family": "family-b"
        })),
        1
    ).await.unwrap();
    assert_eq!(response["result"]["trust_level"], 0);
    assert_eq!(response["result"]["reason"], "different_family");
    
    // Tower Beta evaluates Tower Alpha (different family)
    let response = send_jsonrpc_request(
        &socket2,
        "trust.evaluate_peer",
        Some(json!({
            "peer_id": "tower-alpha",
            "peer_family": "family-a"
        })),
        2
    ).await.unwrap();
    assert_eq!(response["result"]["trust_level"], 0);
    assert_eq!(response["result"]["reason"], "different_family");
}

// ========================================================================
// E2E: Songbird Integration Scenarios
// ========================================================================

#[tokio::test]
async fn e2e_songbird_discovery_protocol() {
    let (_server, _handle, socket) = create_test_server("nat0", "tower1").await;
    
    // Simulate Songbird's discovery sequence
    
    // Step 1: Health check
    let response = send_jsonrpc_request(&socket, "health.check", None, 1)
        .await
        .unwrap();
    assert_eq!(response["result"]["status"], "healthy");
    assert_eq!(response["result"]["primal"], "beardog");
    
    // Step 2: Get identity
    let response = send_jsonrpc_request(&socket, "identity.get_family", None, 2)
        .await
        .unwrap();
    assert_eq!(response["result"]["family"], "nat0");
    assert_eq!(response["result"]["node"], "tower1");
    
    // Step 3: Get capabilities
    let response = send_jsonrpc_request(&socket, "capabilities.get_capabilities", None, 3)
        .await
        .unwrap();
    let capabilities = response["result"]["provided_capabilities"].as_array().unwrap();
    let cap_types: Vec<String> = capabilities.iter()
        .map(|c| c["type"].as_str().unwrap().to_string())
        .collect();
    assert!(cap_types.contains(&"security".to_string()));
    assert!(cap_types.contains(&"encryption".to_string()));
    assert!(cap_types.contains(&"trust".to_string()));
    
    // Step 4: Evaluate peer trust
    let response = send_jsonrpc_request(
        &socket,
        "trust.evaluate_peer",
        Some(json!({
            "peer_id": "tower2",
            "peer_family": "nat0",
            "requested_operation": "federation"
        })),
        4
    ).await.unwrap();
    assert_eq!(response["result"]["trust_level"], 1);
}

// ========================================================================
// E2E: Concurrent Federation
// ========================================================================

#[tokio::test]
async fn e2e_concurrent_multi_tower_federation() {
    let (_server, _handle, socket) = create_test_server("nat0", "tower1").await;
    
    // Simulate multiple towers evaluating simultaneously
    let mut handles = vec![];
    
    for i in 2..12 {
        let socket_clone = socket.clone();
        let handle = tokio::spawn(async move {
            send_jsonrpc_request(
                &socket_clone,
                "trust.evaluate_peer",
                Some(json!({
                    "peer_id": format!("tower{}", i),
                    "peer_family": "nat0"
                })),
                i
            ).await
        });
        handles.push(handle);
    }
    
    let results = futures::future::join_all(handles).await;
    
    for result in results {
        let response = result.unwrap().unwrap();
        assert_eq!(response["result"]["trust_level"], 1);
    }
}

// ========================================================================
// E2E: Environment-Driven Configuration
// ========================================================================

#[tokio::test]
async fn e2e_dynamic_family_configuration() {
    // Test that different environments create different identities
    let configs = vec![
        ("production-family", "prod-node-1"),
        ("staging-family", "stage-node-1"),
        ("dev-family", "dev-node-1"),
    ];
    
    for (family, node) in configs {
        let (_server, _handle, socket) = create_test_server(family, node).await;
        
        let response = send_jsonrpc_request(&socket, "identity", None, 1)
            .await
            .unwrap();
        
        assert_eq!(response["result"]["family"], family);
        assert_eq!(response["result"]["node"], node);
        
        // Verify trust evaluation uses correct family
        let response = send_jsonrpc_request(
            &socket,
            "trust.evaluate_peer",
            Some(json!({
                "peer_id": "peer",
                "peer_family": family
            })),
            2
        ).await.unwrap();
        assert_eq!(response["result"]["trust_level"], 1);
    }
}

// ========================================================================
// E2E: Protocol Flexibility
// ========================================================================

#[tokio::test]
async fn e2e_flexible_parameter_conventions() {
    let (_server, _handle, socket) = create_test_server("nat0", "tower1").await;
    
    // Test different parameter naming conventions
    let conventions = vec![
        // Songbird style
        json!({"peer_id": "p1", "peer_family": "nat0"}),
        // ToadStool style
        json!({"id": "p2", "family": "nat0"}),
        // Generic style
        json!({"peer": "p3", "family": "nat0"}),
    ];
    
    for (idx, params) in conventions.iter().enumerate() {
        let response = send_jsonrpc_request(
            &socket,
            "trust.evaluate_peer",
            Some(params.clone()),
            idx as i32
        ).await.unwrap();
        
        assert_eq!(response["result"]["trust_level"], 1);
    }
}

#[tokio::test]
async fn e2e_method_namespace_flexibility() {
    let (_server, _handle, socket) = create_test_server("nat0", "tower1").await;
    
    let params = json!({
        "peer_id": "peer123",
        "peer_family": "nat0"
    });
    
    // Test different namespace conventions
    let methods = vec![
        "trust.evaluate_peer",
        "security.evaluate",
        "trust.evaluate",
    ];
    
    for (idx, method) in methods.iter().enumerate() {
        let response = send_jsonrpc_request(
            &socket,
            method,
            Some(params.clone()),
            idx as i32
        ).await.unwrap();
        
        assert_eq!(response["result"]["trust_level"], 1);
    }
}

// ========================================================================
// E2E: Primal Sovereignty Validation
// ========================================================================

#[tokio::test]
async fn e2e_no_primal_hardcoding() {
    let (_server, _handle, socket) = create_test_server("nat0", "tower1").await;
    
    // Get all responses and verify no hardcoded primal names
    let methods = vec!["identity", "capabilities", "health.check"];
    
    for method in methods {
        let response = send_jsonrpc_request(&socket, method, None, 1)
            .await
            .unwrap();
        
        let response_str = serde_json::to_string(&response).unwrap().to_lowercase();
        
        // Should not contain other primal names
        assert!(!response_str.contains("songbird"), "Response contains 'songbird' for method: {}", method);
        assert!(!response_str.contains("toadstool"), "Response contains 'toadstool' for method: {}", method);
        assert!(!response_str.contains("squirrel"), "Response contains 'squirrel' for method: {}", method);
        
        // Should only reference itself
        assert!(response_str.contains("beardog"), "Response missing 'beardog' for method: {}", method);
    }
}

#[tokio::test]
async fn e2e_self_knowledge_only() {
    let (_server, _handle, socket) = create_test_server("test-family", "test-node").await;
    
    // Verify all identity comes from environment, not hardcoding
    let response = send_jsonrpc_request(&socket, "identity", None, 1)
        .await
        .unwrap();
    
    assert_eq!(response["result"]["primal"], "beardog");
    assert_eq!(response["result"]["family"], "test-family");
    assert_eq!(response["result"]["node"], "test-node");
    
    // Verify lineage also uses environment
    let response = send_jsonrpc_request(&socket, "trust.get_lineage", None, 2)
        .await
        .unwrap();
    
    assert_eq!(response["result"]["family"], "test-family");
    assert_eq!(response["result"]["node"], "test-node");
}

// ========================================================================
// E2E: Error Handling
// ========================================================================

#[tokio::test]
async fn e2e_graceful_error_handling() {
    let (_server, _handle, socket) = create_test_server("nat0", "tower1").await;
    
    // Test unknown method
    let response = send_jsonrpc_request(&socket, "unknown.method", None, 1)
        .await
        .unwrap();
    assert!(response["error"].is_object());
    assert!(response["error"]["message"].as_str().unwrap().contains("Method not found"));
    
    // Test missing required parameters
    let response = send_jsonrpc_request(
        &socket,
        "trust.evaluate_peer",
        Some(json!({})),
        2
    ).await.unwrap();
    assert!(response["error"].is_object());
    
    // Test invalid JSON-RPC version
    let mut stream = UnixStream::connect(&socket).await.unwrap();
    let invalid_request = json!({
        "jsonrpc": "1.0",
        "method": "health.check",
        "id": 1
    });
    stream.write_all(format!("{}\n", invalid_request).as_bytes()).await.unwrap();
    let mut response = String::new();
    stream.read_to_string(&mut response).await.unwrap();
    let response: serde_json::Value = serde_json::from_str(&response).unwrap();
    assert!(response["error"].is_object());
}

// ========================================================================
// E2E: Complete Federation Lifecycle
// ========================================================================

#[tokio::test]
async fn e2e_complete_federation_lifecycle() {
    // Setup: Three towers, two in same family, one outside
    let (_server1, _handle1, socket1) = create_test_server("nat0", "tower1").await;
    let (_server2, _handle2, socket2) = create_test_server("nat0", "tower2").await;
    let (_server3, _handle3, socket3) = create_test_server("other-family", "tower3").await;
    
    // Phase 1: Initial discovery
    for socket in &[&socket1, &socket2, &socket3] {
        let response = send_jsonrpc_request(socket, "health.check", None, 1)
            .await
            .unwrap();
        assert_eq!(response["result"]["status"], "healthy");
    }
    
    // Phase 2: Identity exchange
    let id1 = send_jsonrpc_request(&socket1, "identity", None, 2).await.unwrap();
    let id2 = send_jsonrpc_request(&socket2, "identity", None, 2).await.unwrap();
    let id3 = send_jsonrpc_request(&socket3, "identity", None, 2).await.unwrap();
    
    assert_eq!(id1["result"]["family"], "nat0");
    assert_eq!(id2["result"]["family"], "nat0");
    assert_eq!(id3["result"]["family"], "other-family");
    
    // Phase 3: Trust evaluation
    // Tower1 evaluates Tower2 (same family - should trust)
    let trust = send_jsonrpc_request(
        &socket1,
        "trust.evaluate_peer",
        Some(json!({"peer_id": "tower2", "peer_family": "nat0"})),
        3
    ).await.unwrap();
    assert_eq!(trust["result"]["trust_level"], 1);
    
    // Tower1 evaluates Tower3 (different family - should not trust)
    let trust = send_jsonrpc_request(
        &socket1,
        "trust.evaluate_peer",
        Some(json!({"peer_id": "tower3", "peer_family": "other-family"})),
        4
    ).await.unwrap();
    assert_eq!(trust["result"]["trust_level"], 0);
    
    // Phase 4: Capability discovery
    let caps = send_jsonrpc_request(&socket1, "capabilities", None, 5).await.unwrap();
    assert!(caps["result"]["provided_capabilities"].is_array());
}

