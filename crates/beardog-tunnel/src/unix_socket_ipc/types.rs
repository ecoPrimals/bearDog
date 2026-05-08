// SPDX-License-Identifier: AGPL-3.0-or-later

//! JSON-RPC 2.0 Types and Protocol Detection
//!
//! Type definitions for JSON-RPC 2.0 protocol used in Unix socket IPC.
//! Fully compliant with JSON-RPC 2.0 specification.

use serde::{Deserialize, Serialize};

/// JSON-RPC 2.0 Request structure
///
/// Represents a complete JSON-RPC 2.0 request as defined by the specification.
/// All `BearDog` inter-primal communication uses this format over Unix sockets.
#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version string, always "2.0"
    pub jsonrpc: String,

    /// Method name to invoke (e.g., "`graph.validate_template`")
    pub method: String,

    /// Optional parameters for the method call
    #[serde(default)]
    pub params: Option<serde_json::Value>,

    /// Optional request identifier for matching responses
    pub id: Option<serde_json::Value>,
}

/// JSON-RPC 2.0 Response structure
///
/// Either `result` or `error` will be present, never both.
/// The `id` matches the request that triggered this response.
#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version string, always "2.0"
    pub jsonrpc: String,

    /// Result of the method call (present on success)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,

    /// Error details (present on error)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,

    /// Request identifier
    pub id: serde_json::Value,
}

/// JSON-RPC 2.0 Error structure
///
/// Standard error codes: -32700 (parse), -32600 (invalid request),
/// -32601 (method not found), -32602 (invalid params), -32603 (internal error)
#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcError {
    /// Error code (standard codes defined by JSON-RPC 2.0)
    pub code: i32,

    /// Human-readable error message
    pub message: String,

    /// Optional additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl JsonRpcError {
    /// Parse error code (-32700)
    ///
    /// Invalid JSON was received by the server. An error occurred on the server
    /// while parsing the JSON text. This typically indicates malformed JSON syntax.
    pub const PARSE_ERROR: i32 = -32700;

    /// Invalid Request error code (-32600)
    ///
    /// The JSON sent is not a valid Request object according to JSON-RPC 2.0 spec.
    /// Required fields may be missing or have incorrect types.
    pub const INVALID_REQUEST: i32 = -32600;

    /// Method not found error code (-32601)
    ///
    /// The method does not exist or is not available. Check the method name
    /// and ensure it's supported by this server.
    pub const METHOD_NOT_FOUND: i32 = -32601;

    /// Invalid params error code (-32602)
    ///
    /// Invalid method parameter(s). The method exists but the parameters provided
    /// are invalid - wrong types, missing required parameters, or invalid values.
    pub const INVALID_PARAMS: i32 = -32602;

    /// Internal error code (-32603)
    ///
    /// Internal JSON-RPC error. An internal error occurred in the JSON-RPC handler.
    /// This typically indicates a bug or unexpected condition in the server.
    pub const INTERNAL_ERROR: i32 = -32603;

    /// Caller identity could not be established (-32000).
    pub const UNAUTHORIZED: i32 = -32000;

    /// Caller lacks scope for the requested method (-32001).
    pub const PERMISSION_DENIED: i32 = -32001;

    /// Primal not yet initialized (-32002).
    pub const NOT_READY: i32 = -32002;

    /// Create a parse error
    pub fn parse_error(message: impl Into<String>) -> Self {
        Self {
            code: Self::PARSE_ERROR,
            message: message.into(),
            data: None,
        }
    }

    /// Create a method not found error
    pub fn method_not_found(method: impl Into<String>) -> Self {
        Self {
            code: Self::METHOD_NOT_FOUND,
            message: format!("Method not found: {}", method.into()),
            data: None,
        }
    }

    /// Create an internal error
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self {
            code: Self::INTERNAL_ERROR,
            message: message.into(),
            data: None,
        }
    }

    /// Create an invalid params error
    pub fn invalid_params(message: impl Into<String>) -> Self {
        Self {
            code: Self::INVALID_PARAMS,
            message: message.into(),
            data: None,
        }
    }

    /// Create an invalid request error
    pub fn invalid_request(message: impl Into<String>) -> Self {
        Self {
            code: Self::INVALID_REQUEST,
            message: message.into(),
            data: None,
        }
    }

    /// Create a permission denied error (method gate, JH-0).
    pub fn permission_denied(method: &str) -> Self {
        Self {
            code: Self::PERMISSION_DENIED,
            message: format!("permission denied: method '{method}' requires a capability token"),
            data: Some(serde_json::json!({ "method": method })),
        }
    }

    /// Create an unauthorized error (caller identity unknown).
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            code: Self::UNAUTHORIZED,
            message: message.into(),
            data: None,
        }
    }
}

/// Protocol detection result
///
/// Priority order:
/// 1. JSON-RPC (PRIMARY) - Universal, comprehensive, production-ready
/// 2. HTTP (LEGACY) - Compatibility only, less secure
///
/// Note: tarpc was removed (Jan 29, 2026) — JSON-RPC provides all needed functionality.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    /// Primary: Universal JSON-RPC 2.0 (security level 4)
    JsonRpc,
    /// Legacy: HTTP compatibility (security level 2)
    Http,
}

impl Protocol {
    /// Detect protocol from first bytes
    ///
    /// Detection logic:
    /// - JSON-RPC: Begins with `{` (JSON object) - PRIMARY protocol
    /// - HTTP: Begins with HTTP verbs (GET, POST, etc.) - LEGACY compatibility
    pub fn detect_from_bytes(first_bytes: &[u8]) -> Self {
        if first_bytes.is_empty() {
            return Self::JsonRpc; // Default to primary protocol
        }

        // Check for HTTP verbs (legacy protocol)
        if first_bytes.starts_with(b"GET ")
            || first_bytes.starts_with(b"POST ")
            || first_bytes.starts_with(b"PUT ")
            || first_bytes.starts_with(b"DELETE ")
            || first_bytes.starts_with(b"PATCH ")
        {
            return Self::Http;
        }

        // Check for JSON-RPC (starts with '{' and likely contains "jsonrpc":"2.0")
        if first_bytes.starts_with(b"{") {
            return Self::JsonRpc;
        }

        // Default: JSON-RPC (primary protocol)
        Self::JsonRpc
    }

    /// Get protocol name for logging
    pub const fn name(&self) -> &'static str {
        match self {
            Self::JsonRpc => "json-rpc",
            Self::Http => "http",
        }
    }

    /// Get security level (5 = highest, 1 = lowest)
    pub const fn security_level(&self) -> u8 {
        match self {
            Self::JsonRpc => 4, // Structured, comprehensive, production-ready
            Self::Http => 2,    // Plain text, less secure, legacy
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_detection_http() {
        assert_eq!(
            Protocol::detect_from_bytes(b"GET /health HTTP/1.1"),
            Protocol::Http
        );
        assert_eq!(
            Protocol::detect_from_bytes(b"POST /api/encrypt HTTP/1.1"),
            Protocol::Http
        );
    }

    #[test]
    fn test_protocol_detection_jsonrpc() {
        assert_eq!(
            Protocol::detect_from_bytes(b"{\"jsonrpc\":\"2.0\""),
            Protocol::JsonRpc
        );
    }

    #[test]
    fn test_protocol_detection_empty() {
        assert_eq!(Protocol::detect_from_bytes(b""), Protocol::JsonRpc);
    }

    #[test]
    fn test_jsonrpc_error_constructors() {
        let err = JsonRpcError::parse_error("Invalid JSON");
        assert_eq!(err.code, JsonRpcError::PARSE_ERROR);
        assert_eq!(err.message, "Invalid JSON");

        let err = JsonRpcError::method_not_found("test.method");
        assert_eq!(err.code, JsonRpcError::METHOD_NOT_FOUND);
        assert!(err.message.contains("test.method"));

        let err = JsonRpcError::internal_error("Something went wrong");
        assert_eq!(err.code, JsonRpcError::INTERNAL_ERROR);
        assert_eq!(err.message, "Something went wrong");
    }

    #[test]
    fn test_protocol_security_levels() {
        assert_eq!(Protocol::JsonRpc.security_level(), 4);
        assert_eq!(Protocol::Http.security_level(), 2);
    }
}
