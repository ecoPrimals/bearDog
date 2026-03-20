// SPDX-License-Identifier: AGPL-3.0-only

#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

// Comprehensive tests for all threat handlers
//
// This module provides comprehensive test coverage for analysis, enrichment, incident, and response handlers.

use super::analysis::{
    ThreatAnalysisMetrics, ThreatAnalysisResult, ThreatDetectionEngine as AnalysisEngine,
};
use super::core::ThreatDetectionEngine as CoreEngine;
use super::enrichment::{ExternalIntelligence, NetworkContextInfo, ThreatEnrichmentHandler};
use super::incident::IncidentResponse;
use super::response::{AutomatedThreatResponseHandler, ThreatResponseConfig};
use crate::threat::types::{
    DetectionRule, MitigationStep, RuleCondition, ThreatEvent, ThreatRuleType, ThreatSeverity,
    ThreatSource, ThreatTarget, ThreatType,
};
use beardog_types::canonical::config::domains::threat::ThreatDetectionConfig;

// ============================================================================
// ANALYSIS HANDLER TESTS
// ============================================================================

#[cfg(test)]
mod analysis_tests {
    use super::*;

    #[test]
    fn test_threat_detection_engine_new() {
        let config = ThreatDetectionConfig::default();
        let engine = AnalysisEngine::new(config);
        assert_eq!(engine.detection_rules.len(), 0);
        assert_eq!(engine.threat_signatures.len(), 0);
        assert_eq!(engine.metrics.analyses_performed, 0);
        assert_eq!(engine.metrics.threats_detected, 0);
    }

    #[test]
    fn test_threat_detection_engine_default() {
        let engine = AnalysisEngine::default();
        assert_eq!(engine.detection_rules.len(), 0);
        assert_eq!(engine.threat_signatures.len(), 0);
    }

    #[test]
    fn test_add_detection_rule() {
        let mut engine = AnalysisEngine::default();
        let rule = DetectionRule {
            id: "rule1".to_string(),
            name: "Test Rule".to_string(),
            description: "Test rule for malware detection".to_string(),
            pattern: "malware_pattern".to_string(),
            severity: ThreatSeverity::High,
            enabled: true,
            confidence: 0.9,
            condition: RuleCondition::FieldEquals {
                field: "type".to_string(),
                value: "malware".to_string(),
            },
            rule_type: ThreatRuleType::Signature,
        };
        engine.add_detection_rule(rule);
        assert_eq!(engine.detection_rules.len(), 1);
        assert_eq!(engine.detection_rules[0].id, "rule1");
    }

    #[test]
    fn test_add_multiple_detection_rules() {
        let mut engine = AnalysisEngine::default();
        for i in 1..=5 {
            let rule = DetectionRule {
                id: format!("rule{i}"),
                name: format!("Test Rule {i}"),
                description: "Test rule".to_string(),
                pattern: "pattern".to_string(),
                severity: ThreatSeverity::Medium,
                enabled: true,
                confidence: 0.8,
                condition: RuleCondition::FieldEquals {
                    field: "field".to_string(),
                    value: "value".to_string(),
                },
                rule_type: ThreatRuleType::Signature,
            };
            engine.add_detection_rule(rule);
        }
        assert_eq!(engine.detection_rules.len(), 5);
    }

    #[test]
    fn test_remove_rule_exists() {
        let mut engine = AnalysisEngine::default();
        let rule = DetectionRule {
            id: "rule_to_remove".to_string(),
            name: "Remove Me".to_string(),
            description: "".to_string(),
            pattern: "".to_string(),
            severity: ThreatSeverity::Low,
            enabled: true,
            confidence: 0.5,
            condition: RuleCondition::FieldEquals {
                field: "".to_string(),
                value: "".to_string(),
            },
            rule_type: ThreatRuleType::Signature,
        };
        engine.add_detection_rule(rule);
        assert_eq!(engine.detection_rules.len(), 1);

        let removed = engine.remove_rule("rule_to_remove");
        assert!(removed);
        assert_eq!(engine.detection_rules.len(), 0);
    }

    #[test]
    fn test_remove_rule_not_exists() {
        let mut engine = AnalysisEngine::default();
        let removed = engine.remove_rule("nonexistent_rule");
        assert!(!removed);
    }

    #[test]
    fn test_remove_rule_from_multiple() {
        let mut engine = AnalysisEngine::default();
        for i in 1..=3 {
            let rule = DetectionRule {
                id: format!("rule{i}"),
                name: format!("Rule {i}"),
                description: "".to_string(),
                pattern: "".to_string(),
                severity: ThreatSeverity::Low,
                enabled: true,
                confidence: 0.5,
                condition: RuleCondition::FieldEquals {
                    field: "".to_string(),
                    value: "".to_string(),
                },
                rule_type: ThreatRuleType::Signature,
            };
            engine.add_detection_rule(rule);
        }
        assert_eq!(engine.detection_rules.len(), 3);

        let removed = engine.remove_rule("rule2");
        assert!(removed);
        assert_eq!(engine.detection_rules.len(), 2);
        assert!(engine.detection_rules.iter().all(|r| r.id != "rule2"));
    }

    #[test]
    fn test_update_threat_signatures() {
        let mut engine = AnalysisEngine::default();
        let mut signatures = std::collections::HashMap::new();
        signatures.insert("malware1".to_string(), "hash123".to_string());
        signatures.insert("malware2".to_string(), "hash456".to_string());

        let result = engine.update_threat_signatures(signatures);
        assert!(result.is_ok());
        assert_eq!(engine.threat_signatures.len(), 2);
        assert_eq!(
            engine.threat_signatures.get("malware1"),
            Some(&"hash123".to_string())
        );
    }

    #[test]
    fn test_update_threat_signatures_extends() {
        let mut engine = AnalysisEngine::default();
        let mut signatures1 = std::collections::HashMap::new();
        signatures1.insert("sig1".to_string(), "val1".to_string());
        engine.update_threat_signatures(signatures1).unwrap();

        let mut signatures2 = std::collections::HashMap::new();
        signatures2.insert("sig2".to_string(), "val2".to_string());
        engine.update_threat_signatures(signatures2).unwrap();

        assert_eq!(engine.threat_signatures.len(), 2);
    }

    #[test]
    fn test_analyze_threat_small_data() {
        let mut engine = AnalysisEngine::default();
        let small_data = vec![0u8; 500]; // Less than 1000 bytes

        let result = engine.analyze_threat(&small_data);
        assert!(result.is_ok());

        let analysis = result.unwrap();
        assert!(!analysis.threat_detected); // Small data should not trigger threat
        assert_eq!(analysis.confidence_score, 0.1);
        assert_eq!(analysis.threat_type, ThreatType::Unknown);
        assert_eq!(engine.metrics.analyses_performed, 1);
        assert_eq!(engine.metrics.threats_detected, 0);
    }

    #[test]
    fn test_analyze_threat_large_data() {
        let mut engine = AnalysisEngine::default();
        let large_data = vec![0u8; 2000]; // More than 1000 bytes

        let result = engine.analyze_threat(&large_data);
        assert!(result.is_ok());

        let analysis = result.unwrap();
        assert!(analysis.threat_detected); // Large data triggers threat
        assert_eq!(analysis.confidence_score, 0.8);
        assert_eq!(analysis.threat_type, ThreatType::Malware);
        assert_eq!(engine.metrics.analyses_performed, 1);
        assert_eq!(engine.metrics.threats_detected, 1);
    }

    #[test]
    fn test_analyze_threat_boundary() {
        let mut engine = AnalysisEngine::default();
        let boundary_data = vec![0u8; 1000]; // Exactly 1000 bytes

        let result = engine.analyze_threat(&boundary_data);
        assert!(result.is_ok());
        assert!(!result.unwrap().threat_detected); // Boundary case: not > 1000
    }

    #[test]
    fn test_analyze_multiple_threats() {
        let mut engine = AnalysisEngine::default();

        // Analyze 3 threats
        engine.analyze_threat(&vec![0u8; 500]).unwrap();
        engine.analyze_threat(&vec![0u8; 2000]).unwrap();
        engine.analyze_threat(&vec![0u8; 3000]).unwrap();

        assert_eq!(engine.metrics.analyses_performed, 3);
        assert_eq!(engine.metrics.threats_detected, 2); // Only 2 were large enough
    }

    #[test]
    fn test_threat_analysis_metrics_default() {
        let metrics = ThreatAnalysisMetrics::default();
        assert_eq!(metrics.analyses_performed, 0);
        assert_eq!(metrics.threats_detected, 0);
        assert_eq!(metrics.false_positives, 0);
        assert_eq!(metrics.detection_accuracy, 0.0);
    }

    #[test]
    fn test_threat_analysis_result_properties() {
        let result = ThreatAnalysisResult {
            threat_detected: true,
            confidence_score: 0.95,
            threat_type: ThreatType::Malware,
            details: "High confidence malware detection".to_string(),
        };

        assert!(result.threat_detected);
        assert_eq!(result.confidence_score, 0.95);
        assert_eq!(result.threat_type, ThreatType::Malware);
        assert!(result.details.contains("malware"));
    }
}

// ============================================================================
// ENRICHMENT HANDLER TESTS
// ============================================================================

#[cfg(test)]
mod enrichment_tests {
    use super::*;

    #[test]
    fn test_threat_enrichment_handler_new() {
        let handler = ThreatEnrichmentHandler::new();
        assert!(handler.enabled);
    }

    #[test]
    fn test_threat_enrichment_handler_default() {
        let handler = ThreatEnrichmentHandler::default();
        assert!(handler.enabled);
    }

    #[test]
    fn test_enrich_threat_enabled() {
        let handler = ThreatEnrichmentHandler::new();
        let result = handler.enrich_threat("threat_123");

        assert!(result.is_ok());
        let intelligence = result.unwrap();
        assert!(intelligence.is_some());

        let intel = intelligence.unwrap();
        assert_eq!(intel.description, "enriched");
        assert_eq!(intel.confidence, 0.8);
    }

    #[test]
    fn test_enrich_threat_disabled() {
        let mut handler = ThreatEnrichmentHandler::new();
        handler.enabled = false;

        let result = handler.enrich_threat("threat_456");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // Returns None when disabled
    }

    #[test]
    fn test_enrich_multiple_threats() {
        let handler = ThreatEnrichmentHandler::new();
        for i in 1..=5 {
            let result = handler.enrich_threat(&format!("threat_{i}"));
            assert!(result.is_ok());
            assert!(result.unwrap().is_some());
        }
    }

    #[test]
    fn test_network_context_info_default() {
        let context = NetworkContextInfo::default();
        assert_eq!(context.network_segment, "unknown");
        assert_eq!(context.security_zone, "default");
        assert_eq!(context.access_level, "standard");
        assert!(context.is_trusted);
    }

    #[test]
    fn test_network_context_info_custom() {
        let context = NetworkContextInfo {
            network_segment: "dmz".to_string(),
            security_zone: "restricted".to_string(),
            access_level: "high".to_string(),
            is_trusted: false,
        };

        assert_eq!(context.network_segment, "dmz");
        assert_eq!(context.security_zone, "restricted");
        assert!(!context.is_trusted);
    }

    #[test]
    fn test_external_intelligence_default() {
        let intel = ExternalIntelligence::default();
        assert_eq!(intel.description, "none");
        assert_eq!(intel.confidence, 0.0);
    }

    #[test]
    fn test_external_intelligence_custom() {
        let intel = ExternalIntelligence {
            description: "Known APT group signature".to_string(),
            confidence: 0.95,
        };

        assert_eq!(intel.description, "Known APT group signature");
        assert_eq!(intel.confidence, 0.95);
    }
}

// ============================================================================
// INCIDENT HANDLER TESTS
// ============================================================================

#[cfg(test)]
mod incident_tests {
    use super::*;

    fn create_test_threat_event() -> ThreatEvent {
        ThreatEvent::new(
            "threat_1".to_string(),
            ThreatType::Malware,
            ThreatSeverity::High,
            ThreatSource::default(),
            ThreatTarget::default(),
        )
    }

    #[test]
    fn test_create_incident_from_threat() {
        let engine = CoreEngine::new(ThreatDetectionConfig::default()).unwrap();
        let threat = create_test_threat_event();

        let result = engine.create_incident_from_threat(&threat);
        assert!(result.is_ok());

        let incident_id = result.unwrap();
        assert!(incident_id.starts_with("INC-"));
        assert!(incident_id.len() > 4); // Should have UUID appended
    }

    #[test]
    fn test_create_multiple_incidents() {
        let engine = CoreEngine::new(ThreatDetectionConfig::default()).unwrap();
        let mut incident_ids = Vec::new();

        for i in 1..=5 {
            let mut threat = create_test_threat_event();
            threat.id = format!("threat_{i}");
            let incident_id = engine.create_incident_from_threat(&threat).unwrap();
            incident_ids.push(incident_id);
        }

        assert_eq!(incident_ids.len(), 5);
        // All IDs should be unique
        let unique_ids: std::collections::HashSet<_> = incident_ids.iter().collect();
        assert_eq!(unique_ids.len(), 5);
    }

    #[test]
    fn test_assign_incident_to_team() {
        let engine = CoreEngine::new(ThreatDetectionConfig::default()).unwrap();
        let result = engine.assign_incident_to_team("INC-123", "security_team");
        assert!(result.is_ok());
    }

    #[test]
    fn test_assign_incident_different_teams() {
        let engine = CoreEngine::new(ThreatDetectionConfig::default()).unwrap();
        assert!(
            engine
                .assign_incident_to_team("INC-1", "team_alpha")
                .is_ok()
        );
        assert!(engine.assign_incident_to_team("INC-2", "team_beta").is_ok());
        assert!(
            engine
                .assign_incident_to_team("INC-3", "team_gamma")
                .is_ok()
        );
    }

    #[test]
    fn test_update_incident_status_to_open() {
        let engine = CoreEngine::new(ThreatDetectionConfig::default()).unwrap();
        let result = engine.update_incident_status("INC-456", "open");
        assert!(result.is_ok());

        let incident = result.unwrap();
        assert!(incident.is_some());
        assert_eq!(incident.unwrap().status, "open");
    }

    #[test]
    fn test_update_incident_status_to_in_progress() {
        let engine = CoreEngine::new(ThreatDetectionConfig::default()).unwrap();
        let result = engine.update_incident_status("INC-789", "in_progress");
        assert!(result.is_ok());

        let incident = result.unwrap();
        assert!(incident.is_some());
        assert_eq!(incident.unwrap().status, "in_progress");
    }

    #[test]
    fn test_update_incident_status_to_resolved() {
        let engine = CoreEngine::new(ThreatDetectionConfig::default()).unwrap();
        let result = engine.update_incident_status("INC-999", "resolved");
        assert!(result.is_ok());

        let incident = result.unwrap();
        assert!(incident.is_some());
        assert_eq!(incident.unwrap().status, "resolved");
    }

    #[test]
    fn test_update_incident_status_returns_incident() {
        let engine = CoreEngine::new(ThreatDetectionConfig::default()).unwrap();
        let result = engine.update_incident_status("INC-111", "closed").unwrap();

        assert!(result.is_some());
        let incident = result.unwrap();
        assert_eq!(incident.id, "INC-111");
        assert_eq!(incident.status, "closed");
        assert_eq!(incident.priority, "medium");
    }

    #[test]
    fn test_get_active_incidents_empty() {
        let engine = CoreEngine::new(ThreatDetectionConfig::default()).unwrap();
        let result = engine.get_active_incidents();
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_close_incident() {
        let engine = CoreEngine::new(ThreatDetectionConfig::default()).unwrap();
        let result = engine.close_incident("INC-222");
        assert!(result.is_ok());
    }

    #[test]
    fn test_close_multiple_incidents() {
        let engine = CoreEngine::new(ThreatDetectionConfig::default()).unwrap();
        for i in 1..=3 {
            let result = engine.close_incident(&format!("INC-{i}"));
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_incident_response_structure() {
        let now = chrono::Utc::now();
        let incident = IncidentResponse {
            id: "INC-TEST".to_string(),
            threat_id: "threat_test".to_string(),
            status: "investigating".to_string(),
            priority: "high".to_string(),
            assigned_team: Some("red_team".to_string()),
            created_at: now,
            updated_at: now,
            estimated_cost: Some(5000.0),
            actual_cost: Some(4500.0),
            containment_actions: vec!["isolated_network".to_string()],
            remediation_actions: vec!["patched_system".to_string()],
            lessons_learned: vec!["improve_monitoring".to_string()],
            severity: "high".to_string(),
        };

        assert_eq!(incident.id, "INC-TEST");
        assert_eq!(incident.priority, "high");
        assert_eq!(incident.assigned_team, Some("red_team".to_string()));
        assert_eq!(incident.containment_actions.len(), 1);
        assert_eq!(incident.remediation_actions.len(), 1);
        assert_eq!(incident.lessons_learned.len(), 1);
    }
}

// ============================================================================
// RESPONSE HANDLER TESTS
// ============================================================================

#[cfg(test)]
mod response_tests {
    use super::*;

    fn create_test_threat_event() -> ThreatEvent {
        ThreatEvent::new(
            "threat_resp_1".to_string(),
            ThreatType::Intrusion,
            ThreatSeverity::Critical,
            ThreatSource::default(),
            ThreatTarget::default(),
        )
    }

    fn create_test_config() -> ThreatResponseConfig {
        ThreatResponseConfig {
            enabled: true,
            max_response_level: ThreatSeverity::Critical,
        }
    }

    #[test]
    fn test_automated_response_handler_new() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);
        assert!(handler.config.enabled);
        assert_eq!(handler.config.max_response_level, ThreatSeverity::Critical);
    }

    #[test]
    fn test_handle_threat_event_enabled() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);
        let threat = create_test_threat_event();

        let result = handler.handle_threat_event(&threat);
        assert!(result.is_ok());
    }

    #[test]
    fn test_handle_threat_event_disabled() {
        let config = ThreatResponseConfig {
            enabled: false,
            max_response_level: ThreatSeverity::Low,
        };
        let handler = AutomatedThreatResponseHandler::new(config);
        let threat = create_test_threat_event();

        let result = handler.handle_threat_event(&threat);
        assert!(result.is_ok()); // Should still succeed but do nothing
    }

    #[test]
    fn test_handle_multiple_threat_events() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);

        for i in 1..=5 {
            let mut threat = create_test_threat_event();
            threat.id = format!("threat_{i}");
            let result = handler.handle_threat_event(&threat);
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_log_threat_event() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);
        let threat = create_test_threat_event();

        let result = handler.log_threat_event(&threat).await;
        assert!(result.is_ok());

        // Verify event was logged
        let history = handler.event_history.read().await;
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].id, "threat_resp_1");
    }

    #[tokio::test]
    async fn test_log_multiple_threat_events() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);

        for i in 1..=3 {
            let mut threat = create_test_threat_event();
            threat.id = format!("threat_log_{i}");
            handler.log_threat_event(&threat).await.unwrap();
        }

        let history = handler.event_history.read().await;
        assert_eq!(history.len(), 3);
        assert_eq!(history[0].id, "threat_log_1");
        assert_eq!(history[1].id, "threat_log_2");
        assert_eq!(history[2].id, "threat_log_3");
    }

    #[tokio::test]
    async fn test_log_threat_event_preserves_data() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);
        let threat = create_test_threat_event();

        handler.log_threat_event(&threat).await.unwrap();

        let history = handler.event_history.read().await;
        assert_eq!(history[0].threat_type, ThreatType::Intrusion);
        assert_eq!(history[0].severity, ThreatSeverity::Critical);
    }

    #[test]
    fn test_update_threat_intelligence() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);
        let threat = create_test_threat_event();

        let result = handler.update_threat_intelligence(&threat);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_threat_intelligence_multiple() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);

        for i in 1..=5 {
            let mut threat = create_test_threat_event();
            threat.id = format!("intel_{i}");
            assert!(handler.update_threat_intelligence(&threat).is_ok());
        }
    }

    #[test]
    fn test_enable_enhanced_monitoring() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);
        let threat = create_test_threat_event();

        let result = handler.enable_enhanced_monitoring(&threat);
        assert!(result.is_ok());
    }

    #[test]
    fn test_collect_forensics() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);
        let threat = create_test_threat_event();

        let result = handler.collect_forensics(&threat);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0); // Simplified implementation returns empty vec
    }

    #[test]
    fn test_threat_response_config() {
        let config = ThreatResponseConfig {
            enabled: false,
            max_response_level: ThreatSeverity::Medium,
        };

        assert!(!config.enabled);
        assert_eq!(config.max_response_level, ThreatSeverity::Medium);
    }

    #[test]
    fn test_response_handler_with_different_severity_levels() {
        for severity in [
            ThreatSeverity::Low,
            ThreatSeverity::Medium,
            ThreatSeverity::High,
            ThreatSeverity::Critical,
        ] {
            let config = ThreatResponseConfig {
                enabled: true,
                max_response_level: severity.clone(),
            };
            let handler = AutomatedThreatResponseHandler::new(config);
            assert_eq!(handler.config.max_response_level, severity);
        }
    }

    #[test]
    fn test_handle_threat_event_with_mitigation_steps() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);
        let mut threat = create_test_threat_event();

        // Add mitigation steps
        threat.add_mitigation_step(MitigationStep::new(
            "step1".to_string(),
            "isolate_system".to_string(),
            "success".to_string(),
            true,
        ));

        let result = handler.handle_threat_event(&threat);
        assert!(result.is_ok());
    }

    #[test]
    fn test_collect_forensics_multiple_threats() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);

        for i in 1..=5 {
            let mut threat = create_test_threat_event();
            threat.id = format!("forensic_{i}");
            let result = handler.collect_forensics(&threat);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().len(), 0);
        }
    }

    #[test]
    fn test_enable_enhanced_monitoring_multiple_threats() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);

        for severity in [
            ThreatSeverity::Low,
            ThreatSeverity::Medium,
            ThreatSeverity::High,
            ThreatSeverity::Critical,
        ] {
            let threat = ThreatEvent::new(
                format!("threat_{:?}", severity),
                ThreatType::Intrusion,
                severity,
                ThreatSource::default(),
                ThreatTarget::default(),
            );
            assert!(handler.enable_enhanced_monitoring(&threat).is_ok());
        }
    }

    #[tokio::test]
    async fn test_log_and_handle_combined() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);
        let threat = create_test_threat_event();

        // Log the threat
        handler.log_threat_event(&threat).await.unwrap();

        // Handle the threat
        handler.handle_threat_event(&threat).unwrap();

        // Verify logging worked
        let history = handler.event_history.read().await;
        assert_eq!(history.len(), 1);
    }

    #[test]
    fn test_threat_response_with_low_severity() {
        let config = ThreatResponseConfig {
            enabled: true,
            max_response_level: ThreatSeverity::Low,
        };
        let handler = AutomatedThreatResponseHandler::new(config);
        let threat = ThreatEvent::new(
            "low_threat".to_string(),
            ThreatType::Suspicious,
            ThreatSeverity::Low,
            ThreatSource::default(),
            ThreatTarget::default(),
        );

        assert!(handler.handle_threat_event(&threat).is_ok());
        assert!(handler.update_threat_intelligence(&threat).is_ok());
    }

    #[tokio::test]
    async fn test_event_history_ordering() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);

        // Log threats in specific order
        for i in 1..=5 {
            let threat = ThreatEvent::new(
                format!("ordered_{i}"),
                ThreatType::Intrusion,
                ThreatSeverity::Medium,
                ThreatSource::default(),
                ThreatTarget::default(),
            );
            handler.log_threat_event(&threat).await.unwrap();
        }

        // Verify order is preserved
        let history = handler.event_history.read().await;
        assert_eq!(history.len(), 5);
        for i in 0..5 {
            assert_eq!(history[i].id, format!("ordered_{}", i + 1));
        }
    }

    #[tokio::test]
    async fn test_log_threat_with_different_types() {
        let config = create_test_config();
        let handler = AutomatedThreatResponseHandler::new(config);

        let threat_types = vec![
            ThreatType::Malware,
            ThreatType::Intrusion,
            ThreatType::DataExfiltration,
            ThreatType::DenialOfService,
        ];

        for (i, threat_type) in threat_types.iter().enumerate() {
            let threat = ThreatEvent::new(
                format!("type_{i}"),
                threat_type.clone(),
                ThreatSeverity::High,
                ThreatSource::default(),
                ThreatTarget::default(),
            );
            handler.log_threat_event(&threat).await.unwrap();
        }

        let history = handler.event_history.read().await;
        assert_eq!(history.len(), 4);
    }
}
