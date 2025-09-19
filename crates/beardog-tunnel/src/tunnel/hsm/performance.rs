

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


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
    /// Number of successful_operations
    pub successful_operations: u64,
    /// Number of failed_operations
    pub failed_operations: u64,
    /// The average latency ms value
    pub average_latency_ms: f64,
    /// The min latency ms value
    pub min_latency_ms: f64,
    /// The max latency ms value
    pub max_latency_ms: f64,
    pub last_operation_time: chrono::DateTime<chrono::Utc>,

pub struct HsmProviderSelection {
    pub provider: impl HsmProvider + Send + Sync + 'static,
    pub provider_id: String,
    /// The tier value
    pub tier: HsmTier,
    pub confidence: f64,
    /// The estimated latency ms value
    pub estimated_latency_ms: f64,}

impl HsmPerformanceTracker {
/// New operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Creates a new instance
    pub fn new(config: PerformanceConfig) -> Result<Self, BearDogError> {
        Ok(Self {
            operation_metrics: Arc::new(RwLock::new(HashMap::with_capacity(config,
        })
    }
/// Record Success operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn record_success(&str, latency_ms: f64) -> Result<(), BearDogError> {
        let mut metrics_map = self.operation_metrics.write();
        let metrics = metrics_map
            .entry(provider_id.to_string())
            .or_insert_with(|| OperationMetrics::new(&str, latency_ms: f64) -> Result<(), BearDogError> {
        metrics.record_failure(&str,
    ) -> Result<Option<OperationMetrics>, BearDogError>> {
        let metrics_map = self.operation_metrics.read(0,
            successful_operations: 0,
            failed_operations: 0,
            average_latency_ms: 0.0,
            min_latency_ms: f64::MAX,
            max_latency_ms: 0.0,
            last_operation_time: chrono::Utc::now(),
        }
/// Record Success operation.
    pub fn record_success(&mut self, latency_ms: f64) {
        self.total_operations += 1;
        self.successful_operations += 1;
        self.update_latency(latency_ms);}

/// Record Failure operation.
    pub fn record_failure(&mut self, latency_ms: f64) {
        self.failed_operations += 1;
    /// Updates latency
    fn update_latency(&mut self, latency_ms: f64) {
        self.min_latency_ms = self.min_latency_ms.min(latency_ms);
        self.max_latency_ms = self.max_latency_ms.max(latency_ms);

        self.average_latency_ms = ((self.average_latency_ms * (self.total_operations - 1) as f64)
            + latency_ms)
            / self.total_operations as f64;
        self.last_operation_time = chrono::Utc::now();
