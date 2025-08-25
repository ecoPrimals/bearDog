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


/// Adaptive and Learning-Based Routing Strategies
///
/// **EXTRACTED FROM**: strategies.rs (950 lines → focused module)
/// Contains routing strategies that learn and adapt based on historical performance.

use super::traits::RoutingStrategy;
use crate::universal::vendor_adapter::{CapabilityHandler, UniversalVendorRequest};
use beardog_errors::BearDogResult;
use beardog_types::canonical::capabilities::CapabilityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
/// Performance metrics for a handler
#[derive(Debug, Clone)]
pub struct HandlerPerformanceMetrics {
    pub handler_id: String,
    pub total_requests: u64,
    pub successful_requests: u64,
    pub total_response_time_ms: u64,
    pub last_updated: std::time::Instant,
    pub recent_requests: std::collections::VecDeque<(std::time::Instant, bool, u64)>,
    pub recent_success_rate: Option<f64>,
    pub error_patterns: std::collections::HashMap<String, u32>,
}
/// **ADAPTIVE ROUTING** - Learn from patterns and adapt
#[derive(Debug)]
pub struct AdaptiveRouting {
    pub name: String,
    pub history: RoutingHistory,
    pub performance_history: Arc<RwLock<HashMap<Uuid, Vec<AdaptivePerformanceRecord>>>>,
/// **LEARNING ROUTING** - Machine learning based routing
pub struct LearningRouting {
    pub model_config: ModelConfig,
/// Routing history for adaptive learning
pub struct RoutingHistory {
    pub decisions: Vec<HistoricalDecision>,
    pub max_history: usize,
/// Historical routing decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalDecision {
    pub timestamp: DateTime<Utc>,
    pub request_type: String,
    pub selected_handler: Uuid,
    pub success: bool,
    pub response_time_ms: u64,
    pub context: HashMap<String, String>,
/// Performance record for adaptive routing
pub struct AdaptivePerformanceRecord {
    pub error: Option<String>,
    pub capability: CapabilityType,
/// Machine learning model configuration
pub struct ModelConfig {
    pub model_type: String,
    pub learning_rate: f64,
    pub features: Vec<String>,}


impl Default for AdaptiveRouting {}


    fn default() -> Self {
        Self {
            name: "Adaptive".to_string(),
            history: RoutingHistory::default(),
            performance_history: Arc::new(RwLock::new(HashMap::new())),
        }
    }
impl Default for RoutingHistory {
            decisions: Vec::new(),
            max_history: 1000,}



impl RoutingStrategy for AdaptiveRouting {}


    fn strategy_name(&self) -> &str {
        &self.name}


    async fn select_handler(
        &self,
        _request: &UniversalVendorRequest,
        available_handlers: &[(Box<dyn CapabilityHandler>, f64)],
    ) -> BearDogResult<Option<usize>> {
        if available_handlers.is_empty() {
            return Ok(None);
        // Adaptive learning logic: Select handler based on historical performance
        let mut best_handler_index = 0;
        let mut best_score = f64::NEG_INFINITY;
        let _performance_history = &self.history;
        for (index, handler) in available_handlers.iter().enumerate() {
            let _handler_id = handler.0.get_metadata().instance_id;
            // Calculate adaptive score based on historical performance
            let score = {
                // Adaptive scoring would be implemented here based on:
                // - Historical success rates
                // - Response time patterns
                // - Recent performance trends
                // - Learning algorithm adjustments
                // For now, use a simple scoring based on handler index
                0.5 + (index as f64 * 0.1) % 1.0
            };
            if score > best_score {
                best_score = score;
                best_handler_index = index;
            }
        tracing::debug!(
            "Adaptive routing selected handler {} with score {:.3}",
            available_handlers[best_handler_index]
                .0
                .get_metadata()
                .instance_id,
            best_score
        );
        Ok(Some(best_handler_index))
    async fn update_with_result(
        _handler_id: Uuid,
        success: bool,
        response_time_ms: u64,
        _error: Option<&str>,
    ) -> BearDogResult<()> {
        // Performance history updates would be implemented here
        // This would track handler performance and adjust routing decisions accordingly
            "Performance update for handler: success={}, response_time={}ms",
            success,
            response_time_ms
        Ok(())
    async fn get_statistics(&self) -> BearDogResult<serde_json::Value> {
        Ok(serde_json::json!({
            "strategy_name": self.strategy_name(),
            "history_size": self.history.decisions.len(),
            "max_history": self.history.max_history
        }))
/// Create an adaptive routing strategy
#[must_use] pub fn create_adaptive_routing() -> AdaptiveRouting {
    AdaptiveRouting::default()
