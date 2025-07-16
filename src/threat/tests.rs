//! Threat detection tests
//!
//! Comprehensive tests for the threat detection and response system.

use crate::threat::types::*;
use crate::threat::handlers::core::ThreatDetectionEngine;

#[tokio::test]
async fn test_threat_detection_engine_creation() {
    let config = ThreatDetectionConfig::default();
    let engine = ThreatDetectionEngine::new(config).await.unwrap();
    
    // Basic validation that the engine was created successfully
    assert!(engine.active_threats.is_empty());
    assert!(engine.blocked_sources.is_empty());
    assert!(engine.quarantined_systems.is_empty());
}

#[tokio::test]
async fn test_threat_severity_scoring() {
    assert_eq!(ThreatSeverity::Critical.to_score(), 100);
    assert_eq!(ThreatSeverity::High.to_score(), 80);
    assert_eq!(ThreatSeverity::Medium.to_score(), 50);
    assert_eq!(ThreatSeverity::Low.to_score(), 30);
    assert_eq!(ThreatSeverity::Info.to_score(), 10);
}

#[tokio::test]
async fn test_threat_type_display() {
    assert_eq!(ThreatType::Malware.to_string(), "Malware");
    assert_eq!(ThreatType::Phishing.to_string(), "Phishing");
    assert_eq!(ThreatType::BruteForceAttack.to_string(), "Brute Force Attack");
}

#[tokio::test]
async fn test_threat_detection_rule_creation() {
    let rule = ThreatDetectionRule {
        rule_id: "test-rule".to_string(),
        name: "Test Rule".to_string(),
        description: "A test rule for malware detection".to_string(),
        enabled: true,
        threat_type: ThreatType::Malware,
        severity: ThreatSeverity::High,
        conditions: vec![RuleCondition::FieldEquals {
            field: "file_type".to_string(),
            value: "executable".to_string(),
        }],
        response_actions: vec![ResponseAction::LogAlert("Test rule triggered".to_string())],
        false_positive_rate: 0.01,
        mitre_techniques: vec!["T1566".to_string()],
        detection_count: 0,
        false_positive_count: 0,
    };

    assert_eq!(rule.rule_id, "test-rule");
    assert_eq!(rule.threat_type, ThreatType::Malware);
    assert!(rule.enabled);
}

#[tokio::test]
async fn test_threat_event_creation() {
    let high_priority_threat = ThreatEvent {
        id: "threat-001".to_string(),
        threat_type: ThreatType::Malware,
        severity: ThreatSeverity::Critical,
        score: 95,
        description: "Critical malware detected".to_string(),
        timestamp: chrono::Utc::now(),
        source: ThreatSource {
            id: "source-001".to_string(),
            ip_address: Some("192.168.1.100".to_string()),
            hostname: Some("workstation-01".to_string()),
            user_agent: None,
            reputation_score: 0.2,
            threat_actor: Some("APT29".to_string()),
            classification: SourceClassification::Malicious,
            confidence_score: 0.8,
            first_seen: Some(chrono::Utc::now()),
            last_seen: Some(chrono::Utc::now()),
            threat_score: 80.0,
            geolocation: None,
        },
        target: ThreatTarget {
            id: "target-001".to_string(),
            resource_id: "prod-server-01".to_string(),
            resource_type: "server".to_string(),
            node_id: Some("node-001".to_string()),
            user_account: None,
            ip_address: Some("10.0.0.100".to_string()),
            hostname: Some("prod-server-01".to_string()),
            service: Some("web-server".to_string()),
            port: Some(80),
            protocol: Some("HTTP".to_string()),
            criticality: AssetCriticality::High,
            protection_level: ProtectionLevel::Enhanced,
        },
        detection_method: DetectionMethod::Signature,
        evidence: vec![],
        recommended_actions: vec![],
        status: ThreatStatus::New,
        assigned_analyst: None,
        related_events: vec![],
        mitigation_steps: vec![],
    };

    assert_eq!(high_priority_threat.threat_type, ThreatType::Malware);
    assert_eq!(high_priority_threat.severity, ThreatSeverity::Critical);
    assert_eq!(high_priority_threat.score, 95);
    assert!(high_priority_threat.is_high_priority());
}

#[tokio::test]
async fn test_threat_detection_engine_with_config() {
    let config = ThreatDetectionConfig {
        enabled: true,
        real_time_detection: true,
        threat_threshold: 70,
        automated_response: true,
        max_alerts_per_minute: 100,
        ml_enhancement: false, // Disable ML for simpler test
        threat_feeds: vec![],
        auto_quarantine: true,
        notification_endpoints: vec![],
        rules_path: crate::config::constants::storage::DEFAULT_RULES_PATH.to_string(),
        monitor_paths: vec![crate::config::constants::storage::DEFAULT_MONITOR_PATH.to_string()],
        alert_threshold: 0.8,
        cache_size: crate::config::constants::performance::DEFAULT_CACHE_SIZE,
        monitoring_interval: 60,
    };
    
    let engine = ThreatDetectionEngine::new(config).await.unwrap();
    
    // Verify configuration was applied
    assert!(engine.config.enabled);
    assert!(engine.config.real_time_detection);
    assert_eq!(engine.config.threat_threshold, 70);
    assert!(engine.config.automated_response);
}

#[tokio::test]
async fn test_threat_indicator_creation() {
    let indicator = ThreatIndicator {
        indicator_type: IndicatorType::IpAddress,
        value: "192.168.1.100".to_string(),
        confidence: 0.8,
        confidence_score: 0.8,
        description: "Known malicious IP address".to_string(),
        first_seen: chrono::Utc::now(),
        last_seen: chrono::Utc::now(),
        threat_types: vec![ThreatType::Malware],
        threat_type: ThreatType::Malware,
        severity: ThreatSeverity::High,
        tags: vec!["malware".to_string()],
        threat_actor: Some("APT29".to_string()),
    };

    assert_eq!(indicator.value, "192.168.1.100");
    assert!(indicator.threat_types.contains(&ThreatType::Malware));
    assert_eq!(indicator.confidence, 0.8);
}

#[tokio::test]
async fn test_threat_event_priority_checking() {
    let high_priority_threat = ThreatEvent {
        id: "threat-high".to_string(),
        threat_type: ThreatType::Malware,
        severity: ThreatSeverity::Critical,
        score: 95,
        description: "Critical threat for priority testing".to_string(),
        timestamp: chrono::Utc::now(),
        source: ThreatSource {
            id: "source-test".to_string(),
            ip_address: Some("192.168.1.200".to_string()),
            hostname: Some("test-host".to_string()),
            user_agent: None,
            reputation_score: 0.1,
            threat_actor: None,
            classification: SourceClassification::Malicious,
            confidence_score: 0.9,
            first_seen: Some(chrono::Utc::now()),
            last_seen: Some(chrono::Utc::now()),
            threat_score: 90.0,
            geolocation: None,
        },
        target: ThreatTarget {
            id: "target-test".to_string(),
            resource_id: "test-server".to_string(),
            resource_type: "server".to_string(),
            node_id: Some("node-test".to_string()),
            user_account: None,
            ip_address: Some("10.0.0.200".to_string()),
            hostname: Some("test-server".to_string()),
            service: Some("test-service".to_string()),
            port: Some(443),
            protocol: Some("HTTPS".to_string()),
            criticality: AssetCriticality::High,
            protection_level: ProtectionLevel::Enhanced,
        },
        detection_method: DetectionMethod::Signature,
        evidence: vec![],
        recommended_actions: vec![],
        status: ThreatStatus::New,
        assigned_analyst: None,
        related_events: vec![],
        mitigation_steps: vec![],
    };

    // Test that high priority threats are properly identified
    assert!(high_priority_threat.is_high_priority());
    assert_eq!(high_priority_threat.severity, ThreatSeverity::Critical);
    assert_eq!(high_priority_threat.score, 95);
}
