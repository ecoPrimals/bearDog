//! HSM Failover Management
//!
//! Handles failover logic for HSM providers.

use beardog_errors::BearDogError;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitBreakerState {
    /// Circuit is closed, allowing requests
    Closed,
    /// Circuit is open, blocking requests
    Open,
    /// Circuit is half-open, testing recovery
    HalfOpen,
}

/// Circuit breaker for failover protection
#[derive(Debug)]
pub struct CircuitBreaker {
    /// Current circuit state
    state: CircuitBreakerState,
    /// Failure count
    failure_count: u32,
    /// Failure threshold
    threshold: u32,
    /// Last failure time
    last_failure: Option<std::time::Instant>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker
    pub fn new(threshold: u32) -> Self {
        Self {
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            threshold,
            last_failure: None,
        }
    }

    /// Record a success
    pub fn record_success(&mut self) {
        match self.state {
            CircuitBreakerState::HalfOpen => {
                info!("✅ Circuit breaker: Recovery successful, closing circuit");
                self.state = CircuitBreakerState::Closed;
                self.failure_count = 0;
            }
            CircuitBreakerState::Closed => {
                // Already closed, reset failure count
                self.failure_count = 0;
            }
            CircuitBreakerState::Open => {
                // Ignore success when open
            }
        }
    }

    /// Record a failure
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure = Some(std::time::Instant::now());

        if self.failure_count >= self.threshold {
            warn!("⚠️ Circuit breaker: Threshold reached, opening circuit");
            self.state = CircuitBreakerState::Open;
        }
    }

    /// Check if circuit allows requests
    pub fn allows_request(&mut self) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                // Check if enough time has passed to try half-open
                if let Some(last_failure) = self.last_failure {
                    if last_failure.elapsed().as_secs() > 60 {
                        info!("🔄 Circuit breaker: Attempting half-open state");
                        self.state = CircuitBreakerState::HalfOpen;
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => true,
        }
    }

    /// Get current state
    pub fn state(&self) -> &CircuitBreakerState {
        &self.state
    }
}

/// Failover manager for HSM providers
pub struct FailoverManager {
    /// Circuit breaker
    circuit_breaker: Arc<RwLock<CircuitBreaker>>,
    /// Maximum retry attempts
    max_retries: u32,
}

impl FailoverManager {
    /// Create a new failover manager
    pub fn new(threshold: u32, max_retries: u32) -> Self {
        Self {
            circuit_breaker: Arc::new(RwLock::new(CircuitBreaker::new(threshold))),
            max_retries,
        }
    }

    /// Execute operation with failover
    pub async fn execute_with_failover<T, F, Fut>(&self, operation: F) -> Result<T, BearDogError>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T, BearDogError>>,
    {
        let mut attempts = 0;

        loop {
            // Check circuit breaker
            let mut breaker = self.circuit_breaker.write().await;
            if !breaker.allows_request() {
                return Err(BearDogError::unavailable(
                    "Circuit breaker is open".to_string(),
                ));
            }
            drop(breaker);

            attempts += 1;

            match operation().await {
                Ok(result) => {
                    let mut breaker = self.circuit_breaker.write().await;
                    breaker.record_success();
                    return Ok(result);
                }
                Err(e) => {
                    let mut breaker = self.circuit_breaker.write().await;
                    breaker.record_failure();

                    if attempts >= self.max_retries {
                        warn!("❌ Failover: All {} attempts failed, giving up", attempts);
                        return Err(e);
                    }

                    debug!("🔄 Failover: Attempt {} failed, retrying...", attempts);
                    drop(breaker);

                    // Exponential backoff for retries (modern pattern)
                    let backoff_ms = 100u64 * (1u64 << (attempts - 1).min(4)); // Cap at 1.6 seconds
                    let backoff = std::time::Duration::from_millis(backoff_ms);
                    tokio::time::sleep(backoff).await;
                }
            }
        }
    }

    /// Get circuit breaker state
    pub async fn circuit_state(&self) -> CircuitBreakerState {
        self.circuit_breaker.read().await.state().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circuit_breaker_creation() -> Result<(), Box<dyn std::error::Error>> {
        let breaker = CircuitBreaker::new(3);
        assert_eq!(breaker.state(), &CircuitBreakerState::Closed);
        Ok(())
    }

    #[test]
    fn test_circuit_breaker_failure() -> Result<(), Box<dyn std::error::Error>> {
        let mut breaker = CircuitBreaker::new(3);

        breaker.record_failure();
        assert_eq!(breaker.state(), &CircuitBreakerState::Closed);

        breaker.record_failure();
        assert_eq!(breaker.state(), &CircuitBreakerState::Closed);

        breaker.record_failure();
        assert_eq!(breaker.state(), &CircuitBreakerState::Open);
        Ok(())
    }

    #[test]
    fn test_circuit_breaker_success() -> Result<(), Box<dyn std::error::Error>> {
        let mut breaker = CircuitBreaker::new(3);

        breaker.record_failure();
        breaker.record_failure();
        breaker.record_success();

        assert_eq!(breaker.state(), &CircuitBreakerState::Closed);
        assert_eq!(breaker.failure_count, 0);
        Ok(())
    }

    #[test]
    fn test_circuit_breaker_allows_request() -> Result<(), Box<dyn std::error::Error>> {
        let mut breaker = CircuitBreaker::new(2);

        assert!(breaker.allows_request());

        breaker.record_failure();
        breaker.record_failure();

        assert!(!breaker.allows_request());
        Ok(())
    }

    #[tokio::test]
    async fn test_failover_manager_creation() -> Result<(), Box<dyn std::error::Error>> {
        let manager = FailoverManager::new(3, 3);
        let state = manager.circuit_state().await;
        assert_eq!(state, CircuitBreakerState::Closed);
        Ok(())
    }

    #[tokio::test]
    async fn test_failover_manager_success() -> Result<(), Box<dyn std::error::Error>> {
        let manager = FailoverManager::new(3, 3);

        let result = manager
            .execute_with_failover(|| async { Ok::<i32, BearDogError>(42) })
            .await;

        assert!(result.is_ok());
        assert_eq!(result?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_failover_manager_retry() -> Result<(), Box<dyn std::error::Error>> {
        let manager = FailoverManager::new(5, 3);
        let counter = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));

        let counter_clone = counter.clone();
        let result = manager
            .execute_with_failover(move || {
                let counter = counter_clone.clone();
                async move {
                    let count = counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                    if count < 2 {
                        Err(BearDogError::internal("Simulated failure".to_string()))
                    } else {
                        Ok::<i32, BearDogError>(42)
                    }
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result?, 42);
        Ok(())
    }

    #[tokio::test]
    async fn test_failover_manager_exhaust_retries() -> Result<(), Box<dyn std::error::Error>> {
        let manager = FailoverManager::new(10, 3);

        let result = manager
            .execute_with_failover(|| async {
                Err::<i32, BearDogError>(BearDogError::internal("Always fails".to_string()))
            })
            .await;

        assert!(result.is_err());
        Ok(())
    }
}
