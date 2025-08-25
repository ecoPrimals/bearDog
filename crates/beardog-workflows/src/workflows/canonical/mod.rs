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


/// # Canonical Workflow System
///
/// **UNIFIED WORKFLOW ARCHITECTURE** - Single source of truth for all workflow operations
/// This module consolidates all workflow functionality into a canonical, unified system
/// that eliminates fragmentation and provides consistent APIs.

// Core workflow types and definitions
pub mod approval;
pub mod configuration;
pub mod core;
pub mod execution;
pub mod processing;

// Re-export all canonical types from their modules
pub use approval::{
    ApprovalRecord, ApprovalStore, InMemoryApprovalStore
};

pub use configuration::{
    ApprovalDecision,
    WorkflowEngineConfig, WorkflowPolicyConfig,
    // Re-export missing types from configuration
    ApprovalRequirements, ApprovalResponse, ApprovalSubmission, ApprovalTier, PendingApproval
};

pub use core::types::{
    CanonicalWorkflow, WorkflowId, WorkflowAuditEntry, ProcessorName, ErrorMessage
};

pub use execution::{
    WorkflowExecutionStatus, WorkflowStore, WorkflowNotificationEngine, InMemoryWorkflowStore
};

pub use processing::{
    WorkflowProcessingResult
};

// Import and re-export canonical types from beardog-types
pub use beardog_types::canonical::workflow::{WorkflowType, WorkflowPriority};

// Re-export unified status from local definition
// Import canonical WorkflowStatus instead
pub use beardog_types::canonical::workflow::WorkflowStatus;

// Create aliases for backward compatibility
pub type Workflow = CanonicalWorkflow;
pub type WorkflowApproval = ApprovalRecord;
pub type WorkflowMetrics = execution::metrics::WorkflowMetrics;

// Define concrete types to break circular references
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// **WORKFLOW REQUEST** - Canonical request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRequest {
    /// Request identifier
    pub id: String,
    /// Workflow type
    pub workflow_type: WorkflowType,
    /// Request data
    pub data: HashMap<String, serde_json::Value>,
    /// Request metadata
    pub metadata: HashMap<String, String>,
    /// Workflow priority
    pub priority: WorkflowPriority,
    /// User who initiated the workflow
    pub initiator: String,
    /// Workflow parameters
    pub parameters: HashMap<String, serde_json::Value>,
    /// Workflow description
    pub description: String,
    /// Additional properties
    pub properties: HashMap<String, serde_json::Value>,
    /// Workflow definition
    pub definition: Option<serde_json::Value>,
    /// Execution context
    pub context: Option<serde_json::Value>,
}

/// **WORKFLOW RESPONSE** - Canonical response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResponse {
    /// Response data
    pub data: HashMap<String, serde_json::Value>,
    /// Success status
    pub success: bool,
    /// Response message
    pub message: String,
    /// Workflow ID
    pub workflow_id: Option<String>,
    /// Estimated completion time
    pub estimated_completion: Option<chrono::DateTime<chrono::Utc>>,
    /// Response metadata
    pub metadata: HashMap<String, String>,
}

// Re-export missing types that are expected by other modules
pub use super::zero_cost_engine::WorkflowEngine;
pub use crate::workflows::canonical::processing::WorkflowExecution;
// WorkflowScheduler moved to processing module

// Import canonical AuditAction instead of defining duplicate
pub use beardog_types::canonical::workflow::AuditAction;
