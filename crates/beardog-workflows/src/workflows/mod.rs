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


/// # BearDog Workflows - Canonical Unified System
///
/// **UNIFIED WORKFLOW ARCHITECTURE** - Single source of truth for all workflow operations
/// This module provides the unified workflow system that eliminates fragmentation
/// and provides a single source of truth for all workflow operations.

// Canonical workflow system (primary module)
pub mod canonical;

// Status definitions - now using canonical types from beardog-types
// pub mod status; // REMOVED - use beardog_types::canonical::workflow::{WorkflowStatus, AuditAction}

// Workflow type definitions (for compatibility)
pub mod types;

// Storage implementations
pub mod storage;

// Zero-cost implementations for high performance
pub mod zero_cost_engine;
pub mod zero_cost_processors;  
pub mod zero_cost_storage;
pub mod zero_cost_traits;
pub mod zero_cost_workflows;

// Traditional workflow handlers (being migrated to canonical)
pub mod handlers;

// Notification system
pub mod notification;

// Re-export ALL canonical types for unified access
// Import handlers from their correct location
pub use handlers::{WorkflowHandler, WorkflowScheduler};

pub use canonical::{
    // Core workflow types
    CanonicalWorkflow, Workflow, WorkflowId, WorkflowAuditEntry, ProcessorName, ErrorMessage,
    
    // Approval system
    ApprovalDecision, ApprovalRecord, ApprovalRequirements, ApprovalResponse,
    ApprovalStore, ApprovalSubmission, ApprovalTier, InMemoryApprovalStore, PendingApproval,
    
    // Execution system  
    WorkflowEngine, WorkflowExecution, WorkflowExecutionStatus, WorkflowStore,
    WorkflowNotificationEngine, InMemoryWorkflowStore,
    
    // Processing system  
    WorkflowProcessingResult,
    
    // Configuration
    WorkflowEngineConfig, WorkflowPolicyConfig,
    
    // Audit actions
    AuditAction,
    
    // Canonical types from beardog-types (re-exported through canonical)
    WorkflowType, WorkflowPriority,
    
    // Metrics
    WorkflowMetrics,
};

// Re-export status from local module
// Import canonical WorkflowStatus instead
pub use beardog_types::canonical::workflow::WorkflowStatus;

// Re-export request/response types from types module
pub use types::{WorkflowRequest, WorkflowResponse};

// Re-export zero-cost implementations
pub use zero_cost_engine::{
    ZeroCostWorkflowEngine, ProductionWorkflowEngine, DevelopmentWorkflowEngine
};

pub use zero_cost_processors::{
    ZeroCostKeyRotationProcessor, ZeroCostPolicyChangeProcessor
};

pub use zero_cost_storage::{
    ZeroCostMemoryWorkflowStore, ZeroCostMemoryApprovalStore
};

pub use zero_cost_traits::{
    ZeroCostWorkflowStore, ZeroCostApprovalStore, ZeroCostWorkflowProcessor
};
