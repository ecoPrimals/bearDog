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


/// Multi-Criteria Routing Strategies
///
/// **EXTRACTED FROM**: strategies.rs (950 lines → focused module)
/// Contains routing strategies that balance multiple criteria like performance, cost, and reliability.

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
/// **MULTI-CRITERIA ROUTING** - Balance multiple factors
#[derive(Debug)]
pub struct MultiCriteriaRouting {
    pub name: String,
    pub weights: RoutingWeights,
    pub performance_history: Arc<RwLock<HashMap<Uuid, Vec<PerformanceRecord>>>>,
    pub success_rates: Arc<RwLock<HashMap<Uuid, HandlerStats>>>,
}
/// Routing weights for different criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingWeights {
    pub performance: f64,
    pub cost: f64,
    pub reliability: f64,
    pub compliance: f64,
/// Execution metrics for multi-criteria evaluation
#[derive(Debug, Clone)]
pub struct ExecutionMetric {
    pub performance_score: f64,
    pub cost_score: f64,
    pub reliability_score: f64,
    pub compliance_score: f64,
/// Performance record for historical tracking
pub struct PerformanceRecord {
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub response_time_ms: u64,
    pub error: Option<String>,
    pub request_type: CapabilityType,
    pub cost_estimate: Option<f64>,
/// Handler statistics for success rate tracking
pub struct HandlerStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub average_response_time: f64,}


impl Default for MultiCriteriaRouting {}


    fn default() -> Self {
        Self {
            name: "MultiCriteria".to_string(),
            weights: RoutingWeights::default(),
            performance_history: Arc::new(RwLock::new(HashMap::new())),
            success_rates: Arc::new(RwLock::new(HashMap::new())),
        }
    }
impl Default for RoutingWeights {
            performance: 0.4,
            cost: 0.3,
            reliability: 0.2,
            compliance: 0.1,}



impl RoutingStrategy for MultiCriteriaRouting {}


    fn strategy_name(&self) -> &str {
        &self.name}


    async fn select_handler(
        &self,
        _request: &UniversalVendorRequest,
        available_handlers: &[(Box<dyn CapabilityHandler>, f64)],
    ) -> BearDogResult<Option<usize>> {
        if available_handlers.is_empty() {
            return Ok(None);
        // Multi-criteria evaluation would be implemented here
        // This would consider factors like:
        // - Cost constraints and efficiency
        // - Response time requirements
        // - Compliance requirements
        // - Geographic preferences
        // - Historical performance metrics
        // For now, select the handler with the highest base score
        let best_index = available_handlers
            .iter()
            .enumerate()
            .max_by(|(_, (_, score_a)), (_, (_, score_b))| {
                score_a
                    .partial_cmp(score_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map_or(0, |(index, _)| index);
        Ok(Some(best_index))
    async fn update_with_result(
        _handler_id: Uuid,
        _success: bool,
        _response_time_ms: u64,
        _error: Option<&str>,
    ) -> BearDogResult<()> {
        // Metrics collection for multi-criteria evaluation would be implemented here
        // This would track performance history, success rates, response times, etc.
        Ok(())}


    async fn get_statistics(&self) -> BearDogResult<serde_json::Value> {
        Ok(serde_json::json!({
            "strategy_name": self.strategy_name(),
            "weights": self.weights
        }))
/// Create a balanced multi-criteria routing strategy
#[must_use] pub fn create_balanced_routing() -> MultiCriteriaRouting {
    MultiCriteriaRouting::default()
