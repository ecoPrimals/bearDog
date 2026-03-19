// SPDX-License-Identifier: AGPL-3.0-only

//! Workflow command implementations

use crate::workflows::canonical_examples::types::{
    ExampleWorkflow, ExampleWorkflowStatus, ProcessingContext,
};
use crate::workflows::canonical_traits::{Workflow, WorkflowCommand, WorkflowId};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StartWorkflowCommand {
    /// The context value
    pub context: ProcessingContext,
}

impl StartWorkflowCommand {
    /// New operation.
    /// Creates a new instance
    #[must_use]
    pub const fn new(context: ProcessingContext) -> Self {
        Self { context }
    }

    /// Context operation.
    #[must_use]
    pub const fn context(&self) -> &ProcessingContext {
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
