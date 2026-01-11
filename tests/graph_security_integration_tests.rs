//! Integration tests for Collaborative Intelligence graph security via Unix socket IPC
//!
//! Tests the complete flow from JSON-RPC request → Unix socket → graph security → response

use serde_json::json;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

/// Test helper: Create a test BTSP provider
async fn create_test_btsp_provider() -> Arc<beardog_tunnel::btsp_provider::BeardogBtspProvider> {
    use beardog_tunnel::tunnel::hsm::manager::HsmManager;
    use beardog_genetics::EcosystemGeneticEngine;
    
    // Auto-initialize HSM
    let hsm = Arc::new(HsmManager::auto_initialize().await.expect("HSM init"));
    
    // Initialize genetic engine
    let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Genetics init"));
    
    // Create BTSP provider
    Arc::new(
        beardog_tunnel::btsp_provider::BeardogBtspProvider::new(hsm, genetics)
            .await
            .expect("BTSP provider init"),
    )
}

/// Test helper: Send JSON-RPC request and receive response
async fn send_jsonrpc_request(
    method: &str,
    params: serde_json::Value,
    socket_path: &str,
) -> serde_json::Value {
    let mut stream = UnixStream::connect(socket_path).await.unwrap();
    
    let request = json!({
        "jsonrpc": "2.0",
        "method": method,
        "params": params,
        "id": 1,
    });
    
    // Send request
    let request_str = serde_json::to_string(&request).unwrap();
    eprintln!("Sending request: {}", request_str);
    stream.write_all(request_str.as_bytes()).await.unwrap();
    stream.write_all(b"\n").await.unwrap();
    stream.flush().await.unwrap();
    
    // Receive response
    let mut reader = BufReader::new(&mut stream);
    let mut response_line = String::new();
    reader.read_line(&mut response_line).await.unwrap();
    
    serde_json::from_str(&response_line).unwrap()
}

#[tokio::test]
async fn test_graph_authorize_modification_via_unix_socket() {
    // Create server
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-graph-authorize.sock";
    
    // Clean up any existing socket
    let _ = std::fs::remove_file(socket_path);
    
    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(socket_path, btsp)
            .await
            .expect("Server creation"),
    );
    
    // Get readiness flag before moving server
    let ready_flag = server.readiness_flag();
    
    // Start server in background
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });
    
    // Wait for readiness
    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready, "Server should be ready");
    
    // Test graph.authorize_modification
    let params = json!({
        "user_id": "alice",
        "graph": {
            "id": "graph-1",
            "owner": "alice",
            "nodes": [],
            "edges": [],
            "metadata": {}
        },
        "modification": {
            "action": "add_node",
            "node": {
                "id": "node-1",
                "type": "compute",
                "primal": "ToadStool",
                "config": {}
            }
        }
    });
    
    let response = send_jsonrpc_request("graph.authorize_modification", params, socket_path).await;
    
    // Verify response
    eprintln!("Response: {}", serde_json::to_string_pretty(&response).unwrap());
    assert_eq!(response["jsonrpc"], "2.0");
    
    if response["error"].is_object() {
        eprintln!("Error: {}", response["error"]);
        panic!("Got error response");
    }
    
    assert!(response["result"].is_object(), "Should have result");
    assert!(response["result"]["authorized"].is_boolean(), "Should have authorized field");
    
    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_graph_validate_template_via_unix_socket() {
    // Create server
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-graph-validate.sock";
    
    // Clean up any existing socket
    let _ = std::fs::remove_file(socket_path);
    
    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(socket_path, btsp)
            .await
            .expect("Server creation"),
    );
    
    // Get readiness flag before moving server
    let ready_flag = server.readiness_flag();
    
    // Start server in background
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });
    
    // Wait for readiness
    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready, "Server should be ready");
    
    // Test graph.validate_template
    let params = json!({
        "template": {
            "id": "template-1",
            "name": "Test Template",
            "creator": "alice",
            "nodes": [{
                "id": "node-1",
                "type": "compute",
                "primal": "ToadStool",
                "config": {}
            }],
            "edges": [],
            "metadata": {
                "version": "1.0",
                "created_at": "2026-01-11T00:00:00Z"
            }
        }
    });
    
    let response = send_jsonrpc_request("graph.validate_template", params, socket_path).await;
    
    // Verify response
    eprintln!("Response: {}", serde_json::to_string_pretty(&response).unwrap());
    assert_eq!(response["jsonrpc"], "2.0");
    
    if response["error"].is_object() {
        eprintln!("Error: {}", response["error"]);
        panic!("Got error response");
    }
    
    assert!(response["result"].is_object(), "Should have result");
    assert!(response["result"]["valid"].is_boolean(), "Should have valid field");
    
    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_graph_audit_origin_via_unix_socket() {
    // Create server
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-graph-audit.sock";
    
    // Clean up any existing socket
    let _ = std::fs::remove_file(socket_path);
    
    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(socket_path, btsp)
            .await
            .expect("Server creation"),
    );
    
    // Get readiness flag before moving server
    let ready_flag = server.readiness_flag();
    
    // Start server in background
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });
    
    // Wait for readiness
    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready, "Server should be ready");
    
    // Test graph.audit_origin
    let params = json!({
        "template_id": "template-123"
    });
    
    let response = send_jsonrpc_request("graph.audit_origin", params, socket_path).await;
    
    // Verify response
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"].is_object(), "Should have result");
    assert!(response["result"]["chain_valid"].is_boolean(), "Should have chain_valid field");
    assert!(response["result"]["trust_score"].is_number(), "Should have trust_score field");
    
    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_graph_capabilities_advertised() {
    // Create server
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-graph-caps.sock";
    
    // Clean up any existing socket
    let _ = std::fs::remove_file(socket_path);
    
    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(socket_path, btsp)
            .await
            .expect("Server creation"),
    );
    
    // Get readiness flag before moving server
    let ready_flag = server.readiness_flag();
    
    // Start server in background
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });
    
    // Wait for readiness
    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready, "Server should be ready");
    
    // Test capabilities
    let response = send_jsonrpc_request("capabilities", json!({}), socket_path).await;
    
    // Verify response
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["provided_capabilities"].is_array(), "Should have capabilities array");
    
    // Check for graph security capability
    let capabilities = response["result"]["provided_capabilities"].as_array().unwrap();
    let graph_cap = capabilities.iter().find(|c| c["type"] == "graph");
    assert!(graph_cap.is_some(), "Should advertise graph capability");
    
    let graph_methods = graph_cap.unwrap()["methods"].as_array().unwrap();
    assert!(graph_methods.contains(&json!("authorize_modification")));
    assert!(graph_methods.contains(&json!("validate_template")));
    assert!(graph_methods.contains(&json!("audit_origin")));
    
    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

