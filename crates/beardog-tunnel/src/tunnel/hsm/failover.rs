//! HSM Failover Management
//!
//! This module provides failover capabilities for HSM operations.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

/// Failover manager for HSM operations
pub struct HsmFailoverManager {
    primary_available: bool,
    last_check: Option<Instant>,
    failure_count: u32,
    circuit_breaker: CircuitBreaker,
}

/// Circuit breaker for failover
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    pub max_failures: u32,
    pub timeout: Duration,
    pub state: CircuitBreakerState,
}

/// Circuit breaker state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

impl CircuitBreaker {
    /// Creates a new circuit breaker
    pub fn new(max_failures: u32, timeout: Duration) -> Self {
        Self {
            max_failures,
            timeout,
            state: CircuitBreakerState::Closed,
        }
    }

    /// Records a failure
    pub fn record_failure(&mut self, failure_count: u32) {
        if failure_count >= self.max_failures {
            self.state = CircuitBreakerState::Open;
            warn!("🔴 Circuit breaker opened after {} failures", failure_count);
        }
    }

    /// Attempts to close the circuit
    pub fn attempt_close(&mut self) {
        if self.state == CircuitBreakerState::Open {
            self.state = CircuitBreakerState::HalfOpen;
            debug!("🟡 Circuit breaker half-open, testing");
        }
    }

    /// Resets the circuit breaker
    pub fn reset(&mut self) {
        self.state = CircuitBreakerState::Closed;
        debug!("🟢 Circuit breaker closed");
    }
}

impl HsmFailoverManager {
    /// Creates a new failover manager
    pub fn new() -> Self {
        info!("🔄 Initializing HSM failover manager");
        Self {
            primary_available: true,
            last_check: None,
            failure_count: 0,
            circuit_breaker: CircuitBreaker::new(3, Duration::from_secs(60)),
        }
    }

    /// Executes operation with failover
    pub fn execute_with_failover<F, T>(&mut self, operation: F) -> Result<T, BearDogError>
    where
        F: FnOnce() -> Result<T, BearDogError>,
    {
        match self.circuit_breaker.state {
            CircuitBreakerState::Open => Err(BearDogError::unavailable(
                "Circuit breaker is open - HSM unavailable".to_string(),
            )),
            CircuitBreakerState::Closed | CircuitBreakerState::HalfOpen => match operation() {
                Ok(result) => {
                    self.failure_count = 0;
                    self.circuit_breaker.reset();
                    Ok(result)
                }
                Err(e) => {
                    self.failure_count += 1;
                    self.circuit_breaker.record_failure(self.failure_count);
                    Err(e)
                }
            },
        }
    }

    /// Checks if primary HSM is available
    pub fn is_primary_available(&self) -> bool {
        self.primary_available
    }

    /// Manually sets primary availability
    pub fn set_primary_available(&mut self, available: bool) {
        self.primary_available = available;
        self.last_check = Some(Instant::now());

        if available {
            self.failure_count = 0;
            self.circuit_breaker.reset();
        }
    }
}

impl Default for HsmFailoverManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() -> Result<(), Box<dyn std::error::Error>> {
        let manager = HsmFailoverManager::new();
        assert!(manager.is_primary_available());
        Ok(())
    }

    #[test]
    fn test_circuit_breaker() -> Result<(), Box<dyn std::error::Error>> {
        let mut breaker = CircuitBreaker::new(3, Duration::from_secs(60));
        assert_eq!(breaker.state, CircuitBreakerState::Closed);

        breaker.record_failure(3);
        assert_eq!(breaker.state, CircuitBreakerState::Open);
        Ok(())
    }

    #[test]
    fn test_failover_execution() -> Result<(), Box<dyn std::error::Error>> {
        let mut manager = HsmFailoverManager::new();

        let result = manager.execute_with_failover(|| Ok(42));
        assert!(result.is_ok());
        assert_eq!(result?, 42);
        Ok(())
    }

    #[test]
    fn test_primary_availability() -> Result<(), Box<dyn std::error::Error>> {
        let mut manager = HsmFailoverManager::new();
        assert!(manager.is_primary_available());

        manager.set_primary_available(false);
        assert!(!manager.is_primary_available());
        Ok(())
    }
}
