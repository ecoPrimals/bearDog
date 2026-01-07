//! Tests for BTSP JSON-RPC methods on Unix socket
//!
//! This test module validates that all 6 BTSP endpoints are accessible
//! via JSON-RPC on the Unix socket, enabling 100% port-free P2P federation.

#[cfg(test)]
mod tests {
    use crate::btsp_provider::BeardogBtspProvider;
    use crate::tunnel::hsm::manager::HsmManager;
    use crate::unix_socket_ipc::UnixSocketIpcServer;
    use beardog_genetics::birdsong::BirdSongManager;
    use beardog_genetics::ecosystem_evolution::EcosystemGeneticEngine;
    use serde_json::json;
    use std::path::PathBuf;
    use std::sync::Arc;
    use tempfile::TempDir;

    /// Create test BTSP provider
    async fn create_test_btsp_provider() -> Arc<BeardogBtspProvider> {
        let hsm = Arc::new(HsmManager::new());
        let genetics = Arc::new(EcosystemGeneticEngine::new().expect("Failed to create genetics engine"));
        
        let provider = BeardogBtspProvider::new(
            hsm,
            genetics,
        ).await.expect("Failed to create BTSP provider");
        
        Arc::new(provider)
    }

    /// Create test server
    async fn create_test_server(socket_path: PathBuf) -> Arc<UnixSocketIpcServer> {
        let provider = create_test_btsp_provider().await;
        let server = UnixSocketIpcServer::new(socket_path, provider).await
            .expect("Failed to create Unix socket IPC server");
        Arc::new(server)
    }

    #[tokio::test]
    async fn test_btsp_capabilities_advertised() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let server = create_test_server(socket_path.clone()).await;

        // Test JSON-RPC capabilities method
        let request = json!({
            "jsonrpc": "2.0",
            "method": "capabilities",
            "id": 1
        });

        let response = server.handle_jsonrpc_request(&request.to_string()).await.unwrap();
        assert!(response.error.is_none(), "Should not have error");
        
        let result = response.result.unwrap();
        let capabilities = result["provided_capabilities"].as_array().unwrap();
        
        // Find BTSP capability
        let btsp_cap = capabilities.iter()
            .find(|c| c["type"] == "btsp")
            .expect("BTSP capability should be advertised");
        
        assert_eq!(btsp_cap["version"], "1.0");
        assert!(result["btsp_enabled"].as_bool().unwrap());
        
        let methods = btsp_cap["methods"].as_array().unwrap();
        assert!(methods.contains(&json!("contact_exchange")));
        assert!(methods.contains(&json!("tunnel_establish")));
        assert!(methods.contains(&json!("tunnel_encrypt")));
        assert!(methods.contains(&json!("tunnel_decrypt")));
        assert!(methods.contains(&json!("tunnel_status")));
        assert!(methods.contains(&json!("tunnel_close")));
    }

    #[tokio::test]
    async fn test_btsp_contact_exchange_jsonrpc() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let server = create_test_server(socket_path.clone()).await;

        // Test with beardog namespace
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

        let response = server.handle_jsonrpc_request(&request.to_string()).await.unwrap();
        
        // May fail if no lineage configured, but method should be recognized
        if let Some(error) = response.error {
            // Error should be about lineage/peer not found, not "method not found"
            assert!(!error.message.contains("Method not found"));
        }
    }

    #[tokio::test]
    async fn test_btsp_contact_exchange_btsp_namespace() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let server = create_test_server(socket_path.clone()).await;

        // Test with btsp namespace
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

        let response = server.handle_jsonrpc_request(&request.to_string()).await.unwrap();
        
        // Method should be recognized (not "method not found")
        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
        }
    }

    #[tokio::test]
    async fn test_btsp_contact_exchange_slash_namespace() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let server = create_test_server(socket_path.clone()).await;

        // Test with btsp/contact/exchange namespace
        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.contact/exchange",
            "params": {
                "peer_id": "tower2",
                "lineage": "tower1"
            },
            "id": 1
        });

        let response = server.handle_jsonrpc_request(&request.to_string()).await.unwrap();
        
        // Method should be recognized
        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
        }
    }

    #[tokio::test]
    async fn test_btsp_tunnel_establish_jsonrpc() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let server = create_test_server(socket_path.clone()).await;

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

        let response = server.handle_jsonrpc_request(&request.to_string()).await.unwrap();
        
        // Method should be recognized
        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
        }
    }

    #[tokio::test]
    async fn test_btsp_tunnel_encrypt_jsonrpc() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let server = create_test_server(socket_path.clone()).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_encrypt",
            "params": {
                "tunnel": {
                    "id": "tunnel-123",
                    "peer_id": "tower2"
                },
                "data": "SGVsbG8sIFdvcmxkIQ==" // "Hello, World!" in base64
            },
            "id": 1
        });

        let response = server.handle_jsonrpc_request(&request.to_string()).await.unwrap();
        
        // Method should be recognized
        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
        }
    }

    #[tokio::test]
    async fn test_btsp_tunnel_decrypt_jsonrpc() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let server = create_test_server(socket_path.clone()).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_decrypt",
            "params": {
                "tunnel": {
                    "id": "tunnel-123",
                    "peer_id": "tower2"
                },
                "data": "c29tZS1jaXBoZXJ0ZXh0" // base64
            },
            "id": 1
        });

        let response = server.handle_jsonrpc_request(&request.to_string()).await.unwrap();
        
        // Method should be recognized
        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
        }
    }

    #[tokio::test]
    async fn test_btsp_tunnel_status_jsonrpc() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let server = create_test_server(socket_path.clone()).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_status",
            "params": {
                "tunnel_id": "tunnel-123"
            },
            "id": 1
        });

        let response = server.handle_jsonrpc_request(&request.to_string()).await.unwrap();
        
        // Method should be recognized
        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
        }
    }

    #[tokio::test]
    async fn test_btsp_tunnel_status_with_id_param() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let server = create_test_server(socket_path.clone()).await;

        // Test alternative parameter name 'id' instead of 'tunnel_id'
        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel/status",
            "params": {
                "id": "tunnel-456"
            },
            "id": 1
        });

        let response = server.handle_jsonrpc_request(&request.to_string()).await.unwrap();
        
        // Method should be recognized
        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
        }
    }

    #[tokio::test]
    async fn test_btsp_tunnel_close_jsonrpc() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let server = create_test_server(socket_path.clone()).await;

        let request = json!({
            "jsonrpc": "2.0",
            "method": "beardog./btsp/tunnel/close",
            "params": {
                "tunnel_id": "tunnel-123"
            },
            "id": 1
        });

        let response = server.handle_jsonrpc_request(&request.to_string()).await.unwrap();
        
        // Method should be recognized
        if let Some(error) = response.error {
            assert!(!error.message.contains("Method not found"));
        }
    }

    #[tokio::test]
    async fn test_btsp_all_namespaces_recognized() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let server = create_test_server(socket_path.clone()).await;

        let test_cases = vec![
            ("beardog./btsp/contact/exchange", json!({"target_peer_id": "test", "requester_lineage": "test"})),
            ("btsp.contact_exchange", json!({"target_peer_id": "test", "requester_lineage": "test"})),
            ("btsp.contact/exchange", json!({"target_peer_id": "test", "requester_lineage": "test"})),
            ("beardog./btsp/tunnel/establish", json!({"id": "test", "address": "127.0.0.1:8080"})),
            ("btsp.tunnel_establish", json!({"id": "test", "address": "127.0.0.1:8080"})),
            ("btsp.tunnel/establish", json!({"id": "test", "address": "127.0.0.1:8080"})),
        ];

        for (method, params) in test_cases {
            let request = json!({
                "jsonrpc": "2.0",
                "method": method,
                "params": params,
                "id": 1
            });

            let response = server.handle_jsonrpc_request(&request.to_string()).await.unwrap();
            
            // All should be recognized (no "Method not found" error)
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
    async fn test_btsp_contact_exchange_missing_params() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let server = create_test_server(socket_path.clone()).await;

        // Missing required params
        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.contact_exchange",
            "params": {},
            "id": 1
        });

        let response = server.handle_jsonrpc_request(&request.to_string()).await.unwrap();
        
        // Should have error about missing params, not "Method not found"
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert!(!error.message.contains("Method not found"));
        assert!(error.message.contains("Missing") || error.message.contains("target_peer_id") || error.message.contains("requester_lineage"));
    }

    #[tokio::test]
    async fn test_btsp_methods_in_error_message() {
        let temp_dir = TempDir::new().unwrap();
        let socket_path = temp_dir.path().join("test.sock");
        let server = create_test_server(socket_path.clone()).await;

        // Call unknown method
        let request = json!({
            "jsonrpc": "2.0",
            "method": "unknown.method",
            "id": 1
        });

        let response = server.handle_jsonrpc_request(&request.to_string()).await.unwrap();
        
        // Error message should mention BTSP methods as available
        assert!(response.error.is_some());
        let error = response.error.unwrap();
        assert!(error.message.contains("Method not found"));
        assert!(
            error.message.contains("btsp.contact_exchange") || 
            error.message.contains("btsp.tunnel_establish"),
            "Error message should list BTSP methods as available"
        );
    }
}

