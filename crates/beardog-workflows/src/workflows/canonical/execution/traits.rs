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


/// # Canonical Workflow Execution Traits
///
/// **UNIFIED TRAIT SYSTEM** for the BearDog workflow execution
/// This module provides zero-cost async traits that replace async_trait usage.

use beardog_errors::BearDogResult;

use super::super::core::types::{CanonicalWorkflow, WorkflowAuditEntry};

/// **CANONICAL** Workflow storage trait - **DYN-COMPATIBLE** ✅
/// 
/// **MODERNIZED**: Uses `impl Future` pattern for dyn compatibility
/// This allows the trait to be used with `Arc<dyn WorkflowStore>`
pub trait WorkflowStore: Send + Sync {
    /// Store a workflow
    fn store_workflow(&self, workflow: CanonicalWorkflow) -> impl std::future::Future<Output = BearDogResult<()>> + Send;

    /// Get a workflow by ID
    fn get_workflow(&self, workflow_id: &str) -> impl std::future::Future<Output = BearDogResult<Option<CanonicalWorkflow>>> + Send;

    /// Update an existing workflow
    fn update_workflow(&self, workflow: CanonicalWorkflow) -> impl std::future::Future<Output = BearDogResult<()>> + Send;

    /// Delete a workflow by ID
    fn delete_workflow(&self, workflow_id: &str) -> impl std::future::Future<Output = BearDogResult<()>> + Send;

    /// List all workflows
    fn list_workflows(&self) -> impl std::future::Future<Output = BearDogResult<Vec<CanonicalWorkflow>>> + Send;

    /// Store an audit entry
    fn store_audit_entry(&self, entry: WorkflowAuditEntry) -> impl std::future::Future<Output = BearDogResult<()>> + Send;
}

/// **CANONICAL** Workflow notification engine trait
/// **ZERO-COST ASYNC OPTIMIZATION** - Native async methods eliminate boxing overhead
#[allow(async_fn_in_trait)]
pub trait WorkflowNotificationEngine: Send + Sync {
    /// Notify that a workflow has been created
    async fn notify_workflow_created(&self, workflow: &CanonicalWorkflow) -> BearDogResult<()>;

    /// Notify that a workflow has been completed
    async fn notify_workflow_completed(&self, workflow: &CanonicalWorkflow) -> BearDogResult<()>;

    /// Notify that a workflow has failed
    async fn notify_workflow_failed(&self, workflow: &CanonicalWorkflow, error: &str) -> BearDogResult<()>;

    /// Notify that approval is required
    async fn notify_approval_required(&self, workflow: &CanonicalWorkflow) -> BearDogResult<()>;

    /// Notify that approval has been granted
    async fn notify_approval_granted(&self, workflow: &CanonicalWorkflow, approver: &str) -> BearDogResult<()>;
} 