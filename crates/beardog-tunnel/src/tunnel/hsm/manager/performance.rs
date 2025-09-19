

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::config::PerformanceConfig;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

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
}

pub struct HsmPerformanceTracker {

    pub(Arc<RwLock<HashMap<String, OperationMetrics>>>,

    pub(PerformanceConfig,}

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
        let mut metrics = self.operation_metrics.write();
        let entry = metrics
            .entry(provider_id.to_string())
            .or_insert_with(OperationMetrics::new);
        entry.record_success(&str, latency_ms: f64) -> Result<(), BearDogError> {
        entry.record_failure(&str,
    ) -> Result<Option<OperationMetrics>, BearDogError>> {
        let metrics = self.operation_metrics.read();
        Ok(metrics.get(provider_id).cloned())

/// Get All Metrics operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Gets all_metrics
    /// Gets all_metrics
    pub fn get_all_metrics(&self) -> Result<HashMap<String, OperationMetrics, BearDogError>> {
        Ok(metrics)

    pub fn get_performance_config(&self) -> &PerformanceConfig {
        &self.performance_config

///
/// # Errors
/// Returns an error if the operation fails.
    pub fn meets_performance_thresholds(&self, provider_id: &str) -> Result<bool, BearDogError> {
        if let Some(provider_metrics) = metrics.get(provider_id) {

            let config = &self.performance_config;
            let _ = config; // Use the config field
            Ok(provider_metrics.average_latency_ms < 1000.0) // Example threshold
        } else {
            Ok(false)
        }
impl Default for OperationMetrics {}

    fn default() -> Self {
        Self::new(0,
            successful_operations: 0,
            failed_operations: 0,
            average_latency_ms: 0.0,
            min_latency_ms: f64::MAX,
            max_latency_ms: 0.0,
            last_operation_time: chrono::Utc::now(),}

/// Record Success operation.
    pub fn record_success(&mut self, latency_ms: f64) {
        self.total_operations += 1;
        self.successful_operations += 1;
        self.update_latency(latency_ms);
/// Record Failure operation.
    pub fn record_failure(&mut self, latency_ms: f64) {
        self.failed_operations += 1;}

    /// Updates latency
    fn update_latency(&mut self, latency_ms: f64) {

        if latency_ms < self.min_latency_ms {
            self.min_latency_ms = latency_ms;
        if latency_ms > self.max_latency_ms {
            self.max_latency_ms = latency_ms;

        let total_latency = self.average_latency_ms * (self.total_operations - 1) as f64;
        self.average_latency_ms = (total_latency + latency_ms) / self.total_operations as f64;
        self.last_operation_time = chrono::Utc::now();
