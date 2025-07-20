//! Maintenance Module
//!
//! Handles cleanup operations, maintenance tasks, and system optimization.

use super::*;
use serde::{Deserialize, Serialize};

impl BearDogSecurityProvider {
    /// Cleanup expired data across all components
    pub async fn cleanup_expired_data(&mut self) -> BearDogResult<CleanupReport> {
        let start_time = Utc::now();
        let mut report = CleanupReport::default();

        // Clean expired sessions
        let expired_sessions = self.cleanup_expired_sessions().await?;
        report.expired_sessions_removed = expired_sessions;

        // Clean expired MFA tokens
        let expired_mfa = self.cleanup_expired_mfa_tokens().await?;
        report.expired_mfa_tokens_removed = expired_mfa;

        // Clean expired account locks
        let expired_locks = self.cleanup_expired_locks().await?;
        report.expired_locks_removed = expired_locks;

        // Clean old rate limit data
        let cleaned_rate_limits = self.cleanup_rate_limit_data().await?;
        report.rate_limit_entries_cleaned = cleaned_rate_limits;

        // Clean old audit data (keep last 30 days by default)
        let cleaned_audit = self.cleanup_old_audit_data(30).await?;
        report.old_audit_entries_removed = cleaned_audit;

        report.cleanup_duration_ms = (Utc::now() - start_time).num_milliseconds() as u64;
        report.cleanup_timestamp = start_time;

        // Update metrics
        self.metrics.maintenance_operations += 1;
        self.metrics.last_cleanup = Some(start_time);

        Ok(report)
    }

    /// Clean rate limiting data
    async fn cleanup_rate_limit_data(&mut self) -> BearDogResult<u32> {
        let now = Utc::now();
        let cutoff = now - Duration::seconds(self.rate_limiter.config.window_seconds as i64 * 2);

        let initial_count = self.rate_limiter.state.len();

        // Remove old rate limiter states
        self.rate_limiter
            .state
            .retain(|_, state| state.last_operation > cutoff);

        let removed_count = initial_count - self.rate_limiter.state.len();
        Ok(removed_count as u32)
    }

    /// Clean old audit data
    async fn cleanup_old_audit_data(&mut self, retention_days: u32) -> BearDogResult<u32> {
        let cutoff = Utc::now() - Duration::days(retention_days as i64);
        self.audit_manager.cleanup_old_events(cutoff).await
    }

    /// Optimize system performance
    pub async fn optimize_system_performance(&mut self) -> BearDogResult<OptimizationReport> {
        let start_time = Utc::now();
        let mut report = OptimizationReport::default();

        // Optimize session storage
        let session_optimizations = self.optimize_session_storage().await?;
        report.session_optimizations = session_optimizations;

        // Optimize rate limiter
        let rate_limit_optimizations = self.optimize_rate_limiter().await?;
        report.rate_limit_optimizations = rate_limit_optimizations;

        // Compact audit logs
        let audit_optimizations = self.compact_audit_logs().await?;
        report.audit_optimizations = audit_optimizations;

        report.optimization_duration_ms = (Utc::now() - start_time).num_milliseconds() as u64;
        report.optimization_timestamp = start_time;

        Ok(report)
    }

    /// Optimize session storage
    async fn optimize_session_storage(&mut self) -> BearDogResult<u32> {
        // Defragment session storage and optimize memory usage
        let initial_capacity = self.session_store.capacity();
        self.session_store.shrink_to_fit();
        let final_capacity = self.session_store.capacity();

        Ok((initial_capacity - final_capacity) as u32)
    }

    /// Optimize rate limiter performance
    async fn optimize_rate_limiter(&mut self) -> BearDogResult<u32> {
        // Optimize rate limiter state storage
        let initial_count = self.rate_limiter.state.len();

        // Remove inactive users (no operations in the last hour)
        let cutoff = Utc::now() - Duration::hours(1);
        self.rate_limiter
            .state
            .retain(|_, state| state.last_operation > cutoff);

        let optimized_count = initial_count - self.rate_limiter.state.len();
        Ok(optimized_count as u32)
    }

    /// Compact audit logs for better performance
    async fn compact_audit_logs(&mut self) -> BearDogResult<u32> {
        self.audit_manager.compact_logs().await
    }

    /// Schedule periodic maintenance
    pub async fn schedule_maintenance(&mut self, interval_hours: u32) -> BearDogResult<()> {
        // In a production system, this would set up a background task
        // For now, we just record the maintenance schedule
        self.metrics.maintenance_schedule_hours = interval_hours;
        Ok(())
    }

    /// Get maintenance status
    pub async fn get_maintenance_status(&self) -> MaintenanceStatus {
        MaintenanceStatus {
            last_cleanup: self.metrics.last_cleanup,
            last_optimization: self.metrics.last_optimization,
            maintenance_operations: self.metrics.maintenance_operations,
            scheduled_interval_hours: self.metrics.maintenance_schedule_hours,
            next_scheduled_maintenance: self
                .metrics
                .last_cleanup
                .map(|last| last + Duration::hours(self.metrics.maintenance_schedule_hours as i64)),
        }
    }
}

/// Cleanup operation report
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CleanupReport {
    pub cleanup_timestamp: chrono::DateTime<Utc>,
    pub cleanup_duration_ms: u64,
    pub expired_sessions_removed: u32,
    pub expired_mfa_tokens_removed: u32,
    pub expired_locks_removed: u32,
    pub rate_limit_entries_cleaned: u32,
    pub old_audit_entries_removed: u32,
}

/// System optimization report
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OptimizationReport {
    pub optimization_timestamp: chrono::DateTime<Utc>,
    pub optimization_duration_ms: u64,
    pub session_optimizations: u32,
    pub rate_limit_optimizations: u32,
    pub audit_optimizations: u32,
}

/// Maintenance status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceStatus {
    pub last_cleanup: Option<chrono::DateTime<Utc>>,
    pub last_optimization: Option<chrono::DateTime<Utc>>,
    pub maintenance_operations: u64,
    pub scheduled_interval_hours: u32,
    pub next_scheduled_maintenance: Option<chrono::DateTime<Utc>>,
}
