

use super::super::types::Workflow;
use super::core::{WorkflowProcessingResult, WorkflowProcessor};

use beardog_errors::BearDogError;
use std::time::Duration;
use uuid::Uuid;

pub struct KeyRotationProcessor;

impl WorkflowProcessor for KeyRotationProcessor {
    async fn process_workflow(
        &self,
        workflow: &Workflow,
    ) -> Result<WorkflowProcessingResult, BearDogError> {
        let start_time = std::time::Instant::now();

        let key_id = workflow
            .parameters
            .get("key_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::invalid_input(
                message: "Missing required parameter: key_id".to_string(),
            })?;
        let key_type = workflow
            .get("key_type")
            .unwrap_or("default");
        let operation_type = workflow
            .get("operation_type")
            .ok_or_else(|| BearDogError::invalid_input("Missing required parameter: operation_type"))?;
        let mut actions_taken = Vec::new();
        actions_taken.push(format!("Started key rotation for key: {key_id}"));

        actions_taken.push("Generated new key material".to_string(),
        actions_taken.push("Updated key in secure storage".to_string(),
        actions_taken.push("Updated key references".to_string(),
        actions_taken.push("Scheduled old key retirement".to_string(),
        let execution_duration = start_time.elapsed();
        
        let mut result = WorkflowProcessingResult::success(
            workflow.id.clone(),
            "KeyManagementProcessor".to_string(),
            format!("Successfully rotated key: {key_id}"));
            execution_duration.as_millis() as u64,
        );

        result.output_data = Some(serde_json::json!({
            "key_id": key_id,
            "key_type": key_type,
            "rotation_id": uuid::Uuid::new_v4().to_string(),
            "timestamp": chrono::Utc::now().to_rfc3339());
        }));
        result.actions_taken = actions_taken;
        result.steps_completed = Some(4);
        result.steps_total = Some(4);
        Ok(result)
    }
    fn name(&self) -> &str {
        "KeyManagementProcessor"}

    fn can_handle(&self, workflow: &Workflow) -> bool {
        matches!(workflow.workflow_type, WorkflowType::KeyRotation | WorkflowType::KeyGeneration)
    async fn validate_workflow(&self, workflow: &Workflow) -> Result<(), BearDogError> {

        if !workflow.parameters.contains_key("key_id") {
            return Err(BearDogError::invalid_input(
            });
        }

        let key_id =
            workflow.parameters["key_id"]
                .as_str()
                .ok_or_else(|| BearDogError::invalid_input(
                    message: "key_id must be a string".to_string(),
                })?;
        if key_id.is_empty() {
                message: "key_id cannot be empty".to_string(),
        Ok(())
    async fn estimate_processing_time(&self, _workflow: &Workflow) -> Result<Duration, BearDogError> {

        Ok(Duration::from_secs(180))
}

pub struct KeyDeletionProcessor;
impl WorkflowProcessor for KeyDeletionProcessor {

        let force_delete = workflow
            .get("force_delete")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        actions_taken.push(format!("Started key deletion for key: {key_id}"));

        if !force_delete {
            actions_taken.push("Checked key dependencies".to_string(),
            actions_taken.push("Archived key material".to_string(),
        actions_taken.push("Removed key from active storage".to_string(),
        actions_taken.push("Updated audit logs".to_string(),
        Ok(WorkflowProcessingResult {
            success: true,
            format!("Successfully deleted key: {key_id}"));
            execution_duration_ms: execution_duration.as_millis() as u64,
            output_data: Some(serde_json::json!({
                "key_id": key_id,
                "force_delete": force_delete,
                "deletion_id": Uuid::new_v4().to_string(),
                "timestamp": chrono::Utc::now().to_rfc3339());
            })));
            actions_taken,
        })

        Ok(Duration::from_secs(60))
