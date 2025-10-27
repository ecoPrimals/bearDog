//! # `BearDog` Workflows - Process Orchestration
//!
//! Comprehensive workflow orchestration and process management for `BearDog` applications,
//! providing stateful workflow execution with audit logging and error recovery.
//!
//! ## Features
//!
//! - **Stateful Workflows**: Multi-step process orchestration
//! - **Audit Logging**: Complete workflow execution history
//! - **Error Recovery**: Automatic retry and failure handling
//! - **Concurrent Execution**: Parallel workflow processing
//! - **Pluggable Storage**: Flexible workflow state persistence
//!
//! ## Example
//!
//! ```rust,no_run
//! use beardog_workflows::{WorkflowService, WorkflowConfig};
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Configure workflow system
//! let config = WorkflowConfig {
//!     max_concurrent_workflows: 10,
//!     default_timeout_seconds: 300,
//!     retry_attempts: 3,
//!     enable_audit_logging: true,
//!     workflow_storage_path: "/var/lib/beardog/workflows".to_string(),
//! };
//!
//! // Initialize workflow service
//! // (implementation details)
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! The workflow system is built on several components:
//! - **Workflow Service**: Orchestrates workflow execution
//! - **Repository**: Persists workflow state
//! - **Processor**: Executes workflow steps
//! - **Observer**: Monitors workflow progress
//!
//! ## Safety
//!
//! All workflow operations are memory-safe with zero unsafe code.

/// Core workflow functionality and types
pub mod workflows;

#[cfg(test)]
mod tests;

// October 26, 2025: Week 2 Day 5 - Execution Tests (FINAL!)
#[cfg(test)]
mod workflow_execution_tests;
#[cfg(test)]
mod workflow_orchestration_tests;

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
