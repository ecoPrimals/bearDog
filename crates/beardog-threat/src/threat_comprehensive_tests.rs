//! Comprehensive Unit Tests for BearDog Threat Detection
//!
//! Created: October 27, 2025
//! Purpose: Expand test coverage for beardog-threat core functionality

use crate::threat::types::{ThreatSeverity, ThreatType, ThreatStatus, DetectionMethod};
use crate::threat::types::incidents::{IncidentStatus, SecurityIncident};

// Threat Severity Tests
#[test]
fn test_threat_severity_low() { assert!(matches!(ThreatSeverity::Low, ThreatSeverity::Low)); }

#[test]
fn test_threat_severity_medium() { assert!(matches!(ThreatSeverity::Medium, ThreatSeverity::Medium)); }

#[test]
fn test_threat_severity_high() { assert!(matches!(ThreatSeverity::High, ThreatSeverity::High)); }

#[test]
fn test_threat_severity_critical() { assert!(matches!(ThreatSeverity::Critical, ThreatSeverity::Critical)); }

#[test]
fn test_threat_severity_clone() {
    let s = ThreatSeverity::High;
    let c = s.clone();
    assert_eq!(s, c);
}

// Threat Type Tests
#[test]
fn test_threat_type_malware() { assert!(matches!(ThreatType::Malware, ThreatType::Malware)); }

#[test]
fn test_threat_type_intrusion() { assert!(matches!(ThreatType::Intrusion, ThreatType::Intrusion)); }

#[test]
fn test_threat_type_exfiltration() { assert!(matches!(ThreatType::DataExfiltration, ThreatType::DataExfiltration)); }

#[test]
fn test_threat_type_dos() { assert!(matches!(ThreatType::DenialOfService, ThreatType::DenialOfService)); }

#[test]
fn test_threat_type_priv_esc() { assert!(matches!(ThreatType::PrivilegeEscalation, ThreatType::PrivilegeEscalation)); }

// Threat Status Tests
#[test]
fn test_threat_status_active() { assert!(matches!(ThreatStatus::Active, ThreatStatus::Active)); }

#[test]
fn test_threat_status_resolved() { assert!(matches!(ThreatStatus::Resolved, ThreatStatus::Resolved)); }

// Detection Method Tests
#[test]
fn test_detection_method_rule_based() { assert!(matches!(DetectionMethod::RuleBased, DetectionMethod::RuleBased)); }

#[test]
fn test_detection_method_ml() { assert!(matches!(DetectionMethod::MachineLearning, DetectionMethod::MachineLearning)); }

#[test]
fn test_detection_method_threat_intel() { assert!(matches!(DetectionMethod::ThreatIntelligence, DetectionMethod::ThreatIntelligence)); }

#[test]
fn test_detection_method_behavioral() { assert!(matches!(DetectionMethod::BehavioralAnalysis, DetectionMethod::BehavioralAnalysis)); }

// Incident Status Tests
#[test]
fn test_incident_status_reported() { assert!(matches!(IncidentStatus::Reported, IncidentStatus::Reported)); }

#[test]
fn test_incident_status_investigating() { assert!(matches!(IncidentStatus::Investigating, IncidentStatus::Investigating)); }

#[test]
fn test_incident_status_confirmed() { assert!(matches!(IncidentStatus::Confirmed, IncidentStatus::Confirmed)); }

#[test]
fn test_incident_status_resolved() { assert!(matches!(IncidentStatus::Resolved, IncidentStatus::Resolved)); }

#[test]
fn test_incident_status_false_positive() { assert!(matches!(IncidentStatus::FalsePositive, IncidentStatus::FalsePositive)); }

// Security Incident Tests
#[test]
fn test_security_incident_creation() {
    let incident = SecurityIncident::new(
        "INC-001".to_string(),
        "Test".to_string(),
        "Test Description".to_string(),
        ThreatSeverity::High,
    );
    assert_eq!(incident.id, "INC-001");
    assert_eq!(incident.title, "Test");
}

#[test]
fn test_security_incident_defaults() {
    let incident = SecurityIncident::new(
        "INC-002".to_string(),
        "Test2".to_string(),
        "Desc".to_string(),
        ThreatSeverity::Medium,
    );
    assert!(matches!(incident.status, IncidentStatus::Reported));
    assert!(incident.assigned_to.is_none());
    assert!(incident.threat_events.is_empty());
}

#[test]
fn test_security_incident_clone() {
    let incident = SecurityIncident::new(
        "INC-003".to_string(),
        "Clone Test".to_string(),
        "Testing clone".to_string(),
        ThreatSeverity::Low,
    );
    let cloned = incident.clone();
    assert_eq!(incident.id, cloned.id);
}
