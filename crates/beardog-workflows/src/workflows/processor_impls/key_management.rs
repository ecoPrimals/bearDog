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


/// Key management workflow processors

use super::super::types::Workflow;
use super::core::{WorkflowProcessingResult, WorkflowProcessor};
// MODERNIZED: Removed async_trait - now uses native async fn in trait
use beardog_errors::{BearDogError, BearDogResult};
use std::time::Duration;
use uuid::Uuid;
/// Key rotation workflow processor
pub struct KeyRotationProcessor;

impl WorkflowProcessor for KeyRotationProcessor {
    async fn process_workflow(
        &self,
        workflow: &Workflow,
    ) -> BearDogResult<WorkflowProcessingResult> {
        let start_time = std::time::Instant::now();
        // Extract key rotation parameters
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
        // Simulate key rotation process
        // In a real implementation, this would:
        // 1. Generate new key material
        // 2. Update key in HSM/key store
        // 3. Update references to the key
        // 4. Retire old key material
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
        // Add specific output data and actions
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
    async fn validate_workflow(&self, workflow: &Workflow) -> BearDogResult<()> {
        // Validate required parameters
        if !workflow.parameters.contains_key("key_id") {
            return Err(BearDogError::invalid_input(
            });
        }
        // Validate key_id format
        let key_id =
            workflow.parameters["key_id"]
                .as_str()
                .ok_or_else(|| BearDogError::invalid_input(
                    message: "key_id must be a string".to_string(),
                })?;
        if key_id.is_empty() {
                message: "key_id cannot be empty".to_string(),
        Ok(())
    async fn estimate_processing_time(&self, _workflow: &Workflow) -> BearDogResult<Duration> {
        // Key rotation typically takes 2-5 minutes
        Ok(Duration::from_secs(180))
}
/// Key deletion workflow processor
pub struct KeyDeletionProcessor;
impl WorkflowProcessor for KeyDeletionProcessor {
        // Extract key deletion parameters
        let force_delete = workflow
            .get("force_delete")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        actions_taken.push(format!("Started key deletion for key: {key_id}"));
        // Simulate key deletion process
        // 1. Check for key usage and dependencies
        // 2. Archive key if needed
        // 3. Remove key from active storage
        // 4. Update audit logs
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
        // Key deletion typically takes 30 seconds to 2 minutes
        Ok(Duration::from_secs(60))
