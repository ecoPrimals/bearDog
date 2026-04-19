// SPDX-License-Identifier: AGPL-3.0-or-later

//! IPC / JSON-RPC fault injection tests.

use beardog_ipc::protocol::error_codes;
use serde_json::json;
use std::panic;
use std::time::Duration;
use tokio::net::UnixStream;

fn handle_jsonrpc_request_line_for_fault(line: &str) -> Option<String> {
    const MAX_JSONRPC_LINE_BYTES: usize = 256 * 1024;

    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.len() > MAX_JSONRPC_LINE_BYTES {
        return Some(
            json!({
                "jsonrpc": "2.0",
                "error": { "code": error_codes::INTERNAL_ERROR, "message": "Message too large" },
                "id": null
            })
            .to_string(),
        );
    }

    let v: serde_json::Value = match serde_json::from_str(trimmed) {
        Ok(v) => v,
        Err(_) => {
            return Some(
                json!({
                    "jsonrpc": "2.0",
                    "error": { "code": error_codes::PARSE_ERROR, "message": "Parse error" },
                    "id": null
                })
                .to_string(),
            );
        }
    };

    let Some(obj) = v.as_object() else {
        return Some(
            json!({
                "jsonrpc": "2.0",
                "error": { "code": error_codes::INVALID_REQUEST, "message": "Invalid Request" },
                "id": null
            })
            .to_string(),
        );
    };

    if obj.get("jsonrpc") != Some(&json!("2.0")) {
        return Some(
            json!({
                "jsonrpc": "2.0",
                "error": { "code": error_codes::INVALID_REQUEST, "message": "Invalid Request" },
                "id": obj.get("id").cloned().unwrap_or(serde_json::Value::Null)
            })
            .to_string(),
        );
    }

    if !obj.contains_key("id") {
        return None;
    }

    let id = obj.get("id").cloned().unwrap_or(serde_json::Value::Null);
    let method = obj.get("method").and_then(|m| m.as_str()).unwrap_or("");

    if method.is_empty() {
        return Some(
            json!({
                "jsonrpc": "2.0",
                "error": { "code": error_codes::INVALID_REQUEST, "message": "Invalid Request" },
                "id": id
            })
            .to_string(),
        );
    }

    Some(
        json!({
            "jsonrpc": "2.0",
            "error": { "code": error_codes::METHOD_NOT_FOUND, "message": "Method not found" },
            "id": id
        })
        .to_string(),
    )
}

#[tokio::test]
async fn ipc_fault_nonexistent_unix_socket_fails_without_hanging() {
    let dir = tempfile::tempdir().expect("temp directory for bogus socket");
    let socket_path = dir.path().join("nonexistent.sock");
    let connect = UnixStream::connect(&socket_path);
    let outcome = tokio::time::timeout(Duration::from_secs(2), connect)
        .await
        .expect("connect should complete within timeout");
    assert!(
        outcome.is_err(),
        "connection to missing socket should fail with error"
    );
}

#[tokio::test]
async fn ipc_fault_malformed_jsonrpc_returns_parse_error_response() {
    let line = "not-json-at-all{{{";
    let response = handle_jsonrpc_request_line_for_fault(line).expect("response for bad line");
    let v: serde_json::Value = serde_json::from_str(&response).expect("response is JSON");
    assert_eq!(v["error"]["code"], error_codes::PARSE_ERROR);
}

#[tokio::test]
async fn ipc_fault_invalid_jsonrpc_version_returns_invalid_request() {
    let line = r#"{"jsonrpc":"1.0","method":"x","id":1}"#;
    let response = handle_jsonrpc_request_line_for_fault(line).expect("response");
    let v: serde_json::Value = serde_json::from_str(&response).expect("response is JSON");
    assert_eq!(v["error"]["code"], error_codes::INVALID_REQUEST);
}

#[tokio::test]
async fn ipc_fault_oversized_jsonrpc_line_rejected() {
    let padding = "x".repeat(300_000);
    let line = format!(r#"{{"jsonrpc":"2.0","method":"x","id":1,"p":"{padding}"}}"#);
    let response = handle_jsonrpc_request_line_for_fault(&line).expect("oversized response");
    let v: serde_json::Value = serde_json::from_str(&response).expect("response is JSON");
    assert_eq!(v["error"]["code"], error_codes::INTERNAL_ERROR);
}

#[tokio::test]
async fn ipc_fault_large_invalid_json_does_not_panic() {
    let line = "a".repeat(500_000);
    let result = panic::catch_unwind(|| serde_json::from_str::<serde_json::Value>(&line));
    assert!(
        result.is_ok(),
        "serde_json should not panic on invalid input"
    );
    assert!(result.expect("no panic").is_err());
}
