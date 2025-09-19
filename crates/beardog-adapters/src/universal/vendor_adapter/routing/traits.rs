

use beardog_errors::BearDogError;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use crate::universal::vendor_adapter::{CapabilityHandler, UniversalVendorRequest};

#[allow(async_fn_in_trait)]
pub trait RoutingStrategy: Send + Sync + std::fmt::Debug {


    fn strategy_name(&UniversalVendorRequest,
        available_handlers: &[(Box<dyn CapabilityHandler>, f64)], // (handler, confidence)
    ) -> Result<Option<usize>, BearDogError>>; // Returns index of selected handler

    /// Updates with_result
    fn update_with_result(Uuid,
        success: bool,
        response_time_ms: u64,
        error: Option<&str>,
    ) -> Result<(), BearDogError>;

    /// Gets statistics
    fn get_statistics(&self) -> Result<serde_json::Value, BearDogError>;
}

#[derive(Debug, Clone)]
    /// The qos requirements value
    pub qos_requirements: QosRequirements,

    /// Optional geo preferences
    pub geo_preferences: Option<GeoPreferences>,

    /// Optional cost constraints
    pub cost_constraints: Option<CostConstraints>,

    /// Collection of compliance requirements
    pub compliance_requirements: Vec<String>,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,

#[derive(Debug, Clone)]
    /// Optional min availability
    pub min_availability: Option<f64>,

    /// Optional max error rate
    pub max_error_rate: Option<f64>,

    /// Optional min throughput rps
    pub min_throughput_rps: Option<u64>,

#[derive(Debug, Clone)]
    /// Collection of excluded regions
    pub excluded_regions: Vec<String>,


    pub data_residency: Option<String>,

#[derive(Debug, Clone)]
    /// The cost weight value
    pub cost_weight: f64,

    /// Optional budget limits
    pub budget_limits: Option<HashMap<String, f64>>,

#[derive(Debug, Clone)]
    pub confidence: f64,

    /// The reasoning value
    pub reasoning: String,

    /// Collection of alternatives
    pub alternatives: Vec<AlternativeHandler>,


    pub timestamp: DateTime<Utc>,

#[derive(Debug, Clone)]
    /// The score value
    pub score: f64,

    /// The reason value
    pub reason: String,}

impl Default for RoutingContext {}

    fn default(RoutingPriority::Normal,
            qos_requirements: QosRequirements::default(None,
            cost_constraints: None,
            compliance_requirements: Vec::new(),
            metadata: HashMap::with_capacity(None,
            confidence: 0.0,
            reasoning: "No decision made".to_string(),
            alternatives: Vec::new(),
            timestamp: Utc::now(),
