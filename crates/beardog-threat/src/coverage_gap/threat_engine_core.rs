// SPDX-License-Identifier: AGPL-3.0-or-later

// ============================================================================
// types/engine/threat_engine.rs - 7 uncovered lines, 84.09% coverage
// ============================================================================

#[cfg(test)]
mod threat_engine_gap_tests {
    use crate::threat::types::engine::threat_engine::{
        MlModel, ThreatDetectionEngine, ThreatDetectionStats,
    };
    use crate::threat::types::{DetectionRule, ThreatDetectionConfig, ThreatIntelligenceFeed};

    #[test]
    fn test_engine_add_threat_feed() {
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default())
            .expect("default threat detection config should construct engine");
        let feed = ThreatIntelligenceFeed {
            id: "feed-1".to_string(),
            name: "Test Feed".to_string(),
            source: "source".to_string(),
            last_updated: std::time::SystemTime::now(),
            enabled: true,
            reliability: 0.9,
            indicators: vec![],
        };
        engine.add_threat_feed(feed);
        assert_eq!(engine.threat_feeds.len(), 1);
    }

    #[test]
    fn test_engine_update_threat_feed() {
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default())
            .expect("default threat detection config should construct engine");
        let feed = ThreatIntelligenceFeed {
            id: "feed-1".to_string(),
            name: "Old Name".to_string(),
            source: "source".to_string(),
            last_updated: std::time::SystemTime::now(),
            enabled: true,
            reliability: 0.5,
            indicators: vec![],
        };
        engine.add_threat_feed(feed);

        let updated_feed = ThreatIntelligenceFeed {
            id: "feed-1".to_string(),
            name: "New Name".to_string(),
            source: "source".to_string(),
            last_updated: std::time::SystemTime::now(),
            enabled: true,
            reliability: 0.95,
            indicators: vec![],
        };
        engine.update_threat_feed(updated_feed);
        assert_eq!(
            engine
                .threat_feeds
                .get("feed-1")
                .expect("feed-1 must exist after update")
                .name,
            "New Name"
        );
    }

    #[test]
    fn test_engine_remove_rule() {
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default())
            .expect("default threat detection config should construct engine");
        let rule = DetectionRule::default();
        let id = rule.id.clone();
        engine.add_detection_rule(rule);
        assert_eq!(engine.detection_rules.len(), 1);

        let removed = engine.remove_rule(&id);
        assert!(removed);
        assert!(engine.detection_rules.is_empty());
    }

    #[test]
    fn test_engine_remove_rule_not_found() {
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default())
            .expect("default threat detection config should construct engine");
        assert!(!engine.remove_rule("nonexistent"));
    }

    #[test]
    fn test_engine_add_ml_model() {
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default())
            .expect("default threat detection config should construct engine");
        let model = MlModel {
            id: "m1".to_string(),
            name: "Test".to_string(),
            description: "desc".to_string(),
            version: "1.0".to_string(),
            accuracy: 0.9,
            precision: 0.85,
            recall: 0.88,
            f1_score: 0.86,
            created_at: chrono::Utc::now(),
            last_updated: chrono::Utc::now(),
            model_type: "classification".to_string(),
            training_data_size: 10000,
            is_active: true,
        };
        engine.add_ml_model(model);
        assert_eq!(engine.ml_models.len(), 1);
    }

    #[test]
    fn test_engine_get_stats() {
        let engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default())
            .expect("default threat detection config should construct engine");
        let stats = engine.get_stats();
        assert_eq!(stats.total_threats_detected, 0);
    }

    #[test]
    fn test_threat_detection_stats_default() {
        let stats = ThreatDetectionStats::default();
        assert_eq!(stats.total_threats_detected, 0);
        assert_eq!(stats.false_positive_rate, 0.0);
        assert_eq!(stats.detection_accuracy, 0.0);
        assert_eq!(stats.average_detection_time_ms, 0.0);
        assert!(stats.threats_by_severity.is_empty());
    }
}
