

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::network::HttpStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ApiErrorHandler;

impl ApiErrorHandler {

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

    pub fn authentication_error(message: impl Into<&str>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::Authentication)
    }

    pub fn authorization_error(message: impl Into<&str>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::Authorization)
    }

    pub fn validation_error(message: impl Into<&str>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::Validation)
    }

    pub fn not_found_error(message: impl Into<&str>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::NotFound)
    }

    pub fn conflict_error(message: impl Into<&str>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::Conflict)
    }

    pub fn rate_limit_error(message: impl Into<&str>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::RateLimit)
    }

    pub fn internal_error(message: impl Into<&str>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::Internal)
    }

    pub fn service_unavailable_error(message: impl Into<&str>) -> BearDogError {
        BearDogError::api(message.into(), beardog_errors::ApiErrorCategory::ServiceUnavailable)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {

    pub error_code: String,

    pub message: String,

    pub details: Option<HashMap<String, String>>,

    pub request_id: Option<String>,

    pub timestamp: String,
}

impl ApiError {

    pub fn new(
        error_type: BearDogError,
        message: impl Into<&str>,
        request_id: Option<&str>,
    ) -> Self {
        Self {
            error_code: ApiErrorHandler::error_code(&error_type).to_string(),
            message: message.into(),
            details: None,
            request_id,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn with_details(mut self, details: HashMap<&str, &str>) -> Self {
        self.details = Some(details);
        self
    }

    pub fn with_detail(mut self, key: impl Into<&str>, value: impl Into<&str>) -> Self {
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

pub mod helpers {
    use super::*;

    pub fn authentication_error(
        message: impl Into<&str>,
        request_id: Option<&str>,
    ) -> BearDogError {
        ApiErrorHandler::authentication_error(message.into())
    }

    pub fn authorization_error(
        message: impl Into<&str>,
        request_id: Option<&str>,
    ) -> BearDogError {
        ApiErrorHandler::authorization_error(message.into())
    }

    pub fn validation_error(
        message: impl Into<&str>,
        request_id: Option<&str>,
    ) -> BearDogError {
        ApiErrorHandler::validation_error(message.into())
    }

    pub fn not_found_error(
        message: impl Into<&str>,
        request_id: Option<&str>,
    ) -> BearDogError {
        ApiErrorHandler::not_found_error(message.into())
    }

    pub fn internal_error(
        message: impl Into<&str>,
        request_id: Option<&str>,
    ) -> BearDogError {
        ApiErrorHandler::internal_error(message.into())
    }

    pub fn rate_limit_error(
        message: impl Into<&str>,
        request_id: Option<&str>,
    ) -> BearDogError {
        ApiErrorHandler::rate_limit_error(message.into())
    }
}
