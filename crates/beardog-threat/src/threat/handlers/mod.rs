// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod analysis;
/// Core functionality
/// Core functionality
pub mod core;
pub mod enrichment;
pub mod incident;
pub mod management;
pub mod ml_integration;
pub mod response;
pub mod threat_feeds;

pub use self::enrichment::*;
pub use self::management::*;

#[allow(
    unused_imports,
    clippy::module_inception,
    clippy::manual_range_contains,
    clippy::assertions_on_constants,
    clippy::useless_vec,
    clippy::absurd_extreme_comparisons,
    unused_comparisons
)]
#[cfg(test)]
mod existing_tests {
    use crate::threat::types::*;
    use beardog_errors::BearDogError;
    #[tokio::test]
    async fn test_threat_detection_engine_creation() -> Result<(), BearDogError> {
        let engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default())?;
        assert_eq!(engine.detection_rules.len(), 0);
        assert_eq!(engine.ml_models.len(), 0);
        Ok(())
    }
    #[tokio::test]
    async fn test_placeholder_engine() -> Result<(), BearDogError> {
        let engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default())?;
        assert!(engine.blocked_sources.is_empty());
        assert!(engine.quarantined_systems.is_empty());
        assert!(engine.active_threats.is_empty());
        assert!(engine.detection_rules.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_basic_event_analysis() {
        let _engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default());
        let event = SecurityEvent::new("test_event", chrono::Utc::now(), "login")
            .with_source_ip("192.168.1.1")
            .with_user_id("test_user");

        assert_eq!(event.event_type, "test_event");
    }
    #[tokio::test]
    async fn test_rule_management() -> Result<(), BearDogError> {
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default())?;
        let initial_count = engine.detection_rules.len();
        let rule = crate::threat::types::DetectionRule {
            id: "test_rule".to_string(),
            name: "Test Rule".to_string(),
            description: "Test detection rule".to_string(),
            pattern: "test_pattern".to_string(),
            severity: ThreatSeverity::Medium,
            enabled: true,
            confidence: 0.8,
            condition: crate::threat::types::RuleCondition::FieldEquals {
                field: "test_field".to_string(),
                value: "test_value".to_string(),
            },
            rule_type: crate::threat::types::ThreatRuleType::Signature,
        };
        engine.add_detection_rule(rule);
        assert_eq!(engine.detection_rules.len(), initial_count + 1);
        let result = engine.remove_rule("test_rule");
        assert!(result);
        assert_eq!(engine.detection_rules.len(), initial_count);
        Ok(())
    }

    #[tokio::test]
    async fn test_statistics_collection() -> Result<(), BearDogError> {
        let engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default())?;
        let stats = engine.get_feed_statistics();
        assert_eq!(stats.total_feeds, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_ml_model_management() -> Result<(), BearDogError> {
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default())?;
        let initial_count = engine.ml_models.len();
        let now = chrono::Utc::now();
        let model = crate::threat::types::engine::threat_engine::MlModel {
            id: "test_model".to_string(),
            name: "Test Model".to_string(),
            description: "Test ML model".to_string(),
            version: "1.0.0".to_string(),
            accuracy: 0.85,
            precision: 0.80,
            recall: 0.90,
            f1_score: 0.85,
            created_at: now,
            last_updated: now,
            model_type: "AnomalyDetection".to_string(),
            training_data_size: 1000,
            is_active: true,
        };
        engine.add_ml_model(model);
        assert_eq!(engine.ml_models.len(), initial_count + 1);
        Ok(())
    }
}

#[allow(
    unused_imports,
    clippy::module_inception,
    clippy::manual_range_contains,
    clippy::assertions_on_constants,
    clippy::useless_vec,
    clippy::absurd_extreme_comparisons,
    unused_comparisons
)]
#[cfg(test)]
mod tests;
