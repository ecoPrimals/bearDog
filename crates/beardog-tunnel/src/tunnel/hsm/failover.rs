//! # HSM Failover Management Module
//!
//! This module provides failover management functionality for HSM providers, including
//! circuit breakers, retry logic, and fallback provider selection.

use super::{
    HsmFailoverManager, HsmProvider, SecurityRequirements, SoftwareHsmConfig, SoftwareHsmType,
    KeyStoreConfig, KeyStorageType, KeySource, MemoryConfig, CryptoBackend,
};
use beardog_errors::{BearDogError, BearDogResult};
use crate::tunnel::hsm::config::FailoverConfig;
use crate::tunnel::hsm::software_hsm::RustSoftwareHsm;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::warn;

/// Default HSM failover manager implementation
pub struct DefaultHsmFailoverManager {
    circuit_breakers: Arc<RwLock<HashMap<String, CircuitBreaker>>>,
    failover_config: FailoverConfig,
    retry_counts: Arc<RwLock<HashMap<String, u32>>>,
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

/// Circuit breaker states
#[derive(Debug, Clone, PartialEq)]
pub enum CircuitBreakerState {
    Closed,   // Normal operation
    Open,     // Failing - reject requests
    HalfOpen, // Testing - allow limited requests
}

impl DefaultHsmFailoverManager {
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
        let provider_info = provider.get_info().await?;
        let provider_id = format!("{}_{}", provider_info.vendor, provider_info.model);

        warn!(
            "🔄 Handling provider failure: {} - {:?}",
            provider_id, error
        );

        // Update circuit breaker
        {
            let mut circuit_breakers = self.circuit_breakers.write().await;
            let circuit_breaker =
                circuit_breakers
                    .entry(provider_id.clone())
                    .or_insert_with(|| {
                        CircuitBreaker::new(self.failover_config.circuit_breaker_threshold)
                    });

            circuit_breaker.record_failure();
        }

        // Update retry count
        {
            let mut retry_counts = self.retry_counts.write().await;
            let count = retry_counts.entry(provider_id.clone()).or_insert(0);
            *count += 1;
        }

        Ok(())
    }

    async fn get_failover_provider(
        &self,
        _failed_provider: &Arc<dyn HsmProvider>,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<Arc<dyn HsmProvider>> {
        // For now, return a simple software HSM as fallback
        // In a real implementation, this would select the best available alternative

        let software_config = SoftwareHsmConfig {
            implementation: SoftwareHsmType::RustSoftwareHsm,
            key_store_config: KeyStoreConfig {
                storage_type: KeyStorageType::Memory,
                encryption_key_source: KeySource::Derived,
                backup_enabled: false,
                cache_size: 100,
                file_config: None,
                db_config: None,
            },
            memory_config: MemoryConfig::default(),
            crypto_backend: CryptoBackend::RustCrypto,
        };

        let fallback_provider = RustSoftwareHsm::new(software_config).await?;
        Ok(Arc::new(fallback_provider))
    }

    async fn perform_with_failover<T, F>(
        &self,
        operation: F,
        requirements: &SecurityRequirements,
    ) -> BearDogResult<T>
    where
        F: Fn(Arc<dyn HsmProvider>) -> Result<T, BearDogError> + Send + Sync + 'static,
        T: Send + 'static,
    {
        // This is a simplified implementation
        // In a real implementation, this would coordinate with the HSM manager
        Err(BearDogError::UnsupportedOperation {
            operation: "Failover operation".to_string(),
            hsm_type: "Generic".to_string(),
            reason: "Not implemented in this simplified version".to_string(),
        })
    }
}

impl CircuitBreaker {
    pub fn new(threshold: u32) -> Self {
        Self {
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
            threshold,
        }
    }

    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(chrono::Utc::now());

        if self.failure_count >= self.threshold {
            self.state = CircuitBreakerState::Open;
        }
    }

    pub fn record_success(&mut self) {
        self.success_count += 1;

        if self.state == CircuitBreakerState::HalfOpen {
            self.state = CircuitBreakerState::Closed;
            self.failure_count = 0;
        }
    }

    pub fn can_execute(&self) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {
                // Check if enough time has passed to try again
                if let Some(last_failure) = self.last_failure_time {
                    let elapsed = chrono::Utc::now().signed_duration_since(last_failure);
                    elapsed.num_minutes() >= 5 // 5 minute recovery period
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => true,
        }
    }
} 