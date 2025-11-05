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

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn test_workflow_system_version() {
        assert_eq!(WORKFLOW_SYSTEM_VERSION, "3.1.0");
        // WORKFLOW_SYSTEM_VERSION is a const, so is_empty check is redundant
    }

    #[test]
    fn test_workflow_config_creation() {
        let config = WorkflowConfig {
            max_concurrent_workflows: 10,
            default_timeout_seconds: 300,
            retry_attempts: 3,
            enable_audit_logging: true,
            workflow_storage_path: "/var/lib/beardog/workflows".to_string(),
        };

        assert_eq!(config.max_concurrent_workflows, 10);
        assert_eq!(config.default_timeout_seconds, 300);
        assert_eq!(config.retry_attempts, 3);
        assert!(config.enable_audit_logging);
        assert_eq!(config.workflow_storage_path, "/var/lib/beardog/workflows");
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: workflows
        // TEST_PRIORITY: normal
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: workflows
    // TEST_PRIORITY: normal
    fn test_workflow_config_clone() {
        let config1 = WorkflowConfig {
            max_concurrent_workflows: 5,
            default_timeout_seconds: 600,
            retry_attempts: 5,
            enable_audit_logging: false,
            workflow_storage_path: "/tmp/workflows".to_string(),
        };

        let config2 = config1.clone();
        assert_eq!(
            config1.max_concurrent_workflows,
            config2.max_concurrent_workflows
        );
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: workflows
        // TEST_PRIORITY: normal
        assert_eq!(
            config1.default_timeout_seconds,
            config2.default_timeout_seconds
        );
        assert_eq!(config1.retry_attempts, config2.retry_attempts);
        assert_eq!(config1.enable_audit_logging, config2.enable_audit_logging);
        assert_eq!(config1.workflow_storage_path, config2.workflow_storage_path);
    }

    #[test]
    fn test_workflow_config_serialization() {
        let config = WorkflowConfig {
            max_concurrent_workflows: 20,
            default_timeout_seconds: 120,
            retry_attempts: 2,
            enable_audit_logging: true,
            workflow_storage_path: "/data/workflows".to_string(),
        };

        let json = serde_json::to_string(&config).expect("Serialization should succeed");
        assert!(json.contains("max_concurrent_workflows"));
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: workflows
        // TEST_PRIORITY: normal
        assert!(json.contains("20"));

        let deserialized: WorkflowConfig =
            serde_json::from_str(&json).expect("Deserialization should succeed");
        assert_eq!(
            config.max_concurrent_workflows,
            deserialized.max_concurrent_workflows
        );
        assert_eq!(
            config.default_timeout_seconds,
            deserialized.default_timeout_seconds
        );
    }

    #[test]
    fn test_workflow_config_edge_cases() {
        // Zero concurrent workflows
        let config = WorkflowConfig {
            max_concurrent_workflows: 0,
            default_timeout_seconds: 1,
            retry_attempts: 0,
            enable_audit_logging: false,
            workflow_storage_path: String::new(),
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: workflows
            // TEST_PRIORITY: important
        };
        assert_eq!(config.max_concurrent_workflows, 0);
        assert_eq!(config.retry_attempts, 0);
        assert!(config.workflow_storage_path.is_empty());

        // Large values
        let config = WorkflowConfig {
            max_concurrent_workflows: 1000,
            default_timeout_seconds: u64::MAX,
            retry_attempts: u32::MAX,
            enable_audit_logging: true,
            workflow_storage_path: "a".repeat(1000),
        };
        assert_eq!(config.max_concurrent_workflows, 1000);
        assert_eq!(config.default_timeout_seconds, u64::MAX);
        assert_eq!(config.retry_attempts, u32::MAX);
    }

    #[test]
    fn test_beardog_workflow_system_construction() {
        use workflows::{
            ExampleWorkflowProcessor, InMemoryWorkflowRepository, LoggingWorkflowObserver,
        };

        // TEST_CATEGORY: unit
        // TEST_DOMAIN: workflows
        // TEST_PRIORITY: normal
        let repo = InMemoryWorkflowRepository::new();
        let processor = ExampleWorkflowProcessor {
            name: "test-processor",
        };
        let observer = LoggingWorkflowObserver {
            name: "test-observer".to_string(),
        };

        let system = BearDogWorkflowSystem::new(repo, processor, observer);
        assert!(format!("{:?}", system).contains("BearDogWorkflowSystem"));
    }

    #[test]
    fn test_workflow_system_debug() {
        use workflows::{
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: workflows
            // TEST_PRIORITY: normal
            ExampleWorkflowProcessor,
            InMemoryWorkflowRepository,
            LoggingWorkflowObserver,
        };

        let repo = InMemoryWorkflowRepository::new();
        let processor = ExampleWorkflowProcessor {
            name: "debug-processor",
        };
        let observer = LoggingWorkflowObserver {
            name: "debug-observer".to_string(),
        };

        let system = BearDogWorkflowSystem::new(repo, processor, observer);
        let debug_str = format!("{:?}", system);
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("service"));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: workflows
    // TEST_PRIORITY: normal
    #[test]
    fn test_workflow_config_debug() {
        let config = WorkflowConfig {
            max_concurrent_workflows: 10,
            default_timeout_seconds: 300,
            retry_attempts: 3,
            enable_audit_logging: true,
            workflow_storage_path: "/var/lib/beardog/workflows".to_string(),
        };

        let debug_str = format!("{:?}", config);
        assert!(!debug_str.is_empty());
        assert!(debug_str.contains("WorkflowConfig"));
    }
}
