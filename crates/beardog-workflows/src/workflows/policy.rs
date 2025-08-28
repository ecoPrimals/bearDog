

use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use super::canonical::status::WorkflowStatus;
use super::canonical::{Workflow, WorkflowPolicyConfig, WorkflowScheduler};

impl WorkflowScheduler {

    pub async fn start(&mut self) -> Result<(), BearDogError> {
        if self.cleanup_enabled {
            info!("Starting workflow scheduler with cleanup enabled");

            let workflows = Arc::clone(&self.workflows);
            let policy_config = self.policy_config.clone(); // Use the correct policy_config field
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

            self.cleanup_task_handle = Some(cleanup_handle);
            info!("Background cleanup task started successfully");
        } else {
            info!("Workflow scheduler started with cleanup disabled");
        }
        Ok(())
    }

    async fn run_cleanup_cycle(
        workflows: &Arc<RwLock<HashMap<&str, Workflow>>>,
        _policy_config: &WorkflowPolicyConfig,
    ) -> Result<u32, BearDogError> {
        let now = chrono::Utc::now();
        let max_age = chrono::Duration::seconds(
            30i64 // Default retention days
                * 24
                * 3600,
        );
        let mut workflows_guard = workflows.write().await;
        let mut to_remove = Vec::new();

        for (id, workflow) in workflows_guard.iter() {
            let age = now.signed_duration_since(workflow.created_at);

            if age > max_age {
                to_remove.push(id.clone());
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
                    to_remove.push(id.clone());
                    debug!(
                        "Marking completed workflow {} for cleanup (idle for: {})",
                        id, age
                    );
                }
            }
        }

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

    pub async fn stop(&mut self) -> Result<(), BearDogError> {
        info!("Stopping workflow scheduler");

        if let Some(handle) = self.cleanup_task_handle.take() {
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
    }

    pub async fn cleanup_expired_workflows(&self) -> Result<u32, BearDogError> {
        info!("Starting manual cleanup of expired workflows");

        let cleaned_count = Self::run_cleanup_cycle(&self.workflows, &self.policy_config).await?;
        info!(
            "Manual cleanup completed: {} workflows cleaned",
            cleaned_count
        );
        Ok(cleaned_count)
    }

    pub async fn schedule_reminders(&self, workflow: &Workflow) -> Result<(), BearDogError> {
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

                let _reminder_key = format_args!("{}:reminder:{}", workflow.id, index).to_string();
                debug!(
                    "Reminder {} triggered for workflow {} (age: {}, interval: {})",
                    index + 1,
                    workflow.id,
                    workflow_age,
                    interval
                );

                self.log_reminder_event(workflow, index + 1, *interval)
                    .await?;
            }
        }

        let approval_timeout_chrono =
            chrono::Duration::from_std(approval_timeout).unwrap_or(chrono::Duration::hours(24));
        let time_remaining = approval_timeout_chrono - workflow_age;
        if time_remaining <= chrono::Duration::minutes(30)
            && time_remaining > chrono::Duration::zero()
        {
            warn!(
                "Workflow {} is approaching timeout: {:?} remaining",
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
    }

    async fn log_reminder_event(
        &self,
        workflow: &Workflow,
        reminder_number: usize,
        interval: chrono::Duration,
    ) -> Result<(), BearDogError> {
        debug!(
            "REMINDER #{} for workflow {} ({}): Approval pending for {:?} - Type: {}, Initiator: {}",
            reminder_number,
            workflow.id,
            workflow.workflow_type,
            interval,
            workflow.initiator
        );

    }

    async fn log_timeout_warning(
        &self,
        workflow: &Workflow,
        time_remaining: chrono::Duration,
    ) -> Result<(), BearDogError> {
        warn!(
            "TIMEOUT WARNING for workflow {} ({}): {:?} remaining until timeout - Immediate attention required",
            workflow.id,
            workflow.workflow_type,
            time_remaining
        );
    }

    async fn log_timeout_exceeded(&self, workflow: &Workflow) -> Result<(), BearDogError> {
        error!(
            "TIMEOUT EXCEEDED for workflow {} ({}): Workflow approval period has expired",
            workflow.id, workflow.workflow_type
        );

    }

    pub async fn check_expiring_workflows(&self) -> Result<Vec<String>, BearDogError>> {
        debug!("Checking for workflows approaching expiration");
        let workflows_guard = self.workflows.read().await;
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
                && time_until_deadline > chrono::Duration::zero()
            {
                expiring_workflows.push(id.clone());
                debug!(
                    "Workflow {} is approaching approval deadline: {} remaining",
                    id, time_until_deadline
                );

                if let Err(e) = self.schedule_reminders(workflow).await {
                    error!("Failed to schedule reminders for workflow {}: {}", id, e);
                }
            } else if time_until_deadline <= chrono::Duration::zero() {
                warn!(
                    "Workflow {} has already expired: {} overdue",
                    workflow.id, time_until_deadline.abs()
                );
            }

            let max_idle_time = chrono::Duration::days(7); // Consider workflow stale after 7 days
            let idle_time = now.signed_duration_since(workflow.created_at);
            if idle_time > max_idle_time && workflow.approvals.is_empty() {
                warn!(
                    "Workflow {} has been idle for {} with no approvals",
                    workflow.id, idle_time
                );
            }
        }
        if expiring_workflows.is_empty() {
            debug!("No workflows found approaching expiration");
            info!(
                "Found {} workflows approaching expiration: {:?}",
                expiring_workflows.len(),
                expiring_workflows
            );
        }
        Ok(expiring_workflows)
    }
}
