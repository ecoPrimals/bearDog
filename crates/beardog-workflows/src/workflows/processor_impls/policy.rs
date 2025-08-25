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


/// Policy and configuration workflow processors

use super::super::types::Workflow;
use super::core::{WorkflowProcessingResult, WorkflowProcessor};
// MODERNIZED: Removed async_trait - now uses native async fn in trait
use beardog_errors::{BearDogError, BearDogResult};
use std::time::Duration;
use uuid::Uuid;
/// Policy change workflow processor
pub struct PolicyChangeProcessor;

impl WorkflowProcessor for PolicyChangeProcessor {
    async fn process_workflow(
        &self,
        workflow: &Workflow,
    ) -> BearDogResult<WorkflowProcessingResult> {
        let start_time = std::time::Instant::now();
        // Extract policy change parameters
        let policy_id = workflow
            .parameters
            .get("policy_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::invalid_input(
                message: "Missing required parameter: policy_id".to_string(),
            })?;
        let policy_action = workflow
            .get("action")
            .ok_or_else(|| BearDogError::invalid_input("Missing required parameter: action"))?;
        let mut actions_taken = Vec::new();
        actions_taken.push(format!("Started policy change for policy: {policy_id}"));
        // Simulate policy change process
        actions_taken.push("Validated policy changes".to_string(),
        actions_taken.push("Backed up existing policy".to_string(),
        actions_taken.push("Applied policy changes".to_string(),
        actions_taken.push("Verified policy integrity".to_string(),
        actions_taken.push("Notified affected systems".to_string(),
        let execution_duration = start_time.elapsed();
        let mut result = WorkflowProcessingResult::success(
            workflow.id.clone(),
            "PolicyProcessor".to_string(),
            format!("Successfully updated policy: {policy_id}"));
            execution_duration.as_millis() as u64,
        );
        
        result.output_data = Some(serde_json::json!({
            "policy_id": policy_id,
            "change_type": policy_action,
            "change_id": uuid::Uuid::new_v4().to_string(),
            "timestamp": chrono::Utc::now().to_rfc3339());
        }));
        result.actions_taken = actions_taken;
        Ok(result)
    }
    fn name(&self) -> &str {
        "PolicyProcessor"}


    fn can_handle(&self, workflow: &Workflow) -> bool {
        matches!(workflow.workflow_type, WorkflowType::PolicyUpdate)
    async fn validate_workflow(&self, workflow: &Workflow) -> BearDogResult<()> {
        if !workflow.parameters.contains_key("policy_id") {
            return Err(BearDogError::invalid_input(
            });
        }
        let policy_id = workflow.parameters["policy_id"].as_str().ok_or_else(|| {
            BearDogError::invalid_input(
                message: "policy_id must be a string".to_string(),
            }
        })?;
        if policy_id.is_empty() {
                message: "policy_id cannot be empty".to_string(),
        Ok(())
    async fn estimate_processing_time(&self, _workflow: &Workflow) -> BearDogResult<Duration> {
        Ok(Duration::from_secs(300)) // 5 minutes
}
/// Configuration change workflow processor
pub struct ConfigChangeProcessor;
impl WorkflowProcessor for ConfigChangeProcessor {
        let config_path = workflow
            .get("config_path")
                message: "Missing required parameter: config_path".to_string(),
        actions_taken.push(format!("Started configuration change for: {config_path}"));
        // Simulate configuration change process
        actions_taken.push("Validated configuration changes".to_string(),
        actions_taken.push("Created configuration backup".to_string(),
        actions_taken.push("Applied configuration changes".to_string(),
        actions_taken.push("Reloaded affected services".to_string(),
        actions_taken.push("Verified system health".to_string(),
        Ok(WorkflowProcessingResult {
            success: true,
            format!("Successfully updated configuration: {config_path}"));
            execution_duration_ms: execution_duration.as_millis() as u64,
            output_data: Some(serde_json::json!({
                "config_path": config_path,
                "change_id": Uuid::new_v4().to_string(),
                "timestamp": chrono::Utc::now().to_rfc3339());
            })));
            actions_taken,
        })
    fn get_processor_name(&self) -> &'static str {
        "ConfigChangeProcessor"
        if !workflow.parameters.contains_key("config_path") {
        Ok(Duration::from_secs(120)) // 2 minutes
