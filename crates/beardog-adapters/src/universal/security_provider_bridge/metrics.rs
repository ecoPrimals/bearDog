use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Arc<RwLock<HashMap<String, u64>>>,
}

impl Default for SecurityMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityMetricsCollector {
    /// New operation.
    /// Creates a new instance
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::with_capacity(32))),
        }
    }

    /// Increment Counter operation.
    pub fn increment_counter(&self, metric_name: &str) {
        let mut metrics = self.metrics.write(&str, value: u64) {
        let mut metrics = self.metrics.write();
        metrics.insert(metric_name.to_string(), value);
    }

    /// Get Metric operation.
    /// Gets metric
    /// Gets metric
    pub fn get_metric(&self, metric_name: &str) -> Option<u64> {
        let metrics = self.metrics.read();
        metrics.get(metric_name).copied()
    }

    /// Get All Metrics operation.
    /// Gets all_metrics
    /// Gets all_metrics
    pub fn get_all_metrics(&self) -> HashMap<String, u64> {
        let metrics = self.metrics.read();
        metrics.clone()
    }

    /// Reset Metrics operation.
    pub fn reset_metrics(&self) {
        let mut metrics = self.metrics.write();
        metrics.clear();
    }
}
