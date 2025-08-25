// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! Simple Core Tests for BearDog
//!
//! Basic validation of core functionality without complex async operations
//! to ensure we can run tests reliably.

use beardog::auth::types::SpawnStatus;
use beardog::config::EncryptionConfig;
use beardog::node_registry::TrustLevel; // The TrustLevel with Basic, Unknown, etc.
use beardog::tunnel::events::*;
use beardog::BearDogError;
// Use specific SecurityLevel from tunnel events to avoid ambiguity
use beardog::tunnel::events::SecurityLevel as EventsSecurityLevel;

#[test]
fn test_basic_error_types() {
    // Test that our error types can be created and used
    let error = BearDogError::SpawnRejected {
        reason: "Test rejection".to_string(),
    };

    assert!(format!("{error:?}").contains("SpawnRejected"));
    assert!(format!("{error:?}").contains("Test rejection"));
}

#[test]
fn test_trust_levels() {
    // Test trust level ordering
    assert!(TrustLevel::Basic as u8 > TrustLevel::Unknown as u8);
    assert!(TrustLevel::Medium as u8 > TrustLevel::Basic as u8);
    assert!(TrustLevel::High as u8 > TrustLevel::Medium as u8);
    assert!(TrustLevel::Explicit as u8 > TrustLevel::High as u8);
}

#[test]
fn test_event_types() {
    // Test that event types can be created
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
    // Test spawn status variants - just test the ones that don't need complex types
    let status = SpawnStatus::Initializing;
    assert!(matches!(status, SpawnStatus::Initializing));

    let rejected = SpawnStatus::Failed("Access denied".to_string());

    // Test pattern matching
    match rejected {
        SpawnStatus::Failed(reason) => {
            assert_eq!(reason, "Access denied");
        }
        _ => panic!("Expected Failed variant"),
    }
}

#[test]
fn test_compliance_types() {
    // Test compliance enumeration
    let compliance_types = [ComplianceType::GDPR,
        ComplianceType::HIPAA,
        ComplianceType::SOX,
        ComplianceType::DataSovereignty,
        ComplianceType::ExportControl];

    assert_eq!(compliance_types.len(), 5);

    // Test that we can match on compliance types
    assert!(matches!(compliance_types[0], ComplianceType::GDPR));
    assert!(matches!(compliance_types[1], ComplianceType::HIPAA));
}

#[test]
fn test_security_levels() {
    // Test security level enumeration
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
    // Test that our key types implement Send and Sync
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
    // Test that our forest protection metaphor is consistent
    let scientist_trust = TrustLevel::High;
    let newcomer_trust = TrustLevel::Basic;

    assert!(scientist_trust as u8 > newcomer_trust as u8);

    // Test that we can create protective events
    let mut evidence_data = std::collections::HashMap::new();
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
    // Test that EncryptionConfig has sensible defaults
    let encryption_config = EncryptionConfig::default();

    // Test available fields instead of non-existent key_size
    assert!(encryption_config.key_derivation_iterations > 0);
    assert!(encryption_config.key_rotation_days > 0); // Should have reasonable rotation
}

#[test]
fn test_memory_layout_sanity() {
    // Test that our structs have reasonable memory layouts
    use std::mem::size_of;

    // These should be reasonably sized
    assert!(size_of::<TrustLevel>() <= 8);
    assert!(size_of::<ComplianceType>() <= 32);
    assert!(size_of::<EventsSecurityLevel>() <= 8);

    // Error types can be larger but should be reasonable
    assert!(size_of::<BearDogError>() <= 256);
}
