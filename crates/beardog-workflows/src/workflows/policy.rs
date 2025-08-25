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


/// Workflow Policy Management System
///
/// Provides comprehensive policy management for workflow operations including approval requirements,
/// escalation policies, and compliance enforcement.
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use super::canonical::status::WorkflowStatus;
use super::canonical::{Workflow, WorkflowPolicyConfig, WorkflowScheduler};
// Remove duplicate WorkflowPolicyEngine impl - using canonical version from types.rs
/// Workflow scheduler for managing workflow execution timing
impl WorkflowScheduler {
    /// Start the workflow scheduler
    pub async fn start(&mut self) -> BearDogResult<()> {
        if self.cleanup_enabled {
            info!("Starting workflow scheduler with cleanup enabled");
            // Start background cleanup task
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
            // Store the handle using interior mutability
            self.cleanup_task_handle = Some(cleanup_handle);
            info!("Background cleanup task started successfully");
        } else {
            info!("Workflow scheduler started with cleanup disabled");
        }
        Ok(())
    }
    /// Internal cleanup cycle runner
    async fn run_cleanup_cycle(
        workflows: &Arc<RwLock<HashMap<String, Workflow>>>,
        _policy_config: &WorkflowPolicyConfig,
    ) -> BearDogResult<u32> {
        let now = chrono::Utc::now();
        let max_age = chrono::Duration::seconds(
            30i64 // Default retention days
                * 24
                * 3600,
        );
        let mut workflows_guard = workflows.write().await;
        let mut to_remove = Vec::new();
        // Find workflows that need cleanup
        for (id, workflow) in workflows_guard.iter() {
            let age = now.signed_duration_since(workflow.created_at);
            // Check if workflow is too old
            if age > max_age {
                to_remove.push(id.clone());
                debug!(
                    "Marking workflow {} for cleanup (age: {:?} > max: {:?})",
                    id, age, max_age
                );
                continue;
            }
            // Check if workflow is completed and has been idle too long
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
    pub async fn stop(&mut self) -> BearDogResult<()> {
        info!("Stopping workflow scheduler");
        // Stop background cleanup task if running
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
        let escalation_intervals = [
            chrono::Duration::hours(2),
            chrono::Duration::hours(8),
            chrono::Duration::hours(24),
        ];
        // Calculate reminder times based on workflow approval requirements
        let approval_req = &workflow.approval_requirements;
        let approval_timeout = approval_req
            .as_ref()
            .and_then(|req| req.timeout)
            .unwrap_or(Duration::from_secs(24 * 3600));
        let workflow_age = now.signed_duration_since(workflow.created_at);
        // Schedule reminders at each escalation interval
        for (index, interval) in escalation_intervals.iter().enumerate() {
            if workflow_age >= *interval {
                // This reminder interval has already passed, check if we need to send it
                let _reminder_key = format!("{}:reminder:{}", workflow.id, index);
                debug!(
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
    /// Log a reminder event
    async fn log_reminder_event(
        &self,
        workflow: &Workflow,
        reminder_number: usize,
        interval: chrono::Duration,
    ) -> BearDogResult<()> {
        debug!(
            "REMINDER #{} for workflow {} ({}): Approval pending for {:?} - Type: {}, Initiator: {}",
            reminder_number,
            workflow.id,
            workflow.workflow_type,
            interval,
            workflow.initiator
        );
        // In a full implementation, this would:
        // 1. Send actual notifications via email/Slack/Teams
        // 2. Update workflow metadata with reminder timestamps
        // 3. Escalate to higher-level approvers if configured
    }
    /// Log a timeout warning
    async fn log_timeout_warning(
        &self,
        workflow: &Workflow,
        time_remaining: chrono::Duration,
    ) -> BearDogResult<()> {
        warn!(
            "TIMEOUT WARNING for workflow {} ({}): {:?} remaining until timeout - Immediate attention required",
            workflow.id,
            workflow.workflow_type,
            time_remaining
        );
    }
    /// Log timeout exceeded
    async fn log_timeout_exceeded(&self, workflow: &Workflow) -> BearDogResult<()> {
        error!(
            "TIMEOUT EXCEEDED for workflow {} ({}): Workflow approval period has expired",
            workflow.id, workflow.workflow_type
        );
        // In a full implementation, this might automatically reject the workflow
    }
    /// Check for workflows approaching expiration
    pub async fn check_expiring_workflows(&self) -> BearDogResult<Vec<String>> {
        debug!("Checking for workflows approaching expiration");
        let workflows_guard = self.workflows.read().await;
        let mut expiring_workflows = Vec::new();
        let now = chrono::Utc::now();
        // Define what "approaching expiration" means
        let warning_threshold = Duration::from_secs(
            24u64 * 3600, // 24 hours warning
        );
        // Only check active workflows
        for (id, workflow) in workflows_guard.iter() {
            if !matches!(workflow.status, WorkflowStatus::PendingApprovals) {
                continue;
            }
            // Check approval timeout expiration
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
                // Schedule reminders for this workflow
                if let Err(e) = self.schedule_reminders(workflow).await {
                    error!("Failed to schedule reminders for workflow {}: {}", id, e);
                }
            } else if time_until_deadline <= chrono::Duration::zero() {
                warn!(
                    "Workflow {} has already expired: {} overdue",
                    workflow.id, time_until_deadline.abs()
                );
            }
            // Check if workflow has been idle too long (no activity)
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
