// SPDX-License-Identifier: AGPL-3.0-only

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_error_connection_display() {
        let err = IpcError::Connection("conn failed".to_string());
        assert_eq!(format!("{}", err), "Connection error: conn failed");
    }

    #[test]
    fn test_ipc_error_protocol_display() {
        let err = IpcError::Protocol("bad protocol".to_string());
        assert_eq!(format!("{}", err), "Protocol error: bad protocol");
    }

    #[test]
    fn test_ipc_error_serialization_display() {
        let err = IpcError::Serialization("bad json".to_string());
        assert_eq!(format!("{}", err), "Serialization error: bad json");
    }

    #[test]
    fn test_ipc_error_service_not_found_display() {
        let err = IpcError::ServiceNotFound("crypto".to_string());
        assert_eq!(format!("{}", err), "Service not found: crypto");
    }

    #[test]
    fn test_ipc_error_timeout_display() {
        let err = IpcError::Timeout;
        assert_eq!(format!("{}", err), "Operation timed out");
    }

    #[test]
    fn test_ipc_error_debug() {
        let err = IpcError::Connection("test".to_string());
        assert!(format!("{:?}", err).contains("Connection"));
    }

    #[test]
    fn test_ipc_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let ipc_err: IpcError = io_err.into();
        assert!(format!("{}", ipc_err).contains("I/O error"));
    }

    #[test]
    fn test_ipc_result_ok() {
        let ok: IpcResult<u32> = Ok(42);
        assert!(ok.is_ok());
        assert_eq!(ok.unwrap(), 42);
    }

    #[test]
    fn test_ipc_result_err() {
        let err: IpcResult<u32> = Err(IpcError::Timeout);
        assert!(err.is_err());
    }
}
