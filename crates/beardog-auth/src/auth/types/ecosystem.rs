

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::genetics::NodeCapability;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkOperation {

    pub operation_type: String,

    pub resources: Vec<String>,

    pub expected_duration: u64,

    pub priority: u8,

    pub required_capabilities: Vec<NodeCapability>,
}

pub struct NetworkEffectAnalysis {

    pub predicted_impact: f64,

    pub affected_nodes: Vec<String>,

    pub resource_requirements: HashMap<String, u64>,

    pub performance_impact: f64,

    pub security_implications: Vec<String>,
