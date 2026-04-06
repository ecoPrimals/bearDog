// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Enhanced Error Module
//!
//! This module contains the main EnhancedBearDogError type and core error handling functionality.

use crate::core::BearDogError;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::{ErrorAnalytics, ErrorContext, ErrorRecovery, ErrorRemediation};

/// **ENHANCED UNIFIED ERROR** - The ultimate error type for BearDog
///
/// This enhanced error type provides comprehensive error handling with rich context,
/// automatic recovery mechanisms, error analytics, and advanced remediation capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedBearDogError {
    /// Core error information
    pub core: BearDogError,

    /// Error context and metadata
    pub context: ErrorContext,

    /// Error recovery information
    pub recovery: ErrorRecovery,

    /// Error analytics and metrics
    pub analytics: ErrorAnalytics,

    /// Error remediation suggestions
    pub remediation: ErrorRemediation,
}

impl EnhancedBearDogError {
    /// Create a new enhanced error from a core error
    #[must_use]
    pub fn new(core: BearDogError) -> Self {
        Self {
            core,
            context: ErrorContext::new(),
            recovery: ErrorRecovery::new(),
            analytics: ErrorAnalytics::new(),
            remediation: ErrorRemediation::new(),
        }
    }

    #[must_use]
    /// Create an enhanced error with context
    pub fn with_context(core: BearDogError, context: ErrorContext) -> Self {
        Self {
            core,
            context,
            recovery: ErrorRecovery::new(),
            analytics: ErrorAnalytics::new(),
            remediation: ErrorRemediation::new(),
        }
    }
 #[must_use]

    /// Create an enhanced error with full configuration
    pub fn with_full_context(
        core: BearDogError,
        context: ErrorContext,
        recovery: ErrorRecovery,
        analytics: ErrorAnalytics,
        remediation: ErrorRemediation,
    ) -> Self {
        Self {
            core,
            context,
            recovery,
            analytics,
            remediation,
        }
    }

    /// Add correlation ID for error tracking
    #[must_use]
    pub fn with_correlation_id(mut self, correlation_id: String) -> Self {
        self.context.correlation_id = correlation_id;
        self
    }

    /// Add component information
    #[must_use]
    pub fn with_component(mut self, component: String) -> Self {
        self.context.component = component;
        self
    }

    /// Add function information
    #[must_use]
    pub fn with_function(mut self, function: String) -> Self {
        self.context.function = function;
        self
    }

    #[must_use]
    /// Add metadata to the error context
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.context.metadata.insert(key, value);
        self
    }

    /// Check if the error is recoverable
    pub fn is_recoverable(&self) -> bool {
        self.recovery.is_recoverable
    }

    /// Get recovery suggestions
    #[must_use]
    pub fn get_recovery_suggestions(&self) -> &[String] {
        &self.recovery.suggestions
    }

    #[must_use]
    /// Get error severity level
    pub fn severity(&self) -> ErrorSeverity {
        self.analytics.severity
    }
    /// Get error category
    #[must_use]
    pub fn category(&self) -> &str {
        &self.analytics.category
    }

    /// Get remediation actions
    #[must_use]
    pub fn get_remediation_actions(&self) -> &[RemediationAction] {
        &self.remediation.actions
    }

    /// Convert to a standard BearDogError
    pub fn into_core(self) -> BearDogError {
        self.core
    }

    /// Get a reference to the core error
    pub fn core(&self) -> &BearDogError {
        &self.core
    }

    /// Get error message with context
    pub fn detailed_message(&self) -> String {
        format!(
            "{} (component: {}, correlation_id: {})",
            self.core, self.context.component, self.context.correlation_id
        )
    }
}

/// **ERROR SEVERITY** - Classification of error severity
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Low severity - informational
    Low,
    /// Medium severity - warning
    Medium,
    /// High severity - error
    High,
    /// Critical severity - system failure
    Critical,
}

impl Default for ErrorSeverity {
    fn default() -> Self {
        Self::Medium
    }
}

/// **REMEDIATION ACTION** - Specific actions to resolve errors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemediationAction {
    /// Action identifier
    pub id: String,
    /// Human-readable description
    pub description: String,
    /// Action type
    pub action_type: RemediationActionType,
    /// Estimated time to complete
    pub estimated_duration: Option<std::time::Duration>,
    /// Prerequisites for this action
    pub prerequisites: Vec<String>,
}

/// **REMEDIATION ACTION TYPE** - Types of remediation actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemediationActionType {
    /// Automatic remediation
    Automatic,
    /// Manual intervention required
    Manual,
    /// Configuration change needed
    Configuration,
    /// System restart required
    Restart,
    /// External service dependency
    External,
}

// Display implementation for enhanced error
impl fmt::Display for EnhancedBearDogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.detailed_message())
    }
}

// Error trait implementation
impl std::error::Error for EnhancedBearDogError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.core)
    }
}

// Conversion from BearDogError
impl From<BearDogError> for EnhancedBearDogError {
    fn from(core: BearDogError) -> Self {
        Self::new(core)
    }
}

// Conversion to BearDogError
impl From<EnhancedBearDogError> for BearDogError {
    fn from(enhanced: EnhancedBearDogError) -> Self {
        enhanced.core
    }
}

impl Default for RemediationAction {
    fn default() -> Self {
        Self {
            id: "unknown".to_string(),
            description: "No specific remediation available".to_string(),
            action_type: RemediationActionType::Manual,
            estimated_duration: None,
            prerequisites: Vec::new(),
        }
    }
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "LOW"),
            Self::Medium => write!(f, "MEDIUM"),
            Self::High => write!(f, "HIGH"),
            Self::Critical => write!(f, "CRITICAL"),
        }
    }
}
