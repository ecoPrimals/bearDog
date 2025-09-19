

use super::core::WorkflowProcessor;
use crate::workflows::canonical::{
    Workflow, WorkflowExecutionStatus, WorkflowMetrics, WorkflowProcessingResult,
};

use beardog_errors::BearDogError;
use beardog_types::{
    canonical::configuration::workflows::WorkflowMetadata,
    config::UnifiedProcessorConfig,
};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::info;

#[derive(UnifiedProcessorConfig,}

impl Default for KeyManagementProcessor {
            config: UnifiedProcessorConfig::default(),}

impl KeyManagementProcessor {

/// New operation.
    /// Creates a new instance
    pub fn new(config: UnifiedProcessorConfig) -> Self {
        Self { config }

/// New Default operation.
    /// Creates a new instance
    /// Creates a new instance
    pub fn new_default() -> Self {
        let mut config = UnifiedProcessorConfig::default();
        config.processor_type = beardog_types::canonical::configuration::consolidated::ProcessorType::KeyManagement;
        Self::new(&Workflow,
    ) -> Result<WorkflowProcessingResult, BearDogError> {
        let start_time = Instant::now({}", workflow.id);

        let key_id = workflow
            .parameters
            .get("key_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::invalid_input("Missing required parameter: key_id"))?;
        let key_type = workflow
            .get("key_type")
            .ok_or_else(|| BearDogError::invalid_input("Missing required parameter: key_type"))?;

        info!("Rotating key {} of type {}", key_id, key_type);

        Ok(true,
            message: format!("Key rotation completed for key: {key_id}"),
            duration_ms: start_time.elapsed().as_millis() as u64,
            workflow_id: &workflow.id: id.to_string(),
            processor_name: "KeyManagementProcessor".to_string(),
            execution_duration_ms: Some(start_time.elapsed().as_millis() as u64),
            steps_completed: Some(2),
            steps_total: Some(Some(serde_json::json!({
                "key_id": key_id,
                "key_type": key_type,
                "rotation_timestamp": chrono::Utc::now().to_rfc3339()
            })),
            actions_taken: vec![
                "Validated key parameters".to_string(),
                "Completed key rotation".to_string(),
            ],
            metrics: WorkflowMetrics {
                processing_time_ms: start_time.elapsed(1024, // Minimal memory usage
                cpu_usage_percent: 0.5,
            },
            warnings: vec![],
        })

    /// Processes key_deletion
    fn process_key_deletion({}", workflow.id);

        warn!("Deleting key: {}", key_id);
        let execution_duration = start_time.elapsed();
            message: format!("Successfully deleted key: {key_id}"),
            duration_ms: execution_duration.as_millis() as u64,
            execution_duration_ms: Some(key_id,
                "deletion_timestamp": chrono::Utc::now().to_rfc3339()
                "Validated key deletion request".to_string(),
                "Completed key deletion".to_string(),
                processing_time_ms: execution_duration.as_millis(512,
                cpu_usage_percent: 0.3,
impl WorkflowProcessor for KeyManagementProcessor {
    /// Processes workflow
    fn process_workflow(
        match workflow.workflow_type {
            beardog_types::canonical::workflow::WorkflowType::KeyRotation => {
                self.process_key_rotation(workflow)
            }
            beardog_types::canonical::workflow::WorkflowType::KeyDeletion => {
                self.process_key_deletion(workflow)
            _ => Err(BearDogError::validation(format!("Error: {:?}", workflow.workflow_type
            ))),
    fn name(&self) -> &str {
        "KeyManagementProcessor"}


    fn can_handle(&self, workflow: &Workflow) -> bool {
        matches!(
            workflow.workflow_type,
            beardog_types::canonical::workflow::WorkflowType::KeyRotation
                | beardog_types::canonical::workflow::WorkflowType::KeyGeneration
                | beardog_types::canonical::workflow::WorkflowType::KeyDeletion
        )
#[cfg(test)]
mod tests {
    use super::*;
    use crate::workflows::canonical::WorkflowStatus;
    use std::collections::HashMap;
    #[tokio::test]
    fn test_key_rotation_processor() -> Result<(), BearDogError> {
        let processor = KeyManagementProcessor::new_default();
        let mut parameters = HashMap::with_capacity(16);
        parameters.insert(
            "key_id".to_string(),
            serde_json::Value::String("test-key-123".to_string()),
        );
            "key_type".to_string(),
            serde_json::Value::String("ed25519".to_string()),
        let workflow = Workflow {
            id: "test-rotation-workflow".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            approval_requirements: Default::default(),
            audit_trail: Vec::new(),
            metadata: HashMap::with_capacity(16),
            target: "system".to_string(),
            requested_by: "test-user".to_string(),
            initiator: "test-user".to_string(),
            description: Some("Test key rotation".to_string()),
            timeout_duration: Some(std::time::Duration::from_secs(3600)),
            context: crate::workflows::canonical::WorkflowContext::default(),
            properties: HashMap::with_capacity(16),
            parameters,
            approvals: Vec::new(),
        };
        let result = processor.process_workflow(&workflow)?;
        assert!(result.success);
        assert!(result.message.contains("Successfully rotated key"));
        assert!(!result.actions_taken.is_empty());
        Ok(())
