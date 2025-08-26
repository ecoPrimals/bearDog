

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceAnomaly {

    pub anomaly_id: String,

    pub component: String,

    pub metric_type: String,

    pub current_value: f64,

    pub expected_value: f64,

    pub severity: AnomalySeverity,

    pub confidence: f64,

    pub context: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnomalySeverity {

    Low,

    Medium,

    High,

    Critical,
