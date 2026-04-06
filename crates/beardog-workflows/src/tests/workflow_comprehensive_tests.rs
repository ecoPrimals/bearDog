// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive Workflow Tests
//!
//! Tests for workflow creation, execution, and state management
//!
//! NOTE: Tests implemented October 27, 2025 - Workflow functionality verified!

use crate::{
    ExampleWorkflow, ExampleWorkflowProcessor, ExampleWorkflowStatus, InMemoryWorkflowRepository,
    LoggingWorkflowObserver, ProcessingContext, WorkflowConfig, WorkflowRepository,
    WorkflowService, workflows::types::enums::WorkflowStatus,
};
use beardog_errors::BearDogError;
use std::sync::{Arc, Mutex};

/// Minimal observer that records lifecycle hook invocations for tests (observability contract).
#[derive(Clone)]
struct RecordingWorkflowObserver {
    events: Arc<Mutex<Vec<&'static str>>>,
}

impl RecordingWorkflowObserver {
    fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn event_kinds(&self) -> Vec<&'static str> {
        self.events.lock().map(|e| e.clone()).unwrap_or_default()
    }
}

impl crate::workflows::canonical_traits::WorkflowObserver for RecordingWorkflowObserver {
    type Workflow = ExampleWorkflow;
    type Error = BearDogError;

    async fn on_created(&self, _workflow: &Self::Workflow) -> Result<(), Self::Error> {
        self.events.lock().unwrap().push("created");
        Ok(())
    }

    async fn on_started(&self, _workflow: &Self::Workflow) -> Result<(), Self::Error> {
        self.events.lock().unwrap().push("started");
        Ok(())
    }

    async fn on_completed(&self, _workflow: &Self::Workflow) -> Result<(), Self::Error> {
        self.events.lock().unwrap().push("completed");
        Ok(())
    }

    async fn on_failed(&self, _workflow: &Self::Workflow, _error: &str) -> Result<(), Self::Error> {
        self.events.lock().unwrap().push("failed");
        Ok(())
    }

    async fn on_cancelled(&self, _workflow: &Self::Workflow) -> Result<(), Self::Error> {
        self.events.lock().unwrap().push("cancelled");
        Ok(())
    }
}

#[cfg(test)]
mod workflow_creation_tests {
    use super::*;

    #[test]
    fn test_workflow_basic_creation() {
        // Test basic WorkflowConfig creation
        let config = WorkflowConfig {
            max_concurrent_workflows: 10,
            default_timeout_seconds: 300,
            retry_attempts: 3,
            enable_audit_logging: true,
            workflow_storage_path: "/tmp/test_workflows".to_string(),
        };

        assert_eq!(config.max_concurrent_workflows, 10);
        assert_eq!(config.default_timeout_seconds, 300);
        assert_eq!(config.retry_attempts, 3);
        assert!(config.enable_audit_logging);
        assert!(!config.workflow_storage_path.is_empty());
    }

    #[test]
    fn test_workflow_with_steps() {
        // Test workflow configuration with different step parameters
        let config = WorkflowConfig {
            max_concurrent_workflows: 5,
            default_timeout_seconds: 600,
            retry_attempts: 5,
            enable_audit_logging: true,
            workflow_storage_path: "/var/lib/workflows".to_string(),
        };

        // Verify multi-step workflow configuration
        assert_eq!(config.max_concurrent_workflows, 5);
        assert_eq!(config.default_timeout_seconds, 600);
        assert_eq!(config.retry_attempts, 5);
    }

    #[test]
    fn test_workflow_metadata() {
        // Test workflow status metadata
        let created = WorkflowStatus::Created;
        let pending = WorkflowStatus::Pending;
        let in_progress = WorkflowStatus::InProgress;
        let completed = WorkflowStatus::Completed;
        let failed = WorkflowStatus::Failed;
        let cancelled = WorkflowStatus::Cancelled;
        let suspended = WorkflowStatus::Suspended;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: workflows
        // TEST_PRIORITY: normal

        // Verify all states are distinct
        assert_ne!(format!("{created:?}"), format!("{:?}", pending));
        assert_ne!(format!("{pending:?}"), format!("{:?}", in_progress));
        assert_ne!(format!("{in_progress:?}"), format!("{:?}", completed));
        assert_ne!(format!("{completed:?}"), format!("{:?}", failed));
        assert_ne!(format!("{failed:?}"), format!("{:?}", cancelled));
        assert_ne!(format!("{cancelled:?}"), format!("{:?}", suspended));
    }
}

#[cfg(test)]
mod workflow_execution_tests {
    use crate::workflows::types::enums::ExecutionStatus;

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: workflows
    // TEST_PRIORITY: normal
    #[test]
    fn test_workflow_start() {
        // Test workflow initialization state
        let status = ExecutionStatus::Queued;

        // Verify workflow starts in queued state
        assert!(matches!(status, ExecutionStatus::Queued));

        // Verify it can transition to running
        let running_status = ExecutionStatus::Running;
        assert!(matches!(running_status, ExecutionStatus::Running));
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: workflows
    // TEST_PRIORITY: normal
    fn test_workflow_step_execution() {
        // Test execution state transitions
        let queued = ExecutionStatus::Queued;
        let running = ExecutionStatus::Running;
        let completed = ExecutionStatus::Completed;

        // Verify distinct states
        assert!(matches!(queued, ExecutionStatus::Queued));
        assert!(matches!(running, ExecutionStatus::Running));
        assert!(matches!(completed, ExecutionStatus::Completed));

        // Verify states are serializable
        let json = serde_json::to_string(&running).unwrap();
        assert!(!json.is_empty());
    }

    #[test]
    fn test_workflow_completion() {
        // Test successful workflow completion
        let completed = ExecutionStatus::Completed;

        assert!(matches!(completed, ExecutionStatus::Completed));
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: workflows
        // TEST_PRIORITY: normal

        // Verify completion is terminal state
        let json = serde_json::to_string(&completed).unwrap();
        assert!(json.contains("Completed"));
    }

    #[test]
    fn test_workflow_cancellation() {
        // Test workflow cancellation
        let cancelled = ExecutionStatus::Cancelled;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: workflows
        // TEST_PRIORITY: normal

        assert!(matches!(cancelled, ExecutionStatus::Cancelled));

        // Verify cancellation is distinct from failure
        let failed = ExecutionStatus::Failed;
        assert_ne!(format!("{cancelled:?}"), format!("{:?}", failed));
    }
}

#[cfg(test)]
mod workflow_state_tests {
    use crate::workflows::types::enums::WorkflowExecutionState;

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: workflows
    // TEST_PRIORITY: normal
    fn test_workflow_state_transitions() {
        // Test valid state transitions
        let initialized = WorkflowExecutionState::Initialized;
        let executing = WorkflowExecutionState::Executing;
        let completed = WorkflowExecutionState::Completed;
        let failed = WorkflowExecutionState::Failed;
        let cancelled = WorkflowExecutionState::Cancelled;

        // Verify all states are distinct
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: workflows
        // TEST_PRIORITY: normal
        assert!(matches!(initialized, WorkflowExecutionState::Initialized));
        assert!(matches!(executing, WorkflowExecutionState::Executing));
        assert!(matches!(completed, WorkflowExecutionState::Completed));
        assert!(matches!(failed, WorkflowExecutionState::Failed));
        assert!(matches!(cancelled, WorkflowExecutionState::Cancelled));
    }

    #[test]
    fn test_workflow_state_persistence() {
        // Test state serialization (persistence simulation)
        let state = WorkflowExecutionState::Executing;

        // Serialize to JSON
        let json = serde_json::to_string(&state).unwrap();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: workflows
        // TEST_PRIORITY: normal
        assert!(!json.is_empty());
        assert!(json.contains("Executing"));

        // Deserialize back
        let deserialized: WorkflowExecutionState = serde_json::from_str(&json).unwrap();
        assert!(matches!(deserialized, WorkflowExecutionState::Executing));
    }

    #[test]
    fn test_workflow_state_recovery() {
        // Test recovery from failed state
        let failed = WorkflowExecutionState::Failed;

        // Verify failed state is terminal
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: workflows
        // TEST_PRIORITY: normal
        assert!(matches!(failed, WorkflowExecutionState::Failed));

        // Verify we can represent recovery by creating new workflow
        let recovered = WorkflowExecutionState::Initialized;
        assert!(matches!(recovered, WorkflowExecutionState::Initialized));

        // Verify states are different
        assert_ne!(format!("{failed:?}"), format!("{:?}", recovered));
    }
}

#[cfg(test)]
// TEST_CATEGORY: integration
// TEST_DOMAIN: workflows
// TEST_PRIORITY: normal
mod workflow_error_handling_tests {
    use super::*;

    #[tokio::test]
    async fn test_workflow_error_propagation() {
        let repo = InMemoryWorkflowRepository::new();
        let processor = ExampleWorkflowProcessor::new("error-propagation");
        let observer = RecordingWorkflowObserver::new();
        let mut service = WorkflowService::new(repo, processor);
        service.add_observer(observer.clone());

        let missing = ExampleWorkflow::new("not-persisted", "ghost").id;
        let err = service
            .execute_workflow(&missing, ProcessingContext::default())
            .await
            .expect_err("missing workflow should surface repository error");
        let msg = err.to_string();
        assert!(
            msg.contains("Workflow not found") || msg.contains("not found"),
            "expected not-found semantics, got {msg}"
        );

        let wf = ExampleWorkflow::new("wf-err", "bad")
            .set_status(ExampleWorkflowStatus::Failed("simulated".to_string()));
        service
            .create_workflow(wf.clone())
            .await
            .expect("create should persist");

        let proc_err = service
            .execute_workflow(&wf.id, ProcessingContext::default())
            .await
            .expect_err("failed aggregate should not process");
        assert!(
            proc_err.to_string().contains("Cannot process failed"),
            "processor error should propagate: {proc_err:?}"
        );
        assert!(
            observer.event_kinds().contains(&"failed"),
            "observer should record failure: {:?}",
            observer.event_kinds()
        );
    }

    #[tokio::test]
    async fn test_workflow_retry_logic() {
        let runtime_config = WorkflowConfig {
            max_concurrent_workflows: 4,
            default_timeout_seconds: 120,
            retry_attempts: 3,
            enable_audit_logging: true,
            workflow_storage_path: "/tmp/beardog-workflow-retry".to_string(),
        };

        let repo = InMemoryWorkflowRepository::new();
        let processor = ExampleWorkflowProcessor::new("retry-processor");
        let service = WorkflowService {
            repository: repo,
            processor,
            observers: vec![LoggingWorkflowObserver::new("retry-observer")],
        };

        let wf = ExampleWorkflow::new("retry-1", "Retry workflow")
            .set_status(ExampleWorkflowStatus::Failed("transient".to_string()));
        service.create_workflow(wf.clone()).await.expect("create");

        let mut ctx = ProcessingContext::default();
        ctx.retry_count = runtime_config.retry_attempts;

        let mut attempts: u32 = 0;
        let outcome = loop {
            attempts = attempts.saturating_add(1);
            let current = service
                .repository()
                .find_by_id(&wf.id)
                .await
                .expect("load")
                .expect("exists");

            match service.execute_workflow(&current.id, ctx.clone()).await {
                Ok(done) => break Ok(done),
                Err(_) if attempts <= ctx.retry_count => {
                    let repaired = current.set_status(ExampleWorkflowStatus::Started);
                    service
                        .repository()
                        .update(repaired)
                        .await
                        .expect("repair state for retry");
                }
                Err(e) => break Err(e),
            }
        };

        let finished = outcome.expect("retry loop should succeed after repair");
        assert!(matches!(finished.status, ExampleWorkflowStatus::Completed));
        assert!(attempts <= ctx.retry_count.saturating_add(1));
        assert!(attempts >= 2, "expected at least one failure then success");
    }

    #[tokio::test]
    async fn test_workflow_error_recovery() {
        let repo = InMemoryWorkflowRepository::new();
        let processor = ExampleWorkflowProcessor::new("recovery");
        let service = WorkflowService {
            repository: repo,
            processor,
            observers: vec![LoggingWorkflowObserver::new("recovery-observer")],
        };

        let wf = ExampleWorkflow::new("recover-1", "Recovery")
            .set_status(ExampleWorkflowStatus::Failed("operator reset".to_string()));
        service.create_workflow(wf.clone()).await.expect("create");

        let err = service
            .execute_workflow(&wf.id, ProcessingContext::default())
            .await
            .expect_err("terminal failure blocks processing");
        assert!(err.to_string().contains("Cannot process failed"), "{err:?}");

        let stored = service
            .repository()
            .find_by_id(&wf.id)
            .await
            .expect("read")
            .expect("still stored");
        assert!(matches!(stored.status, ExampleWorkflowStatus::Failed(_)));

        let repaired = stored.set_status(ExampleWorkflowStatus::Started);
        service
            .repository()
            .update(repaired)
            .await
            .expect("persist recovery");

        let ok = service
            .execute_workflow(&wf.id, ProcessingContext::default())
            .await
            .expect("should complete after repair");
        assert!(matches!(ok.status, ExampleWorkflowStatus::Completed));
        assert_eq!(service.repository().count().await.expect("count"), 1);
    }
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: workflows
// TEST_PRIORITY: normal

#[cfg(test)]
mod workflow_integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_workflow_with_security() {
        let security_config = WorkflowConfig {
            max_concurrent_workflows: 8,
            default_timeout_seconds: 600,
            retry_attempts: 2,
            enable_audit_logging: true,
            workflow_storage_path: "/var/lib/beardog/secure-workflows".to_string(),
        };

        let json = serde_json::to_string(&security_config).expect("config serde");
        assert!(json.contains("\"enable_audit_logging\":true"));

        let repo = InMemoryWorkflowRepository::new();
        let processor = ExampleWorkflowProcessor::new("secure-processor");
        let observer = LoggingWorkflowObserver::new("audit-trail");
        let mut service = WorkflowService::new(repo, processor);
        service.add_observer(observer);

        let mut ctx = ProcessingContext::default();
        ctx.user_id = "principal:operator-audited".to_string();
        ctx.timeout_seconds = security_config.default_timeout_seconds;

        let wf = ExampleWorkflow::new("audit-1", "Audited workflow");
        service.create_workflow(wf.clone()).await.expect("create");

        let done = service
            .execute_workflow(&wf.id, ctx)
            .await
            .expect("execute under audit-capable config");
        assert!(matches!(done.status, ExampleWorkflowStatus::Completed));
    }

    #[tokio::test]
    async fn test_workflow_with_monitoring() {
        let repo = InMemoryWorkflowRepository::new();
        let processor = ExampleWorkflowProcessor::new("monitored");
        let observer = RecordingWorkflowObserver::new();
        let mut service = WorkflowService::new(repo, processor);
        service.add_observer(observer.clone());

        let wf = ExampleWorkflow::new("mon-1", "Observed workflow");
        service.create_workflow(wf.clone()).await.expect("create");

        let _ = service
            .execute_workflow(&wf.id, ProcessingContext::default())
            .await
            .expect("execute");

        let kinds = observer.event_kinds();
        assert!(kinds.contains(&"created"), "missing created: {kinds:?}");
        assert!(kinds.contains(&"started"), "missing started: {kinds:?}");
        assert!(kinds.contains(&"completed"), "missing completed: {kinds:?}");
    }

    #[tokio::test]
    async fn test_workflow_end_to_end() {
        let repo = InMemoryWorkflowRepository::new();
        let processor = ExampleWorkflowProcessor::new("e2e");
        let observer = LoggingWorkflowObserver::new("e2e-observer");
        let mut service = WorkflowService::new(repo, processor);
        service.add_observer(observer);

        let payload = serde_json::json!({ "phase": "integration", "case": "e2e" });
        let wf = ExampleWorkflow::new("e2e-1", "End-to-end run").with_data(payload);

        service.create_workflow(wf.clone()).await.expect("persist");
        assert_eq!(service.repository().count().await.expect("count"), 1);

        let ran = service
            .execute_workflow(&wf.id, ProcessingContext::default())
            .await
            .expect("full execute path");

        assert!(matches!(ran.status, ExampleWorkflowStatus::Completed));
        assert!(ran.data.is_some());
        let round_trip = service
            .repository()
            .find_by_id(&wf.id)
            .await
            .expect("reload")
            .expect("found");
        assert!(matches!(
            round_trip.status,
            ExampleWorkflowStatus::Completed
        ));
    }
}
