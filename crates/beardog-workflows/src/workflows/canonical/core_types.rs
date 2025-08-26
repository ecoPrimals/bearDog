

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::workflow::{WorkflowPriority, WorkflowType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

use super::approval::{ApprovalRecord, ApprovalRequirements};
use super::status::WorkflowStatus;

pub type ProcessorName = Arc<str>;
pub type WorkflowId = Arc<str>;
pub type ErrorMessage = Arc<str>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanonicalWorkflow {

    pub id: String,

    pub workflow_type: WorkflowType,

    pub status: WorkflowStatus,

    pub priority: WorkflowPriority,

    pub description: Option<String>,

    pub target: String,

    pub initiator: String,

    pub requested_by: String,

    pub expires_at: Option<DateTime<Utc>>,

    pub timeout_duration: Option<Duration>,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,

    pub metadata: HashMap<String, String>,

    pub parameters: HashMap<String, serde_json::Value>,

    pub properties: HashMap<String, serde_json::Value>,

    pub context: WorkflowContext,

    pub approval_requirements: Option<ApprovalRequirements>,

    pub approvals: Vec<ApprovalRecord>,

    pub audit_trail: Vec<WorkflowAuditEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowContext {

    pub initiated_by: String,

    pub target: String,

    pub data: HashMap<String, serde_json::Value>,
}

impl Default for WorkflowContext {
    fn default() -> Self {
        Self {
            initiated_by: "system".to_string(),
            target: "default".to_string(),
            data: HashMap::with_capacity(16),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResult {

    pub success: bool,

    pub message: String,

    pub metadata: HashMap<String, serde_json::Value>,
}

pub use beardog_types::canonical::workflow::AuditAction;

pub use beardog_types::canonical::workflow::WorkflowAuditEntry;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRequest {

    pub workflow_type: WorkflowType,

    pub initiator: String,

    pub description: Option<String>,

    pub parameters: HashMap<String, serde_json::Value>,

    pub properties: HashMap<String, serde_json::Value>,

    pub priority: WorkflowPriority,

    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowResponse {

    pub success: bool,

    pub message: String,

    pub workflow_id: String,

    pub estimated_completion: Option<DateTime<Utc>>,

    pub data: Option<serde_json::Value>,
}

// Implement ZeroCostWorkflowStore for CanonicalWorkflow
impl crate::workflows::zero_cost_traits::ZeroCostWorkflowStore for CanonicalWorkflow {
    type WorkflowId = String;
    type Workflow = CanonicalWorkflow;

    async fn store_workflow(&self, _id: Self::WorkflowId, _workflow: Self::Workflow) -> BearDogResult<()> {
        // Placeholder implementation - would store in actual backend
        Ok(())
    }

    async fn get_workflow(&self, _id: &Self::WorkflowId) -> BearDogResult<Option<Self::Workflow>> {
        // Placeholder implementation - would retrieve from actual backend
        Ok(Some(self.clone()))
    }

    async fn update_workflow(&self, _id: &Self::WorkflowId, _workflow: Self::Workflow) -> BearDogResult<()> {
        // Placeholder implementation - would update in actual backend
        Ok(())
    }

    async fn delete_workflow(&self, _id: &Self::WorkflowId) -> BearDogResult<()> {
        // Placeholder implementation - would delete from actual backend
        Ok(())
    }

    async fn list_workflows(&self) -> BearDogResult<Vec<(Self::WorkflowId, Self::Workflow)>> {
        // Placeholder implementation - would list from actual backend
        Ok(vec![(self.id.clone(), self.clone())])
    }

    async fn count(&self) -> BearDogResult<usize> {
        // Placeholder implementation - would count from actual backend
        Ok(1)
    }
} 