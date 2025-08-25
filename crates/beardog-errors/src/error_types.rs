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


/// Basic error types and enums for `BearDog` error handling
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Error severity levels for AI decision making
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ErrorSeverity {
    /// Critical: System unusable, immediate intervention required
    Critical,
    /// High: Major functionality impaired, urgent attention needed
    High,
    /// Medium: Some functionality impaired, should be addressed
    Medium,
    /// Low: Minor issues, can be deferred
    Low,
    /// Info: Informational, no action required
    Info,
}
/// Error categories for AI pattern recognition and handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorCategory {
    /// Authentication and authorization errors
    Security,
    /// Network and connectivity issues
    Network,
    /// Data persistence and storage issues
    Storage,
    /// Configuration and setup problems
    Configuration,
    /// Resource exhaustion (memory, CPU, disk)
    Resource,
    /// External service dependencies
    External,
    /// User input validation
    Validation,
    /// Internal system logic errors
    Internal,
    /// Temporary conditions that may resolve
    Transient,
    /// System initialization and startup errors
    Initialization,
}

/// AI-actionable remediation suggestions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationAction {
    /// Action type identifier for AI recognition
    pub action_type: String,
    /// Human-readable description
    pub description: String,
    /// Specific parameters for the action
    pub parameters: HashMap<String, serde_json::Value>,
    /// Estimated time to resolve (in seconds)
    pub estimated_time_seconds: Option<u64>,
    /// Whether this action can be automated
    pub automatable: bool,
    /// Prerequisites for this action
    pub prerequisites: Vec<String>,
}

/// Rich error context for AI understanding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Unique error occurrence ID for tracking
    pub error_id: String,
    /// Timestamp when error occurred
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Component where error originated
    pub component: String,
    /// Operation that was being performed
    pub operation: String,
    /// User ID if applicable
    pub user_id: Option<String>,
    /// Request ID for tracing
    pub request_id: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Stack trace if available
    pub stack_trace: Option<String>,
}

/// Enhanced error context for AI understanding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedErrorInfo {
    /// Structured error code (e.g., "AUTH_001", "NET_502")
    pub error_code: String,
    /// Severity level
    pub severity: ErrorSeverity,
    /// Error category
    pub category: ErrorCategory,
    /// Human-readable message
    pub message: String,
    /// Technical details for debugging
    pub technical_details: Option<String>,
    /// Rich context information
    pub context: ErrorContext,
    /// Suggested remediation actions
    pub remediation_actions: Vec<RemediationAction>,
    /// Whether error is retryable
    pub retryable: bool,
    /// Retry strategy if retryable
    pub retry_after_seconds: Option<u64>,
    /// Related error codes for pattern analysis
    pub related_errors: Vec<String>,
}
