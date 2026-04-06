// SPDX-License-Identifier: AGPL-3.0-or-later

//! Example workflow processor implementation

use crate::workflows::canonical_examples::types::{
    ExampleWorkflow, ExampleWorkflowStatus, ProcessingContext,
};
use crate::workflows::canonical_traits::{Workflow, WorkflowId, WorkflowProcessor};
use beardog_errors::BearDogError;
use tokio::time::{Duration, sleep};
use tracing::info;

/// Sample [`WorkflowProcessor`] that validates ids, simulates work, and completes with annotated JSON data.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ExampleWorkflowProcessor {
    /// Name of the item
    pub name: &'static str,
}

impl ExampleWorkflowProcessor {
    /// New operation.
    /// Creates a new instance
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
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
            sleep(Duration::from_millis(100)).await; // Simulate work

            let processed_data = {
                use serde_json::{Map, Value};
                let mut data = Map::new();
                data.insert(
                    "processed_by".to_string(),
                    Value::String(processor_name.to_string()),
                );
                data.insert(
                    "processed_at".to_string(),
                    Value::String(chrono::Utc::now().to_rfc3339()),
                );
                data.insert(
                    "original_data".to_string(),
                    workflow.data.clone().unwrap_or(Value::Null),
                );
                Value::Object(data)
            };

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
