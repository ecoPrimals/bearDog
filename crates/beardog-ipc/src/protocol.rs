// SPDX-License-Identifier: AGPL-3.0-or-later

//! JSON-RPC 2.0 protocol implementation (batch-capable, NDJSON-aware)

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;

/// JSON-RPC 2.0 request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: Cow<'static, str>,
    /// Method name
    pub method: String,
    /// Method parameters
    pub params: Value,
    /// Request ID (nullable per spec for notifications)
    pub id: u64,
}

impl JsonRpcRequest {
    /// Construct a new request with version pre-set
    #[must_use]
    pub fn new(method: impl Into<String>, params: Value, id: u64) -> Self {
        Self {
            jsonrpc: Cow::Borrowed(JSONRPC_VERSION),
            method: method.into(),
            params,
            id,
        }
    }
}

/// JSON-RPC 2.0 response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: Cow<'static, str>,
    /// Result (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    /// Request ID
    pub id: u64,
}

impl JsonRpcResponse {
    /// Construct a success response
    #[must_use]
    pub fn success(id: u64, result: Value) -> Self {
        Self {
            jsonrpc: Cow::Borrowed(JSONRPC_VERSION),
            result: Some(result),
            error: None,
            id,
        }
    }

    /// Construct an error response
    #[must_use]
    pub fn error(id: u64, code: i32, message: impl Into<String>) -> Self {
        Self {
            jsonrpc: Cow::Borrowed(JSONRPC_VERSION),
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.into(),
                data: None,
            }),
            id,
        }
    }
}

/// JSON-RPC 2.0 error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// An incoming message that may be a single request or a batch (JSON-RPC 2.0 spec section 6).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcMessage {
    /// A single JSON-RPC request
    Single(JsonRpcRequest),
    /// A batch of JSON-RPC requests (array)
    Batch(Vec<JsonRpcRequest>),
}

/// An outgoing message that may be a single response or a batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcResponseMessage {
    /// A single JSON-RPC response
    Single(JsonRpcResponse),
    /// A batch of JSON-RPC responses (array)
    Batch(Vec<JsonRpcResponse>),
}

/// Canonical JSON-RPC version string (avoid repeated allocation)
pub const JSONRPC_VERSION: &str = "2.0";

/// Standard JSON-RPC error codes
pub mod error_codes {
    /// Parse error
    pub const PARSE_ERROR: i32 = -32700;
    /// Invalid request
    pub const INVALID_REQUEST: i32 = -32600;
    /// Method not found
    pub const METHOD_NOT_FOUND: i32 = -32601;
    /// Invalid params
    pub const INVALID_PARAMS: i32 = -32602;
    /// Internal error
    pub const INTERNAL_ERROR: i32 = -32603;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_request_serialization() {
        let request = JsonRpcRequest::new("test.method", json!({"key": "value"}), 1);

        let json = serde_json::to_string(&request).expect("serialize JsonRpcRequest");
        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"method\":\"test.method\""));
    }

    #[test]
    fn test_request_roundtrip() {
        let request = JsonRpcRequest::new("test.method", json!({"key": "value"}), 42);
        let json = serde_json::to_string(&request).expect("serialize JsonRpcRequest");
        let restored: JsonRpcRequest =
            serde_json::from_str(&json).expect("deserialize JsonRpcRequest roundtrip");
        assert_eq!(restored.method, request.method);
        assert_eq!(restored.id, request.id);
    }

    #[test]
    fn test_request_deserialization() {
        let json = r#"{"jsonrpc":"2.0","method":"primal.ping","params":null,"id":1}"#;
        let request: JsonRpcRequest =
            serde_json::from_str(json).expect("deserialize JsonRpcRequest from literal");
        assert_eq!(request.jsonrpc.as_ref(), "2.0");
        assert_eq!(request.method, "primal.ping");
        assert_eq!(request.id, 1);
    }

    #[test]
    fn test_response_with_result() {
        let response = JsonRpcResponse::success(1, json!({"success": true}));
        let json = serde_json::to_string(&response).expect("serialize JsonRpcResponse");
        assert!(json.contains("\"result\""));
        assert!(!json.contains("\"error\""));
    }

    #[test]
    fn test_response_with_error() {
        let response = JsonRpcResponse::error(1, error_codes::METHOD_NOT_FOUND, "Method not found");
        let json = serde_json::to_string(&response).expect("serialize JsonRpcResponse");
        assert!(!json.contains("\"result\""));
        assert!(json.contains("\"error\""));
    }

    #[test]
    fn test_response_roundtrip() {
        let response = JsonRpcResponse::success(99, json!({"registered": true}));
        let json = serde_json::to_string(&response).expect("serialize JsonRpcResponse");
        let restored: JsonRpcResponse =
            serde_json::from_str(&json).expect("deserialize JsonRpcResponse roundtrip");
        assert_eq!(restored.id, 99);
        assert!(restored.result.is_some());
    }

    #[test]
    fn test_json_rpc_error_with_data() {
        let err = JsonRpcError {
            code: error_codes::INVALID_PARAMS,
            message: "Invalid params".to_string(),
            data: Some(json!({"field": "missing"})),
        };
        let json = serde_json::to_string(&err).expect("serialize JsonRpcError with data");
        let restored: JsonRpcError =
            serde_json::from_str(&json).expect("deserialize JsonRpcError with data");
        assert_eq!(restored.code, error_codes::INVALID_PARAMS);
        assert!(restored.data.is_some());
    }

    #[test]
    fn test_json_rpc_error_without_data() {
        let err = JsonRpcError {
            code: error_codes::PARSE_ERROR,
            message: "Parse error".to_string(),
            data: None,
        };
        let json = serde_json::to_string(&err).expect("serialize JsonRpcError without data");
        assert!(!json.contains("\"data\""));
    }

    #[test]
    fn test_error_codes_constants() {
        assert_eq!(error_codes::PARSE_ERROR, -32700);
        assert_eq!(error_codes::INVALID_REQUEST, -32600);
        assert_eq!(error_codes::METHOD_NOT_FOUND, -32601);
        assert_eq!(error_codes::INVALID_PARAMS, -32602);
        assert_eq!(error_codes::INTERNAL_ERROR, -32603);
    }

    #[test]
    fn test_request_clone_debug() {
        let request = JsonRpcRequest::new("test", json!(null), 1);
        let cloned = request.clone();
        assert_eq!(cloned.method, request.method);
        assert!(format!("{request:?}").contains("test"));
    }

    #[test]
    fn test_batch_request_deserialization() {
        let json = r#"[
            {"jsonrpc":"2.0","method":"health.liveness","params":null,"id":1},
            {"jsonrpc":"2.0","method":"health.readiness","params":null,"id":2}
        ]"#;
        let msg: JsonRpcMessage = serde_json::from_str(json).expect("deserialize batch request");
        match msg {
            JsonRpcMessage::Batch(reqs) => {
                assert_eq!(reqs.len(), 2);
                assert_eq!(reqs[0].method, "health.liveness");
                assert_eq!(reqs[1].method, "health.readiness");
            }
            JsonRpcMessage::Single(_) => panic!("expected batch"),
        }
    }

    #[test]
    fn test_single_request_via_message() {
        let json = r#"{"jsonrpc":"2.0","method":"primal.ping","params":null,"id":1}"#;
        let msg: JsonRpcMessage =
            serde_json::from_str(json).expect("deserialize single as message");
        assert!(matches!(msg, JsonRpcMessage::Single(_)));
    }

    #[test]
    fn test_batch_response_serialization() {
        let batch = JsonRpcResponseMessage::Batch(vec![
            JsonRpcResponse::success(1, json!("pong")),
            JsonRpcResponse::error(2, error_codes::METHOD_NOT_FOUND, "not found"),
        ]);
        let json = serde_json::to_string(&batch).expect("serialize batch response");
        assert!(json.starts_with('['));
        assert!(json.contains("\"pong\""));
        assert!(json.contains("not found"));
    }

    #[test]
    fn test_constructor_helpers() {
        let req = JsonRpcRequest::new("health.check", json!({}), 42);
        assert_eq!(req.jsonrpc.as_ref(), JSONRPC_VERSION);
        assert_eq!(req.method, "health.check");

        let ok = JsonRpcResponse::success(42, json!(true));
        assert!(ok.result.is_some());
        assert!(ok.error.is_none());

        let err = JsonRpcResponse::error(42, error_codes::INTERNAL_ERROR, "boom");
        assert!(err.result.is_none());
        assert!(err.error.is_some());
    }
}
