// Unified Workflow Trait System
//
// This module provides workflow-related traits for process orchestration,
// execution management, and workflow lifecycle in the BearDog ecosystem.

// Re-export core workflow provider
pub use super::providers::WorkflowProvider;

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Workflow execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowStatus {
    /// Workflow is pending execution
    Pending,
    /// Workflow is currently running
    Running,
    /// Workflow is paused
    Paused,
    /// Workflow completed successfully
    Completed,
    /// Workflow failed
    Failed,
    /// Workflow was cancelled
    Cancelled,
}

/// Workflow execution context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowContext {
    /// Workflow execution ID
    pub execution_id: String,

    /// Current status
    pub status: WorkflowStatus,

    /// Start time
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Completion time
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Execution metadata
    pub metadata: HashMap<String, serde_json::Value>,

    /// Error message if failed
    pub error_message: Option<String>,
}

/// Workflow executor trait for executing workflows
pub trait WorkflowExecutor: Send + Sync {
    /// Workflow result type
    type Result: Send + Sync + Serialize + for<'de> Deserialize<'de>;

    /// Execute a workflow
    fn execute(
        &self,
        workflow_id: &str,
        context: WorkflowContext,
    ) -> impl std::future::Future<Output = Result<Self::Result, BearDogError>> + Send;

    /// Cancel a running workflow
    fn cancel(
        &self,
        execution_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Pause a running workflow
    fn pause(
        &self,
        execution_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Resume a paused workflow
    fn resume(
        &self,
        execution_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Get workflow execution status
    fn get_status(
        &self,
        execution_id: &str,
    ) -> impl std::future::Future<Output = Result<WorkflowStatus, BearDogError>> + Send;
}

/// Workflow step trait for individual workflow steps
pub trait WorkflowStep: Send + Sync {
    /// Step result type
    type StepResult: Send + Sync;

    /// Execute this workflow step
    fn execute_step(
        &self,
        context: &WorkflowContext,
    ) -> impl std::future::Future<Output = Result<Self::StepResult, BearDogError>> + Send;

    /// Rollback this step (for compensating transactions)
    fn rollback(
        &self,
        _context: &WorkflowContext,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send {
        async move { Ok(()) }
    }

    /// Validate step preconditions
    fn validate_preconditions(
        &self,
        _context: &WorkflowContext,
    ) -> impl std::future::Future<Output = Result<bool, BearDogError>> + Send {
        async move { Ok(true) }
    }
}

/// Workflow orchestrator for complex multi-step workflows
pub trait WorkflowOrchestrator: Send + Sync {
    /// Step type for this orchestrator
    type Step: Send + Sync;

    /// Orchestrate a multi-step workflow
    fn orchestrate(
        &self,
        steps: Vec<Self::Step>,
        context: WorkflowContext,
    ) -> impl std::future::Future<Output = Result<WorkflowContext, BearDogError>> + Send;

    /// Handle workflow compensation (rollback)
    fn compensate(
        &self,
        steps: Vec<Self::Step>,
        context: WorkflowContext,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;
}

/// Workflow monitoring trait for observability
pub trait WorkflowMonitor: Send + Sync {
    /// Record workflow start
    fn record_start(
        &self,
        execution_id: &str,
        workflow_id: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Record workflow completion
    fn record_completion(
        &self,
        execution_id: &str,
        status: WorkflowStatus,
        duration_ms: u64,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Record workflow error
    fn record_error(
        &self,
        execution_id: &str,
        error: &str,
    ) -> impl std::future::Future<Output = Result<(), BearDogError>> + Send;

    /// Get workflow metrics
    fn get_metrics(
        &self,
        workflow_id: &str,
    ) -> impl std::future::Future<Output = Result<HashMap<String, f64>, BearDogError>> + Send;
}

impl Default for WorkflowContext {
    fn default() -> Self {
        Self {
            execution_id: uuid::Uuid::new_v4().to_string(),
            status: WorkflowStatus::Pending,
            started_at: None,
            completed_at: None,
            metadata: HashMap::new(),
            error_message: None,
        }
    }
}

impl std::fmt::Display for WorkflowStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "Pending"),
            Self::Running => write!(f, "Running"),
            Self::Paused => write!(f, "Paused"),
            Self::Completed => write!(f, "Completed"),
            Self::Failed => write!(f, "Failed"),
            Self::Cancelled => write!(f, "Cancelled"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workflow_context_default() {
        let context = WorkflowContext::default();
        assert!(!context.execution_id.is_empty());
        assert_eq!(context.status, WorkflowStatus::Pending);
    }

    #[test]
    fn test_workflow_status_display() {
        assert_eq!(WorkflowStatus::Running.to_string(), "Running");
        assert_eq!(WorkflowStatus::Completed.to_string(), "Completed");
        assert_eq!(WorkflowStatus::Failed.to_string(), "Failed");
    }

    #[test]
    fn test_workflow_status_equality() {
        assert_eq!(WorkflowStatus::Pending, WorkflowStatus::Pending);
        assert_ne!(WorkflowStatus::Running, WorkflowStatus::Completed);
    }
}
