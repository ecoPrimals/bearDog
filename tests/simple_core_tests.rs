

use beardog::auth::types::SpawnStatus;
use beardog::config::EncryptionConfig;
use beardog::node_registry::TrustLevel; // The TrustLevel with Basic, Unknown, etc.
use beardog::tunnel::events::*;
use beardog::BearDogError;

use beardog::tunnel::events::SecurityLevel as EventsSecurityLevel;

#[test]
fn test_basic_error_types() {

    let error = BearDogError::SpawnRejected {
        reason: "Test rejection".to_string(),
    };

    assert!(format!("{error:?}").contains("SpawnRejected"));
    assert!(format!("{error:?}").contains("Test rejection"));
}

#[test]
fn test_trust_levels() {

    assert!(TrustLevel::Basic as u8 > TrustLevel::Unknown as u8);
    assert!(TrustLevel::Medium as u8 > TrustLevel::Basic as u8);
    assert!(TrustLevel::High as u8 > TrustLevel::Medium as u8);
    assert!(TrustLevel::Explicit as u8 > TrustLevel::High as u8);
}

#[test]
fn test_event_types() {

    let peer_caps = PeerCapabilities {
        identity_proof: None,
        gaming_profile: None,
        supported_crypto: vec!["AES-256-GCM".to_string()],
        max_bandwidth: 1000000000, // 1 GB/s
        latency_tolerance: std::time::Duration::from_millis(100),
    };

    let event = NetworkSecurityEvent::PeerDiscovered {
        peer_id: "test-peer".to_string(),
        peer_capabilities: peer_caps,
        trust_indicators: vec![TrustIndicator::LocalNetworkPeer],
    };

    match event {
        NetworkSecurityEvent::PeerDiscovered { peer_id, .. } => {
            assert_eq!(peer_id, "test-peer");
        }
        _ => panic!("Event type mismatch"),
    }
}

#[test]
fn test_spawn_status() {

    let status = SpawnStatus::Initializing;
    assert!(matches!(status, SpawnStatus::Initializing));

    let rejected = SpawnStatus::Failed("Access denied".to_string());

    match rejected {
        SpawnStatus::Failed(reason) => {
            assert_eq!(reason, "Access denied");
        }
        _ => panic!("Expected Failed variant"),
    }
}

#[test]
fn test_compliance_types() {

    let compliance_types = [ComplianceType::GDPR,
        ComplianceType::HIPAA,
        ComplianceType::SOX,
        ComplianceType::DataSovereignty,
        ComplianceType::ExportControl];

    assert_eq!(compliance_types.len(), 5);

    assert!(matches!(compliance_types[0], ComplianceType::GDPR));
    assert!(matches!(compliance_types[1], ComplianceType::HIPAA));
}

#[test]
fn test_security_levels() {

    let levels = [EventsSecurityLevel::Low,
        EventsSecurityLevel::Medium,
        EventsSecurityLevel::High,
        EventsSecurityLevel::Ultimate,
        EventsSecurityLevel::Adaptive,
        EventsSecurityLevel::Optimized];

    assert_eq!(levels.len(), 6);
}

#[test]
fn test_thread_safety_markers() {

    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}

    is_send::<BearDogError>();
    is_send::<TrustLevel>();
    is_send::<ComplianceType>();
    is_send::<EventsSecurityLevel>();

    is_sync::<TrustLevel>();
    is_sync::<ComplianceType>();
    is_sync::<EventsSecurityLevel>();
}

#[test]
fn test_forest_metaphor_consistency() {

    let scientist_trust = TrustLevel::High;
    let newcomer_trust = TrustLevel::Basic;

    assert!(scientist_trust as u8 > newcomer_trust as u8);

    let mut evidence_data = std::collections::HashMap::with_capacity(16);
    evidence_data.insert("source_ip".to_string(), "malicious.example.com".to_string());
    evidence_data.insert("attempts".to_string(), "multiple".to_string());

    let evidence = NetworkEvidence {
        evidence_type: "authentication_failure".to_string(),
        data: evidence_data,
        timestamp: std::time::SystemTime::now(),
        confidence: 0.8,
    };

    let protection_event = NetworkSecurityEvent::SuspiciousActivity {
        source_peer: "suspicious-node-001".to_string(),
        activity_type: SuspiciousActivityType::FailedAuthentication,
        severity: NetworkThreatLevel::High,
        evidence: vec![evidence],
    };

    match protection_event {
        NetworkSecurityEvent::SuspiciousActivity { severity, .. } => {
            assert_eq!(severity, NetworkThreatLevel::High);
        }
        _ => panic!("Protection event failed"),
    }
}

#[test]
fn test_encryption_config_sanity() {

    let encryption_config = EncryptionConfig::default();

    assert!(encryption_config.key_derivation_iterations > 0);
    assert!(encryption_config.key_rotation_days > 0); // Should have reasonable rotation
}

#[test]
fn test_memory_layout_sanity() {

    use std::mem::size_of;

    assert!(size_of::<TrustLevel>() <= 8);
    assert!(size_of::<ComplianceType>() <= 32);
    assert!(size_of::<EventsSecurityLevel>() <= 8);

    assert!(size_of::<BearDogError>() <= 256);
}
