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


/// Business Logic Error Categories
///
/// **BUSINESS AND APPLICATION LOGIC ERROR TYPES**
use crate::error_types::ErrorSeverity;
use thiserror::Error;

/// Business logic and application error types
#[derive(Error, Debug, Clone)]
pub enum BusinessError {
    /// Validation errors
    #[error("Validation error: {message}")]
    Validation { message: String },
    /// Parse errors
    #[error("Parse error: {message}")]
    Parse { message: String },
    /// Not found errors
    #[error("Not found: {message}")]
    NotFound { message: String },
    /// Already exists errors
    #[error("Already exists: {message}")]
    AlreadyExists { message: String },
    /// Conflict errors
    #[error("Conflict: {message}")]
    Conflict { message: String },
    /// Invalid input errors
    #[error("Invalid input: {message}")]
    InvalidInput { message: String },
    /// Workflow errors
    #[error("Workflow error: {message}")]
    Workflow { message: String },
    /// Compliance errors
    #[error("Compliance error: {message}")]
    Compliance { message: String },
    /// Audit errors
    #[error("Audit error: {message}")]
    Audit { message: String },
    /// Genetics operation errors
    #[error("Invalid genetics operation: {message}")]
    InvalidGenetics { message: String },
    /// Session not found errors
    #[error("Session not found: {message}")]
    SessionNotFound { message: String },
    /// Federation errors
    #[error("Federation error: {message}")]
    Federation { message: String },
    /// Unsupported operation errors
    #[error("Unsupported operation: {message}")]
    UnsupportedOperation { message: String },
    /// Service error
    #[error("Service error: {message}")]
    ServiceError { message: String },
    /// Rate limit errors
    #[error("Rate limit exceeded: {message}")]
    RateLimit { message: String },
}
impl BusinessError {
    /// Create a new validation error}


    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }
    /// Create a new parse error
    pub fn parse(message: impl Into<String>) -> Self {
        Self::Parse {
            message: message.into(),
        }
    }
    
    /// Create a new not found error
    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound {
            message: message.into(),
        }
    }
    
    /// Create a new invalid input error
    pub fn invalid_input(message: impl Into<String>) -> Self {
        Self::InvalidInput {
            message: message.into(),
        }
    }
    /// Check if this is a user error (vs system error)
    pub fn is_user_error(&self) -> bool {
        matches!(
            self,
            BusinessError::Validation { .. }
                | BusinessError::Parse { .. }
                | BusinessError::InvalidInput { .. }
                | BusinessError::NotFound { .. }
        )
    }
    
    /// Get the error severity level
    pub fn severity(&self) -> ErrorSeverity {
        match self {
            BusinessError::Compliance { .. } | BusinessError::Audit { .. } => ErrorSeverity::High,
            BusinessError::Workflow { .. } | BusinessError::Federation { .. } => {
                ErrorSeverity::Medium
            }
            _ => ErrorSeverity::Low,
        }
    }
    
    /// Get the error category for logging/metrics
    pub fn category(&self) -> &'static str {
        match self {
            BusinessError::Validation { .. } => "validation",
            BusinessError::Parse { .. } => "parse",
            BusinessError::NotFound { .. } => "not_found",
            BusinessError::AlreadyExists { .. } => "already_exists",
            BusinessError::Conflict { .. } => "conflict",
            BusinessError::InvalidInput { .. } => "invalid_input",
            BusinessError::Workflow { .. } => "workflow",
            BusinessError::Compliance { .. } => "compliance",
            BusinessError::Audit { .. } => "audit",
            BusinessError::InvalidGenetics { .. } => "invalid_genetics",
            BusinessError::SessionNotFound { .. } => "session_not_found",
            BusinessError::Federation { .. } => "federation",
            BusinessError::UnsupportedOperation { .. } => "unsupported_operation",
            BusinessError::ServiceError { .. } => "service_error",
            BusinessError::RateLimit { .. } => "rate_limit",
        }
    }
}
