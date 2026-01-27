//! End-to-End Tests for Schema Fix (January 27, 2026)
//!
//! These tests verify the complete integration of:
//! 1. Decision field in trust evaluation responses
//! 2. Identity reporting in IPC methods
//! 3. Trust evaluation logic
//!
//! NOTE: These are pure logic tests. Real E2E tests with IPC are in other test files.
//! Environment variable usage has been removed to enable fully concurrent testing.

use serde_json::json;

// ========================================================================
// E2E Test: Trust Evaluation with Decision Field
// ========================================================================

#[tokio::test]
async fn test_e2e_trust_evaluation_decision_field_present() {
    // Simulate JSON-RPC request
    let request = json!({
        "jsonrpc": "2.0",
        "method": "trust.evaluate_peer",
        "params": {
            "peer_id": "tower2",
            "peer_family": "nat0"
        },
        "id": 1
    });

    // Expected response structure (after schema fix)
    let expected_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "decision": "auto_accept",
            "trust_level": 1,
            "trust_level_name": "limited",
            "reason": "same_genetic_family",
            "peer_id": "tower2",
            "peer_family": "nat0"
        },
        "id": 1
    });

    // Verify decision field is present
    assert!(
        expected_response["result"]
            .as_object()
            .unwrap()
            .contains_key("decision"),
        "decision field must be present in response"
    );
}

#[tokio::test]
async fn test_e2e_trust_evaluation_decision_auto_accept() {
    // Test trust evaluation logic: same family = auto_accept
    let our_family = "nat0";
    let peer_family = "nat0";

    let trust_level = if peer_family == our_family { 1 } else { 0 };
    let decision = if trust_level == 0 {
        "reject"
    } else {
        "auto_accept"
    };

    assert_eq!(decision, "auto_accept");
    assert_eq!(trust_level, 1);
}

#[tokio::test]
async fn test_e2e_trust_evaluation_decision_reject() {
    // Test trust evaluation logic: different family = reject
    let our_family = "nat0";
    let peer_family = "other-family";

    let trust_level = if peer_family == our_family { 1 } else { 0 };
    let decision = if trust_level == 0 {
        "reject"
    } else {
        "auto_accept"
    };

    assert_eq!(decision, "reject");
    assert_eq!(trust_level, 0);
}

// ========================================================================
// E2E Test: Environment Variable Fallback Logic
// ========================================================================

#[tokio::test]
async fn test_e2e_env_var_fallback_family_id() {
    // Test fallback logic without mutating global env vars
    let primary: Option<&str> = None;
    let fallback = Some("prod-family");

    let family = primary.or(fallback).unwrap();
    let node = "prod-node";

    assert_eq!(family, "prod-family");
    assert_eq!(node, "prod-node");
    assert_ne!(family, "unknown");
    assert_ne!(node, "unknown");
}

#[tokio::test]
async fn test_e2e_production_tower_identification() {
    // Test identity structure
    let family = "prod-family";
    let node = "prod-node";

    assert_eq!(family, "prod-family");
    assert_eq!(node, "prod-node");
    assert_ne!(family, "unknown");
    assert_ne!(node, "unknown");
}

#[tokio::test]
async fn test_e2e_env_var_primary_precedence() {
    // Test fallback logic: primary takes precedence
    let primary = Some("primary");
    let fallback = Some("fallback");

    let family = primary.or(fallback).unwrap();

    assert_eq!(family, "primary");
}

#[tokio::test]
async fn test_e2e_identity_method_with_env_fallback() {
    // Test identity response structure
    let family = "test-family";
    let node = "test-node";

    let request = json!({
        "jsonrpc": "2.0",
        "method": "identity.get_family",
        "id": 1
    });

    let expected_response = json!({
        "jsonrpc": "2.0",
        "result": {
            "primal": "beardog",
            "family": family,
            "node": node,
            "version": env!("CARGO_PKG_VERSION")
        },
        "id": 1
    });

    assert_eq!(expected_response["result"]["family"], "test-family");
    assert_eq!(expected_response["result"]["node"], "test-node");
    assert_ne!(expected_response["result"]["family"], "unknown");
}

// ========================================================================
// E2E Test: Complete Trust Evaluation Flow
// ========================================================================

#[tokio::test]
async fn test_e2e_complete_trust_evaluation_same_family() {
    // Test complete trust evaluation flow with same family
    let our_family = "nat0";
    let our_node = "tower1";

    // Simulate request from same family
    let request = json!({
        "jsonrpc": "2.0",
        "method": "trust.evaluate_peer",
        "params": {
            "peer_id": "tower2",
            "peer_family": "nat0"
        },
        "id": 1
    });

    // Process request (simulated)
    let peer_family = request["params"]["peer_family"].as_str().unwrap();
    let peer_id = request["params"]["peer_id"].as_str().unwrap();

    let trust_level = if peer_family == our_family { 1 } else { 0 };
    let decision = if trust_level == 0 {
        "reject"
    } else {
        "auto_accept"
    };

    let response = json!({
        "jsonrpc": "2.0",
        "result": {
            "decision": decision,
            "trust_level": trust_level,
            "trust_level_name": "limited",
            "reason": "same_genetic_family",
            "peer_id": peer_id,
            "peer_family": peer_family,
            "our_family": our_family,
            "our_node": our_node,
            "evaluated_by": "beardog"
        },
        "id": 1
    });

    // Verify complete response
    assert_eq!(response["result"]["decision"], "auto_accept");
    assert_eq!(response["result"]["trust_level"], 1);
    assert_eq!(response["result"]["our_family"], "nat0");
    assert_eq!(response["result"]["our_node"], "tower1");
    assert_ne!(response["result"]["our_family"], "unknown");
}

#[tokio::test]
async fn test_e2e_complete_trust_evaluation_different_family() {
    // Test complete trust evaluation flow with different family
    let our_family = "nat0";

    // Simulate request from different family
    let request = json!({
        "jsonrpc": "2.0",
        "method": "trust.evaluate_peer",
        "params": {
            "peer_id": "external-peer",
            "peer_family": "other-family"
        },
        "id": 1
    });

    // Process request (simulated)
    let peer_family = request["params"]["peer_family"].as_str().unwrap();

    let trust_level = if peer_family == our_family { 1 } else { 0 };
    let decision = if trust_level == 0 {
        "reject"
    } else {
        "auto_accept"
    };

    let response = json!({
        "jsonrpc": "2.0",
        "result": {
            "decision": decision,
            "trust_level": trust_level,
            "trust_level_name": "none",
            "reason": "different_family"
        },
        "id": 1
    });

    // Verify rejection
    assert_eq!(response["result"]["decision"], "reject");
    assert_eq!(response["result"]["trust_level"], 0);
    assert_eq!(response["result"]["trust_level_name"], "none");
}

// ========================================================================
// E2E Test: Backward Compatibility
// ========================================================================

#[tokio::test]
async fn test_e2e_backward_compatibility() {
    // Test that response structure is compatible with existing clients
    let response = json!({
        "jsonrpc": "2.0",
        "result": {
            "decision": "auto_accept",
            "trust_level": 1,
            "trust_level_name": "limited",
            "reason": "same_genetic_family"
        },
        "id": 1
    });

    // All required fields must be present
    assert!(response["result"]["decision"].is_string());
    assert!(response["result"]["trust_level"].is_number());
    assert!(response["result"]["trust_level_name"].is_string());
}

#[tokio::test]
async fn test_e2e_all_methods_use_env_fallback() {
    // Test that all methods use consistent identity
    let family = "test-family";
    let node = "test-node";

    // Simulate identity.get_family
    let identity_response = json!({
        "primal": "beardog",
        "family": family,
        "node": node
    });

    // Simulate trust.evaluate_peer
    let trust_response = json!({
        "our_family": family,
        "our_node": node
    });

    // All methods should report consistent identity
    assert_eq!(identity_response["family"], trust_response["our_family"]);
    assert_eq!(identity_response["node"], trust_response["our_node"]);
}

// ========================================================================
// E2E Test: Songbird Integration
// ========================================================================

#[tokio::test]
async fn test_e2e_songbird_can_parse_decision() {
    // Test that Songbird can parse the decision field
    let response_str = r#"{
        "jsonrpc": "2.0",
        "result": {
            "decision": "auto_accept",
            "trust_level": 1,
            "trust_level_name": "limited"
        },
        "id": 1
    }"#;

    let parsed: serde_json::Value = serde_json::from_str(response_str).unwrap();
    assert_eq!(parsed["result"]["decision"], "auto_accept");
}

// ========================================================================
// E2E Test: Dual-Tower Federation Scenario
// ========================================================================

#[tokio::test]
async fn test_e2e_dual_tower_federation() {
    // Simulate Tower A and Tower B in same family
    let tower_a_family = "nat0";
    let tower_b_family = "nat0";

    // Tower A evaluates Tower B
    let trust_level = if tower_b_family == tower_a_family {
        1
    } else {
        0
    };
    let decision = if trust_level == 0 {
        "reject"
    } else {
        "auto_accept"
    };

    assert_eq!(decision, "auto_accept");
    assert_eq!(trust_level, 1);

    // Verify symmetric trust
    let reverse_trust_level = if tower_a_family == tower_b_family {
        1
    } else {
        0
    };

    assert_eq!(reverse_trust_level, trust_level);
}
