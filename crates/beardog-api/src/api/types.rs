

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub use beardog_types::constants::unified::api::VERSION as API_VERSION;
pub use beardog_types::constants::unified::api::VERSION as API_VERSION_HEADER;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {

    pub page: Option<u32>,

    pub per_page: Option<u32>,

    pub sort_by: Option<String>,

    pub sort_order: Option<String>,
}
impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(20),
            sort_by: None,
            sort_order: Some("asc".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterParams {

    pub start_date: Option<String>,

    pub end_date: Option<String>,

    pub status: Option<String>,

    pub category: Option<String>,

    pub search: Option<String>,

    pub filters: Option<HashMap<String, String>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {

    pub request_id: String,

    pub processing_time_ms: u64,

    pub timestamp: String,

    pub api_version: String,

    pub cached: bool,}

impl ResponseMetadata {

    pub fn new(request_id: &str, processing_time_ms: u64, cached: bool) -> Self {
            request_id: request_id.to_string(),
            processing_time_ms,
            timestamp: chrono::Utc::now().to_rfc3339(),
            api_version: API_VERSION.to_string(),
            cached,

// UNIFIED: Use canonical HealthStatus from beardog-types
pub use beardog_types::canonical::HealthStatus;
    Healthy,

    Degraded,

    Unhealthy,

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {

    pub name: String,

    pub status: HealthStatus,

    pub health_percentage: f64,

    pub last_check: String,

    pub issues: Vec<String>,

    pub metadata: Option<HashMap<String, String>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitInfo {

    pub limit: u32,

    pub remaining: u32,

    pub reset_time: String,

    pub window_seconds: u32,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {

    pub id: String,

    pub actor: String,

    pub action: String,

    pub resource: String,

    pub outcome: String,

    pub context: Option<HashMap<String, String>>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlag {

    pub enabled: bool,

    pub description: Option<String>,

    pub rollout_percentage: Option<f64>,

}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointInfo {

    pub method: String,

    pub path: String,

    pub description: String,

    pub requires_auth: bool,

    pub permissions: Vec<String>,

    pub rate_limit_tier: Option<String>,

}
