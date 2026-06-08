// SPDX-License-Identifier: AGPL-3.0-or-later

//! Peer-role scenario tests: UI, storage, and compute peer integration via graph security RPC.

use super::helpers::*;
use serde_json::json;

// ── UI peer ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn test_ui_peer_user_modifies_graph() {
    let (socket_path, server_task, _guard) = start_test_server().await;

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

    let response = send_jsonrpc_request("graph.authorize_modification", params, &socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["authorized"].is_boolean());

    server_task.abort();
}

#[tokio::test]
async fn test_ui_peer_live_graph_visualization() {
    let (socket_path, server_task, _guard) = start_test_server().await;

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

    let response = send_jsonrpc_request("graph.validate_template", params, &socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["valid"].is_boolean());
    assert!(response["result"]["security_score"].is_number());

    server_task.abort();
}

#[tokio::test]
async fn test_ui_peer_template_browser() {
    let (socket_path, server_task, _guard) = start_test_server().await;

    let params = json!({ "template_id": "community-template-42" });
    let response = send_jsonrpc_request("graph.audit_origin", params, &socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["trust_score"].is_number());
    assert!(response["result"]["chain_valid"].is_boolean());

    server_task.abort();
}

// ── Storage peer ─────────────────────────────────────────────────────────

#[tokio::test]
async fn test_storage_peer_template_storage() {
    let (socket_path, server_task, _guard) = start_test_server().await;

    let params = json!({
        "template": {
            "id": "storage-template-1", "name": "Production Workflow", "creator": "bob",
            "nodes": [{ "id": "node-1", "type": "storage", "primal": "storage-peer", "config": { "capacity": "100GB" } }],
            "edges": [],
            "metadata": { "version": "1.0", "created_at": "2026-01-11T12:00:00Z", "production": true }
        }
    });

    let response = send_jsonrpc_request("graph.validate_template", params, &socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["valid"].is_boolean());

    server_task.abort();
}

#[tokio::test]
async fn test_storage_peer_template_retrieval() {
    let (socket_path, server_task, _guard) = start_test_server().await;

    let params = json!({ "template_id": "stored-template-99" });
    let response = send_jsonrpc_request("graph.audit_origin", params, &socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["trust_score"].is_number());

    server_task.abort();
}

#[tokio::test]
async fn test_storage_peer_version_control() {
    let (socket_path, server_task, _guard) = start_test_server().await;

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

    let response = send_jsonrpc_request("graph.authorize_modification", params, &socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["authorized"].is_boolean());

    server_task.abort();
}

// ── Compute peer ─────────────────────────────────────────────────────────

#[tokio::test]
async fn test_compute_peer_ai_suggests_modification() {
    let (socket_path, server_task, _guard) = start_test_server().await;

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

    let response = send_jsonrpc_request("graph.authorize_modification", params, &socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["authorized"].is_boolean());
    assert!(response["result"]["reasoning"].is_string());

    server_task.abort();
}

#[tokio::test]
async fn test_compute_peer_learns_from_patterns() {
    let (socket_path, server_task, _guard) = start_test_server().await;

    let params = json!({ "template_id": "high-performing-template" });
    let response = send_jsonrpc_request("graph.audit_origin", params, &socket_path).await;
    assert_eq!(response["jsonrpc"], "2.0");
    assert!(response["result"]["trust_score"].is_number());
    assert!(response["result"]["community_usage"].is_object());

    server_task.abort();
}
