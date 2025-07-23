//! Error Handling for BearDog API
//!
//! Centralized error handling and response formatting for the API layer.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Standard API error response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    /// Error code for programmatic handling
    pub error_code: String,
    /// Human-readable error message
    pub message: String,
    /// Additional error details
    pub details: Option<HashMap<String, String>>,
    /// Request ID for tracing
    pub request_id: Option<String>,
    /// Timestamp of the error
    pub timestamp: String,
}

/// API error types
#[derive(Debug, Clone, PartialEq)]
pub enum ApiErrorType {
    /// Authentication errors
    Authentication,
    /// Authorization errors
    Authorization,
    /// Validation errors
    Validation,
    /// Resource not found
    NotFound,
    /// Conflict errors
    Conflict,
    /// Rate limiting errors
    RateLimit,
    /// Internal server errors
    Internal,
    /// Service unavailable
    ServiceUnavailable,
}

impl ApiErrorType {
    /// Convert error type to HTTP status code
    pub fn to_status_code(&self) -> StatusCode {
        match self {
            ApiErrorType::Authentication => StatusCode::UNAUTHORIZED,
            ApiErrorType::Authorization => StatusCode::FORBIDDEN,
            ApiErrorType::Validation => StatusCode::BAD_REQUEST,
            ApiErrorType::NotFound => StatusCode::NOT_FOUND,
            ApiErrorType::Conflict => StatusCode::CONFLICT,
            ApiErrorType::RateLimit => StatusCode::TOO_MANY_REQUESTS,
            ApiErrorType::Internal => StatusCode::INTERNAL_SERVER_ERROR,
            ApiErrorType::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    /// Get error code string
    pub fn error_code(&self) -> &'static str {
        match self {
            ApiErrorType::Authentication => "AUTHENTICATION_ERROR",
            ApiErrorType::Authorization => "AUTHORIZATION_ERROR",
            ApiErrorType::Validation => "VALIDATION_ERROR",
            ApiErrorType::NotFound => "NOT_FOUND",
            ApiErrorType::Conflict => "CONFLICT",
            ApiErrorType::RateLimit => "RATE_LIMIT_EXCEEDED",
            ApiErrorType::Internal => "INTERNAL_ERROR",
            ApiErrorType::ServiceUnavailable => "SERVICE_UNAVAILABLE",
        }
    }
}

impl ApiError {
    /// Create a new API error
    pub fn new(
        error_type: ApiErrorType,
        message: impl Into<String>,
        request_id: Option<String>,
    ) -> Self {
        Self {
            error_code: error_type.error_code().to_string(),
            message: message.into(),
            details: None,
            request_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    /// Add error details
    pub fn with_details(mut self, details: HashMap<String, String>) -> Self {
        self.details = Some(details);
        self
    }

    /// Add a single detail
    pub fn with_detail(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let mut details = self.details.unwrap_or_default();
        details.insert(key.into(), value.into());
        self.details = Some(details);
        self
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.error_code.as_str() {
            "AUTHENTICATION_ERROR" => StatusCode::UNAUTHORIZED,
            "AUTHORIZATION_ERROR" => StatusCode::FORBIDDEN,
            "VALIDATION_ERROR" => StatusCode::BAD_REQUEST,
            "NOT_FOUND" => StatusCode::NOT_FOUND,
            "CONFLICT" => StatusCode::CONFLICT,
            "RATE_LIMIT_EXCEEDED" => StatusCode::TOO_MANY_REQUESTS,
            "SERVICE_UNAVAILABLE" => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(self)).into_response()
    }
}

/// Helper functions for common error responses
pub mod helpers {
    use super::*;

    /// Create an authentication error
    pub fn authentication_error(
        message: impl Into<String>,
        request_id: Option<String>,
    ) -> ApiError {
        ApiError::new(ApiErrorType::Authentication, message, request_id)
    }

    /// Create an authorization error
    pub fn authorization_error(message: impl Into<String>, request_id: Option<String>) -> ApiError {
        ApiError::new(ApiErrorType::Authorization, message, request_id)
    }

    /// Create a validation error
    pub fn validation_error(message: impl Into<String>, request_id: Option<String>) -> ApiError {
        ApiError::new(ApiErrorType::Validation, message, request_id)
    }

    /// Create a not found error
    pub fn not_found_error(message: impl Into<String>, request_id: Option<String>) -> ApiError {
        ApiError::new(ApiErrorType::NotFound, message, request_id)
    }

    /// Create an internal error
    pub fn internal_error(message: impl Into<String>, request_id: Option<String>) -> ApiError {
        ApiError::new(ApiErrorType::Internal, message, request_id)
    }

    /// Create a rate limit error
    pub fn rate_limit_error(message: impl Into<String>, request_id: Option<String>) -> ApiError {
        ApiError::new(ApiErrorType::RateLimit, message, request_id)
    }
}
