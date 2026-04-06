// SPDX-License-Identifier: AGPL-3.0-or-later

//! Retry strategy trait interface
//!
//! Provides a common interface for different retry implementations,
//! enabling generic retry logic across the codebase.

use std::error::Error;
use std::time::Duration;

/// Trait for retry strategy configuration
///
/// Provides a common interface for different retry implementations
/// while allowing domain-specific customization.
///
/// # Example
///
/// ```rust,no_run
/// use beardog_types::canonical::traits::RetryStrategy;
/// use beardog_types::canonical::config::domains::retry::CanonicalRetryConfig;
/// use std::time::Duration;
///
/// let config = CanonicalRetryConfig::default();
///
/// // Check retry limit
/// if !config.is_limit_reached(3) {
///     let delay = config.delay_for_attempt(3);
///     println!("Retry after {:?}", delay);
/// }
/// ```
pub trait RetryStrategy: Send + Sync {
    /// Maximum number of retry attempts
    ///
    /// Returns the maximum number of times an operation should be retried
    /// before giving up.
    fn max_attempts(&self) -> u32;

    /// Calculate delay for a specific attempt
    ///
    /// # Arguments
    ///
    /// * `attempt` - The attempt number (1-indexed). First retry is attempt 1.
    ///
    /// # Returns
    ///
    /// The duration to wait before the next retry attempt.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog_types::canonical::traits::RetryStrategy;
    /// # use beardog_types::canonical::config::domains::retry::CanonicalRetryConfig;
    /// # let strategy = CanonicalRetryConfig::default();
    ///
    /// let delay1 = strategy.delay_for_attempt(1); // First retry
    /// let delay2 = strategy.delay_for_attempt(2); // Second retry
    /// // delay2 might be longer than delay1 for exponential backoff
    /// ```
    fn delay_for_attempt(&self, attempt: u32) -> Duration;

    /// Determine if an error should trigger a retry
    ///
    /// # Arguments
    ///
    /// * `error` - The error that occurred
    ///
    /// # Returns
    ///
    /// `true` if the error is retryable, `false` otherwise.
    ///
    /// # Default Implementation
    ///
    /// By default, all errors are considered retryable. Implementations
    /// can override this to provide more sophisticated error filtering.
    fn should_retry_error(&self, _error: &(dyn Error + Send + Sync)) -> bool {
        true // Retry all errors by default
    }

    /// Get backoff multiplier for exponential backoff
    ///
    /// # Returns
    ///
    /// The multiplier applied to delays for exponential backoff.
    /// Default is 2.0 (delay doubles each attempt).
    fn backoff_multiplier(&self) -> f64 {
        2.0 // Default exponential backoff
    }

    /// Check if retry limit has been reached
    ///
    /// # Arguments
    ///
    /// * `attempt` - The current attempt number (1-indexed)
    ///
    /// # Returns
    ///
    /// `true` if no more retries should be attempted, `false` otherwise.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use beardog_types::canonical::traits::RetryStrategy;
    /// # use beardog_types::canonical::config::domains::retry::CanonicalRetryConfig;
    /// # let strategy = CanonicalRetryConfig::default();
    ///
    /// let mut attempt = 1;
    /// while !strategy.is_limit_reached(attempt) {
    ///     // Try operation
    ///     attempt += 1;
    /// }
    /// ```
    fn is_limit_reached(&self, attempt: u32) -> bool {
        attempt >= self.max_attempts()
    }

    /// Calculate total delay for all attempts up to given attempt number
    ///
    /// Useful for timeout calculations and logging.
    ///
    /// # Arguments
    ///
    /// * `attempt` - The attempt number to calculate total delay for
    ///
    /// # Returns
    ///
    /// The total time spent waiting for all retries up to this attempt.
    fn total_delay(&self, attempt: u32) -> Duration {
        (1..=attempt).map(|n| self.delay_for_attempt(n)).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Simple test implementation
    struct TestRetryStrategy {
        max_attempts: u32,
        base_delay: Duration,
    }

    impl RetryStrategy for TestRetryStrategy {
        fn max_attempts(&self) -> u32 {
            self.max_attempts
        }

        fn delay_for_attempt(&self, _attempt: u32) -> Duration {
            self.base_delay
        }
    }

    #[test]
    fn test_is_limit_reached() {
        let strategy = TestRetryStrategy {
            max_attempts: 3,
            base_delay: Duration::from_secs(1),
        };

        assert!(!strategy.is_limit_reached(1));
        assert!(!strategy.is_limit_reached(2));
        assert!(strategy.is_limit_reached(3));
        assert!(strategy.is_limit_reached(4));
    }

    #[test]
    fn test_total_delay() {
        let strategy = TestRetryStrategy {
            max_attempts: 3,
            base_delay: Duration::from_secs(1),
        };

        assert_eq!(strategy.total_delay(1), Duration::from_secs(1));
        assert_eq!(strategy.total_delay(2), Duration::from_secs(2));
        assert_eq!(strategy.total_delay(3), Duration::from_secs(3));
    }

    #[test]
    fn test_default_should_retry_error() {
        let strategy = TestRetryStrategy {
            max_attempts: 3,
            base_delay: Duration::from_secs(1),
        };

        let error = std::io::Error::other("test");
        assert!(strategy.should_retry_error(&error));
    }

    #[test]
    fn test_default_backoff_multiplier() {
        let strategy = TestRetryStrategy {
            max_attempts: 3,
            base_delay: Duration::from_secs(1),
        };

        assert_eq!(strategy.backoff_multiplier(), 2.0);
    }
}
