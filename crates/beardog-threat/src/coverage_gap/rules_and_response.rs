// SPDX-License-Identifier: AGPL-3.0-only

// ============================================================================
// types/engine/rules.rs - 59 uncovered lines
// ============================================================================

#[cfg(test)]
mod rules_gap_tests {
    use crate::threat::types::ThreatSeverity;
    use crate::threat::types::engine::conditions::RuleCondition;
    use crate::threat::types::engine::rules::{
        DetectionRule, RulePerformanceMetrics, ThreatRuleType,
    };
    use std::collections::HashMap;

    #[test]
    fn test_rule_execute_no_match() {
        let rule = DetectionRule::new(
            "r1",
            "Test",
            "desc",
            ThreatRuleType::Signature,
            ThreatSeverity::Medium,
            RuleCondition::field_equals("type", "malware"),
        );
        let mut data = HashMap::new();
        data.insert("type".to_string(), "normal".to_string());

        let result = rule.execute(&data);
        assert!(!result.matched);
        assert_eq!(result.confidence, 0.0);
        assert!(result.evidence.is_empty());
        assert!(result.threat_type.is_none());
        assert!(result.matched_conditions.is_empty());
    }

    #[test]
    fn test_rule_execute_disabled() {
        let mut rule = DetectionRule::new(
            "r1",
            "Test",
            "desc",
            ThreatRuleType::Signature,
            ThreatSeverity::Medium,
            RuleCondition::Always,
        );
        rule.disable();

        let result = rule.execute(&HashMap::new());
        assert!(!result.matched);
    }

    #[test]
    fn test_rule_validate_empty_description_warning() {
        let rule = DetectionRule::new(
            "r1",
            "Name",
            "",
            ThreatRuleType::Signature,
            ThreatSeverity::Medium,
            RuleCondition::Always,
        );
        let validation = rule.validate();
        assert!(validation.is_valid);
        assert!(
            validation
                .warnings
                .iter()
                .any(|w| w.contains("description"))
        );
    }

    #[test]
    fn test_rule_validate_empty_tags_suggestion() {
        let rule = DetectionRule::new(
            "r1",
            "Name",
            "desc",
            ThreatRuleType::Signature,
            ThreatSeverity::Medium,
            RuleCondition::Always,
        );
        let validation = rule.validate();
        assert!(validation.suggestions.iter().any(|s| s.contains("tags")));
    }

    #[test]
    fn test_rule_is_complex() {
        let simple_rule = DetectionRule::new(
            "r1",
            "Simple",
            "desc",
            ThreatRuleType::Signature,
            ThreatSeverity::Low,
            RuleCondition::field_equals("a", "b"),
        );
        assert!(!simple_rule.is_complex());

        let complex_rule = DetectionRule::new(
            "r2",
            "Complex",
            "desc",
            ThreatRuleType::Anomaly,
            ThreatSeverity::High,
            RuleCondition::and(vec![
                RuleCondition::field_equals("a", "b"),
                RuleCondition::field_exists("c"),
            ]),
        );
        assert!(complex_rule.is_complex());
    }

    #[test]
    fn test_rule_age_days() {
        let rule = DetectionRule::new(
            "r1",
            "Test",
            "desc",
            ThreatRuleType::Signature,
            ThreatSeverity::Low,
            RuleCondition::Always,
        );
        assert_eq!(rule.age_days(), 0);
    }

    #[test]
    fn test_rule_add_metadata() {
        let mut rule = DetectionRule::new(
            "r1",
            "Test",
            "desc",
            ThreatRuleType::Signature,
            ThreatSeverity::Low,
            RuleCondition::Always,
        );
        rule.add_metadata("source", "custom");
        assert_eq!(rule.metadata.get("source"), Some(&"custom".to_string()));
    }

    #[test]
    fn test_rule_performance_metrics_is_performing_well() {
        let mut metrics = RulePerformanceMetrics::new("r1");
        for _ in 0..100 {
            metrics.update_with_result(5, true, true);
        }
        assert!(metrics.is_performing_well());
    }

    #[test]
    fn test_rule_performance_metrics_not_performing_well() {
        let mut metrics = RulePerformanceMetrics::new("r1");
        for _ in 0..50 {
            metrics.update_with_result(5, true, true);
        }
        for _ in 0..50 {
            metrics.update_with_result(5, false, true);
        }
        assert!(!metrics.is_performing_well());
    }

    #[test]
    fn test_threat_rule_type_display() {
        assert_eq!(ThreatRuleType::Signature.to_string(), "Signature");
        assert_eq!(ThreatRuleType::Behavioral.to_string(), "Behavioral");
        assert_eq!(ThreatRuleType::Anomaly.to_string(), "Anomaly");
        assert_eq!(ThreatRuleType::Heuristic.to_string(), "Heuristic");
        assert_eq!(
            ThreatRuleType::MachineLearning.to_string(),
            "Machine Learning"
        );
        assert_eq!(ThreatRuleType::Correlation.to_string(), "Correlation");
        assert_eq!(ThreatRuleType::Threshold.to_string(), "Threshold");
        assert_eq!(
            ThreatRuleType::Custom("YARA".to_string()).to_string(),
            "Custom: YARA"
        );
    }

    #[test]
    fn test_rule_default() {
        let rule = DetectionRule::default();
        assert!(!rule.id.is_empty()); // UUID
        assert_eq!(rule.name, "Default Rule");
        assert_eq!(rule.rule_type, ThreatRuleType::Signature);
        assert!(rule.enabled);
        assert_eq!(rule.priority, 5);
    }
}

// ============================================================================
// handlers/response.rs - 26 uncovered lines, 61.76% coverage
// ============================================================================

#[cfg(test)]
mod response_gap_tests {
    use crate::threat::handlers::response::{AutomatedThreatResponseHandler, ThreatResponseConfig};
    use crate::threat::types::{
        ThreatEvent, ThreatSeverity, ThreatSource, ThreatTarget, ThreatType,
    };

    fn make_handler(enabled: bool) -> AutomatedThreatResponseHandler {
        AutomatedThreatResponseHandler::new(ThreatResponseConfig {
            enabled,
            max_response_level: ThreatSeverity::Critical,
        })
    }

    fn make_threat() -> ThreatEvent {
        ThreatEvent::new(
            "resp-t1".to_string(),
            ThreatType::Malware,
            ThreatSeverity::High,
            ThreatSource::default(),
            ThreatTarget::default(),
        )
    }

    #[test]
    fn test_handler_debug() {
        let handler = make_handler(true);
        let debug = format!("{handler:?}");
        assert!(debug.contains("AutomatedThreatResponseHandler"));
    }

    #[test]
    fn test_response_config_debug() {
        let config = ThreatResponseConfig {
            enabled: true,
            max_response_level: ThreatSeverity::High,
        };
        let debug = format!("{config:?}");
        assert!(debug.contains("ThreatResponseConfig"));
    }

    #[tokio::test]
    async fn test_log_event_and_verify_history() {
        let handler = make_handler(true);
        let t1 = make_threat();
        handler.log_threat_event(&t1).await.unwrap();

        let history = handler.event_history.read().await;
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].id, "resp-t1");
    }

    #[test]
    fn test_handle_disabled_handler() {
        let handler = make_handler(false);
        let threat = make_threat();
        assert!(handler.handle_threat_event(&threat).is_ok());
    }

    #[test]
    fn test_handle_enabled_handler() {
        let handler = make_handler(true);
        let threat = make_threat();
        assert!(handler.handle_threat_event(&threat).is_ok());
    }

    #[test]
    fn test_update_intelligence() {
        let handler = make_handler(true);
        let threat = make_threat();
        assert!(handler.update_threat_intelligence(&threat).is_ok());
    }

    #[test]
    fn test_enable_enhanced_monitoring() {
        let handler = make_handler(true);
        let threat = make_threat();
        assert!(handler.enable_enhanced_monitoring(&threat).is_ok());
    }

    #[test]
    fn test_collect_forensics_returns_artifacts() {
        let handler = make_handler(true);
        let threat = make_threat();
        let artifacts = handler.collect_forensics(&threat).unwrap();
        assert!(!artifacts.is_empty());
        assert!(artifacts.iter().any(|a| a.starts_with("threat_id:")));
    }
}
