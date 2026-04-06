// SPDX-License-Identifier: AGPL-3.0-or-later

//! Error types for Tower Atomic

use serde_json::Value;
use thiserror::Error;

/// Tower Atomic error type
#[derive(Debug, Error)]
pub enum Error {
    /// Primal not found during discovery
    #[error("Primal not found: {0}")]
    PrimalNotFound(String),

    /// JSON-RPC error from remote primal
    #[error("JSON-RPC error (code {code}): {message}")]
    JsonRpcError {
        /// JSON-RPC error code (e.g. `-32601` method not found).
        code: i32,
        /// Human-readable error message from the remote primal.
        message: String,
        /// Optional structured error payload from the peer, when provided.
        data: Option<Value>,
    },

    /// Connection failed
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// Serialization/deserialization error
    #[error("Serialization failed: {0}")]
    SerializationFailed(String),

    /// Environment variable error
    #[error("Environment error: {0}")]
    EnvironmentError(#[from] std::env::VarError),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

/// Result type for Tower Atomic operations
pub type Result<T> = std::result::Result<T, Error>;
