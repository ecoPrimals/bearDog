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


/// # Canonical In-Memory Workflow Storage
///
/// **UNIFIED STORAGE IMPLEMENTATION** for the BearDog workflow system
/// This module provides in-memory storage for workflows and audit entries.

use beardog_errors::BearDogResult;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::super::core::types::{CanonicalWorkflow, WorkflowAuditEntry};
use super::super::execution::traits::WorkflowStore;

/// **CANONICAL** In-memory workflow store implementation
#[derive(Debug)]
pub struct InMemoryWorkflowStore {
    /// Stored workflows
    workflows: Arc<RwLock<HashMap<String, CanonicalWorkflow>>>,
    /// Stored audit entries
    audit_entries: Arc<RwLock<Vec<WorkflowAuditEntry>>>,
}

impl InMemoryWorkflowStore {
    /// Create a new in-memory workflow store
    pub fn new() -> Self {
        Self {
            workflows: Arc::new(RwLock::new(HashMap::new())),
            audit_entries: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Get the number of stored workflows
    pub async fn workflow_count(&self) -> usize {
        let workflows = self.workflows.read().await;
        workflows.len()
    }

    /// Get the number of stored audit entries
    pub async fn audit_entry_count(&self) -> usize {
        let entries = self.audit_entries.read().await;
        entries.len()
    }

    /// Clear all stored workflows
    pub async fn clear_workflows(&self) {
        let mut workflows = self.workflows.write().await;
        workflows.clear();
    }

    /// Clear all stored audit entries
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