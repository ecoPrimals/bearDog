//! Error types for IPC operations

use thiserror::Error;

/// IPC operation errors
#[derive(Debug, Error)]
pub enum IpcError {
    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Protocol error
    #[error("Protocol error: {0}")]
    Protocol(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Service not found
    #[error("Service not found: {0}")]
    ServiceNotFound(String),

    /// Timeout
    #[error("Operation timed out")]
    Timeout,
}

/// Result type for IPC operations
pub type IpcResult<T> = Result<T, IpcError>;

