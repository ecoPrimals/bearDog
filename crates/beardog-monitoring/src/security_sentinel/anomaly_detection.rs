

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
    /// The component value
    pub component: String,

    /// The metric type value
    pub metric_type: String,

    /// The current value value
    pub current_value: f64,

    /// The expected value value
    pub expected_value: f64,

    /// The severity value
    pub severity: AnomalySeverity,


    pub confidence: f64,

    /// Mapping of context
    pub context: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnomalySeverity {


    /// Represents low variant
    Low,


    /// Represents medium variant
    Medium,


    /// Represents high variant
    High,


    /// Represents critical variant
    Critical,
