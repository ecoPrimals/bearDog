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


/// Canonical Workflow Approval Types
///
/// **MODERNIZED** - Approval types with native async traits
/// This module contains approval-related types using zero-cost abstractions.
// MODERNIZED: Using native async fn in traits instead of async_trait
use beardog_errors::BearDogResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::configuration::{ApprovalDecision, WorkflowEngineConfig, WorkflowPolicyConfig};
// Use canonical WorkflowId type from core_types
pub use super::core::types::WorkflowId;
/// Canonical approval status - unified definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum ApprovalStatus {
    /// Approval is pending
    #[default]
    Pending,
    /// Approval has been submitted
    Submitted,
    /// Approval has been granted
    Approved,
    /// Approval has been denied/rejected
    Rejected,
    /// Approval has expired
    Expired,
}
/// Unified canonical approval record supporting both old and new schemas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    /// Unique approval identifier
    pub id: String,
    /// Associated workflow ID (using String for compatibility)
    pub workflow_id: String,
    /// User who can approve this request (canonical field)
    pub approver: String,
    /// Legacy approver_id field for backward compatibility
    pub approver_id: String,
    /// User who requested the approval
    pub requester: Option<String>,
    /// Approval decision (old schema compatibility)
    pub decision: ApprovalDecision,
    /// Approval status (new canonical field)
    pub status: ApprovalStatus,
    /// Approval request message
    pub message: Option<String>,
    /// Approval response/reason
    pub response: Option<String>,
    /// Legacy reason field
    pub reason: Option<String>,
    /// Legacy comments field
    pub comments: Option<String>,
    /// Digital signature
    pub signature: Option<String>,
    /// When the approval was requested
    pub requested_at: DateTime<Utc>,
    /// Legacy timestamp field
    pub timestamp: DateTime<Utc>,
    /// When the approval was responded to
    pub responded_at: Option<DateTime<Utc>>,
    /// Legacy decided_at field
    pub decided_at: DateTime<Utc>,
    /// Approval expiration time
    pub expires_at: Option<DateTime<Utc>>,
    /// Approver IP address
    pub approver_ip: Option<String>,
}


impl ApprovalRecord {
    /// Create a new approval record with unified fields
    pub fn new(
        workflow_id: String,
        approver_id: String,
        decision: ApprovalDecision,
        comments: Option<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            workflow_id,
            approver: approver_id.clone(),
            approver_id,
            requester: None,
            decision,
            status: ApprovalStatus::Pending,
            message: comments.clone(),
            response: None,
            reason: None,
            comments,
            signature: None,
            requested_at: Utc::now(),
            timestamp: Utc::now(),
            responded_at: None,
            decided_at: Utc::now(),
            expires_at: None,
            approver_ip: None,
        }
    }
}


    pub fn new(
        id: String,
        workflow_id: String,
        approver_id: String,
        decision: ApprovalDecision,
    ) -> ApprovalRecord {
        let now = Utc::now();
        let status = match decision {
            ApprovalDecision::Pending => ApprovalStatus::Pending,
            ApprovalDecision::Submitted => ApprovalStatus::Submitted,
            ApprovalDecision::Approved => ApprovalStatus::Approved,
            ApprovalDecision::Rejected => ApprovalStatus::Rejected,
        };
        ApprovalRecord {
            id,
            workflow_id: workflow_id.clone(),
            approver: approver_id.clone(),
            approver_id,
            requester: None,
            decision,
            status,
            message: None,
            response: None,
            reason: None,
            comments: None,
            signature: None,
            requested_at: now,
            timestamp: now,
            responded_at: None,
            decided_at: now,
            expires_at: None,
            approver_ip: None,
        }
    }

/// Trait for approval storage backends - **DYN-COMPATIBLE** ✅
/// 
/// **MODERNIZED**: Uses `impl Future` pattern for dyn compatibility
/// This allows the trait to be used with `Arc<dyn ApprovalStore>`
pub trait ApprovalStore: Send + Sync {
    /// Store an approval
    fn store_approval(&self, approval: ApprovalRecord) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<()>> + Send + '_>>;
    
    /// Get approvals for a workflow
    fn get_approvals<'a>(&'a self, workflow_id: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<Vec<ApprovalRecord>>> + Send + 'a>>;
    
    /// Update an approval
    fn update_approval(&self, approval: ApprovalRecord) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<()>> + Send + '_>>;
    
    /// Delete approvals for a workflow
    fn delete_approval<'a>(&'a self, approval_id: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<()>> + Send + 'a>>;
}

/// In-memory approval store implementation  
#[derive(Debug)]
pub struct InMemoryApprovalStore {
    /// Stored approvals
    pub approvals: Arc<RwLock<HashMap<String, Vec<ApprovalRecord>>>>,
}


impl InMemoryApprovalStore {
    pub fn new() -> Self {
        Self {
            approvals: Arc::new(RwLock::new(HashMap::new())),
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
            let workflow_approvals = approvals
                .entry(approval.workflow_id.clone())
                .or_insert_with(Vec::new);
            workflow_approvals.push(approval);
            Ok(())
        })
    }

    fn get_approvals<'a>(&'a self, workflow_id: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<Vec<ApprovalRecord>>> + Send + 'a>> {
        Box::pin(async move {
            let approvals = self.approvals.read().await;
            Ok(approvals.get(workflow_id).cloned().unwrap_or_default())
        })
    }
    
    fn update_approval(&self, approval: ApprovalRecord) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<()>> + Send + '_>> {
        Box::pin(async move {
            let mut approvals = self.approvals.write().await;
            if let Some(workflow_approvals) = approvals.get_mut(&approval.workflow_id) {
                if let Some(existing) = workflow_approvals.iter_mut().find(|a| a.id == approval.id) {
                    *existing = approval;
                } else {
                    workflow_approvals.push(approval);
                }
            } else {
                approvals.insert(approval.workflow_id.clone(), vec![approval]);
            }
            Ok(())
        })
    }

    fn delete_approval<'a>(&'a self, approval_id: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<()>> + Send + 'a>> {
        Box::pin(async move {
            // Find and remove the approval with the given ID across all workflows
            let mut approvals = self.approvals.write().await;
            for workflow_approvals in approvals.values_mut() {
                workflow_approvals.retain(|approval| approval.id != approval_id);
            }
            Ok(())
        })
    }
}

/// Workflow approval engine for managing approval workflows
#[derive(Debug)]
pub struct WorkflowApprovalEngine {
    /// Approval records by workflow ID
    approvals: Arc<RwLock<HashMap<String, Vec<ApprovalRecord>>>>,
    /// Engine configuration
    #[allow(dead_code)]
    config: WorkflowEngineConfig,
}

impl Default for WorkflowApprovalEngine {
    fn default() -> Self {
        Self::new()
    }
}



impl WorkflowApprovalEngine {
    /// Create a new approval engine with optional config
    pub fn new() -> Self {
        Self {
            approvals: Arc::new(RwLock::new(HashMap::new())),
            config: WorkflowEngineConfig::default(),
        }
    }

    /// Create a new approval engine with specific config
    pub fn with_config(config: WorkflowEngineConfig) -> Self {
        Self {
            approvals: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Request approval for a workflow
    pub async fn request_approval(&self, approval: ApprovalRecord) -> BearDogResult<()> {
        let mut approvals = self.approvals.write().await;
        approvals.entry(approval.workflow_id.clone())
            .or_insert_with(Vec::new)
            .push(approval);
        Ok(())
    }

    /// Get approvals for a workflow
    pub async fn get_approvals(&self, workflow_id: &str) -> BearDogResult<Vec<ApprovalRecord>> {
        let approvals = self.approvals.read().await;
        let workflow_approvals: Vec<ApprovalRecord> = approvals
            .values()
            .filter(|approvals| !approvals.is_empty())
            .flat_map(|approvals| approvals.iter())
            .filter(|approval| approval.workflow_id == workflow_id)
            .cloned()
            .collect();
        Ok(workflow_approvals)
    }

    /// Update an approval record
    pub async fn update_approval(&self, approval: ApprovalRecord) -> BearDogResult<()> {
        let mut approvals = self.approvals.write().await;
        if let Some(workflow_approvals) = approvals.get_mut(&approval.workflow_id) {
            if let Some(existing) = workflow_approvals.iter_mut().find(|a| a.id == approval.id) {
                *existing = approval;
            }
        }
        Ok(())
    }

    /// Delete an approval by ID
    pub async fn delete_approval(&self, approval_id: &str) -> BearDogResult<()> {
        let mut approvals = self.approvals.write().await;
        approvals.remove(approval_id);
        Ok(())
    }
}
/// Rule engine for workflow policies
#[derive(Debug, Clone)]
pub struct RuleEngine {
    /// Rule configuration
    pub config: WorkflowPolicyConfig,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self {
            config: WorkflowPolicyConfig::default(),
        }
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Compliance checker for regulatory requirements
pub struct ComplianceChecker {
    /// Compliance configuration
    pub config: WorkflowPolicyConfig,
}

impl ComplianceChecker {
    pub fn new() -> Self {
        Self {
            config: WorkflowPolicyConfig::default(),
        }
    }
}

impl Default for ComplianceChecker {
    fn default() -> Self {
        Self::new()
    }
}
