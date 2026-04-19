// SPDX-License-Identifier: AGPL-3.0-or-later

//! Peer-role scenario tests: UI, storage, and compute peer integration via graph security RPC.

use super::helpers::*;
use beardog_core::socket_config::IpcCapabilitySymlinksConfig;
use beardog_tunnel::btsp_handshake::BtspSecurityMode;
use beardog_types::primal_identity::PrimalIdentity;
use serde_json::json;
use std::sync::Arc;

// ── UI peer ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_ui_peer_user_modifies_graph() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-ui-peer-modify.sock";
    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
            BtspSecurityMode::Development,
            IpcCapabilitySymlinksConfig::default(),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move { server_clone.start().await.unwrap() });
    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    let params = json!({
        "user_id": "alice",
        "graph": {
            "id": "user-workflow-1", "owner": "alice",
            "nodes": [{ "id": "existing-node", "type": "compute", "primal": "compute.general", "config": {} }],
            "edges": [], "metadata": { "created_by": "ui-peer", "version": "1.0" }
        },
        "modification": {
            "action": "add_node",
            "node": { "id": "new-ai-node", "type": "ai", "primal": "compute-peer", "config": { "model": "gpt-4", "temperature": 0.7 } }
        }
    });

    let response = send_jsonrpc_request("graph.authorize_modification", params, socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["authorized"].is_boolean());

    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_ui_peer_live_graph_visualization() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-ui-peer-viz.sock";
    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
            BtspSecurityMode::Development,
            IpcCapabilitySymlinksConfig::default(),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move { server_clone.start().await.unwrap() });
    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    let params = json!({
        "template": {
            "id": "wip-template-1", "name": "User Workflow", "creator": "alice",
            "nodes": [
                { "id": "input-node", "type": "data", "primal": "compute-peer", "config": {} },
                { "id": "process-node", "type": "compute", "primal": "compute.general", "config": {} }
            ],
            "edges": [{ "from": "input-node", "to": "process-node" }],
            "metadata": { "version": "0.1", "created_at": "2026-01-11T12:00:00Z" }
        }
    });

    let response = send_jsonrpc_request("graph.validate_template", params, socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["valid"].is_boolean());
    assert!(response["result"]["security_score"].is_number());

    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_ui_peer_template_browser() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-ui-peer-browse.sock";
    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
            BtspSecurityMode::Development,
            IpcCapabilitySymlinksConfig::default(),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move { server_clone.start().await.unwrap() });
    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    let params = json!({ "template_id": "community-template-42" });
    let response = send_jsonrpc_request("graph.audit_origin", params, socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["trust_score"].is_number());
    assert!(response["result"]["chain_valid"].is_boolean());

    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

// ── Storage peer ─────────────────────────────────────────────────────────

#[tokio::test]
async fn test_storage_peer_template_storage() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-storage-peer-store.sock";
    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
            BtspSecurityMode::Development,
            IpcCapabilitySymlinksConfig::default(),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move { server_clone.start().await.unwrap() });
    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    let params = json!({
        "template": {
            "id": "storage-template-1", "name": "Production Workflow", "creator": "bob",
            "nodes": [{ "id": "node-1", "type": "storage", "primal": "storage-peer", "config": { "capacity": "100GB" } }],
            "edges": [],
            "metadata": { "version": "1.0", "created_at": "2026-01-11T12:00:00Z", "production": true }
        }
    });

    let response = send_jsonrpc_request("graph.validate_template", params, socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["valid"].is_boolean());

    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_storage_peer_template_retrieval() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-storage-peer-retrieve.sock";
    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
            BtspSecurityMode::Development,
            IpcCapabilitySymlinksConfig::default(),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move { server_clone.start().await.unwrap() });
    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    let params = json!({ "template_id": "stored-template-99" });
    let response = send_jsonrpc_request("graph.audit_origin", params, socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["trust_score"].is_number());

    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_storage_peer_version_control() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-storage-peer-version.sock";
    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
            BtspSecurityMode::Development,
            IpcCapabilitySymlinksConfig::default(),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move { server_clone.start().await.unwrap() });
    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    let params = json!({
        "user_id": "bob",
        "graph": {
            "id": "versioned-graph-1", "owner": "bob",
            "nodes": [{ "id": "v1-node", "type": "compute", "primal": "compute.general", "config": {"version": "1.0"} }],
            "edges": [], "metadata": { "version": "1.0" }
        },
        "modification": {
            "action": "add_node",
            "node": { "id": "v2-node", "type": "compute", "primal": "compute.general", "config": {"version": "2.0"} }
        }
    });

    let response = send_jsonrpc_request("graph.authorize_modification", params, socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["authorized"].is_boolean());

    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

// ── Compute peer ─────────────────────────────────────────────────────────

#[tokio::test]
async fn test_compute_peer_ai_suggests_modification() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-compute-peer-suggest.sock";
    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
            BtspSecurityMode::Development,
            IpcCapabilitySymlinksConfig::default(),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move { server_clone.start().await.unwrap() });
    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    let params = json!({
        "user_id": "ai-agent-compute",
        "graph": {
            "id": "ai-optimized-graph", "owner": "alice",
            "nodes": [{ "id": "slow-node", "type": "compute", "primal": "compute.general", "config": {"cpu": "1"} }],
            "edges": [], "metadata": { "ai_suggestions_enabled": true }
        },
        "modification": {
            "action": "add_node",
            "node": { "id": "cache-node", "type": "cache", "primal": "compute-peer", "config": { "strategy": "lru", "suggested_by": "ai" } }
        }
    });

    let response = send_jsonrpc_request("graph.authorize_modification", params, socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["authorized"].is_boolean());
    assert!(response["result"]["reasoning"].is_string());

    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_compute_peer_learns_from_patterns() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-compute-peer-learn.sock";
    let _ = std::fs::remove_file(socket_path);

    let server = Arc::new(
        beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::new(
            socket_path,
            btsp,
            Arc::new(PrimalIdentity::for_test("test-family", "test-node")),
            BtspSecurityMode::Development,
            IpcCapabilitySymlinksConfig::default(),
        )
        .await
        .expect("Server creation"),
    );

    let ready_flag = server.readiness_flag();
    let server_clone = Arc::clone(&server);
    let server_task = tokio::spawn(async move { server_clone.start().await.unwrap() });
    let ready = beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer::wait_ready_flag(
        &ready_flag,
        std::time::Duration::from_secs(5),
    )
    .await;
    assert!(ready);

    let params = json!({ "template_id": "high-performing-template" });
    let response = send_jsonrpc_request("graph.audit_origin", params, socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["trust_score"].is_number());
    assert!(response["result"]["community_usage"].is_object());

    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}
