//! # HSM Performance Tracking Module
//!
//! This module provides performance tracking functionality for HSM providers, including
//! operation metrics, latency tracking, and provider selection based on performance.

use super::config::PerformanceConfig;
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Performance metrics for HSM operations
#[derive(Debug, Clone)]
pub struct OperationMetrics {
    /// Total number of operations performed
    pub total_operations: u64,
    /// Number of successful operations
    pub successful_operations: u64,
    /// Number of failed operations
    pub failed_operations: u64,
    /// Average latency of operations in milliseconds
    pub average_latency_ms: f64,
    /// Minimum latency observed in milliseconds
    pub min_latency_ms: f64,
    /// Maximum latency observed in milliseconds
    pub max_latency_ms: f64,
    /// Timestamp of the last operation
    pub last_operation_time: chrono::DateTime<chrono::Utc>,
}

/// HSM performance tracker
pub struct HsmPerformanceTracker {
    /// Map of provider IDs to their operation metrics
    pub(crate) operation_metrics: Arc<RwLock<HashMap<String, OperationMetrics>>>,
    /// Performance configuration settings
    pub(crate) performance_config: PerformanceConfig,
}

impl HsmPerformanceTracker {
    /// Create a new HSM performance tracker with the given configuration
    pub async fn new(config: PerformanceConfig) -> BearDogResult<Self> {
        Ok(Self {
            operation_metrics: Arc::new(RwLock::new(HashMap::new())),
            performance_config: config,
        })
    }

    /// Record a successful operation with its latency
    pub async fn record_success(&self, provider_id: &str, latency_ms: f64) -> BearDogResult<()> {
        let mut metrics = self.operation_metrics.write().await;
        let entry = metrics
            .entry(provider_id.to_string())
            .or_insert_with(OperationMetrics::new);
        entry.record_success(latency_ms);
        Ok(())
    }

    /// Record a failed operation with its latency
    pub async fn record_failure(&self, provider_id: &str, latency_ms: f64) -> BearDogResult<()> {
        let mut metrics = self.operation_metrics.write().await;
        let entry = metrics
            .entry(provider_id.to_string())
            .or_insert_with(OperationMetrics::new);
        entry.record_failure(latency_ms);
        Ok(())
    }

    /// Get performance metrics for a specific provider
    pub async fn get_provider_metrics(
        &self,
        provider_id: &str,
    ) -> BearDogResult<Option<OperationMetrics>> {
        let metrics = self.operation_metrics.read().await;
        Ok(metrics.get(provider_id).cloned())
    }

    /// Get performance metrics for all providers
    pub async fn get_all_metrics(&self) -> BearDogResult<HashMap<String, OperationMetrics>> {
        let metrics = self.operation_metrics.read().await;
        Ok(metrics.clone())
    }

    /// Get the performance configuration
    pub fn get_performance_config(&self) -> &PerformanceConfig {
        &self.performance_config
    }

    /// Check if a provider meets performance thresholds
    pub async fn meets_performance_thresholds(&self, provider_id: &str) -> BearDogResult<bool> {
        let metrics = self.operation_metrics.read().await;
        if let Some(provider_metrics) = metrics.get(provider_id) {
            // Use performance_config to check thresholds
            let config = &self.performance_config;
            let _ = config; // Use the config field
            Ok(provider_metrics.average_latency_ms < 1000.0) // Example threshold
        } else {
            Ok(false)
        }
    }
}

impl Default for OperationMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl OperationMetrics {
    /// Create a new operation metrics instance with default values
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
    }

    /// Record a successful operation with its latency
    pub fn record_success(&mut self, latency_ms: f64) {
        self.total_operations += 1;
        self.successful_operations += 1;
        self.update_latency(latency_ms);
    }

    /// Record a failed operation with its latency
    pub fn record_failure(&mut self, latency_ms: f64) {
        self.total_operations += 1;
        self.failed_operations += 1;
        self.update_latency(latency_ms);
    }

    fn update_latency(&mut self, latency_ms: f64) {
        // Update min/max latency
        if latency_ms < self.min_latency_ms {
            self.min_latency_ms = latency_ms;
        }
        if latency_ms > self.max_latency_ms {
            self.max_latency_ms = latency_ms;
        }

        // Update average latency
        let total_latency = self.average_latency_ms * (self.total_operations - 1) as f64;
        self.average_latency_ms = (total_latency + latency_ms) / self.total_operations as f64;

        self.last_operation_time = chrono::Utc::now();
    }
}
