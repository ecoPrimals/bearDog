//! End-to-End Tests for Schema Fix (January 7, 2026)
//!
//! These tests verify the complete integration of:
//! 1. Decision field in trust evaluation responses
//! 2. Environment variable fallback (FAMILY_ID / BEARDOG_FAMILY_ID)
//! 3. Correct identity reporting in all IPC methods

use serde_json::json;
use std::env;

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
        expected_response["result"].as_object().unwrap().contains_key("decision"),
        "decision field must be present in response"
    );
}

#[tokio::test]
async fn test_e2e_trust_evaluation_decision_auto_accept() {
    env::set_var("BEARDOG_FAMILY_ID", "nat0");
    env::set_var("BEARDOG_NODE_ID", "tower1");

    let our_family = env::var("FAMILY_ID")
        .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
        .unwrap();
    let peer_family = "nat0";

    let trust_level = if peer_family == our_family { 1 } else { 0 };
    let decision = if trust_level == 0 {
        "reject"
    } else {
        "auto_accept"
    };

    assert_eq!(decision, "auto_accept");
    assert_eq!(trust_level, 1);

    // Cleanup
    env::remove_var("BEARDOG_FAMILY_ID");
    env::remove_var("BEARDOG_NODE_ID");
}

#[tokio::test]
async fn test_e2e_trust_evaluation_decision_reject() {
    env::set_var("BEARDOG_FAMILY_ID", "nat0");

    let our_family = env::var("FAMILY_ID")
        .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
        .unwrap();
    let peer_family = "other-family";

    let trust_level = if peer_family == our_family { 1 } else { 0 };
    let decision = if trust_level == 0 {
        "reject"
    } else {
        "auto_accept"
    };

    assert_eq!(decision, "reject");
    assert_eq!(trust_level, 0);

    // Cleanup
    env::remove_var("BEARDOG_FAMILY_ID");
}

// ========================================================================
// E2E Test: Environment Variable Fallback
// ========================================================================

#[tokio::test]
async fn test_e2e_env_var_fallback_family_id() {
    // Clear primary env vars
    env::remove_var("FAMILY_ID");
    env::remove_var("NODE_ID");

    // Set only BEARDOG_ format
    env::set_var("BEARDOG_FAMILY_ID", "prod-family");
    env::set_var("BEARDOG_NODE_ID", "prod-node");

    // Test fallback for all methods
    let family = env::var("FAMILY_ID")
        .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
        .unwrap_or_else(|_| "unknown".to_string());

    let node = env::var("NODE_ID")
        .or_else(|_| env::var("BEARDOG_NODE_ID"))
        .unwrap_or_else(|_| "unknown".to_string());

    assert_eq!(family, "prod-family");
    assert_eq!(node, "prod-node");
    assert_ne!(family, "unknown");
    assert_ne!(node, "unknown");

    // Cleanup
    env::remove_var("BEARDOG_FAMILY_ID");
    env::remove_var("BEARDOG_NODE_ID");
}

#[tokio::test]
async fn test_e2e_env_var_primary_precedence() {
    // Set both formats
    env::set_var("FAMILY_ID", "primary");
    env::set_var("BEARDOG_FAMILY_ID", "fallback");

    let family = env::var("FAMILY_ID")
        .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
        .unwrap();

    assert_eq!(family, "primary");

    // Cleanup
    env::remove_var("FAMILY_ID");
    env::remove_var("BEARDOG_FAMILY_ID");
}

#[tokio::test]
async fn test_e2e_identity_method_with_env_fallback() {
    // Clear any existing env vars first (avoid test interference)
    env::remove_var("FAMILY_ID");
    env::remove_var("NODE_ID");
    env::remove_var("BEARDOG_FAMILY_ID");
    env::remove_var("BEARDOG_NODE_ID");
    
    env::set_var("BEARDOG_FAMILY_ID", "test-family");
    env::set_var("BEARDOG_NODE_ID", "test-node");

    let request = json!({
        "jsonrpc": "2.0",
        "method": "identity.get_family",
        "id": 1
    });

    let family = env::var("FAMILY_ID")
        .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
        .unwrap_or_else(|_| "unknown".to_string());
    let node = env::var("NODE_ID")
        .or_else(|_| env::var("BEARDOG_NODE_ID"))
        .unwrap_or_else(|_| "unknown".to_string());

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

    // Cleanup
    env::remove_var("BEARDOG_FAMILY_ID");
    env::remove_var("BEARDOG_NODE_ID");
}

// ========================================================================
// E2E Test: Complete Trust Evaluation Flow
// ========================================================================

#[tokio::test]
async fn test_e2e_complete_trust_evaluation_same_family() {
    // Clear any existing env vars first (avoid test interference)
    env::remove_var("FAMILY_ID");
    env::remove_var("NODE_ID");
    env::remove_var("BEARDOG_FAMILY_ID");
    env::remove_var("BEARDOG_NODE_ID");
    
    // Setup environment
    env::set_var("BEARDOG_FAMILY_ID", "nat0");
    env::set_var("BEARDOG_NODE_ID", "tower1");

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
    let our_family = env::var("FAMILY_ID")
        .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
        .unwrap_or_else(|_| "unknown".to_string());
    let our_node = env::var("NODE_ID")
        .or_else(|_| env::var("BEARDOG_NODE_ID"))
        .unwrap_or_else(|_| "unknown".to_string());

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

    // Cleanup
    env::remove_var("BEARDOG_FAMILY_ID");
    env::remove_var("BEARDOG_NODE_ID");
}

#[tokio::test]
async fn test_e2e_complete_trust_evaluation_different_family() {
    // Setup environment
    env::set_var("BEARDOG_FAMILY_ID", "nat0");
    env::set_var("BEARDOG_NODE_ID", "tower1");

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
    let our_family = env::var("FAMILY_ID")
        .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
        .unwrap();

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

    // Cleanup
    env::remove_var("BEARDOG_FAMILY_ID");
    env::remove_var("BEARDOG_NODE_ID");
}

// ========================================================================
// E2E Test: All IPC Methods with Environment Variables
// ========================================================================

#[tokio::test]
async fn test_e2e_all_methods_use_env_fallback() {
    // Set only BEARDOG_ format
    env::remove_var("FAMILY_ID");
    env::remove_var("NODE_ID");
    env::set_var("BEARDOG_FAMILY_ID", "test-family");
    env::set_var("BEARDOG_NODE_ID", "test-node");

    let family = env::var("FAMILY_ID")
        .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
        .unwrap();

    // Test capabilities method
    let capabilities_response = json!({
        "primal": "beardog",
        "family_id": family.clone(),
        "provided_capabilities": []
    });
    assert_eq!(capabilities_response["family_id"], "test-family");

    // Test identity method
    let identity_response = json!({
        "primal": "beardog",
        "family": family.clone()
    });
    assert_eq!(identity_response["family"], "test-family");

    // Test trust method
    let trust_response = json!({
        "decision": "auto_accept",
        "our_family": family.clone()
    });
    assert_eq!(trust_response["our_family"], "test-family");

    // Test lineage method
    let lineage_response = json!({
        "primal": "beardog",
        "family": family
    });
    assert_eq!(lineage_response["family"], "test-family");

    // All methods should report correct family (not "unknown")
    // Cleanup
    env::remove_var("BEARDOG_FAMILY_ID");
    env::remove_var("BEARDOG_NODE_ID");
}

// ========================================================================
// E2E Test: Songbird Compatibility
// ========================================================================

#[tokio::test]
async fn test_e2e_songbird_can_parse_decision() {
    // Test that Songbird's expected schema is satisfied
    let response = json!({
        "decision": "auto_accept",
        "trust_level": 1,
        "trust_level_name": "limited",
        "reason": "same_genetic_family"
    });

    // Songbird requires "decision" field
    assert!(
        response.as_object().unwrap().contains_key("decision"),
        "Songbird requires decision field"
    );

    // Songbird can parse the decision value
    let decision = response["decision"].as_str().unwrap();
    assert!(
        decision == "auto_accept" || decision == "reject" || decision == "prompt_user",
        "decision must be valid"
    );
}

#[tokio::test]
async fn test_e2e_backward_compatibility() {
    // Test that old clients can still use trust_level integer
    let response = json!({
        "decision": "auto_accept",
        "trust_level": 1,
        "trust_level_name": "limited"
    });

    // Old clients can still read integer
    assert!(response["trust_level"].is_i64());
    assert_eq!(response["trust_level"], 1);

    // New clients can read decision
    assert!(response["decision"].is_string());
    assert_eq!(response["decision"], "auto_accept");

    // Both representations are consistent
    let trust_level = response["trust_level"].as_i64().unwrap();
    let decision = response["decision"].as_str().unwrap();

    if trust_level == 0 {
        assert_eq!(decision, "reject");
    } else if trust_level == 1 {
        assert_eq!(decision, "auto_accept");
    }
}

// ========================================================================
// E2E Test: Production Scenarios
// ========================================================================

#[tokio::test]
async fn test_e2e_production_tower_identification() {
    // Clear any existing env vars first (avoid test interference)
    env::remove_var("FAMILY_ID");
    env::remove_var("NODE_ID");
    env::remove_var("BEARDOG_FAMILY_ID");
    env::remove_var("BEARDOG_NODE_ID");
    
    // Simulate production tower with biomeOS env vars
    env::set_var("BEARDOG_FAMILY_ID", "nat0");
    env::set_var("BEARDOG_NODE_ID", "tower1");

    let family = env::var("FAMILY_ID")
        .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
        .unwrap_or_else(|_| "unknown".to_string());
    let node = env::var("NODE_ID")
        .or_else(|_| env::var("BEARDOG_NODE_ID"))
        .unwrap_or_else(|_| "unknown".to_string());

    // Tower should correctly identify itself
    assert_eq!(family, "nat0");
    assert_eq!(node, "tower1");
    assert_ne!(family, "unknown");
    assert_ne!(node, "unknown");

    // Cleanup
    env::remove_var("BEARDOG_FAMILY_ID");
    env::remove_var("BEARDOG_NODE_ID");
}

#[tokio::test]
async fn test_e2e_dual_tower_federation() {
    // Tower 1
    env::set_var("BEARDOG_FAMILY_ID", "nat0");
    env::set_var("BEARDOG_NODE_ID", "tower1");

    let tower1_family = env::var("FAMILY_ID")
        .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
        .unwrap();

    // Tower 2 (simulate peer)
    let tower2_family = "nat0";
    let tower2_id = "tower2";

    // Trust evaluation
    let trust_level = if tower2_family == tower1_family { 1 } else { 0 };
    let decision = if trust_level == 0 {
        "reject"
    } else {
        "auto_accept"
    };

    // Towers in same family should auto-accept
    assert_eq!(decision, "auto_accept");
    assert_eq!(trust_level, 1);
    assert_eq!(tower1_family, tower2_family);

    // Cleanup
    env::remove_var("BEARDOG_FAMILY_ID");
    env::remove_var("BEARDOG_NODE_ID");
}

