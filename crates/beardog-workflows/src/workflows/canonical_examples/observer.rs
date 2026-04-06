// SPDX-License-Identifier: AGPL-3.0-or-later

//! Logging workflow observer implementation

use crate::workflows::canonical_examples::types::ExampleWorkflow;
use crate::workflows::canonical_traits::{Workflow, WorkflowId, WorkflowObserver};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};

/// [`WorkflowObserver`] that emits structured tracing events for each lifecycle transition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
