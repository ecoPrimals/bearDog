

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {

    pub id: String,

    pub timestamp: DateTime<Utc>,

    pub operation: String,

    pub key_id: Option<String>,

    pub actor: String,

    pub result: OperationResult,

    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperationResult {

    Success,

    Failure(String),

#[derive(Debug, Clone)]}

pub struct AuditLogFilter {

    pub operation: Option<String>,

    pub actor: Option<String>,

    pub result: Option<OperationResult>,

    pub from_time: Option<DateTime<Utc>>,

    pub to_time: Option<DateTime<Utc>>,

    pub limit: Option<usize>,}

impl AuditLogEntry {

    #[must_use]
    pub fn new(operation: &str, actor: &str, result: OperationResult) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            operation,
            key_id: None,
            actor,
            result,
            metadata: HashMap::with_capacity(16),
        }
    }

    pub fn with_key_id(mut self, key_id: &str) -> Self {
        self.key_id = Some(key_id);
        self

    pub fn with_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata.insert(key, value);
