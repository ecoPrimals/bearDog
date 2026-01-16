//! Comprehensive Unit Tests for Universal Trust API
//!
//! These tests verify individual components of the trust evaluation system
//! without requiring a running server.

use beardog_tunnel::api::trust::{
    IdentityAttestation, TrustApiState, TrustDecision, TrustEvaluationRequest,
    TrustEvaluationResponse, TrustLevel,
};
use serde_json::json;

// ====================================================================================
// TrustApiState Unit Tests
// ====================================================================================

#[test]
fn test_trust_api_state_creation_with_family() {
    let state = TrustApiState::new(Some("test_family".to_string()), "test_node");

    assert_eq!(state.family_id, Some("test_family".to_string()));
    assert_eq!(state.encryption_tag, "beardog:family:test_family:test_node");
    assert_eq!(state.capabilities.len(), 3);
    assert!(state.capabilities.contains(&"btsp".to_string()));
    assert!(state.capabilities.contains(&"birdsong".to_string()));
    assert!(state.capabilities.contains(&"lineage".to_string()));
}

#[test]
fn test_trust_api_state_creation_without_family() {
    let state = TrustApiState::new(None, "test_node");

    assert_eq!(state.family_id, None);
    assert_eq!(state.encryption_tag, "beardog:node:test_node");
    assert_eq!(state.capabilities.len(), 3);
}

#[test]
fn test_trust_api_state_with_special_characters() {
    let state = TrustApiState::new(Some("family-123_test".to_string()), "node@host.com");

    assert_eq!(state.family_id, Some("family-123_test".to_string()));
    assert_eq!(
        state.encryption_tag,
        "beardog:family:family-123_test:node@host.com"
    );
}

#[test]
fn test_trust_api_state_with_long_identifiers() {
    let long_family = "a".repeat(100);
    let long_node = "b".repeat(100);
    let state = TrustApiState::new(Some(long_family.clone()), &long_node);

    assert_eq!(state.family_id, Some(long_family.clone()));
    assert!(state.encryption_tag.contains(&long_family));
    assert!(state.encryption_tag.contains(&long_node));
}

// ====================================================================================
// IdentityAttestation Unit Tests
// ====================================================================================

#[test]
fn test_identity_attestation_serialization() {
    let attestation = IdentityAttestation {
        provider_capability: "security/identity".to_string(),
        format: "tag_list".to_string(),
        data: json!({
            "tags": ["beardog:family:test:node1"],
            "family_id": "test"
        }),
    };

    let json = serde_json::to_string(&attestation).unwrap();
    assert!(json.contains("security/identity"));
    assert!(json.contains("tag_list"));
    assert!(json.contains("beardog:family:test:node1"));
}

#[test]
fn test_identity_attestation_deserialization() {
    let json = r#"{
        "provider_capability": "security/identity",
        "format": "tag_list",
        "data": {
            "tags": ["beardog:family:test:node1"],
            "family_id": "test"
        }
    }"#;

    let attestation: IdentityAttestation = serde_json::from_str(json).unwrap();
    assert_eq!(attestation.provider_capability, "security/identity");
    assert_eq!(attestation.format, "tag_list");
    assert!(attestation.data.get("tags").is_some());
}

#[test]
fn test_identity_attestation_with_certificate_format() {
    let attestation = IdentityAttestation {
        provider_capability: "security/certificate".to_string(),
        format: "x509".to_string(),
        data: json!({
            "certificate": "-----BEGIN CERTIFICATE-----\n...",
            "issuer": "CN=Test CA"
        }),
    };

    let json = serde_json::to_value(&attestation).unwrap();
    assert_eq!(json["provider_capability"], "security/certificate");
    assert_eq!(json["format"], "x509");
}

#[test]
fn test_identity_attestation_with_empty_data() {
    let attestation = IdentityAttestation {
        provider_capability: "test/empty".to_string(),
        format: "none".to_string(),
        data: json!({}),
    };

    let serialized = serde_json::to_string(&attestation).unwrap();
    let deserialized: IdentityAttestation = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.provider_capability, "test/empty");
}

// ====================================================================================
// TrustEvaluationRequest Unit Tests
// ====================================================================================

#[test]
fn test_trust_request_universal_format_serialization() {
    let request = TrustEvaluationRequest {
        request_format: Some("universal_trust_v1".to_string()),
        evaluator: Some(beardog_tunnel::api::trust::EvaluatorInfo {
            peer_id: "peer1".to_string(),
            attestations: vec![IdentityAttestation {
                provider_capability: "security/identity".to_string(),
                format: "tag_list".to_string(),
                data: json!({"tags": ["test"]}),
            }],
        }),
        context: Some(json!({"method": "udp"})),
        requested_operation: None,
        peer_id: None,
        peer_tags: None,
        connection_info: None,
    };

    let json = serde_json::to_value(&request).unwrap();
    assert_eq!(json["request_format"], "universal_trust_v1");
    assert!(json["evaluator"].is_object());
}

#[test]
fn test_trust_request_legacy_format_serialization() {
    let request = TrustEvaluationRequest {
        request_format: None,
        evaluator: None,
        context: None,
        requested_operation: None,
        peer_id: Some("peer1".to_string()),
        peer_tags: Some(vec!["beardog:family:test:peer1".to_string()]),
        connection_info: Some(
            [("endpoint".to_string(), "http://localhost".to_string())]
                .iter()
                .cloned()
                .collect(),
        ),
    };

    let json = serde_json::to_value(&request).unwrap();
    assert!(json["request_format"].is_null());
    assert_eq!(json["peer_id"], "peer1");
}

#[test]
fn test_trust_request_mixed_fields_serialization() {
    // Should serialize all non-null fields
    let request = TrustEvaluationRequest {
        request_format: Some("universal_trust_v1".to_string()),
        evaluator: Some(beardog_tunnel::api::trust::EvaluatorInfo {
            peer_id: "peer1".to_string(),
            attestations: vec![],
        }),
        context: Some(json!({})),
        requested_operation: None,
        peer_id: Some("legacy_peer".to_string()),
        peer_tags: Some(vec![]),
        connection_info: None,
    };

    let json = serde_json::to_value(&request).unwrap();
    assert!(json["request_format"].is_string());
    assert!(json["evaluator"].is_object());
    assert!(json["peer_id"].is_string());
}

// ====================================================================================
// TrustEvaluationResponse Unit Tests
// ====================================================================================

#[test]
fn test_trust_response_universal_format() {
    let response = TrustEvaluationResponse {
        response_format: Some("universal_trust_v1".to_string()),
        decision: TrustDecision::AutoAccept,
        confidence: 1.0,
        reason: "Same family".to_string(),
        reason_code: Some("same_genetic_family".to_string()),
        metadata: Some(json!({"family_id": "test"})),
        expires_at: Some("2026-01-04T00:00:00Z".to_string()),
        trust_level_numeric: Some(3),
        allowed_capabilities: Some(vec!["btsp".to_string(), "birdsong".to_string()]),
        denied_capabilities: Some(vec![]),
        elevation_path: None,
        trust_level: Some(TrustLevel::Highest),
        encryption_tag: Some("beardog:family:test:peer1".to_string()),
    };

    let json = serde_json::to_value(&response).unwrap();
    assert_eq!(json["response_format"], "universal_trust_v1");
    assert_eq!(json["decision"], "auto_accept");
    assert_eq!(json["confidence"], 1.0);
    assert_eq!(json["reason_code"], "same_genetic_family");
}

#[test]
fn test_trust_response_all_decision_types() {
    let decisions = vec![
        TrustDecision::AutoAccept,
        TrustDecision::PromptUser,
        TrustDecision::Reject,
    ];

    for decision in decisions {
        let response = TrustEvaluationResponse {
            response_format: Some("universal_trust_v1".to_string()),
            decision: decision.clone(),
            confidence: 0.5,
            reason: "Test".to_string(),
            reason_code: Some("test".to_string()),
            metadata: None,
            expires_at: None,
            trust_level_numeric: None,
            allowed_capabilities: None,
            denied_capabilities: None,
            elevation_path: None,
            trust_level: None,
            encryption_tag: None,
        };

        let json = serde_json::to_value(&response).unwrap();
        assert!(json["decision"].is_string());
    }
}

#[test]
fn test_trust_response_all_trust_levels() {
    let levels = vec![
        TrustLevel::Highest,
        TrustLevel::Elevated,
        TrustLevel::Limited,
        TrustLevel::None,
    ];

    for level in levels {
        let response = TrustEvaluationResponse {
            response_format: None,
            decision: TrustDecision::AutoAccept,
            confidence: 0.5,
            reason: "Test".to_string(),
            reason_code: None,
            metadata: None,
            expires_at: None,
            trust_level_numeric: None,
            allowed_capabilities: None,
            denied_capabilities: None,
            elevation_path: None,
            trust_level: Some(level),
            encryption_tag: None,
        };

        let json = serde_json::to_value(&response).unwrap();
        assert!(json["trust_level"].is_string());
    }
}

#[test]
fn test_trust_response_confidence_bounds() {
    let confidences = vec![0.0, 0.3, 0.5, 0.7, 1.0];

    for confidence in confidences {
        let response = TrustEvaluationResponse {
            response_format: Some("universal_trust_v1".to_string()),
            decision: TrustDecision::AutoAccept,
            confidence,
            reason: "Test".to_string(),
            reason_code: Some("test".to_string()),
            metadata: None,
            expires_at: None,
            trust_level_numeric: None,
            allowed_capabilities: None,
            denied_capabilities: None,
            elevation_path: None,
            trust_level: None,
            encryption_tag: None,
        };

        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json["confidence"].as_f64().unwrap(), confidence);
    }
}

#[test]
fn test_trust_response_optional_fields_omitted() {
    let response = TrustEvaluationResponse {
        response_format: Some("universal_trust_v1".to_string()),
        decision: TrustDecision::AutoAccept,
        confidence: 1.0,
        reason: "Test".to_string(),
        reason_code: Some("test".to_string()),
        metadata: None,
        expires_at: None,
        trust_level_numeric: None,
        allowed_capabilities: None,
        denied_capabilities: None,
        elevation_path: None,
        trust_level: None,
        encryption_tag: None,
    };

    let json = serde_json::to_value(&response).unwrap();
    assert!(!json.as_object().unwrap().contains_key("metadata"));
    assert!(!json.as_object().unwrap().contains_key("expires_at"));
    assert!(!json.as_object().unwrap().contains_key("trust_level"));
    assert!(!json.as_object().unwrap().contains_key("encryption_tag"));
}

// ====================================================================================
// TrustDecision Unit Tests
// ====================================================================================

#[test]
fn test_trust_decision_serialization() {
    assert_eq!(
        serde_json::to_string(&TrustDecision::AutoAccept).unwrap(),
        "\"auto_accept\""
    );
    assert_eq!(
        serde_json::to_string(&TrustDecision::PromptUser).unwrap(),
        "\"prompt_user\""
    );
    assert_eq!(
        serde_json::to_string(&TrustDecision::Reject).unwrap(),
        "\"reject\""
    );
}

#[test]
fn test_trust_decision_deserialization() {
    assert_eq!(
        serde_json::from_str::<TrustDecision>("\"auto_accept\"").unwrap(),
        TrustDecision::AutoAccept
    );
    assert_eq!(
        serde_json::from_str::<TrustDecision>("\"prompt_user\"").unwrap(),
        TrustDecision::PromptUser
    );
    assert_eq!(
        serde_json::from_str::<TrustDecision>("\"reject\"").unwrap(),
        TrustDecision::Reject
    );
}

#[test]
fn test_trust_decision_roundtrip() {
    let decisions = vec![
        TrustDecision::AutoAccept,
        TrustDecision::PromptUser,
        TrustDecision::Reject,
    ];

    for decision in decisions {
        let json = serde_json::to_string(&decision).unwrap();
        let deserialized: TrustDecision = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, decision);
    }
}

// ====================================================================================
// TrustLevel Unit Tests
// ====================================================================================

#[test]
fn test_trust_level_serialization() {
    assert_eq!(
        serde_json::to_string(&TrustLevel::Highest).unwrap(),
        "\"highest\""
    );
    assert_eq!(
        serde_json::to_string(&TrustLevel::Elevated).unwrap(),
        "\"elevated\""
    );
    assert_eq!(
        serde_json::to_string(&TrustLevel::Limited).unwrap(),
        "\"limited\""
    );
    assert_eq!(
        serde_json::to_string(&TrustLevel::None).unwrap(),
        "\"none\""
    );
}

#[test]
fn test_trust_level_deserialization() {
    assert_eq!(
        serde_json::from_str::<TrustLevel>("\"highest\"").unwrap(),
        TrustLevel::Highest
    );
    assert_eq!(
        serde_json::from_str::<TrustLevel>("\"elevated\"").unwrap(),
        TrustLevel::Elevated
    );
    assert_eq!(
        serde_json::from_str::<TrustLevel>("\"limited\"").unwrap(),
        TrustLevel::Limited
    );
    assert_eq!(
        serde_json::from_str::<TrustLevel>("\"none\"").unwrap(),
        TrustLevel::None
    );
}

#[test]
fn test_trust_level_roundtrip() {
    let levels = vec![
        TrustLevel::Highest,
        TrustLevel::Elevated,
        TrustLevel::Limited,
        TrustLevel::None,
    ];

    for level in levels {
        let json = serde_json::to_string(&level).unwrap();
        let deserialized: TrustLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, level);
    }
}

// ====================================================================================
// Edge Cases and Validation Tests
// ====================================================================================

#[test]
fn test_large_attestation_data() {
    let large_data = json!({
        "tags": (0..1000).map(|i| format!("tag_{}", i)).collect::<Vec<_>>(),
        "metadata": (0..100).map(|i| (format!("key_{}", i), format!("value_{}", i))).collect::<std::collections::HashMap<_, _>>()
    });

    let attestation = IdentityAttestation {
        provider_capability: "test/large".to_string(),
        format: "tag_list".to_string(),
        data: large_data,
    };

    let serialized = serde_json::to_string(&attestation).unwrap();
    let deserialized: IdentityAttestation = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.provider_capability, "test/large");
    assert!(
        deserialized
            .data
            .get("tags")
            .unwrap()
            .as_array()
            .unwrap()
            .len()
            == 1000
    );
}

#[test]
fn test_unicode_in_identifiers() {
    let state = TrustApiState::new(Some("famille-français".to_string()), "nœud-测试");

    assert_eq!(state.family_id, Some("famille-français".to_string()));
    assert!(state.encryption_tag.contains("famille-français"));
    assert!(state.encryption_tag.contains("nœud-测试"));
}

#[test]
fn test_empty_capabilities() {
    let state = TrustApiState {
        family_id: None,
        encryption_tag: "test".to_string(),
        capabilities: vec![],
    };

    assert_eq!(state.capabilities.len(), 0);
}

#[test]
fn test_confidence_precision() {
    let response = TrustEvaluationResponse {
        response_format: Some("universal_trust_v1".to_string()),
        decision: TrustDecision::PromptUser,
        confidence: 0.123456789,
        reason: "Test".to_string(),
        reason_code: Some("test".to_string()),
        metadata: None,
        expires_at: None,
        trust_level_numeric: None,
        allowed_capabilities: None,
        denied_capabilities: None,
        elevation_path: None,
        trust_level: None,
        encryption_tag: None,
    };

    let json = serde_json::to_value(&response).unwrap();
    let confidence = json["confidence"].as_f64().unwrap();
    assert!((confidence - 0.123456789).abs() < 1e-9);
}

#[test]
fn test_metadata_with_nested_objects() {
    let metadata = json!({
        "level1": {
            "level2": {
                "level3": "deep_value"
            }
        },
        "array": [1, 2, 3]
    });

    let response = TrustEvaluationResponse {
        response_format: Some("universal_trust_v1".to_string()),
        decision: TrustDecision::AutoAccept,
        confidence: 1.0,
        reason: "Test".to_string(),
        reason_code: Some("test".to_string()),
        metadata: Some(metadata.clone()),
        expires_at: None,
        trust_level_numeric: None,
        allowed_capabilities: None,
        denied_capabilities: None,
        elevation_path: None,
        trust_level: None,
        encryption_tag: None,
    };

    let json = serde_json::to_value(&response).unwrap();
    assert_eq!(json["metadata"]["level1"]["level2"]["level3"], "deep_value");
}
