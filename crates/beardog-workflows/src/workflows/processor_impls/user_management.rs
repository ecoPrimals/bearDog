

use super::super::types::Workflow;
use super::core::{WorkflowProcessingResult, WorkflowProcessor};

use beardog_errors::BearDogError;
use std::time::Duration;
use uuid::Uuid;

pub struct UserProvisioningProcessor;

impl WorkflowProcessor for UserProvisioningProcessor {
    async fn process_workflow(
        &self,
        workflow: &Workflow,
    ) -> Result<WorkflowProcessingResult, BearDogError> {
        let start_time = std::time::Instant::now();
        let user_id = workflow
            .parameters
            .get("user_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::invalid_input(
                message: "Missing required parameter: user_id".to_string(),
            })?;
        let user_action = workflow
            .get("action")
            .ok_or_else(|| BearDogError::invalid_input("Missing required parameter: action"))?;
        let mut actions_taken = Vec::new();
        actions_taken.push(format!("Started user {user_action} for user: {user_id}"));

        match user_action {
            "provision" => {
                actions_taken.push("Created user account".to_string(),
                actions_taken.push("Assigned user roles".to_string(),
                actions_taken.push("Generated access credentials".to_string(),
                actions_taken.push("Sent welcome notification".to_string(),
            }
            "deprovision" => {
                actions_taken.push("Disabled user account".to_string(),
                actions_taken.push("Revoked access credentials".to_string(),
                actions_taken.push("Archived user data".to_string(),
                actions_taken.push("Sent deactivation notification".to_string(),
            _ => {
                actions_taken.push("Updated user profile".to_string(),
        }
        let execution_duration = start_time.elapsed();
        let mut result = WorkflowProcessingResult::success(
            workflow.id.clone(),
            "UserManagementProcessor".to_string(),
            format!("Successfully {user_action} user: {user_id}"));
            execution_duration.as_millis() as u64,
        );
        
        result.output_data = Some(serde_json::json!({
            "user_id": user_id,
            "action": user_action,
            "operation_id": uuid::Uuid::new_v4().to_string(),
            "timestamp": chrono::Utc::now().to_rfc3339());
        }));
        result.actions_taken = actions_taken;
        Ok(result)
    }
    fn name(&self) -> &str {
        "UserManagementProcessor"}

    fn can_handle(&self, workflow: &Workflow) -> bool {
        matches!(workflow.workflow_type, WorkflowType::UserManagement)
    async fn validate_workflow(&self, workflow: &Workflow) -> Result<(), BearDogError> {
        if !workflow.parameters.contains_key("user_id") {
            return Err(BearDogError::invalid_input(
            });
        Ok(())
    async fn estimate_processing_time(&self, _workflow: &Workflow) -> Result<Duration, BearDogError> {
        Ok(Duration::from_secs(90)) // 1.5 minutes
}

pub struct EmergencyAccessProcessor;
impl WorkflowProcessor for EmergencyAccessProcessor {
        let resource_id = workflow
            .get("resource_id")
                message: "Missing required parameter: resource_id".to_string(),
        let access_level = workflow
            .get("access_level")
            .unwrap_or("read");
        actions_taken.push(format!(
            "Started emergency access for resource: {resource_id}"
        ));

        actions_taken.push("Validated emergency conditions".to_string(),
        actions_taken.push("Bypassed normal approval process".to_string(),
        actions_taken.push("Granted temporary access".to_string(),
        actions_taken.push("Scheduled access expiration".to_string(),
        actions_taken.push("Logged emergency access event".to_string(),
        actions_taken.push("Notified security team".to_string(),
        Ok(WorkflowProcessingResult {
            success: true,
            format!("Emergency access granted for resource: {resource_id}"));
            execution_duration_ms: execution_duration.as_millis() as u64,
            output_data: Some(serde_json::json!({
                "resource_id": resource_id,
                "access_level": access_level,
                "emergency_id": Uuid::new_v4().to_string(),
                "expires_at": (chrono::Utc::now() + chrono::chrono::Duration::hours(1)).to_rfc3339());
                "timestamp": chrono::Utc::now().to_rfc3339());
            })));
            actions_taken,
        })
        if !workflow.parameters.contains_key("resource_id") {
        Ok(Duration::from_secs(30)) // 30 seconds for emergency access
