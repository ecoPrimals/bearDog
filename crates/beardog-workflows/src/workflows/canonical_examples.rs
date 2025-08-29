// Comprehensive examples demonstrating the new canonical workflow traits
// These examples show proper usage of Repository, Service, Observer, and Command patterns

use crate::workflows::canonical_traits::{
    Workflow, WorkflowCommand, WorkflowId, WorkflowObserver, WorkflowProcessor, WorkflowRepository,
    WorkflowService, WorkflowStatus,
};
use beardog_errors::BearDogError;
// Removed async_trait - using native async fn in traits
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};

// ================================
// EXAMPLE WORKFLOW TYPES
// ================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExampleWorkflowId(pub String);

impl std::fmt::Display for ExampleWorkflowId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl WorkflowId for ExampleWorkflowId {
    fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleWorkflow {
    pub id: ExampleWorkflowId,
    pub name: String,
    pub status: ExampleWorkflowStatus,
    pub data: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExampleWorkflowStatus {
    Created,
    Started,
    Processing,
    Completed,
    Failed(String),
    Cancelled,
}

impl WorkflowStatus for ExampleWorkflowStatus {
    fn is_terminal(&self) -> bool {
        matches!(self, Self::Completed | Self::Failed(_) | Self::Cancelled)
    }

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

    fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }
}

impl ExampleWorkflow {
    pub fn new(id: String, name: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: ExampleWorkflowId(id),
            name,
            status: ExampleWorkflowStatus::Created,
            data: serde_json::json!({}),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_data(mut self, data: serde_json::Value) -> Self {
        self.data = data;
        self.updated_at = chrono::Utc::now();
        self
    }

    pub fn set_status(mut self, status: ExampleWorkflowStatus) -> Self {
        self.status = status;
        self.updated_at = chrono::Utc::now();
        self
    }
}

// ================================
// REPOSITORY IMPLEMENTATION
// ================================

/// In-memory repository implementation for examples
#[derive(Debug)]
pub struct InMemoryWorkflowRepository {
    workflows: Arc<Mutex<HashMap<ExampleWorkflowId, ExampleWorkflow>>>,
}

impl InMemoryWorkflowRepository {
    pub fn new() -> Self {
        Self {
            workflows: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn len(&self) -> usize {
        self.workflows.lock().map(|w| w.len()).unwrap_or(0)
    }

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

    async fn save(&self, workflow: Self::Workflow) -> Result<(), Self::Error> {
        info!("Saving workflow: {}", workflow.id().as_str());
        let mut workflows = self
            .workflows
            .lock()
            .map_err(|e| BearDogError::internal(format!("Failed to acquire lock: {e}")))?;
        workflows.insert(workflow.id().clone(), workflow);
        Ok(())
    }

    fn find_by_id(
        &self,
        id: &<Self::Workflow as Workflow>::Id,
    ) -> impl std::future::Future<Output = Result<Option<Self::Workflow>, Self::Error>> + Send {
        let id = id.clone();
        let workflows = self.workflows.clone();
        async move {
            let workflows = workflows
                .lock()
                .map_err(|e| BearDogError::internal(format!("Failed to acquire lock: {e}")))?;
            Ok(workflows.get(&id).cloned())
        }
    }

    fn update(
        &self,
        workflow: Self::Workflow,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let workflows = self.workflows.clone();
        async move {
            info!("Updating workflow: {}", workflow.id().as_str());
            let mut workflows = workflows
                .lock()
                .map_err(|e| BearDogError::internal(format!("Failed to acquire lock: {e}")))?;

            if workflows.contains_key(workflow.id()) {
                workflows.insert(workflow.id().clone(), workflow);
                Ok(())
            } else {
                Err(BearDogError::not_found(format!(
                    "Workflow not found: {}",
                    workflow.id().as_str()
                )))
            }
        }
    }

    fn delete(
        &self,
        id: &<Self::Workflow as Workflow>::Id,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let id = id.clone();
        let workflows = self.workflows.clone();
        async move {
            info!("Deleting workflow: {}", id.as_str());
            let mut workflows = workflows
                .lock()
                .map_err(|e| BearDogError::internal(format!("Failed to acquire lock: {e}")))?;

            if workflows.remove(&id).is_some() {
                Ok(())
            } else {
                Err(BearDogError::not_found(format!(
                    "Workflow not found: {}",
                    id.as_str()
                )))
            }
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

    fn exists(
        &self,
        id: &<Self::Workflow as Workflow>::Id,
    ) -> impl std::future::Future<Output = Result<bool, Self::Error>> + Send {
        let id = id.clone();
        let workflows = self.workflows.clone();
        async move {
            let workflows = workflows
                .lock()
                .map_err(|e| BearDogError::internal(format!("Failed to acquire lock: {e}")))?;
            Ok(workflows.contains_key(&id))
        }
    }
}

// ================================
// PROCESSOR IMPLEMENTATION
// ================================

#[derive(Debug, Clone)]
pub struct ProcessingContext {
    pub user_id: String,
    pub timeout_secs: u64,
    pub retry_count: u32,
}

impl Default for ProcessingContext {
    fn default() -> Self {
        Self {
            user_id: "system".to_string(),
            timeout_secs: 30,
            retry_count: 3,
        }
    }
}

/// Example processor that simulates some work
#[derive(Debug)]
pub struct ExampleWorkflowProcessor {
    name: &'static str,
}

impl ExampleWorkflowProcessor {
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

impl WorkflowProcessor for ExampleWorkflowProcessor {
    type Workflow = ExampleWorkflow;
    type Context = ProcessingContext;
    type Error = BearDogError;

    fn process(
        &self,
        mut workflow: Self::Workflow,
        context: Self::Context,
    ) -> impl std::future::Future<Output = Result<Self::Workflow, Self::Error>> + Send {
        let processor_name = self.name;
        async move {
            info!(
                "Processing workflow {} with processor {}",
                workflow.id().as_str(),
                processor_name
            );

            // Validate before processing - we need to inline this since we can't call self methods
            if workflow.name.is_empty() {
                return Err(BearDogError::validation(
                    "Workflow name cannot be empty".to_string(),
                ));
            }

            if matches!(workflow.status, ExampleWorkflowStatus::Failed(_)) {
                return Err(BearDogError::validation(
                    "Cannot process failed workflow".to_string(),
                ));
            }

            // Simulate processing work
            workflow = workflow.set_status(ExampleWorkflowStatus::Processing);
            sleep(Duration::from_millis(100)).await; // Simulate work

            // Update workflow data based on processing
            let processed_data = serde_json::json!({
                "processed_by": processor_name,
                "processed_at": chrono::Utc::now().to_rfc3339(),
                "user_id": context.user_id,
                "original_data": workflow.data
            });

            workflow = workflow
                .with_data(processed_data)
                .set_status(ExampleWorkflowStatus::Completed);

            info!("Successfully processed workflow {}", workflow.id().as_str());
            Ok(workflow)
        }
    }

    fn validate(
        &self,
        workflow: &Self::Workflow,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let workflow_name = workflow.name.clone();
        let workflow_status = workflow.status.clone();
        async move {
            if workflow_name.is_empty() {
                return Err(BearDogError::validation(
                    "Workflow name cannot be empty".to_string(),
                ));
            }

            if matches!(workflow_status, ExampleWorkflowStatus::Failed(_)) {
                return Err(BearDogError::validation(
                    "Cannot process failed workflow".to_string(),
                ));
            }

            Ok(())
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

// ================================
// OBSERVER IMPLEMENTATION
// ================================

/// Example observer that logs workflow events
#[derive(Debug)]
pub struct LoggingWorkflowObserver {
    name: String,
}

impl LoggingWorkflowObserver {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl WorkflowObserver for LoggingWorkflowObserver {
    type Workflow = ExampleWorkflow;
    type Error = BearDogError;

    fn on_created(
        &self,
        workflow: &Self::Workflow,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let name = self.name.clone();
        let workflow_id = workflow.id().as_str().to_string();
        let workflow_name = workflow.name.clone();
        async move {
            info!(
                "[{}] Workflow created: {} ({})",
                name, workflow_id, workflow_name
            );
            Ok(())
        }
    }

    fn on_started(
        &self,
        workflow: &Self::Workflow,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let name = self.name.clone();
        let workflow_id = workflow.id().as_str().to_string();
        let workflow_name = workflow.name.clone();
        async move {
            info!(
                "[{}] Workflow started: {} ({})",
                name, workflow_id, workflow_name
            );
            Ok(())
        }
    }

    fn on_completed(
        &self,
        workflow: &Self::Workflow,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let name = self.name.clone();
        let workflow_id = workflow.id().as_str().to_string();
        let workflow_name = workflow.name.clone();
        async move {
            info!(
                "[{}] Workflow completed: {} ({})",
                name, workflow_id, workflow_name
            );
            Ok(())
        }
    }

    fn on_failed(
        &self,
        workflow: &Self::Workflow,
        error: &str,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let name = self.name.clone();
        let workflow_id = workflow.id().as_str().to_string();
        let workflow_name = workflow.name.clone();
        let error = error.to_string();
        async move {
            error!(
                "[{}] Workflow failed: {} ({}) - Error: {}",
                name, workflow_id, workflow_name, error
            );
            Ok(())
        }
    }

    fn on_cancelled(
        &self,
        workflow: &Self::Workflow,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        let name = self.name.clone();
        let workflow_id = workflow.id().as_str().to_string();
        let workflow_name = workflow.name.clone();
        async move {
            warn!(
                "[{}] Workflow cancelled: {} ({})",
                name, workflow_id, workflow_name
            );
            Ok(())
        }
    }
}

// ================================
// COMMAND IMPLEMENTATION
// ================================

/// Command to start a workflow
#[derive(Debug)]
pub struct StartWorkflowCommand {
    context: ProcessingContext,
}

impl StartWorkflowCommand {
    pub fn new(context: ProcessingContext) -> Self {
        Self { context }
    }

    /// Get the processing context for this workflow command
    pub fn context(&self) -> &ProcessingContext {
        &self.context
    }
}

impl WorkflowCommand for StartWorkflowCommand {
    type Workflow = ExampleWorkflow;
    type Result = ExampleWorkflow;
    type Error = BearDogError;

    async fn execute(
        &self,
        mut workflow: Self::Workflow,
    ) -> Result<Self::Result, Self::Error> {
        info!(
            "Executing StartWorkflowCommand for workflow: {}",
            workflow.id().as_str()
        );

        if !matches!(workflow.status, ExampleWorkflowStatus::Created) {
            return Err(BearDogError::validation(format!(
                "Cannot start workflow in status: {:?}",
                workflow.status
            )));
        }

        workflow = workflow.set_status(ExampleWorkflowStatus::Started);
        Ok(workflow)
    }

    fn can_execute(&self, workflow: &Self::Workflow) -> bool {
        matches!(workflow.status, ExampleWorkflowStatus::Created)
    }

    fn description(&self) -> &'static str {
        "Start a created workflow"
    }
}

// ================================
// COMPREHENSIVE EXAMPLE USAGE
// ================================

/// Demonstrates the complete workflow system in action
pub async fn run_comprehensive_example() -> Result<(), BearDogError> {
    info!("🚀 Starting comprehensive workflow example");

    // Create components
    let repository = InMemoryWorkflowRepository::new();
    let processor = ExampleWorkflowProcessor::new("ExampleProcessor");
    let observer = LoggingWorkflowObserver::new("MainObserver");

    // Create the service
    let mut service = WorkflowService::new(repository, processor);
    service.add_observer(observer);

    // Create a workflow
    let workflow = ExampleWorkflow::new("example-001".to_string(), "Example Workflow".to_string())
        .with_data(serde_json::json!({
            "input": "test data",
            "priority": "high"
        }));

    info!("📝 Created workflow: {}", workflow.id().as_str());

    // Create and execute start command
    let _start_command = StartWorkflowCommand::new(ProcessingContext {
        user_id: "user123".to_string(),
        timeout_secs: 60,
        retry_count: 2,
    });

    // First create the workflow
    service.create_workflow(workflow.clone()).await?;

    // Then process it
    let result = service
        .process_workflow(workflow.id(), ProcessingContext::default())
        .await?;

    info!("✅ Workflow processing completed successfully");
    info!("📊 Final workflow status: {:?}", result.status);
    info!(
        "📄 Final workflow data: {}",
        serde_json::to_string_pretty(&result.data)
            .unwrap_or_else(|_| "Unable to serialize".to_string())
    );

    // Demonstrate repository operations
    info!(
        "🔍 Repository contains {} workflows",
        service.repository().count().await?
    );

    let all_workflows = service.repository().list_all().await?;
    for wf in all_workflows {
        info!(
            "📋 Stored workflow: {} - Status: {:?}",
            wf.id().as_str(),
            wf.status
        );
    }

    Ok(())
}

// ================================
// UNIT TESTS
// ================================

#[cfg(test)]
mod tests {
    use super::*;
    // Tests use tokio_test for async testing

    #[tokio::test]
    async fn test_repository_operations() {
        let repo = InMemoryWorkflowRepository::new();
        let workflow = ExampleWorkflow::new("test-1".to_string(), "Test Workflow".to_string());
        let workflow_id = workflow.id().clone();

        // Test save
        repo.save(workflow.clone()).await.unwrap();
        assert_eq!(repo.count().await.unwrap(), 1);

        // Test find
        let found = repo.find_by_id(&workflow_id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test Workflow");

        // Test exists
        assert!(repo.exists(&workflow_id).await.unwrap());

        // Test update
        let updated = workflow.set_status(ExampleWorkflowStatus::Completed);
        repo.update(updated).await.unwrap();

        // Test delete
        repo.delete(&workflow_id).await.unwrap();
        assert_eq!(repo.count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_processor() {
        let processor = ExampleWorkflowProcessor::new("TestProcessor");
        let workflow = ExampleWorkflow::new("test-2".to_string(), "Test Processing".to_string());
        let context = ProcessingContext::default();

        assert!(processor.can_process(&workflow));

        let result = processor.process(workflow, context).await.unwrap();
        assert!(matches!(result.status, ExampleWorkflowStatus::Completed));
        assert!(result.data["processed_by"].as_str().unwrap() == "TestProcessor");
    }

    #[tokio::test]
    async fn test_observer() {
        let observer = LoggingWorkflowObserver::new("TestObserver");
        let workflow = ExampleWorkflow::new("test-3".to_string(), "Test Observer".to_string());

        // These should not fail
        observer.on_created(&workflow).await.unwrap();
        observer.on_started(&workflow).await.unwrap();
        observer.on_completed(&workflow).await.unwrap();
        observer.on_failed(&workflow, "test error").await.unwrap();
        observer.on_cancelled(&workflow).await.unwrap();
    }

    #[tokio::test]
    async fn test_command() {
        let command = StartWorkflowCommand::new(ProcessingContext::default());
        let workflow = ExampleWorkflow::new("test-4".to_string(), "Test Command".to_string());

        assert!(command.can_execute(&workflow));
        assert_eq!(command.description(), "Start a created workflow");

        let result = command.execute(workflow).await.unwrap();
        assert!(matches!(result.status, ExampleWorkflowStatus::Started));
    }

    #[tokio::test]
    async fn test_full_integration() {
        let repository = InMemoryWorkflowRepository::new();
        let processor = ExampleWorkflowProcessor::new("IntegrationProcessor");
        let observer = LoggingWorkflowObserver::new("IntegrationObserver");

        let mut service = WorkflowService::new(repository, processor);
        service.add_observer(observer);

        let workflow = ExampleWorkflow::new(
            "integration-test".to_string(),
            "Integration Test Workflow".to_string(),
        );

        service.create_workflow(workflow.clone()).await.unwrap();
        let result = service
            .process_workflow(workflow.id(), ProcessingContext::default())
            .await
            .unwrap();

        assert!(matches!(result.status, ExampleWorkflowStatus::Completed));
        assert_eq!(service.repository().count().await.unwrap(), 1);
    }
}
