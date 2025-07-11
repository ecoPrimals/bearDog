//! Storage implementations for workflows and approvals
//! 
//! Contains in-memory and persistent storage implementations for workflow data.

use super::types::*;
use crate::BearDogResult;

use std::collections::HashMap;
use tokio::sync::RwLock;
use futures::future::{BoxFuture, FutureExt};

impl Default for InMemoryWorkflowStore {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryWorkflowStore {
    /// Create a new in-memory workflow store
    pub fn new() -> Self {
        Self {
            workflows: RwLock::new(HashMap::new()),
        }
    }
}

impl WorkflowStore for InMemoryWorkflowStore {
    fn store_workflow(&self, workflow: &Workflow) -> BoxFuture<'_, BearDogResult<()>> {
        let workflow = workflow.clone();
        async move {
            let mut store = self.workflows.write().await;
            store.insert(workflow.id.clone(), workflow);
            Ok(())
        }.boxed()
    }

    fn get_workflow(&self, workflow_id: &str) -> BoxFuture<'_, BearDogResult<Option<Workflow>>> {
        let workflow_id = workflow_id.to_string();
        async move {
            let store = self.workflows.read().await;
            Ok(store.get(&workflow_id).cloned())
        }.boxed()
    }

    fn update_workflow(&self, workflow: &Workflow) -> BoxFuture<'_, BearDogResult<()>> {
        let workflow = workflow.clone();
        async move {
            let mut store = self.workflows.write().await;
            store.insert(workflow.id.clone(), workflow);
            Ok(())
        }.boxed()
    }

    fn list_workflows(&self, status: Option<WorkflowStatus>) -> BoxFuture<'_, BearDogResult<Vec<Workflow>>> {
        async move {
            let store = self.workflows.read().await;
            let mut workflows: Vec<Workflow> = store.values().cloned().collect();
            
            if let Some(filter_status) = status {
                workflows.retain(|w| w.status == filter_status);
            }
            
            // Sort by created_at descending
            workflows.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            
            Ok(workflows)
        }.boxed()
    }
}

impl Default for InMemoryApprovalStore {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryApprovalStore {
    /// Create a new in-memory approval store
    pub fn new() -> Self {
        Self {
            approvals: RwLock::new(HashMap::new()),
        }
    }
}

impl ApprovalStore for InMemoryApprovalStore {
    fn store_approval(&self, approval: &ApprovalRecord) -> BoxFuture<'_, BearDogResult<()>> {
        let approval = approval.clone();
        async move {
            let mut store = self.approvals.write().await;
            store.entry(approval.workflow_id.clone())
                .or_insert_with(Vec::new)
                .push(approval);
            Ok(())
        }.boxed()
    }

    fn get_approvals(&self, workflow_id: &str) -> BoxFuture<'_, BearDogResult<Vec<ApprovalRecord>>> {
        let workflow_id = workflow_id.to_string();
        async move {
            let store = self.approvals.read().await;
            Ok(store.get(&workflow_id).cloned().unwrap_or_default())
        }.boxed()
    }

    fn list_pending_approvals(&self, approver: &str) -> BoxFuture<'_, BearDogResult<Vec<PendingApproval>>> {
        let _ = approver;
        async move {
            // In a real implementation, this would query pending approvals
            // For now, return empty list as this is just in-memory storage
            Ok(Vec::new())
        }.boxed()
    }
} 