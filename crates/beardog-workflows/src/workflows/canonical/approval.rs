

use beardog_errors::BearDogResult;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::configuration::{ApprovalDecision, WorkflowEngineConfig, WorkflowPolicyConfig};

pub use super::core::types::WorkflowId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[derive(Default)]
pub enum ApprovalStatus {

    #[default]
    Pending,

    Submitted,

    Approved,

    Rejected,

    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {

    pub id: String,

    pub workflow_id: String,

    pub approver: String,

    pub requester: Option<String>,

    pub decision: ApprovalDecision,

    pub status: ApprovalStatus,

    pub message: Option<String>,

    pub response: Option<String>,

    pub signature: Option<String>,

    pub requested_at: DateTime<Utc>,

    pub responded_at: Option<DateTime<Utc>>,

    pub expires_at: Option<DateTime<Utc>>,

    pub approver_ip: Option<String>,
}

impl ApprovalRecord {

    pub fn new(
        workflow_id: &str,
        approver_id: &str,
        decision: ApprovalDecision,
        comments: Option<&str>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            workflow_id: workflow_id.to_string(),
            approver: approver_id.to_string(),
            requester: None,
            decision,
            status: ApprovalStatus::Pending,
            message: comments.map(|s| s.to_string()),
            response: None,
            signature: None,
            requested_at: Utc::now(),
            responded_at: None,
            expires_at: None,
            approver_ip: None,
        }
    }

    pub fn with_id(
        id: &str,
        workflow_id: &str,
        approver_id: &str,
        decision: ApprovalDecision,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: id.to_string(),
            workflow_id: workflow_id.to_string(),
            approver: approver_id.to_string(),
            requester: None,
            decision,
            status: ApprovalStatus::Pending,
            message: None,
            response: None,
            signature: None,
            requested_at: now,
            responded_at: None,
            expires_at: None,
            approver_ip: None,
        }
    }
}

pub trait ApprovalStore: Send + Sync {

    fn store_approval(&self, approval: ApprovalRecord) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<()>> + Send + '_>>;

    fn get_approvals<'a>(&'a self, workflow_id: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<Vec<ApprovalRecord>>> + Send + 'a>>;

    fn update_approval(&self, approval: ApprovalRecord) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<()>> + Send + '_>>;

    fn delete_approval<'a>(&'a self, approval_id: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = BearDogResult<()>> + Send + 'a>>;
}

#[derive(Debug)]
pub struct InMemoryApprovalStore {

    pub approvals: Arc<RwLock<HashMap<String, Vec<ApprovalRecord>>>>,
}

impl InMemoryApprovalStore {
    pub fn new() -> Self {
        Self {
            approvals: Arc::new(RwLock::new(HashMap::with_capacity(16))),
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

            let mut approvals = self.approvals.write().await;
            for workflow_approvals in approvals.values_mut() {
                workflow_approvals.retain(|approval| approval.id != approval_id);
            }
            Ok(())
        })
    }
}

#[derive(Debug)]
pub struct WorkflowApprovalEngine {

    approvals: Arc<RwLock<HashMap<String, Vec<ApprovalRecord>>>>,

        config: WorkflowEngineConfig,
}

impl Default for WorkflowApprovalEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowApprovalEngine {

    pub fn new() -> Self {
        Self {
            approvals: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            config: WorkflowEngineConfig::default(),
        }
    }

    pub fn with_config(config: WorkflowEngineConfig) -> Self {
        Self {
            approvals: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            config,
        }
    }

    pub async fn request_approval(&self, approval: ApprovalRecord) -> BearDogResult<()> {
        let mut approvals = self.approvals.write().await;
        approvals.entry(approval.workflow_id.clone())
            .or_insert_with(Vec::new)
            .push(approval);
        Ok(())
    }

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

    pub async fn update_approval(&self, approval: ApprovalRecord) -> BearDogResult<()> {
        let mut approvals = self.approvals.write().await;
        if let Some(workflow_approvals) = approvals.get_mut(&approval.workflow_id) {
            if let Some(existing) = workflow_approvals.iter_mut().find(|a| a.id == approval.id) {
                *existing = approval;
            }
        }
        Ok(())
    }

    pub async fn delete_approval(&self, approval_id: &str) -> BearDogResult<()> {
        let mut approvals = self.approvals.write().await;
        approvals.remove(approval_id);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct RuleEngine {

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

pub struct ComplianceChecker {

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
