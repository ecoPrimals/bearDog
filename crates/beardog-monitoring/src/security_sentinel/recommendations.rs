// SPDX-License-Identifier: AGPL-3.0-only



use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
    /// The recommendation type value
    pub recommendation_type: String,

    /// The priority value
    pub priority: RecommendationPriority,

    /// The description value
    pub description: String,

    /// The suggested action value
    pub suggested_action: String,

    /// The expected impact value
    pub expected_impact: String,


    pub effort_estimate: String,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,
}

pub enum RecommendationPriority {


    /// Represents low variant
    Low,


    /// Represents medium variant
    Medium,


    /// Represents high variant
    High,


    /// Represents critical variant
    Critical,
