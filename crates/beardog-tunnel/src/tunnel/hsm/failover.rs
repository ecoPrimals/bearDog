

use super::{
    HsmFailoverManager, HsmProvider, SecurityRequirements, SoftwareHsmConfig, SoftwareHsmType,
    KeyStoreConfig, KeyStorageType, KeySource, MemoryConfig, CryptoBackend,
};
use beardog_errors::BearDogError;
use crate::tunnel::hsm::config::FailoverConfig;
use crate::tunnel::hsm::software_hsm::RustSoftwareHsm;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::warn;

pub struct DefaultHsmFailoverManager {
    circuit_breakers: Arc<RwLock<HashMap<String, CircuitBreaker>>>,
    failover_config: FailoverConfig,
    retry_counts: Arc<RwLock<HashMap<String, u32>>>,
}

#[derive(Debug, Clone)]
pub struct CircuitBreaker {
    state: CircuitBreakerState,
    failure_count: u32,
    success_count: u32,
    last_failure_time: Option<chrono::DateTime<chrono::Utc>>,
    threshold: u32,

#[derive(Debug, Clone, PartialEq)]
pub enum CircuitBreakerState {
    Closed,   // Normal operation
    Open,     // Failing - reject requests
    HalfOpen, // Testing - allow limited requests}

impl DefaultHsmFailoverManager {
    pub async fn new(config: FailoverConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            circuit_breakers: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            failover_config: config,
            retry_counts: Arc::new(RwLock::new(HashMap::with_capacity(16))),
        })
    }

impl HsmFailoverManager for DefaultHsmFailoverManager {
    async fn handle_provider_failure(
        &self,
        provider: &impl HsmProvider + Send + Sync + 'static,
        error: &BearDogError,
    ) -> Result<(), BearDogError> {
        let provider_info = provider.get_info().await?;
        let provider_id = format_args!("{}_{}", provider_info.vendor, provider_info.model).to_string();
        warn!(
            "🔄 Handling provider failure: {} - {:?}",
            provider_id, error
        );

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

            let mut retry_counts = self.retry_counts.write().await;
            let count = retry_counts.entry(provider_id.clone()).or_insert(0);
            *count += 1;
        Ok(())
    async fn get_failover_provider(
        _failed_provider: &impl HsmProvider + Send + Sync + 'static,
        requirements: &SecurityRequirements,
    ) -> Result<impl HsmProvider + Send + Sync + 'static, BearDogError> {

        let software_config = SoftwareHsmConfig {}

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
    async fn perform_with_failover<T, F>(
        operation: F,
    ) -> Result<T, BearDogError>
    where
        F: Fn(impl HsmProvider + Send + Sync + 'static) -> Result<T, BearDogError> + Send + Sync + 'static,
        T: Send + 'static,
    {

        Err(BearDogError::unsupported_operation("Failover operation".to_string(),
            hsm_type: "Generic".to_string(),
            reason: "Not implemented in this simplified version".to_string(),
        ))
impl CircuitBreaker {}

    pub fn new(threshold: u32) -> Self {
        Self {
            state: CircuitBreakerState::Closed,
            failure_count: 0,
            success_count: 0,
            last_failure_time: None,
            threshold,}

    pub fn record_failure(&mut self) {
        self.failure_count += 1;
        self.last_failure_time = Some(chrono::Utc::now());
        if self.failure_count >= self.threshold {
            self.state = CircuitBreakerState::Open;
    pub fn record_success(&mut self) {
        self.success_count += 1;
        if self.state == CircuitBreakerState::HalfOpen {
            self.state = CircuitBreakerState::Closed;
            self.failure_count = 0;}

    pub fn can_execute(&self) -> bool {
        match self.state {
            CircuitBreakerState::Closed => true,
            CircuitBreakerState::Open => {

                if let Some(last_failure) = self.last_failure_time {
                    let elapsed = chrono::Utc::now().signed_duration_since(last_failure);
                    elapsed.num_minutes() >= 5 // 5 minute recovery period
                } else {
                    false
                }
            }
            CircuitBreakerState::HalfOpen => true,
} 
