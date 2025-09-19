// Removed unused Workflow trait import
// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod workflows;

// Canonical workflow configuration - modernized
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowConfig {
    /// Number of `max_concurrent_workflows`
    pub max_concurrent_workflows: usize,
    pub default_timeout_seconds: u64,
    /// Number of `retry_attempts`
    pub retry_attempts: u32,
    /// Whether `enable_audit_logging` is enabled
    pub enable_audit_logging: bool,
    /// The workflow storage path value
    pub workflow_storage_path: String,
}

pub use workflows::{
    ExampleWorkflow, ExampleWorkflowProcessor, ExampleWorkflowStatus, InMemoryWorkflowRepository,
    LoggingWorkflowObserver, ProcessingContext, WorkflowCommand, WorkflowId, WorkflowObserver,
    WorkflowProcessor, WorkflowRepository, WorkflowRequest, WorkflowResponse, WorkflowService,
    WorkflowStatus,
};

pub const WORKFLOW_SYSTEM_VERSION: &str = "3.1.0";

/// Canonical `BearDog` workflow system - modernized and unified
#[derive(Debug)]
pub struct BearDogWorkflowSystem<R, P, O>
where
    R: WorkflowRepository,
    P: WorkflowProcessor<Workflow = R::Workflow>,
    O: WorkflowObserver<Workflow = R::Workflow>,
{
    /// The service value
    pub service: WorkflowService<R, P, O>,
}

impl<R, P, O> BearDogWorkflowSystem<R, P, O>
where
    R: WorkflowRepository,
    P: WorkflowProcessor<Workflow = R::Workflow>,
    O: WorkflowObserver<Workflow = R::Workflow>,
{
    /// New canonical workflow system
    /// Creates a new instance
    pub fn new(repository: R, processor: P, observer: O) -> Self {
        let mut service = WorkflowService::new(repository, processor);
        service.add_observer(observer);
        Self { service }
    }

    /// Execute workflow with context
    /// Executes workflow
    /// Executes workflow
    pub async fn execute_workflow(
        &mut self,
        workflow: R::Workflow,
        context: P::Context,
    ) -> Result<R::Workflow, R::Error>
    where
        O::Error: std::fmt::Debug,
        P::Error: std::fmt::Debug + Into<R::Error>,
    {
        let workflow_clone = workflow.clone();
        self.service.repository.save(workflow_clone).await?;
        self.service
            .processor
            .process(workflow, context)
            .await
            .map_err(Into::into)
    }
}
