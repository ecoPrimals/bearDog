

pub mod workflows;

pub use workflows::{

    CanonicalWorkflow, Workflow, WorkflowId, WorkflowAuditEntry,

    ApprovalDecision, ApprovalRecord, ApprovalRequirements, ApprovalResponse,
    ApprovalStore, ApprovalSubmission, ApprovalTier, InMemoryApprovalStore, PendingApproval,

    WorkflowEngine, WorkflowExecution, WorkflowExecutionStatus, WorkflowStore,
    WorkflowNotificationEngine, WorkflowScheduler, InMemoryWorkflowStore,

    WorkflowHandler, WorkflowProcessingResult,

    WorkflowEngineConfig, WorkflowPolicyConfig,

    WorkflowStatus, WorkflowMetrics, AuditAction,

    WorkflowRequest, WorkflowResponse,

    WorkflowType, WorkflowPriority,
};

pub use workflows::{
    zero_cost_engine::{ZeroCostWorkflowEngine, ProductionWorkflowEngine, DevelopmentWorkflowEngine},
    zero_cost_processors::{ZeroCostKeyRotationProcessor, ZeroCostPolicyChangeProcessor},
    zero_cost_storage::{ZeroCostMemoryWorkflowStore, ZeroCostMemoryApprovalStore},
    zero_cost_traits::{ZeroCostWorkflowStore, ZeroCostApprovalStore, ZeroCostWorkflowProcessor},
    zero_cost_engine::WorkflowEngineInterface,
};

pub const WORKFLOW_SYSTEM_VERSION: &str = "2.0.0";

// Simple processor for compilation
pub struct DefaultWorkflowProcessor;

impl crate::workflows::zero_cost_traits::ZeroCostWorkflowProcessor for DefaultWorkflowProcessor {
    type Request = String;
    type Response = String;
    type Context = ();

    async fn process(&self, request: Self::Request, _context: Self::Context) -> beardog_errors::BearDogResult<Self::Response> {
        Ok(format!("Processed: {}", request))
    }

    async fn validate(&self, _request: &Self::Request) -> beardog_errors::BearDogResult<()> {
        Ok(())
    }

    fn capabilities(&self) -> &'static [&'static str] {
        &["basic"]
    }

    fn name(&self) -> &'static str {
        "DefaultWorkflowProcessor"
    }
}

pub struct BearDogWorkflowSystem<E = crate::workflows::zero_cost_engine::ZeroCostWorkflowEngine<
    crate::workflows::zero_cost_storage::ZeroCostMemoryWorkflowStore<1000>,
    crate::workflows::zero_cost_storage::ZeroCostMemoryApprovalStore<1000>, 
    DefaultWorkflowProcessor
>> {
    engine: E,
}

impl<T> BearDogWorkflowSystem<T>
where
    T: Send,
{
    pub fn with_engine(engine: T) -> Self {
        Self { engine }
    }
    
    pub async fn execute_workflow<W>(&self, workflow: W) -> beardog_errors::BearDogResult<()>
    where
        W: Send,
    {
        // Implementation placeholder
        Ok(())
    }
}

impl BearDogWorkflowSystem {
    pub async fn new() -> beardog_errors::BearDogResult<Self> {
        let workflow_store = crate::workflows::zero_cost_storage::ZeroCostMemoryWorkflowStore::<1000>::new();
        let approval_store = crate::workflows::zero_cost_storage::ZeroCostMemoryApprovalStore::<1000>::new();
        let processor = DefaultWorkflowProcessor;
        let engine = crate::workflows::zero_cost_engine::ZeroCostWorkflowEngine::new(workflow_store, approval_store, processor);
        Ok(Self {
            engine,
        })
    }

    pub async fn with_config(_config: WorkflowEngineConfig) -> beardog_errors::BearDogResult<Self> {
        let workflow_store = crate::workflows::zero_cost_storage::ZeroCostMemoryWorkflowStore::<1000>::new();
        let approval_store = crate::workflows::zero_cost_storage::ZeroCostMemoryApprovalStore::<1000>::new();
        let processor = DefaultWorkflowProcessor;
        let engine = crate::workflows::zero_cost_engine::ZeroCostWorkflowEngine::new(workflow_store, approval_store, processor);
        Ok(Self {
            engine,
        })
    }

    pub async fn submit_workflow(&self, workflow: Workflow) -> beardog_errors::BearDogResult<WorkflowId> {
        self.engine.submit_workflow(workflow).await
    }

    pub async fn get_workflow_status(&self, workflow_id: &WorkflowId) -> beardog_errors::BearDogResult<WorkflowStatus> {
        self.engine.get_workflow_status(workflow_id).await
    }
}

pub struct WorkflowSystemBuilder {
    _config: WorkflowEngineConfig,
}

impl WorkflowSystemBuilder {

    pub fn new() -> Self {
        Self {
            _config: WorkflowEngineConfig::default(),
        }
    }

    pub fn max_concurrent_workflows(mut self, max: usize) -> Self {
        self._config.max_concurrent = max;
        self
    }

    pub fn with_approvals(self, _enabled: bool) -> Self {

        self
    }

    pub async fn build(self) -> beardog_errors::BearDogResult<BearDogWorkflowSystem> {
        BearDogWorkflowSystem::with_config(self._config).await
    }
}

impl Default for WorkflowSystemBuilder {
    fn default() -> Self {
        Self::new()
    }
}
