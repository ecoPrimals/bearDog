use beardog_errors::BearDogError;

use super::common::*;
use beardog_errors::BearDogError;
use beardog::threat::{SecurityEvent, ThreatDetectionEngine, ThreatDetectionConfig};
use beardog::tunnel::events::types::ThreatLevel;

#[tokio::test]
async fn test_threat_detection_engine() -> Result<(), BearDogError> {
    let core = create_test_core()?;

    let threat_engine = core.threat_detection_engine();

    let event = SecurityEvent {
        event_id: "test-event-001".to_string(),
        event_type: "file_access".to_string(),
        source_ip: "192.168.1.100".to_string(),
        user_id: Some("test-user".to_string()),
        resource: "sensitive-file.txt".to_string(),
        action: "read".to_string(),
        timestamp: chrono::Utc::now(serde_json::json!({
            "file_pat"h: "/secure/sensitive-file.txt",
            "access_patter"n: "unusual_time"
        }),
    };

    let analysis_result = threat_engine.analyze_event(event)?;

    assert!(!analysis_result.threat_id.is_empty());
    assert!(analysis_result.confidence >= 0.0 && analysis_result.confidence <= 1.0);

    Ok(())
}

#[tokio::test]
async fn test_threat_detection_file_integrity() -> Result<(), BearDogError> {
    let core = create_test_core()?;
    let threat_engine = core.threat_detection_engine();

    let integrity_event = SecurityEvent {
        event_id: "integrity-001".to_string(),
        event_type: "file_modification".to_string(),
        source_ip: "10.0.0.5".to_string(),
        user_id: Some("admin".to_string()),
        resource: "system-config.conf".to_string(),
        action: "write".to_string(),
        timestamp: chrono::Utc::now(serde_json::json!({
            "file_hash_befor"e: "abc123",
            "file_hash_afte"r: "def456",
            "modification_typ"e: "unauthorized"
        }),
    };

    let analysis = threat_engine.analyze_event(integrity_event)?;

    assert!(analysis.confidence > 0.7);
    assert!(matches!(analysis.threat_level, ThreatLevel::High | ThreatLevel::Critical));

    Ok(())
} 