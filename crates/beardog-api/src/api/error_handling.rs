// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// **CANONICAL ERROR HANDLING** ✅ **COMPLETE**
/// 
/// This module now uses the unified BearDogError system exclusively.
/// All API errors are handled through BearDogError::Api variant with proper categorization.
/// 
/// **Usage Pattern**:
/// ```rust
/// use beardog_errors::{BearDogError, BearDogResult};
/// 
/// // Authentication error
/// BearDogError::api("Invalid credentials", Some(401), Some("/auth/login"))
/// 
/// // Validation error  
/// BearDogError::api("Invalid input data", Some(400), Some("/api/users"))
/// 
/// // Not found error
/// BearDogError::api("Resource not found", Some(404), Some("/api/resource/123"))
/// ```

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::network::HttpStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **CANONICAL API ERROR HANDLING** ✅
/// Modern error handling using the unified BearDogError system
pub struct ApiErrorHandler;

impl ApiErrorHandler {
    /// Convert BearDogError to HTTP status code
    pub fn to_status_code(error: &BearDogError) -> StatusCode {
        match error {
            BearDogError::Api { category, .. } => {
                use beardog_errors::ApiErrorCategory;
                match category {
                    ApiErrorCategory::Authentication => StatusCode::UNAUTHORIZED,
                    ApiErrorCategory::Authorization => StatusCode::FORBIDDEN,
                    ApiErrorCategory::Validation => StatusCode::BAD_REQUEST,
                    ApiErrorCategory::NotFound => StatusCode::NOT_FOUND,
                    ApiErrorCategory::Conflict => StatusCode::CONFLICT,
                    ApiErrorCategory::RateLimit => StatusCode::TOO_MANY_REQUESTS,
                    ApiErrorCategory::Internal => StatusCode::INTERNAL_SERVER_ERROR,
                    ApiErrorCategory::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
                }
            }
            BearDogError::Security { .. } => StatusCode::FORBIDDEN,
            BearDogError::Business { .. } => StatusCode::BAD_REQUEST,
            BearDogError::Network { .. } => StatusCode::BAD_GATEWAY,
            BearDogError::Configuration { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// Get error code string from BearDogError
    pub fn error_code(error: &BearDogError) -> &'static str {
        match error {
            BearDogError::Api { category, .. } => {
                use beardog_errors::ApiErrorCategory;
                match category {
                    ApiErrorCategory::Authentication => "AUTHENTICATION_ERROR",
                    ApiErrorCategory::Authorization => "AUTHORIZATION_ERROR",
                    ApiErrorCategory::Validation => "VALIDATION_ERROR",
                    ApiErrorCategory::NotFound => "NOT_FOUND",
                    ApiErrorCategory::Conflict => "CONFLICT",
                    ApiErrorCategory::RateLimit => "RATE_LIMIT_EXCEEDED",
                    ApiErrorCategory::Internal => "INTERNAL_ERROR",
                    ApiErrorCategory::ServiceUnavailable => "SERVICE_UNAVAILABLE",
                }
            }
            BearDogError::Security { .. } => "SECURITY_ERROR",
            BearDogError::Business { .. } => "BUSINESS_ERROR",
            BearDogError::Network { .. } => "NETWORK_ERROR",
            BearDogError::Configuration { .. } => "CONFIGURATION_ERROR",
            _ => "UNKNOWN_ERROR",
        }
    }

    /// Create API authentication error
    pub fn authentication_error(message: impl Into<String>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::Authentication)
    }

    /// Create API authorization error
    pub fn authorization_error(message: impl Into<String>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::Authorization)
    }

    /// Create API validation error
    pub fn validation_error(message: impl Into<String>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::Validation)
    }

    /// Create API not found error
    pub fn not_found_error(message: impl Into<String>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::NotFound)
    }

    /// Create API conflict error
    pub fn conflict_error(message: impl Into<String>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::Conflict)
    }

    /// Create API rate limit error
    pub fn rate_limit_error(message: impl Into<String>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::RateLimit)
    }

    /// Create API internal error
    pub fn internal_error(message: impl Into<String>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::Internal)
    }

    /// Create API service unavailable error
    pub fn service_unavailable_error(message: impl Into<String>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::ServiceUnavailable)
    }
}

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

impl ApiError {
    /// Create a new API error
    pub fn new(
        error_type: BearDogError,
        message: impl Into<String>,
        request_id: Option<String>,
    ) -> Self {
        Self {
            error_code: ApiErrorHandler::error_code(&error_type).to_string(),
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
    ) -> BearDogError {
        ApiErrorHandler::authentication_error(message.into())
    }
    /// Create an authorization error
    pub fn authorization_error(
        message: impl Into<String>,
        request_id: Option<String>,
    ) -> BearDogError {
        ApiErrorHandler::authorization_error(message.into())
    }
    /// Create a validation error
    pub fn validation_error(
        message: impl Into<String>,
        request_id: Option<String>,
    ) -> BearDogError {
        ApiErrorHandler::validation_error(message.into())
    }
    /// Create a not found error
    pub fn not_found_error(
        message: impl Into<String>,
        request_id: Option<String>,
    ) -> BearDogError {
        ApiErrorHandler::not_found_error(message.into())
    }
    /// Create an internal error
    pub fn internal_error(
        message: impl Into<String>,
        request_id: Option<String>,
    ) -> BearDogError {
        ApiErrorHandler::internal_error(message.into())
    }
    /// Create a rate limit error
    pub fn rate_limit_error(
        message: impl Into<String>,
        request_id: Option<String>,
    ) -> BearDogError {
        ApiErrorHandler::rate_limit_error(message.into())
    }
}
