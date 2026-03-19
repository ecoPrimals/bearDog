// SPDX-License-Identifier: AGPL-3.0-only

//! In-memory workflow repository implementation

use crate::workflows::canonical_examples::types::{ExampleWorkflow, ExampleWorkflowId};
use crate::workflows::canonical_traits::{Workflow, WorkflowId, WorkflowRepository};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::info;

/// Canonical in-memory workflow repository implementation
#[derive(Debug, Clone)]
pub struct InMemoryWorkflowRepository {
    workflows: Arc<Mutex<HashMap<String, ExampleWorkflow>>>,
}

impl InMemoryWorkflowRepository {
    /// New operation.
    /// Creates a new instance
    #[must_use]
    pub fn new() -> Self {
        Self {
            workflows: Arc::new(Mutex::new(HashMap::with_capacity(16))),
        }
    }

    /// Len operation.
    #[must_use]
    pub fn len(&self) -> usize {
        self.workflows.lock().map(|w| w.len()).unwrap_or(0)
    }

    /// Is Empty operation.
    /// Checks if empty
    #[must_use]
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

    async fn find_by_id(
        &self,
        id: &ExampleWorkflowId,
    ) -> Result<Option<Self::Workflow>, Self::Error> {
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
        let mut workflows = self
            .workflows
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
