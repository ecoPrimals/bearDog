// SPDX-License-Identifier: AGPL-3.0-or-later

// ============================================================================
// types/mod.rs - 63 uncovered lines, 65.19% coverage
// ============================================================================

#[cfg(test)]
mod types_mod_tests {
    use crate::threat::types::{
        DetectionMethod, DetectionRule, MitigationStep, RuleCondition, SecurityEvent, ThreatAction,
        ThreatEvent, ThreatRuleType, ThreatSeverity, ThreatSource, ThreatStatus, ThreatTarget,
        ThreatType,
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
