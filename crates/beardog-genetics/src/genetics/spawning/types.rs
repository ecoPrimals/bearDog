// SPDX-License-Identifier: AGPL-3.0-or-later

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use beardog_auth::auth::{BearDogGenetics, NodeCapability, SecurityClearance};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Inputs for spawning: desired capabilities, clearance, and optional parent genomes.
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
    #[must_use]
    pub fn success(genetics: BearDogGenetics, metrics: HashMap<String, f64>) -> Self {
        Self {
            genetics,
            success: true,
            messages: vec!["Spawning completed successfully".to_string()],
            metrics,
        }
    }

    /// Failure operation.
    #[must_use]
    pub fn failure(error_message: &str) -> Self {
        Self {
            genetics: BearDogGenetics::default(),
            success: false,
            messages: vec![error_message.to_string()],
            metrics: HashMap::new(),
        }
    }
}

#[cfg(test)]
#[expect(clippy::unwrap_used, clippy::expect_used, reason = "test assertions")]
mod tests {
    use super::*;

    // === SpawnRequest Tests ===

    #[test]
    fn test_spawn_request_default() {
        let request = SpawnRequest::default();
        assert!(request.required_capabilities.is_empty());
        assert!(matches!(
            request.security_clearance,
            SecurityClearance::Basic
        ));
        assert!(request.parent_genetics.is_empty());
    }

    #[test]
    fn test_spawn_request_clone() {
        let request = SpawnRequest::default();
        let cloned = request.clone();
        assert_eq!(
            request.required_capabilities.len(),
            cloned.required_capabilities.len()
        );
    }

    #[test]
    fn test_spawn_request_serialization() {
        let request = SpawnRequest::default();
        let serialized = serde_json::to_string(&request).expect("serialize");
        let deserialized: SpawnRequest = serde_json::from_str(&serialized).expect("deserialize");
        assert!(deserialized.required_capabilities.is_empty());
    }

    #[test]
    fn test_spawn_request_with_capabilities() {
        let request = SpawnRequest {
            required_capabilities: vec![
                NodeCapability::SecurityAnalysis,
                NodeCapability::ThreatDetection,
            ],
            security_clearance: SecurityClearance::High,
            parent_genetics: vec![],
        };
        assert_eq!(request.required_capabilities.len(), 2);
    }

    // === SpawnResult Tests ===

    #[test]
    fn test_spawn_result_success() {
        let genetics = BearDogGenetics::default();
        let mut metrics = HashMap::new();
        metrics.insert("duration_ms".to_string(), 100.0);

        let result = SpawnResult::success(genetics, metrics);
        assert!(result.success);
        assert!(!result.messages.is_empty());
        assert!(result.messages[0].contains("successfully"));
    }

    #[test]
    fn test_spawn_result_failure() {
        let result = SpawnResult::failure("Spawning failed: insufficient entropy");
        assert!(!result.success);
        assert!(result.messages[0].contains("insufficient entropy"));
        assert!(result.metrics.is_empty());
    }

    #[test]
    fn test_spawn_result_clone() {
        let result = SpawnResult::failure("test error");
        let cloned = result.clone();
        assert_eq!(result.success, cloned.success);
        assert_eq!(result.messages, cloned.messages);
    }

    #[test]
    fn test_spawn_result_serialization() {
        let result = SpawnResult::failure("test");
        let serialized = serde_json::to_string(&result).expect("serialize");
        let deserialized: SpawnResult = serde_json::from_str(&serialized).expect("deserialize");
        assert_eq!(result.success, deserialized.success);
    }

    #[test]
    fn test_spawn_result_with_metrics() {
        let genetics = BearDogGenetics::default();
        let mut metrics = HashMap::new();
        metrics.insert("generation_time".to_string(), 250.5);
        metrics.insert("entropy_used".to_string(), 1024.0);
        metrics.insert("fitness_score".to_string(), 0.95);

        let result = SpawnResult::success(genetics, metrics);
        assert_eq!(result.metrics.len(), 3);
        assert_eq!(result.metrics.get("fitness_score"), Some(&0.95));
    }
}
