// SPDX-License-Identifier: AGPL-3.0-only

//! Failover Tests

use super::failover::*;
use beardog_errors::BearDogError;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(test)]
#[allow(clippy::module_inception)]
mod failover_tests {
    use super::*;

    /// Test circuit breaker creation
    #[test]
    fn test_circuit_breaker_creation() {
        let breaker = CircuitBreaker::new(3);
        assert!(matches!(breaker.state(), CircuitBreakerState::Closed));
    }

    /// Test circuit breaker allows request when closed
    #[test]
    fn test_circuit_breaker_allows_request_when_closed() {
        let mut breaker = CircuitBreaker::new(3);
        assert!(breaker.allows_request());
    }

    /// Test circuit breaker opens after threshold failures
    #[test]
    fn test_circuit_breaker_opens_after_threshold() {
        let mut breaker = CircuitBreaker::new(3);

        // Record failures up to threshold
        breaker.record_failure();
        assert!(breaker.allows_request());

        breaker.record_failure();
        assert!(breaker.allows_request());

        breaker.record_failure();
        // Should now be open
        assert!(!breaker.allows_request());
        assert!(matches!(breaker.state(), CircuitBreakerState::Open));
    }

    /// Test circuit breaker records success
    #[test]
    fn test_circuit_breaker_records_success() {
        let mut breaker = CircuitBreaker::new(3);

        breaker.record_failure();
        breaker.record_success();

        // Should still be closed
        assert!(breaker.allows_request());
    }

    /// Test circuit breaker state
    #[test]
    fn test_circuit_breaker_state() {
        let breaker = CircuitBreaker::new(3);
        let state = breaker.state();
        assert!(matches!(state, CircuitBreakerState::Closed));
    }

    /// Test failover manager creation
    #[test]
    fn test_failover_manager_creation() {
        let _manager = FailoverManager::new(3, 5);
        // Test passes if manager creation succeeds
    }

    /// Test successful operation with failover
    #[tokio::test]
    async fn test_execute_with_failover_success() {
        let manager = FailoverManager::new(3, 5);

        let result = manager
            .execute_with_failover(|| async { Ok::<i32, BearDogError>(42) })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    /// Test failover retries on failure
    #[tokio::test]
    async fn test_execute_with_failover_retries() {
        let manager = FailoverManager::new(5, 3);
        let attempt = Arc::new(AtomicU32::new(0));

        let attempt_clone = attempt.clone();
        let result = manager
            .execute_with_failover(|| {
                let attempt = attempt_clone.clone();
                async move {
                    let current = attempt.fetch_add(1, Ordering::SeqCst);
                    if current < 2 {
                        // Fail first 2 attempts
                        Err(BearDogError::unavailable("Temporary failure".to_string()))
                    } else {
                        // Succeed on 3rd attempt
                        Ok(42)
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempt.load(Ordering::SeqCst), 3);
    }

    /// Test failover gives up after max retries
    #[tokio::test]
    async fn test_execute_with_failover_gives_up() {
        let manager = FailoverManager::new(5, 3);

        let result = manager
            .execute_with_failover(|| async {
                Err::<i32, BearDogError>(BearDogError::unavailable("Always fails".to_string()))
            })
            .await;

        assert!(result.is_err());
    }

    /// Test circuit breaker opens during failover
    #[tokio::test]
    async fn test_circuit_breaker_opens_during_failover() {
        let manager = FailoverManager::new(2, 5);

        // First operation - should fail and open circuit breaker
        let _result1 = manager
            .execute_with_failover(|| async {
                Err::<i32, BearDogError>(BearDogError::unavailable("Failure".to_string()))
            })
            .await;

        let state = manager.circuit_state().await;
        // Circuit should be open after failures
        assert!(matches!(state, CircuitBreakerState::Open));
    }

    /// Test circuit breaker state is accessible
    #[tokio::test]
    async fn test_circuit_state() {
        let manager = FailoverManager::new(3, 5);

        let state = manager.circuit_state().await;
        assert!(matches!(state, CircuitBreakerState::Closed));
    }

    /// Test failover with zero retries
    #[tokio::test]
    async fn test_failover_zero_retries() {
        let manager = FailoverManager::new(5, 1); // Only 1 attempt

        let result = manager
            .execute_with_failover(|| async {
                Err::<i32, BearDogError>(BearDogError::unavailable("Failure".to_string()))
            })
            .await;

        assert!(result.is_err());
    }

    /// Test concurrent failover operations
    #[tokio::test]
    async fn test_concurrent_failover_operations() {
        let manager = Arc::new(FailoverManager::new(10, 3));

        let mut handles = vec![];

        for i in 0..5 {
            let manager_clone = manager.clone();
            let handle = tokio::spawn(async move {
                manager_clone
                    .execute_with_failover(|| async { Ok::<i32, BearDogError>(i) })
                    .await
            });
            handles.push(handle);
        }

        let mut results = vec![];
        for handle in handles {
            let result = handle.await.unwrap();
            results.push(result);
        }

        // All should succeed
        assert_eq!(results.iter().filter(|r| r.is_ok()).count(), 5);
    }

    /// Test failover delay between retries
    #[tokio::test]
    async fn test_failover_retry_delay() {
        let manager = FailoverManager::new(5, 3);
        let attempt = Arc::new(AtomicU32::new(0));

        let start = std::time::Instant::now();

        let attempt_clone = attempt.clone();
        let _result = manager
            .execute_with_failover(|| {
                let attempt = attempt_clone.clone();
                async move {
                    let current = attempt.fetch_add(1, Ordering::SeqCst);
                    if current < 2 {
                        Err(BearDogError::unavailable("Failure".to_string()))
                    } else {
                        Ok(42)
                    }
                }
            })
            .await;

        let duration = start.elapsed();

        // Should have some delay between retries (100ms + 200ms = 300ms minimum)
        assert!(duration.as_millis() >= 200);
    }

    /// Test circuit breaker state transitions
    #[test]
    fn test_circuit_breaker_state_transitions() {
        let mut breaker = CircuitBreaker::new(2);

        // Start closed
        assert!(matches!(breaker.state(), CircuitBreakerState::Closed));

        // Success in closed state resets counter
        breaker.record_success();
        assert!(matches!(breaker.state(), CircuitBreakerState::Closed));

        // After failures, should open
        breaker.record_failure();
        breaker.record_failure();
        assert!(matches!(breaker.state(), CircuitBreakerState::Open));

        // Success when open doesn't close it (needs to be HalfOpen first)
        breaker.record_success();
        assert!(matches!(breaker.state(), CircuitBreakerState::Open));
    }

    /// Test multiple sequential operations
    #[tokio::test]
    async fn test_multiple_sequential_operations() {
        let manager = FailoverManager::new(5, 3);

        for i in 0..3 {
            let result = manager
                .execute_with_failover(|| async move { Ok::<i32, BearDogError>(i) })
                .await;

            assert!(result.is_ok());
        }
    }
}
