// SPDX-License-Identifier: AGPL-3.0-only

// ============================================================================
// handlers/management.rs - 112 uncovered lines, 2.61% coverage
// ============================================================================

#[cfg(test)]
mod management_tests {
    use crate::threat::handlers::core::ThreatDetectionEngine;
    use crate::threat::types::{
        DetectionRule, RuleCondition, ThreatEvent, ThreatRuleType, ThreatSeverity, ThreatSource,
        ThreatTarget, ThreatType,
    };
    use beardog_errors::BearDogError;
    use beardog_types::canonical::config::domains::threat::ThreatDetectionConfig;

    fn make_engine() -> ThreatDetectionEngine {
        ThreatDetectionEngine::new(ThreatDetectionConfig::default())
            .expect("default threat detection config should construct engine")
    }

    fn make_rule(id: &str, name: &str) -> DetectionRule {
        DetectionRule {
            id: id.to_string(),
            name: name.to_string(),
            description: "Test rule".to_string(),
            pattern: "test_pattern".to_string(),
            severity: ThreatSeverity::Medium,
            enabled: true,
            confidence: 0.8,
            condition: RuleCondition::FieldEquals {
                field: "type".to_string(),
                value: "test".to_string(),
            },
            rule_type: ThreatRuleType::Signature,
        }
    }

    #[test]
    fn test_add_detection_rule() {
        let mut engine = make_engine();
        let rule = make_rule("rule-1", "Test Rule");
        engine.add_detection_rule(rule);
        assert_eq!(engine.detection_rules.len(), 1);
        assert_eq!(engine.detection_rules[0].id, "rule-1");
    }

    #[test]
    fn test_remove_detection_rule_exists() {
        let mut engine = make_engine();
        engine.add_detection_rule(make_rule("r1", "Rule 1"));
        engine.add_detection_rule(make_rule("r2", "Rule 2"));
        assert_eq!(engine.detection_rules.len(), 2);

        let removed = engine.remove_detection_rule("r1");
        assert!(removed);
        assert_eq!(engine.detection_rules.len(), 1);
        assert_eq!(engine.detection_rules[0].id, "r2");
    }

    #[test]
    fn test_remove_detection_rule_not_exists() {
        let mut engine = make_engine();
        engine.add_detection_rule(make_rule("r1", "Rule 1"));

        let removed = engine.remove_detection_rule("nonexistent");
        assert!(!removed);
        assert_eq!(engine.detection_rules.len(), 1);
    }

    #[test]
    fn test_enable_detection_rule_enable() {
        let mut engine = make_engine();
        let mut rule = make_rule("r1", "Rule 1");
        rule.enabled = false;
        engine.add_detection_rule(rule);

        let result = engine.enable_detection_rule("r1", true);
        assert!(result);
        assert!(engine.detection_rules[0].enabled);
    }

    #[test]
    fn test_enable_detection_rule_disable() {
        let mut engine = make_engine();
        engine.add_detection_rule(make_rule("r1", "Rule 1"));

        let result = engine.enable_detection_rule("r1", false);
        assert!(result);
        assert!(!engine.detection_rules[0].enabled);
    }

    #[test]
    fn test_enable_detection_rule_not_found() {
        let mut engine = make_engine();
        let result = engine.enable_detection_rule("nonexistent", true);
        assert!(!result);
    }

    #[test]
    fn test_get_statistics() {
        let engine = make_engine();
        let stats = engine.get_statistics();
        assert_eq!(stats.total_threats_detected, 0);
        assert_eq!(stats.false_positive_rate, 0.0);
        assert_eq!(stats.detection_accuracy, 0.0);
    }

    #[test]
    fn test_reset_statistics() {
        let mut engine = make_engine();
        engine.stats.total_threats_detected = 100;
        engine.stats.false_positive_rate = 0.5;
        engine.reset_statistics();

        let stats = engine.get_statistics();
        assert_eq!(stats.total_threats_detected, 0);
        assert_eq!(stats.false_positive_rate, 0.0);
    }

    #[test]
    fn test_update_detection_rule_exists() {
        let mut engine = make_engine();
        engine.add_detection_rule(make_rule("r1", "Old Name"));

        let updated_rule = make_rule("r1", "New Name");
        let result = engine.update_detection_rule("r1", updated_rule);
        assert!(result);
        assert_eq!(engine.detection_rules[0].name, "New Name");
    }

    #[test]
    fn test_update_detection_rule_not_exists() {
        let mut engine = make_engine();
        let result = engine.update_detection_rule("nonexistent", make_rule("r1", "Rule"));
        assert!(!result);
    }

    #[test]
    fn test_load_default_rules() -> Result<(), BearDogError> {
        let mut engine = make_engine();
        engine.load_default_rules()?;

        assert_eq!(engine.detection_rules.len(), 2);
        assert_eq!(engine.detection_rules[0].id, "brute_force_detection");
        assert_eq!(engine.detection_rules[1].id, "network_anomaly_detection");
        assert_eq!(engine.detection_rules[0].severity, ThreatSeverity::High);
        assert_eq!(engine.detection_rules[1].severity, ThreatSeverity::Medium);
        Ok(())
    }

    #[test]
    fn test_get_system_health_healthy() {
        let mut engine = make_engine();
        engine.add_detection_rule(make_rule("r1", "Rule 1"));

        let health = engine.get_system_health();
        assert_eq!(
            health.overall_status,
            beardog_types::canonical::providers_unified::traits::HealthStatus::Healthy
        );
        assert_eq!(
            health.components.get("detection_rules"),
            Some(&beardog_types::canonical::providers_unified::traits::HealthStatus::Healthy)
        );
        assert_eq!(
            health.components.get("threat_status"),
            Some(&beardog_types::canonical::providers_unified::traits::HealthStatus::Healthy)
        );
        assert_eq!(health.details, "Threat management system health check");
    }

    #[test]
    fn test_get_system_health_degraded_no_enabled_rules() {
        let mut engine = make_engine();
        let mut rule = make_rule("r1", "Rule 1");
        rule.enabled = false;
        engine.add_detection_rule(rule);

        let health = engine.get_system_health();
        assert_eq!(
            health.components.get("detection_rules"),
            Some(&beardog_types::canonical::providers_unified::traits::HealthStatus::Degraded)
        );
    }

    #[test]
    fn test_get_system_health_degraded_active_threats() {
        let mut engine = make_engine();
        engine.add_detection_rule(make_rule("r1", "Rule 1"));
        engine.active_threats.insert(
            "t1".to_string(),
            ThreatEvent::new(
                "t1".to_string(),
                ThreatType::Malware,
                ThreatSeverity::High,
                ThreatSource::default(),
                ThreatTarget::default(),
            ),
        );

        let health = engine.get_system_health();
        assert_eq!(
            health.overall_status,
            beardog_types::canonical::providers_unified::traits::HealthStatus::Degraded
        );
        assert_eq!(
            health.components.get("threat_status"),
            Some(&beardog_types::canonical::providers_unified::traits::HealthStatus::Degraded)
        );
    }

    #[tokio::test]
    async fn test_perform_maintenance_empty_history() -> Result<(), BearDogError> {
        let mut engine = make_engine();
        engine.perform_maintenance().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_perform_maintenance_large_history() -> Result<(), BearDogError> {
        let mut engine = make_engine();

        {
            let mut history = engine.event_history.write().await;
            for i in 0..5500 {
                history.push(ThreatEvent::new(
                    format!("t-{i}"),
                    ThreatType::Suspicious,
                    ThreatSeverity::Low,
                    ThreatSource::default(),
                    ThreatTarget::default(),
                ));
            }
        }

        engine.perform_maintenance().await?;

        let history = engine.event_history.read().await;
        assert_eq!(history.len(), 4500);
        Ok(())
    }

    #[tokio::test]
    async fn test_perform_maintenance_cleans_completed_incidents() -> Result<(), BearDogError> {
        use crate::threat::types::{IncidentResponse, ResponseStatus};

        let mut engine = make_engine();

        {
            let mut incidents = engine.active_incidents.write().await;
            incidents.insert(
                "inc-1".to_string(),
                IncidentResponse {
                    id: "inc-1".to_string(),
                    threat_id: "t1".to_string(),
                    status: ResponseStatus::Completed,
                    assigned_team: "team".to_string(),
                    started_at: std::time::SystemTime::now(),
                    completed_at: Some(std::time::SystemTime::now()),
                    actions: vec![],
                    notes: String::new(),
                },
            );
            incidents.insert(
                "inc-2".to_string(),
                IncidentResponse {
                    id: "inc-2".to_string(),
                    threat_id: "t2".to_string(),
                    status: ResponseStatus::InProgress,
                    assigned_team: "team".to_string(),
                    started_at: std::time::SystemTime::now(),
                    completed_at: None,
                    actions: vec![],
                    notes: String::new(),
                },
            );
        }

        engine.perform_maintenance().await?;

        let incidents = engine.active_incidents.read().await;
        assert_eq!(incidents.len(), 1);
        assert!(incidents.contains_key("inc-2"));
        Ok(())
    }

    #[test]
    fn test_get_system_status() {
        let engine = make_engine();
        let status = engine.get_system_status();
        assert_eq!(status.system_uptime, "N/A");
        assert_eq!(status.memory_usage, "N/A");
        assert_eq!(status.cpu_usage, "N/A");
    }

    #[test]
    fn test_system_health_serde() {
        use crate::threat::handlers::management::SystemHealth;
        let engine = make_engine();
        let health = engine.get_system_health();

        let serialized = serde_json::to_string(&health).expect("SystemHealth should serialize");
        assert!(!serialized.is_empty());

        let deserialized: SystemHealth =
            serde_json::from_str(&serialized).expect("SystemHealth round-trip JSON");
        assert_eq!(deserialized.details, health.details);
    }

    #[test]
    fn test_system_status_serde() {
        use crate::threat::handlers::management::SystemStatus;
        let engine = make_engine();
        let status = engine.get_system_status();

        let serialized = serde_json::to_string(&status).expect("SystemStatus should serialize");
        let deserialized: SystemStatus =
            serde_json::from_str(&serialized).expect("SystemStatus round-trip JSON");
        assert_eq!(deserialized.system_uptime, "N/A");
    }

    #[test]
    fn test_threat_stats_default() {
        use crate::threat::handlers::management::ThreatStats;
        let stats = ThreatStats::default();
        assert_eq!(stats.total_rules, 0);
        assert_eq!(stats.active_threats, 0);
        assert_eq!(stats.blocked_sources, 0);
        assert_eq!(stats.quarantined_systems, 0);
        assert_eq!(stats.threat_feeds, 0);
        assert_eq!(stats.ml_models, 0);
        assert_eq!(stats.events_processed, 0);
    }
}
