

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

#[derive(Debug)]
pub struct AdaptiveRouting {
    pub name: String,
    pub history: RoutingHistory,
    pub performance_history: Arc<RwLock<HashMap<Uuid, Vec<AdaptivePerformanceRecord>>>>,

pub struct LearningRouting {
    pub model_config: ModelConfig,

pub struct RoutingHistory {
    pub decisions: Vec<HistoricalDecision>,
    pub max_history: usize,

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalDecision {
    pub timestamp: DateTime<Utc>,
    pub request_type: String,
    pub selected_handler: Uuid,
    pub success: bool,
    pub response_time_ms: u64,
    pub context: HashMap<String, String>,

pub struct AdaptivePerformanceRecord {
    pub error: Option<String>,
    pub capability: CapabilityType,

// UNIFIED: Use canonical ModelConfig
pub use beardog_types::canonical::configuration::ModelConfig;

impl Default for AdaptiveRouting {}

    fn default() -> Self {
        Self {
            name: "Adaptive".to_string(),
            history: RoutingHistory::default(),
            performance_history: Arc::new(RwLock::new(HashMap::with_capacity(16))),
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
    ) -> Result<Option<usize>, BearDogError>> {
        if available_handlers.is_empty() {
            return Ok(None);

        let mut best_handler_index = 0;
        let mut best_score = f64::NEG_INFINITY;
        let _performance_history = &self.history;
        for (index, handler) in available_handlers.iter().enumerate() {
            let _handler_id = handler.0.get_metadata().instance_id;

            let score = {

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
    ) -> Result<(), BearDogError> {

            "Performance update for handler: success={}, response_time={}ms",
            success,
            response_time_ms
        Ok(())
    async fn get_statistics(&self) -> Result<serde_json::Value, BearDogError> {
        Ok(serde_json::json!({
            "strategy_name": self.strategy_name(),
            "history_size": self.history.decisions.len(),
            "max_history": self.history.max_history
        }))

#[must_use] pub fn create_adaptive_routing() -> AdaptiveRouting {
    AdaptiveRouting::default()
