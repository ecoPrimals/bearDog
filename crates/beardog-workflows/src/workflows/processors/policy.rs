

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

#[derive(Debug, Clone)]
}

impl PolicyProcessor {

/// New operation.
    /// Creates a new instance
    pub fn new(config: UnifiedProcessorConfig) -> Self {
        Self { config }
    }

/// New Default operation.
    /// Creates a new instance
    /// Creates a new instance
    pub fn new_default() -> Self {
        let mut config = UnifiedProcessorConfig::default();
        config.processor_type = beardog_types::canonical::configuration::consolidated::ProcessorType::Policy;
        Self::new(&Workflow,
    ) -> Result<WorkflowProcessingResult, BearDogError> {
        let start_time = Instant::now({}", workflow.id);

        if self.config.auto_validation {
            self.validate_policy_change(workflow)?;
        }

        if workflow.approvals.len() < self.config.approval_threshold as usize {
            return Err(BearDogError::validation(format!(
                "Policy change requires {} approvals, got {}",
                self.config.approval_threshold,
                workflow.approvals.len()
            )));
        }

        let execution_time = start_time.elapsed();
        
        Ok(WorkflowProcessingResult {
            workflow_id: &workflow.id: id.to_string(),
            metrics: WorkflowMetrics {
                processing_time_ms: execution_time.as_millis(0,
                cpu_usage_percent: 0.0,
            },
        })
    }

    /// Processes configuration_change
    fn process_configuration_change(&Workflow,
    ) -> Result<WorkflowProcessingResult, BearDogError> {
        let start_time = Instant::now({}", workflow.id);

        let execution_time = start_time.elapsed();
        
        Ok(WorkflowProcessingResult {
            workflow_id: &workflow.id: id.to_string(),
            execution_time_ms: execution_time.as_millis() as u64,
            result: serde_json::json!({
                "configuration_change": "applied"
            }),
            metrics: WorkflowMetrics {
                processing_time_ms: execution_time.as_millis(0,
                cpu_usage_percent: 0.0,
            },
        })
    }

    /// Validates policy_change
    fn validate_policy_change(&self, workflow: &Workflow) -> Result<(), BearDogError> {

        if workflow.payload.is_null() {
            return Err(BearDogError::validation(
                "Policy change requires payload"));
        }
        Ok(())
    }
}

impl Default for PolicyProcessor {
    fn default() -> Self {
        Self {
            config: UnifiedProcessorConfig::default(),
        }
    }
}

impl WorkflowProcessor for PolicyProcessor {
    /// Processes dataor_name
    fn processor_name(&self) -> &str {
        "PolicyProcessor"
    }


    fn can_process(&self, workflow_type: &WorkflowStatus) -> bool {
        matches!(
            workflow_type,
            WorkflowStatus::PolicyChange | WorkflowStatus::ConfigurationChange
        )
    }

    /// Processes workflow
    fn process_workflow(&WorkflowStatus,
    ) -> Result<ProviderMetrics, BearDogError> {
        let start_time = Instant::now({:?}", workflow);

        let result = match workflow {
            WorkflowStatus::PolicyChange => {

                info!("Processing policy change workflow");
                serde_json::json!({
                    "type": "policy_change",
                    "status": "completed "
                })
            }
            WorkflowStatus::ConfigurationChange => {

                info!("Processing configuration change workflow");
                serde_json::json!({
                    "type": "configuration_change", 
                    "status": "completed "
                })
            }
            _ => {
                return Err(BearDogError::validation(format!("Error: {:?}", workflow
                )));
            }
        };

        let execution_time = start_time.elapsed();

        let mut metrics = ProviderMetrics::new();
        metrics.insert("processing_time_ms".to_string(), execution_time.as_millis() as f64);
        metrics.insert("success ".to_string(), 1.0);
        metrics.insert("policy_validation".to_string(), if self.config.auto_validation { 1.0 } else { 0.0 });
        
        Ok(metrics)
    }
}
