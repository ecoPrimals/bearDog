// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive workflow example

use crate::workflows::canonical_examples::commands::StartWorkflowCommand;
use crate::workflows::canonical_examples::observer::LoggingWorkflowObserver;
use crate::workflows::canonical_examples::processor::ExampleWorkflowProcessor;
use crate::workflows::canonical_examples::repository::InMemoryWorkflowRepository;
use crate::workflows::canonical_examples::types::{ExampleWorkflow, ProcessingContext};
use crate::workflows::canonical_traits::{
    Workflow, WorkflowId, WorkflowProcessor, WorkflowService,
};
use beardog_errors::BearDogError;
use tracing::{info, warn};

/// Run Comprehensive Example operation.
///
/// # Errors
/// Returns an error if the operation fails.
/// Runs `comprehensive_example`
pub async fn run_comprehensive_example() -> Result<(), BearDogError> {
    info!("🚀 Starting comprehensive workflow example");

    let repository = InMemoryWorkflowRepository::new();
    let processor = ExampleWorkflowProcessor::new("ExampleProcessor");
    let observer = LoggingWorkflowObserver::new("MainObserver");

    let mut service = WorkflowService::new(repository, processor);
    service.add_observer(observer);

    let workflow_data = {
        use serde_json::{Map, Value};
        let mut data = Map::new();
        data.insert("input".to_string(), Value::String("test data".to_string()));
        data.insert("priority".to_string(), Value::String("high".to_string()));
        Value::Object(data)
    };
    let workflow = ExampleWorkflow::new("example-001", "Example Workflow").with_data(workflow_data);

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
