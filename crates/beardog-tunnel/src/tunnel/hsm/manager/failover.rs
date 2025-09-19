

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::config::FailoverConfig;
use super::{HsmFailoverManager, HsmProvider, SecurityRequirements};
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
    failure_count: u32,
    success_count: u32,
    last_failure_time: Option<chrono::DateTime<chrono::Utc>>,
    threshold: u32,

pub struct DefaultHsmFailoverManager {
    pub(Arc<RwLock<HashMap<String, CircuitBreaker>>>,
    pub(FailoverConfig,
    pub(Arc<RwLock<HashMap<String, u32>>>,}

impl DefaultHsmFailoverManager {

/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub async fn new(config: FailoverConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            circuit_breakers: Arc::new(RwLock::new(HashMap::with_capacity(config,
            retry_counts: Arc::new(RwLock::new(HashMap::with_capacity(&impl HsmProvider + Send + Sync + 'static,
        error: &BearDogError,
    ) -> Result<(), BearDogError> {

        let provider_info = provider.get_info()?;
        let provider_id = provider_info.vendor;

        let mut circuit_breakers = self.circuit_breakers.write();
        let circuit_breaker = circuit_breakers
            .entry(provider_id)
            .or_insert_with(|| CircuitBreaker::new(self.failover_config.circuit_breaker_threshold));
        circuit_breaker.record_failure();

        let mut retry_counts = self.retry_counts.write();
        let retry_count = retry_counts.entry(provider_id).or_insert(0);
        *retry_count += 1;
        tracing::warn!(
            "HSM provider {} failed: {:?} (retry count: {})",
            provider_id,
            error,
            retry_count
        );
        Ok(&impl HsmProvider + Send + Sync + 'static,
        requirements: &SecurityRequirements,
    ) -> Result<impl HsmProvider + Send + Sync + 'static, BearDogError> {

        Err(BearDogError::no_suitable_provider({}requirements:?"},
    fn perform_with_failover<T, F>(
        _operation: F,
    ) -> Result<T, BearDogError>
    where
        F: Fn(Send + 'static,
    {

        Err(BearDogError::no_suitable_provider(format!("No suitable provider found for requirements: {}requirements:?"),}

impl CircuitBreaker {

/// New operation.
    /// Creates a new instance
    pub fn new(threshold: u32) -> Self {
        Self {
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
            threshold,
        }

/// Record Failure operation.
    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(chrono::Utc::now());
        if self.failure_count >= self.threshold {
            self.state = CircuitBreakerState::Open;

/// Record Success operation.
    pub fn record_success(&mut self) {
        self.success_count += 1;
        self.failure_count = 0;
        if self.state == CircuitBreakerState::HalfOpen {
            self.state = CircuitBreakerState::Closed;

/// Can Execute operation.
    pub fn can_execute(&self) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {

                if let Some(last_failure) = self.last_failure_time {
                    let elapsed = chrono::Utc::now() - last_failure;
                    elapsed > chrono::Duration::seconds(30) // Timeout after 30 seconds
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => true,
