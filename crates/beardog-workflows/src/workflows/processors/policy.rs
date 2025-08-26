

use super::core::WorkflowProcessor;
use crate::workflows::canonical::{
    Workflow, WorkflowExecutionStatus, WorkflowMetrics, WorkflowProcessingResult,
};

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::workflow::{WorkflowType, WorkflowStatus};
use beardog_types::aliases::ProviderMetrics;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
#[deprecated(since = "3.1.0", note = "Use UnifiedProcessorConfig instead")]
pub struct PolicyProcessorConfig {

    pub auto_validation: bool,

    pub approval_threshold: u32,

    pub enable_rollback: bool,
}

impl Default for PolicyProcessorConfig {
    fn default() -> Self {
        Self {
            auto_validation: true,
            approval_threshold: 2,
            enable_rollback: true,
        }
    }
}

#[derive(Debug)]
pub struct PolicyProcessor {
    pub config: UnifiedProcessorConfig,
}

impl PolicyProcessor {

    pub fn new(config: UnifiedProcessorConfig) -> Self {
        Self { config }
    }

    pub fn new_default() -> Self {
        Self::new(PolicyProcessorConfig::default())
    }

    async fn process_policy_change(
        &self,
        workflow: &Workflow,
    ) -> BearDogResult<WorkflowProcessingResult> {
        let start_time = Instant::now();
        info!("Processing policy change for workflow: {}", workflow.id);

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
            workflow_id: workflow.id.clone(),
            status: WorkflowExecutionStatus::Completed,
            execution_time_ms: execution_time.as_millis() as u64,
            result: serde_json::json!({
                "policy_change": "applied",
                "validation": self.config.auto_validation,
                "approvals": workflow.approvals.len()
            }),
            metrics: WorkflowMetrics {
                processing_time_ms: execution_time.as_millis() as u64,
                memory_usage_mb: 0,
                cpu_usage_percent: 0.0,
            },
        })
    }

    async fn process_configuration_change(
        &self,
        workflow: &Workflow,
    ) -> BearDogResult<WorkflowProcessingResult> {
        let start_time = Instant::now();
        info!("Processing configuration change for workflow: {}", workflow.id);

        let execution_time = start_time.elapsed();
        
        Ok(WorkflowProcessingResult {
            workflow_id: workflow.id.clone(),
            status: WorkflowExecutionStatus::Completed,
            execution_time_ms: execution_time.as_millis() as u64,
            result: serde_json::json!({
                "configuration_change": "applied"
            }),
            metrics: WorkflowMetrics {
                processing_time_ms: execution_time.as_millis() as u64,
                memory_usage_mb: 0,
                cpu_usage_percent: 0.0,
            },
        })
    }

    fn validate_policy_change(&self, workflow: &Workflow) -> BearDogResult<()> {

        if workflow.payload.is_null() {
            return Err(BearDogError::validation(
                "Policy change requires payload".to_string(),
            ));
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
    fn processor_name(&self) -> &str {
        "PolicyProcessor"
    }

    fn can_process(&self, workflow_type: &WorkflowStatus) -> bool {
        matches!(
            workflow_type,
            WorkflowStatus::PolicyChange | WorkflowStatus::ConfigurationChange
        )
    }

    async fn process_workflow(
        &self,
        workflow: &WorkflowStatus,
    ) -> BearDogResult<ProviderMetrics> {
        let start_time = Instant::now();
        info!("Processing policy workflow with status: {:?}", workflow);

        let result = match workflow {
            WorkflowStatus::PolicyChange => {

                info!("Processing policy change workflow");
                serde_json::json!({
                    "type": "policy_change",
                    "status": "completed"
                })
            }
            WorkflowStatus::ConfigurationChange => {

                info!("Processing configuration change workflow");
                serde_json::json!({
                    "type": "configuration_change", 
                    "status": "completed"
                })
            }
            _ => {
                return Err(BearDogError::validation(format!(
                    "Unsupported workflow type for policy processor: {:?}",
                    workflow
                )));
            }
        };

        let execution_time = start_time.elapsed();

        let mut metrics = ProviderMetrics::new();
        metrics.insert("processing_time_ms".to_string(), execution_time.as_millis() as f64);
        metrics.insert("success".to_string(), 1.0);
        metrics.insert("policy_validation".to_string(), if self.config.auto_validation { 1.0 } else { 0.0 });
        
        Ok(metrics)
    }
}
