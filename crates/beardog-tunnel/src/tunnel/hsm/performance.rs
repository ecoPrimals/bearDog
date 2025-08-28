

use super::types::{HsmCapabilities, HsmHealthStatus, HsmTier, SystemMetrics};
use super::HsmProvider;
use crate::tunnel::hsm::config::PerformanceConfig;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

pub struct HsmPerformanceTracker {
    operation_metrics: Arc<RwLock<HashMap<String, OperationMetrics>>>,
    performance_config: PerformanceConfig,
}

#[derive(Debug, Clone)]
pub struct OperationMetrics {
    pub total_operations: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub average_latency_ms: f64,
    pub min_latency_ms: f64,
    pub max_latency_ms: f64,
    pub last_operation_time: chrono::DateTime<chrono::Utc>,

#[derive(Clone)]
pub struct HsmProviderSelection {
    pub provider: impl HsmProvider + Send + Sync + 'static,
    pub provider_id: String,
    pub tier: HsmTier,
    pub confidence: f64,
    pub estimated_latency_ms: f64,}

impl HsmPerformanceTracker {
    pub async fn new(config: PerformanceConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            operation_metrics: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            performance_config: config,
        })
    }
    pub async fn record_success(&self, provider_id: &str, latency_ms: f64) -> Result<(), BearDogError> {
        let mut metrics_map = self.operation_metrics.write().await;
        let metrics = metrics_map
            .entry(provider_id.to_string())
            .or_insert_with(|| OperationMetrics::new());
        metrics.record_success(latency_ms);
        Ok(())}

    pub async fn record_failure(&self, provider_id: &str, latency_ms: f64) -> Result<(), BearDogError> {
        metrics.record_failure(latency_ms);
    pub async fn get_provider_metrics(
        &self,
        provider_id: &str,
    ) -> Result<Option<OperationMetrics>, BearDogError>> {
        let metrics_map = self.operation_metrics.read().await;
        Ok(metrics_map.get(provider_id).cloned())}

    pub async fn get_all_metrics(&self) -> Result<HashMap<String, OperationMetrics, BearDogError>> {
        Ok(metrics_map.clone())
impl OperationMetrics {}

    pub fn new() -> Self {
        Self {
            total_operations: 0,
            successful_operations: 0,
            failed_operations: 0,
            average_latency_ms: 0.0,
            min_latency_ms: f64::MAX,
            max_latency_ms: 0.0,
            last_operation_time: chrono::Utc::now(),
        }
    pub fn record_success(&mut self, latency_ms: f64) {
        self.total_operations += 1;
        self.successful_operations += 1;
        self.update_latency(latency_ms);}

    pub fn record_failure(&mut self, latency_ms: f64) {
        self.failed_operations += 1;
    fn update_latency(&mut self, latency_ms: f64) {
        self.min_latency_ms = self.min_latency_ms.min(latency_ms);
        self.max_latency_ms = self.max_latency_ms.max(latency_ms);

        self.average_latency_ms = ((self.average_latency_ms * (self.total_operations - 1) as f64)
            + latency_ms)
            / self.total_operations as f64;
        self.last_operation_time = chrono::Utc::now();
