// SPDX-License-Identifier: AGPL-3.0-only

//! JSON-RPC 2.0 protocol implementation

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC 2.0 request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Method name
    pub method: String,
    /// Method parameters
    pub params: Value,
    /// Request ID
    pub id: u64,
}

/// JSON-RPC 2.0 response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Result (if successful)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    /// Error (if failed)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    /// Request ID
    pub id: u64,
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
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "test.method".to_string(),
            params: json!({"key": "value"}),
            id: 1,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"method\":\"test.method\""));
    }

    #[test]
    fn test_request_roundtrip() {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "test.method".to_string(),
            params: json!({"key": "value"}),
            id: 42,
        };
        let json = serde_json::to_string(&request).unwrap();
        let restored: JsonRpcRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.method, request.method);
        assert_eq!(restored.id, request.id);
    }

    #[test]
    fn test_request_deserialization() {
        let json = r#"{"jsonrpc":"2.0","method":"primal.ping","params":null,"id":1}"#;
        let request: JsonRpcRequest = serde_json::from_str(json).unwrap();
        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "primal.ping");
        assert_eq!(request.id, 1);
    }

    #[test]
    fn test_response_with_result() {
        let response = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({"success": true})),
            error: None,
            id: 1,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"result\""));
        assert!(!json.contains("\"error\""));
    }

    #[test]
    fn test_response_with_error() {
        let response = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: error_codes::METHOD_NOT_FOUND,
                message: "Method not found".to_string(),
                data: None,
            }),
            id: 1,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(!json.contains("\"result\""));
        assert!(json.contains("\"error\""));
    }

    #[test]
    fn test_response_roundtrip() {
        let response = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(json!({"registered": true})),
            error: None,
            id: 99,
        };
        let json = serde_json::to_string(&response).unwrap();
        let restored: JsonRpcResponse = serde_json::from_str(&json).unwrap();
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
        let json = serde_json::to_string(&err).unwrap();
        let restored: JsonRpcError = serde_json::from_str(&json).unwrap();
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
        let json = serde_json::to_string(&err).unwrap();
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
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            method: "test".to_string(),
            params: json!(null),
            id: 1,
        };
        let cloned = request.clone();
        assert_eq!(cloned.method, request.method);
        assert!(format!("{:?}", request).contains("test"));
    }
}
