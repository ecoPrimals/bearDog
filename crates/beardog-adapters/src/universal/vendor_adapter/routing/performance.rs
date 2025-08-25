// MODERNIZED: Removed async_trait - now uses native async fn in trait

// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Performance-Based Routing Strategies
///
/// **EXTRACTED FROM**: strategies.rs (950 lines → focused module)
/// Contains routing strategies that prioritize performance metrics and response times.

use super::traits::{AlternativeHandler, RoutingStrategy};
use crate::universal::vendor_adapter::{CapabilityHandler, UniversalVendorRequest};
use beardog_errors::{BearDogError, BearDogResult};
use chrono::{DateTime, Utc};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use uuid::Uuid;
// CANONICAL IMPORT: use beardog_types::config::UnifiedPerformanceConfig;
/// **PERFORMANCE FIRST ROUTING** - Select based on fastest response time
#[derive(Debug)]
pub struct PerformanceFirstRouting {
    pub name: String,
    pub historical_data: Arc<RwLock<HashMap<Uuid, Vec<PerformanceMetric>>>>,
    pub window_size: usize,
    pub min_samples: usize,
}
/// Performance metric for tracking handler performance
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


    #[must_use] pub fn new(name: String) -> Self {
        Self {
            name,
            historical_data: Arc::new(RwLock::new(HashMap::new())),
            window_size: 100, // Keep last 100 measurements
            min_samples: 5,   // Need at least 5 samples for reliable metrics
        }
    #[must_use] pub fn with_config(name: String, window_size: usize, min_samples: usize) -> Self {
            window_size,
            min_samples,
    /// Calculate average response time for a handler}


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
    /// Calculate success rate for a handler
    fn calculate_success_rate(&self, handler_id: Uuid) -> f64 {
        let Ok(data) = self.historical_data.read() else {
            return 0.0; // Default to 0% success rate if lock is poisoned
        };
            if !metrics.is_empty() {
                let successful = metrics.iter().filter(|m| m.success).count();
                return successful as f64 / metrics.len() as f64;
        0.0
    /// Get performance score combining response time and success rate
    fn get_performance_score(&self, handler_id: Uuid) -> f64 {
        let avg_response_time = self.calculate_avg_response_time(handler_id);
        let success_rate = self.calculate_success_rate(handler_id);
        match avg_response_time {
            Some(response_time) => {
                // Score = success_rate / (response_time_in_seconds + 1)
                // Higher success rate and lower response time = higher score
                let response_time_seconds = response_time / 1000.0;
                success_rate / (response_time_seconds + 1.0)
            None => {
                // No performance data - give neutral score
                0.5
#[async_trait]}


impl RoutingStrategy for PerformanceFirstRouting {}


    fn strategy_name(&self) -> &str {
        &self.name}


    async fn select_handler(
        &self,
        _request: &UniversalVendorRequest,
        available_handlers: &[(Box<dyn CapabilityHandler>, f64)],
    ) -> BearDogResult<Option<usize>> {
        if available_handlers.is_empty() {
            return Ok(None);
        let mut best_index = 0;
        let mut best_score = -1.0;
        let mut alternatives = Vec::new();
        // Evaluate each handler
        for (index, (_handler, confidence)) in available_handlers.iter().enumerate() {
            // Get handler ID for performance lookup
            let handler_id = Uuid::new_v4(); // In real implementation, get from handler metadata
            let performance_score = self.get_performance_score(handler_id);
            let success_rate = self.calculate_success_rate(handler_id);
            // Combined score: performance * confidence * success_rate
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
        // Sort alternatives by score (highest first)
        alternatives.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(Some(best_index))
    async fn update_with_result(
        handler_id: Uuid,
        success: bool,
        response_time_ms: u64,
        error: Option<&str>,
    ) -> BearDogResult<()> {
        let metric = PerformanceMetric {
            timestamp: Utc::now(),
            response_time_ms,
            success,
            error: error.map(|s| s.to_string()),
        let mut data = self
            .historical_data
            .write()
            .map_err(|_| BearDogError::internal("Failed to acquire write lock on historical data".to_string(),
            ))?;
        let handler_metrics = data.entry(handler_id).or_insert_with(Vec::new);
        handler_metrics.push(metric);
        // Keep only the most recent metrics within window size
        if handler_metrics.len() > self.window_size {
            handler_metrics.drain(0..handler_metrics.len() - self.window_size);
        Ok(())}


    async fn get_statistics(&self) -> BearDogResult<serde_json::Value> {
        let data = self
            .read()
                message: "Failed to acquire read lock on historical data".to_string(),
        let mut stats = HashMap::new();
        stats.insert("strategy_name", json!(self.strategy_name()));
        stats.insert("window_size", json!(self.window_size));
        stats.insert("min_samples", json!(self.min_samples));
        stats.insert("tracked_handlers", json!(data.len()));
        let mut handler_stats = HashMap::new();
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
        stats.insert("handlers", json!(handler_stats));
        Ok(json!(stats))
/// Create a performance-first routing strategy
#[must_use] pub fn create_performance_routing() -> PerformanceFirstRouting {
    PerformanceFirstRouting::new("PerformanceOptimized".to_string())
/// Create a performance routing strategy with custom configuration
#[must_use] pub fn create_custom_performance_routing(
    name: String,
    window_size: usize,
    min_samples: usize,
) -> PerformanceFirstRouting {
    PerformanceFirstRouting::with_config(name, window_size, min_samples)
/// Performance routing configuration
// MIGRATED: PerformanceRoutingConfig -> use beardog_types::config::UnifiedPerformanceConfig;


impl Default for PerformanceRoutingConfig {
            name: "PerformanceRouting".to_string(),
            window_size: 100,
            min_samples: 5,
            response_time_weight: 0.6,
            success_rate_weight: 0.4,}


impl From<PerformanceRoutingConfig> for PerformanceFirstRouting {
    fn from(config: PerformanceRoutingConfig) -> Self {
        Self::with_config(config.name, config.window_size, config.min_samples)
