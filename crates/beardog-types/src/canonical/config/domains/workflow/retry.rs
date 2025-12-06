//! Workflow Retry Configuration
//!
//! Retry policies and exponential backoff configuration for workflows.

use crate::canonical::traits::RetryStrategy;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Retry configuration for workflows
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetryConfig {
    /// Maximum retry attempts
    pub max_attempts: usize,

    /// Initial delay
    pub initial_delay: Duration,

    /// Backoff multiplier
    pub backoff_multiplier: f64,

    /// Maximum delay
    pub max_delay: Duration,
}

// ============================================================================
// Trait Implementations
// ============================================================================

// Implement RetryStrategy trait for workflow retry configuration
impl RetryStrategy for RetryConfig {
    fn max_attempts(&self) -> u32 {
        self.max_attempts as u32 // Convert from usize
    }

    fn delay_for_attempt(&self, attempt: u32) -> Duration {
        // Exponential backoff
        // Note: Precision loss is acceptable for delay calculations (not cryptographic)
        #[allow(
            clippy::cast_precision_loss,
            clippy::cast_possible_wrap,
            clippy::cast_sign_loss
        )]
        let delay_ms = {
            let initial_ms = self.initial_delay.as_millis().min(u64::MAX as u128) as f64;
            let computed = initial_ms * self.backoff_multiplier.powi(attempt.min(30) as i32);
            computed.max(0.0) as u64
        };
        let delay = Duration::from_millis(delay_ms);
        delay.min(self.max_delay)
    }

    fn backoff_multiplier(&self) -> f64 {
        self.backoff_multiplier
    }

    fn should_retry_error(&self, _error: &(dyn std::error::Error + Send + Sync)) -> bool {
        // Workflow config: retry all errors by default
        true
    }

    fn is_limit_reached(&self, attempts: u32) -> bool {
        attempts >= self.max_attempts as u32
    }

    fn total_delay(&self, attempts: u32) -> Duration {
        let mut total = Duration::from_secs(0);
        for attempt in 0..attempts {
            total += self.delay_for_attempt(attempt);
        }
        total
    }
}

// ============================================================================
// Default Implementation
// ============================================================================

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: std::env::var("BEARDOG_WORKFLOW_RETRY_MAX_ATTEMPTS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            initial_delay: Duration::from_millis(
                std::env::var("BEARDOG_RETRY_INITIAL_DELAY_MS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100),
            ),
            backoff_multiplier: std::env::var("BEARDOG_RETRY_BACKOFF_MULTIPLIER")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2.0),
            max_delay: Duration::from_secs(
                std::env::var("BEARDOG_RETRY_MAX_DELAY_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
        }
    }
}
