// SPDX-License-Identifier: AGPL-3.0-or-later

//! Chain processing and retry configuration

use crate::canonical::traits::RetryStrategy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Chain processing and workflow configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChainConfig {
    /// Maximum chain length
    pub max_chain_length: usize,

    /// Chain processing timeout
    pub processing_timeout: Duration,

    /// Step configuration
    pub step: StepConfig,

    /// Retry configuration
    pub retry: RetryConfig,

    /// Enable parallel processing
    pub parallel_enabled: bool,

    /// Maximum parallel workers
    pub max_workers: usize,
}

/// Individual step configuration within chains
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StepConfig {
    /// Step timeout
    pub timeout: Duration,

    /// Maximum step attempts
    pub max_attempts: u32,

    /// Enable step validation
    pub validation_enabled: bool,

    /// Step metadata
    pub metadata: HashMap<String, String>,
}

/// Retry configuration for failed operations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetryConfig {
    /// Maximum retry attempts
    pub max_attempts: u32,

    /// Initial retry delay
    pub initial_delay: Duration,

    /// Maximum retry delay
    pub max_delay: Duration,

    /// Backoff multiplier
    pub backoff_multiplier: f64,

    /// Enable exponential backoff
    pub exponential_backoff: bool,

    /// Jitter factor (0.0 to 1.0)
    pub jitter_factor: f64,
}

impl ChainConfig {
    /// Default maximum chain length
    pub const DEFAULT_MAX_CHAIN_LENGTH: usize = 10;

    /// Default processing timeout in seconds
    pub const DEFAULT_PROCESSING_TIMEOUT_SECS: u64 = 300;

    /// Default maximum workers
    pub const DEFAULT_MAX_WORKERS: usize = 4;

    /// Create `ChainConfig` with hardcoded defaults
    pub fn with_defaults() -> Self {
        Self {
            max_chain_length: Self::DEFAULT_MAX_CHAIN_LENGTH,
            processing_timeout: Duration::from_secs(Self::DEFAULT_PROCESSING_TIMEOUT_SECS),
            step: StepConfig::default(),
            retry: RetryConfig::default(),
            parallel_enabled: true,
            max_workers: Self::DEFAULT_MAX_WORKERS,
        }
    }

    /// Create `ChainConfig` from environment variables
    pub fn from_env() -> Self {
        Self {
            max_chain_length: std::env::var("BEARDOG_ADAPTER_MAX_CHAIN_LENGTH")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(Self::DEFAULT_MAX_CHAIN_LENGTH),
            processing_timeout: Duration::from_secs(
                std::env::var("BEARDOG_ADAPTER_PROCESSING_TIMEOUT_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(Self::DEFAULT_PROCESSING_TIMEOUT_SECS),
            ),
            step: StepConfig::default(),
            retry: RetryConfig::default(),
            parallel_enabled: true,
            max_workers: std::env::var("BEARDOG_ADAPTER_MAX_WORKERS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(Self::DEFAULT_MAX_WORKERS),
        }
    }
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl Default for StepConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(
                std::env::var("BEARDOG_ADAPTER_STEP_TIMEOUT_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            max_attempts: std::env::var("BEARDOG_ADAPTER_STEP_MAX_ATTEMPTS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            validation_enabled: std::env::var("BEARDOG_ADAPTER_STEP_VALIDATION_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            metadata: HashMap::new(),
        }
    }
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: std::env::var("BEARDOG_ADAPTER_RETRY_MAX_ATTEMPTS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            initial_delay: Duration::from_millis(
                std::env::var("BEARDOG_ADAPTER_RETRY_INITIAL_DELAY_MS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(100),
            ),
            max_delay: Duration::from_secs(
                std::env::var("BEARDOG_ADAPTER_RETRY_MAX_DELAY_SECS")
                    .ok()
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(30),
            ),
            backoff_multiplier: std::env::var("BEARDOG_ADAPTER_RETRY_BACKOFF_MULTIPLIER")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2.0),
            exponential_backoff: true,
            jitter_factor: 0.1,
        }
    }
}

// Implement RetryStrategy trait for adapter retry configuration
impl RetryStrategy for RetryConfig {
    fn max_attempts(&self) -> u32 {
        self.max_attempts
    }

    fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if self.exponential_backoff {
            // Exponential backoff with multiplier and optional jitter
            #[expect(
                clippy::cast_possible_truncation,
                reason = "backoff delay clamped to millis as u64 for Duration::from_millis"
            )]
            #[expect(
                clippy::cast_precision_loss,
                reason = "acceptable imprecision for non-cryptographic backoff math"
            )]
            #[expect(
                clippy::cast_sign_loss,
                reason = "delay clamped non-negative before f64 to u64"
            )]
            #[expect(
                clippy::cast_possible_wrap,
                reason = "retry attempt capped at 30 for powi exponent"
            )]
            let delay_ms = {
                let initial_ms = self.initial_delay.as_millis().min(u128::from(u64::MAX)) as f64;
                let max_ms = self.max_delay.as_millis().min(u128::from(u64::MAX)) as u64;
                let base_delay = initial_ms * self.backoff_multiplier.powi(attempt.min(30) as i32);

                let jitter = if self.jitter_factor > 0.0 {
                    use std::collections::hash_map::RandomState;
                    use std::hash::BuildHasher;

                    let random = (RandomState::new().hash_one(attempt) % 1000) as f64 / 1000.0;
                    base_delay * self.jitter_factor * random
                } else {
                    0.0
                };

                (base_delay + jitter).min(max_ms as f64).max(0.0) as u64
            };
            Duration::from_millis(delay_ms)
        } else {
            // Linear backoff
            self.initial_delay.min(self.max_delay)
        }
    }

    fn backoff_multiplier(&self) -> f64 {
        self.backoff_multiplier
    }

    fn should_retry_error(&self, _error: &(dyn std::error::Error + Send + Sync)) -> bool {
        true
    }

    fn is_limit_reached(&self, attempts: u32) -> bool {
        attempts >= self.max_attempts
    }

    fn total_delay(&self, attempts: u32) -> Duration {
        let mut total = Duration::from_secs(0);
        for attempt in 0..attempts {
            total += self.delay_for_attempt(attempt);
        }
        total
    }
}
