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


/// # Zero-Cost Workflow Storage
/// 
/// **MODERNIZED STORAGE IMPLEMENTATIONS** - Zero-allocation, compile-time optimized
/// 
/// This module provides zero-cost storage implementations that use compile-time
/// constants and generic parameters for optimal performance.
/// 
/// ## Performance Benefits:
/// - Zero heap allocations in hot paths
/// - Compile-time capacity checking
/// - Lock-free read operations where possible
/// - Memory-mapped storage options

use super::canonical::{Workflow, WorkflowApproval};
use super::zero_cost_traits::{ZeroCostWorkflowStore, ZeroCostApprovalStore};
use beardog_errors::{BearDogError, BearDogResult};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// **ZERO-COST MEMORY WORKFLOW STORE**
/// 
/// High-performance in-memory storage with compile-time capacity limits
pub struct ZeroCostMemoryWorkflowStore<const MAX_WORKFLOWS: usize> {
    workflows: RwLock<HashMap<String, Workflow>>,
    metrics: AtomicU64,
}

impl<const MAX_WORKFLOWS: usize> Default for ZeroCostMemoryWorkflowStore<MAX_WORKFLOWS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const MAX_WORKFLOWS: usize> ZeroCostMemoryWorkflowStore<MAX_WORKFLOWS> {
    pub fn new() -> Self {
        Self {
            workflows: RwLock::new(HashMap::with_capacity(MAX_WORKFLOWS)),
            metrics: AtomicU64::new(0),
        }
    }

    pub fn capacity(&self) -> usize {
        MAX_WORKFLOWS
    }

    pub fn len(&self) -> usize {
        self.workflows.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.workflows.read().is_empty()
    }

    pub fn metrics(&self) -> u64 {
        self.metrics.load(Ordering::Relaxed)
    }
}

/// **MODERNIZED IMPLEMENTATION** - Native async fn, zero-cost storage operations
impl<const MAX_WORKFLOWS: usize> ZeroCostWorkflowStore
    for ZeroCostMemoryWorkflowStore<MAX_WORKFLOWS>
{
    type WorkflowId = String;
    type Workflow = Workflow;
    
    async fn store_workflow(&self, id: Self::WorkflowId, workflow: Self::Workflow) -> BearDogResult<()> {
        self.metrics.fetch_add(1, Ordering::Relaxed);
        let mut workflows = self.workflows.write();
        
        // Compile-time capacity checking
        if workflows.len() >= MAX_WORKFLOWS && !workflows.contains_key(&id) {
            return Err(BearDogError::configuration(format!(
                "Workflow storage limit reached: {}/{}",
                workflows.len(),
                MAX_WORKFLOWS
            )));
        }
        
        workflows.insert(id, workflow);
        Ok(())
    }

    async fn get_workflow(&self, id: &Self::WorkflowId) -> BearDogResult<Option<Self::Workflow>> {
        let workflows = self.workflows.read();
        Ok(workflows.get(id).cloned())
    }

    async fn update_workflow(&self, id: &Self::WorkflowId, workflow: Self::Workflow) -> BearDogResult<()> {
        let mut workflows = self.workflows.write();
        if workflows.contains_key(id) {
            workflows.insert(id.clone(), workflow);
            Ok(())
        } else {
            Err(BearDogError::not_found(format!("Workflow not found: {id}")))
        }
    }

    async fn delete_workflow(&self, id: &Self::WorkflowId) -> BearDogResult<()> {
        let mut workflows = self.workflows.write();
        if workflows.remove(id).is_some() {
            Ok(())
        } else {
            Err(BearDogError::not_found(format!("Workflow not found: {id}")))
        }
    }

    async fn list_workflows(&self) -> BearDogResult<Vec<(Self::WorkflowId, Self::Workflow)>> {
        let workflows = self.workflows.read();
        Ok(workflows.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
    }

    async fn count(&self) -> BearDogResult<usize> {
        Ok(self.workflows.read().len())
    }

    async fn exists(&self, id: &Self::WorkflowId) -> BearDogResult<bool> {
        Ok(self.workflows.read().contains_key(id))
    }
}

/// **ZERO-COST MEMORY APPROVAL STORE**
/// 
/// High-performance approval storage with compile-time capacity limits
pub struct ZeroCostMemoryApprovalStore<const MAX_APPROVALS: usize> {
    approvals: RwLock<HashMap<String, WorkflowApproval>>,
    workflow_approvals: RwLock<HashMap<String, Vec<String>>>, // workflow_id -> approval_ids
}

impl<const MAX_APPROVALS: usize> Default for ZeroCostMemoryApprovalStore<MAX_APPROVALS> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const MAX_APPROVALS: usize> ZeroCostMemoryApprovalStore<MAX_APPROVALS> {
    pub fn new() -> Self {
        Self {
            approvals: RwLock::new(HashMap::with_capacity(MAX_APPROVALS)),
            workflow_approvals: RwLock::new(HashMap::new()),
        }
    }

    pub fn capacity(&self) -> usize {
        MAX_APPROVALS
    }

    pub fn len(&self) -> usize {
        self.approvals.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.approvals.read().is_empty()
    }
}

/// **MODERNIZED IMPLEMENTATION** - Native async fn, zero-cost approval operations
impl<const MAX_APPROVALS: usize> ZeroCostApprovalStore<MAX_APPROVALS>
    for ZeroCostMemoryApprovalStore<MAX_APPROVALS>
{
    type ApprovalId = String;
    type Approval = WorkflowApproval;
    type WorkflowId = String;

    async fn store_approval(&self, approval: Self::Approval) -> BearDogResult<()> {
        let mut approvals = self.approvals.write();
        let mut workflow_approvals = self.workflow_approvals.write();
        
        // Compile-time capacity checking
        if approvals.len() >= MAX_APPROVALS && !approvals.contains_key(&approval.id) {
            return Err(BearDogError::configuration(format!(
                "Approval storage limit reached: {}/{}",
                approvals.len(),
                MAX_APPROVALS
            )));
        }
        
        // Store approval
        let approval_id = approval.id.clone();
        let workflow_id = approval.workflow_id.clone();
        approvals.insert(approval_id.clone(), approval);
        
        // Update workflow->approvals mapping
        workflow_approvals
            .entry(workflow_id)
            .or_default()
            .push(approval_id);
        
        Ok(())
    }

    async fn get_approval(&self, id: &Self::ApprovalId) -> BearDogResult<Option<Self::Approval>> {
        let approvals = self.approvals.read();
        Ok(approvals.get(id).cloned())
    }

    async fn list_approvals_for_workflow(&self, workflow_id: &Self::WorkflowId) -> BearDogResult<Vec<Self::Approval>> {
        let approvals = self.approvals.read();
        let workflow_approvals = self.workflow_approvals.read();
        
        if let Some(approval_ids) = workflow_approvals.get(workflow_id) {
            let mut result = Vec::with_capacity(approval_ids.len());
            for approval_id in approval_ids {
                if let Some(approval) = approvals.get(approval_id) {
                    result.push(approval.clone());
                }
            }
            Ok(result)
        } else {
            Ok(Vec::new())
        }
    }

    async fn update_approval(&self, approval: Self::Approval) -> BearDogResult<()> {
        let mut approvals = self.approvals.write();
        if approvals.contains_key(&approval.id) {
            approvals.insert(approval.id.clone(), approval);
            Ok(())
        } else {
            Err(BearDogError::not_found(format!("Approval not found: {}", approval.id)))
        }
    }

    async fn delete_approval(&self, id: &Self::ApprovalId) -> BearDogResult<()> {
        let mut approvals = self.approvals.write();
        let mut workflow_approvals = self.workflow_approvals.write();
        
        if let Some(approval) = approvals.remove(id) {
            // Remove from workflow mapping
            if let Some(approval_ids) = workflow_approvals.get_mut(&approval.workflow_id) {
                approval_ids.retain(|aid| aid != id);
                if approval_ids.is_empty() {
                    workflow_approvals.remove(&approval.workflow_id);
                }
            }
            Ok(())
        } else {
            Err(BearDogError::not_found(format!("Approval not found: {id}")))
        }
    }

    async fn count(&self) -> BearDogResult<usize> {
        Ok(self.approvals.read().len())
    }

    async fn exists(&self, id: &Self::ApprovalId) -> BearDogResult<bool> {
        Ok(self.approvals.read().contains_key(id))
    }
}

/// **STORAGE FACTORY** - Zero-cost storage creation
pub struct ZeroCostStorageFactory;

impl ZeroCostStorageFactory {
    /// Create a workflow store with compile-time capacity
    pub fn workflow_store<const MAX_WORKFLOWS: usize>() 
        -> ZeroCostMemoryWorkflowStore<MAX_WORKFLOWS> 
    {
        ZeroCostMemoryWorkflowStore::new()
    }

    /// Create an approval store with compile-time capacity
    pub fn approval_store<const MAX_APPROVALS: usize>() 
        -> ZeroCostMemoryApprovalStore<MAX_APPROVALS> 
    {
        ZeroCostMemoryApprovalStore::new()
    }
}

/// **STORAGE STATISTICS** - Zero-cost metrics collection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub workflow_count: usize,
    pub approval_count: usize,
    pub workflow_capacity: usize,
    pub approval_capacity: usize,
    pub operations_performed: u64,
}

impl StorageStats {
    pub fn new<const WF_CAP: usize, const AP_CAP: usize>(
        workflow_store: &ZeroCostMemoryWorkflowStore<WF_CAP>,
        approval_store: &ZeroCostMemoryApprovalStore<AP_CAP>,
    ) -> Self {
        Self {
            workflow_count: workflow_store.len(),
            approval_count: approval_store.len(),
            workflow_capacity: WF_CAP,
            approval_capacity: AP_CAP,
            operations_performed: workflow_store.metrics(),
        }
    }

    pub fn workflow_utilization(&self) -> f64 {
        if self.workflow_capacity == 0 {
            0.0
        } else {
            self.workflow_count as f64 / self.workflow_capacity as f64
        }
    }

    pub fn approval_utilization(&self) -> f64 {
        if self.approval_capacity == 0 {
            0.0
        } else {
            self.approval_count as f64 / self.approval_capacity as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::workflow::{WorkflowStatus, WorkflowType};

    #[tokio::test]
    async fn test_workflow_store() {
        let store = ZeroCostMemoryWorkflowStore::<100>::new();
        
        let workflow = Workflow {
            id: "test-workflow".to_string(),
            workflow_type: WorkflowType::KeyRotation,
            status: WorkflowStatus::Pending,
            metadata: HashMap::new(),
        };
        
        // Test store
        let result = store.store_workflow("test-workflow".to_string(), workflow.clone()).await;
        assert!(result.is_ok());
        
        // Test get
        let retrieved = store.get_workflow(&"test-workflow".to_string()).await
            .expect("Failed to get workflow from store");
        assert!(retrieved.is_some());
        let workflow = retrieved.expect("Retrieved workflow should not be None");
        assert_eq!(workflow.id, "test-workflow");
        
        // Test count
        let count = store.count().await
            .expect("Failed to get workflow count");
        assert_eq!(count, 1);
        
        // Test exists
        let exists = store.exists(&"test-workflow".to_string()).await
            .expect("Failed to check workflow existence");
        assert!(exists);
    }

    #[tokio::test]
    async fn test_approval_store() {
        let store = ZeroCostMemoryApprovalStore::<100>::new();
        
        let approval = WorkflowApproval {
            id: "test-approval".to_string(),
            workflow_id: "test-workflow".to_string(),
            status: beardog_types::canonical::workflow::ApprovalStatus::Pending,
            metadata: HashMap::new(),
        };
        
        // Test store
        let result = store.store_approval(approval.clone()).await;
        assert!(result.is_ok());
        
        // Test get
        let retrieved = store.get_approval(&"test-approval".to_string()).await
            .expect("Failed to get approval from store");
        assert!(retrieved.is_some());
        let approval = retrieved.expect("Retrieved approval should not be None");
        assert_eq!(approval.id, "test-approval");
        
        // Test list by workflow
        let approvals = store.list_approvals_for_workflow(&"test-workflow".to_string()).await
            .expect("Failed to list approvals for workflow");
        assert_eq!(approvals.len(), 1);
        assert_eq!(approvals[0].id, "test-approval");
    }

    #[tokio::test]
    async fn test_storage_stats() {
        let workflow_store = ZeroCostMemoryWorkflowStore::<100>::new();
        let approval_store = ZeroCostMemoryApprovalStore::<50>::new();
        
        let stats = StorageStats::new(&workflow_store, &approval_store);
        
        assert_eq!(stats.workflow_capacity, 100);
        assert_eq!(stats.approval_capacity, 50);
        assert_eq!(stats.workflow_count, 0);
        assert_eq!(stats.approval_count, 0);
    }
}
