

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


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
    /// Number of total_requests
    pub total_requests: u64,
    /// Number of successful_requests
    pub successful_requests: u64,
    pub total_response_time_ms: u64,
    /// The last updated value
    pub last_updated: std::time::Instant,
    /// The recent requests value
    pub recent_requests: std::collections::VecDeque<(std::time::Instant, bool, u64)>,
    /// Optional recent success rate
    pub recent_success_rate: Option<f64>,
    /// The error patterns value
    pub error_patterns: std::collections::HashMap<String, u32>,
}

#[derive(Debug, Clone)]
    /// The history value
    pub history: RoutingHistory,
    pub performance_history: Arc<RwLock<HashMap<Uuid, Vec<AdaptivePerformanceRecord>>>>,

pub struct LearningRouting {
    pub model_config: ModelConfig,

pub struct RoutingHistory {
    /// Collection of decisions
    pub decisions: Vec<HistoricalDecision>,
    /// Number of max_history
    pub max_history: usize,

#[derive(Debug, Clone)]
    /// The request type value
    pub request_type: String,
    /// The selected handler value
    pub selected_handler: Uuid,
    /// Whether success is enabled
    pub success: bool,
    pub response_time_ms: u64,
    /// Mapping of context
    pub context: HashMap<String, String>,

pub struct AdaptivePerformanceRecord {
    /// Optional error
    pub error: Option<String>,
    /// The capability value
    pub capability: CapabilityType,

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
            decisions: Vec::new(1000,}

impl RoutingStrategy for AdaptiveRouting {}


    fn strategy_name(&UniversalVendorRequest,
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
                .get_metadata(Uuid,
        success: bool,
        response_time_ms: u64,
        _error: Option<&str>,
    ) -> Result<(), BearDogError> {

            "Performance update for handler: success={}, response_time={}ms",
            success,
            response_time_ms
        Ok(())
    /// Gets statistics
    fn get_statistics(&self) -> Result<serde_json::Value, BearDogError> {
        Ok(serde_json::json!({
            "strategy_name": self.strategy_name(),
            "history_size": self.history.decisions.len(),
            "max_history": self.history.max_history
        }))

/// Create Adaptive Routing operation.
#[must_use] pub fn create_adaptive_routing() -> AdaptiveRouting {
    AdaptiveRouting::default()
