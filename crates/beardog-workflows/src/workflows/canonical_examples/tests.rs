// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for canonical workflow examples

use super::commands::StartWorkflowCommand;
use super::example::run_comprehensive_example;
use super::observer::LoggingWorkflowObserver;
use super::processor::ExampleWorkflowProcessor;
use super::repository::InMemoryWorkflowRepository;
use super::types::{ExampleWorkflow, ExampleWorkflowId, ExampleWorkflowStatus, ProcessingContext};
use crate::workflows::canonical_traits::{
    Workflow, WorkflowCommand, WorkflowId, WorkflowObserver, WorkflowProcessor, WorkflowRepository,
    WorkflowService, WorkflowStatus,
};
use beardog_errors::BearDogError;
use std::sync::Arc;

#[tokio::test]
async fn test_repository_operations() -> Result<(), BearDogError> {
    let repo = InMemoryWorkflowRepository::new();
    let workflow = ExampleWorkflow::new("test-1", "Test Workflow");
    let workflow_id = workflow.id().clone();

    repo.save(workflow.clone())
        .await
        .map_err(|e| BearDogError::system(format!("Failed to save workflow: {e:?}")))?;
    assert_eq!(
        repo.count()
            .await
            .map_err(|e| BearDogError::system(format!("Failed to count workflows: {e:?}")))?,
        1
    );

    let found = repo
        .find_by_id(&workflow_id)
        .await
        .map_err(|e| BearDogError::system(format!("Failed to find workflow: {e:?}")))?;
    assert!(found.is_some());
    let found_workflow =
        found.ok_or_else(|| BearDogError::system("Expected workflow not found".to_string()))?;
    assert_eq!(found_workflow.name, "Test Workflow");

    assert!(repo
        .exists(&workflow_id)
        .await
        .map_err(|e| BearDogError::system(format!("Failed to check workflow existence: {e:?}")))?);

    let updated = workflow.set_status(ExampleWorkflowStatus::Completed);
    repo.update(updated)
        .await
        .map_err(|e| BearDogError::system(format!("Failed to update workflow: {e:?}")))?;

    repo.delete(&workflow_id)
        .await
        .map_err(|e| BearDogError::system(format!("Failed to delete workflow: {e:?}")))?;
    assert_eq!(
        repo.count()
            .await
            .map_err(|e| BearDogError::system(format!(
                "Failed to count workflows after delete: {e:?}"
            )))?,
        0
    );

    Ok(())
}

#[tokio::test]
async fn test_processor() -> Result<(), BearDogError> {
    let processor = ExampleWorkflowProcessor::new("TestProcessor");
    let workflow = ExampleWorkflow::new("test-2", "Test Processing");
    let context = ProcessingContext::default();

    assert!(processor.can_process(&workflow));

    let result = processor
        .process(workflow, context)
        .await
        .map_err(|e| BearDogError::system(format!("Failed to process workflow: {e:?}")))?;
    assert!(matches!(result.status, ExampleWorkflowStatus::Completed));

    let processed_by = result
        .data
        .as_ref()
        .and_then(|data| data["processed_by"].as_str())
        .ok_or_else(|| BearDogError::system("Missing processed_by field".to_string()))?;
    assert_eq!(processed_by, "TestProcessor");

    Ok(())
}

#[tokio::test]
async fn test_observer() -> Result<(), BearDogError> {
    let observer = LoggingWorkflowObserver::new("TestObserver");
    let workflow = ExampleWorkflow::new("test-3", "Test Observer");

    observer
        .on_created(&workflow)
        .await
        .map_err(|e| BearDogError::system(format!("Failed on_created: {e:?}")))?;
    observer
        .on_started(&workflow)
        .await
        .map_err(|e| BearDogError::system(format!("Failed on_started: {e:?}")))?;
    observer
        .on_completed(&workflow)
        .await
        .map_err(|e| BearDogError::system(format!("Failed on_completed: {e:?}")))?;
    observer
        .on_failed(&workflow, "test error")
        .await
        .map_err(|e| BearDogError::system(format!("Failed on_failed: {e:?}")))?;
    observer
        .on_cancelled(&workflow)
        .await
        .map_err(|e| BearDogError::system(format!("Failed on_cancelled: {e:?}")))?;

    Ok(())
}

#[tokio::test]
async fn test_command() -> Result<(), BearDogError> {
    let command = StartWorkflowCommand::new(ProcessingContext::default());
    let workflow = ExampleWorkflow::new("test-4", "Test Command");

    assert!(command.can_execute(&workflow));
    assert_eq!(command.description(), "Start a created workflow");

    let result = command
        .execute(workflow)
        .await
        .map_err(|e| BearDogError::system(format!("Failed to execute command: {e:?}")))?;
    assert!(matches!(result.status, ExampleWorkflowStatus::Started));

    Ok(())
}

#[tokio::test]
async fn test_full_integration() -> Result<(), BearDogError> {
    let repository = InMemoryWorkflowRepository::new();
    let processor = ExampleWorkflowProcessor::new("IntegrationProcessor");
    let observer = LoggingWorkflowObserver::new("IntegrationObserver");

    let mut service = WorkflowService::new(repository, processor);
    service.add_observer(observer);

    let workflow = ExampleWorkflow::new("integration-test", "Integration Test Workflow");

    service
        .create_workflow(workflow.clone())
        .await
        .map_err(|e| BearDogError::system(format!("Failed to create workflow: {e:?}")))?;
    let execution_result = service
        .execute_workflow(workflow.id(), ProcessingContext::default())
        .await;

    assert!(execution_result.is_ok());
    assert_eq!(
        service
            .repository()
            .count()
            .await
            .map_err(|e| BearDogError::system(format!("Failed to count workflows: {e:?}")))?,
        1
    );

    Ok(())
}

#[test]
fn test_example_workflow_id_display() {
    let id = ExampleWorkflowId("test-id-123".to_string());
    assert_eq!(format!("{}", id), "test-id-123");
    assert_eq!(id.as_str(), "test-id-123");
}

#[test]
fn test_example_workflow_status_is_terminal() {
    assert!(ExampleWorkflowStatus::Completed.is_terminal());
    assert!(ExampleWorkflowStatus::Failed("error".to_string()).is_terminal());
    assert!(ExampleWorkflowStatus::Cancelled.is_terminal());
    assert!(!ExampleWorkflowStatus::Created.is_terminal());
    assert!(!ExampleWorkflowStatus::Started.is_terminal());
    assert!(!ExampleWorkflowStatus::Processing.is_terminal());
}

#[test]
fn test_example_workflow_status_is_active() {
    assert!(ExampleWorkflowStatus::Started.is_active());
    assert!(ExampleWorkflowStatus::Processing.is_active());
    assert!(!ExampleWorkflowStatus::Created.is_active());
    assert!(!ExampleWorkflowStatus::Completed.is_active());
    assert!(!ExampleWorkflowStatus::Failed("error".to_string()).is_active());
    assert!(!ExampleWorkflowStatus::Cancelled.is_active());
}

#[test]
fn test_example_workflow_new() {
    let workflow = ExampleWorkflow::new("wf-001", "Test Workflow");
    assert_eq!(workflow.id().as_str(), "wf-001");
    assert_eq!(workflow.name, "Test Workflow");
    assert!(matches!(workflow.status, ExampleWorkflowStatus::Created));
    assert!(workflow.data.is_none());
}

#[test]
fn test_example_workflow_with_data() {
    let workflow =
        ExampleWorkflow::new("wf-002", "Data Test").with_data(serde_json::json!({"key": "value"}));
    assert!(workflow.data.is_some());
    assert_eq!(workflow.data.unwrap()["key"], "value");
}

#[test]
fn test_example_workflow_set_status() {
    let workflow =
        ExampleWorkflow::new("wf-003", "Status Test").set_status(ExampleWorkflowStatus::Started);
    assert!(matches!(workflow.status, ExampleWorkflowStatus::Started));
}

#[test]
fn test_example_workflow_trait_methods() {
    let workflow = ExampleWorkflow::new("wf-004", "Trait Test");
    assert_eq!(workflow.id().as_str(), "wf-004");
    assert!(matches!(workflow.status(), &ExampleWorkflowStatus::Created));
    assert!(workflow.created_at() <= chrono::Utc::now());
}

#[test]
fn test_in_memory_repository_default() {
    let repo = InMemoryWorkflowRepository::default();
    assert!(repo.is_empty());
    assert_eq!(repo.len(), 0);
}

#[tokio::test]
async fn test_repository_list_all() -> Result<(), BearDogError> {
    let repo = InMemoryWorkflowRepository::new();
    let wf1 = ExampleWorkflow::new("list-1", "First");
    let wf2 = ExampleWorkflow::new("list-2", "Second");
    let wf3 = ExampleWorkflow::new("list-3", "Third");

    repo.save(wf1).await?;
    repo.save(wf2).await?;
    repo.save(wf3).await?;

    let all = repo.list_all().await?;
    assert_eq!(all.len(), 3);
    Ok(())
}

#[tokio::test]
async fn test_repository_delete_nonexistent() {
    let repo = InMemoryWorkflowRepository::new();
    let id = ExampleWorkflowId("nonexistent".to_string());
    let result = repo.delete(&id).await;
    assert!(result.is_err());
}

#[test]
fn test_processing_context_default() {
    let context = ProcessingContext::default();
    assert_eq!(context.user_id, "system");
    assert_eq!(context.timeout_seconds, 30);
    assert_eq!(context.retry_count, 3);
}

#[test]
fn test_example_workflow_processor_new() {
    let processor = ExampleWorkflowProcessor::new("TestProc");
    assert_eq!(processor.name(), "TestProc");
}

#[tokio::test]
async fn test_processor_validation() -> Result<(), BearDogError> {
    let processor = ExampleWorkflowProcessor::new("ValidationTest");
    let valid = ExampleWorkflow::new("valid-1", "Valid Workflow");
    assert!(processor.validate(&valid).await.is_ok());

    let invalid_id = ExampleWorkflow::new("", "No ID");
    assert!(processor.validate(&invalid_id).await.is_err());

    let mut invalid_name = ExampleWorkflow::new("name-test", "");
    invalid_name.name = String::new();
    assert!(processor.validate(&invalid_name).await.is_err());

    Ok(())
}

#[test]
fn test_processor_can_process() {
    let processor = ExampleWorkflowProcessor::new("CanProcessTest");
    let created =
        ExampleWorkflow::new("cp-1", "Created").set_status(ExampleWorkflowStatus::Created);
    let started =
        ExampleWorkflow::new("cp-2", "Started").set_status(ExampleWorkflowStatus::Started);
    let processing =
        ExampleWorkflow::new("cp-3", "Processing").set_status(ExampleWorkflowStatus::Processing);
    let completed =
        ExampleWorkflow::new("cp-4", "Completed").set_status(ExampleWorkflowStatus::Completed);

    assert!(processor.can_process(&created));
    assert!(processor.can_process(&started));
    assert!(!processor.can_process(&processing));
    assert!(!processor.can_process(&completed));
}

#[tokio::test]
async fn test_processor_failed_workflow() {
    let processor = ExampleWorkflowProcessor::new("FailedTest");
    let failed = ExampleWorkflow::new("fail-1", "Failed Test")
        .set_status(ExampleWorkflowStatus::Failed("Previous error".to_string()));

    let result = processor
        .process(failed, ProcessingContext::default())
        .await;
    assert!(result.is_err());
}

#[test]
fn test_logging_observer_new() {
    let observer = LoggingWorkflowObserver::new("ObserverTest");
    assert_eq!(observer.name, "ObserverTest");
}

#[test]
fn test_start_command_new() {
    let context = ProcessingContext::default();
    let command = StartWorkflowCommand::new(context.clone());
    assert_eq!(command.context(), &context);
}

#[test]
fn test_start_command_can_execute() {
    let command = StartWorkflowCommand::new(ProcessingContext::default());
    let created = ExampleWorkflow::new("cmd-1", "Created");
    let started =
        ExampleWorkflow::new("cmd-2", "Started").set_status(ExampleWorkflowStatus::Started);

    assert!(command.can_execute(&created));
    assert!(!command.can_execute(&started));
}

#[tokio::test]
async fn test_start_command_execute_invalid_status() {
    let command = StartWorkflowCommand::new(ProcessingContext::default());
    let already_started = ExampleWorkflow::new("invalid-1", "Already Started")
        .set_status(ExampleWorkflowStatus::Started);

    let result = command.execute(already_started).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_workflow_with_chained_operations() {
    let workflow = ExampleWorkflow::new("chain-1", "Chained Test")
        .with_data(serde_json::json!({"step": 1}))
        .set_status(ExampleWorkflowStatus::Started);

    assert!(workflow.data.is_some());
    assert!(matches!(workflow.status, ExampleWorkflowStatus::Started));
}

#[tokio::test]
async fn test_run_comprehensive_example() {
    let result = run_comprehensive_example().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_processor_empty_name_validation() {
    let processor = ExampleWorkflowProcessor::new("EmptyNameTest");
    let mut workflow = ExampleWorkflow::new("empty-name", "");
    workflow.name = String::new();
    let result = processor
        .process(workflow, ProcessingContext::default())
        .await;
    assert!(result.is_err());
}

#[test]
fn test_example_workflow_serialization_roundtrip() {
    let workflow = ExampleWorkflow::new("serde-1", "Serde Test")
        .with_data(serde_json::json!({"key": "value"}));
    let json = serde_json::to_string(&workflow).unwrap();
    let restored: ExampleWorkflow = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.id.as_str(), "serde-1");
    assert_eq!(restored.name, "Serde Test");
    assert!(restored.data.is_some());
}

#[test]
fn test_example_workflow_status_serialization() {
    for status in [
        ExampleWorkflowStatus::Created,
        ExampleWorkflowStatus::Started,
        ExampleWorkflowStatus::Processing,
        ExampleWorkflowStatus::Completed,
        ExampleWorkflowStatus::Failed("test error".to_string()),
        ExampleWorkflowStatus::Cancelled,
    ] {
        let json = serde_json::to_string(&status).unwrap();
        let restored: ExampleWorkflowStatus = serde_json::from_str(&json).unwrap();
        assert_eq!(status, restored);
    }
}

#[test]
fn test_processing_context_serialization() {
    let ctx = ProcessingContext {
        user_id: "user-42".to_string(),
        timeout_seconds: 120,
        retry_count: 5,
    };
    let json = serde_json::to_string(&ctx).unwrap();
    let restored: ProcessingContext = serde_json::from_str(&json).unwrap();
    assert_eq!(ctx, restored);
}

#[test]
fn test_start_workflow_command_serialization() {
    let cmd = StartWorkflowCommand::new(ProcessingContext::default());
    let json = serde_json::to_string(&cmd).unwrap();
    let restored: StartWorkflowCommand = serde_json::from_str(&json).unwrap();
    assert_eq!(cmd, restored);
}

#[test]
fn test_example_workflow_id_hash() {
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(ExampleWorkflowId("a".to_string()));
    set.insert(ExampleWorkflowId("b".to_string()));
    set.insert(ExampleWorkflowId("a".to_string())); // duplicate
    assert_eq!(set.len(), 2);
}

#[test]
fn test_example_workflow_debug() {
    let workflow = ExampleWorkflow::new("debug-1", "Debug Test");
    let debug = format!("{:?}", workflow);
    assert!(debug.contains("debug-1"));
    assert!(debug.contains("Debug Test"));
}

#[test]
fn test_in_memory_repository_clone() {
    let repo = InMemoryWorkflowRepository::new();
    let cloned = repo.clone();
    assert!(cloned.is_empty());
}

#[test]
fn test_logging_observer_serialization() {
    let observer = LoggingWorkflowObserver::new("SerdeObserver");
    let json = serde_json::to_string(&observer).unwrap();
    let restored: LoggingWorkflowObserver = serde_json::from_str(&json).unwrap();
    assert_eq!(observer, restored);
}

#[tokio::test]
async fn test_repository_concurrent_operations() {
    let repo = Arc::new(InMemoryWorkflowRepository::new());
    let mut handles = vec![];

    for i in 0..8 {
        let r = Arc::clone(&repo);
        handles.push(tokio::spawn(async move {
            let workflow =
                ExampleWorkflow::new(&format!("concurrent-{}", i), &format!("Concurrent {}", i));
            r.save(workflow).await
        }));
    }

    for h in handles {
        let result = h.await.unwrap();
        assert!(result.is_ok());
    }

    assert_eq!(repo.len(), 8);

    let all = repo.list_all().await.unwrap();
    assert_eq!(all.len(), 8);
}

#[tokio::test]
async fn test_service_create_and_find_workflow() {
    let repo = InMemoryWorkflowRepository::new();
    let processor = ExampleWorkflowProcessor::new("ServiceTest");
    let observer = LoggingWorkflowObserver::new("ServiceObserver");

    let mut service = WorkflowService::new(repo, processor);
    service.add_observer(observer);

    let wf = ExampleWorkflow::new("svc-1", "Service Test");
    service.create_workflow(wf.clone()).await.unwrap();

    let found = service.repository().find_by_id(wf.id()).await.unwrap();
    assert!(found.is_some());
}
