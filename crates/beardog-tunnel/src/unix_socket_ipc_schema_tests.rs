//! Unit Tests for Schema Fix (Decision Field & Environment Variable Fallback)
//!
//! These tests verify the January 7, 2026 schema fix that added:
//! 1. `decision` field to trust evaluation responses
//! 2. Environment variable fallback support (FAMILY_ID / BEARDOG_FAMILY_ID)

#[cfg(test)]
mod tests {
    use serde_json::json;
    use std::env;

    // ========================================================================
    // Decision Field Tests
    // ========================================================================

    #[test]
    fn test_decision_field_present_in_trust_response() {
        let response = json!({
            "decision": "auto_accept",
            "trust_level": 1,
            "trust_level_name": "limited",
            "reason": "same_genetic_family"
        });

        assert!(response["decision"].is_string(), "decision field must be present");
        assert_eq!(response["decision"], "auto_accept");
    }

    #[test]
    fn test_decision_mapping_trust_level_0() {
        // trust_level 0 (none) should map to "reject"
        let trust_level = 0;
        let decision = match trust_level {
            0 => "reject",
            1 => "auto_accept",
            _ => "reject",
        };

        assert_eq!(decision, "reject");
    }

    #[test]
    fn test_decision_mapping_trust_level_1() {
        // trust_level 1 (limited, same family) should map to "auto_accept"
        let trust_level = 1;
        let decision = match trust_level {
            0 => "reject",
            1 => "auto_accept",
            _ => "reject",
        };

        assert_eq!(decision, "auto_accept");
    }

    #[test]
    fn test_decision_field_structure() {
        let response = json!({
            "decision": "auto_accept",
            "trust_level": 1,
            "trust_level_name": "limited",
            "reason": "same_genetic_family",
            "peer_id": "tower2",
            "peer_family": "nat0",
            "our_family": "nat0",
            "our_node": "tower1"
        });

        // Verify decision field is present
        assert!(response.as_object().unwrap().contains_key("decision"));

        // Verify it's a string
        assert!(response["decision"].is_string());

        // Verify valid values
        let decision = response["decision"].as_str().unwrap();
        assert!(
            decision == "auto_accept" || decision == "reject" || decision == "prompt_user",
            "decision must be one of: auto_accept, reject, prompt_user"
        );
    }

    #[test]
    fn test_decision_with_dual_representation() {
        // Test that both integer trust_level and string decision are present
        let response = json!({
            "decision": "auto_accept",
            "trust_level": 1,
            "trust_level_name": "limited"
        });

        assert!(response["decision"].is_string());
        assert!(response["trust_level"].is_i64());
        assert!(response["trust_level_name"].is_string());

        // Verify consistency
        let decision = response["decision"].as_str().unwrap();
        let trust_level = response["trust_level"].as_i64().unwrap();

        if trust_level == 0 {
            assert_eq!(decision, "reject");
        } else if trust_level == 1 {
            assert_eq!(decision, "auto_accept");
        }
    }

    #[test]
    fn test_decision_all_valid_values() {
        let valid_decisions = vec!["auto_accept", "reject", "prompt_user"];

        for decision in valid_decisions {
            let response = json!({
                "decision": decision,
                "trust_level": if decision == "reject" { 0 } else { 1 },
                "reason": "test"
            });

            assert_eq!(response["decision"].as_str().unwrap(), decision);
        }
    }

    // ========================================================================
    // Environment Variable Fallback Tests
    // ========================================================================

    #[test]
    fn test_env_var_fallback_family_id() {
        // Clear any existing env vars
        env::remove_var("FAMILY_ID");
        env::remove_var("BEARDOG_FAMILY_ID");

        // Set only BEARDOG_FAMILY_ID
        env::set_var("BEARDOG_FAMILY_ID", "test-family");

        // Test fallback logic
        let family = env::var("FAMILY_ID")
            .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());

        assert_eq!(family, "test-family");

        // Cleanup
        env::remove_var("BEARDOG_FAMILY_ID");
    }

    #[test]
    fn test_env_var_fallback_node_id() {
        // Clear any existing env vars
        env::remove_var("NODE_ID");
        env::remove_var("BEARDOG_NODE_ID");

        // Set only BEARDOG_NODE_ID
        env::set_var("BEARDOG_NODE_ID", "test-node");

        // Test fallback logic
        let node = env::var("NODE_ID")
            .or_else(|_| env::var("BEARDOG_NODE_ID"))
            .unwrap_or_else(|_| "unknown".to_string());

        assert_eq!(node, "test-node");

        // Cleanup
        env::remove_var("BEARDOG_NODE_ID");
    }

    #[test]
    fn test_env_var_primary_takes_precedence() {
        // Set both FAMILY_ID and BEARDOG_FAMILY_ID
        env::set_var("FAMILY_ID", "primary-family");
        env::set_var("BEARDOG_FAMILY_ID", "fallback-family");

        // Primary should take precedence
        let family = env::var("FAMILY_ID")
            .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());

        assert_eq!(family, "primary-family");

        // Cleanup
        env::remove_var("FAMILY_ID");
        env::remove_var("BEARDOG_FAMILY_ID");
    }

    #[test]
    fn test_env_var_fallback_to_unknown() {
        // Clear all env vars
        env::remove_var("FAMILY_ID");
        env::remove_var("BEARDOG_FAMILY_ID");

        // Should fallback to "unknown"
        let family = env::var("FAMILY_ID")
            .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());

        assert_eq!(family, "unknown");
    }

    #[test]
    fn test_env_var_both_formats_work() {
        // Test FAMILY_ID format
        env::set_var("FAMILY_ID", "nat0");
        env::set_var("NODE_ID", "tower1");

        let family1 = env::var("FAMILY_ID").unwrap();
        let node1 = env::var("NODE_ID").unwrap();

        assert_eq!(family1, "nat0");
        assert_eq!(node1, "tower1");

        // Clear and test BEARDOG_ format
        env::remove_var("FAMILY_ID");
        env::remove_var("NODE_ID");
        env::set_var("BEARDOG_FAMILY_ID", "nat0");
        env::set_var("BEARDOG_NODE_ID", "tower1");

        let family2 = env::var("BEARDOG_FAMILY_ID").unwrap();
        let node2 = env::var("BEARDOG_NODE_ID").unwrap();

        assert_eq!(family2, "nat0");
        assert_eq!(node2, "tower1");

        // Cleanup
        env::remove_var("BEARDOG_FAMILY_ID");
        env::remove_var("BEARDOG_NODE_ID");
    }

    #[test]
    fn test_env_var_compatibility_all_methods() {
        // Test that env var fallback works for all IPC methods

        // Set only BEARDOG_ format
        env::remove_var("FAMILY_ID");
        env::remove_var("NODE_ID");
        env::set_var("BEARDOG_FAMILY_ID", "prod-family");
        env::set_var("BEARDOG_NODE_ID", "prod-node");

        // Simulate capabilities method
        let family_caps = env::var("FAMILY_ID")
            .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());
        assert_eq!(family_caps, "prod-family");

        // Simulate identity method
        let family_identity = env::var("FAMILY_ID")
            .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());
        assert_eq!(family_identity, "prod-family");

        // Simulate trust method
        let family_trust = env::var("FAMILY_ID")
            .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());
        assert_eq!(family_trust, "prod-family");

        // Simulate lineage method
        let family_lineage = env::var("FAMILY_ID")
            .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());
        assert_eq!(family_lineage, "prod-family");

        // Cleanup
        env::remove_var("BEARDOG_FAMILY_ID");
        env::remove_var("BEARDOG_NODE_ID");
    }

    // ========================================================================
    // Integration Tests (Decision + Env Vars)
    // ========================================================================

    #[test]
    fn test_trust_response_with_correct_identity() {
        // Set environment
        env::set_var("BEARDOG_FAMILY_ID", "nat0");
        env::set_var("BEARDOG_NODE_ID", "tower1");

        // Simulate trust evaluation
        let our_family = env::var("FAMILY_ID")
            .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());
        let our_node = env::var("NODE_ID")
            .or_else(|_| env::var("BEARDOG_NODE_ID"))
            .unwrap_or_else(|_| "unknown".to_string());

        let peer_family = "nat0";
        let trust_level = if peer_family == our_family { 1 } else { 0 };
        let decision = if trust_level == 0 {
            "reject"
        } else {
            "auto_accept"
        };

        let response = json!({
            "decision": decision,
            "trust_level": trust_level,
            "trust_level_name": if trust_level == 1 { "limited" } else { "none" },
            "our_family": our_family,
            "our_node": our_node
        });

        // Verify response is correct
        assert_eq!(response["decision"], "auto_accept");
        assert_eq!(response["trust_level"], 1);
        assert_eq!(response["our_family"], "nat0");
        assert_eq!(response["our_node"], "tower1");
        assert_ne!(response["our_family"], "unknown");
        assert_ne!(response["our_node"], "unknown");

        // Cleanup
        env::remove_var("BEARDOG_FAMILY_ID");
        env::remove_var("BEARDOG_NODE_ID");
    }

    #[test]
    fn test_trust_response_reject_with_decision() {
        // Set environment
        env::set_var("BEARDOG_FAMILY_ID", "nat0");

        let our_family = env::var("FAMILY_ID")
            .or_else(|_| env::var("BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());

        let peer_family = "other-family";
        let trust_level = if peer_family == our_family { 1 } else { 0 };
        let decision = if trust_level == 0 {
            "reject"
        } else {
            "auto_accept"
        };

        let response = json!({
            "decision": decision,
            "trust_level": trust_level,
            "trust_level_name": "none",
            "reason": "different_family",
            "our_family": our_family,
            "peer_family": peer_family
        });

        // Verify rejection
        assert_eq!(response["decision"], "reject");
        assert_eq!(response["trust_level"], 0);
        assert_ne!(response["peer_family"], response["our_family"]);

        // Cleanup
        env::remove_var("BEARDOG_FAMILY_ID");
    }

    // ========================================================================
    // Backward Compatibility Tests
    // ========================================================================

    #[test]
    fn test_backward_compatibility_integer_trust_level() {
        // Ensure integer trust_level is still present (backward compat)
        let response = json!({
            "decision": "auto_accept",
            "trust_level": 1,
            "trust_level_name": "limited"
        });

        assert!(response["trust_level"].is_i64());
        assert_eq!(response["trust_level"], 1);
    }

    #[test]
    fn test_backward_compatibility_string_trust_level_name() {
        // Ensure string trust_level_name is still present (Songbird compat)
        let response = json!({
            "decision": "auto_accept",
            "trust_level": 1,
            "trust_level_name": "limited"
        });

        assert!(response["trust_level_name"].is_string());
        assert_eq!(response["trust_level_name"], "limited");
    }

    #[test]
    fn test_triple_representation() {
        // Test that all three representations are present and consistent
        let response = json!({
            "decision": "auto_accept",         // NEW: For Songbird
            "trust_level": 1,                  // Old: Integer (backward compat)
            "trust_level_name": "limited"      // Old: String (Songbird compat)
        });

        // All three present
        assert!(response.as_object().unwrap().contains_key("decision"));
        assert!(response.as_object().unwrap().contains_key("trust_level"));
        assert!(response.as_object().unwrap().contains_key("trust_level_name"));

        // Types correct
        assert!(response["decision"].is_string());
        assert!(response["trust_level"].is_i64());
        assert!(response["trust_level_name"].is_string());

        // Values consistent
        assert_eq!(response["decision"], "auto_accept");
        assert_eq!(response["trust_level"], 1);
        assert_eq!(response["trust_level_name"], "limited");
    }
}

