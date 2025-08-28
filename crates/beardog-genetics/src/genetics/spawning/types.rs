//! Spawning Types
//!
//! This module defines the core types for genetic spawning operations.

use beardog_auth::auth::{BearDogGenetics, NodeCapability, SecurityClearance};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request for spawning new genetics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnRequest {
    pub required_capabilities: Vec<NodeCapability>,
    pub security_clearance: SecurityClearance,
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

/// Result of a genetics spawning operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnResult {
    pub genetics: BearDogGenetics,
    pub success: bool,
    pub messages: Vec<String>,
    pub metrics: HashMap<String, f64>,
}

impl SpawnResult {
    pub fn success(genetics: BearDogGenetics, metrics: HashMap<String, f64>) -> Self {
        Self {
            genetics,
            success: true,
            messages: vec!["Spawning completed successfully".to_string()],
            metrics,
        }
    }

    pub fn failure(error_message: String) -> Self {
        Self {
            genetics: BearDogGenetics::default(),
            success: false,
            messages: vec![error_message],
            metrics: HashMap::new(),
        }
    }
}
