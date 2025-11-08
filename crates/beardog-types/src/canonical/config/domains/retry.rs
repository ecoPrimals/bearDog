// Canonical Retry Configuration
//
// This is the unified retry configuration used across all BearDog components.
// All domain-specific RetryConfig types should use this canonical version
// or create type aliases to it.

use crate::canonical::traits::RetryStrategy;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canonical retry configuration for all BearDog operations
///
/// This configuration provides a unified retry strategy that can be used
/// across adapters, discovery, workflows, networking, and other components.
///
/// # Examples
///
/// ```
/// use beardog_types::canonical::config::domains::retry::CanonicalRetryConfig;
/// use std::time::Duration;
///
/// let config = CanonicalRetryConfig {
///     max_attempts: 5,
///     initial_delay: Duration::from_millis(100),
///     max_delay: Duration::from_secs(30),
///     backoff_multiplier: 2.0,
///     enable_exponential_backoff: true,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CanonicalRetryConfig {
    /// Maximum number of retry attempts
    ///
    /// The operation will be attempted at most this many times (including the initial attempt).
    /// A value of 1 means no retries (only the initial attempt).
    /// A value of 0 is invalid and will be treated as 1.
    pub max_attempts: u32,

    /// Initial delay before the first retry
    ///
    /// This is the base delay that will be used for the first retry.
    /// If exponential backoff is enabled, subsequent retries will use
    /// progressively longer delays based on the backoff multiplier.
    pub initial_delay: Duration,

    /// Maximum delay between retry attempts
    ///
    /// This caps the exponential backoff growth to prevent extremely long delays.
    /// Even if exponential backoff would calculate a longer delay, this maximum
    /// will be enforced.
    pub max_delay: Duration,

    /// Backoff multiplier for exponential backoff
    ///
    /// Each retry delay is calculated as: previous_delay * backoff_multiplier
    /// Common values:
    /// - 1.0: Linear backoff (constant delay)
    /// - 2.0: Standard exponential backoff (doubles each time)
    /// - 1.5: Moderate exponential backoff
    pub backoff_multiplier: f64,

    /// Enable exponential backoff strategy
    ///
    /// When true, each retry delay is multiplied by the backoff_multiplier.
    /// When false, all retries use the initial_delay (constant backoff).
    #[serde(default = "default_exponential_backoff")]
    pub enable_exponential_backoff: bool,
}

fn default_exponential_backoff() -> bool {
    true
}

impl Default for CanonicalRetryConfig {
    /// Default retry configuration with conservative settings
    ///
    /// - max_attempts: 3 (initial + 2 retries)
    /// - initial_delay: 100ms
    /// - max_delay: 30 seconds
    /// - backoff_multiplier: 2.0 (exponential)
    /// - exponential_backoff: true
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            enable_exponential_backoff: true,
        }
    }
}

// ============================================================================
// RetryStrategy Trait Implementation
// ============================================================================

impl RetryStrategy for CanonicalRetryConfig {
    fn max_attempts(&self) -> u32 {
        self.max_attempts.max(1) // Ensure at least 1 attempt
    }

    fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return Duration::ZERO;
        }

        if self.enable_exponential_backoff {
            // Calculate exponential backoff: initial_delay * multiplier^(attempt-1)
            let delay_ms = (self.initial_delay.as_millis() as f64
                * self.backoff_multiplier.powi((attempt - 1) as i32)) as u64;

            // Cap at max_delay
            Duration::from_millis(delay_ms.min(self.max_delay.as_millis() as u64))
        } else {
            // Constant backoff: always use initial_delay
            self.initial_delay.min(self.max_delay)
        }
    }

    fn backoff_multiplier(&self) -> f64 {
        self.backoff_multiplier
    }

    // Uses default implementations for:
    // - should_retry_error (retries all errors)
    // - is_limit_reached (checks against max_attempts)
    // - total_delay (sums all delays)
}

impl CanonicalRetryConfig {
    /// Create an aggressive retry configuration
    ///
    /// Suitable for critical operations where you want to retry many times
    /// with short delays.
    pub fn aggressive() -> Self {
        Self {
            max_attempts: 10,
            initial_delay: Duration::from_millis(50),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 1.5,
            enable_exponential_backoff: true,
        }
    }

    /// Create a conservative retry configuration
    ///
    /// Suitable for operations where you want to avoid overloading systems.
    pub fn conservative() -> Self {
        Self {
            max_attempts: 2,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
            backoff_multiplier: 3.0,
            enable_exponential_backoff: true,
        }
    }

    /// Create a no-retry configuration
    ///
    /// The operation will only be attempted once with no retries.
    pub fn no_retry() -> Self {
        Self {
            max_attempts: 1,
            initial_delay: Duration::from_millis(0),
            max_delay: Duration::from_millis(0),
            backoff_multiplier: 1.0,
            enable_exponential_backoff: false,
        }
    }

    /// Calculate the delay for a specific retry attempt
    ///
    /// # Arguments
    /// * `attempt` - The retry attempt number (0 = first retry, 1 = second retry, etc.)
    ///
    /// # Returns
    /// The duration to wait before this retry attempt
    pub fn delay_for_attempt(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return self.initial_delay;
        }

        if !self.enable_exponential_backoff {
            return self.initial_delay.min(self.max_delay);
        }

        let multiplier = self.backoff_multiplier.powi(attempt as i32);
        let delay_ms = self.initial_delay.as_millis() as f64 * multiplier;
        let delay = Duration::from_millis(delay_ms as u64);

        delay.min(self.max_delay)
    }

    /// Validate the retry configuration
    ///
    /// Returns an error message if the configuration is invalid.
    pub fn validate(&self) -> Result<(), String> {
        if self.max_attempts == 0 {
            return Err("max_attempts must be at least 1".to_string());
        }

        if self.backoff_multiplier < 1.0 {
            return Err("backoff_multiplier must be >= 1.0".to_string());
        }

        if self.initial_delay > self.max_delay {
            return Err("initial_delay cannot exceed max_delay".to_string());
        }

        Ok(())
    }
}

/// Type alias for backwards compatibility
pub type RetryConfig = CanonicalRetryConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CanonicalRetryConfig::default();
        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.initial_delay, Duration::from_millis(100));
        assert_eq!(config.max_delay, Duration::from_secs(30));
        assert_eq!(config.backoff_multiplier, 2.0);
        assert!(config.enable_exponential_backoff);
    }

    #[test]
    fn test_delay_calculation() {
        let config = CanonicalRetryConfig::default();

        // First retry: 100ms
        assert_eq!(config.delay_for_attempt(0), Duration::from_millis(100));

        // Second retry: 200ms (100 * 2^1)
        assert_eq!(config.delay_for_attempt(1), Duration::from_millis(200));

        // Third retry: 400ms (100 * 2^2)
        assert_eq!(config.delay_for_attempt(2), Duration::from_millis(400));
    }

    #[test]
    fn test_max_delay_cap() {
        let config = CanonicalRetryConfig {
            max_attempts: 10,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
            enable_exponential_backoff: true,
        };

        // Large attempt number would normally exceed max_delay
        let delay = config.delay_for_attempt(10);
        assert!(delay <= config.max_delay);
        assert_eq!(delay, Duration::from_secs(5));
    }

    #[test]
    fn test_linear_backoff() {
        let config = CanonicalRetryConfig {
            max_attempts: 5,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 1.0,
            enable_exponential_backoff: false,
        };

        // All retries should use initial_delay
        assert_eq!(config.delay_for_attempt(0), Duration::from_millis(100));
        assert_eq!(config.delay_for_attempt(1), Duration::from_millis(100));
        assert_eq!(config.delay_for_attempt(2), Duration::from_millis(100));
    }

    #[test]
    fn test_validation() {
        let mut config = CanonicalRetryConfig::default();

        // Valid config
        assert!(config.validate().is_ok());

        // Invalid: max_attempts = 0
        config.max_attempts = 0;
        assert!(config.validate().is_err());
        config.max_attempts = 3;

        // Invalid: backoff_multiplier < 1.0
        config.backoff_multiplier = 0.5;
        assert!(config.validate().is_err());
        config.backoff_multiplier = 2.0;

        // Invalid: initial_delay > max_delay
        config.initial_delay = Duration::from_secs(60);
        config.max_delay = Duration::from_secs(30);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_presets() {
        let aggressive = CanonicalRetryConfig::aggressive();
        assert_eq!(aggressive.max_attempts, 10);

        let conservative = CanonicalRetryConfig::conservative();
        assert_eq!(conservative.max_attempts, 2);

        let no_retry = CanonicalRetryConfig::no_retry();
        assert_eq!(no_retry.max_attempts, 1);
    }
}

