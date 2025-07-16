//! Threat detection types
//!
//! This module provides comprehensive type definitions for threat detection, analysis,
//! and response operations. The types are organized into logical modules while
//! maintaining backward compatibility through re-exports.
//!
//! ## Module Organization
//!
//! - [`config`] - Configuration types for threat detection engine
//! - [`core`] - Core threat types (severity, type, events)
//! - [`sources`] - Threat source and target information
//! - [`detection`] - Detection methods and evidence types
//! - [`actions`] - Actions, responses, and mitigation steps
//! - [`statistics`] - Statistics and metrics tracking
//! - [`intelligence`] - Threat intelligence and indicators
//! - [`engine`] - Detection engine and rule types
//! - [`analysis`] - Analysis results and security events
//! - [`incidents`] - Incident response and management
//!
//! ## Usage
//!
//! ```rust
//! use beardog::threat::types::{
//!     ThreatSeverity, ThreatType, ThreatEvent,
//!     ThreatDetectionConfig, DetectionMethod,
//!     ThreatAction, ThreatStatus
//! };
//!
//! // Create a threat event
//! let event = ThreatEvent {
//!     id: "threat-001".to_string(),
//!     threat_type: ThreatType::Malware,
//!     severity: ThreatSeverity::High,
//!     // ... other fields
//! };
//! ```
//!
//! ## Architecture
//!
//! The threat types are designed to work together as a cohesive system:
//!
//! 1. **Core Types**: Basic threat classifications and events
//! 2. **Detection**: Methods and evidence for threat identification
//! 3. **Intelligence**: External threat intelligence integration
//! 4. **Analysis**: Event correlation and analysis results
//! 5. **Response**: Actions and incident management
//! 6. **Statistics**: Performance and effectiveness metrics

// Module declarations
pub mod actions;
pub mod analysis;
pub mod config;
pub mod core;
pub mod detection;
pub mod engine;
pub mod incidents;
pub mod intelligence;
pub mod sources;
pub mod statistics;

// Re-exports for backward compatibility and convenience

// Configuration types
pub use config::ThreatDetectionConfig;

// Core threat types
pub use core::{ThreatEvent, ThreatSeverity, ThreatType};

// Source and target types
pub use sources::{
    AssetCriticality, GeoLocation, ProtectionLevel, SourceClassification, ThreatSource,
    ThreatTarget,
};

// Detection types
pub use detection::{
    DetectionMethod, EvidenceData, EvidenceType, FileMetadataData, LogEntryData, NetworkPacketData,
    ThreatEvidence,
};

// Action and response types
pub use actions::{MitigationStep, ResponseAction, ThreatAction, ThreatStatus};

// Statistics types
pub use statistics::{DetectionMethodStats, ThreatDetectionStats, ThreatStatistics, ThreatTrend};

// Intelligence types
pub use intelligence::{
    FeedStatus, FeedType, IndicatorType, ThreatIndicator, ThreatIntelligenceFeed, UpdateFrequency,
};

// Engine types
pub use engine::{
    DetectionRule, MlModel, MlModelType, RuleCondition, ThreatDetectionEngine, ThreatDetectionRule,
};

// Analysis types
pub use analysis::{
    AnalysisMetrics, AnalysisSession, CorrelationType, EventCorrelationResult, SecurityEvent,
    ThreatAnalysisResult,
};

// Incident types
pub use incidents::{
    IncidentMetrics, IncidentResponse, IncidentRole, IncidentStatus, IncidentTeamMember,
    IncidentTimelineEntry, TimelineEntryType,
};

// Type aliases for backward compatibility
/// Type alias for response action type for backward compatibility
pub type ResponseActionType = ResponseAction;

/// Comprehensive threat detection system
///
/// This module provides a complete threat detection ecosystem with
/// integrated components for detection, analysis, and response.
///
/// ## Key Features
///
/// - **Multi-layered Detection**: Signature, anomaly, behavioral, and ML-based detection
/// - **Threat Intelligence**: Integration with external threat feeds and indicators
/// - **Incident Response**: Full incident lifecycle management
/// - **Analytics**: Performance metrics and trend analysis
/// - **Extensibility**: Modular architecture for easy extension
///
/// ## Example: Complete Threat Detection Workflow
///
/// ```rust
/// use beardog::threat::types::*;
/// use chrono::Utc;
///
/// // 1. Configure the detection engine
/// let config = ThreatDetectionConfig {
///     real_time_detection: true,
///     threat_threshold: 70,
///     automated_response: true,
///     ml_enhancement: true,
///     ..Default::default()
/// };
///
/// // 2. Create detection engine
/// let mut engine = ThreatDetectionEngine::new(config);
///
/// // 3. Add detection rules
/// let rule = DetectionRule::new(
///     "malware-001".to_string(),
///     "Malware Detection".to_string(),
///     RuleCondition::FieldEquals {
///         field: "file_hash".to_string(),
///         value: "known_malware_hash".to_string(),
///     },
///     ThreatType::Malware,
///     ThreatSeverity::Critical
/// );
/// engine.add_detection_rule(rule);
///
/// // 4. Process security event
/// let event = SecurityEvent::new(
///     "event-001".to_string(),
///     "file_upload".to_string(),
///     "192.168.1.100".to_string(),
///     "10.0.0.1".to_string(),
///     "user123".to_string()
/// );
///
/// // 5. Analyze for threats
/// let analysis_result = ThreatAnalysisResult::new("event-001".to_string());
///
/// // 6. Create incident if threats detected
/// if analysis_result.has_threats() {
///     let incident = IncidentResponse::new(
///         "INC-2024-001".to_string(),
///         "threat-001".to_string(),
///         ThreatSeverity::High,
///         "Malware detected in file upload".to_string()
///     );
/// }
/// ```
///
/// ## Module Dependencies
///
/// ```text
/// core ←── sources, detection, actions, statistics, intelligence
/// engine ←── core, config, intelligence, statistics
/// analysis ←── core, detection, engine
/// incidents ←── core, actions
/// ```
///
/// ## Performance Considerations
///
/// - Use `ThreatDetectionStats` to monitor system performance
/// - Implement rate limiting with `ThreatDetectionConfig::max_alerts_per_minute`
/// - Use high-confidence indicators from `ThreatIntelligenceFeed`
/// - Optimize rule conditions for faster evaluation
///
/// ## Security Considerations
///
/// - Validate all external threat intelligence data
/// - Implement proper access controls for sensitive incident data
/// - Use secure channels for threat intelligence feed updates
/// - Regularly review and update detection rules
///
/// ## Extensibility
///
/// The type system is designed for easy extension:
///
/// - Add new `ThreatType` variants for emerging threats
/// - Implement custom `RuleCondition` types for specialized detection
/// - Extend `EvidenceType` for new data sources
/// - Create custom `IndicatorType` for domain-specific indicators

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_threat_severity_ordering() {
        assert!(ThreatSeverity::Critical > ThreatSeverity::High);
        assert!(ThreatSeverity::High > ThreatSeverity::Medium);
        assert!(ThreatSeverity::Medium > ThreatSeverity::Low);
        assert!(ThreatSeverity::Low > ThreatSeverity::Info);
    }

    #[test]
    fn test_threat_severity_scoring() {
        assert_eq!(ThreatSeverity::Critical.to_score(), 100);
        assert_eq!(ThreatSeverity::High.to_score(), 80);
        assert_eq!(ThreatSeverity::Medium.to_score(), 50);
        assert_eq!(ThreatSeverity::Low.to_score(), 30);
        assert_eq!(ThreatSeverity::Info.to_score(), 10);
    }

    #[test]
    fn test_threat_type_typical_severity() {
        assert_eq!(
            ThreatType::Ransomware.typical_severity(),
            ThreatSeverity::Critical
        );
        assert_eq!(
            ThreatType::Phishing.typical_severity(),
            ThreatSeverity::Medium
        );
        assert_eq!(
            ThreatType::BruteForce.typical_severity(),
            ThreatSeverity::High
        );
    }

    #[test]
    fn test_threat_event_priority() {
        let mut event = ThreatEvent {
            id: "test_event".to_string(),
            threat_type: ThreatType::Unknown,
            severity: ThreatSeverity::High,
            score: 85,
            timestamp: Utc::now(),
            source: ThreatSource {
                id: "test_source".to_string(),
                ip_address: Some("192.168.1.1".to_string()),
                hostname: None,
                geolocation: None,
                user_agent: None,
                reputation_score: 0.0,
                threat_actor: None,
                classification: SourceClassification::Unknown,
                confidence_score: 0.0,
                first_seen: Some(Utc::now()),
                last_seen: Some(Utc::now()),
                threat_score: 0.0,
            },
            target: ThreatTarget {
                id: "test_target".to_string(),
                resource_id: "test_resource".to_string(),
                resource_type: "file".to_string(),
                node_id: None,
                user_account: None,
                criticality: AssetCriticality::Low,
                protection_level: ProtectionLevel::Basic,
                ip_address: None,
                hostname: None,
                service: None,
                port: None,
                protocol: None,
            },
            description: "Test threat event".to_string(),
            evidence: vec![],
            detection_method: DetectionMethod::Signature,
            recommended_actions: vec![],
            status: ThreatStatus::New,
            assigned_analyst: None,
            related_events: vec![],
            mitigation_steps: vec![],
        };

        assert!(event.is_high_priority());

        event.severity = ThreatSeverity::Medium;
        event.score = 60;

        assert!(!event.is_high_priority());
    }

    #[test]
    fn test_detection_config_validation() {
        let config = ThreatDetectionConfig {
            threat_threshold: 105,  // Invalid
            alert_threshold: 1.5,   // Invalid
            cache_size: 0,          // Invalid
            monitoring_interval: 0, // Invalid
            ..Default::default()
        };

        assert!(!config.is_valid());

        let valid_config = ThreatDetectionConfig::default();
        assert!(valid_config.is_valid());
    }

    #[test]
    fn test_threat_source_classification() {
        let mut source = ThreatSource::default();
        source.classification = SourceClassification::Malicious;
        source.reputation_score = 0.1;

        assert!(source.is_malicious());
        assert!(!source.is_trustworthy());

        source.classification = SourceClassification::Trusted;
        source.reputation_score = 0.9;

        assert!(source.is_trustworthy());
        assert!(!source.is_malicious());
    }

    #[test]
    fn test_threat_target_risk_assessment() {
        let mut target = ThreatTarget::default();
        target.criticality = AssetCriticality::Critical;
        target.protection_level = ProtectionLevel::Basic;

        assert!(target.is_high_value());
        assert!(!target.is_well_protected());

        let risk = target.risk_score();
        assert!(risk > 0.5); // High risk due to critical asset with basic protection
    }

    #[test]
    fn test_detection_method_characteristics() {
        assert!(DetectionMethod::Signature.typical_accuracy() > 0.9);
        assert!(DetectionMethod::Signature.false_positive_rate() < 0.1);
        assert!(!DetectionMethod::Signature.is_good_for_unknown_threats());

        assert!(DetectionMethod::Anomaly.is_good_for_unknown_threats());
        assert!(DetectionMethod::Anomaly.false_positive_rate() > 0.1);
    }

    #[test]
    fn test_threat_status_transitions() {
        let status = ThreatStatus::New;
        let valid_next = status.valid_next_statuses();

        assert!(valid_next.contains(&ThreatStatus::Investigating));
        assert!(valid_next.contains(&ThreatStatus::FalsePositive));
        assert!(!valid_next.contains(&ThreatStatus::Resolved));

        assert!(status.can_transition_to(&ThreatStatus::Investigating));
        assert!(!status.can_transition_to(&ThreatStatus::Resolved));
    }

    #[test]
    fn test_threat_action_automation() {
        assert!(ThreatAction::BlockSource.is_automated());
        assert!(ThreatAction::BlockSource.is_reversible());
        assert_eq!(ThreatAction::BlockSource.severity_level(), 3);

        assert!(!ThreatAction::NotifyLawEnforcement.is_automated());
        assert!(!ThreatAction::NotifyLawEnforcement.is_reversible());
        assert_eq!(ThreatAction::NotifyLawEnforcement.severity_level(), 5);
    }

    #[test]
    fn test_threat_intelligence_indicator() {
        let mut indicator =
            ThreatIndicator::new(IndicatorType::IpAddress, "192.168.1.100".to_string(), 0.9);

        assert!(indicator.is_high_confidence());
        assert!(indicator.is_recent(24));

        indicator.add_threat_type(ThreatType::Malware);
        indicator.add_tag("botnet".to_string());

        assert_eq!(indicator.threat_types.len(), 1);
        assert_eq!(indicator.tags.len(), 1);
    }

    #[test]
    fn test_incident_response_workflow() {
        let mut incident = IncidentResponse::new(
            "INC-2024-001".to_string(),
            "threat-001".to_string(),
            ThreatSeverity::High,
            "Malware detected".to_string(),
        );

        assert!(incident.is_active());
        assert!(incident.is_high_priority());
        assert_eq!(incident.status, IncidentStatus::Open);

        incident.update_status(IncidentStatus::InProgress);
        incident.assign_to("analyst-001".to_string());
        incident.add_containment_action("System isolated".to_string());

        assert_eq!(incident.status, IncidentStatus::InProgress);
        assert_eq!(incident.assigned_to, Some("analyst-001".to_string()));
        assert_eq!(incident.containment_actions.len(), 1);

        incident.update_status(IncidentStatus::Resolved);
        assert!(incident.status.can_transition_to(&IncidentStatus::Closed));
    }

    #[test]
    fn test_analysis_metrics_tracking() {
        let mut metrics = AnalysisMetrics::new();

        metrics.update_with_analysis(100, 2);
        metrics.update_with_analysis(200, 0);
        metrics.update_with_analysis(150, 1);

        assert_eq!(metrics.total_events_analyzed, 3);
        assert_eq!(metrics.total_threats_detected, 3);
        assert_eq!(metrics.events_with_threats, 2);
        assert_eq!(metrics.get_threat_detection_rate(), 2.0 / 3.0);
        assert_eq!(metrics.get_avg_threats_per_event(), 1.0);
        assert_eq!(metrics.avg_analysis_time_ms, 150.0);
    }

    #[test]
    fn test_security_event_characteristics() {
        let mut event = SecurityEvent::new(
            "event-001".to_string(),
            "failed_login".to_string(),
            "192.168.1.100".to_string(),
            "10.0.0.1".to_string(),
            "user123".to_string(),
        );

        assert!(event.is_internal_source());
        assert!(event.is_recent(60));

        event.data_size = 20_000_000.0; // 20MB
        assert!(event.is_large_data_transfer());

        let severity = event.severity_score();
        assert!(severity > 0.0);
    }

    #[test]
    fn test_rule_condition_complexity() {
        let simple = RuleCondition::FieldEquals {
            field: "event_type".to_string(),
            value: "login".to_string(),
        };

        let complex = RuleCondition::And {
            conditions: vec![
                simple.clone(),
                RuleCondition::FrequencyThreshold {
                    count: 5,
                    window_minutes: 1,
                },
            ],
        };

        assert!(!simple.is_complex());
        assert!(complex.is_complex());
        assert!(complex.complexity_score() > simple.complexity_score());
    }

    #[test]
    fn test_ml_model_lifecycle() {
        let mut model = MlModel::new(
            "model-001".to_string(),
            "Anomaly Detector".to_string(),
            MlModelType::AnomalyDetection,
            0.92,
        );

        assert!(model.is_high_accuracy());
        assert_eq!(model.age_days(), 0);
        assert!(!model.needs_retraining());

        model.update_training_timestamp();
        assert_eq!(model.age_days(), 0);
    }

    #[test]
    fn test_comprehensive_workflow() {
        // Test a complete workflow from detection to incident resolution
        let config = ThreatDetectionConfig::default();
        let mut engine = ThreatDetectionEngine::new(config);

        // Add a detection rule
        let rule = DetectionRule::new(
            "test-rule".to_string(),
            "Test Rule".to_string(),
            RuleCondition::FieldEquals {
                field: "threat_type".to_string(),
                value: "malware".to_string(),
            },
            ThreatType::Malware,
            ThreatSeverity::High,
        );
        engine.add_detection_rule(rule);

        // Create a security event
        let event = SecurityEvent::new(
            "event-001".to_string(),
            "file_scan".to_string(),
            "192.168.1.100".to_string(),
            "10.0.0.1".to_string(),
            "user123".to_string(),
        );

        // Analyze the event
        let mut analysis = ThreatAnalysisResult::new("event-001".to_string());
        analysis.add_recommendation("Quarantine file".to_string());

        // Create incident if needed
        if analysis.has_threats() {
            let incident = IncidentResponse::new(
                "INC-2024-001".to_string(),
                "threat-001".to_string(),
                ThreatSeverity::High,
                "Malware detected".to_string(),
            );

            assert!(incident.is_active());
            assert!(incident.is_high_priority());
        }

        // Verify engine state
        assert_eq!(engine.detection_rules.len(), 1);
        assert_eq!(engine.get_enabled_rules().len(), 1);
    }
}

/// Type validation utilities
pub mod validation {
    use super::*;

    /// Validate threat event
    pub fn validate_threat_event(event: &ThreatEvent) -> Result<(), String> {
        if event.id.is_empty() {
            return Err("Event ID cannot be empty".to_string());
        }

        if event.score > 100 {
            return Err("Threat score cannot exceed 100".to_string());
        }

        if event.description.is_empty() {
            return Err("Event description cannot be empty".to_string());
        }

        Ok(())
    }

    /// Validate detection rule
    pub fn validate_detection_rule(rule: &DetectionRule) -> Result<(), String> {
        if rule.id.is_empty() {
            return Err("Rule ID cannot be empty".to_string());
        }

        if rule.name.is_empty() {
            return Err("Rule name cannot be empty".to_string());
        }

        if rule.condition.complexity_score() > 10 {
            return Err("Rule condition too complex".to_string());
        }

        Ok(())
    }

    /// Validate threat intelligence indicator
    pub fn validate_threat_indicator(indicator: &ThreatIndicator) -> Result<(), String> {
        if indicator.value.is_empty() {
            return Err("Indicator value cannot be empty".to_string());
        }

        if indicator.confidence < 0.0 || indicator.confidence > 1.0 {
            return Err("Confidence must be between 0.0 and 1.0".to_string());
        }

        if indicator.first_seen > indicator.last_seen {
            return Err("First seen cannot be after last seen".to_string());
        }

        Ok(())
    }

    /// Validate security event
    pub fn validate_security_event(event: &SecurityEvent) -> Result<(), String> {
        if event.event_id.is_empty() {
            return Err("Event ID cannot be empty".to_string());
        }

        if event.event_type.is_empty() {
            return Err("Event type cannot be empty".to_string());
        }

        if event.source_ip.is_empty() {
            return Err("Source IP cannot be empty".to_string());
        }

        if event.user_id.is_empty() {
            return Err("User ID cannot be empty".to_string());
        }

        Ok(())
    }
}
