//! Error types for BearDog client

use thiserror::Error;

/// Result type for BearDog client operations
pub type ClientResult<T> = Result<T, BearDogClientError>;

/// BearDog client errors
#[derive(Debug, Error)]
pub enum BearDogClientError {
    /// Tower Atomic connection failed
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),

    /// API returned an error
    #[error("API error: {0}")]
    ApiError(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    /// Invalid response from API
    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    /// Timeout
    #[error("Request timeout")]
    Timeout,

    /// Connection error
    #[error("Connection error: {0}")]
    ConnectionError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_failed_error() {
        let err = BearDogClientError::ConnectionFailed("socket not found".to_string());
        assert_eq!(format!("{}", err), "Connection failed: socket not found");
    }

    #[test]
    fn test_api_error() {
        let err = BearDogClientError::ApiError("method not found".to_string());
        assert_eq!(format!("{}", err), "API error: method not found");
    }

    #[test]
    fn test_serialization_error_from_serde() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let err: BearDogClientError = json_err.into();
        assert!(format!("{}", err).contains("Serialization error"));
    }

    #[test]
    fn test_invalid_response_error() {
        let err = BearDogClientError::InvalidResponse("missing field".to_string());
        assert_eq!(format!("{}", err), "Invalid response: missing field");
    }

    #[test]
    fn test_timeout_error() {
        let err = BearDogClientError::Timeout;
        assert_eq!(format!("{}", err), "Request timeout");
    }

    #[test]
    fn test_connection_error() {
        let err = BearDogClientError::ConnectionError("network unreachable".to_string());
        assert_eq!(format!("{}", err), "Connection error: network unreachable");
    }

    #[test]
    fn test_error_debug() {
        let err = BearDogClientError::Timeout;
        let debug = format!("{:?}", err);
        assert!(debug.contains("Timeout"));
    }

    #[test]
    fn test_client_result_ok() {
        let result: ClientResult<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_client_result_err() {
        let result: ClientResult<i32> = Err(BearDogClientError::Timeout);
        assert!(result.is_err());
    }
}
