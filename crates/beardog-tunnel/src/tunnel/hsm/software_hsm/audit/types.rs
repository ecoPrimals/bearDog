//! Audit types and traits

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: DateTime<Utc>,
    pub operation: String,
    pub user_id: Option<String>,
    pub key_id: Option<String>,
    pub result: OperationResult,
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
    Success,
    Failure(String),
}

/// Audit log filter
#[derive(Debug, Clone, Default)]
pub struct AuditLogFilter {
    pub operation: Option<String>,
    pub user_id: Option<String>,
    pub key_id: Option<String>,
    pub result: Option<OperationResult>,
    pub from_time: Option<DateTime<Utc>>,
    pub to_time: Option<DateTime<Utc>>,
    pub limit: Option<usize>,
}

impl AuditLogFilter {
    /// Check if entry matches filter
    pub fn matches(&self, entry: &AuditLogEntry) -> bool {
        if let Some(ref op) = self.operation {
            if &entry.operation != op {
                return false;
            }
        }

        if let Some(ref user) = self.user_id {
            if entry.user_id.as_ref() != Some(user) {
                return false;
            }
        }

        if let Some(ref key) = self.key_id {
            if entry.key_id.as_ref() != Some(key) {
                return false;
            }
        }

        if let Some(ref from) = self.from_time {
            if entry.timestamp < *from {
                return false;
            }
        }

        if let Some(ref to) = self.to_time {
            if entry.timestamp > *to {
                return false;
            }
        }

        true
    }
}

/// Audit logger trait
#[async_trait::async_trait]
pub trait AuditLogger: Send + Sync {
    async fn log_operation(
        &self,
        operation: &AuditLogEntry,
    ) -> Result<(), beardog_errors::BearDogError>;
    async fn get_audit_log(
        &self,
        filter: &AuditLogFilter,
    ) -> Result<Vec<AuditLogEntry>, beardog_errors::BearDogError>;
}
