//! Common API types and utilities

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

/// Standard API error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error message
    pub error: String,
    /// Error code (for programmatic handling)
    pub code: String,
    /// HTTP status code
    pub status: u16,
}

impl ApiError {
    /// Create a new API error
    pub fn new(status: StatusCode, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: message.into(),
            code: code.into(),
            status: status.as_u16(),
        }
    }

    /// Create a bad request error
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "bad_request", message)
    }

    /// Create an internal server error
    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "internal_error", message)
    }

    /// Create a not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, "not_found", message)
    }

    /// Create an unauthorized error
    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "unauthorized", message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

/// Convert BearDogError to ApiError
impl From<beardog_errors::BearDogError> for ApiError {
    fn from(err: beardog_errors::BearDogError) -> Self {
        use beardog_errors::BearDogError;

        match err {
            BearDogError::Security { message, .. } => {
                ApiError::new(StatusCode::UNAUTHORIZED, "security_error", message)
            }
            BearDogError::Business { message, .. } => {
                ApiError::new(StatusCode::BAD_REQUEST, "business_error", message)
            }
            BearDogError::Network { message, .. } => {
                ApiError::new(StatusCode::SERVICE_UNAVAILABLE, "network_error", message)
            }
            BearDogError::Configuration { message, .. } => {
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "configuration_error", message)
            }
            _ => ApiError::internal(err.to_string()),
        }
    }
}

/// Standard success response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Success indicator
    pub success: bool,
    /// Response data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// Optional message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    /// Create a success response
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: None,
        }
    }

    /// Create a success response with message
    pub fn success_with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            message: Some(message.into()),
        }
    }
}

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Service status
    pub status: String,
    /// Service version
    pub version: String,
    /// Available capabilities
    pub capabilities: Vec<String>,
}

/// Base64 serialization helper
pub mod base64_serde {
    use base64::{engine::general_purpose::STANDARD, Engine};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(data: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&STANDARD.encode(data))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        STANDARD.decode(s).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_creation() {
        let err = ApiError::bad_request("Invalid input");
        assert_eq!(err.status, 400);
        assert_eq!(err.code, "bad_request");
        assert_eq!(err.error, "Invalid input");
    }

    #[test]
    fn test_api_response() {
        let resp = ApiResponse::success("test data");
        assert!(resp.success);
        assert_eq!(resp.data, Some("test data"));
        assert_eq!(resp.message, None);
    }
}

