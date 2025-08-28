

use super::traits::{AlternativeHandler, RoutingStrategy};
use crate::universal::vendor_adapter::{CapabilityHandler, UniversalVendorRequest};
use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Debug)]
pub struct PerformanceFirstRouting {
    pub name: String,
    pub historical_data: Arc<RwLock<HashMap<Uuid, Vec<PerformanceMetric>>>>,
    pub window_size: usize,
    pub min_samples: usize,
}

#[derive(Debug, Clone)]
pub struct PerformanceMetric {
    pub timestamp: DateTime<Utc>,
    pub response_time_ms: u64,
    pub success: bool,
    pub error: Option<String>,}

impl Default for PerformanceFirstRouting {}

    fn default() -> Self {
        Self::new("PerformanceFirst".to_string())
    }
impl PerformanceFirstRouting {}

    #[must_use] pub fn new(name: &str) -> Self {
        Self {
            name,
            historical_data: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            window_size: 100, // Keep last 100 measurements
            min_samples: 5,   // Need at least 5 samples for reliable metrics
        }
    #[must_use] pub fn with_config(name: &str, window_size: usize, min_samples: usize) -> Self {
            window_size,
            min_samples,

    fn calculate_avg_response_time(&self, handler_id: Uuid) -> Option<f64> {
        let data = self.historical_data.read().ok()?;
        if let Some(metrics) = data.get(&handler_id) {
            if metrics.len() >= self.min_samples {
                let successful_metrics: Vec<_> = metrics.iter().filter(|m| m.success).collect();
                if !successful_metrics.is_empty() {
                    let sum: u64 = successful_metrics.iter().map(|m| m.response_time_ms).sum();
                    return Some(sum as f64 / successful_metrics.len() as f64);
                }
            }
        None

    fn calculate_success_rate(&self, handler_id: Uuid) -> f64 {
        let Ok(data) = self.historical_data.read() else {
            return 0.0; // Default to 0% success rate if lock is poisoned
        };
            if !metrics.is_empty() {
                let successful = metrics.iter().filter(|m| m.success).count();
                return successful as f64 / metrics.len() as f64;
        0.0

    fn get_performance_score(&self, handler_id: Uuid) -> f64 {
        let avg_response_time = self.calculate_avg_response_time(handler_id);
        let success_rate = self.calculate_success_rate(handler_id);
        match avg_response_time {
            Some(response_time) => {

                let response_time_seconds = response_time / 1000.0;
                success_rate / (response_time_seconds + 1.0)
            None => {

                0.5
            }
        }
    }
}

#[allow(async_fn_in_trait)]
impl RoutingStrategy for PerformanceFirstRouting {
    fn strategy_name(&self) -> &str {
        &self.name
    }

    async fn select_handler(
        &self,
        _request: &UniversalVendorRequest,
        available_handlers: &[(Box<dyn CapabilityHandler>, f64)],
    ) -> Result<Option<usize>, BearDogError>> {
        if available_handlers.is_empty() {
            return Ok(None);
        let mut best_index = 0;
        let mut best_score = -1.0;
        let mut alternatives = Vec::new();

        for (index, (_handler, confidence)) in available_handlers.iter().enumerate() {

            let handler_id = Uuid::new_v4(); // In real implementation, get from handler metadata
            let performance_score = self.get_performance_score(handler_id);
            let success_rate = self.calculate_success_rate(handler_id);

            let combined_score = performance_score * confidence * success_rate;
            alternatives.push(AlternativeHandler {
                handler_index: index,
                score: combined_score,
                reason: format!(
                    "Performance: {performance_score:.3}, Confidence: {confidence:.3}, Success Rate: {success_rate:.3}"
                ),
            });
            if combined_score > best_score {
                best_score = combined_score;
                best_index = index;

        alternatives.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(Some(best_index))
    }

    async fn update_with_result(
        &self,
        handler_id: Uuid,
        success: bool,
        response_time_ms: u64,
        error: Option<&str>,
    ) -> Result<(), BearDogError> {
        let metric = PerformanceMetric {
            timestamp: Utc::now(),
            response_time_ms,
            success,
            error: error.map(|s| s.to_string()),
        };
        
        let mut data = self
            .historical_data
            .write()
            .map_err(|_| BearDogError::internal("Failed to acquire write lock on historical data".to_string()))?;
        
        let handler_metrics = data.entry(handler_id).or_insert_with(Vec::new);
        handler_metrics.push(metric);

        if handler_metrics.len() > self.window_size {
            handler_metrics.drain(0..handler_metrics.len() - self.window_size);
        }
        
        Ok(())
    }

    async fn get_statistics(&self) -> Result<serde_json::Value, BearDogError> {
        let data = self
            .historical_data
            .read()
            .map_err(|_| BearDogError::internal("Failed to acquire read lock on historical data".to_string()))?;
        
        let mut stats = HashMap::with_capacity(16);
        stats.insert("strategy_name", json!(self.strategy_name()));
        stats.insert("window_size", json!(self.window_size));
        stats.insert("min_samples", json!(self.min_samples));
        stats.insert("tracked_handlers", json!(data.len()));
        
        let mut handler_stats = HashMap::with_capacity(16);
        for (handler_id, metrics) in data.iter() {
            let avg_response_time = self.calculate_avg_response_time(*handler_id);
            let success_rate = self.calculate_success_rate(*handler_id);
            let performance_score = self.get_performance_score(*handler_id);
            
            handler_stats.insert(
                handler_id.to_string(),
                json!({
                    "sample_count": metrics.len(),
                    "avg_response_time_ms": avg_response_time,
                    "success_rate": success_rate,
                    "performance_score": performance_score,
                    "last_updated": metrics.last().map(|m| m.timestamp)
                }),
            );
        }
        
        stats.insert("handlers", json!(handler_stats));
        Ok(json!(stats))
    }
}

#[must_use] pub fn create_performance_routing() -> PerformanceFirstRouting {
    PerformanceFirstRouting::new("PerformanceOptimized".to_string())

#[must_use] pub fn create_custom_performance_routing(
    name: &str,
    window_size: usize,
    min_samples: usize,
) -> PerformanceFirstRouting {
    PerformanceFirstRouting::with_config(name, window_size, min_samples)

impl Default for PerformanceRoutingConfig {
            name: "PerformanceRouting".to_string(),
            window_size: 100,
            min_samples: 5,
            response_time_weight: 0.6,
            success_rate_weight: 0.4,}

impl From<PerformanceRoutingConfig> for PerformanceFirstRouting {
    fn from(config: PerformanceRoutingConfig) -> Self {
        Self::with_config(config.name, config.window_size, config.min_samples)
