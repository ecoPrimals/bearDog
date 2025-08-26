

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceRecommendation {

    pub recommendation_id: String,

    pub recommendation_type: String,

    pub priority: RecommendationPriority,

    pub description: String,

    pub suggested_action: String,

    pub expected_impact: String,

    pub effort_estimate: String,

    pub metadata: HashMap<String, String>,
}

pub enum RecommendationPriority {

    Low,

    Medium,

    High,

    Critical,
