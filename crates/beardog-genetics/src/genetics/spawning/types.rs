// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_auth::auth::{BearDogGenetics, NodeCapability, SecurityClearance};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnRequest {
    /// Collection of required capabilities
    pub required_capabilities: Vec<NodeCapability>,
    /// Security clearance level
    /// The security clearance value
    pub security_clearance: SecurityClearance,
    /// Collection of parent genetics
    pub parent_genetics: Vec<BearDogGenetics>,
}

impl Default for SpawnRequest {
    fn default() -> Self {
        Self {
            required_capabilities: vec![],
            security_clearance: SecurityClearance::Basic,
            parent_genetics: vec![],
        }
    }
}

/// Result of spawning operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnResult {
    /// The generated genetics
    /// The genetics value
    pub genetics: BearDogGenetics,
    /// Whether the spawning was successful
    /// Whether success is enabled
    pub success: bool,
    /// Status messages
    /// Collection of messages
    pub messages: Vec<String>,
    /// Mapping of metrics
    pub metrics: HashMap<String, f64>,
}

impl SpawnResult {
    /// Success operation.
    pub fn success(genetics: BearDogGenetics, metrics: HashMap<String, f64>) -> Self {
        Self {
            genetics,
            success: true,
            messages: vec!["Spawning completed successfully".to_string()],
            metrics,
        }
    }

    /// Failure operation.
    pub fn failure(error_message: &str) -> Self {
        Self {
            genetics: BearDogGenetics::default(),
            success: false,
            messages: vec![error_message.to_string()],
            metrics: HashMap::new(),
        }
    }
}
