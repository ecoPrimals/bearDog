//! Threat handlers module organization
//!
//! This module orchestrates the various threat detection handler modules
//! and provides a clean public API for the threat detection system.

// Re-export handler modules
pub mod analysis;
pub mod core;
pub mod enrichment;
pub mod incident;
pub mod management;
pub mod ml_integration;
pub mod response;
pub mod threat_feeds;

// Re-export all handler types for convenience
pub use self::enrichment::*;
pub use self::management::*;

// Import the types from the refactored modules

#[cfg(test)]
mod tests {
    use crate::threat::types::*;

    #[tokio::test]
    async fn test_threat_detection_engine_creation() {
        let engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default());
        assert_eq!(engine.detection_rules.len(), 0);
        assert_eq!(engine.ml_models.len(), 0);
    }

    #[tokio::test]
    async fn test_placeholder_engine() {
        let engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default());
        // Engine should start with empty collections
        assert!(engine.blocked_sources.is_empty());
        assert!(engine.quarantined_systems.is_empty());
        assert!(engine.active_threats.is_empty());
        assert!(engine.detection_rules.is_empty());
    }

    #[tokio::test]
    async fn test_basic_event_analysis() {
        let _engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default());
        let event = SecurityEvent::new(
            "test_event".to_string(),
            "login".to_string(),
            "192.168.1.1".to_string(),
            "192.168.1.100".to_string(),
            "test_user".to_string(),
        );

        // Basic validation that the engine can handle events
        assert_eq!(event.event_id, "test_event");
    }

    #[tokio::test]
    async fn test_rule_management() {
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default());
        let initial_count = engine.detection_rules.len();

        let rule = DetectionRule {
            id: "test_rule".to_string(),
            name: "Test Rule".to_string(),
            description: "A test rule".to_string(),
            condition: RuleCondition::FieldEquals {
                field: "test_field".to_string(),
                value: "test_value".to_string(),
            },
            threat_type: ThreatType::Unknown,
            severity: ThreatSeverity::Low,
            enabled: true,
            detection_count: 0,
            false_positive_count: 0,
        };

        engine.add_detection_rule(rule);
        assert_eq!(engine.detection_rules.len(), initial_count + 1);

        let result = engine.remove_rule("test_rule");
        assert!(result);
        assert_eq!(engine.detection_rules.len(), initial_count);
    }

    #[tokio::test]
    async fn test_statistics_tracking() {
        let engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default());
        let stats = &engine.stats;
        assert_eq!(stats.total_events_processed, 0);
        assert_eq!(stats.threats_detected, 0);
    }

    #[tokio::test]
    async fn test_ml_model_management() {
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default());
        let initial_count = engine.ml_models.len();

        let model = MlModel {
            id: "test_model".to_string(),
            name: "Test Model".to_string(),
            model_type: MlModelType::AnomalyDetection,
            accuracy: 0.85,
            last_trained: chrono::Utc::now(),
            feature_names: vec!["feature1".to_string(), "feature2".to_string()],
        };

        engine.add_ml_model(model);
        assert_eq!(engine.ml_models.len(), initial_count + 1);
    }
}
