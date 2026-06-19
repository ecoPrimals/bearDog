// SPDX-License-Identifier: AGPL-3.0-or-later

//! JSON-RPC 2.0 protocol implementation (batch-capable, NDJSON-aware)

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;

/// JSON-RPC 2.0 request
///
/// Per the JSON-RPC 2.0 spec, `id` may be a String, Number, or Null. Requests
/// without `id` are *notifications* and MUST NOT receive a response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: Cow<'static, str>,
    /// Method name
    pub method: Cow<'static, str>,
    /// Method parameters
    pub params: Value,
    /// Request ID -- absent for notifications (JSON-RPC 2.0 spec section 4.1)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
}

impl JsonRpcRequest {
    /// Construct a new request with version pre-set
    #[must_use]
    pub fn new(method: impl Into<Cow<'static, str>>, params: Value, id: impl Into<Value>) -> Self {
        Self {
            jsonrpc: Cow::Borrowed(JSONRPC_VERSION),
            method: method.into(),
            params,
            id: Some(id.into()),
        }
    }

    /// Construct a notification (no id, server MUST NOT respond)
    #[must_use]
    pub fn notification(method: impl Into<Cow<'static, str>>, params: Value) -> Self {
        Self {
            jsonrpc: Cow::Borrowed(JSONRPC_VERSION),
            method: method.into(),
            params,
            id: None,
        }
    }

    /// Returns `true` when this is a notification (no `id` field).
    #[must_use]
    pub fn is_notification(&self) -> bool {
        self.id.is_none()
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
    /// Request ID echoed back from the request
    pub id: Value,
}

impl JsonRpcResponse {
    /// Construct a success response
    #[must_use]
    pub fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: Cow::Borrowed(JSONRPC_VERSION),
            result: Some(result),
            error: None,
            id,
        }
    }

    /// Construct an error response
    #[must_use]
    pub fn error(id: Value, code: i32, message: impl Into<String>) -> Self {
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

    // ── Ecosystem method-gate codes (JH-0) ──────────────────────────

    /// Caller identity could not be established.
    pub const UNAUTHORIZED: i32 = -32000;
    /// Caller lacks scope for the requested method.
    pub const PERMISSION_DENIED: i32 = -32001;
    /// Primal not yet initialized (still starting).
    pub const NOT_READY: i32 = -32002;
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
        assert_eq!(request.id, Some(json!(1)));
    }

    #[test]
    fn test_request_deserialization_string_id() {
        let json = r#"{"jsonrpc":"2.0","method":"primal.ping","params":null,"id":"abc-123"}"#;
        let request: JsonRpcRequest = serde_json::from_str(json).expect("deserialize string id");
        assert_eq!(request.id, Some(json!("abc-123")));
    }

    #[test]
    fn test_notification_deserialization() {
        let json = r#"{"jsonrpc":"2.0","method":"primal.notify","params":{}}"#;
        let request: JsonRpcRequest = serde_json::from_str(json).expect("deserialize notification");
        assert!(request.is_notification());
        assert_eq!(request.id, None);
    }

    #[test]
    fn test_notification_constructor() {
        let n = JsonRpcRequest::notification("log.event", json!({"level": "info"}));
        assert!(n.is_notification());
        assert_eq!(n.id, None);
        let json = serde_json::to_string(&n).expect("serialize notification");
        assert!(!json.contains("\"id\""));
    }

    #[test]
    fn test_response_with_result() {
        let response = JsonRpcResponse::success(json!(1), json!({"success": true}));
        let json = serde_json::to_string(&response).expect("serialize JsonRpcResponse");
        assert!(json.contains("\"result\""));
        assert!(!json.contains("\"error\""));
    }

    #[test]
    fn test_response_with_error() {
        let response =
            JsonRpcResponse::error(json!(1), error_codes::METHOD_NOT_FOUND, "Method not found");
        let json = serde_json::to_string(&response).expect("serialize JsonRpcResponse");
        assert!(!json.contains("\"result\""));
        assert!(json.contains("\"error\""));
    }

    #[test]
    fn test_response_roundtrip() {
        let response = JsonRpcResponse::success(json!(99), json!({"registered": true}));
        let json = serde_json::to_string(&response).expect("serialize JsonRpcResponse");
        let restored: JsonRpcResponse =
            serde_json::from_str(&json).expect("deserialize JsonRpcResponse roundtrip");
        assert_eq!(restored.id, json!(99));
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
    fn test_response_with_null_id() {
        let response = JsonRpcResponse::error(Value::Null, error_codes::PARSE_ERROR, "bad");
        let json = serde_json::to_string(&response).expect("serialize null id response");
        assert!(json.contains("\"id\":null"));
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
            JsonRpcResponse::success(json!(1), json!("pong")),
            JsonRpcResponse::error(json!(2), error_codes::METHOD_NOT_FOUND, "not found"),
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

        let ok = JsonRpcResponse::success(json!(42), json!(true));
        assert!(ok.result.is_some());
        assert!(ok.error.is_none());

        let err = JsonRpcResponse::error(json!(42), error_codes::INTERNAL_ERROR, "boom");
        assert!(err.result.is_none());
        assert!(err.error.is_some());
    }
}
