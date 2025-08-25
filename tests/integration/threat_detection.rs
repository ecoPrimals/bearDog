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


//! Threat detection integration tests

use super::common::*;
use beardog::errors::BearDogResult;
use beardog::threat::{SecurityEvent, ThreatDetectionEngine, ThreatDetectionConfig};
use beardog::tunnel::events::types::ThreatLevel;

#[tokio::test]
async fn test_threat_detection_engine() -> BearDogResult<()> {
    let core = create_test_core().await?;

    // Test threat detection through core
    let threat_engine = core.threat_detection_engine();

    // Create a test security event
    let event = SecurityEvent {
        event_id: "test-event-001".to_string(),
        event_type: "file_access".to_string(),
        source_ip: "192.168.1.100".to_string(),
        user_id: Some("test-user".to_string()),
        resource: "sensitive-file.txt".to_string(),
        action: "read".to_string(),
        timestamp: chrono::Utc::now(),
        metadata: serde_json::json!({
            "file_path": "/secure/sensitive-file.txt",
            "access_pattern": "unusual_time"
        }),
    };

    // Analyze the event for threats
    let analysis_result = threat_engine.analyze_event(event).await?;
    
    // Verify analysis was performed
    assert!(!analysis_result.threat_id.is_empty());
    assert!(analysis_result.confidence >= 0.0 && analysis_result.confidence <= 1.0);

    Ok(())
}

#[tokio::test]
async fn test_threat_detection_file_integrity() -> BearDogResult<()> {
    let core = create_test_core().await?;
    let threat_engine = core.threat_detection_engine();

    // Test file integrity threat detection
    let integrity_event = SecurityEvent {
        event_id: "integrity-001".to_string(),
        event_type: "file_modification".to_string(),
        source_ip: "10.0.0.5".to_string(),
        user_id: Some("admin".to_string()),
        resource: "system-config.conf".to_string(),
        action: "write".to_string(),
        timestamp: chrono::Utc::now(),
        metadata: serde_json::json!({
            "file_hash_before": "abc123",
            "file_hash_after": "def456",
            "modification_type": "unauthorized"
        }),
    };

    let analysis = threat_engine.analyze_event(integrity_event).await?;
    
    // File integrity threats should have high confidence
    assert!(analysis.confidence > 0.7);
    assert!(matches!(analysis.threat_level, ThreatLevel::High | ThreatLevel::Critical));

    Ok(())
} 