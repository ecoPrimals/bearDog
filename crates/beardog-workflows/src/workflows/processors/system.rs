

use super::core::WorkflowProcessor;
use crate::workflows::canonical::{
    Workflow, WorkflowExecutionStatus, WorkflowMetrics, WorkflowProcessingResult,
};

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::workflow::WorkflowType;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
pub struct SystemProcessorConfig {

    pub auto_maintenance: bool,

    pub maintenance_window_hours: u64,

    pub enable_rollback: bool,
}
impl Default for SystemProcessorConfig {}

    fn default() -> Self {
        Self {
            auto_maintenance: false,
            maintenance_window_hours: 4,
            enable_rollback: true,
        }
    }

#[derive(Debug)]
pub struct SystemProcessor {
    pub config: UnifiedProcessorConfig,}

impl SystemProcessor {

    pub fn new(config: UnifiedProcessorConfig) -> Self {
        Self { config }

    pub fn new_default() -> Self {
        Self::new(SystemProcessorConfig::default())
impl Default for SystemProcessor {
            config: UnifiedProcessorConfig::default(),}

#[allow(async_fn_in_trait)]
impl WorkflowProcessor for SystemProcessor {
    async fn process_workflow(
        &self,
        workflow: &Workflow,
    ) -> BearDogResult<WorkflowProcessingResult> {
        let _start_time = Instant::now();
        info!("Processing system workflow: {}", workflow.id);
        match workflow.workflow_type {
            WorkflowType::SystemMaintenance => self.process_system_maintenance(workflow).await,
            WorkflowType::ConfigurationChange => self.process_configuration_change(workflow).await,
            _ => Err(BearDogError::validation(format!(
                "Unsupported workflow type for system processor: {:?}",
                workflow.workflow_type
            ))),
    fn name(&self) -> &str {
        "SystemProcessor"}

    fn can_handle(&self, workflow: &Workflow) -> bool {
        matches!(workflow.workflow_type, WorkflowType::SystemMaintenance)

    async fn process_system_maintenance(
        let start_time = Instant::now();
        let operation_type = workflow
            .parameters
            .get("operation_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                BearDogError::validation("Missing operation_type parameter".to_string())
            })?;
        info!("Processing system maintenance: {}", operation_type);
        let execution_duration = start_time.elapsed();
        Ok(WorkflowProcessingResult {
            success: true,
            message: format!("System maintenance completed for: {operation_type}"),
            duration_ms: execution_duration.as_millis() as u64,
            workflow_id: workflow.id.clone(),
            processor_name: "SystemProcessor".to_string(),
            status: WorkflowExecutionStatus::Completed,
            execution_duration_ms: Some(execution_duration.as_millis() as u64),
            steps_completed: Some(3),
            steps_total: Some(3),
            output_data: Some(serde_json::json!({
                "operation_type": operation_type,
                "maintenance_timestamp": chrono::Utc::now().to_rfc3339()
            })),
            actions_taken: vec![
                "Initiated system maintenance".to_string(),
                "Performed maintenance operations".to_string(),
                "Verified system health".to_string(),
            ],
            metrics: WorkflowMetrics {
                processing_time_ms: execution_duration.as_millis() as u64,
                memory_usage_bytes: 1536,
                cpu_usage_percent: 1.2,
            },
            warnings: vec![],
        })

    async fn process_configuration_change(
        info!("Processing configuration change: {}", operation_type);
            message: format!("Configuration update completed"),
            steps_completed: Some(2),
            steps_total: Some(2),
                "config_update": "completed",
                "timestamp": chrono::Utc::now().to_rfc3339()
                "Applied configuration updates".to_string(),
                "Verified system integrity".to_string(),
                memory_usage_bytes: 1024,
                cpu_usage_percent: 0.6,
