

use beardog_errors::BearDogResult;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::super::core::types::{CanonicalWorkflow, WorkflowAuditEntry};
use super::super::execution::traits::WorkflowStore;

#[derive(Debug)]
pub struct InMemoryWorkflowStore {

    workflows: Arc<RwLock<HashMap<String, CanonicalWorkflow>>>,

    audit_entries: Arc<RwLock<Vec<WorkflowAuditEntry>>>,
}

impl InMemoryWorkflowStore {

    pub fn new() -> Self {
        Self {
            workflows: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            audit_entries: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn workflow_count(&self) -> usize {
        let workflows = self.workflows.read().await;
        workflows.len()
    }

    pub async fn audit_entry_count(&self) -> usize {
        let entries = self.audit_entries.read().await;
        entries.len()
    }

    pub async fn clear_workflows(&self) {
        let mut workflows = self.workflows.write().await;
        workflows.clear();
    }

    pub async fn clear_audit_entries(&self) {
        let mut entries = self.audit_entries.write().await;
        entries.clear();
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
        let mut entries = self.audit_entries.write().await;
        entries.push(entry);
        Ok(())
    }
} 