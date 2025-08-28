// New canonical workflow system
pub mod canonical_examples;
pub mod canonical_traits;

// Legacy modules kept for compatibility
pub mod types;

// Clean exports - only canonical system
pub use canonical_traits::{
    Workflow, WorkflowCommand, WorkflowId, WorkflowObserver, WorkflowProcessor, WorkflowRepository,
    WorkflowService, WorkflowStatus,
};

pub use canonical_examples::{
    ExampleWorkflow, ExampleWorkflowProcessor, ExampleWorkflowStatus, InMemoryWorkflowRepository,
    LoggingWorkflowObserver, ProcessingContext,
};

pub use types::{WorkflowRequest, WorkflowResponse};
