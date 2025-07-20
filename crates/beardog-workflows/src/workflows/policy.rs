//! Policy engine and scheduler for workflows
//!
//! Contains policy determination and workflow scheduling logic.

use super::types::*;
use beardog_errors::BearDogResult;
use chrono::{Datelike, Duration, Timelike, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

impl WorkflowPolicyEngine {
    /// Create a new workflow policy engine
    pub fn new(config: PolicyConfig) -> Self {
        Self { config }
    }

    /// Determine approval requirements for a workflow
    pub async fn determine_approval_requirements(
        &self,
        workflow_type: &WorkflowType,
        priority: &WorkflowPriority,
    ) -> BearDogResult<ApprovalRequirements> {
        // Use default approval requirements based on workflow type
        // (approval_matrix field doesn't exist in current PolicyConfig structure)

        // Default approval requirements based on priority and type
        let (required_approvals, timeout) = match priority {
            WorkflowPriority::Emergency => (1, Duration::hours(1)),
            WorkflowPriority::Critical => (2, Duration::hours(4)),
            WorkflowPriority::High => (2, Duration::hours(8)),
            WorkflowPriority::Normal => (
                1,
                Duration::hours(self.config.default_approval_timeout_hours as i64),
            ),
            WorkflowPriority::Low => (1, Duration::days(2)),
        };

        // Create approval tiers based on workflow type
        let approval_hierarchy = self
            .create_approval_hierarchy(workflow_type, required_approvals)
            .await?;

        Ok(ApprovalRequirements {
            tiers: approval_hierarchy,
            minimum_approvals: required_approvals,
            require_all_tiers: false,
            approval_timeout: Some(timeout),
            allow_delegation: self.is_delegation_allowed(workflow_type),
        })
    }

    async fn create_approval_hierarchy(
        &self,
        workflow_type: &WorkflowType,
        required_approvals: u32,
    ) -> BearDogResult<Vec<ApprovalTier>> {
        let _workflow_key = match workflow_type {
            WorkflowType::KeyRotation => "key_rotation",
            WorkflowType::KeyDeletion => "key_deletion",
            WorkflowType::PolicyChange => "policy_change",
            WorkflowType::ConfigurationChange => "configuration_change",
            WorkflowType::UserProvisioning => "user_provisioning",
            WorkflowType::EmergencyAccess => "emergency_access",
            WorkflowType::SystemMaintenance => "system_maintenance",
            WorkflowType::ComplianceAudit => "compliance_audit",
        };

        // Get eligible users from configuration
        // Note: eligible_users_by_workflow field doesn't exist in current PolicyConfig
        // Using default admin users for now
        let eligible_users = vec!["admin".to_string(), "security_admin".to_string()];

        match workflow_type {
            WorkflowType::KeyRotation | WorkflowType::KeyDeletion => Ok(vec![ApprovalTier {
                name: "security_team".to_string(),
                required_approvers: eligible_users,
                minimum_approvals: required_approvals,
                optional: false,
            }]),
            WorkflowType::PolicyChange | WorkflowType::ConfigurationChange => {
                Ok(vec![ApprovalTier {
                    name: "policy_admins".to_string(),
                    required_approvers: eligible_users.clone(),
                    minimum_approvals: required_approvals,
                    optional: false,
                }])
            }
            WorkflowType::UserProvisioning => Ok(vec![ApprovalTier {
                name: "user_admins".to_string(),
                required_approvers: eligible_users.clone(),
                minimum_approvals: required_approvals,
                optional: false,
            }]),
            WorkflowType::EmergencyAccess => {
                Ok(vec![ApprovalTier {
                    name: "emergency_contacts".to_string(),
                    required_approvers: eligible_users.clone(),
                    minimum_approvals: 1, // Emergency access needs quick approval
                    optional: false,
                }])
            }
            WorkflowType::SystemMaintenance => Ok(vec![ApprovalTier {
                name: "system_admins".to_string(),
                required_approvers: vec!["admin".to_string()],
                minimum_approvals: required_approvals,
                optional: false,
            }]),
            WorkflowType::ComplianceAudit => Ok(vec![ApprovalTier {
                name: "compliance_officers".to_string(),
                required_approvers: vec!["admin".to_string()],
                minimum_approvals: required_approvals,
                optional: false,
            }]),
        }
    }

    fn get_required_roles(&self, workflow_type: &WorkflowType) -> Vec<String> {
        match workflow_type {
            WorkflowType::KeyRotation | WorkflowType::KeyDeletion => {
                vec!["security_admin".to_string()]
            }
            WorkflowType::PolicyChange | WorkflowType::ConfigurationChange => {
                vec!["policy_admin".to_string()]
            }
            WorkflowType::UserProvisioning => {
                vec!["user_admin".to_string()]
            }
            WorkflowType::EmergencyAccess => {
                vec!["emergency_contact".to_string()]
            }
            WorkflowType::SystemMaintenance => {
                vec!["system_admin".to_string()]
            }
            WorkflowType::ComplianceAudit => {
                vec!["compliance_officer".to_string()]
            }
        }
    }

    fn is_delegation_allowed(&self, workflow_type: &WorkflowType) -> bool {
        match workflow_type {
            WorkflowType::EmergencyAccess => false, // No delegation for emergency access
            WorkflowType::KeyDeletion => false,     // No delegation for key deletion
            _ => true,
        }
    }

    fn is_self_approval_allowed(&self, workflow_type: &WorkflowType) -> bool {
        !matches!(
            workflow_type,
            WorkflowType::KeyRotation
                | WorkflowType::KeyDeletion
                | WorkflowType::PolicyChange
                | WorkflowType::ConfigurationChange
                | WorkflowType::EmergencyAccess
        )
    }

    /// Check if a workflow is within business hours
    pub fn is_within_business_hours(&self) -> bool {
        let business_hours = &self.config.business_hours;
        let now = Utc::now();
        let weekday = now.weekday().num_days_from_sunday();

        // Check if current day is a business day
        if !business_hours.days_of_week.contains(&weekday) {
            return false;
        }

        // Check if current hour is within business hours
        let current_hour = now.hour();
        current_hour >= business_hours.start_hour && current_hour < business_hours.end_hour
    }

    /// Get escalation intervals for a workflow
    pub fn get_escalation_intervals(&self, _workflow_type: &WorkflowType) -> Vec<Duration> {
        // Default escalation intervals since escalation_intervals field doesn't exist
        vec![Duration::hours(2), Duration::hours(8), Duration::hours(24)]
    }
}

impl WorkflowScheduler {
    /// Start the workflow scheduler
    pub async fn start(&self) -> BearDogResult<()> {
        if self.cleanup_enabled {
            info!("Starting workflow scheduler with cleanup enabled");

            // Start background cleanup task
            let workflows = Arc::clone(&self.workflows);
            let policy_config = Arc::clone(&self.policy_config);

            let cleanup_handle = tokio::spawn(async move {
                let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // Run every hour
                loop {
                    interval.tick().await;
                    match Self::run_cleanup_cycle(&workflows, &policy_config).await {
                        Ok(cleaned_count) => {
                            if cleaned_count > 0 {
                                info!("Cleaned up {} expired workflows", cleaned_count);
                            } else {
                                debug!("No workflows needed cleanup");
                            }
                        }
                        Err(e) => {
                            error!("Error during workflow cleanup cycle: {}", e);
                        }
                    }
                }
            });

            // Store the handle using interior mutability
            *self.cleanup_task_handle.write().await = Some(cleanup_handle);
            info!("Background cleanup task started successfully");
        } else {
            info!("Workflow scheduler started with cleanup disabled");
        }
        Ok(())
    }

    /// Internal cleanup cycle runner
    async fn run_cleanup_cycle(
        workflows: &Arc<RwLock<HashMap<String, Workflow>>>,
        policy_config: &Arc<PolicyConfig>,
    ) -> BearDogResult<u32> {
        let now = chrono::Utc::now();
        let max_age = chrono::Duration::days(policy_config.data_retention_days as i64);
        let mut workflows_guard = workflows.write().await;
        let mut to_remove = Vec::new();

        // Find workflows that need cleanup
        for (id, workflow) in workflows_guard.iter() {
            let age = now.signed_duration_since(workflow.created_at);

            // Check if workflow is too old
            if age > max_age {
                to_remove.push(id.clone());
                debug!(
                    "Marking workflow {} for cleanup (age: {} > max: {})",
                    id, age, max_age
                );
                continue;
            }

            // Check if workflow is completed and has been idle too long
            if matches!(
                workflow.status,
                WorkflowStatus::Completed | WorkflowStatus::Rejected | WorkflowStatus::Expired
            ) {
                let idle_duration = chrono::Duration::days(7); // Keep completed workflows for 7 days
                if age > idle_duration {
                    to_remove.push(id.clone());
                    debug!(
                        "Marking completed workflow {} for cleanup (idle for: {})",
                        id, age
                    );
                }
            }
        }

        // Remove the workflows
        let cleanup_count = to_remove.len() as u32;
        for id in to_remove {
            if let Some(removed) = workflows_guard.remove(&id) {
                info!(
                    "Cleaned up workflow: {} (type: {}, status: {:?}, age: {})",
                    id,
                    removed.workflow_type,
                    removed.status,
                    now.signed_duration_since(removed.created_at)
                );
            }
        }

        Ok(cleanup_count)
    }

    /// Stop the workflow scheduler
    pub async fn stop(&self) -> BearDogResult<()> {
        info!("Stopping workflow scheduler");

        // Stop background cleanup task if running
        if let Some(handle) = self.cleanup_task_handle.write().await.take() {
            handle.abort();
            match handle.await {
                Ok(_) => debug!("Background cleanup task completed normally"),
                Err(e) if e.is_cancelled() => {
                    debug!("Background cleanup task cancelled successfully")
                }
                Err(e) => warn!("Background cleanup task ended with error: {}", e),
            }
        }

        info!("Workflow scheduler stopped successfully");
        Ok(())
    }

    /// Cleanup expired workflows
    pub async fn cleanup_expired_workflows(&self) -> BearDogResult<u32> {
        info!("Starting manual cleanup of expired workflows");

        // Run the cleanup cycle manually
        let cleaned_count = Self::run_cleanup_cycle(&self.workflows, &self.policy_config).await?;

        info!(
            "Manual cleanup completed: {} workflows cleaned",
            cleaned_count
        );
        Ok(cleaned_count)
    }

    /// Schedule workflow reminder notifications
    pub async fn schedule_reminders(&self, workflow: &Workflow) -> BearDogResult<()> {
        debug!("Scheduling reminders for workflow: {}", workflow.id);

        // Only schedule reminders for pending workflows
        if !matches!(workflow.status, WorkflowStatus::PendingApprovals) {
            debug!(
                "Workflow {} is not pending, skipping reminder scheduling",
                workflow.id
            );
            return Ok(());
        }

        let now = chrono::Utc::now();
        let escalation_intervals = [Duration::hours(2), Duration::hours(8), Duration::hours(24)];

        // Calculate reminder times based on workflow approval requirements
        let approval_req = &workflow.approval_requirements;
        let approval_timeout = approval_req.approval_timeout.unwrap_or(Duration::hours(24));
        let workflow_age = now.signed_duration_since(workflow.created_at);

        // Schedule reminders at each escalation interval
        for (index, interval) in escalation_intervals.iter().enumerate() {
            if workflow_age >= *interval {
                // This reminder interval has already passed, check if we need to send it
                let _reminder_key = format!("{}:reminder:{}", workflow.id, index);

                info!(
                    "Reminder {} triggered for workflow {} (age: {}, interval: {})",
                    index + 1,
                    workflow.id,
                    workflow_age,
                    interval
                );

                // Here we would typically:
                // 1. Check if this reminder was already sent (using metadata or separate storage)
                // 2. Send notification to approvers
                // 3. Mark reminder as sent

                // For now, we'll just log the reminder event
                self.log_reminder_event(workflow, index + 1, *interval)
                    .await?;
            }
        }

        // Check if workflow is approaching timeout
        let time_remaining = approval_timeout - workflow_age;
        if time_remaining <= chrono::Duration::minutes(30)
            && time_remaining > chrono::Duration::zero()
        {
            warn!(
                "Workflow {} is approaching timeout: {} remaining",
                workflow.id, time_remaining
            );
            self.log_timeout_warning(workflow, time_remaining).await?;
        } else if time_remaining <= chrono::Duration::zero() {
            warn!("Workflow {} has exceeded its approval timeout", workflow.id);
            self.log_timeout_exceeded(workflow).await?;
        }

        debug!(
            "Reminder scheduling completed for workflow: {}",
            workflow.id
        );
        Ok(())
    }

    /// Log a reminder event
    async fn log_reminder_event(
        &self,
        workflow: &Workflow,
        reminder_number: usize,
        interval: Duration,
    ) -> BearDogResult<()> {
        info!(
            "REMINDER #{} for workflow {} ({}): Approval pending for {} - Type: {}, Initiator: {}",
            reminder_number,
            workflow.id,
            workflow.workflow_type,
            interval,
            workflow.workflow_type,
            workflow.initiator
        );

        // In a full implementation, this would:
        // 1. Send actual notifications via email/Slack/Teams
        // 2. Update workflow metadata with reminder timestamps
        // 3. Escalate to higher-level approvers if configured

        Ok(())
    }

    /// Log a timeout warning
    async fn log_timeout_warning(
        &self,
        workflow: &Workflow,
        time_remaining: Duration,
    ) -> BearDogResult<()> {
        warn!(
            "TIMEOUT WARNING for workflow {} ({}): {} remaining until timeout - Immediate attention required",
            workflow.id,
            workflow.workflow_type,
            time_remaining
        );
        Ok(())
    }

    /// Log timeout exceeded
    async fn log_timeout_exceeded(&self, workflow: &Workflow) -> BearDogResult<()> {
        error!(
            "TIMEOUT EXCEEDED for workflow {} ({}): Workflow approval period has expired",
            workflow.id, workflow.workflow_type
        );
        // In a full implementation, this might automatically reject the workflow
        Ok(())
    }

    /// Check for workflows approaching expiration
    pub async fn check_expiring_workflows(&self) -> BearDogResult<Vec<String>> {
        debug!("Checking for workflows approaching expiration");

        let now = chrono::Utc::now();
        let workflows_guard = self.workflows.read().await;
        let mut expiring_workflows = Vec::new();

        // Define what "approaching expiration" means
        let warning_threshold = chrono::Duration::hours(2); // Warn if expiring within 2 hours

        for (id, workflow) in workflows_guard.iter() {
            // Only check active workflows
            if !matches!(workflow.status, WorkflowStatus::PendingApprovals) {
                continue;
            }

            // Check approval timeout expiration
            let approval_req = &workflow.approval_requirements;
            let approval_deadline =
                workflow.created_at + approval_req.approval_timeout.unwrap_or(Duration::hours(24));
            let time_until_deadline = approval_deadline.signed_duration_since(now);

            if time_until_deadline <= warning_threshold
                && time_until_deadline > chrono::Duration::zero()
            {
                expiring_workflows.push(id.clone());
                debug!(
                    "Workflow {} is approaching approval deadline: {} remaining",
                    id, time_until_deadline
                );

                // Schedule reminders for this workflow
                if let Err(e) = self.schedule_reminders(workflow).await {
                    error!("Failed to schedule reminders for workflow {}: {}", id, e);
                }
            } else if time_until_deadline <= chrono::Duration::zero() {
                expiring_workflows.push(id.clone());
                warn!(
                    "Workflow {} has already expired: {} overdue",
                    id,
                    time_until_deadline.abs()
                );
            }

            // Check if workflow has been idle too long (no activity)
            let max_idle_time = chrono::Duration::days(7); // Consider workflow stale after 7 days
            let idle_time = now.signed_duration_since(workflow.created_at);

            if idle_time > max_idle_time && workflow.approvals.is_empty() {
                expiring_workflows.push(id.clone());
                warn!(
                    "Workflow {} has been idle for {} with no approvals",
                    id, idle_time
                );
            }
        }

        if expiring_workflows.is_empty() {
            debug!("No workflows found approaching expiration");
        } else {
            info!(
                "Found {} workflows approaching expiration: {:?}",
                expiring_workflows.len(),
                expiring_workflows
            );
        }

        Ok(expiring_workflows)
    }
}
