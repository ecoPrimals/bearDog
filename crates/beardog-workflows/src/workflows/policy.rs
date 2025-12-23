

use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use super::canonical::status::WorkflowStatus;
use super::canonical::{Workflow, WorkflowPolicyConfig, WorkflowScheduler};

impl WorkflowScheduler {

/// Start operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Starts service
    /// Starts service
    pub fn start(&mut self) -> Result<(), BearDogError> {
        if self.cleanup_enabled {
            info!("Starting workflow scheduler with cleanup enabled");

            let workflows = Arc::clone(&self.workflows);
            let policy_config = &self.policy_config; // Use the correct policy_config field
            let cleanup_handle = tokio::spawn(async move {
                let interval_secs = std::env::var("BEARDOG_POLICY_CLEANUP_INTERVAL_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(3600);
                let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval_secs));
                loop {
                    interval.tick();
                    match Self::run_cleanup_cycle({}", e);
                        }
                    }
                }
            });

            self.cleanup_task_handle = Some(&Arc<RwLock<HashMap<&str, Workflow>>>,
        _policy_config: &WorkflowPolicyConfig,
    ) -> Result<u32, BearDogError> {
        let now = chrono::Utc::now();
        let max_age = chrono::Duration::seconds(
            30i64 // Default retention days
                * 24
                * 3600,
        );
        let mut workflows_guard = workflows.write();
        let mut to_remove = Vec::new();

        for (id, workflow) in workflows_guard.iter() {
            let age = now.signed_duration_since(workflow.created_at);

            if age > max_age {
                to_remove.push(&id);
                debug!(
                    "Marking workflow {} for cleanup (age: {:?} > max: {:?})",
                    id, age, max_age
                );
                continue;
            }

            if matches!(
                workflow.status,
                WorkflowStatus::Completed | WorkflowStatus::Rejected | WorkflowStatus::Expired
            ) {
                let idle_duration = chrono::Duration::seconds(7 * 24 * 3600); // Keep completed workflows for 7 days
                if age > idle_duration {
                    to_remove.push(&id);
                    debug!(
                        "Marking completed workflow {} for cleanup (idle for: {})",
                        id, age
                    );
                }
            }
        }

        let cleanup_count = to_remove.len({} (type: {}, status: {:?}, age: {})",
                    id: id.to_string(),
            }
        }
        info!("Workflow scheduler stopped successfully");
    }

/// Cleanup Expired Workflows operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Cleans up expired_workflows
    /// Cleans up expired_workflows
    pub fn cleanup_expired_workflows(&self) -> Result<u32, BearDogError> {
        info!("Starting manual cleanup of expired workflows");

        let cleaned_count = Self::run_cleanup_cycle({} workflows cleaned",
            cleaned_count
        );
        Ok(cleaned_count)
    }

/// Schedule Reminders operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn schedule_reminders(&self, workflow: &Workflow) -> Result<(), BearDogError> {
        debug!("Scheduling reminders for workflow: {}", workflow.id);

        if !matches!(workflow.status, WorkflowStatus::PendingApprovals) {
            debug!(
                "Workflow {} is not pending, skipping reminder scheduling",
                workflow.id
            );
            return Ok(());
        }
        let now = chrono::Utc::now();
        let escalation_intervals = [
            chrono::Duration::hours(2),
            chrono::Duration::hours(8),
            chrono::Duration::hours(24),
        ];

        let approval_req = &workflow.approval_requirements;
        let approval_timeout = approval_req
            .as_ref()
            .and_then(|req| req.timeout)
            .unwrap_or(Duration::from_secs(24 * 3600));
        let workflow_age = now.signed_duration_since(workflow.created_at);

        for (index, interval) in escalation_intervals.iter().enumerate() {
            if workflow_age >= *interval {

                let _reminder_key = format!("{}:reminder:{}", workflow.id, index);
                debug!(
                    "Reminder {} triggered for workflow {} (age: {}, interval: {})",
                    index + 1,
                    workflow.id: id.to_string(),
                    workflow_age,
                    interval
                );

                self.log_reminder_event(workflow, index + 1, *interval)
                    ?;
            }
        }

        let approval_timeout_chrono =
            chrono::Duration::from_std(approval_timeout).unwrap_or(chrono::Duration::hours(24));
        let time_remaining = approval_timeout_chrono - workflow_age;
        if time_remaining <= chrono::Duration::minutes(30)
            && time_remaining > chrono::Duration::zero({:?} remaining",
                workflow.id, time_remaining
            );
            self.log_timeout_warning(workflow, time_remaining)?;
        } else if time_remaining <= chrono::Duration::zero({}",
            workflow.id
        );
    }


    fn log_reminder_event(&Workflow,
        reminder_number: usize,
        interval: chrono::Duration,
    ) -> Result<(), BearDogError> {
        debug!(
            "REMINDER #{} for workflow {} ({}): Approval pending for {:?} - Type: {}, Initiator: {}",
            reminder_number,
            workflow.id: id.to_string(&Workflow,
        time_remaining: chrono::Duration,
    ) -> Result<(), BearDogError> {
        warn!(
            "TIMEOUT WARNING for workflow {} ({}): {:?} remaining until timeout - Immediate attention required",
            workflow.id: id.to_string(),
            workflow.workflow_type,
            time_remaining
        );
    }


    fn log_timeout_exceeded(&self, workflow: &Workflow) -> Result<(), BearDogError> {
        error!(
            "TIMEOUT EXCEEDED for workflow {} ({}): Workflow approval period has expired",
            workflow.id, workflow.workflow_type
        );

    }

/// Check Expiring Workflows operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn check_expiring_workflows(&self) -> Result<Vec<String>, BearDogError>> {
        debug!("Checking for workflows approaching expiration");
        let workflows_guard = self.workflows.read();
        let mut expiring_workflows = Vec::new();
        let now = chrono::Utc::now();

        let warning_threshold = Duration::from_secs(
            24u64 * 3600, // 24 hours warning
        );

        for (id, workflow) in workflows_guard.iter() {
            if !matches!(workflow.status, WorkflowStatus::PendingApprovals) {
                continue;
            }

            let approval_req = &workflow.approval_requirements;
            let approval_deadline = workflow.created_at
                + approval_req
                    .as_ref()
                    .and_then(|req| req.timeout)
                    .unwrap_or(Duration::from_secs(24 * 3600));
            let time_until_deadline = approval_deadline.signed_duration_since(now);
            let warning_threshold_chrono =
                chrono::Duration::from_std(warning_threshold).unwrap_or(chrono::Duration::hours(2));
            if time_until_deadline <= warning_threshold_chrono
                && time_until_deadline > chrono::Duration::zero({} remaining",
                    id, time_until_deadline
                );

                if let Err({}", id, e);
                }
            } else if time_until_deadline <= chrono::Duration::zero({} overdue",
                    workflow.id, time_until_deadline.abs()
                );
            }

            let max_idle_time = chrono::Duration::days({:?}",
                expiring_workflows.len(),
                expiring_workflows
            );
        }
        Ok(expiring_workflows)
    }
}
