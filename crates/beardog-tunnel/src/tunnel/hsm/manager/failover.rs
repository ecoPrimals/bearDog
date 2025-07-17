//! # HSM Failover Management
//!
//! This module provides failover capabilities for HSM providers,
//! including circuit breaker patterns and automatic retry logic.

use super::config::FailoverConfig;
use super::{HsmFailoverManager, HsmProvider, SecurityRequirements};
use beardog_errors::{BearDogError, BearDogResult};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Circuit breaker state for HSM provider failover
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitBreakerState {
    /// Normal operation - requests are processed
    Closed,   // Normal operation
    /// Failing state - requests are rejected
    Open,     // Failing - reject requests
    /// Testing state - limited requests are allowed
    HalfOpen, // Testing - allow limited requests
}

/// Circuit breaker for HSM providers
#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    state: CircuitBreakerState,
    failure_count: u32,
    success_count: u32,
    last_failure_time: Option<chrono::DateTime<chrono::Utc>>,
    threshold: u32,
}

/// Default HSM failover manager
pub struct DefaultHsmFailoverManager {
    pub(crate) circuit_breakers: Arc<RwLock<HashMap<String, CircuitBreaker>>>,
    pub(crate) failover_config: FailoverConfig,
    pub(crate) retry_counts: Arc<RwLock<HashMap<String, u32>>>,
}

impl DefaultHsmFailoverManager {
    /// Create a new HSM failover manager with the specified configuration
    pub async fn new(config: FailoverConfig) -> BearDogResult<Self> {
        Ok(Self {
            circuit_breakers: Arc::new(RwLock::new(HashMap::new())),
            failover_config: config,
            retry_counts: Arc::new(RwLock::new(HashMap::new())),
        })
    }
}

#[async_trait]
impl HsmFailoverManager for DefaultHsmFailoverManager {
    async fn handle_provider_failure(
        &self,
        provider: &Arc<dyn HsmProvider>,
        error: &BearDogError,
    ) -> BearDogResult<()> {
        // Get provider info to get ID
        let provider_info = provider.get_info().await?;
        let provider_id = provider_info.vendor;

        // Update circuit breaker
        let mut circuit_breakers = self.circuit_breakers.write().await;
        let circuit_breaker = circuit_breakers
            .entry(provider_id.clone())
            .or_insert_with(|| CircuitBreaker::new(self.failover_config.circuit_breaker_threshold));

        circuit_breaker.record_failure();

        // Update retry count
        let mut retry_counts = self.retry_counts.write().await;
        let retry_count = retry_counts.entry(provider_id.clone()).or_insert(0);
        *retry_count += 1;

        tracing::warn!(
            "HSM provider {} failed: {:?} (retry count: {})",
            provider_id,
            error,
            retry_count
        );

        Ok(())
    }

    async fn get_failover_provider(
        &self,
        _failed_provider: &Arc<dyn HsmProvider>,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<Arc<dyn HsmProvider>> {
        // For now, return an error - in a real implementation,
        // we would have a list of backup providers
        Err(BearDogError::NoSuitableProvider {
            requirements: format!("{requirements:?}"),
        })
    }

    async fn perform_with_failover<T, F>(
        &self,
        _operation: F,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<T>
    where
        F: Fn(Arc<dyn HsmProvider>) -> Result<T, BearDogError> + Send + Sync + 'static,
        T: Send + 'static,
    {
        // Simple failover - in a real implementation, we would try multiple providers
        Err(BearDogError::NoSuitableProvider {
            requirements: format!("{requirements:?}"),
        })
    }
}

impl CircuitBreaker {
    /// Create a new circuit breaker with the specified failure threshold
    pub fn new(threshold: u32) -> Self {
        Self {
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
            threshold,
        }
    }

    /// Record a failure and potentially open the circuit
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(chrono::Utc::now());

        if self.failure_count >= self.threshold {
            self.state = CircuitBreakerState::Open;
        }
    }

    /// Record a success and potentially close the circuit
    pub fn record_success(&mut self) {
        self.success_count += 1;
        self.failure_count = 0;

        if self.state == CircuitBreakerState::HalfOpen {
            self.state = CircuitBreakerState::Closed;
        }
    }

    /// Check if the circuit allows execution
    pub fn can_execute(&self) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                // Check if we should try half-open
                if let Some(last_failure) = self.last_failure_time {
                    let elapsed = chrono::Utc::now() - last_failure;
                    elapsed > chrono::Duration::seconds(30) // Timeout after 30 seconds
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => true,
        }
    }
}
