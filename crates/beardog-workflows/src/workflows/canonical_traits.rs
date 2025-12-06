// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_errors::BearDogError;
use std::future::Future;

pub trait WorkflowId: Clone + Send + Sync + std::fmt::Debug + std::fmt::Display {
    /// Returns as str
    fn as_str(&self) -> &str;
}

pub trait WorkflowStatus: Clone + Send + Sync + std::fmt::Debug {
    /// Checks if terminal
    fn is_terminal(&self) -> bool;
    /// Checks if active
    fn is_active(&self) -> bool;
}

pub trait Workflow: Clone + Send + Sync + std::fmt::Debug {
    type Id: WorkflowId;
    type Status: WorkflowStatus;

    fn id(&self) -> &Self::Id;
    fn status(&self) -> &Self::Status;
    /// Creates `itemd_at`
    fn created_at(&self) -> chrono::DateTime<chrono::Utc>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultWorkflowStatus {
    /// Operation in progress
    Pending,
    /// Currently running
    Running,
    /// Successful completion state
    Completed,
    /// Error or failure state
    Failed,
    /// State indicating cancelled
    Cancelled,
}

impl WorkflowStatus for DefaultWorkflowStatus {
    /// Checks if terminal
    fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed | Self::Cancelled)
    }

    /// Checks if active
    fn is_active(&self) -> bool {
        matches!(self, Self::Running)
    }
}

pub trait WorkflowRepository: Send + Sync {
    type Workflow: Workflow;
    type Error: From<BearDogError> + Send + Sync + 'static;

    /// Saves data
    fn save(
        &self,
        workflow: Self::Workflow,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn find_by_id(
        &self,
        id: &<Self::Workflow as Workflow>::Id,
    ) -> impl Future<Output = Result<Option<Self::Workflow>, Self::Error>> + Send;

    /// Updates item
    fn update(
        &self,
        workflow: Self::Workflow,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Removes
    fn delete(
        &self,
        id: &<Self::Workflow as Workflow>::Id,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn list_all(&self) -> impl Future<Output = Result<Vec<Self::Workflow>, Self::Error>> + Send;

    fn count(&self) -> impl Future<Output = Result<usize, Self::Error>> + Send;

    fn exists(
        &self,
        id: &<Self::Workflow as Workflow>::Id,
    ) -> impl Future<Output = Result<bool, Self::Error>> + Send;
}

pub trait WorkflowProcessor: Send + Sync {
    type Workflow: Workflow;
    type Context: Send + Sync;
    type Error: From<BearDogError> + Send + Sync + 'static;

    /// Processes data
    fn process(
        &self,
        workflow: Self::Workflow,
        context: Self::Context,
    ) -> impl Future<Output = Result<Self::Workflow, Self::Error>> + Send;

    /// Validates input
    fn validate(
        &self,
        workflow: &Self::Workflow,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn can_process(&self, workflow: &Self::Workflow) -> bool;

    fn name(&self) -> &str;
}

pub trait WorkflowObserver: Send + Sync {
    type Workflow: Workflow;
    type Error: From<BearDogError> + Send + Sync + 'static;

    fn on_created(
        &self,
        workflow: &Self::Workflow,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn on_started(
        &self,
        workflow: &Self::Workflow,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn on_completed(
        &self,
        workflow: &Self::Workflow,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn on_failed(
        &self,
        workflow: &Self::Workflow,
        error: &str,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;

    fn on_cancelled(
        &self,
        workflow: &Self::Workflow,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

pub trait WorkflowCommand: Send + Sync {
    type Workflow: Workflow;
    type Result: Send + Sync;
    type Error: From<BearDogError> + Send + Sync + 'static;

    /// Executes operation
    fn execute(
        &self,
        workflow: Self::Workflow,
    ) -> impl Future<Output = Result<Self::Result, Self::Error>> + Send;

    fn can_execute(&self, workflow: &Self::Workflow) -> bool;

    fn description(&self) -> &str;
}

#[derive(Debug)]
pub struct WorkflowService<R, P, O>
where
    R: WorkflowRepository,
    P: WorkflowProcessor<Workflow = R::Workflow>,
    O: WorkflowObserver<Workflow = R::Workflow>,
{
    /// The repository value
    pub repository: R,
    /// The processor value
    pub processor: P,
    /// Collection of observers
    pub observers: Vec<O>,
}

impl<R, P, O> WorkflowService<R, P, O>
where
    R: WorkflowRepository,
    P: WorkflowProcessor<Workflow = R::Workflow>,
    O: WorkflowObserver<Workflow = R::Workflow>,
{
    /// New operation.
    /// Creates a new instance
    pub const fn new(repository: R, processor: P) -> Self {
        Self {
            repository,
            processor,
            observers: Vec::new(),
        }
    }

    /// Add Observer operation.
    pub fn add_observer(&mut self, observer: O) {
        self.observers.push(observer);
    }

    /// Repository operation.
    pub const fn repository(&self) -> &R {
        &self.repository
    }

    /// Create Workflow operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Creates workflow
    /// Creates workflow
    pub async fn create_workflow(&self, workflow: R::Workflow) -> Result<(), R::Error>
    where
        O::Error: std::fmt::Debug,
    {
        let workflow_clone = workflow.clone();
        self.repository.save(workflow_clone).await?;

        for observer in &self.observers {
            if let Err(e) = observer.on_created(&workflow).await {
                tracing::warn!("Observer notification failed: {:?}", e);
            }
        }

        Ok(())
    }

    /// Execute Workflow operation.
    ///
    /// # Errors
    /// Returns an error if the operation fails.
    /// Executes workflow
    /// Executes workflow
    pub async fn execute_workflow(
        &self,
        workflow_id: &<R::Workflow as Workflow>::Id,
        context: P::Context,
    ) -> Result<R::Workflow, R::Error>
    where
        O::Error: std::fmt::Debug,
        P::Error: std::fmt::Debug + Into<R::Error>,
    {
        let workflow = self
            .repository
            .find_by_id(workflow_id)
            .await?
            .ok_or_else(|| BearDogError::not_found("Workflow not found".to_string()))?;

        for observer in &self.observers {
            if let Err(e) = observer.on_started(&workflow).await {
                tracing::warn!("Observer notification failed: {:?}", e);
            }
        }

        match self.processor.process(workflow.clone(), context).await {
            Ok(processed_workflow) => {
                let workflow_clone = processed_workflow.clone();
                self.repository.update(workflow_clone).await?;

                for observer in &self.observers {
                    if let Err(e) = observer.on_completed(&processed_workflow).await {
                        tracing::warn!("Observer notification failed: {:?}", e);
                    }
                }

                Ok(processed_workflow)
            }
            Err(e) => {
                let error_msg = format!("{e:?}");
                for observer in &self.observers {
                    if let Err(e) = observer.on_failed(&workflow, &error_msg).await {
                        tracing::warn!("Observer notification failed: {:?}", e);
                    }
                }

                Err(e.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: workflows
    // TEST_PRIORITY: normal
    #[test]
    fn test_traits_exist() {}

    #[test]
    fn test_default_workflow_status_is_terminal() {
        assert!(!DefaultWorkflowStatus::Pending.is_terminal());
        assert!(!DefaultWorkflowStatus::Running.is_terminal());
        assert!(DefaultWorkflowStatus::Completed.is_terminal());
        assert!(DefaultWorkflowStatus::Failed.is_terminal());
        assert!(DefaultWorkflowStatus::Cancelled.is_terminal());
    }

    #[test]
    fn test_default_workflow_status_is_active() {
        assert!(!DefaultWorkflowStatus::Pending.is_active());
        assert!(DefaultWorkflowStatus::Running.is_active());
        assert!(!DefaultWorkflowStatus::Completed.is_active());
        assert!(!DefaultWorkflowStatus::Failed.is_active());
        assert!(!DefaultWorkflowStatus::Cancelled.is_active());
    }

    #[test]
    fn test_default_workflow_status_clone() {
        let status = DefaultWorkflowStatus::Running;
        let cloned = status;
        assert_eq!(status, cloned);
    }

    #[test]
    fn test_default_workflow_status_copy() {
        let status = DefaultWorkflowStatus::Completed;
        let copied = status;
        assert_eq!(status, copied);
    }

    #[test]
    fn test_default_workflow_status_debug() {
        let status = DefaultWorkflowStatus::Running;
        let debug_str = format!("{:?}", status);
        assert!(debug_str.contains("Running"));
    }

    #[test]
    fn test_default_workflow_status_all_variants() {
        let statuses = [
            DefaultWorkflowStatus::Pending,
            DefaultWorkflowStatus::Running,
            DefaultWorkflowStatus::Completed,
            DefaultWorkflowStatus::Failed,
            DefaultWorkflowStatus::Cancelled,
        ];
        assert_eq!(statuses.len(), 5);
    }

    #[test]
    fn test_default_workflow_status_equality() {
        assert_eq!(
            DefaultWorkflowStatus::Pending,
            DefaultWorkflowStatus::Pending
        );
        assert_ne!(
            DefaultWorkflowStatus::Pending,
            DefaultWorkflowStatus::Running
        );
        assert_ne!(
            DefaultWorkflowStatus::Running,
            DefaultWorkflowStatus::Completed
        );
    }

    #[test]
    fn test_default_workflow_status_terminal_states() {
        let terminal_states = vec![
            DefaultWorkflowStatus::Completed,
            DefaultWorkflowStatus::Failed,
            DefaultWorkflowStatus::Cancelled,
        ];

        for status in terminal_states {
            assert!(status.is_terminal(), "{:?} should be terminal", status);
            assert!(!status.is_active(), "{:?} should not be active", status);
        }
    }

    #[test]
    fn test_default_workflow_status_non_terminal_states() {
        let non_terminal_states = vec![
            DefaultWorkflowStatus::Pending,
            DefaultWorkflowStatus::Running,
        ];

        for status in non_terminal_states {
            assert!(!status.is_terminal(), "{:?} should not be terminal", status);
        }
    }

    #[test]
    fn test_default_workflow_status_active_only_running() {
        // Only Running should be active
        assert!(DefaultWorkflowStatus::Running.is_active());

        // All others should not be active
        assert!(!DefaultWorkflowStatus::Pending.is_active());
        assert!(!DefaultWorkflowStatus::Completed.is_active());
        assert!(!DefaultWorkflowStatus::Failed.is_active());
        assert!(!DefaultWorkflowStatus::Cancelled.is_active());
    }

    // Mock implementations for trait testing
    #[derive(Debug, Clone)]
    struct MockWorkflowId(String);

    impl std::fmt::Display for MockWorkflowId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl WorkflowId for MockWorkflowId {
        fn as_str(&self) -> &str {
            &self.0
        }
    }

    #[test]
    fn test_workflow_id_trait() {
        let id = MockWorkflowId("test-id-123".to_string());
        assert_eq!(id.as_str(), "test-id-123");
        assert_eq!(format!("{}", id), "test-id-123");
    }

    #[test]
    fn test_workflow_id_clone() {
        let id = MockWorkflowId("clone-test".to_string());
        let cloned = id.clone();
        assert_eq!(id.as_str(), cloned.as_str());
    }

    #[derive(Debug, Clone)]
    struct MockWorkflow {
        id: MockWorkflowId,
        status: DefaultWorkflowStatus,
        created: chrono::DateTime<chrono::Utc>,
    }

    impl Workflow for MockWorkflow {
        type Id = MockWorkflowId;
        type Status = DefaultWorkflowStatus;

        fn id(&self) -> &Self::Id {
            &self.id
        }

        fn status(&self) -> &Self::Status {
            &self.status
        }

        fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
            self.created
        }
    }

    #[test]
    fn test_workflow_trait() {
        let now = chrono::Utc::now();
        let workflow = MockWorkflow {
            id: MockWorkflowId("wf-001".to_string()),
            status: DefaultWorkflowStatus::Running,
            created: now,
        };

        assert_eq!(workflow.id().as_str(), "wf-001");
        assert_eq!(*workflow.status(), DefaultWorkflowStatus::Running);
        assert_eq!(workflow.created_at(), now);
    }

    #[test]
    fn test_workflow_with_different_statuses() {
        let now = chrono::Utc::now();
        let statuses = [
            DefaultWorkflowStatus::Pending,
            DefaultWorkflowStatus::Running,
            DefaultWorkflowStatus::Completed,
        ];

        for (i, status) in statuses.iter().enumerate() {
            let workflow = MockWorkflow {
                id: MockWorkflowId(format!("wf-{}", i)),
                status: *status,
                created: now,
            };
            assert_eq!(workflow.status(), status);
        }
    }

    #[test]
    fn test_workflow_status_trait_methods() {
        // Test that WorkflowStatus trait methods work correctly
        let status = DefaultWorkflowStatus::Running;
        assert!(status.is_active());
        assert!(!status.is_terminal());

        let completed = DefaultWorkflowStatus::Completed;
        assert!(!completed.is_active());
        assert!(completed.is_terminal());
    }

    #[test]
    fn test_workflow_clone() {
        let workflow = MockWorkflow {
            id: MockWorkflowId("clone-wf".to_string()),
            status: DefaultWorkflowStatus::Completed,
            created: chrono::Utc::now(),
        };

        let cloned = workflow.clone();
        assert_eq!(cloned.id().as_str(), workflow.id().as_str());
        assert_eq!(*cloned.status(), *workflow.status());
    }

    #[test]
    fn test_workflow_debug() {
        let workflow = MockWorkflow {
            id: MockWorkflowId("debug-wf".to_string()),
            status: DefaultWorkflowStatus::Failed,
            created: chrono::Utc::now(),
        };

        let debug_str = format!("{:?}", workflow);
        assert!(debug_str.contains("MockWorkflow"));
    }

    #[test]
    fn test_workflow_lifecycle_statuses() {
        let now = chrono::Utc::now();

        // Start pending
        let mut workflow = MockWorkflow {
            id: MockWorkflowId("lifecycle-test".to_string()),
            status: DefaultWorkflowStatus::Pending,
            created: now,
        };
        assert!(!workflow.status().is_active());
        assert!(!workflow.status().is_terminal());

        // Move to running
        workflow.status = DefaultWorkflowStatus::Running;
        assert!(workflow.status().is_active());
        assert!(!workflow.status().is_terminal());

        // Complete
        workflow.status = DefaultWorkflowStatus::Completed;
        assert!(!workflow.status().is_active());
        assert!(workflow.status().is_terminal());
    }

    #[test]
    fn test_workflow_failure_path() {
        let now = chrono::Utc::now();
        let mut workflow = MockWorkflow {
            id: MockWorkflowId("failure-test".to_string()),
            status: DefaultWorkflowStatus::Running,
            created: now,
        };

        // Initially running
        assert!(workflow.status().is_active());

        // Fail
        workflow.status = DefaultWorkflowStatus::Failed;
        assert!(!workflow.status().is_active());
        assert!(workflow.status().is_terminal());
    }

    #[test]
    fn test_workflow_cancellation() {
        let now = chrono::Utc::now();
        let mut workflow = MockWorkflow {
            id: MockWorkflowId("cancel-test".to_string()),
            status: DefaultWorkflowStatus::Pending,
            created: now,
        };

        // Cancel from pending
        workflow.status = DefaultWorkflowStatus::Cancelled;
        assert!(!workflow.status().is_active());
        assert!(workflow.status().is_terminal());
    }

    #[test]
    fn test_workflow_id_empty_string() {
        let id = MockWorkflowId(String::new());
        assert_eq!(id.as_str(), "");
        assert_eq!(format!("{}", id), "");
    }

    #[test]
    fn test_workflow_id_special_characters() {
        let id = MockWorkflowId("test-id_123.456@abc".to_string());
        assert_eq!(id.as_str(), "test-id_123.456@abc");
    }

    #[test]
    fn test_workflow_status_match_patterns() {
        // Test that match patterns work correctly
        let status = DefaultWorkflowStatus::Running;

        let result = match status {
            DefaultWorkflowStatus::Pending => "pending",
            DefaultWorkflowStatus::Running => "running",
            DefaultWorkflowStatus::Completed => "completed",
            DefaultWorkflowStatus::Failed => "failed",
            DefaultWorkflowStatus::Cancelled => "cancelled",
        };

        assert_eq!(result, "running");
    }

    #[test]
    fn test_workflow_created_at_ordering() {
        // ✅ MODERNIZED: Removed sleep - chrono::Utc::now() is monotonic
        let time1 = chrono::Utc::now();
        let time2 = chrono::Utc::now();

        let wf1 = MockWorkflow {
            id: MockWorkflowId("wf1".to_string()),
            status: DefaultWorkflowStatus::Running,
            created: time1,
        };

        let wf2 = MockWorkflow {
            id: MockWorkflowId("wf2".to_string()),
            status: DefaultWorkflowStatus::Running,
            created: time2,
        };

        assert!(wf1.created_at() <= wf2.created_at());
    }
}
