// SPDX-License-Identifier: AGPL-3.0-only

//! Audit types and traits

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    /// When the operation occurred
    pub timestamp: DateTime<Utc>,
    /// Name of the operation performed
    pub operation: String,
    /// User who initiated the operation (if authenticated)
    pub user_id: Option<String>,
    /// Key involved in the operation (if applicable)
    pub key_id: Option<String>,
    /// Outcome of the operation
    pub result: OperationResult,
    /// Additional context about the operation
    pub metadata: HashMap<String, String>,
}

impl AuditLogEntry {
    /// Create a successful audit log entry
    pub fn success(operation: impl Into<String>, key_id: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            operation: operation.into(),
            user_id: None,
            key_id: Some(key_id.into()),
            result: OperationResult::Success,
            metadata: HashMap::new(),
        }
    }

    /// Create a failed audit log entry
    pub fn failure(operation: impl Into<String>, error: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            operation: operation.into(),
            user_id: None,
            key_id: None,
            result: OperationResult::Failure(error.into()),
            metadata: HashMap::new(),
        }
    }
}

/// Operation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OperationResult {
    /// Operation completed successfully
    Success,
    /// Operation failed with the given error message
    Failure(String),
}

/// Audit log filter
#[derive(Debug, Clone, Default)]
pub struct AuditLogFilter {
    /// Filter by operation name
    pub operation: Option<String>,
    /// Filter by user identifier
    pub user_id: Option<String>,
    /// Filter by key identifier
    pub key_id: Option<String>,
    /// Filter by operation result
    pub result: Option<OperationResult>,
    /// Filter entries after this time
    pub from_time: Option<DateTime<Utc>>,
    /// Filter entries before this time
    pub to_time: Option<DateTime<Utc>>,
    /// Maximum number of entries to return
    pub limit: Option<usize>,
}

impl AuditLogFilter {
    /// Check if entry matches filter
    pub fn matches(&self, entry: &AuditLogEntry) -> bool {
        if let Some(ref op) = self.operation
            && &entry.operation != op
        {
            return false;
        }

        if let Some(ref user) = self.user_id
            && entry.user_id.as_ref() != Some(user)
        {
            return false;
        }

        if let Some(ref key) = self.key_id
            && entry.key_id.as_ref() != Some(key)
        {
            return false;
        }

        if let Some(ref from) = self.from_time
            && entry.timestamp < *from
        {
            return false;
        }

        if let Some(ref to) = self.to_time
            && entry.timestamp > *to
        {
            return false;
        }

        true
    }
}

/// Audit logger trait
#[async_trait::async_trait]
pub trait AuditLogger: Send + Sync {
    /// Record an audit log entry
    async fn log_operation(
        &self,
        operation: &AuditLogEntry,
    ) -> Result<(), beardog_errors::BearDogError>;
    /// Retrieve audit log entries matching the given filter
    async fn get_audit_log(
        &self,
        filter: &AuditLogFilter,
    ) -> Result<Vec<AuditLogEntry>, beardog_errors::BearDogError>;
}
