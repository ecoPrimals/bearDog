

use crate::BearDogError; // Use canonical definition from lib.rs
use crate::error_types::{
    EnhancedErrorInfo, ErrorCategory, ErrorContext, ErrorSeverity, RemediationAction,
};
use std::collections::HashMap;

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

    pub fn new(error_code: &str) -> Self {
        Self {
            error_code,
            severity: ErrorSeverity::Medium,
            category: ErrorCategory::Internal,
            message: String::with_capacity(64),
            technical_details: None,
            component: "unknown".to_string(),
            operation: None,
            user_id: None,
            request_id: None,
            metadata: HashMap::with_capacity(16),
            remediation_actions: Vec::new(),
            retryable: false,
            retry_after_seconds: None,
            related_errors: Vec::new(),
        }
    }

    pub fn severity(mut self, severity: ErrorSeverity) -> Self {
        self.severity = severity;
        self
    }

    pub fn category(mut self, category: ErrorCategory) -> Self {
        self.category = category;
        self
    }

    pub fn message(mut self, message: impl Into<&str>) -> Self {
        self.message = message.into();
        self
    }

    pub fn technical_details(mut self, details: impl Into<&str>) -> Self {
        self.technical_details = Some(details.into());
        self
    }

    pub fn component(mut self, component: impl Into<&str>) -> Self {
        self.component = component.into();
        self
    }

    pub fn operation(mut self, operation: impl Into<&str>) -> Self {
        self.operation = Some(operation.into());
        self
    }

    pub fn user_id(mut self, user_id: impl Into<&str>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn request_id(mut self, request_id: impl Into<&str>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    pub fn metadata(mut self, key: impl Into<&str>, value: serde_json::Value) -> Self {
        self.metadata.insert(key.into(), value);
        self
    }

    pub fn remediation_action(mut self, action: RemediationAction) -> Self {
        self.remediation_actions.push(action);
        self
    }

    pub fn retryable(mut self, retryable: bool) -> Self {
        self.retryable = retryable;
        self
    }

    pub fn retry_after(mut self, seconds: u64) -> Self {
        self.retry_after_seconds = Some(seconds);
        self
    }

    pub fn related_error(mut self, error_code: impl Into<&str>) -> Self {
        self.related_errors.push(error_code.into());
        self
    }

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
