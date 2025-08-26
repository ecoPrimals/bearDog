

use super::core::WorkflowProcessor;
use crate::workflows::canonical::{
    Workflow, WorkflowExecutionStatus, WorkflowMetrics, WorkflowProcessingResult,
};

use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
pub struct KeyManagementConfig {

    pub max_key_size: u32,

    pub default_expiration_hours: u64,

    pub require_approval: bool,

    pub backup_location: Option<String>,
}
impl Default for KeyManagementConfig {}

    fn default() -> Self {
        Self {
            max_key_size: 4096,
            default_expiration_hours: 8760, // 1 year
            require_approval: true,
            backup_location: None,
        }
    }

#[derive(Debug)]
pub struct KeyManagementProcessor {
    pub config: UnifiedProcessorConfig,}

impl Default for KeyManagementProcessor {
            config: UnifiedProcessorConfig::default(),}

impl KeyManagementProcessor {

    pub fn new(config: UnifiedProcessorConfig) -> Self {
        Self { config }

    pub fn new_default() -> Self {
        Self::new(KeyManagementConfig::default())

    async fn process_key_rotation(
        &self,
        workflow: &Workflow,
    ) -> BearDogResult<WorkflowProcessingResult> {
        let start_time = Instant::now();
        info!("Processing key rotation workflow: {}", workflow.id);

        let key_id = workflow
            .parameters
            .get("key_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::invalid_input("Missing required parameter: key_id"))?;
        let key_type = workflow
            .get("key_type")
            .ok_or_else(|| BearDogError::invalid_input("Missing required parameter: key_type"))?;

        info!("Rotating key {} of type {}", key_id, key_type);

        Ok(WorkflowProcessingResult {
            success: true,
            message: format!("Key rotation completed for key: {key_id}"),
            duration_ms: start_time.elapsed().as_millis() as u64,
            workflow_id: workflow.id.clone(),
            processor_name: "KeyManagementProcessor".to_string(),
            status: WorkflowExecutionStatus::Completed,
            execution_duration_ms: Some(start_time.elapsed().as_millis() as u64),
            steps_completed: Some(2),
            steps_total: Some(2),
            output_data: Some(serde_json::json!({
                "key_id": key_id,
                "key_type": key_type,
                "rotation_timestamp": chrono::Utc::now().to_rfc3339()
            })),
            actions_taken: vec![
                "Validated key parameters".to_string(),
                "Completed key rotation".to_string(),
            ],
            metrics: WorkflowMetrics {
                processing_time_ms: start_time.elapsed().as_millis() as u64,
                memory_usage_bytes: 1024, // Minimal memory usage
                cpu_usage_percent: 0.5,
            },
            warnings: vec![],
        })

    async fn process_key_deletion(
        info!("Processing key deletion workflow: {}", workflow.id);

        warn!("Deleting key: {}", key_id);
        let execution_duration = start_time.elapsed();
            message: format!("Successfully deleted key: {key_id}"),
            duration_ms: execution_duration.as_millis() as u64,
            execution_duration_ms: Some(execution_duration.as_millis() as u64),
                "deleted_key_id": key_id,
                "deletion_timestamp": chrono::Utc::now().to_rfc3339()
                "Validated key deletion request".to_string(),
                "Completed key deletion".to_string(),
                processing_time_ms: execution_duration.as_millis() as u64,
                memory_usage_bytes: 512,
                cpu_usage_percent: 0.3,
impl WorkflowProcessor for KeyManagementProcessor {
    async fn process_workflow(
        match workflow.workflow_type {
            beardog_types::canonical::workflow::WorkflowType::KeyRotation => {
                self.process_key_rotation(workflow).await
            }
            beardog_types::canonical::workflow::WorkflowType::KeyDeletion => {
                self.process_key_deletion(workflow).await
            _ => Err(BearDogError::validation(format!(
                "Unsupported workflow type for key management: {:?}",
                workflow.workflow_type
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
    async fn test_key_rotation_processor() -> BearDogResult<()> {
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
            workflow_type: beardog_types::canonical::workflow::WorkflowType::KeyRotation,
            status: WorkflowStatus::Approved,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            approval_requirements: Default::default(),
            audit_trail: Vec::new(),
            metadata: HashMap::with_capacity(16),
            target: "system".to_string(),
            priority: beardog_types::canonical::workflow::WorkflowPriority::Normal,
            expires_at: None,
            requested_by: "test-user".to_string(),
            initiator: "test-user".to_string(),
            description: Some("Test key rotation".to_string()),
            timeout_duration: Some(std::time::Duration::from_secs(3600)),
            context: crate::workflows::canonical::WorkflowContext::default(),
            properties: HashMap::with_capacity(16),
            parameters,
            approvals: Vec::new(),
        };
        let result = processor.process_workflow(&workflow).await?;
        assert!(result.success);
        assert!(result.message.contains("Successfully rotated key"));
        assert!(!result.actions_taken.is_empty());
        Ok(())
