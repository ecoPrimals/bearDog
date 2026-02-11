//! Coverage gap tests for beardog-threat
//!
//! Targets the identified coverage gaps across multiple modules.

#![allow(
    unused_imports,
    clippy::float_cmp,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    clippy::nonminimal_bool
)]

// ============================================================================
// ThreatAPI (mod.rs) - 8 uncovered lines, 0% coverage
// ============================================================================

#[cfg(test)]
mod threat_api_tests {
    use crate::threat::ThreatAPI;
    use beardog_errors::BearDogError;

    #[test]
    fn test_create_default() -> Result<(), BearDogError> {
        let engine = ThreatAPI::create_default()?;
        assert!(engine.detection_rules.is_empty());
        assert!(engine.active_threats.is_empty());
        Ok(())
    }

    #[test]
    fn test_new_with_ml() -> Result<(), BearDogError> {
        let (threat_engine, _ml_engine) = ThreatAPI::new_with_ml()?;
        assert!(threat_engine.detection_rules.is_empty());
        Ok(())
    }
}

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
        ThreatDetectionEngine::new(ThreatDetectionConfig::default()).unwrap()
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
            Some(
                &beardog_types::canonical::providers_unified::traits::HealthStatus::Healthy
            )
        );
        assert_eq!(
            health.components.get("threat_status"),
            Some(
                &beardog_types::canonical::providers_unified::traits::HealthStatus::Healthy
            )
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
            Some(
                &beardog_types::canonical::providers_unified::traits::HealthStatus::Degraded
            )
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
            Some(
                &beardog_types::canonical::providers_unified::traits::HealthStatus::Degraded
            )
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

        let serialized = serde_json::to_string(&health).unwrap();
        assert!(!serialized.is_empty());

        let deserialized: SystemHealth = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.details, health.details);
    }

    #[test]
    fn test_system_status_serde() {
        use crate::threat::handlers::management::SystemStatus;
        let engine = make_engine();
        let status = engine.get_system_status();

        let serialized = serde_json::to_string(&status).unwrap();
        let deserialized: SystemStatus = serde_json::from_str(&serialized).unwrap();
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

// ============================================================================
// types/mod.rs - 63 uncovered lines, 65.19% coverage
// ============================================================================

#[cfg(test)]
mod types_mod_tests {
    use crate::threat::types::{
        DetectionMethod, DetectionRule, MitigationStep, RuleCondition, SecurityEvent,
        ThreatAction, ThreatEvent, ThreatRuleType, ThreatSeverity, ThreatSource, ThreatStatus,
        ThreatTarget, ThreatType,
    };

    #[test]
    fn test_security_event_new() {
        let event = SecurityEvent::new("login_attempt", chrono::Utc::now(), "auth_service");
        assert_eq!(event.event_type, "login_attempt");
        assert_eq!(event.source, "auth_service");
        assert_eq!(event.severity, ThreatSeverity::Low);
        assert!(event.description.is_empty());
        assert!(event.data.is_empty());
        assert!(!event.id.is_empty());
    }

    #[test]
    fn test_security_event_with_source_ip() {
        let event = SecurityEvent::new("scan", chrono::Utc::now(), "firewall")
            .with_source_ip("192.168.1.100");
        assert_eq!(
            event.data.get("source_ip"),
            Some(&"192.168.1.100".to_string())
        );
    }

    #[test]
    fn test_security_event_with_user_id() {
        let event =
            SecurityEvent::new("access", chrono::Utc::now(), "gateway").with_user_id("admin");
        assert_eq!(event.data.get("user_id"), Some(&"admin".to_string()));
    }

    #[test]
    fn test_security_event_chained_builders() {
        let event = SecurityEvent::new("attempt", chrono::Utc::now(), "svc")
            .with_source_ip("10.0.0.1")
            .with_user_id("root");
        assert_eq!(event.data.get("source_ip"), Some(&"10.0.0.1".to_string()));
        assert_eq!(event.data.get("user_id"), Some(&"root".to_string()));
    }

    #[test]
    fn test_threat_event_new() {
        let event = ThreatEvent::new(
            "te-1".to_string(),
            ThreatType::Malware,
            ThreatSeverity::Critical,
            ThreatSource::default(),
            ThreatTarget::default(),
        );
        assert_eq!(event.id, "te-1");
        assert_eq!(event.threat_type, ThreatType::Malware);
        assert_eq!(event.severity, ThreatSeverity::Critical);
        assert_eq!(event.status, ThreatStatus::Detected);
        assert_eq!(event.confidence, 0.5);
        assert_eq!(event.score, 50);
        assert!(!event.mitigated);
        assert!(event.evidence.is_empty());
        assert!(event.recommended_actions.is_empty());
    }

    #[test]
    fn test_threat_event_add_mitigation_step() {
        let mut event = ThreatEvent::new(
            "te-2".to_string(),
            ThreatType::Intrusion,
            ThreatSeverity::High,
            ThreatSource::default(),
            ThreatTarget::default(),
        );
        assert!(event.mitigation_steps.is_empty());

        event.add_mitigation_step(MitigationStep::new(
            "s1".to_string(),
            "block".to_string(),
            "success".to_string(),
            true,
        ));
        assert_eq!(event.mitigation_steps.len(), 1);
        assert_eq!(event.mitigation_steps[0].id, "s1");
    }

    #[test]
    fn test_threat_event_update_status() {
        let mut event = ThreatEvent::new(
            "te-3".to_string(),
            ThreatType::Anomaly,
            ThreatSeverity::Medium,
            ThreatSource::default(),
            ThreatTarget::default(),
        );
        assert_eq!(event.status, ThreatStatus::Detected);

        event.update_status(ThreatStatus::Analyzing);
        assert_eq!(event.status, ThreatStatus::Analyzing);

        event.update_status(ThreatStatus::Resolved);
        assert_eq!(event.status, ThreatStatus::Resolved);
    }

    #[test]
    fn test_threat_event_is_active() {
        let mut event = ThreatEvent::new(
            "te-4".to_string(),
            ThreatType::Suspicious,
            ThreatSeverity::Low,
            ThreatSource::default(),
            ThreatTarget::default(),
        );
        assert!(event.is_active()); // Detected is active

        event.update_status(ThreatStatus::Analyzing);
        assert!(event.is_active());

        event.update_status(ThreatStatus::Mitigating);
        assert!(event.is_active());

        event.update_status(ThreatStatus::Contained);
        assert!(!event.is_active());

        event.update_status(ThreatStatus::Resolved);
        assert!(!event.is_active());

        event.update_status(ThreatStatus::FalsePositive);
        assert!(!event.is_active());

        event.update_status(ThreatStatus::Active);
        assert!(!event.is_active());
    }

    #[test]
    fn test_threat_severity_as_str() {
        assert_eq!(ThreatSeverity::Low.as_str(), "low");
        assert_eq!(ThreatSeverity::Medium.as_str(), "medium");
        assert_eq!(ThreatSeverity::High.as_str(), "high");
        assert_eq!(ThreatSeverity::Critical.as_str(), "critical");
    }

    #[test]
    fn test_threat_severity_score() {
        assert_eq!(ThreatSeverity::Low.score(), 1);
        assert_eq!(ThreatSeverity::Medium.score(), 2);
        assert_eq!(ThreatSeverity::High.score(), 3);
        assert_eq!(ThreatSeverity::Critical.score(), 4);
    }

    #[test]
    fn test_detection_rule_new_const() {
        let rule = DetectionRule::new(
            "r1".to_string(),
            "Rule 1".to_string(),
            "Description".to_string(),
            "pattern".to_string(),
            ThreatSeverity::High,
        );
        assert_eq!(rule.id, "r1");
        assert_eq!(rule.name, "Rule 1");
        assert!(rule.enabled);
        assert_eq!(rule.confidence, 0.8);
    }

    #[test]
    fn test_detection_rule_enable_disable() {
        let mut rule = DetectionRule::default();
        assert!(rule.enabled);

        rule.disable();
        assert!(!rule.enabled);

        rule.enable();
        assert!(rule.enabled);
    }

    #[test]
    fn test_mitigation_step_new() {
        let step = MitigationStep::new(
            "step-1".to_string(),
            "isolate".to_string(),
            "completed".to_string(),
            true,
        );
        assert_eq!(step.id, "step-1");
        assert_eq!(step.action, "isolate");
        assert_eq!(step.result, "completed");
        assert!(step.success);
    }

    #[test]
    fn test_threat_source_default() {
        let source = ThreatSource::default();
        assert_eq!(source.source_type, "unknown");
        assert_eq!(source.identifier, "unknown");
        assert!(source.ip_address.is_none());
        assert_eq!(source.reputation_score, 0.0);
    }

    #[test]
    fn test_threat_target_default() {
        let target = ThreatTarget::default();
        assert_eq!(target.target_type, "unknown");
        assert_eq!(target.identifier, "unknown");
        assert_eq!(target.resource_type, "unknown");
    }

    #[test]
    fn test_detection_rule_default() {
        let rule = DetectionRule::default();
        assert!(rule.enabled);
        assert_eq!(rule.severity, ThreatSeverity::Medium);
        assert_eq!(rule.confidence, 0.5);
    }

    #[test]
    fn test_threat_type_variants() {
        let types = vec![
            ThreatType::Malware,
            ThreatType::Intrusion,
            ThreatType::DataExfiltration,
            ThreatType::DenialOfService,
            ThreatType::PrivilegeEscalation,
            ThreatType::SuspiciousNetwork,
            ThreatType::ConfigurationTampering,
            ThreatType::UnauthorizedAccess,
            ThreatType::ResourceAbuse,
            ThreatType::Anomaly,
            ThreatType::Suspicious,
            ThreatType::Malicious,
            ThreatType::Unknown,
        ];
        for t in &types {
            let serialized = serde_json::to_string(t).unwrap();
            assert!(!serialized.is_empty());
        }
    }

    #[test]
    fn test_threat_action_variants() {
        let actions = vec![
            ThreatAction::BlockSource,
            ThreatAction::AlertSecurityTeam,
            ThreatAction::InvestigateActivity,
            ThreatAction::QuarantineAsset,
            ThreatAction::EscalateToAdmin,
            ThreatAction::LogForAnalysis,
        ];
        for a in &actions {
            let serialized = serde_json::to_string(a).unwrap();
            assert!(!serialized.is_empty());
        }
    }

    #[test]
    fn test_indicator_type_display() {
        use crate::threat::types::IndicatorType;
        assert_eq!(IndicatorType::IpAddress.to_string(), "IP Address");
        assert_eq!(IndicatorType::DomainName.to_string(), "Domain Name");
        assert_eq!(IndicatorType::Url.to_string(), "URL");
        assert_eq!(IndicatorType::FileHash.to_string(), "File Hash");
        assert_eq!(IndicatorType::EmailAddress.to_string(), "Email Address");
        assert_eq!(IndicatorType::UserAgent.to_string(), "User Agent");
        assert_eq!(IndicatorType::RegistryKey.to_string(), "Registry Key");
        assert_eq!(IndicatorType::ProcessName.to_string(), "Process Name");
    }
}

// ============================================================================
// types/incidents.rs - 18 uncovered lines, 52.63% coverage
// ============================================================================

#[cfg(test)]
mod incidents_tests {
    use crate::threat::types::incidents::{IncidentStatus, SecurityIncident};
    use crate::threat::types::ThreatSeverity;

    #[test]
    fn test_security_incident_new() {
        let incident = SecurityIncident::new(
            "INC-001".to_string(),
            "Malware Detected".to_string(),
            "Malware found on server".to_string(),
            ThreatSeverity::High,
        );
        assert_eq!(incident.id, "INC-001");
        assert_eq!(incident.title, "Malware Detected");
        assert_eq!(incident.status, IncidentStatus::Reported);
        assert!(incident.assigned_to.is_none());
        assert!(incident.threat_events.is_empty());
        assert!(incident.response_actions.is_empty());
        assert!(incident.notes.is_empty());
    }

    #[test]
    fn test_update_status() {
        let mut incident = SecurityIncident::new(
            "INC-002".to_string(),
            "Test".to_string(),
            "desc".to_string(),
            ThreatSeverity::Medium,
        );
        incident.update_status(IncidentStatus::Investigating);
        assert_eq!(incident.status, IncidentStatus::Investigating);

        incident.update_status(IncidentStatus::Resolved);
        assert_eq!(incident.status, IncidentStatus::Resolved);
    }

    #[test]
    fn test_assign_to() {
        let mut incident = SecurityIncident::new(
            "INC-003".to_string(),
            "Test".to_string(),
            "desc".to_string(),
            ThreatSeverity::Low,
        );
        assert!(incident.assigned_to.is_none());

        incident.assign_to("security_team".to_string());
        assert_eq!(incident.assigned_to, Some("security_team".to_string()));
    }

    #[test]
    fn test_add_note() {
        let mut incident = SecurityIncident::new(
            "INC-004".to_string(),
            "Test".to_string(),
            "desc".to_string(),
            ThreatSeverity::Low,
        );
        incident.add_note("Initial investigation started".to_string());
        incident.add_note("Found suspicious file".to_string());
        assert_eq!(incident.notes.len(), 2);
        assert_eq!(incident.notes[0], "Initial investigation started");
    }

    #[test]
    fn test_link_threat_event() {
        let mut incident = SecurityIncident::new(
            "INC-005".to_string(),
            "Test".to_string(),
            "desc".to_string(),
            ThreatSeverity::Critical,
        );
        incident.link_threat_event("threat-001".to_string());
        incident.link_threat_event("threat-002".to_string());
        assert_eq!(incident.threat_events.len(), 2);

        // Duplicate should not be added
        incident.link_threat_event("threat-001".to_string());
        assert_eq!(incident.threat_events.len(), 2);
    }

    #[test]
    fn test_incident_status_variants() {
        let statuses = vec![
            IncidentStatus::Reported,
            IncidentStatus::Investigating,
            IncidentStatus::Confirmed,
            IncidentStatus::Containing,
            IncidentStatus::Contained,
            IncidentStatus::Remediating,
            IncidentStatus::Resolved,
            IncidentStatus::FalsePositive,
            IncidentStatus::Closed,
        ];
        for s in &statuses {
            let serialized = serde_json::to_string(s).unwrap();
            assert!(!serialized.is_empty());
        }
    }
}

// ============================================================================
// types/engine/conditions.rs - 49 uncovered lines, 77.52% coverage
// ============================================================================

#[cfg(test)]
mod conditions_gap_tests {
    use crate::threat::types::engine::conditions::{ConditionBuilder, RuleCondition};
    use std::collections::HashMap;

    #[test]
    fn test_field_in_match() {
        let condition = RuleCondition::FieldIn {
            field: "severity".to_string(),
            values: vec!["high".to_string(), "critical".to_string()],
        };
        let mut data = HashMap::new();
        data.insert("severity".to_string(), "high".to_string());
        assert!(condition.evaluate(&data));
    }

    #[test]
    fn test_field_in_no_match() {
        let condition = RuleCondition::FieldIn {
            field: "severity".to_string(),
            values: vec!["high".to_string(), "critical".to_string()],
        };
        let mut data = HashMap::new();
        data.insert("severity".to_string(), "low".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_field_in_missing_field() {
        let condition = RuleCondition::FieldIn {
            field: "severity".to_string(),
            values: vec!["high".to_string()],
        };
        let data = HashMap::new();
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_field_greater_than_match() {
        let condition = RuleCondition::FieldGreaterThan {
            field: "score".to_string(),
            value: 50.0,
        };
        let mut data = HashMap::new();
        data.insert("score".to_string(), "75".to_string());
        assert!(condition.evaluate(&data));
    }

    #[test]
    fn test_field_greater_than_no_match() {
        let condition = RuleCondition::FieldGreaterThan {
            field: "score".to_string(),
            value: 50.0,
        };
        let mut data = HashMap::new();
        data.insert("score".to_string(), "30".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_field_greater_than_not_number() {
        let condition = RuleCondition::FieldGreaterThan {
            field: "score".to_string(),
            value: 50.0,
        };
        let mut data = HashMap::new();
        data.insert("score".to_string(), "not_a_number".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_field_less_than_match() {
        let condition = RuleCondition::FieldLessThan {
            field: "latency".to_string(),
            value: 100.0,
        };
        let mut data = HashMap::new();
        data.insert("latency".to_string(), "50".to_string());
        assert!(condition.evaluate(&data));
    }

    #[test]
    fn test_field_less_than_no_match() {
        let condition = RuleCondition::FieldLessThan {
            field: "latency".to_string(),
            value: 100.0,
        };
        let mut data = HashMap::new();
        data.insert("latency".to_string(), "200".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_field_between_match() {
        let condition = RuleCondition::FieldBetween {
            field: "temp".to_string(),
            min: 20.0,
            max: 30.0,
        };
        let mut data = HashMap::new();
        data.insert("temp".to_string(), "25".to_string());
        assert!(condition.evaluate(&data));
    }

    #[test]
    fn test_field_between_no_match() {
        let condition = RuleCondition::FieldBetween {
            field: "temp".to_string(),
            min: 20.0,
            max: 30.0,
        };
        let mut data = HashMap::new();
        data.insert("temp".to_string(), "35".to_string());
        assert!(!condition.evaluate(&data));
    }

    #[test]
    fn test_field_between_boundary() {
        let condition = RuleCondition::FieldBetween {
            field: "val".to_string(),
            min: 10.0,
            max: 20.0,
        };
        let mut data = HashMap::new();
        data.insert("val".to_string(), "10".to_string());
        assert!(condition.evaluate(&data));
        data.insert("val".to_string(), "20".to_string());
        assert!(condition.evaluate(&data));
    }

    #[test]
    fn test_field_matches_returns_false() {
        let condition = RuleCondition::FieldMatches {
            field: "msg".to_string(),
            pattern: ".*error.*".to_string(),
        };
        let mut data = HashMap::new();
        data.insert("msg".to_string(), "an error occurred".to_string());
        assert!(!condition.evaluate(&data)); // Simplified to always false
    }

    #[test]
    fn test_frequency_threshold_returns_false() {
        let condition = RuleCondition::FrequencyThreshold {
            count: 5,
            window_minutes: 10,
        };
        assert!(!condition.evaluate(&HashMap::new()));
    }

    #[test]
    fn test_time_window_returns_false() {
        let condition = RuleCondition::TimeWindow {
            start_hour: 9,
            end_hour: 17,
        };
        assert!(!condition.evaluate(&HashMap::new()));
    }

    #[test]
    fn test_custom_condition_returns_false() {
        let condition = RuleCondition::Custom {
            name: "custom_check".to_string(),
            parameters: HashMap::new(),
        };
        assert!(!condition.evaluate(&HashMap::new()));
    }

    #[test]
    fn test_always_condition() {
        let condition = RuleCondition::Always;
        assert!(condition.evaluate(&HashMap::new()));
    }

    #[test]
    fn test_is_complex_custom() {
        let condition = RuleCondition::Custom {
            name: "test".to_string(),
            parameters: HashMap::new(),
        };
        assert!(condition.is_complex());
    }

    #[test]
    fn test_complexity_score_custom() {
        let condition = RuleCondition::Custom {
            name: "test".to_string(),
            parameters: HashMap::new(),
        };
        assert_eq!(condition.complexity_score(), 3);
    }

    #[test]
    fn test_complexity_score_frequency_threshold() {
        let condition = RuleCondition::FrequencyThreshold {
            count: 5,
            window_minutes: 10,
        };
        assert_eq!(condition.complexity_score(), 2);
    }

    #[test]
    fn test_referenced_fields_logical_not() {
        let condition = RuleCondition::logical_not(RuleCondition::field_equals("f1", "v1"));
        let fields = condition.referenced_fields();
        assert_eq!(fields, vec!["f1"]);
    }

    #[test]
    fn test_referenced_fields_always() {
        let condition = RuleCondition::Always;
        assert!(condition.referenced_fields().is_empty());
    }

    #[test]
    fn test_referenced_fields_field_in() {
        let condition = RuleCondition::FieldIn {
            field: "status".to_string(),
            values: vec!["a".to_string()],
        };
        let fields = condition.referenced_fields();
        assert_eq!(fields, vec!["status"]);
    }

    #[test]
    fn test_referenced_fields_field_between() {
        let condition = RuleCondition::FieldBetween {
            field: "temp".to_string(),
            min: 0.0,
            max: 100.0,
        };
        let fields = condition.referenced_fields();
        assert_eq!(fields, vec!["temp"]);
    }

    #[test]
    fn test_builder_single_condition_build_and() {
        let condition = ConditionBuilder::new()
            .field_equals("key", "value")
            .build_and();
        assert!(!condition.is_complex());
    }

    #[test]
    fn test_builder_single_condition_build_or() {
        let condition = ConditionBuilder::new().field_exists("key").build_or();
        assert!(!condition.is_complex());
    }

    #[test]
    fn test_builder_multiple_conditions_build_or() {
        let condition = ConditionBuilder::new()
            .field_equals("a", "1")
            .field_equals("b", "2")
            .build_or();
        assert!(condition.is_complex());
    }

    #[test]
    fn test_builder_add_condition() {
        let condition = ConditionBuilder::new()
            .add_condition(RuleCondition::Always)
            .add_condition(RuleCondition::field_exists("x"))
            .build_and();
        assert!(condition.is_complex());
    }

    #[test]
    fn test_builder_field_contains() {
        let condition = ConditionBuilder::new()
            .field_contains("msg", "error")
            .build_and();
        let mut data = HashMap::new();
        data.insert("msg".to_string(), "an error happened".to_string());
        assert!(condition.evaluate(&data));
    }

    #[test]
    fn test_logical_not_complexity() {
        let inner = RuleCondition::field_equals("a", "b");
        let not = RuleCondition::logical_not(inner);
        assert_eq!(not.complexity_score(), 2);
    }
}

// ============================================================================
// types/engine/ml_models.rs - 37 uncovered lines, 83.41% coverage
// ============================================================================

#[cfg(test)]
mod ml_models_gap_tests {
    use crate::threat::types::engine::ml_models::{
        ConfusionMatrix, MlModel, MlModelType, ModelPerformanceMetrics, ModelPrediction,
        PredictionValue,
    };

    #[test]
    fn test_model_summary() {
        let model = MlModel::new(
            "Summary Test",
            "desc",
            MlModelType::AnomalyDetection,
            vec!["f1".to_string(), "f2".to_string()],
        );
        let summary = model.summary();
        assert_eq!(summary.name, "Summary Test");
        assert_eq!(summary.model_type, MlModelType::AnomalyDetection);
        assert_eq!(summary.feature_count, 2);
        assert!(!summary.is_active);
    }

    #[test]
    fn test_model_get_age_days() {
        let model = MlModel::new("Age Test", "desc", MlModelType::Classification, vec![]);
        assert_eq!(model.get_age_days(), 0);
    }

    #[test]
    fn test_model_update_training_timestamp() {
        let mut model = MlModel::new("Train Test", "desc", MlModelType::Regression, vec![]);
        let before = model.last_trained;
        model.update_training_timestamp();
        assert!(model.last_trained >= before);
    }

    #[test]
    fn test_model_add_metadata() {
        let mut model = MlModel::new("Meta Test", "desc", MlModelType::Clustering, vec![]);
        model.add_metadata("framework", "pytorch");
        model.add_metadata("gpu", "A100");
        assert_eq!(model.metadata.get("framework"), Some(&"pytorch".to_string()));
        assert_eq!(model.metadata.len(), 2);
    }

    #[test]
    fn test_confusion_matrix_all_zeros() {
        let cm = ConfusionMatrix::default();
        assert_eq!(cm.accuracy(), 0.0);
        assert_eq!(cm.precision(), 0.0);
        assert_eq!(cm.recall(), 0.0);
        assert_eq!(cm.f1_score(), 0.0);
    }

    #[test]
    fn test_confusion_matrix_perfect() {
        let cm = ConfusionMatrix {
            true_positives: 100,
            true_negatives: 100,
            false_positives: 0,
            false_negatives: 0,
        };
        assert_eq!(cm.accuracy(), 1.0);
        assert_eq!(cm.precision(), 1.0);
        assert_eq!(cm.recall(), 1.0);
        assert_eq!(cm.f1_score(), 1.0);
    }

    #[test]
    fn test_prediction_value_variants() {
        let _ = PredictionValue::Binary(true);
        let _ = PredictionValue::Class("threat".to_string());
        let _ = PredictionValue::Numeric(0.95);
        let _ = PredictionValue::AnomalyScore(0.8);
        let _ = PredictionValue::Custom(serde_json::json!({"key": "value"}));

        let mut probs = std::collections::HashMap::new();
        probs.insert("class_a".to_string(), 0.7);
        let _ = PredictionValue::Probability(probs);
    }

    #[test]
    fn test_model_prediction_low_confidence() {
        let pred = ModelPrediction::new("m1", PredictionValue::Binary(false), 0.3);
        assert!(!pred.is_high_confidence());
    }

    #[test]
    fn test_model_prediction_add_feature_importance() {
        let mut pred = ModelPrediction::new("m1", PredictionValue::Binary(true), 0.9);
        pred.add_feature_importance("ip_address", 0.8);
        pred.add_feature_importance("user_agent", 0.6);
        assert_eq!(pred.feature_importance.len(), 2);
        assert_eq!(pred.feature_importance.get("ip_address"), Some(&0.8));
    }

    #[test]
    fn test_ml_model_type_display_all() {
        assert_eq!(MlModelType::Classification.to_string(), "Classification");
        assert_eq!(MlModelType::Regression.to_string(), "Regression");
        assert_eq!(MlModelType::Clustering.to_string(), "Clustering");
        assert_eq!(
            MlModelType::AnomalyDetection.to_string(),
            "Anomaly Detection"
        );
        assert_eq!(
            MlModelType::NaturalLanguageProcessing.to_string(),
            "Natural Language Processing"
        );
        assert_eq!(MlModelType::TimeSeries.to_string(), "Time Series");
        assert_eq!(MlModelType::DeepLearning.to_string(), "Deep Learning");
        assert_eq!(MlModelType::Ensemble.to_string(), "Ensemble");
        assert_eq!(
            MlModelType::Custom("BERT".to_string()).to_string(),
            "Custom: BERT"
        );
    }

    #[test]
    fn test_model_performance_metrics_default() {
        let metrics = ModelPerformanceMetrics::default();
        assert_eq!(metrics.accuracy, 0.0);
        assert_eq!(metrics.precision, 0.0);
        assert_eq!(metrics.recall, 0.0);
        assert_eq!(metrics.f1_score, 0.0);
        assert_eq!(metrics.specificity, 0.0);
        assert_eq!(metrics.sensitivity, 0.0);
        assert_eq!(metrics.auc_roc, 0.0);
        assert_eq!(metrics.training_time_ms, 0);
        assert_eq!(metrics.inference_time_ms, 0);
        assert_eq!(metrics.model_size_bytes, 0);
    }

    #[test]
    fn test_model_needs_retraining_high_accuracy() {
        let mut model = MlModel::new("Test", "desc", MlModelType::Classification, vec![]);
        model.update_accuracy(0.95);
        assert!(!model.needs_retraining());
    }
}

// ============================================================================
// types/engine/rules.rs - 59 uncovered lines
// ============================================================================

#[cfg(test)]
mod rules_gap_tests {
    use crate::threat::types::engine::conditions::RuleCondition;
    use crate::threat::types::engine::rules::{
        DetectionRule, RulePerformanceMetrics, ThreatRuleType,
    };
    use crate::threat::types::ThreatSeverity;
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
        assert!(validation.warnings.iter().any(|w| w.contains("description")));
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
    use crate::threat::types::{ThreatEvent, ThreatSeverity, ThreatSource, ThreatTarget, ThreatType};

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
        let debug = format!("{:?}", handler);
        assert!(debug.contains("AutomatedThreatResponseHandler"));
    }

    #[test]
    fn test_response_config_debug() {
        let config = ThreatResponseConfig {
            enabled: true,
            max_response_level: ThreatSeverity::High,
        };
        let debug = format!("{:?}", config);
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
    fn test_collect_forensics_returns_empty() {
        let handler = make_handler(true);
        let threat = make_threat();
        let result = handler.collect_forensics(&threat).unwrap();
        assert!(result.is_empty());
    }
}

// ============================================================================
// ml_engine.rs - 77 uncovered lines, 84.57% coverage
// ============================================================================

#[cfg(test)]
mod ml_engine_gap_tests {
    use crate::threat::ml_engine::{MlEngine, MlModel, UniversalComputeAdapter};
    use beardog_errors::BearDogError;
    use std::future::Future;
    use std::pin::Pin;

    struct MockAdapter;

    impl UniversalComputeAdapter for MockAdapter {
        fn request_compute(
            &self,
            _endpoint: &str,
            _request: &serde_json::Value,
        ) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, BearDogError>> + Send + '_>>
        {
            Box::pin(async {
                Ok(serde_json::json!({
                    "confidence": 0.92,
                    "risk_score": 0.85,
                    "reasoning": ["Network ML analysis", "Pattern matched"]
                }))
            })
        }
    }

    struct FailingAdapter;

    impl UniversalComputeAdapter for FailingAdapter {
        fn request_compute(
            &self,
            _endpoint: &str,
            _request: &serde_json::Value,
        ) -> Pin<Box<dyn Future<Output = Result<serde_json::Value, BearDogError>> + Send + '_>>
        {
            Box::pin(async {
                Err(BearDogError::unavailable("ML service unavailable".into()))
            })
        }
    }

    #[test]
    fn test_get_stats() {
        let mut engine = MlEngine::new();
        let model = MlModel {
            name: "test-model".to_string(),
            version: "1.0".to_string(),
            model_type: "neural_network".to_string(),
            accuracy: 0.95,
            last_updated: chrono::Utc::now(),
        };
        engine.add_model(model);

        let stats = engine.get_stats();
        assert_eq!(stats.local_predictions, 0);
        assert_eq!(stats.network_predictions, 0);
        assert_eq!(stats.models_loaded, 1);
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let engine = MlEngine::new();

        // Predict something to populate cache
        let event = crate::threat::types::SecurityEvent {
            id: "cache-test".to_string(),
            event_type: "test_event".to_string(),
            severity: crate::threat::types::ThreatSeverity::Low,
            timestamp: chrono::Utc::now().into(),
            source: "test".to_string(),
            description: "test".to_string(),
            data: std::collections::HashMap::new(),
        };
        engine.predict_threat(&event).await.unwrap();

        // Clear the cache - exercises the code path
        engine.clear_cache().await;

        // After clearing, a new prediction should still work
        let pred = engine.predict_threat(&event).await.unwrap();
        assert!(pred.confidence > 0.0);
    }

    #[tokio::test]
    async fn test_predict_via_universal_adapter_success() {
        let engine = MlEngine::new().with_universal_adapter(Box::new(MockAdapter));

        let event = crate::threat::types::SecurityEvent {
            id: "adapter-test".to_string(),
            event_type: "network_scan".to_string(),
            severity: crate::threat::types::ThreatSeverity::High,
            timestamp: chrono::Utc::now().into(),
            source: "scanner".to_string(),
            description: "scan detected".to_string(),
            data: std::collections::HashMap::new(),
        };

        let prediction = engine.predict_threat(&event).await.unwrap();
        assert_eq!(prediction.confidence, 0.92);
        assert_eq!(prediction.model_version, "beardog-network-v2.0");
        assert!(prediction.reasoning.iter().any(|r| r.contains("Network")));
    }

    #[tokio::test]
    async fn test_predict_via_universal_adapter_fallback() {
        let engine = MlEngine::new().with_universal_adapter(Box::new(FailingAdapter));

        let event = crate::threat::types::SecurityEvent {
            id: "fallback-test".to_string(),
            event_type: "auth_failure".to_string(),
            severity: crate::threat::types::ThreatSeverity::Medium,
            timestamp: chrono::Utc::now().into(),
            source: "auth".to_string(),
            description: "auth failed".to_string(),
            data: std::collections::HashMap::new(),
        };

        // Should fallback to local prediction
        let prediction = engine.predict_threat(&event).await.unwrap();
        assert_eq!(prediction.model_version, "beardog-local-v1.0");
        assert_eq!(prediction.confidence, 0.75);
    }
}

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
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default()).unwrap();
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
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default()).unwrap();
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
        assert_eq!(engine.threat_feeds.get("feed-1").unwrap().name, "New Name");
    }

    #[test]
    fn test_engine_remove_rule() {
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default()).unwrap();
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
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default()).unwrap();
        assert!(!engine.remove_rule("nonexistent"));
    }

    #[test]
    fn test_engine_add_ml_model() {
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default()).unwrap();
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
        let engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default()).unwrap();
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

// ============================================================================
// handlers/core.rs - 5 uncovered lines, 80.77% coverage
// ============================================================================

#[cfg(test)]
mod core_engine_gap_tests {
    use crate::threat::handlers::core::ThreatDetectionEngine;
    use beardog_types::canonical::config::domains::threat::ThreatDetectionConfig;

    #[test]
    fn test_engine_with_ml_enabled() {
        let mut config = ThreatDetectionConfig::default();
        config.ml_enhancement = true;
        let engine = ThreatDetectionEngine::new(config).unwrap();
        assert!(engine.ml_engine.is_some());
    }

    #[test]
    fn test_engine_without_ml() {
        let mut config = ThreatDetectionConfig::default();
        config.ml_enhancement = false;
        let engine = ThreatDetectionEngine::new(config).unwrap();
        assert!(engine.ml_engine.is_none());
    }

    #[test]
    fn test_engine_remove_rule_delegates() {
        let mut engine = ThreatDetectionEngine::new(ThreatDetectionConfig::default()).unwrap();
        let result = engine.remove_rule("nonexistent");
        assert!(!result);
    }
}
