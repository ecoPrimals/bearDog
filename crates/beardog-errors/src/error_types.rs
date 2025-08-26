

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the severity level of an error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    /// Critical errors that require immediate attention and may cause system failure
    Critical,
    /// High priority errors that significantly impact functionality
    High,
    /// Medium priority errors that may affect some functionality
    Medium,
    /// Low priority errors that have minimal impact
    Low,
    /// Informational errors for logging and debugging purposes
    Info,
}

/// Categorizes errors by their functional domain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// Security-related errors including authentication and authorization
    Security,
    /// Network connectivity and communication errors
    Network,
    /// Data storage and persistence errors
    Storage,
    /// Configuration and setup errors
    Configuration,
    /// System resource and infrastructure errors
    Resource,
    /// External service and dependency errors
    External,
    /// Input validation and data format errors
    Validation,
    /// Internal system and logic errors
    Internal,
    /// Temporary errors that may resolve automatically
    Transient,
    /// System initialization and startup errors
    Initialization,
}

/// Represents a specific remediation action for an error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationAction {
    /// The type of remediation action to take
    pub action_type: String,
    /// Human-readable description of the action
    pub description: String,
    /// Parameters needed to execute the action
    pub parameters: HashMap<String, serde_json::Value>,
    /// Estimated time in seconds to complete the action
    pub estimated_time_seconds: Option<u64>,
    /// Whether this action can be automated
    pub automatable: bool,
    /// Prerequisites that must be met before executing this action
    pub prerequisites: Vec<String>,
}

/// Contextual information about when and where an error occurred
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Unique identifier for this error instance
    pub error_id: String,
    /// Timestamp when the error occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Component or module where the error originated
    pub component: String,
    /// Operation that was being performed when the error occurred
    pub operation: String,
    /// User ID associated with the operation, if applicable
    pub user_id: Option<String>,
    /// Request ID for tracing, if applicable
    pub request_id: Option<String>,
    /// Additional metadata about the error context
    pub metadata: HashMap<String, serde_json::Value>,
    /// Stack trace information, if available
    pub stack_trace: Option<String>,
}

/// Enhanced error information with rich context and remediation guidance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedErrorInfo {
    /// Unique error code for this type of error
    pub error_code: String,
    /// Severity level of the error
    pub severity: ErrorSeverity,
    /// Functional category of the error
    pub category: ErrorCategory,
    /// Human-readable error message
    pub message: String,
    /// Technical details for debugging
    pub technical_details: Option<String>,
    /// Contextual information about the error
    pub context: ErrorContext,
    /// Suggested remediation actions
    pub remediation_actions: Vec<RemediationAction>,
    /// Whether this error condition can be retried
    pub retryable: bool,
    /// Suggested delay before retry, if retryable
    pub retry_after_seconds: Option<u64>,
    /// Related error IDs for correlation
    pub related_errors: Vec<String>,
}
