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


/// Workflow type definitions
///
/// This module contains all type definitions for the workflow system,
/// unified to use canonical types from beardog-types.
// Re-export canonical types from beardog-types
pub use beardog_types::canonical::workflow::WorkflowType;

// Local type definitions
pub mod enums;
// Re-export main types for backwards compatibility from the main canonical module
pub use crate::workflows::canonical::{
    ApprovalDecision, ApprovalRecord, ApprovalRequirements, ApprovalResponse, ApprovalStore,
    ApprovalSubmission, ApprovalTier, PendingApproval, Workflow, WorkflowAuditEntry,
    WorkflowEngine, WorkflowEngineConfig, WorkflowExecution, WorkflowPolicyConfig,
    WorkflowRequest, WorkflowResponse, WorkflowStatus, WorkflowStore,
};
pub use enums::{
    ApprovalStatus, AuditAction, ExecutionStatus, WorkflowExecutionState, WorkflowPriority,
    WorkflowTarget,
};
// Core traits and common types
// async_trait removed - using canonical async patterns
