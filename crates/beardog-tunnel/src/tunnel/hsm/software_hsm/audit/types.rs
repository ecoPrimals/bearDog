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


/// Audit Types for Software HSM
///
/// Defines the core audit logging types for the software HSM.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Audit log entry for HSM operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    /// Unique entry ID
    pub id: String,
    /// Timestamp of the operation
    pub timestamp: DateTime<Utc>,
    /// Operation that was performed
    pub operation: String,
    /// Key ID involved (if applicable)
    pub key_id: Option<String>,
    /// User or system that performed the operation
    pub actor: String,
    /// Result of the operation
    pub result: OperationResult,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}
/// Result of an HSM operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OperationResult {
    /// Operation succeeded
    Success,
    /// Operation failed with error message
    Failure(String),
/// Filter for querying audit logs
#[derive(Debug, Clone)]}


pub struct AuditLogFilter {
    /// Filter by operation type
    pub operation: Option<String>,
    /// Filter by key ID
    /// Filter by actor
    pub actor: Option<String>,
    /// Filter by result type
    pub result: Option<OperationResult>,
    /// Filter by time range (start)
    pub from_time: Option<DateTime<Utc>>,
    /// Filter by time range (end)
    pub to_time: Option<DateTime<Utc>>,
    /// Maximum number of entries to return
    pub limit: Option<usize>,}


impl AuditLogEntry {
    /// Create a new audit log entry}


    #[must_use]
    pub fn new(operation: String, actor: String, result: OperationResult) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            operation,
            key_id: None,
            actor,
            result,
            metadata: HashMap::new(),
        }
    }
    /// Set the key ID for this entry
    pub fn with_key_id(mut self, key_id: String) -> Self {
        self.key_id = Some(key_id);
        self
    /// Add metadata to this entry}


    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
