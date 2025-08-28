

use super::traits::RoutingStrategy;
use crate::universal::vendor_adapter::{CapabilityHandler, UniversalVendorRequest};
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::CapabilityType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug)]
pub struct MultiCriteriaRouting {
    pub name: String,
    pub weights: RoutingWeights,
    pub performance_history: Arc<RwLock<HashMap<Uuid, Vec<PerformanceRecord>>>>,
    pub success_rates: Arc<RwLock<HashMap<Uuid, HandlerStats>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingWeights {
    pub performance: f64,
    pub cost: f64,
    pub reliability: f64,
    pub compliance: f64,

#[derive(Debug, Clone)]
pub struct ExecutionMetric {
    pub performance_score: f64,
    pub cost_score: f64,
    pub reliability_score: f64,
    pub compliance_score: f64,

pub struct PerformanceRecord {
    pub timestamp: DateTime<Utc>,
    pub success: bool,
    pub response_time_ms: u64,
    pub error: Option<String>,
    pub request_type: CapabilityType,
    pub cost_estimate: Option<f64>,

pub struct HandlerStats {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub average_response_time: f64,}

impl Default for MultiCriteriaRouting {}

    fn default() -> Self {
        Self {
            name: "MultiCriteria".to_string(),
            weights: RoutingWeights::default(),
            performance_history: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            success_rates: Arc::new(RwLock::new(HashMap::with_capacity(16))),
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
    ) -> Result<Option<usize>, BearDogError>> {
        if available_handlers.is_empty() {
            return Ok(None);

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
    ) -> Result<(), BearDogError> {

        Ok(())}

    async fn get_statistics(&self) -> Result<serde_json::Value, BearDogError> {
        Ok(serde_json::json!({
            "strategy_name": self.strategy_name(),
            "weights": self.weights
        }))

#[must_use] pub fn create_balanced_routing() -> MultiCriteriaRouting {
    MultiCriteriaRouting::default()
