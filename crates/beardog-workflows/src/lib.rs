use crate::workflows::canonical_traits::Workflow;
pub mod workflows;

pub use beardog_types::canonical::configuration::consolidated::WorkflowConfig;

// Modern canonical workflow system exports
pub use workflows::{
    // Example implementations
    ExampleWorkflow,
    ExampleWorkflowProcessor,
    ExampleWorkflowStatus,
    InMemoryWorkflowRepository,
    LoggingWorkflowObserver,

    ProcessingContext,
    WorkflowCommand,
    // Core traits
    WorkflowId,
    WorkflowObserver,
    WorkflowProcessor,
    WorkflowRepository,
    // Legacy types for compatibility
    WorkflowRequest,
    WorkflowResponse,
    WorkflowService,

    WorkflowStatus,
};

pub const WORKFLOW_SYSTEM_VERSION: &str = "3.1.0";

// Version updated to reflect canonical trait system implementation

/// Modern workflow system using canonical traits
pub struct BearDogWorkflowSystem<R, P, O>
where
    R: WorkflowRepository,
    P: WorkflowProcessor<Workflow = R::Workflow>,
    O: WorkflowObserver<Workflow = R::Workflow>,
{
    service: WorkflowService<R, P, O>,
}

impl<R, P, O> BearDogWorkflowSystem<R, P, O>
where
    R: WorkflowRepository,
    P: WorkflowProcessor<Workflow = R::Workflow>,
    O: WorkflowObserver<Workflow = R::Workflow>,
{
    pub fn new(repository: R, processor: P, observer: O) -> Self {
        let mut service = WorkflowService::new(repository, processor);
        service.add_observer(observer);
        Self { service }
    }

    pub async fn execute_workflow(
        &self,
        workflow: R::Workflow,
        context: P::Context,
    ) -> Result<R::Workflow, R::Error>
    where
        O::Error: std::fmt::Debug,
        P::Error: std::fmt::Debug + Into<R::Error>,
    {
        self.service.create_workflow(workflow.clone()).await?;
        self.service.process_workflow(workflow.id(), context).await
    }
}

// Legacy implementation methods removed - use the new constructor with canonical traits

// Example usage:
// ```rust
// use beardog_workflows::*;
//
// let repository = InMemoryWorkflowRepository::new();
// let processor = ExampleWorkflowProcessor::new("MyProcessor");
// let observer = LoggingWorkflowObserver::new("MyObserver");
//
// let system = BearDogWorkflowSystem::new(repository, processor, observer);
// ```
