

use crate::workflows::canonical::{
    ApprovalRecord, ApprovalStore, WorkflowStore,
    CanonicalWorkflow,
};
use beardog_errors::BearDogResult;
use beardog_types::canonical::workflow::WorkflowAuditEntry;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct InMemoryWorkflowStore {
    workflows: Arc<RwLock<HashMap<String, CanonicalWorkflow>>>,
}

impl InMemoryWorkflowStore {
    pub fn new() -> Self {
        Self {
            workflows: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }
    }
}

impl Default for InMemoryWorkflowStore {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowStore for InMemoryWorkflowStore {
    async fn store_workflow(&self, workflow: CanonicalWorkflow) -> BearDogResult<()> {
        let mut workflows = self.workflows.write().await;
        workflows.insert(workflow.id.clone(), workflow);
        Ok(())
    }

    async fn get_workflow(&self, workflow_id: &str) -> BearDogResult<Option<CanonicalWorkflow>> {
        let workflows = self.workflows.read().await;
        Ok(workflows.get(workflow_id).cloned())
    }

    async fn update_workflow(&self, workflow: CanonicalWorkflow) -> BearDogResult<()> {
        let mut workflows = self.workflows.write().await;
        workflows.insert(workflow.id.clone(), workflow);
        Ok(())
    }

    async fn delete_workflow(&self, workflow_id: &str) -> BearDogResult<()> {
        let mut workflows = self.workflows.write().await;
        workflows.remove(workflow_id);
        Ok(())
    }

    async fn list_workflows(&self) -> BearDogResult<Vec<CanonicalWorkflow>> {
        let workflows = self.workflows.read().await;
        Ok(workflows.values().cloned().collect())
    }

    async fn store_audit_entry(&self, entry: WorkflowAuditEntry) -> BearDogResult<()> {

        tracing::info!(
            "Storing workflow audit entry: {} - {} for workflow {}",
            entry.id,
            entry.action,
            entry.workflow_id
        );

        Ok(())
    }
}

#[derive(Debug)]
pub struct InMemoryApprovalStore {
    approvals: Arc<RwLock<HashMap<String, ApprovalRecord>>>,
    workflow_approvals: Arc<RwLock<HashMap<String, Vec<String>>>>, // workflow_id -> approval_ids
}

impl InMemoryApprovalStore {
    pub fn new() -> Self {
        Self {
            approvals: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            workflow_approvals: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        }
    }
}

impl Default for InMemoryApprovalStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ApprovalStore for InMemoryApprovalStore {
    fn store_approval(&self, approval: ApprovalRecord) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<()>> + Send + '_>> {
        Box::pin(async move {
        let mut approvals = self.approvals.write().await;
        let mut workflow_approvals = self.workflow_approvals.write().await;

        approvals.insert(approval.id.clone(), approval.clone());

        workflow_approvals
            .entry(approval.workflow_id.clone())
            .or_insert_with(Vec::new)
            .push(approval.id.clone());
        
        Ok(())
        })
    }

    fn get_approvals<'a>(&'a self, workflow_id: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<Vec<ApprovalRecord>>> + Send + 'a>> {
        Box::pin(async move {
        let approvals = self.approvals.read().await;
        let workflow_approvals = self.workflow_approvals.read().await;
        
        if let Some(approval_ids) = workflow_approvals.get(workflow_id) {
            let mut results = Vec::new();
            for approval_id in approval_ids {
                if let Some(approval) = approvals.get(approval_id) {
                    results.push(approval.clone());
                }
            }
            Ok(results)
        } else {
            Ok(Vec::new())
        }
        })
    }

    fn update_approval(&self, approval: ApprovalRecord) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<()>> + Send + '_>> {
        Box::pin(async move {
        let mut approvals = self.approvals.write().await;
        approvals.insert(approval.id.clone(), approval);
        Ok(())
        })
    }

    fn delete_approval<'a>(&'a self, approval_id: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<()>> + Send + 'a>> {
        Box::pin(async move {
        let mut approvals = self.approvals.write().await;
        let mut workflow_approvals = self.workflow_approvals.write().await;

        if let Some(approval) = approvals.remove(approval_id) {

            if let Some(approval_ids) = workflow_approvals.get_mut(&approval.workflow_id) {
                approval_ids.retain(|id| id != approval_id);
            }
        }
        
        Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_workflow_store_operations() -> BearDogResult<()> {
        let store = InMemoryWorkflowStore::new();
        let workflow = CanonicalWorkflow {
            context: std::collections::HashMap::with_capacity(16),
            id: "test-workflow".to_string(),
            workflow_type: beardog_types::canonical::workflow::WorkflowType::KeyRotation,
            status: WorkflowStatus::Pending,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            approval_requirements: Default::default(),
            audit_trail: Vec::new(),
            metadata: HashMap::with_capacity(16),
            target: "System".to_string(),
            priority: beardog_types::canonical::workflow::WorkflowPriority::Normal,
            expires_at: None,
            requested_by: "test-user".to_string(),
            initiator: "test-user".to_string(),
            description: Some("Test workflow".to_string()),
            timeout_duration: chrono::chrono::Duration::hours(24),
            properties: HashMap::with_capacity(16),
            parameters: HashMap::with_capacity(16),
            approvals: Vec::new(),
        };

        store.store_workflow(workflow).await?;
        let retrieved = store.get_workflow("test-workflow").await?;
        assert!(retrieved.is_some());

        store
            .update_workflow(CanonicalWorkflow {
                context: std::collections::HashMap::with_capacity(16),
                id: "test-workflow".to_string(),
                workflow_type: beardog_types::canonical::workflow::WorkflowType::KeyRotation,
                status: WorkflowStatus::Approved,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                approval_requirements: Default::default(),
                audit_trail: Vec::new(),
                metadata: HashMap::with_capacity(16),
                target: "System".to_string(),
                priority: beardog_types::canonical::workflow::WorkflowPriority::Normal,
                expires_at: None,
                requested_by: "test-user".to_string(),
                initiator: "test-user".to_string(),
                description: Some("Test workflow".to_string()),
                timeout_duration: chrono::chrono::Duration::hours(24),
                properties: HashMap::with_capacity(16),
                parameters: HashMap::with_capacity(16),
                approvals: Vec::new(),
            })
            .await?;
        let updated = store.get_workflow("test-workflow").await?;
        let workflow = updated.expect("Updated workflow should exist");
        assert_eq!(workflow.status, WorkflowStatus::Approved);
        
        Ok(())
    }
}

impl WorkflowStore for Arc<InMemoryWorkflowStore> {
    async fn store_workflow(&self, workflow: CanonicalWorkflow) -> BearDogResult<()> {
        (**self).store_workflow(workflow).await
    }

    async fn get_workflow(&self, workflow_id: &str) -> BearDogResult<Option<CanonicalWorkflow>> {
        (**self).get_workflow(workflow_id).await
    }

    fn update_workflow(&self, workflow: CanonicalWorkflow) -> impl std::future::Future<Output = BearDogResult<()>> + Send {
        (**self).update_workflow(workflow)
    }

    async fn delete_workflow(&self, workflow_id: &str) -> BearDogResult<()> {
        (**self).delete_workflow(workflow_id).await
    }

    async fn list_workflows(&self) -> BearDogResult<Vec<CanonicalWorkflow>> {
        (**self).list_workflows().await
    }

    async fn store_audit_entry(&self, entry: WorkflowAuditEntry) -> BearDogResult<()> {
        (**self).store_audit_entry(entry).await
    }

}
