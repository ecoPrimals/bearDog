// SPDX-License-Identifier: AGPL-3.0-or-later

//! Core graph RPC tests: authorize_modification, validate_template, audit_origin, capabilities.

use super::helpers::*;
use serde_json::json;

#[tokio::test]
async fn test_graph_authorize_modification_via_unix_socket() {
    let (socket_path, server_task, _guard) = start_test_server().await;

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

    let response = send_jsonrpc_request("graph.authorize_modification", params, &socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(
        !response["error"].is_object(),
        "Expected success but got error: {}",
        response["error"]
    );
    assert!(response["result"].is_object());
    assert!(response["result"]["authorized"].is_boolean());

    server_task.abort();
}

#[tokio::test]
async fn test_graph_validate_template_via_unix_socket() {
    let (socket_path, server_task, _guard) = start_test_server().await;

    let params = json!({
        "template": {
            "id": "template-1", "name": "Test Template", "creator": "alice",
            "nodes": [{ "id": "node-1", "type": "compute", "primal": "compute.general", "config": {} }],
            "edges": [],
            "metadata": { "version": "1.0", "created_at": "2026-01-11T00:00:00Z" }
        }
    });

    let response = send_jsonrpc_request("graph.validate_template", params, &socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(
        !response["error"].is_object(),
        "Expected success but got error: {}",
        response["error"]
    );
    assert!(response["result"]["valid"].is_boolean());

    server_task.abort();
}

#[tokio::test]
async fn test_graph_audit_origin_via_unix_socket() {
    let (socket_path, server_task, _guard) = start_test_server().await;

    let params = json!({ "template_id": "template-123" });
    let response = send_jsonrpc_request("graph.audit_origin", params, &socket_path).await;

    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["chain_valid"].is_boolean());
    assert!(response["result"]["trust_score"].is_number());

    server_task.abort();
}

#[tokio::test]
async fn test_graph_capabilities_advertised() {
    let (socket_path, server_task, _guard) = start_test_server().await;

    let response = send_jsonrpc_request("capabilities", json!({}), &socket_path).await;
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
}
