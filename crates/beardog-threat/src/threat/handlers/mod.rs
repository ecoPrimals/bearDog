pub mod analysis;
pub mod core;
pub mod enrichment;
pub mod incident;
pub mod management;
pub mod ml_integration;
pub mod response;
pub mod threat_feeds;

pub use self::enrichment::*;
pub use self::management::*;

#[cfg(test)]
mod tests {
    use crate::threat::types::{
        engine::{DetectionRule, MlModel},
        *,
    };
    use beardog_errors::BearDogError;
    #[tokio::test]
    async fn test_threat_detection_engine_creation() {
        let engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default());
        assert_eq!(engine.detection_rules.len(), 0);
        assert_eq!(engine.ml_models.len(), 0);
    }
    #[tokio::test]
    async fn test_placeholder_engine() -> Result<(), BearDogError> {
        let engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default());
        assert!(engine.blocked_sources.is_empty());
        assert!(engine.quarantined_systems.is_empty());
        assert!(engine.active_threats.is_empty());
        assert!(engine.detection_rules.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_basic_event_analysis() {
        let _engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default());
        let event = SecurityEvent::new(
            "test_event".to_string(),
            chrono::Utc::now(),
            "login".to_string(),
        )
        .with_source_ip("192.168.1.1".to_string())
        .with_user_id("test_user".to_string());

        assert_eq!(event.event_type, "test_event");
    }
    #[tokio::test]
    async fn test_rule_management() -> Result<(), BearDogError> {
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default());
        let initial_count = engine.detection_rules.len();
        let rule = DetectionRule {
            id: "test_rule".to_string(),
            name: "Test Rule".to_string(),
            description: "A test rule".to_string(),
            condition: crate::threat::types::engine::conditions::RuleCondition::Always,
            threat_type: ThreatType::Unknown,
            severity: ThreatSeverity::Medium,
            enabled: true,
            detection_count: 0,
            false_positive_count: 0,
            rule_type: "test".to_string(),
            confidence: 0.5,
            detection_logic: "test_logic".to_string(),
            mitre_technique_id: None,
            mitre_tactic: None,
            author: "Test".to_string(),
            actions: Vec::new(),
            version: "1.0.0".to_string(),
            created_at: chrono::Utc::now(),
            modified_at: chrono::Utc::now(),
            tags: Vec::new(),
            references: Vec::new(),
            last_triggered: None,
            metadata: std::collections::HashMap::new(),
        };
        engine.add_detection_rule(rule);
        assert_eq!(engine.detection_rules.len(), initial_count + 1);
        let result = engine.remove_rule("test_rule");
        assert!(result);
        assert_eq!(engine.detection_rules.len(), initial_count);
        Ok(())
    }

    #[tokio::test]
    async fn test_statistics_tracking() -> Result<(), BearDogError> {
        let engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default());
        let stats = &engine.stats;
        assert_eq!(stats.detections, 0);
        assert_eq!(stats.total_threats, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_ml_model_management() -> Result<(), BearDogError> {
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
        Ok(())
    }
}
