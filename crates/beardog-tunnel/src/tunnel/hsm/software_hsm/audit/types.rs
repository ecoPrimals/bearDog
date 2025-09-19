

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
    pub timestamp: DateTime<Utc>,

    /// The operation value
    pub operation: String,


    pub key_id: Option<String>,

    /// The actor value
    pub actor: String,

    /// The result value
    pub result: OperationResult,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
    /// Optional actor
    pub actor: Option<String>,

    /// Optional result
    pub result: Option<OperationResult>,


    pub from_time: Option<DateTime<Utc>>,


    pub to_time: Option<DateTime<Utc>>,

    /// Optional limit
    pub limit: Option<usize>,}

impl AuditLogEntry {

/// New operation.
    /// Creates a new instance
    pub fn new(&str, actor: &str, result: OperationResult) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(None,
            actor,
            result,
            metadata: HashMap::with_capacity(16),
        }
    }

/// With Key Id operation.
    /// Creates instance with key id
    pub fn with_key_id(mut self, key_id: &str) -> Self {
        self.key_id = Some(&str, value: &str) -> Self {
        self.metadata.insert(key.to_string(), value.into());
