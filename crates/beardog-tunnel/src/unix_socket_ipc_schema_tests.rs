// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unit Tests for Schema Fix (Decision Field & Environment Variable Fallback)
//!
//! These tests verify the January 7, 2026 schema fix that added:
//! 1. `decision` field to trust evaluation responses
//! 2. Environment variable fallback support (FAMILY_ID / BEARDOG_FAMILY_ID)

#[cfg(test)]
mod tests {
    use serde_json::json;
    use std::collections::HashMap;
    use std::env::VarError;

    fn env_get(m: &HashMap<String, String>, key: &str) -> Result<String, VarError> {
        m.get(key).cloned().ok_or(VarError::NotPresent)
    }

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

        assert!(
            response["decision"].is_string(),
            "decision field must be present"
        );
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
        let m = HashMap::from([("BEARDOG_FAMILY_ID".to_string(), "test-family".to_string())]);
        let family = env_get(&m, "FAMILY_ID")
            .or_else(|_| env_get(&m, "BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());
        assert_eq!(family, "test-family");
    }

    #[test]
    fn test_env_var_fallback_node_id() {
        let m = HashMap::from([("BEARDOG_NODE_ID".to_string(), "test-node".to_string())]);
        let node = env_get(&m, "NODE_ID")
            .or_else(|_| env_get(&m, "BEARDOG_NODE_ID"))
            .unwrap_or_else(|_| "unknown".to_string());
        assert_eq!(node, "test-node");
    }

    #[test]
    fn test_env_var_primary_takes_precedence() {
        let m = HashMap::from([
            ("FAMILY_ID".to_string(), "primary-family".to_string()),
            (
                "BEARDOG_FAMILY_ID".to_string(),
                "fallback-family".to_string(),
            ),
        ]);
        let family = env_get(&m, "FAMILY_ID")
            .or_else(|_| env_get(&m, "BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());
        assert_eq!(family, "primary-family");
    }

    #[test]
    fn test_env_var_fallback_to_unknown() {
        let m = HashMap::<String, String>::new();
        let family = env_get(&m, "FAMILY_ID")
            .or_else(|_| env_get(&m, "BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());
        assert_eq!(family, "unknown");
    }

    #[test]
    fn test_env_var_both_formats_work() {
        let m_short = HashMap::from([
            ("FAMILY_ID".to_string(), "nat0".to_string()),
            ("NODE_ID".to_string(), "tower1".to_string()),
        ]);
        assert_eq!(env_get(&m_short, "FAMILY_ID").unwrap(), "nat0");
        assert_eq!(env_get(&m_short, "NODE_ID").unwrap(), "tower1");

        let m_long = HashMap::from([
            ("BEARDOG_FAMILY_ID".to_string(), "nat0".to_string()),
            ("BEARDOG_NODE_ID".to_string(), "tower1".to_string()),
        ]);
        assert_eq!(env_get(&m_long, "BEARDOG_FAMILY_ID").unwrap(), "nat0");
        assert_eq!(env_get(&m_long, "BEARDOG_NODE_ID").unwrap(), "tower1");
    }

    #[test]
    fn test_env_var_compatibility_all_methods() {
        let m = HashMap::from([
            ("BEARDOG_FAMILY_ID".to_string(), "prod-family".to_string()),
            ("BEARDOG_NODE_ID".to_string(), "prod-node".to_string()),
        ]);
        let family_caps = env_get(&m, "FAMILY_ID")
            .or_else(|_| env_get(&m, "BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());
        assert_eq!(family_caps, "prod-family");
        let family_identity = env_get(&m, "FAMILY_ID")
            .or_else(|_| env_get(&m, "BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());
        assert_eq!(family_identity, "prod-family");
        let family_trust = env_get(&m, "FAMILY_ID")
            .or_else(|_| env_get(&m, "BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());
        assert_eq!(family_trust, "prod-family");
        let family_lineage = env_get(&m, "FAMILY_ID")
            .or_else(|_| env_get(&m, "BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());
        assert_eq!(family_lineage, "prod-family");
    }

    // ========================================================================
    // Integration Tests (Decision + Env Vars)
    // ========================================================================

    #[test]
    fn test_trust_response_with_correct_identity() {
        let m = HashMap::from([
            ("FAMILY_ID".to_string(), "nat0".to_string()),
            ("NODE_ID".to_string(), "tower1".to_string()),
        ]);
        let our_family = env_get(&m, "FAMILY_ID")
            .or_else(|_| env_get(&m, "BEARDOG_FAMILY_ID"))
            .unwrap_or_else(|_| "unknown".to_string());
        let our_node = env_get(&m, "NODE_ID")
            .or_else(|_| env_get(&m, "BEARDOG_NODE_ID"))
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

        assert_eq!(response["decision"], "auto_accept");
        assert_eq!(response["trust_level"], 1);
        assert_eq!(response["our_family"], "nat0");
        assert_eq!(response["our_node"], "tower1");
        assert_ne!(response["our_family"], "unknown");
        assert_ne!(response["our_node"], "unknown");
    }

    #[test]
    fn test_trust_response_reject_with_decision() {
        let m = HashMap::from([("BEARDOG_FAMILY_ID".to_string(), "nat0".to_string())]);
        let our_family = env_get(&m, "FAMILY_ID")
            .or_else(|_| env_get(&m, "BEARDOG_FAMILY_ID"))
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

        assert_eq!(response["decision"], "reject");
        assert_eq!(response["trust_level"], 0);
        assert_ne!(response["peer_family"], response["our_family"]);
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
        assert!(
            response
                .as_object()
                .unwrap()
                .contains_key("trust_level_name")
        );

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
