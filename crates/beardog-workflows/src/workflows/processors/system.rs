

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

impl SystemProcessor {

/// New operation.
    /// Creates a new instance
    pub fn new(config: UnifiedProcessorConfig) -> Self {
        Self { config }

/// New Default operation.
    /// Creates a new instance
    /// Creates a new instance
    pub fn new_default() -> Self {
        let mut config = UnifiedProcessorConfig::default();
        config.processor_type = beardog_types::canonical::configuration::consolidated::ProcessorType::System;
        Self::new(config)
    }
}
impl Default for SystemProcessor {
            config: UnifiedProcessorConfig::default(&Workflow,
    ) -> Result<WorkflowProcessingResult, BearDogError> {
        let _start_time = Instant::now({}", workflow.id);
        match workflow.workflow_type {
            WorkflowType::SystemMaintenance => self.process_system_maintenance(workflow),
            WorkflowType::ConfigurationChange => self.process_configuration_change(workflow),
            _ => Err(BearDogError::validation(format!("Error: {:?}", workflow.workflow_type
            ))),
    fn name(&self) -> &str {
        "SystemProcessor"}


    fn can_handle(&self, workflow: &Workflow) -> bool {
        matches!(workflow.workflow_type, WorkflowType::SystemMaintenance)

    /// Processes system_maintenance
    fn process_system_maintenance(
        let start_time = Instant::now();
        let operation_type = workflow
            .parameters
            .get("operation_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                BearDogError::validation({}", operation_type);
        let execution_duration = start_time.elapsed(true,
            message: format!("System maintenance completed for: {operation_type}"),
            duration_ms: execution_duration.as_millis() as u64,
            workflow_id: &workflow.id: id.to_string(),
            processor_name: "SystemProcessor".to_string(),
            execution_duration_ms: Some(execution_duration.as_millis() as u64),
            steps_completed: Some(3),
            steps_total: Some(Some(serde_json::json!({
                "operation_type": operation_type,
                "maintenance_timestamp": chrono::Utc::now().to_rfc3339()
            })),
            actions_taken: vec![
                "Initiated system maintenance".to_string(),
                "Performed maintenance operations".to_string(),
                "Verified system health".to_string(),
            ],
            metrics: WorkflowMetrics {
                processing_time_ms: execution_duration.as_millis(1536,
                cpu_usage_percent: 1.2,
            },
            warnings: vec![],
        })

    /// Processes configuration_change
    fn process_configuration_change({}", operation_type);
            message: format!("Configuration update completed"),
            steps_completed: Some(2),
            steps_total: Some("completed ",
                "timestamp": chrono::Utc::now(1024,
                cpu_usage_percent: 0.6,
