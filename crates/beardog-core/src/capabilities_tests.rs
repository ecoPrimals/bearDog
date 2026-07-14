// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for capability system
//!
//! Covers: Unit, E2E, Chaos, and Fault testing

/// Helper to get expected primal name from environment or default
fn expected_primal_name() -> String {
    beardog_errors::process_env::var("PRIMAL_NAME")
        .or_else(|_| beardog_errors::process_env::var("BEARDOG_NAME"))
        .unwrap_or_else(|_| "beardog".to_string())
}

#[cfg(test)]
mod capability_unit_tests {
    use super::super::*;
    use super::expected_primal_name;
    use std::collections::BTreeMap;

    // ============================================================================
    // Unit Tests - Capability Manifest
    // ============================================================================

    #[test]
    fn test_capability_manifest_creation() {
        let caps = BearDogCapabilities::new(Some("test_family".to_string()), "node1".to_string());

        assert_eq!(caps.primal_id, expected_primal_name());
        assert_eq!(caps.family_id, Some("test_family".to_string()));
        assert_eq!(caps.node_id, "node1");
        assert!(!caps.provides.is_empty());
        assert!(!caps.endpoints.is_empty());
    }

    #[test]
    fn test_capability_manifest_no_family() {
        let caps = BearDogCapabilities::new(None, "node1".to_string());

        assert_eq!(caps.primal_id, expected_primal_name());
        assert_eq!(caps.family_id, None);
        assert_eq!(caps.node_id, "node1");
    }

    #[test]
    fn test_provides_encryption_capability() {
        let caps = BearDogCapabilities::new(None, "node1".to_string());

        let encryption_request = Capability::Encryption {
            algorithms: vec!["any".to_string()],
            key_types: vec!["any".to_string()],
        };

        assert!(caps.provides_capability(&encryption_request));
    }

    #[test]
    fn test_provides_trust_evaluation_capability() {
        let caps = BearDogCapabilities::new(None, "node1".to_string());

        let trust_request = Capability::TrustEvaluation {
            trust_models: vec!["any".to_string()],
        };

        assert!(caps.provides_capability(&trust_request));
    }

    #[test]
    fn test_provides_key_management_capability() {
        let caps = BearDogCapabilities::new(None, "node1".to_string());

        let km_request = Capability::KeyManagement {
            hsm_types: vec!["any".to_string()],
        };

        assert!(caps.provides_capability(&km_request));
    }

    #[test]
    fn test_provides_signatures_capability() {
        let caps = BearDogCapabilities::new(None, "node1".to_string());

        let sig_request = Capability::Signatures {
            algorithms: vec!["any".to_string()],
        };

        assert!(caps.provides_capability(&sig_request));
    }

    #[test]
    fn test_does_not_provide_discovery_capability() {
        let caps = BearDogCapabilities::new(None, "node1".to_string());

        let discovery_request = Capability::Discovery {
            protocols: vec!["any".to_string()],
        };

        // BearDog requires discovery but doesn't provide it
        assert!(!caps.provides_capability(&discovery_request));
    }

    #[test]
    fn test_does_not_provide_storage_capability() {
        let caps = BearDogCapabilities::new(None, "node1".to_string());

        let storage_request = Capability::Storage {
            storage_types: vec!["any".to_string()],
        };

        assert!(!caps.provides_capability(&storage_request));
    }

    #[test]
    fn test_does_not_provide_compute_capability() {
        let caps = BearDogCapabilities::new(None, "node1".to_string());

        let compute_request = Capability::Compute {
            compute_types: vec!["any".to_string()],
        };

        assert!(!caps.provides_capability(&compute_request));
    }

    #[test]
    fn test_capability_serialization() {
        let caps = BearDogCapabilities::new(Some("test".to_string()), "node1".to_string());

        let json = serde_json::to_string_pretty(&caps).unwrap();
        assert!(json.contains(&expected_primal_name()));
        assert!(json.contains("encryption"));
        assert!(json.contains("trust_evaluation"));

        let deserialized: BearDogCapabilities = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.primal_id, expected_primal_name());
        assert_eq!(deserialized.family_id, Some("test".to_string()));
    }

    #[test]
    fn test_endpoint_unix_socket() {
        let caps = BearDogCapabilities::new(Some("nat0".to_string()), "node1".to_string());

        let has_unix_socket = caps
            .endpoints
            .iter()
            .any(|e| matches!(e, IpcEndpoint::UnixSocket { .. }));

        assert!(has_unix_socket);
    }

    #[test]
    fn test_endpoint_http() {
        let caps = BearDogCapabilities::new(None, "node1".to_string());

        let has_http = caps
            .endpoints
            .iter()
            .any(|e| matches!(e, IpcEndpoint::Http { .. }));

        assert!(has_http);
    }

    #[test]
    fn test_metadata_includes_version() {
        let caps = BearDogCapabilities::new(None, "node1".to_string());

        assert!(caps.metadata.contains_key("version"));
        assert!(caps.metadata.contains_key("primal_type"));
        assert_eq!(caps.metadata.get("primal_type").unwrap(), "security");
    }

    // ============================================================================
    // Unit Tests - Capability Types
    // ============================================================================

    #[test]
    fn test_capability_encryption_serialization() {
        let cap = Capability::Encryption {
            algorithms: vec!["ChaCha20Poly1305".to_string()],
            key_types: vec!["X25519".to_string()],
        };

        let json = serde_json::to_string(&cap).unwrap();
        assert!(json.contains("encryption"));
        assert!(json.contains("ChaCha20Poly1305"));

        let deserialized: Capability = serde_json::from_str(&json).unwrap();
        assert!(matches!(deserialized, Capability::Encryption { .. }));
    }

    #[test]
    fn test_capability_trust_evaluation_serialization() {
        let cap = Capability::TrustEvaluation {
            trust_models: vec!["family_based".to_string()],
        };

        let json = serde_json::to_string(&cap).unwrap();
        assert!(json.contains("trust_evaluation"));
        assert!(json.contains("family_based"));
    }

    #[test]
    fn test_capability_custom_serialization() {
        let mut properties = BTreeMap::new();
        properties.insert("key1".to_string(), "value1".to_string());

        let cap = Capability::Custom {
            name: "custom_capability".to_string(),
            version: "1.0.0".to_string(),
            properties,
        };

        let json = serde_json::to_string(&cap).unwrap();
        assert!(json.contains("custom"));
        assert!(json.contains("custom_capability"));
    }

    // ============================================================================
    // Unit Tests - Capability Request/Response
    // ============================================================================

    #[test]
    fn test_capability_request_serialization() {
        let request = CapabilityRequest {
            from_primal: "peer-beta".to_string(),
            capability: Capability::Encryption {
                algorithms: vec!["any".to_string()],
                key_types: vec!["any".to_string()],
            },
            params: BTreeMap::new(),
            request_id: "req_123".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("peer-beta"));
        assert!(json.contains("req_123"));

        let deserialized: CapabilityRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.from_primal, "peer-beta");
        assert_eq!(deserialized.request_id, "req_123");
    }

    #[test]
    fn test_capability_response_success() {
        let response = CapabilityResponse {
            request_id: "req_123".to_string(),
            status: ResponseStatus::Success,
            data: Some(serde_json::json!({"result": "encrypted"})),
            error: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("success"));
        assert!(json.contains("req_123"));
    }

    #[test]
    fn test_capability_response_error() {
        let response = CapabilityResponse {
            request_id: "req_123".to_string(),
            status: ResponseStatus::Error,
            data: None,
            error: Some("Encryption failed".to_string()),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("error"));
        assert!(json.contains("Encryption failed"));
    }

    #[test]
    fn test_capability_response_not_available() {
        let response = CapabilityResponse {
            request_id: "req_123".to_string(),
            status: ResponseStatus::NotAvailable,
            data: None,
            error: Some("Capability not provided".to_string()),
        };

        assert!(matches!(response.status, ResponseStatus::NotAvailable));
    }

    // ============================================================================
    // Unit Tests - Capability Matching
    // ============================================================================

    #[test]
    fn test_capability_matching_same_type() {
        let _provided = Capability::Encryption {
            algorithms: vec!["ChaCha20".to_string()],
            key_types: vec!["X25519".to_string()],
        };

        let requested = Capability::Encryption {
            algorithms: vec!["any".to_string()],
            key_types: vec!["any".to_string()],
        };

        let caps = BearDogCapabilities::new(None, "node1".to_string());
        // Should match because both are Encryption type
        assert!(caps.provides_capability(&requested));
    }

    #[test]
    fn test_capability_matching_different_type() {
        let caps = BearDogCapabilities::new(None, "node1".to_string());

        let storage_request = Capability::Storage {
            storage_types: vec!["any".to_string()],
        };

        // BearDog provides Encryption, not Storage
        assert!(!caps.provides_capability(&storage_request));
    }

    // ============================================================================
    // Unit Tests - Edge Cases
    // ============================================================================

    #[test]
    fn test_empty_family_id() {
        let caps = BearDogCapabilities::new(Some(String::new()), "node1".to_string());
        assert_eq!(caps.family_id, Some(String::new()));
    }

    #[test]
    fn test_very_long_node_id() {
        let long_id = "a".repeat(1000);
        let caps = BearDogCapabilities::new(None, long_id.clone());
        assert_eq!(caps.node_id, long_id);
    }

    #[test]
    fn test_unicode_in_node_id() {
        let caps = BearDogCapabilities::new(None, "node_塔1_🐕".to_string());
        assert_eq!(caps.node_id, "node_塔1_🐕");
    }

    #[test]
    fn test_capability_request_with_complex_params() {
        let mut params = BTreeMap::new();
        params.insert(
            "nested".to_string(),
            serde_json::json!({
                "level1": {
                    "level2": {
                        "value": 42
                    }
                }
            }),
        );

        let request = CapabilityRequest {
            from_primal: "test".to_string(),
            capability: Capability::Encryption {
                algorithms: vec!["any".to_string()],
                key_types: vec!["any".to_string()],
            },
            params,
            request_id: "req_123".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: CapabilityRequest = serde_json::from_str(&json).unwrap();

        assert!(deserialized.params.contains_key("nested"));
    }
}

#[cfg(test)]
mod capability_e2e_tests {
    use super::super::*;

    // ============================================================================
    // E2E Tests - Full Capability Flow
    // ============================================================================

    #[test]
    fn test_e2e_capability_advertisement() {
        // Simulate primal startup and capability advertisement
        let caps = BearDogCapabilities::new(Some("nat0".to_string()), "tower1".to_string());

        // Verify all expected capabilities are advertised
        assert!(caps.provides.len() >= 4); // Encryption, Trust, KeyMgmt, Signatures

        // Verify endpoints are configured
        assert!(caps.endpoints.len() >= 2); // Unix socket + HTTP
    }

    #[test]
    fn test_e2e_capability_discovery() {
        // Simulate another primal discovering BearDog's capabilities
        let beardog_caps = BearDogCapabilities::new(Some("nat0".to_string()), "tower1".to_string());

        // Serialize (as biomeOS would store in registry)
        let json = serde_json::to_string(&beardog_caps).unwrap();

        // Deserialize (as another primal would read from registry)
        let discovered_caps: BearDogCapabilities = serde_json::from_str(&json).unwrap();

        // Verify discovered capabilities match
        assert_eq!(discovered_caps.primal_id, super::expected_primal_name());
        assert_eq!(discovered_caps.provides.len(), beardog_caps.provides.len());
    }

    #[test]
    fn test_e2e_capability_request_encryption() {
        // Simulate Peer-beta requesting encryption from BearDog
        let beardog_caps = BearDogCapabilities::new(Some("nat0".to_string()), "tower1".to_string());

        let request = CapabilityRequest {
            from_primal: "peer-beta".to_string(),
            capability: Capability::Encryption {
                algorithms: vec!["ChaCha20Poly1305".to_string()],
                key_types: vec!["X25519".to_string()],
            },
            params: std::collections::BTreeMap::from([(
                "plaintext".to_string(),
                serde_json::json!("test data"),
            )]),
            request_id: uuid::Uuid::new_v4().to_string(),
        };

        // Verify BearDog can handle this request
        assert!(beardog_caps.provides_capability(&request.capability));
    }

    #[test]
    fn test_e2e_capability_request_trust_evaluation() {
        // Simulate peer-alpha requesting trust evaluation from BearDog
        let beardog_caps = BearDogCapabilities::new(Some("nat0".to_string()), "tower1".to_string());

        let request = CapabilityRequest {
            from_primal: "peer-alpha".to_string(),
            capability: Capability::TrustEvaluation {
                trust_models: vec!["family_based".to_string()],
            },
            params: std::collections::BTreeMap::from([(
                "peer_family_id".to_string(),
                serde_json::json!("nat0"),
            )]),
            request_id: uuid::Uuid::new_v4().to_string(),
        };

        // Verify BearDog can handle this request
        assert!(beardog_caps.provides_capability(&request.capability));
    }

    #[test]
    fn test_e2e_multiple_primals_same_capability() {
        // Simulate two encryption providers
        let beardog_caps = BearDogCapabilities::new(Some("nat0".to_string()), "tower1".to_string());

        // Hypothetical second provider
        let gorilla_caps = BearDogCapabilities {
            primal_id: "gorilla-encrypt".to_string(),
            family_id: Some("nat0".to_string()),
            node_id: "tower1".to_string(),
            provides: vec![Capability::Encryption {
                algorithms: vec!["AES-256-GCM".to_string()],
                key_types: vec!["RSA-2048".to_string()],
            }],
            requires: vec![],
            endpoints: vec![],
            metadata: std::collections::BTreeMap::new(),
        };

        let encryption_request = Capability::Encryption {
            algorithms: vec!["any".to_string()],
            key_types: vec!["any".to_string()],
        };

        // Both should be able to handle encryption
        assert!(beardog_caps.provides_capability(&encryption_request));
        assert!(gorilla_caps.provides_capability(&encryption_request));
    }
}

#[cfg(test)]
mod capability_chaos_tests {
    use super::super::*;

    // ============================================================================
    // Chaos Tests - Random Failures and Edge Cases
    // ============================================================================

    #[test]
    fn test_chaos_random_capability_types() {
        use rand::Rng;
        let mut rng = rand::rng();

        let caps = BearDogCapabilities::new(Some("nat0".to_string()), "tower1".to_string());

        // Test with random capability types
        for _ in 0..100 {
            let random_type = rng.random_range(0..7);
            let capability = match random_type {
                0 => Capability::Encryption {
                    algorithms: vec!["any".to_string()],
                    key_types: vec!["any".to_string()],
                },
                1 => Capability::TrustEvaluation {
                    trust_models: vec!["any".to_string()],
                },
                2 => Capability::KeyManagement {
                    hsm_types: vec!["any".to_string()],
                },
                3 => Capability::Signatures {
                    algorithms: vec!["any".to_string()],
                },
                4 => Capability::Discovery {
                    protocols: vec!["any".to_string()],
                },
                5 => Capability::Storage {
                    storage_types: vec!["any".to_string()],
                },
                _ => Capability::Compute {
                    compute_types: vec!["any".to_string()],
                },
            };

            // Should not panic
            let _ = caps.provides_capability(&capability);
        }
    }

    #[test]
    fn test_chaos_malformed_json() {
        let malformed_jsons = vec![
            r#"{"primal_id": "beardog""#, // Missing closing brace
            r#"{"primal_id": beardog}"#,  // Missing quotes
            r#"{primal_id: "beardog"}"#,  // Missing quotes on key
            r#"{"primal_id": null}"#,     // Null value
            r"{}",                        // Empty object
        ];

        for json in malformed_jsons {
            let result = serde_json::from_str::<BearDogCapabilities>(json);
            assert!(
                result.is_err(),
                "Should fail to parse malformed JSON: {json}"
            );
        }
    }

    #[test]
    fn test_chaos_concurrent_capability_checks() {
        use std::sync::Arc;
        use std::thread;

        let caps = Arc::new(BearDogCapabilities::new(
            Some("nat0".to_string()),
            "tower1".to_string(),
        ));

        let mut handles = vec![];

        // Spawn 10 threads checking capabilities concurrently
        for i in 0..10 {
            let caps_clone = Arc::clone(&caps);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let capability = match i % 4 {
                        0 => Capability::Encryption {
                            algorithms: vec!["any".to_string()],
                            key_types: vec!["any".to_string()],
                        },
                        1 => Capability::TrustEvaluation {
                            trust_models: vec!["any".to_string()],
                        },
                        2 => Capability::Discovery {
                            protocols: vec!["any".to_string()],
                        },
                        _ => Capability::Storage {
                            storage_types: vec!["any".to_string()],
                        },
                    };

                    // Should be thread-safe
                    let _ = caps_clone.provides_capability(&capability);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_chaos_large_capability_list() {
        // Test with a primal that provides many capabilities
        let mut provides = vec![];
        for i in 0..1000 {
            provides.push(Capability::Custom {
                name: format!("capability_{i}"),
                version: "1.0.0".to_string(),
                properties: std::collections::BTreeMap::new(),
            });
        }

        let caps = BearDogCapabilities {
            primal_id: "test_primal".to_string(),
            family_id: None,
            node_id: "node1".to_string(),
            provides,
            requires: vec![],
            endpoints: vec![],
            metadata: std::collections::BTreeMap::new(),
        };

        // Should handle large capability lists
        assert_eq!(caps.provides.len(), 1000);

        // Serialization should work
        let json = serde_json::to_string(&caps).unwrap();
        let deserialized: BearDogCapabilities = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.provides.len(), 1000);
    }
}

#[cfg(test)]
mod capability_fault_tests {
    use super::super::*;

    // ============================================================================
    // Fault Tests - Edge Cases and Error Conditions
    // ============================================================================

    #[test]
    fn test_fault_missing_required_fields() {
        let json = r#"{"primal_id": "beardog"}"#;
        let result = serde_json::from_str::<BearDogCapabilities>(json);
        assert!(result.is_err(), "Should fail without required fields");
    }

    #[test]
    fn test_fault_invalid_capability_type() {
        let json = r#"{"type": "invalid_capability"}"#;
        let result = serde_json::from_str::<Capability>(json);
        assert!(result.is_err(), "Should fail with invalid capability type");
    }

    #[test]
    fn test_fault_empty_algorithms_list() {
        let cap = Capability::Encryption {
            algorithms: vec![],
            key_types: vec![],
        };

        // Should serialize/deserialize even with empty lists
        let json = serde_json::to_string(&cap).unwrap();
        let deserialized: Capability = serde_json::from_str(&json).unwrap();

        match deserialized {
            Capability::Encryption {
                algorithms,
                key_types,
            } => {
                assert!(algorithms.is_empty());
                assert!(key_types.is_empty());
            }
            _ => panic!("Wrong capability type"),
        }
    }

    #[test]
    fn test_fault_null_family_id_in_json() {
        let json = r#"{
            "primal_id": "beardog",
            "family_id": null,
            "node_id": "node1",
            "provides": [],
            "requires": [],
            "endpoints": [],
            "metadata": {}
        }"#;

        let caps: BearDogCapabilities = serde_json::from_str(json).unwrap();
        assert_eq!(caps.family_id, None);
    }

    #[test]
    fn test_fault_special_characters_in_ids() {
        let special_chars = vec![
            "node\n1", "node\t1", "node\r1", "node\"1", "node'1", "node\\1",
        ];

        for special_id in special_chars {
            let caps = BearDogCapabilities::new(None, special_id.to_string());
            assert_eq!(caps.node_id, special_id);

            // Should serialize/deserialize correctly
            let json = serde_json::to_string(&caps).unwrap();
            let deserialized: BearDogCapabilities = serde_json::from_str(&json).unwrap();
            assert_eq!(deserialized.node_id, special_id);
        }
    }

    #[test]
    fn test_fault_very_large_json() {
        let mut large_metadata = std::collections::BTreeMap::new();
        for i in 0..10000 {
            large_metadata.insert(format!("key_{i}"), format!("value_{i}"));
        }

        let caps = BearDogCapabilities {
            primal_id: "test".to_string(),
            family_id: None,
            node_id: "node1".to_string(),
            provides: vec![],
            requires: vec![],
            endpoints: vec![],
            metadata: large_metadata,
        };

        // Should handle large JSON
        let json = serde_json::to_string(&caps).unwrap();
        assert!(json.len() > 100_000);

        let deserialized: BearDogCapabilities = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.metadata.len(), 10000);
    }

    #[test]
    fn test_fault_circular_capability_requests() {
        // Test that capability system doesn't allow circular dependencies
        let caps = BearDogCapabilities::new(None, "node1".to_string());

        // BearDog requires Discovery but doesn't provide it
        assert!(
            caps.requires
                .iter()
                .any(|c| matches!(c, Capability::Discovery { .. }))
        );
        assert!(
            !caps
                .provides
                .iter()
                .any(|c| matches!(c, Capability::Discovery { .. }))
        );
    }

    #[test]
    fn test_fault_duplicate_capabilities() {
        let caps = BearDogCapabilities {
            primal_id: "test".to_string(),
            family_id: None,
            node_id: "node1".to_string(),
            provides: vec![
                Capability::Encryption {
                    algorithms: vec!["ChaCha20".to_string()],
                    key_types: vec!["X25519".to_string()],
                },
                Capability::Encryption {
                    algorithms: vec!["AES-256".to_string()],
                    key_types: vec!["RSA".to_string()],
                },
            ],
            requires: vec![],
            endpoints: vec![],
            metadata: std::collections::BTreeMap::new(),
        };

        // Should handle duplicate capability types
        let json = serde_json::to_string(&caps).unwrap();
        let deserialized: BearDogCapabilities = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.provides.len(), 2);
    }

    #[test]
    fn test_fault_response_status_consistency() {
        // Test that response status matches error field
        let error_response = CapabilityResponse {
            request_id: "req_123".to_string(),
            status: ResponseStatus::Error,
            data: None,
            error: Some("Error message".to_string()),
        };

        assert!(matches!(error_response.status, ResponseStatus::Error));
        assert!(error_response.error.is_some());
        assert!(error_response.data.is_none());

        let success_response = CapabilityResponse {
            request_id: "req_123".to_string(),
            status: ResponseStatus::Success,
            data: Some(serde_json::json!({"result": "ok"})),
            error: None,
        };

        assert!(matches!(success_response.status, ResponseStatus::Success));
        assert!(success_response.data.is_some());
        assert!(success_response.error.is_none());
    }
}
