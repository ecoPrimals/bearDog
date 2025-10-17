// Capability Discovery Tests
//
// Testing capability discovery functionality

use crate::universal::capability_discovery::*;
use beardog_types::adapters::CapabilityType;

#[test]
fn test_discovery_request_creation() {
    let request = CapabilityDiscoveryRequest {
        required_capabilities: vec![CapabilityType::Security, CapabilityType::Compute],
        constraints: std::collections::HashMap::new(),
    };

    assert_eq!(request.required_capabilities.len(), 2);
    assert!(request.constraints.is_empty());
}

#[test]
fn test_discovery_request_with_constraints() {
    let mut constraints = std::collections::HashMap::new();
    constraints.insert("region".to_string(), "us-west".to_string());
    constraints.insert("availability".to_string(), "high".to_string());

    let request = CapabilityDiscoveryRequest {
        required_capabilities: vec![CapabilityType::Storage],
        constraints,
    };

    assert_eq!(request.constraints.len(), 2);
    assert_eq!(request.constraints.get("region"), Some(&"us-west".to_string()));
}

#[test]
fn test_discovery_response_creation() {
    let response = CapabilityDiscoveryResponse {
        discovered_providers: vec![
            "provider1".to_string(),
            "provider2".to_string(),
        ],
        confidence_scores: std::collections::HashMap::new(),
    };

    assert_eq!(response.discovered_providers.len(), 2);
}

#[test]
fn test_discovery_response_with_scores() {
    let mut scores = std::collections::HashMap::new();
    scores.insert("provider1".to_string(), 0.95);
    scores.insert("provider2".to_string(), 0.87);

    let response = CapabilityDiscoveryResponse {
        discovered_providers: vec!["provider1".to_string(), "provider2".to_string()],
        confidence_scores: scores,
    };

    assert_eq!(response.confidence_scores.len(), 2);
    assert!(response.confidence_scores["provider1"] > 0.9);
}

#[test]
fn test_discovery_request_single_capability() {
    let request = CapabilityDiscoveryRequest {
        required_capabilities: vec![CapabilityType::Monitoring],
        constraints: std::collections::HashMap::new(),
    };

    assert_eq!(request.required_capabilities.len(), 1);
}

#[test]
fn test_discovery_response_no_providers() {
    let response = CapabilityDiscoveryResponse {
        discovered_providers: vec![],
        confidence_scores: std::collections::HashMap::new(),
    };

    assert!(response.discovered_providers.is_empty());
    assert!(response.confidence_scores.is_empty());
}

#[test]
fn test_discovery_request_multiple_capabilities() {
    let request = CapabilityDiscoveryRequest {
        required_capabilities: vec![
            CapabilityType::Security,
            CapabilityType::Compute,
            CapabilityType::Storage,
            CapabilityType::Networking,
        ],
        constraints: std::collections::HashMap::new(),
    };

    assert_eq!(request.required_capabilities.len(), 4);
}

#[test]
fn test_discovery_confidence_score_range() {
    let mut scores = std::collections::HashMap::new();
    scores.insert("high_confidence".to_string(), 0.99);
    scores.insert("medium_confidence".to_string(), 0.75);
    scores.insert("low_confidence".to_string(), 0.45);

    let response = CapabilityDiscoveryResponse {
        discovered_providers: vec![
            "high_confidence".to_string(),
            "medium_confidence".to_string(),
            "low_confidence".to_string(),
        ],
        confidence_scores: scores.clone(),
    };

    assert!(scores["high_confidence"] >= 0.0 && scores["high_confidence"] <= 1.0);
    assert!(scores["medium_confidence"] >= 0.0 && scores["medium_confidence"] <= 1.0);
    assert!(scores["low_confidence"] >= 0.0 && scores["low_confidence"] <= 1.0);
}

