// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Policy workflow processors
///
/// Handles policy change and configuration workflows.
use super::core::WorkflowProcessor;
use crate::workflows::canonical::{
    Workflow, WorkflowExecutionStatus, WorkflowMetrics, WorkflowProcessingResult,
};
// MODERNIZED: Removed async_trait - now uses native async fn in trait
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::workflow::{WorkflowType, WorkflowStatus};
use beardog_types::aliases::ProviderMetrics;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use tracing::info;

/// Configuration for policy processors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyProcessorConfig {
    /// Enable automatic policy validation
    pub auto_validation: bool,
    /// Policy change approval threshold
    pub approval_threshold: u32,
    /// Enable policy rollback capability
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

/// Policy workflow processor
#[derive(Debug)]
pub struct PolicyProcessor {
    pub config: PolicyProcessorConfig,
}

impl PolicyProcessor {
    /// Create a new policy processor
    pub fn new(config: PolicyProcessorConfig) -> Self {
        Self { config }
    }

    /// Create with default configuration
    pub fn new_default() -> Self {
        Self::new(PolicyProcessorConfig::default())
    }

    /// Process policy change workflow
    async fn process_policy_change(
        &self,
        workflow: &Workflow,
    ) -> BearDogResult<WorkflowProcessingResult> {
        let start_time = Instant::now();
        info!("Processing policy change for workflow: {}", workflow.id);

        // Validate policy change request
        if self.config.auto_validation {
            self.validate_policy_change(workflow)?;
        }

        // Check approval threshold
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

    /// Process configuration change workflow
    async fn process_configuration_change(
        &self,
        workflow: &Workflow,
    ) -> BearDogResult<WorkflowProcessingResult> {
        let start_time = Instant::now();
        info!("Processing configuration change for workflow: {}", workflow.id);

        // Apply configuration changes
        // In a real implementation, this would interact with configuration management
        
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

    /// Validate policy change
    fn validate_policy_change(&self, workflow: &Workflow) -> BearDogResult<()> {
        // Basic validation - in real implementation would have comprehensive policy validation
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
            config: PolicyProcessorConfig::default(),
        }
    }
}

// MODERNIZED: Native async fn implementation - no async_trait overhead
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

        // Convert WorkflowStatus to processing logic
        let result = match workflow {
            WorkflowStatus::PolicyChange => {
                // Process policy change
                info!("Processing policy change workflow");
                serde_json::json!({
                    "type": "policy_change",
                    "status": "completed"
                })
            }
            WorkflowStatus::ConfigurationChange => {
                // Process configuration change
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
        
        // Return metrics as expected by the canonical trait
        let mut metrics = ProviderMetrics::new();
        metrics.insert("processing_time_ms".to_string(), execution_time.as_millis() as f64);
        metrics.insert("success".to_string(), 1.0);
        metrics.insert("policy_validation".to_string(), if self.config.auto_validation { 1.0 } else { 0.0 });
        
        Ok(metrics)
    }
}
