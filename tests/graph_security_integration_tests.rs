//! Integration tests for Collaborative Intelligence graph security via Unix socket IPC
//!
//! Tests the complete flow from JSON-RPC request → Unix socket → graph security → response

use beardog_types::primal_identity::PrimalIdentity;
use serde_json::json;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

/// Test helper: Create a test BTSP provider
async fn create_test_btsp_provider() -> Arc<beardog_tunnel::btsp_provider::BeardogBtspProvider> {
    use beardog_genetics::EcosystemGeneticEngine;
    use beardog_tunnel::tunnel::hsm::manager::HsmManager;

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
    eprintln!("Sending request: {request_str}");
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
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
        )
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
    eprintln!(
        "Response: {}",
        serde_json::to_string_pretty(&response).unwrap()
    );
    assert_eq!(response["jsonrpc"], "2.0");

    if response["error"].is_object() {
        eprintln!("Error: {}", response["error"]);
        panic!("Got error response");
    }

    assert!(response["result"].is_object(), "Should have result");
    assert!(
        response["result"]["authorized"].is_boolean(),
        "Should have authorized field"
    );

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
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
        )
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
    eprintln!(
        "Response: {}",
        serde_json::to_string_pretty(&response).unwrap()
    );
    assert_eq!(response["jsonrpc"], "2.0");

    if response["error"].is_object() {
        eprintln!("Error: {}", response["error"]);
        panic!("Got error response");
    }

    assert!(response["result"].is_object(), "Should have result");
    assert!(
        response["result"]["valid"].is_boolean(),
        "Should have valid field"
    );

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
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
        )
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
    assert!(
        response["result"]["chain_valid"].is_boolean(),
        "Should have chain_valid field"
    );
    assert!(
        response["result"]["trust_score"].is_number(),
        "Should have trust_score field"
    );

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
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
        )
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
    assert!(
        response["result"]["provided_capabilities"].is_array(),
        "Should have capabilities array"
    );

    // Check for graph security capability
    let capabilities = response["result"]["provided_capabilities"]
        .as_array()
        .unwrap();
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

// ============================================================================
// petalTongue Integration Tests (User Interface)
// ============================================================================

#[tokio::test]
async fn test_petaltongue_user_modifies_graph() {
    // Simulate petalTongue sending user modification request
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-petaltongue-modify.sock";

    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    // User wants to add a node via petalTongue UI
    let params = json!({
        "user_id": "alice",
        "graph": {
            "id": "user-workflow-1",
            "owner": "alice",
            "nodes": [{
                "id": "existing-node",
                "type": "compute",
                "primal": "ToadStool",
                "config": {}
            }],
            "edges": [],
            "metadata": {
                "created_by": "petalTongue",
                "version": "1.0"
            }
        },
        "modification": {
            "action": "add_node",
            "node": {
                "id": "new-ai-node",
                "type": "ai",
                "primal": "Squirrel",
                "config": {
                    "model": "gpt-4",
                    "temperature": 0.7
                }
            }
        }
    });

    let response = send_jsonrpc_request("graph.authorize_modification", params, socket_path).await;

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["authorized"].is_boolean());

    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_petaltongue_live_graph_visualization() {
    // petalTongue requests validation of template being edited
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-petaltongue-viz.sock";

    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    // Validate work-in-progress template
    let params = json!({
        "template": {
            "id": "wip-template-1",
            "name": "User Workflow",
            "creator": "alice",
            "nodes": [
                {
                    "id": "input-node",
                    "type": "data",
                    "primal": "Squirrel",
                    "config": {}
                },
                {
                    "id": "process-node",
                    "type": "compute",
                    "primal": "ToadStool",
                    "config": {}
                }
            ],
            "edges": [{
                "from": "input-node",
                "to": "process-node"
            }],
            "metadata": {
                "version": "0.1",
                "created_at": "2026-01-11T12:00:00Z"
            }
        }
    });

    let response = send_jsonrpc_request("graph.validate_template", params, socket_path).await;

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["valid"].is_boolean());
    assert!(response["result"]["security_score"].is_number());

    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_petaltongue_template_browser() {
    // User browses community templates, petalTongue checks trust
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-petaltongue-browse.sock";

    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    // Audit community template before showing to user
    let params = json!({
        "template_id": "community-template-42"
    });

    let response = send_jsonrpc_request("graph.audit_origin", params, socket_path).await;

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["trust_score"].is_number());
    assert!(response["result"]["chain_valid"].is_boolean());

    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

// ============================================================================
// NestGate Integration Tests (Storage/Persistence)
// ============================================================================

#[tokio::test]
async fn test_nestgate_template_storage() {
    // NestGate validates template before storing
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-nestgate-store.sock";

    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    // Validate before storing
    let params = json!({
        "template": {
            "id": "storage-template-1",
            "name": "Production Workflow",
            "creator": "bob",
            "nodes": [{
                "id": "node-1",
                "type": "storage",
                "primal": "NestGate",
                "config": {
                    "capacity": "100GB"
                }
            }],
            "edges": [],
            "metadata": {
                "version": "1.0",
                "created_at": "2026-01-11T12:00:00Z",
                "production": true
            }
        }
    });

    let response = send_jsonrpc_request("graph.validate_template", params, socket_path).await;

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["valid"].is_boolean());

    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_nestgate_template_retrieval() {
    // NestGate provides trust info when retrieving template
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-nestgate-retrieve.sock";

    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    // Audit stored template
    let params = json!({
        "template_id": "stored-template-99"
    });

    let response = send_jsonrpc_request("graph.audit_origin", params, socket_path).await;

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["trust_score"].is_number());

    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_nestgate_version_control() {
    // NestGate validates template modifications for version control
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-nestgate-version.sock";

    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    // Authorize modification for new version
    let params = json!({
        "user_id": "bob",
        "graph": {
            "id": "versioned-graph-1",
            "owner": "bob",
            "nodes": [{
                "id": "v1-node",
                "type": "compute",
                "primal": "ToadStool",
                "config": {"version": "1.0"}
            }],
            "edges": [],
            "metadata": {
                "version": "1.0"
            }
        },
        "modification": {
            "action": "add_node",
            "node": {
                "id": "v2-node",
                "type": "compute",
                "primal": "ToadStool",
                "config": {"version": "2.0"}
            }
        }
    });

    let response = send_jsonrpc_request("graph.authorize_modification", params, socket_path).await;

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["authorized"].is_boolean());

    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

// ============================================================================
// Squirrel Integration Tests (AI/Intelligence)
// ============================================================================

#[tokio::test]
async fn test_squirrel_ai_suggests_modification() {
    // Squirrel (AI) suggests graph modification, BearDog authorizes
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-squirrel-suggest.sock";

    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    // AI suggests adding optimization node
    let params = json!({
        "user_id": "ai-agent-squirrel",
        "graph": {
            "id": "ai-optimized-graph",
            "owner": "alice",
            "nodes": [{
                "id": "slow-node",
                "type": "compute",
                "primal": "ToadStool",
                "config": {"cpu": "1"}
            }],
            "edges": [],
            "metadata": {
                "ai_suggestions_enabled": true
            }
        },
        "modification": {
            "action": "add_node",
            "node": {
                "id": "cache-node",
                "type": "cache",
                "primal": "Squirrel",
                "config": {
                    "strategy": "lru",
                    "suggested_by": "ai"
                }
            }
        }
    });

    let response = send_jsonrpc_request("graph.authorize_modification", params, socket_path).await;

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["authorized"].is_boolean());
    // AI modifications should have reasoning
    assert!(response["result"]["reasoning"].is_string());

    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_squirrel_learns_from_patterns() {
    // Squirrel analyzes successful templates for patterns
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-squirrel-learn.sock";

    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move {
        server_clone.start().await.unwrap();
    });

    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    // Audit successful template for learning
    let params = json!({
        "template_id": "high-performing-template"
    });

    let response = send_jsonrpc_request("graph.audit_origin", params, socket_path).await;

    assert_eq!(response["jsonrpc"], "2.0");
    // High trust score indicates good pattern
    assert!(response["result"]["trust_score"].is_number());
    // Community usage indicates success
    assert!(response["result"]["community_usage"].is_object());

    // Cleanup
    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}
