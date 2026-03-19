// SPDX-License-Identifier: AGPL-3.0-only

//! Handler Recovery Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: core/handlers/recovery
//! `TEST_PRIORITY`: critical


#![allow(unused_imports, clippy::float_cmp, clippy::useless_vec, clippy::needless_range_loop, clippy::uninlined_format_args, clippy::field_reassign_with_default, clippy::manual_range_contains, unused_variables, dead_code)]

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 8: Handler Error Recovery
    ///
    /// Tests error recovery in handlers:
    /// - Retry mechanisms
    /// - Fallback handlers
    /// - Error transformation
    /// - Circuit breaker pattern
    #[test]
    fn test_handler_error_recovery() {
        // Test retry mechanism
        let retry_handler = RetryHandler::new(3); // Max 3 retries

        let attempt_count = Arc::new(AtomicUsize::new(0));
        let attempt_clone = attempt_count.clone();

        let result = retry_handler.execute(move || {
            let attempt = attempt_clone.fetch_add(1, Ordering::SeqCst);
            if attempt < 2 {
                Err(BearDogError::system("temporary error".to_string()))
            } else {
                Ok(42)
            }
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempt_count.load(Ordering::SeqCst), 3); // Failed twice, succeeded third time

        // Test max retries exceeded
        let fail_count = Arc::new(AtomicUsize::new(0));
        let fail_clone = fail_count.clone();

        let fail_result = retry_handler.execute(move || {
            fail_clone.fetch_add(1, Ordering::SeqCst);
            Err::<i32, _>(BearDogError::system("persistent error".to_string()))
        });

        assert!(fail_result.is_err());
        assert_eq!(fail_count.load(Ordering::SeqCst), 4); // Initial + 3 retries

        // Test fallback handler
        let fallback_handler = FallbackHandler::new();

        let primary_called = Arc::new(AtomicBool::new(false));
        let fallback_called = Arc::new(AtomicBool::new(false));

        let p_called = primary_called.clone();
        let f_called = fallback_called.clone();

        let result = fallback_handler.execute_with_fallback(
            move || {
                p_called.store(true, Ordering::SeqCst);
                Err::<i32, _>(BearDogError::system("primary failed".to_string()))
            },
            move || {
                f_called.store(true, Ordering::SeqCst);
                Ok(99)
            },
        );

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 99);
        assert!(primary_called.load(Ordering::SeqCst));
        assert!(fallback_called.load(Ordering::SeqCst));

        // Test circuit breaker
        let circuit_breaker = CircuitBreaker::new(3, Duration::from_millis(100));

        // Trigger failures to open circuit
        for _ in 0..3 {
            let _ = circuit_breaker
                .execute(|| Err::<i32, _>(BearDogError::system("error".to_string())));
        }

        assert!(circuit_breaker.is_open());

        // Circuit is open, should fail fast
        let fast_fail_result = circuit_breaker.execute(|| Ok(42));

        assert!(fast_fail_result.is_err());
        assert!(fast_fail_result
            .unwrap_err()
            .to_string()
            .contains("Circuit breaker"));

        // ✅ MODERNIZED: Use manual circuit state transition instead of sleep
        // Circuit breaker should have a method to force half-open state for testing
        circuit_breaker.transition_to_half_open(); // Test circuit recovery mechanism

        // Try again, should allow in half-open state
        let recovery_result = circuit_breaker.execute(|| Ok(42));

        assert!(recovery_result.is_ok());
        assert_eq!(recovery_result.unwrap(), 42);
        assert!(circuit_breaker.is_closed());

        // Test error transformation
        let transform_handler = TransformingHandler::new();

        let result = transform_handler.execute_with_transform(
            || Err::<i32, _>(BearDogError::system("original error".to_string())),
            |error: BearDogError| BearDogError::security(format!("Transformed: {}", error)),
        );

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Transformed"));
        assert!(err.to_string().contains("original error"));
    }
}
