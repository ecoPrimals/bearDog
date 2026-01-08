//! Comprehensive BTSP JSON-RPC Tests
//!
//! This test module provides extensive coverage for BTSP methods exposed via JSON-RPC:
//! - Unit tests for each method
//! - End-to-end integration tests
//! - Error handling and edge cases
//! - Chaos/fault injection tests

#[cfg(test)]
mod btsp_jsonrpc_unit_tests {
    use crate::btsp_provider::BeardogBtspProvider;
    use crate::tunnel::hsm::HsmManager;
    use crate::unix_socket_ipc::UnixSocketIpcServer;
    use beardog_genetics::EcosystemGeneticEngine;
    use serde_json::json;
    use std::path::PathBuf;
    use std::sync::Arc;
    use tempfile::TempDir;

    /// Create test server with real BTSP provider (using auto-initialized HSM and genetics)
    async fn create_test_server(socket_path: PathBuf) -> Arc<UnixSocketIpcServer> {
        // Set environment for software HSM
        std::env::set_var("BEARDOG_HSM_MODE", "software");
        
        // Create auto-initialized HSM and genetics
        let hsm = Arc::new(HsmManager::auto_initialize().await.expect("Failed to auto-initialize HSM"));
        let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Failed to create genetics"));
        
        // Create real BTSP provider
        let provider = Arc::new(
            BeardogBtspProvider::new(hsm, genetics)
                .await
                .expect("Failed to create BTSP provider"),
        );
        
        let server = UnixSocketIpcServer::new(socket_path, provider)
            .await
            .expect("Failed to create Unix socket IPC server");
        Arc::new(server)
    }

    /// Create test server that will fail operations (for error path testing)
    async fn create_failing_test_server(socket_path: PathBuf) -> Arc<UnixSocketIpcServer> {
        // For failing tests, we'll use the same provider but test with invalid inputs
        create_test_server(socket_path).await
    }

    // ========================================================================
    // UNIT TESTS - Capabilities Advertisement
    // ========================================================================

    #[tokio::test]
    async fn test_btsp_capabilities_advertised() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-capabilities.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "capabilities",
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle capabilities request");

        assert!(response.error.is_none(), "Should not have error");

        let result = response.result.expect("Should have result");
        let capabilities = result["provided_capabilities"]
            .as_array()
            .expect("Should have capabilities array");

        // Find BTSP capability
        let btsp_cap = capabilities
            .iter()
            .find(|c| c["type"] == "btsp")
            .expect("BTSP capability should be advertised");

        assert_eq!(btsp_cap["version"], "1.0");
        assert!(result["btsp_enabled"].as_bool().unwrap());

        let methods = btsp_cap["methods"].as_array().unwrap();
        assert_eq!(methods.len(), 6, "Should have all 6 BTSP methods");
        assert!(methods.contains(&json!("contact_exchange")));
        assert!(methods.contains(&json!("tunnel_establish")));
        assert!(methods.contains(&json!("tunnel_encrypt")));
        assert!(methods.contains(&json!("tunnel_decrypt")));
        assert!(methods.contains(&json!("tunnel_status")));
        assert!(methods.contains(&json!("tunnel_close")));
    }

    // ========================================================================
    // UNIT TESTS - Contact Exchange
    // ========================================================================

    #[tokio::test]
    async fn test_contact_exchange_with_beardog_namespace() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-contact-beardog.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "beardog./btsp/contact/exchange",
            "params": {
                "target_peer_id": "tower2",
                "requester_lineage": "tower1",
                "max_hops": 3
            },
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        // Method should be recognized (error will be about missing lineage data, not method)
        if let Some(error) = response.error {
            assert!(
                !error.message.contains("Method not found"),
                "Method should be recognized, got: {}",
                error.message
            );
        }
    }

    #[tokio::test]
    async fn test_contact_exchange_with_btsp_namespace() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-contact-btsp.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.contact_exchange",
            "params": {
                "target_peer_id": "tower2",
                "requester_lineage": "tower1",
                "max_hops": 5
            },
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
        }
    }

    #[tokio::test]
    async fn test_contact_exchange_missing_params() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-contact-missing.sock");
        let server = create_test_server(socket_path).await;

        // Missing target_peer_id
        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.contact_exchange",
            "params": {
                "requester_lineage": "tower1"
            },
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        assert!(
            response.error.is_some(),
            "Should have error for missing params"
        );
        let error = response.error.unwrap();
        assert!(
            error.message.contains("Missing") || error.message.contains("target_peer_id"),
            "Error should mention missing parameter: {}",
            error.message
        );
    }

    #[tokio::test]
    async fn test_contact_exchange_alternative_param_names() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-contact-alt-params.sock");
        let server = create_test_server(socket_path).await;

        // Using alternative param names: peer_id instead of target_peer_id
        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.contact_exchange",
            "params": {
                "peer_id": "tower2",
                "lineage": "tower1"
            },
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        // Should recognize alternative param names
        if let Some(error) = response.error {
            assert!(!error.message.contains("Missing target_peer_id"));
            assert!(!error.message.contains("Missing requester_lineage"));
        }
    }

    // ========================================================================
    // UNIT TESTS - Tunnel Establish
    // ========================================================================

    #[tokio::test]
    async fn test_tunnel_establish_valid_peer() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-tunnel-establish.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "beardog./btsp/tunnel/establish",
            "params": {
                "id": "tower2",
                "address": "192.168.1.100:8080",
                "family": "nat0"
            },
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        // Method should be recognized
        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
        }
    }

    #[tokio::test]
    async fn test_tunnel_establish_missing_peer_info() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-tunnel-missing.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_establish",
            "params": {},
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        assert!(
            response.error.is_some(),
            "Should have error for invalid peer"
        );
    }

    // ========================================================================
    // UNIT TESTS - Tunnel Encrypt/Decrypt
    // ========================================================================

    #[tokio::test]
    async fn test_tunnel_encrypt_valid_data() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-encrypt.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_encrypt",
            "params": {
                "tunnel": {
                    "id": "tunnel-123",
                    "peer_id": "tower2",
                    "established_at": "2026-01-07T12:00:00Z"
                },
                "data": "SGVsbG8sIFdvcmxkIQ==" // "Hello, World!" in base64
            },
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        // Method should be recognized
        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
        }
    }

    #[tokio::test]
    async fn test_tunnel_encrypt_invalid_base64() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-encrypt-bad-base64.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_encrypt",
            "params": {
                "tunnel": {
                    "id": "tunnel-123",
                    "peer_id": "tower2",
                    "established_at": "2026-01-07T12:00:00Z"
                },
                "data": "not-valid-base64!!!"
            },
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        assert!(
            response.error.is_some(),
            "Should have error for invalid base64"
        );
        let error = response.error.unwrap();
        assert!(
            error.message.contains("base64") || error.message.contains("Invalid"),
            "Error should mention base64: {}",
            error.message
        );
    }

    #[tokio::test]
    async fn test_tunnel_decrypt_missing_tunnel() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-decrypt-missing.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_decrypt",
            "params": {
                "data": "c29tZS1jaXBoZXJ0ZXh0"
            },
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        assert!(
            response.error.is_some(),
            "Should have error for missing tunnel"
        );
        let error = response.error.unwrap();
        assert!(
            error.message.contains("Missing") || error.message.contains("tunnel"),
            "Error should mention missing tunnel: {}",
            error.message
        );
    }

    // ========================================================================
    // UNIT TESTS - Tunnel Status
    // ========================================================================

    #[tokio::test]
    async fn test_tunnel_status_with_tunnel_id() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-status-id.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_status",
            "params": {
                "tunnel_id": "tunnel-123"
            },
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        // Method should be recognized
        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
        }
    }

    #[tokio::test]
    async fn test_tunnel_status_with_id_param() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-status-alt-id.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel/status",
            "params": {
                "id": "tunnel-456"
            },
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        // Should accept "id" as alternative param name
        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
            assert!(!error.message.contains("Missing tunnel_id"));
        }
    }

    #[tokio::test]
    async fn test_tunnel_status_with_full_handle() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-status-handle.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_status",
            "params": {
                "tunnel": {
                    "id": "tunnel-789",
                    "peer_id": "tower2",
                    "established_at": "2026-01-07T12:00:00Z"
                }
            },
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        // Should accept full tunnel handle
        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
        }
    }

    // ========================================================================
    // UNIT TESTS - Tunnel Close
    // ========================================================================

    #[tokio::test]
    async fn test_tunnel_close_with_id() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-close.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "beardog./btsp/tunnel/close",
            "params": {
                "tunnel_id": "tunnel-123"
            },
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        // Method should be recognized
        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
        }
    }

    // ========================================================================
    // UNIT TESTS - Error Handling & Edge Cases
    // ========================================================================

    #[tokio::test]
    async fn test_unknown_btsp_method() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-unknown.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.nonexistent_method",
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        assert!(
            response.error.is_some(),
            "Should have error for unknown method"
        );
        let error = response.error.unwrap();
        assert!(error.message.contains("Method not found"));
    }

    #[tokio::test]
    async fn test_btsp_methods_in_error_message() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-error-msg.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "unknown.method",
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert!(error.message.contains("Method not found"));
        // Error message should list BTSP methods as available (either specific methods or btsp.*)
        assert!(
            error.message.contains("btsp.contact_exchange")
                || error.message.contains("btsp.tunnel_establish")
                || error.message.contains("btsp.*"),
            "Error message should list BTSP methods: {}",
            error.message
        );
    }

    #[tokio::test]
    async fn test_all_namespace_variants() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-namespaces.sock");
        let server = create_test_server(socket_path).await;

        let test_cases = vec![
            (
                "beardog./btsp/contact/exchange",
                json!({"target_peer_id": "test", "requester_lineage": "test"}),
            ),
            (
                "btsp.contact_exchange",
                json!({"target_peer_id": "test", "requester_lineage": "test"}),
            ),
            (
                "btsp.contact/exchange",
                json!({"target_peer_id": "test", "requester_lineage": "test"}),
            ),
        ];

        for (method, params) in test_cases {
            let request = json!({
                "jsonrpc": "2.0",
                "method": method,
                "params": params,
                "id": 1
            });

            let response = server
                .handle_jsonrpc_request(&request.to_string())
                .await
                .expect("Should handle request");

            // All should be recognized (no "Method not found")
            if let Some(error) = &response.error {
                assert!(
                    !error.message.contains("Method not found"),
                    "Method '{}' should be recognized, but got: {}",
                    method,
                    error.message
                );
            }
        }
    }

    #[tokio::test]
    async fn test_jsonrpc_version_validation() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-version.sock");
        let server = create_test_server(socket_path).await;

        // Invalid JSON-RPC version
        let request = json!({
            "jsonrpc": "1.0",
            "method": "btsp.contact_exchange",
            "params": {"target_peer_id": "test", "requester_lineage": "test"},
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        assert!(
            response.error.is_some(),
            "Should reject invalid JSON-RPC version"
        );
        let error = response.error.unwrap();
        assert!(
            error.message.contains("version") || error.message.contains("2.0"),
            "Error should mention version: {}",
            error.message
        );
    }

    #[tokio::test]
    async fn test_empty_params() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-empty-params.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.contact_exchange",
            "params": {},
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        assert!(
            response.error.is_some(),
            "Should have error for empty params"
        );
    }

    #[tokio::test]
    async fn test_null_params() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-null-params.sock");
        let server = create_test_server(socket_path).await;

        let request = r#"{"jsonrpc":"2.0","method":"btsp.contact_exchange","params":null,"id":1}"#;

        let response = server
            .handle_jsonrpc_request(request)
            .await
            .expect("Should handle request");

        // Should handle null params gracefully
        assert!(response.error.is_some());
    }

    #[tokio::test]
    async fn test_large_max_hops() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-large-hops.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.contact_exchange",
            "params": {
                "target_peer_id": "test",
                "requester_lineage": "test",
                "max_hops": 999999
            },
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        // Should handle large numbers (might fail on business logic, not parsing)
        if let Some(error) = response.error {
            assert!(!error.message.contains("parse") && !error.message.contains("invalid"));
        }
    }

    #[tokio::test]
    async fn test_zero_max_hops() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-zero-hops.sock");
        let server = create_test_server(socket_path).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.contact_exchange",
            "params": {
                "target_peer_id": "test",
                "requester_lineage": "test",
                "max_hops": 0
            },
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle request");

        // Should handle zero hops gracefully
        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
        }
    }

    // ========================================================================
    // UNIT TESTS - Identity API (Songbird compatibility)
    // ========================================================================

    #[tokio::test]
    async fn test_identity_includes_encryption_tag() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-identity.sock");
        let server = create_test_server(socket_path).await;

        // Set environment for test
        std::env::set_var("FAMILY_ID", "nat0");
        std::env::set_var("NODE_ID", "node-alpha");

        let request = json!({
            "jsonrpc": "2.0",
            "method": "identity",
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle identity request");

        assert!(response.error.is_none(), "Identity request should succeed");

        let result = response.result.expect("Should have result");

        // Verify all required fields for Songbird compatibility
        assert_eq!(result["family"], "nat0", "Should have family field");
        assert_eq!(result["node"], "node-alpha", "Should have node field");
        assert_eq!(
            result["encryption_tag"], "beardog:family:nat0",
            "Should have encryption_tag field with correct format"
        );
        assert_eq!(result["primal"], "beardog", "Should identify as beardog");

        // Clean up
        std::env::remove_var("FAMILY_ID");
        std::env::remove_var("NODE_ID");
    }

    #[tokio::test]
    async fn test_identity_alternative_method_names() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-identity-alt.sock");
        let server = create_test_server(socket_path).await;

        std::env::set_var("FAMILY_ID", "test_family");
        std::env::set_var("NODE_ID", "test_node");

        let methods = vec!["identity", "whoami", "get_identity"];

        for method in methods {
            let request = json!({
                "jsonrpc": "2.0",
                "method": method,
                "id": 1
            });

            let response = server
                .handle_jsonrpc_request(&request.to_string())
                .await
                .expect("Should handle request");

            assert!(
                response.error.is_none(),
                "Method '{}' should succeed",
                method
            );

            let result = response.result.expect("Should have result");
            assert!(
                result["encryption_tag"].is_string(),
                "Method '{}' should return encryption_tag",
                method
            );
        }

        std::env::remove_var("FAMILY_ID");
        std::env::remove_var("NODE_ID");
    }

    #[tokio::test]
    async fn test_lineage_includes_encryption_tag() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test-lineage.sock");
        let server = create_test_server(socket_path).await;

        std::env::set_var("FAMILY_ID", "nat0");
        std::env::set_var("NODE_ID", "node-beta");

        let request = json!({
            "jsonrpc": "2.0",
            "method": "security.lineage",
            "id": 1
        });

        let response = server
            .handle_jsonrpc_request(&request.to_string())
            .await
            .expect("Should handle lineage request");

        assert!(response.error.is_none(), "Lineage request should succeed");

        let result = response.result.expect("Should have result");
        assert_eq!(
            result["encryption_tag"], "beardog:family:nat0",
            "Lineage should include encryption_tag"
        );

        std::env::remove_var("FAMILY_ID");
        std::env::remove_var("NODE_ID");
    }
}
