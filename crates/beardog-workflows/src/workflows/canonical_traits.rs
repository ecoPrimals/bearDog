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

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: workflows
    // TEST_PRIORITY: normal
    #[test]
    fn test_traits_exist() {}
}
