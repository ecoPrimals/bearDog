// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use std::collections::HashMap;
use std::fmt::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::RwLock;
// Removed unused imports: use chrono::{DateTime, Utc};
use crate::monitoring::types::MetricValue;
use beardog_errors::BearDogError;
use beardog_types::canonical::HealthStatus;

/// `MetricsCollector` provides comprehensive metrics collection functionality
#[derive(Debug)]
pub struct MetricsCollector {
    metrics: Arc<RwLock<HashMap<String, MetricValue>>>,
    counter: AtomicUsize,
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsCollector {
    /// Creates a new `MetricsCollector` instance
    #[must_use]
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
            counter: AtomicUsize::new(0),
        }
    }

    /// Records a counter metric value
    ///
    /// # Errors
    /// Returns an error if the metrics storage cannot be accessed
    pub async fn record_counter(&self, name: &str, value: u64) -> Result<(), BearDogError> {
        self.metrics
            .write().await
            .await
            .insert(name.to_string(), MetricValue::Counter(value));
        self.counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    /// Records a gauge metric value
    ///
    /// # Errors
    /// Returns an error if the metrics storage cannot be accessed
    pub async fn record_gauge(&self, name: &str, value: f64) -> Result<(), BearDogError> {
        self.metrics
            .write().await
            .await
            .insert(name.to_string(), MetricValue::Gauge(value));
        self.counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    ///
    /// # Errors
    /// Returns an error if the metrics storage cannot be accessed
    pub async fn record_histogram(&self, name: &str, values: Vec<f64>) -> Result<(), BearDogError> {
        self.metrics
            .write().await
            .await
            .insert(name.to_string(), MetricValue::Histogram(values));
        self.counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    /// Records a timer metric value
    ///
    /// # Errors
    /// Returns an error if the metrics storage cannot be accessed
    pub fn record_timer(
        &self,
        name: &str,
        duration: std::time::Duration,
    ) -> Result<(), BearDogError> {
        self.metrics
            .write().await
            .insert(name.to_string(), MetricValue::Timer(duration));
        self.counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    /// Gets all current metrics
    ///
    /// # Errors
    /// Returns an error if the metrics storage cannot be accessed
    /// Gets all_metrics
    /// Gets all_metrics
    pub fn get_all_metrics(&self) -> Result<HashMap<String, MetricValue>, BearDogError> {
        Ok(self.metrics.read().clone())
    }

    /// Gets the total number of recorded metrics
    #[must_use]
    /// Gets metric_count
    /// Gets metric_count
    pub fn get_metric_count(&self) -> usize {
        self.counter.load(Ordering::Relaxed)
    }

    /// Clears all stored metrics
    ///
    /// # Errors
    /// Returns an error if the metrics storage cannot be accessed
    pub fn clear_metrics(&self) -> Result<(), BearDogError> {
        self.metrics.write().clear();
        self.counter.store(0, Ordering::Relaxed);
        Ok(())
    }
}

// PrometheusConfig is imported from types module to avoid duplication
pub use super::types::PrometheusConfig;

/// `PrometheusExporter` provides Prometheus metrics export functionality
#[derive(Debug)]
pub struct PrometheusExporter {
    config: PrometheusConfig,
    collector: Arc<MetricsCollector>,
}

impl PrometheusExporter {
    /// Creates a new `PrometheusExporter`
    #[must_use]
    pub const fn new(config: PrometheusConfig, collector: Arc<MetricsCollector>) -> Self {
        Self { config, collector }
    }

    ///
    /// # Errors
    pub fn export_metrics(&self) -> Result<String, BearDogError> {
        let metrics = self.collector.get_all_metrics()?;
        let mut output = String::new();

        for (name, value) in &metrics {
            match value {
                MetricValue::Counter(val) => {
                    writeln!(output, "beardog_counter_{name} {val}").map_err(|e| {
                        BearDogError::system(format!("Failed to write counter metric: {e}"))
                    })?;
                }
                MetricValue::Gauge(val) => {
                    writeln!(output, "beardog_gauge_{name} {val}").map_err(|e| {
                        BearDogError::system(format!("Failed to write gauge metric: {e}"))
                    })?;
                }
                MetricValue::Histogram(vals) => {
                    let avg = if vals.is_empty() {
                        0.0
                    } else {
                        #[allow(clippy::cast_precision_loss)]
                        let len_f64 = vals.len() as f64;
                        vals.iter().sum::<f64>() / len_f64
                    };
                    writeln!(output, "beardog_histogram_{name}_avg {avg}").map_err(|e| {
                        BearDogError::system(format!("Failed to write histogram metric: {e}"))
                    })?;
                }
                MetricValue::Timer(duration) => {
                    writeln!(output, "beardog_timer_{name}_ms {}", duration.as_millis()).map_err(
                        |e| BearDogError::system(format!("Failed to write timer metric: {e}")),
                    )?;
                }
            }
        }

        Ok(output)
    }

    /// Gets the health status of the Prometheus exporter
    #[must_use]
    pub const fn get_health_status(&self) -> HealthStatus {
        if self.config.enabled {
            HealthStatus::Healthy
        } else {
            HealthStatus::Unhealthy
        }
    }
}
