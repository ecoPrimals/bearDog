//! Protocol Implementations for Universal Adapter
//!
//! Protocol-agnostic communication layer

use beardog_errors::{BearDogError, BearDogResult};

use std::collections::HashMap;

/// Protocol trait for universal adapter
pub trait Protocol: Send + Sync {
    /// Protocol name
    fn name(&self) -> &str;

    /// Protocol version
    fn version(&self) -> &str;

    /// Validate connection parameters
    fn validate_connection(&self, params: &HashMap<String, String>) -> BearDogResult<()>;
}

/// HTTP protocol implementation
pub struct HttpProtocol {
    version: String,
}

impl Default for HttpProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpProtocol {
    pub fn new() -> Self {
        Self {
            version: "1.1".to_string(),
        }
    }
}

impl Protocol for HttpProtocol {
    fn name(&self) -> &str {
        "http"
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_connection(&self, params: &HashMap<String, String>) -> BearDogResult<()> {
        let _endpoint = params
            .get("endpoint")
            .ok_or_else(|| BearDogError::ConfigurationError {
                message: "Missing 'endpoint' parameter for HTTP protocol".to_string(),
            })?;

        // Basic URL validation could go here
        Ok(())
    }
}

/// WebSocket protocol implementation
pub struct WebSocketProtocol {
    version: String,
}

impl Default for WebSocketProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl WebSocketProtocol {
    pub fn new() -> Self {
        Self {
            version: "13".to_string(),
        }
    }
}

impl Protocol for WebSocketProtocol {
    fn name(&self) -> &str {
        "websocket"
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_connection(&self, params: &HashMap<String, String>) -> BearDogResult<()> {
        let endpoint = params
            .get("endpoint")
            .ok_or_else(|| BearDogError::ConfigurationError {
                message: "Missing 'endpoint' parameter for WebSocket protocol".to_string(),
            })?;

        if !endpoint.starts_with("ws://") && !endpoint.starts_with("wss://") {
            return Err(BearDogError::ConfigurationError {
                message: "WebSocket endpoint must start with ws:// or wss://".to_string(),
            });
        }

        Ok(())
    }
}

/// gRPC protocol implementation
pub struct GrpcProtocol {
    version: String,
}

impl Default for GrpcProtocol {
    fn default() -> Self {
        Self::new()
    }
}

impl GrpcProtocol {
    pub fn new() -> Self {
        Self {
            version: "2.0".to_string(),
        }
    }
}

impl Protocol for GrpcProtocol {
    fn name(&self) -> &str {
        "grpc"
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn validate_connection(&self, params: &HashMap<String, String>) -> BearDogResult<()> {
        let _endpoint = params
            .get("endpoint")
            .ok_or_else(|| BearDogError::ConfigurationError {
                message: "Missing 'endpoint' parameter for gRPC protocol".to_string(),
            })?;

        // gRPC-specific validation could go here
        Ok(())
    }
}
