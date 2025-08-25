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


/// Error builder patterns for BearDogError
use crate::BearDogError; // Use canonical definition from lib.rs
use crate::error_types::{
    EnhancedErrorInfo, ErrorCategory, ErrorContext, ErrorSeverity, RemediationAction,
};
use std::collections::HashMap;

/// Builder for creating enhanced AI-friendly errors
pub struct EnhancedErrorBuilder {
    error_code: String,
    severity: ErrorSeverity,
    category: ErrorCategory,
    message: String,
    technical_details: Option<String>,
    component: String,
    operation: Option<String>,
    user_id: Option<String>,
    request_id: Option<String>,
    metadata: HashMap<String, serde_json::Value>,
    remediation_actions: Vec<RemediationAction>,
    retryable: bool,
    retry_after_seconds: Option<u64>,
    related_errors: Vec<String>,
}
impl EnhancedErrorBuilder {
    /// Create a new builder with the given error code}


    pub fn new(error_code: String) -> Self {
        Self {
            error_code,
            severity: ErrorSeverity::Medium,
            category: ErrorCategory::Internal,
            message: String::new(),
            technical_details: None,
            component: "unknown".to_string(),
            operation: None,
            user_id: None,
            request_id: None,
            metadata: HashMap::new(),
            remediation_actions: Vec::new(),
            retryable: false,
            retry_after_seconds: None,
            related_errors: Vec::new(),
        }
    }
    /// Set the error severity
    pub fn severity(mut self, severity: ErrorSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Set the error category
    pub fn category(mut self, category: ErrorCategory) -> Self {
        self.category = category;
        self
    }

    /// Set the error message
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Set technical details
    pub fn technical_details(mut self, details: impl Into<String>) -> Self {
        self.technical_details = Some(details.into());
        self
    }

    /// Set the component where error occurred
    pub fn component(mut self, component: impl Into<String>) -> Self {
        self.component = component.into();
        self
    }

    /// Set the operation being performed
    pub fn operation(mut self, operation: impl Into<String>) -> Self {
        self.operation = Some(operation.into());
        self
    }

    /// Set the user ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// Set the request ID
    pub fn request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// Add metadata
    pub fn metadata(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }

    /// Add a remediation action
    pub fn remediation_action(mut self, action: RemediationAction) -> Self {
        self.remediation_actions.push(action);
        self
    }

    /// Set if the operation is retryable
    pub fn retryable(mut self, retryable: bool) -> Self {
        self.retryable = retryable;
        self
    }

    /// Set retry delay in seconds
    pub fn retry_after(mut self, seconds: u64) -> Self {
        self.retry_after_seconds = Some(seconds);
        self
    }

    /// Add a related error
    pub fn related_error(mut self, error_code: impl Into<String>) -> Self {
        self.related_errors.push(error_code.into());
        self
    }

    /// Build the enhanced error}


    pub fn build(self) -> BearDogError {
        let context = ErrorContext {
            timestamp: chrono::Utc::now(),
            component: self.component,
            operation: self.operation,
            user_id: self.user_id,
            request_id: self.request_id,
            metadata: self.metadata,
            stack_trace: None, // Could be populated with backtrace if needed
        };
        
        let info = EnhancedErrorInfo {
            error_code: self.error_code.clone(),
            severity: self.severity,
            category: self.category,
            message: self.message,
            technical_details: self.technical_details,
            context,
            remediation_actions: self.remediation_actions,
            retryable: self.retryable,
            retry_after_seconds: self.retry_after_seconds,
            related_errors: self.related_errors,
        };
        
        BearDogError::Enhanced {
            info: Box::new(info),
        }
    }
}
