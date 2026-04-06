// SPDX-License-Identifier: AGPL-3.0-or-later



// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::genetics::NodeCapability;

#[derive(Debug, Clone)]
    /// Collection of resources
    pub resources: Vec<String>,

    /// Number of expected_duration
    pub expected_duration: u64,

    /// Number of priority
    pub priority: u8,

    /// Collection of required capabilities
    pub required_capabilities: Vec<NodeCapability>,
}

pub struct NetworkEffectAnalysis {

    /// The predicted impact value
    pub predicted_impact: f64,

    /// Collection of affected nodes
    pub affected_nodes: Vec<String>,

    /// Mapping of resource requirements
    pub resource_requirements: HashMap<String, u64>,


    pub performance_impact: f64,

    /// Collection of security implications
    pub security_implications: Vec<String>,
