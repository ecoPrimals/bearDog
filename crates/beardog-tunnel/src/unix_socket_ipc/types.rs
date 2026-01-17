//! JSON-RPC 2.0 Types and Protocol Detection
//!
//! Type definitions for JSON-RPC 2.0 protocol used in Unix socket IPC.
//! Fully compliant with JSON-RPC 2.0 specification.

use serde::{Deserialize, Serialize};

/// JSON-RPC 2.0 Request
#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default)]
    pub params: Option<serde_json::Value>,
    pub id: Option<serde_json::Value>,
}

/// JSON-RPC 2.0 Response
#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    pub id: serde_json::Value,
}

/// JSON-RPC 2.0 Error
#[derive(Debug, Clone, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl JsonRpcError {
    /// Standard JSON-RPC 2.0 error codes
    pub const PARSE_ERROR: i32 = -32700;
    pub const INVALID_REQUEST: i32 = -32600;
    pub const METHOD_NOT_FOUND: i32 = -32601;
    pub const INVALID_PARAMS: i32 = -32602;
    pub const INTERNAL_ERROR: i32 = -32603;

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
}

/// Protocol detection result
///
/// Priority order (from upstream evolution debt):
/// 1. tarpc (PRIMARY) - Type-safe, efficient, modern Rust
/// 2. JSON-RPC (FALLBACK) - Universal adapter
/// 3. HTTP (LEGACY) - Less secure, less reliable, less fractal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    /// Primary: Type-safe inter-primal (security level 5)
    Tarpc,
    /// Fallback: Universal adapter (security level 4)
    JsonRpc,
    /// Legacy: Compatibility only (security level 2)
    Http,
}

impl Protocol {
    /// Detect protocol from first bytes
    ///
    /// Deep debt solution: Pattern matching on actual bytes, not magic numbers
    ///
    /// Detection logic:
    /// - tarpc: Begins with tarpc magic bytes (TBD: document exact format)
    /// - JSON-RPC: Begins with `{` (JSON object)
    /// - HTTP: Begins with HTTP verbs (GET, POST, etc.)
    pub fn detect_from_bytes(first_bytes: &[u8]) -> Self {
        if first_bytes.is_empty() {
            return Protocol::JsonRpc; // Default fallback
        }

        // Check for HTTP verbs (legacy protocol)
        if first_bytes.starts_with(b"GET ")
            || first_bytes.starts_with(b"POST ")
            || first_bytes.starts_with(b"PUT ")
            || first_bytes.starts_with(b"DELETE ")
            || first_bytes.starts_with(b"PATCH ")
        {
            return Protocol::Http;
        }

        // Check for JSON-RPC (starts with '{' and likely contains "jsonrpc":"2.0")
        if first_bytes.starts_with(b"{") {
            return Protocol::JsonRpc;
        }

        // Check for tarpc magic bytes: 0x54 0x52 0x50 0x43 ("TRPC" in ASCII)
        // tarpc uses bincode serialization with magic header
        if first_bytes.len() >= 4 && first_bytes.starts_with(b"TRPC") {
            return Protocol::Tarpc;
        }

        // Default: JSON-RPC (universal fallback)
        Protocol::JsonRpc
    }

    /// Get protocol name for logging
    pub fn name(&self) -> &'static str {
        match self {
            Protocol::Tarpc => "tarpc",
            Protocol::JsonRpc => "json-rpc",
            Protocol::Http => "http",
        }
    }

    /// Get security level (5 = highest, 1 = lowest)
    pub fn security_level(&self) -> u8 {
        match self {
            Protocol::Tarpc => 5,   // Type-safe, encrypted
            Protocol::JsonRpc => 4, // Structured, can be encrypted
            Protocol::Http => 2,    // Plain text, less secure
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
        assert_eq!(Protocol::Tarpc.security_level(), 5);
        assert_eq!(Protocol::JsonRpc.security_level(), 4);
        assert_eq!(Protocol::Http.security_level(), 2);
    }
}
