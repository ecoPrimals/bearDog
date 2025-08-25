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


/// Core Workflow Types - Fundamental Structures
///
/// This module contains the core workflow types that form the foundation
/// of the canonical workflow system.

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::workflow::{WorkflowPriority, WorkflowType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use super::approval::{ApprovalRecord, ApprovalRequirements};
use super::status::WorkflowStatus;

// Zero-copy optimization: Use Arc<str> for commonly reused strings
pub type ProcessorName = Arc<str>;
pub type WorkflowId = Arc<str>;
pub type ErrorMessage = Arc<str>;

/// Canonical workflow definition - single source of truth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalWorkflow {
    /// Unique workflow identifier
    pub id: String,
    /// Workflow type classification
    pub workflow_type: WorkflowType,
    /// Current workflow status
    pub status: WorkflowStatus,
    /// Workflow priority level
    pub priority: WorkflowPriority,
    /// Workflow description
    pub description: Option<String>,
    /// Target system or resource
    pub target: String,
    /// User who initiated the workflow
    pub initiator: String,
    /// User who requested the workflow
    pub requested_by: String,
    /// Workflow expiration timestamp
    pub expires_at: Option<DateTime<Utc>>,
    /// Timeout duration for workflow execution
    pub timeout_duration: Option<Duration>,
    /// Workflow creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// Workflow metadata
    pub metadata: HashMap<String, String>,
    /// Workflow parameters for execution
    pub parameters: HashMap<String, serde_json::Value>,
    /// Additional workflow properties
    pub properties: HashMap<String, serde_json::Value>,
    /// Execution context
    pub context: WorkflowContext,
    /// Approval requirements for this workflow
    pub approval_requirements: Option<ApprovalRequirements>,
    /// Current approvals for this workflow
    pub approvals: Vec<ApprovalRecord>,
    /// Audit trail for this workflow
    pub audit_trail: Vec<WorkflowAuditEntry>,
}

/// Workflow execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowContext {
    /// User who initiated the workflow
    pub initiated_by: String,
    /// Target system or resource
    pub target: String,
    /// Additional context data
    pub data: HashMap<String, serde_json::Value>,
}

impl Default for WorkflowContext {
    fn default() -> Self {
        Self {
            initiated_by: "system".to_string(),
            target: "default".to_string(),
            data: HashMap::new(),
        }
    }
}

/// Workflow result information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {
    /// Success status
    pub success: bool,
    /// Result message
    pub message: String,
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

// Import canonical AuditAction instead of defining duplicate
pub use beardog_types::canonical::workflow::AuditAction;

// Import canonical WorkflowAuditEntry instead of defining duplicate
pub use beardog_types::canonical::workflow::WorkflowAuditEntry;

/// Workflow request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRequest {
    /// Workflow type
    pub workflow_type: WorkflowType,
    /// User who initiated the request
    pub initiator: String,
    /// Request description
    pub description: Option<String>,
    /// Request parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Request properties
    pub properties: HashMap<String, serde_json::Value>,
    /// Request priority
    pub priority: WorkflowPriority,
    /// Request metadata
    pub metadata: HashMap<String, String>,
}

/// Workflow response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResponse {
    /// Response status
    pub success: bool,
    /// Response message
    pub message: String,
    /// Workflow ID
    pub workflow_id: String,
    /// Estimated completion time
    pub estimated_completion: Option<DateTime<Utc>>,
    /// Response data
    pub data: Option<serde_json::Value>,
} 