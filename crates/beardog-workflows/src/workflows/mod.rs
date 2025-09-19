// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod canonical_examples;
pub mod canonical_traits;

pub mod types;

pub use canonical_traits::{
    Workflow, WorkflowCommand, WorkflowId, WorkflowObserver, WorkflowProcessor, WorkflowRepository,
    WorkflowService, WorkflowStatus,
};

pub use canonical_examples::{
    ExampleWorkflow, ExampleWorkflowProcessor, ExampleWorkflowStatus, InMemoryWorkflowRepository,
    LoggingWorkflowObserver, ProcessingContext,
};

pub use types::{WorkflowRequest, WorkflowResponse};
