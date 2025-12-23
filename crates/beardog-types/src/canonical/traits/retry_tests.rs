//! Integration tests for RetryStrategy implementations

#[cfg(test)]
mod tests {
    use crate::canonical::config::domains::retry::CanonicalRetryConfig;
    use crate::canonical::traits::RetryStrategy;
    use std::time::Duration;

    #[test]
    fn test_canonical_retry_config_trait() {
        let config = CanonicalRetryConfig {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            enable_exponential_backoff: true,
        };

        // Test max_attempts
        assert_eq!(config.max_attempts(), 3);

        // Test is_limit_reached
        assert!(!config.is_limit_reached(1));
        assert!(!config.is_limit_reached(2));
        assert!(config.is_limit_reached(3));
        assert!(config.is_limit_reached(4));

        // Test delay_for_attempt with exponential backoff
        assert_eq!(config.delay_for_attempt(1), Duration::from_millis(100)); // 100 * 2^0
        assert_eq!(config.delay_for_attempt(2), Duration::from_millis(200)); // 100 * 2^1
        assert_eq!(config.delay_for_attempt(3), Duration::from_millis(400)); // 100 * 2^2

        // Test backoff_multiplier
        assert_eq!(config.backoff_multiplier(), 2.0);
    }

    #[test]
    fn test_exponential_backoff_capped() {
        let config = CanonicalRetryConfig {
            max_attempts: 10,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
            enable_exponential_backoff: true,
        };

        // First few delays should grow exponentially
        assert_eq!(config.delay_for_attempt(1), Duration::from_secs(1)); // 1 * 2^0
        assert_eq!(config.delay_for_attempt(2), Duration::from_secs(2)); // 1 * 2^1
        assert_eq!(config.delay_for_attempt(3), Duration::from_secs(4)); // 1 * 2^2

        // Should be capped at max_delay
        assert_eq!(config.delay_for_attempt(4), Duration::from_secs(5)); // Would be 8, capped at 5
        assert_eq!(config.delay_for_attempt(5), Duration::from_secs(5)); // Would be 16, capped at 5
    }

    #[test]
    fn test_constant_backoff() {
        let config = CanonicalRetryConfig {
            max_attempts: 5,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0, // Ignored when exponential is disabled
            enable_exponential_backoff: false,
        };

        // All delays should be constant
        assert_eq!(config.delay_for_attempt(1), Duration::from_millis(500));
        assert_eq!(config.delay_for_attempt(2), Duration::from_millis(500));
        assert_eq!(config.delay_for_attempt(3), Duration::from_millis(500));
        assert_eq!(config.delay_for_attempt(4), Duration::from_millis(500));
    }

    #[test]
    fn test_total_delay() {
        let config = CanonicalRetryConfig {
            max_attempts: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
            enable_exponential_backoff: true,
        };

        // Total delay for 1 attempt: 100ms
        assert_eq!(config.total_delay(1), Duration::from_millis(100));

        // Total delay for 2 attempts: 100ms + 200ms = 300ms
        assert_eq!(config.total_delay(2), Duration::from_millis(300));

        // Total delay for 3 attempts: 100ms + 200ms + 400ms = 700ms
        assert_eq!(config.total_delay(3), Duration::from_millis(700));
    }

    #[test]
    fn test_default_config() {
        let config = CanonicalRetryConfig::default();

        // Verify default values work with trait
        assert_eq!(config.max_attempts(), 3);
        assert_eq!(config.backoff_multiplier(), 2.0);
        assert!(!config.is_limit_reached(1));
        assert!(config.is_limit_reached(3));

        // Verify exponential backoff is enabled by default
        let delay1 = config.delay_for_attempt(1);
        let delay2 = config.delay_for_attempt(2);
        assert!(delay2 > delay1); // Should grow exponentially
    }

    #[test]
    fn test_zero_attempt_handling() {
        let config = CanonicalRetryConfig::default();

        // Zero attempt should return zero delay
        assert_eq!(config.delay_for_attempt(0), Duration::ZERO);
    }

    #[test]
    fn test_should_retry_error_default() {
        let config = CanonicalRetryConfig::default();

        // Default implementation retries all errors
        let error = std::io::Error::new(std::io::ErrorKind::Other, "test");
        assert!(config.should_retry_error(&error));
    }

    #[test]
    fn test_polymorphic_usage() {
        // Test that we can use the trait polymorphically
        fn should_continue_retrying<S: RetryStrategy>(strategy: &S, attempt: u32) -> bool {
            !strategy.is_limit_reached(attempt)
        }

        let config = CanonicalRetryConfig::default();

        assert!(should_continue_retrying(&config, 1));
        assert!(should_continue_retrying(&config, 2));
        assert!(!should_continue_retrying(&config, 3));
    }
}

