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


/// # Zero-Cost Workflow Traits
///
/// **ZERO-COST ABSTRACTION TRAITS** - Compile-time optimized workflow interfaces
/// 
/// This module provides zero-cost async traits that eliminate runtime overhead
/// through compile-time specialization and monomorphization.

use beardog_errors::BearDogResult;
use beardog_types::canonical::workflow::WorkflowType;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// **ZERO-COST WORKFLOW STORE** - Generic storage interface
/// 
/// This trait provides compile-time specialized storage operations with no runtime overhead.
/// Different implementations can be optimized for specific use cases.
#[allow(async_fn_in_trait)]
pub trait ZeroCostWorkflowStore: Send + Sync {
    /// Workflow identifier type
    type WorkflowId: Clone + Send + Sync;
    /// Workflow data type
    type Workflow: Clone + Send + Sync;
    
    /// Store a workflow with zero-cost async
    async fn store_workflow(&self, id: Self::WorkflowId, workflow: Self::Workflow) -> BearDogResult<()>;
    
    /// Get a workflow by ID
    async fn get_workflow(&self, id: &Self::WorkflowId) -> BearDogResult<Option<Self::Workflow>>;
    
    /// Update an existing workflow
    async fn update_workflow(&self, id: &Self::WorkflowId, workflow: Self::Workflow) -> BearDogResult<()>;
    
    /// Delete a workflow by ID
    async fn delete_workflow(&self, id: &Self::WorkflowId) -> BearDogResult<()>;
    
    /// List all workflows
    async fn list_workflows(&self) -> BearDogResult<Vec<(Self::WorkflowId, Self::Workflow)>>;
    
    /// Get workflow count
    async fn count(&self) -> BearDogResult<usize>;
    
    /// Check if workflow exists
    async fn exists(&self, id: &Self::WorkflowId) -> BearDogResult<bool>;
}

/// **ZERO-COST APPROVAL STORE** - Generic approval storage interface
#[allow(async_fn_in_trait)]
pub trait ZeroCostApprovalStore<const CAPACITY: usize>: Send + Sync {
    /// Approval identifier type
    type ApprovalId: Clone + Send + Sync;
    /// Approval data type
    type Approval: Clone + Send + Sync;
    /// Workflow identifier type
    type WorkflowId: Clone + Send + Sync;
    
    /// Store an approval
    async fn store_approval(&self, approval: Self::Approval) -> BearDogResult<()>;
    
    /// Get an approval by ID
    async fn get_approval(&self, id: &Self::ApprovalId) -> BearDogResult<Option<Self::Approval>>;
    
    /// Update an approval
    async fn update_approval(&self, approval: Self::Approval) -> BearDogResult<()>;
    
    /// Delete an approval
    async fn delete_approval(&self, id: &Self::ApprovalId) -> BearDogResult<()>;
    
    /// List approvals for a workflow
    async fn list_approvals_for_workflow(&self, workflow_id: &Self::WorkflowId) -> BearDogResult<Vec<Self::Approval>>;
    
    /// Get approval count
    async fn count(&self) -> BearDogResult<usize>;
    
    /// Check if approval exists
    async fn exists(&self, id: &Self::ApprovalId) -> BearDogResult<bool>;
}

/// **ZERO-COST WORKFLOW PROCESSOR** - Generic processing interface
#[allow(async_fn_in_trait)]
pub trait ZeroCostWorkflowProcessor: Send + Sync {
    /// Request type for processing
    type Request: Send + Sync;
    /// Response type from processing
    type Response: Send + Sync;
    /// Context type for processing
    type Context: Send + Sync;
    
    /// Process a workflow request
    async fn process(&self, request: Self::Request, context: Self::Context) -> BearDogResult<Self::Response>;
    
    /// Validate a request before processing
    async fn validate(&self, request: &Self::Request) -> BearDogResult<()>;
    
    /// Get processor capabilities
    fn capabilities(&self) -> &'static [&'static str];
    
    /// Get processor name
    fn name(&self) -> &'static str;
}

/// **WORKFLOW PROCESSING REQUEST** - Generic request structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowProcessingRequest<T> {
    /// Request identifier
    pub id: String,
    /// Workflow type
    pub workflow_type: WorkflowType,
    /// Request payload
    pub payload: T,
    /// Request metadata
    pub metadata: HashMap<String, String>,
}

/// **WORKFLOW PROCESSING RESPONSE** - Generic response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowProcessingResponse<T> {
    /// Response identifier
    pub id: String,
    /// Processing success flag
    pub success: bool,
    /// Response data
    pub data: Option<T>,
    /// Error message if failed
    pub error: Option<String>,
    /// Processing duration in milliseconds
    pub duration_ms: u64,
    /// Response metadata
    pub metadata: HashMap<String, String>,
}

/// **WORKFLOW PROCESSING CONTEXT** - Generic context structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowProcessingContext {
    /// User context
    pub user_id: Option<String>,
    /// Session context
    pub session_id: Option<String>,
    /// Request source
    pub source: String,
    /// Processing environment
    pub environment: String,
    /// Context metadata
    pub metadata: HashMap<String, String>,
}

impl Default for WorkflowProcessingContext {
    fn default() -> Self {
        Self {
            user_id: None,
            session_id: None,
            source: "system".to_string(),
            environment: "production".to_string(),
            metadata: HashMap::new(),
        }
    }
}

/// **ZERO-COST ENGINE STATISTICS** - Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostEngineStats {
    /// Total workflows processed
    pub workflows_processed: u64,
    /// Average processing time in milliseconds
    pub avg_processing_time_ms: f64,
    /// Current active workflows
    pub active_workflows: usize,
    /// Error rate (percentage)
    pub error_rate: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: usize,
    /// Engine uptime in seconds
    pub uptime_seconds: u64,
}

impl Default for ZeroCostEngineStats {
    fn default() -> Self {
        Self {
            workflows_processed: 0,
            avg_processing_time_ms: 0.0,
            active_workflows: 0,
            error_rate: 0.0,
            memory_usage_bytes: 0,
            uptime_seconds: 0,
        }
    }
}
