// SPDX-License-Identifier: AGPL-3.0-or-later

//! Core graph RPC tests: authorize_modification, validate_template, audit_origin, capabilities.

use super::helpers::*;
use beardog_core::socket_config::IpcCapabilitySymlinksConfig;
use beardog_tunnel::btsp_handshake::BtspSecurityMode;
use beardog_types::primal_identity::PrimalIdentity;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn test_graph_authorize_modification_via_unix_socket() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-graph-authorize.sock";
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
    assert!(ready, "Server should be ready");

    let params = json!({
        "user_id": "alice",
        "graph": {
            "id": "graph-1", "owner": "alice", "nodes": [], "edges": [], "metadata": {}
        },
        "modification": {
            "action": "add_node",
            "node": { "id": "node-1", "type": "compute", "primal": "compute.general", "config": {} }
        }
    });

    let response = send_jsonrpc_request("graph.authorize_modification", params, socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(
        !response["error"].is_object(),
        "Expected success but got error: {}",
        response["error"]
    );
    assert!(response["result"].is_object());
    assert!(response["result"]["authorized"].is_boolean());

    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_graph_validate_template_via_unix_socket() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-graph-validate.sock";
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
            "id": "template-1", "name": "Test Template", "creator": "alice",
            "nodes": [{ "id": "node-1", "type": "compute", "primal": "compute.general", "config": {} }],
            "edges": [],
            "metadata": { "version": "1.0", "created_at": "2026-01-11T00:00:00Z" }
        }
    });

    let response = send_jsonrpc_request("graph.validate_template", params, socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(
        !response["error"].is_object(),
        "Expected success but got error: {}",
        response["error"]
    );
    assert!(response["result"]["valid"].is_boolean());

    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_graph_audit_origin_via_unix_socket() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-graph-audit.sock";
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

    let params = json!({ "template_id": "template-123" });
    let response = send_jsonrpc_request("graph.audit_origin", params, socket_path).await;

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["chain_valid"].is_boolean());
    assert!(response["result"]["trust_score"].is_number());

    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}

#[tokio::test]
async fn test_graph_capabilities_advertised() {
    let btsp = create_test_btsp_provider().await;
    let socket_path = "/tmp/beardog-test-graph-caps.sock";
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

    let response = send_jsonrpc_request("capabilities", json!({}), socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["provided_capabilities"].is_array());

    let capabilities = response["result"]["provided_capabilities"]
        .as_array()
        .unwrap();
    let graph_cap = capabilities.iter().find(|c| c["type"] == "graph");
    assert!(graph_cap.is_some());

    let graph_methods = graph_cap.unwrap()["methods"].as_array().unwrap();
    assert!(graph_methods.contains(&json!("authorize_modification")));
    assert!(graph_methods.contains(&json!("validate_template")));
    assert!(graph_methods.contains(&json!("audit_origin")));

    server_task.abort();
    let _ = std::fs::remove_file(socket_path);
}
