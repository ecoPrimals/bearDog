// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use crate::workflows::canonical_traits::{
    Workflow, WorkflowCommand, WorkflowId, WorkflowObserver, WorkflowProcessor, WorkflowRepository,
    WorkflowService, WorkflowStatus,
};
use beardog_errors::BearDogError;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExampleWorkflowId(pub String);

impl std::fmt::Display for ExampleWorkflowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl WorkflowId for ExampleWorkflowId {
    /// Returns as str
    fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExampleWorkflowStatus {
    /// State indicating created
    Created,
    /// State indicating started
    Started,
    /// Currently processing
    Processing,
    /// Successful completion state
    Completed,
    /// Error or failure state
    Failed(String),
    /// State indicating cancelled
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExampleWorkflow {
    pub id: ExampleWorkflowId,
    /// Name of the item
    pub name: String,
    /// Current status of the component
    pub status: ExampleWorkflowStatus,
    /// Optional data
    pub data: Option<serde_json::Value>,
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// The updated at value
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl WorkflowStatus for ExampleWorkflowStatus {
    /// Checks if terminal
    fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed(_) | Self::Cancelled)
    }

    /// Checks if active
    fn is_active(&self) -> bool {
        matches!(self, Self::Started | Self::Processing)
    }
}

impl Workflow for ExampleWorkflow {
    type Id = ExampleWorkflowId;
    type Status = ExampleWorkflowStatus;

    fn id(&self) -> &Self::Id {
        &self.id
    }

    fn status(&self) -> &Self::Status {
        &self.status
    }

    /// Creates itemd_at
    fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
}

impl ExampleWorkflow {
    /// New operation.
    /// Creates a new instance
    pub fn new(id: &str, name: &str) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: ExampleWorkflowId(id.to_string()),
            name: name.to_string(),
            status: ExampleWorkflowStatus::Created,
            data: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// With Data operation.
    /// Creates instance with data
    pub fn with_data(mut self, workflow_data: serde_json::Value) -> Self {
        self.data = Some(workflow_data);
        self
    }

    /// Set Status operation.
    /// Sets status
    /// Sets status
    pub fn set_status(mut self, status: ExampleWorkflowStatus) -> Self {
        self.status = status;
        self.updated_at = chrono::Utc::now();
        self
    }
}

/// Canonical in-memory workflow repository implementation
#[derive(Debug, Clone)]
pub struct InMemoryWorkflowRepository {
    workflows: Arc<Mutex<HashMap<String, ExampleWorkflow>>>,
}

impl InMemoryWorkflowRepository {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            workflows: Arc::new(Mutex::new(HashMap::with_capacity(16))),
        }
    }

    /// Len operation.
    pub fn len(&self) -> usize {
        self.workflows.lock().map(|w| w.len()).unwrap_or(0)
    }

    /// Is Empty operation.
    /// Checks if empty
    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.workflows.lock().map(|w| w.is_empty()).unwrap_or(true)
    }
}

impl Default for InMemoryWorkflowRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl WorkflowRepository for InMemoryWorkflowRepository {
    type Workflow = ExampleWorkflow;
    type Error = BearDogError;

    /// Saves data
    async fn save(&self, workflow: Self::Workflow) -> Result<(), Self::Error> {
        info!("Saving workflow: {}", workflow.id().as_str());
        let mut workflows = self
            .workflows
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to acquire lock: {e}")))?;
        workflows.insert(workflow.id().as_str().to_string(), workflow);
        Ok(())
    }

    async fn find_by_id(&self, id: &ExampleWorkflowId) -> Result<Option<Self::Workflow>, Self::Error> {
        let workflows = self
            .workflows
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to acquire lock: {e}")))?;
        Ok(workflows.get(id.as_str()).cloned())
    }

    async fn exists(&self, id: &ExampleWorkflowId) -> Result<bool, Self::Error> {
        let workflows = self
            .workflows
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to acquire lock: {e}")))?;
        Ok(workflows.contains_key(id.as_str()))
    }

    /// Updates item
    async fn update(&self, workflow: Self::Workflow) -> Result<(), Self::Error> {
        let mut workflows = self
            .workflows
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to acquire lock: {e}")))?;
        workflows.insert(workflow.id().as_str().to_string(), workflow);
        Ok(())
    }

    /// Removes
    async fn delete(&self, id: &ExampleWorkflowId) -> Result<(), Self::Error> {
        let mut workflows = self.workflows
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to acquire lock: {e}")))?;

        if workflows.remove(id.as_str()).is_some() {
            Ok(())
        } else {
            Err(BearDogError::not_found(format!(
                "Workflow with id {} not found",
                id.as_str()
            )))
        }
    }

    fn list_all(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Self::Workflow>, Self::Error>> + Send {
        let workflows = self.workflows.clone();
        async move {
            let workflows = workflows
                .lock()
                .map_err(|e| BearDogError::internal(format!("Failed to acquire lock: {e}")))?;
            Ok(workflows.values().cloned().collect())
        }
    }

    fn count(&self) -> impl std::future::Future<Output = Result<usize, Self::Error>> + Send {
        let workflows = self.workflows.clone();
        async move {
            let workflows = workflows
                .lock()
                .map_err(|e| BearDogError::internal(format!("Failed to acquire lock: {e}")))?;
            Ok(workflows.len())
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessingContext {
    pub user_id: String,
    pub timeout_seconds: u64,
    /// Number of retry
    pub retry_count: u32,
}

impl Default for ProcessingContext {
    fn default() -> Self {
        Self {
            user_id: "system".to_string(),
            timeout_seconds: 30,
            retry_count: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExampleWorkflowProcessor {
    /// Name of the item
    pub name: &'static str,
}

impl ExampleWorkflowProcessor {
    /// New operation.
    /// Creates a new instance
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl WorkflowProcessor for ExampleWorkflowProcessor {
    type Workflow = ExampleWorkflow;
    type Context = ProcessingContext;
    type Error = BearDogError;

    /// Validates input
    async fn validate(&self, workflow: &Self::Workflow) -> Result<(), Self::Error> {
        // Canonical validation logic
        if workflow.id().0.is_empty() {
            return Err(BearDogError::validation("Workflow ID cannot be empty"));
        }
        if workflow.name.is_empty() {
            return Err(BearDogError::validation("Workflow name cannot be empty"));
        }
        Ok(())
    }

    /// Processes data
    fn process(
        &self,
        mut workflow: Self::Workflow,
        _context: Self::Context,
    ) -> impl std::future::Future<Output = Result<Self::Workflow, Self::Error>> + Send {
        let processor_name = self.name;
        async move {
            info!(
                "Processing workflow {} with processor {}",
                workflow.id().as_str(),
                processor_name
            );

            if workflow.name.is_empty() {
                return Err(BearDogError::validation("Workflow name cannot be empty"));
            }

            if matches!(workflow.status, ExampleWorkflowStatus::Failed(_)) {
                return Err(BearDogError::validation("Cannot process failed workflow"));
            }

            workflow = workflow.set_status(ExampleWorkflowStatus::Processing);
            sleep(Duration::from_millis(100)); // Simulate work

            let processed_data = serde_json::json!({
                "processed_by": processor_name,
                "processed_at": chrono::Utc::now(),
                "original_data": workflow.data
            });

            workflow = workflow
                .with_data(processed_data)
                .set_status(ExampleWorkflowStatus::Completed);

            info!("Successfully processed workflow {}", workflow.id().as_str());
            Ok(workflow)
        }
    }

    fn can_process(&self, workflow: &Self::Workflow) -> bool {
        matches!(
            workflow.status,
            ExampleWorkflowStatus::Created | ExampleWorkflowStatus::Started
        )
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoggingWorkflowObserver {
    /// Name of the item
    pub name: String,
}

impl LoggingWorkflowObserver {
    /// New operation.
    /// Creates a new instance
    pub fn new<'a>(name: impl Into<&'a str>) -> Self {
        Self {
            name: name.into().to_string(),
        }
    }
}

impl WorkflowObserver for LoggingWorkflowObserver {
    type Workflow = ExampleWorkflow;
    type Error = BearDogError;

    async fn on_created(&self, workflow: &Self::Workflow) -> Result<(), Self::Error> {
        let name = &self.name;
        let workflow_id = workflow.id().as_str();
        let workflow_name = &workflow.name;
        info!(
            "[{}] Workflow created: {} ({})",
            name, workflow_id, workflow_name
        );
        Ok(())
    }

    async fn on_started(&self, workflow: &Self::Workflow) -> Result<(), Self::Error> {
        let name = &self.name;
        let workflow_id = workflow.id().as_str();
        let workflow_name = &workflow.name;
        info!(
            "[{}] Workflow started: {} ({})",
            name, workflow_id, workflow_name
        );
        Ok(())
    }

    async fn on_completed(&self, workflow: &Self::Workflow) -> Result<(), Self::Error> {
        let name = &self.name;
        let workflow_id = workflow.id().as_str();
        let workflow_name = &workflow.name;
        info!(
            "[{}] Workflow completed: {} ({})",
            name, workflow_id, workflow_name
        );
        Ok(())
    }

    async fn on_failed(&self, workflow: &Self::Workflow, error: &str) -> Result<(), Self::Error> {
        let name = &self.name;
        let workflow_id = workflow.id().as_str();
        let workflow_name = &workflow.name;
        error!(
            "[{}] Workflow failed: {} ({}) - Error: {}",
            name, workflow_id, workflow_name, error
        );
        Ok(())
    }

    async fn on_cancelled(&self, workflow: &Self::Workflow) -> Result<(), Self::Error> {
        let name = &self.name;
        let workflow_id = workflow.id().as_str();
        let workflow_name = &workflow.name;
        warn!(
            "[{}] Workflow cancelled: {} ({})",
            name, workflow_id, workflow_name
        );
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StartWorkflowCommand {
    /// The context value
    pub context: ProcessingContext,
}

impl StartWorkflowCommand {
    /// New operation.
    /// Creates a new instance
    pub fn new(context: ProcessingContext) -> Self {
        Self { context }
    }

    /// Context operation.
    pub fn context(&self) -> &ProcessingContext {
        &self.context
    }
}

impl WorkflowCommand for StartWorkflowCommand {
    type Workflow = ExampleWorkflow;
    type Result = ExampleWorkflow;
    type Error = BearDogError;

    /// Executes operation
    async fn execute(&self, mut workflow: Self::Workflow) -> Result<Self::Result, Self::Error> {
        info!(
            "Executing StartWorkflowCommand for workflow: {}",
            workflow.id().as_str()
        );

        if !matches!(workflow.status, ExampleWorkflowStatus::Created) {
            return Err(BearDogError::validation("Workflow status is not Created"));
        }

        workflow.status = ExampleWorkflowStatus::Started;
        Ok(workflow)
    }

    fn can_execute(&self, workflow: &Self::Workflow) -> bool {
        matches!(workflow.status, ExampleWorkflowStatus::Created)
    }

    fn description(&self) -> &'static str {
        "Start a created workflow"
    }
}

/// Run Comprehensive Example operation.
///
/// # Errors
/// Returns an error if the operation fails.
/// Runs comprehensive_example
pub async fn run_comprehensive_example() -> Result<(), BearDogError> {
    info!("🚀 Starting comprehensive workflow example");

    let repository = InMemoryWorkflowRepository::new();
    let processor = ExampleWorkflowProcessor::new("ExampleProcessor");
    let observer = LoggingWorkflowObserver::new("MainObserver");

    let mut service = WorkflowService::new(repository, processor);
    service.add_observer(observer);

    let workflow =
        ExampleWorkflow::new("example-001", "Example Workflow").with_data(serde_json::json!({
            "input": "test data",
            "priority": "high"
        }));

    info!("🔄 Created workflow: {}", workflow.id().as_str());

    let _start_command = StartWorkflowCommand::new(ProcessingContext {
        user_id: "user123".to_string(),
        timeout_seconds: 60,
        retry_count: 2,
    });

    service.create_workflow(workflow.clone()).await?;

    // Process workflow using processor directly
    let result = service
        .processor
        .process(workflow.clone(), ProcessingContext::default())
        .await
        .map_err(|e| BearDogError::system(format!("Failed to process workflow: {e:?}")))?;

    match serde_json::to_string_pretty(&result) {
        Ok(json) => info!("📄 Final workflow data: {}", json),
        Err(e) => warn!("Failed to serialize workflow data: {}", e),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    fn test_repository_operations() -> Result<(), BearDogError> {
        let repo = InMemoryWorkflowRepository::new();
        let workflow = ExampleWorkflow::new("test-1", "Test Workflow");
        let workflow_id = workflow.id().clone();

        repo.save(workflow.clone())
            .map_err(|e| BearDogError::system(format!("Failed to save workflow: {e:?}")))?;
        assert_eq!(
            repo.count()
                .map_err(|e| BearDogError::system(format!("Failed to count workflows: {e:?}")))?,
            1
        );

        let found = repo
            .find_by_id(&workflow_id)
            .map_err(|e| BearDogError::system(format!("Failed to find workflow: {e:?}")))?;
        assert!(found.is_some());
        let found_workflow =
            found.ok_or_else(|| BearDogError::system("Expected workflow not found".to_string()))?;
        assert_eq!(found_workflow.name, "Test Workflow");

        assert!(repo
            .exists(&workflow_id)
            .map_err(|e| BearDogError::system(format!(
                "Failed to check workflow existence: {e:?}"
            )))?);

        let updated = workflow.set_status(ExampleWorkflowStatus::Completed);
        repo.update(updated)
            .map_err(|e| BearDogError::system(format!("Failed to update workflow: {e:?}")))?;

        repo.delete(&workflow_id)
            .map_err(|e| BearDogError::system(format!("Failed to delete workflow: {e:?}")))?;
        assert_eq!(
            repo.count().map_err(|e| BearDogError::system(format!(
                "Failed to count workflows after delete: {e:?}"
            )))?,
            0
        );

        Ok(())
    }

    #[tokio::test]
    fn test_processor() -> Result<(), BearDogError> {
        let processor = ExampleWorkflowProcessor::new("TestProcessor");
        let workflow = ExampleWorkflow::new("test-2", "Test Processing");
        let context = ProcessingContext::default();

        assert!(processor.can_process(&workflow));

        let result = processor
            .process(workflow, context)
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
    fn test_observer() -> Result<(), BearDogError> {
        let observer = LoggingWorkflowObserver::new("TestObserver");
        let workflow = ExampleWorkflow::new("test-3", "Test Observer");

        observer
            .on_created(&workflow)
            .map_err(|e| BearDogError::system(format!("Failed on_created: {e:?}")))?;
        observer
            .on_started(&workflow)
            .map_err(|e| BearDogError::system(format!("Failed on_started: {e:?}")))?;
        observer
            .on_completed(&workflow)
            .map_err(|e| BearDogError::system(format!("Failed on_completed: {e:?}")))?;
        observer
            .on_failed(&workflow, "test error")
            .map_err(|e| BearDogError::system(format!("Failed on_failed: {e:?}")))?;
        observer
            .on_cancelled(&workflow)
            .map_err(|e| BearDogError::system(format!("Failed on_cancelled: {e:?}")))?;

        Ok(())
    }

    #[tokio::test]
    fn test_command() -> Result<(), BearDogError> {
        let command = StartWorkflowCommand::new(ProcessingContext::default());
        let workflow = ExampleWorkflow::new("test-4", "Test Command");

        assert!(command.can_execute(&workflow));
        assert_eq!(command.description(), "Start a created workflow");

        let result = command
            .execute(workflow)
            .map_err(|e| BearDogError::system(format!("Failed to execute command: {e:?}")))?;
        assert!(matches!(result.status, ExampleWorkflowStatus::Started));

        Ok(())
    }

    #[tokio::test]
    fn test_full_integration() -> Result<(), BearDogError> {
        let repository = InMemoryWorkflowRepository::new();
        let processor = ExampleWorkflowProcessor::new("IntegrationProcessor");
        let observer = LoggingWorkflowObserver::new("IntegrationObserver");

        let mut service = WorkflowService::new(repository, processor);
        service.add_observer(observer);

        let workflow = ExampleWorkflow::new("integration-test", "Integration Test Workflow");

        service
            .create_workflow(workflow.clone())
            .map_err(|e| BearDogError::system(format!("Failed to create workflow: {e:?}")))?;
        // Execute workflow using the service
        let execution_result =
            service.execute_workflow(workflow.id(), ProcessingContext::default());

        assert!(execution_result.is_ok());
        assert_eq!(
            service
                .repository()
                .count()
                .map_err(|e| BearDogError::system(format!("Failed to count workflows: {e:?}")))?,
            1
        );

        Ok(())
    }
}
