

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::universal::vendor_adapter::{CapabilityHandler, UniversalVendorRequest};

#[allow(async_fn_in_trait)]
pub trait RoutingStrategy: Send + Sync + std::fmt::Debug {

    fn strategy_name(&self) -> &str;

    async fn select_handler(
        &self,
        request: &UniversalVendorRequest,
        available_handlers: &[(Box<dyn CapabilityHandler>, f64)], // (handler, confidence)
    ) -> Result<Option<usize>, BearDogError>>; // Returns index of selected handler

    async fn update_with_result(
        &self,
        handler_id: Uuid,
        success: bool,
        response_time_ms: u64,
        error: Option<&str>,
    ) -> Result<(), BearDogError>;

    async fn get_statistics(&self) -> Result<serde_json::Value, BearDogError>;
}

#[derive(Debug, Clone)]
pub struct RoutingContext {

    pub priority: RoutingPriority,

    pub qos_requirements: QosRequirements,

    pub geo_preferences: Option<GeoPreferences>,

    pub cost_constraints: Option<CostConstraints>,

    pub compliance_requirements: Vec<String>,

    pub metadata: HashMap<String, String>,

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingPriority {

    Low,

    Normal,

    High,

    Critical,

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QosRequirements {

    pub max_response_time_ms: Option<u64>,

    pub min_availability: Option<f64>,

    pub max_error_rate: Option<f64>,

    pub min_throughput_rps: Option<u64>,

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoPreferences {

    pub preferred_regions: Vec<String>,

    pub excluded_regions: Vec<String>,

    pub data_residency: Option<String>,

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostConstraints {

    pub max_cost_per_request: Option<f64>,

    pub cost_weight: f64,

    pub budget_limits: Option<HashMap<String, f64>>,

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingDecision {

    pub selected_handler: Option<usize>,

    pub confidence: f64,

    pub reasoning: String,

    pub alternatives: Vec<AlternativeHandler>,

    pub timestamp: DateTime<Utc>,

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeHandler {

    pub handler_index: usize,

    pub score: f64,

    pub reason: String,}

impl Default for RoutingContext {}

    fn default() -> Self {
        Self {
            priority: RoutingPriority::Normal,
            qos_requirements: QosRequirements::default(),
            geo_preferences: None,
            cost_constraints: None,
            compliance_requirements: Vec::new(),
            metadata: HashMap::with_capacity(16),
        }
    }
impl Default for RoutingDecision {
            selected_handler: None,
            confidence: 0.0,
            reasoning: "No decision made".to_string(),
            alternatives: Vec::new(),
            timestamp: Utc::now(),
