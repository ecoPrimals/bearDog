

use beardog_errors::BearDogResult;
use beardog_types::canonical::workflow::WorkflowType;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[allow(async_fn_in_trait)]
pub trait ZeroCostWorkflowStore: Send + Sync {

    type WorkflowId: Clone + Send + Sync;

    type Workflow: Clone + Send + Sync;

    async fn store_workflow(&self, id: Self::WorkflowId, workflow: Self::Workflow) -> BearDogResult<()>;

    async fn get_workflow(&self, id: &Self::WorkflowId) -> BearDogResult<Option<Self::Workflow>>;

    async fn update_workflow(&self, id: &Self::WorkflowId, workflow: Self::Workflow) -> BearDogResult<()>;

    async fn delete_workflow(&self, id: &Self::WorkflowId) -> BearDogResult<()>;

    async fn list_workflows(&self) -> BearDogResult<Vec<(Self::WorkflowId, Self::Workflow)>>;

    async fn count(&self) -> BearDogResult<usize>;

    async fn exists(&self, id: &Self::WorkflowId) -> BearDogResult<bool>;
}

#[allow(async_fn_in_trait)]
pub trait ZeroCostApprovalStore<const CAPACITY: usize>: Send + Sync {

    type ApprovalId: Clone + Send + Sync;

    type Approval: Clone + Send + Sync;

    type WorkflowId: Clone + Send + Sync;

    async fn store_approval(&self, approval: Self::Approval) -> BearDogResult<()>;

    async fn get_approval(&self, id: &Self::ApprovalId) -> BearDogResult<Option<Self::Approval>>;

    async fn update_approval(&self, approval: Self::Approval) -> BearDogResult<()>;

    async fn delete_approval(&self, id: &Self::ApprovalId) -> BearDogResult<()>;

    async fn list_approvals_for_workflow(&self, workflow_id: &Self::WorkflowId) -> BearDogResult<Vec<Self::Approval>>;

    async fn count(&self) -> BearDogResult<usize>;

    async fn exists(&self, id: &Self::ApprovalId) -> BearDogResult<bool>;
}

#[allow(async_fn_in_trait)]
pub trait ZeroCostWorkflowProcessor: Send + Sync {

    type Request: Send + Sync;

    type Response: Send + Sync;

    type Context: Send + Sync;

    async fn process(&self, request: Self::Request, context: Self::Context) -> BearDogResult<Self::Response>;

    async fn validate(&self, request: &Self::Request) -> BearDogResult<()>;

    fn capabilities(&self) -> &'static [&'static str];

    fn name(&self) -> &'static str;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowProcessingRequest<T> {

    pub id: String,

    pub workflow_type: WorkflowType,

    pub payload: T,

    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowProcessingResponse<T> {

    pub id: String,

    pub success: bool,

    pub data: Option<T>,

    pub error: Option<String>,

    pub duration_ms: u64,

    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowProcessingContext {

    pub user_id: Option<String>,

    pub session_id: Option<String>,

    pub source: String,

    pub environment: String,

    pub metadata: HashMap<String, String>,
}

impl Default for WorkflowProcessingContext {
    fn default() -> Self {
        Self {
            user_id: None,
            session_id: None,
            source: "system".to_string(),
            environment: "production".to_string(),
            metadata: HashMap::with_capacity(16),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostEngineStats {

    pub workflows_processed: u64,

    pub avg_processing_time_ms: f64,

    pub active_workflows: usize,

    pub error_rate: f64,

    pub memory_usage_bytes: usize,

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
