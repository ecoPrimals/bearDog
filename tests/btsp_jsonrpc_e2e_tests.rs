//! BTSP JSON-RPC End-to-End Tests
//!
//! These tests validate the complete BTSP JSON-RPC flow including:
//! - Full request/response cycles
//! - Multiple concurrent requests
//! - Real-world usage patterns
//! - Integration with underlying BTSP provider

use beardog_tunnel::{
    btsp_provider::BeardogBtspProvider,
    tunnel::hsm::manager::HsmManager,
    unix_socket_ipc::UnixSocketIpcServer,
};
use beardog_genetics::{
    birdsong::BirdSongManager,
    ecosystem_evolution::EcosystemGeneticEngine,
};
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

/// Create test BTSP provider for E2E tests
async fn create_e2e_btsp_provider() -> Arc<BeardogBtspProvider> {
    let hsm = Arc::new(HsmManager::new());
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Failed to create genetics"));
    
    let provider = BeardogBtspProvider::new(hsm, genetics)
        .await
        .expect("Failed to create BTSP provider");
    
    Arc::new(provider)
}

/// Create test server for E2E tests
async fn create_e2e_server(socket_path: PathBuf) -> Arc<UnixSocketIpcServer> {
    let provider = create_e2e_btsp_provider().await;
    let server = UnixSocketIpcServer::new(socket_path, provider)
        .await
        .expect("Failed to create server");
    Arc::new(server)
}

#[tokio::test]
async fn test_e2e_complete_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("e2e-workflow.sock");
    let server = create_e2e_server(socket_path).await;

    // Step 1: Check capabilities
    let caps_request = json!({
        "jsonrpc": "2.0",
        "method": "capabilities",
        "id": 1
    });

    let caps_response = server
        .handle_jsonrpc_request(&caps_request.to_string())
        .await
        .expect("Should get capabilities");

    assert!(caps_response.error.is_none());
    assert!(caps_response.result.is_some());

    // Step 2: Attempt contact exchange
    let contact_request = json!({
        "jsonrpc": "2.0",
        "method": "beardog./btsp/contact/exchange",
        "params": {
            "target_peer_id": "test_peer",
            "requester_lineage": "test_requester",
            "max_hops": 3
        },
        "id": 2
    });

    let contact_response = server
        .handle_jsonrpc_request(&contact_request.to_string())
        .await
        .expect("Should handle contact exchange");

    // Should recognize method even if lineage data doesn't exist
    if let Some(error) = contact_response.error {
        assert!(!error.message.contains("Method not found"));
    }
}

#[tokio::test]
async fn test_e2e_multiple_concurrent_requests() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("e2e-concurrent.sock");
    let server = create_e2e_server(socket_path).await;

    // Send multiple requests concurrently
    let server1 = Arc::clone(&server);
    let server2 = Arc::clone(&server);
    let server3 = Arc::clone(&server);

    let handle1 = tokio::spawn(async move {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.contact_exchange",
            "params": {
                "target_peer_id": "peer1",
                "requester_lineage": "req1"
            },
            "id": 1
        });
        server1.handle_jsonrpc_request(&request.to_string()).await
    });

    let handle2 = tokio::spawn(async move {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.contact_exchange",
            "params": {
                "target_peer_id": "peer2",
                "requester_lineage": "req2"
            },
            "id": 2
        });
        server2.handle_jsonrpc_request(&request.to_string()).await
    });

    let handle3 = tokio::spawn(async move {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "capabilities",
            "id": 3
        });
        server3.handle_jsonrpc_request(&request.to_string()).await
    });

    let (result1, result2, result3) = tokio::join!(handle1, handle2, handle3);

    // All requests should be handled
    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());

    // Capabilities should always succeed
    let caps_response = result3.unwrap().unwrap();
    assert!(caps_response.error.is_none());
}

#[tokio::test]
async fn test_e2e_rapid_fire_requests() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("e2e-rapid-fire.sock");
    let server = create_e2e_server(socket_path).await;

    // Send 50 requests in rapid succession
    let mut handles = vec![];
    for i in 0..50 {
        let server_clone = Arc::clone(&server);
        let handle = tokio::spawn(async move {
            let request = json!({
                "jsonrpc": "2.0",
                "method": "btsp.contact_exchange",
                "params": {
                    "target_peer_id": format!("peer{}", i),
                    "requester_lineage": "requester"
                },
                "id": i
            });
            server_clone.handle_jsonrpc_request(&request.to_string()).await
        });
        handles.push(handle);
    }

    // Wait for all to complete
    let results = futures::future::join_all(handles).await;

    // All should complete successfully (even if with business logic errors)
    for result in results {
        assert!(result.is_ok(), "Request should complete");
        let response = result.unwrap();
        assert!(response.is_ok(), "Should get response");
    }
}

#[tokio::test]
async fn test_e2e_mixed_valid_and_invalid_requests() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("e2e-mixed.sock");
    let server = create_e2e_server(socket_path).await;

    // Valid request
    let valid_request = json!({
        "jsonrpc": "2.0",
        "method": "capabilities",
        "id": 1
    });

    // Invalid method
    let invalid_method = json!({
        "jsonrpc": "2.0",
        "method": "nonexistent.method",
        "id": 2
    });

    // Invalid JSON-RPC version
    let invalid_version = json!({
        "jsonrpc": "1.0",
        "method": "capabilities",
        "id": 3
    });

    // Missing params
    let missing_params = json!({
        "jsonrpc": "2.0",
        "method": "btsp.contact_exchange",
        "id": 4
    });

    let valid_response = server.handle_jsonrpc_request(&valid_request.to_string()).await.unwrap();
    let invalid_method_response = server.handle_jsonrpc_request(&invalid_method.to_string()).await.unwrap();
    let invalid_version_response = server.handle_jsonrpc_request(&invalid_version.to_string()).await.unwrap();
    let missing_params_response = server.handle_jsonrpc_request(&missing_params.to_string()).await.unwrap();

    // Valid request should succeed
    assert!(valid_response.error.is_none());
    assert!(valid_response.result.is_some());

    // Invalid requests should have errors
    assert!(invalid_method_response.error.is_some());
    assert!(invalid_version_response.error.is_some());
    assert!(missing_params_response.error.is_some());
}

#[tokio::test]
async fn test_e2e_all_btsp_methods_accessible() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("e2e-all-methods.sock");
    let server = create_e2e_server(socket_path).await;

    let methods = vec![
        ("beardog./btsp/contact/exchange", json!({"target_peer_id": "t", "requester_lineage": "r"})),
        ("beardog./btsp/tunnel/establish", json!({"id": "t", "address": "127.0.0.1:8080"})),
        ("beardog./btsp/tunnel/encrypt", json!({"tunnel": {"id": "t", "peer_id": "p", "established_at": "2026-01-07T00:00:00Z"}, "data": "dGVzdA=="})),
        ("beardog./btsp/tunnel/decrypt", json!({"tunnel": {"id": "t", "peer_id": "p", "established_at": "2026-01-07T00:00:00Z"}, "data": "dGVzdA=="})),
        ("beardog./btsp/tunnel/status", json!({"tunnel_id": "test"})),
        ("beardog./btsp/tunnel/close", json!({"tunnel_id": "test"})),
    ];

    for (method, params) in methods {
        let request = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        // All methods should be recognized (no "Method not found")
        if let Some(error) = response.error {
            assert!(
                !error.message.contains("Method not found"),
                "Method {} should be recognized, got error: {}",
                method,
                error.message
            );
        }
    }
}

#[tokio::test]
async fn test_e2e_request_id_preservation() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("e2e-id-preservation.sock");
    let server = create_e2e_server(socket_path).await;

    // Test with different ID types
    let test_cases = vec![
        json!(1),
        json!("string-id"),
        json!(12345),
        json!(null),
    ];

    for id in test_cases {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "capabilities",
            "id": id
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        // Response ID should match request ID
        assert_eq!(response.id, id, "Response ID should match request ID");
    }
}

#[tokio::test]
async fn test_e2e_stress_test_sustained_load() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("e2e-stress.sock");
    let server = create_e2e_server(socket_path).await;

    // Sustained load: 200 requests across 10 concurrent clients
    let mut handles = vec![];
    for client_id in 0..10 {
        let server_clone = Arc::clone(&server);
        let handle = tokio::spawn(async move {
            let mut results = vec![];
            for request_id in 0..20 {
                let request = json!({
                    "jsonrpc": "2.0",
                    "method": "capabilities",
                    "id": format!("client{}-req{}", client_id, request_id)
                });
                
                let response = server_clone
                    .handle_jsonrpc_request(&request.to_string())
                    .await;
                results.push(response);
            }
            results
        });
        handles.push(handle);
    }

    // Wait for all clients
    let all_results = futures::future::join_all(handles).await;

    // All should complete
    for client_results in all_results {
        assert!(client_results.is_ok());
        let responses = client_results.unwrap();
        assert_eq!(responses.len(), 20);
        
        // All responses should be successful
        for response in responses {
            assert!(response.is_ok());
            let resp = response.unwrap();
            assert!(resp.error.is_none());
        }
    }
}

