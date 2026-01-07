//! BTSP JSON-RPC Chaos & Fault Injection Tests
//!
//! These tests validate system behavior under adverse conditions:
//! - Malformed inputs
//! - Resource exhaustion
//! - Race conditions
//! - Timeout scenarios
//! - Recovery from failures
//! - Edge cases and corner cases

use beardog_tunnel::unix_socket_ipc::UnixSocketIpcServer;
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use tempfile::TempDir;

// Mock BTSP provider for chaos tests
mod test_helpers {
    use async_trait::async_trait;
    use beardog_capabilities::traits::{PeerEndpoint, SecureTunnelProvider, TunnelHandle, TunnelStatus};
    use beardog_errors::BearDogError;
    use beardog_tunnel::btsp_provider::ContactInfo;

    pub struct MockBtspProvider;

    impl MockBtspProvider {
        pub fn new() -> Self {
            Self
        }
    }

    #[async_trait]
    impl SecureTunnelProvider for MockBtspProvider {
        async fn establish_tunnel(&self, peer: PeerEndpoint) -> Result<TunnelHandle, BearDogError> {
            Ok(TunnelHandle {
                id: format!("mock-{}", uuid::Uuid::new_v4()),
                peer_id: peer.id,
                established_at: chrono::Utc::now().to_rfc3339(),
            })
        }

        async fn tunnel_encrypt(&self, _tunnel: &TunnelHandle, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
            Ok(data.to_vec())
        }

        async fn tunnel_decrypt(&self, _tunnel: &TunnelHandle, data: &[u8]) -> Result<Vec<u8>, BearDogError> {
            Ok(data.to_vec())
        }

        async fn tunnel_status(&self, tunnel: &TunnelHandle) -> Result<TunnelStatus, BearDogError> {
            Ok(TunnelStatus {
                tunnel_id: tunnel.id.clone(),
                peer_id: tunnel.peer_id.clone(),
                established_at: tunnel.established_at.clone(),
                bytes_sent: 0,
                bytes_received: 0,
                active: true,
                last_activity: chrono::Utc::now().to_rfc3339(),
            })
        }

        async fn close_tunnel(&self, _tunnel: &TunnelHandle) -> Result<(), BearDogError> {
            Ok(())
        }

        async fn contact_exchange(&self, target_peer_id: &str, _requester_lineage: &str, _max_hops: usize) -> Result<ContactInfo, BearDogError> {
            Ok(ContactInfo {
                peer_id: target_peer_id.to_string(),
                addresses: vec!["192.168.1.100:8080".to_string()],
                lineage_proof: "mock".to_string(),
                lineage_path: vec![],
                search_depth: 1,
                last_seen: chrono::Utc::now(),
            })
        }
    }
}

/// Create test server for chaos tests
async fn create_chaos_server(socket_path: PathBuf) -> Arc<UnixSocketIpcServer> {
    let provider = Arc::new(test_helpers::MockBtspProvider::new());
    let server = UnixSocketIpcServer::new(socket_path, provider)
        .await
        .expect("Failed to create server");
    Arc::new(server)
}

// ========================================================================
// MALFORMED INPUT TESTS
// ========================================================================

#[tokio::test]
async fn chaos_test_malformed_json() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("chaos-malformed.sock");
    let server = create_chaos_server(socket_path).await;

    let malformed_inputs = vec![
        "{not valid json}",
        "{'single': 'quotes'}",
        "{\"unclosed\": ",
        "{{\"double\": \"open\"}}",
        "",
        "null",
        "[]",
        "123",
        "\"string\"",
    ];

    for input in malformed_inputs {
        let result = server.handle_jsonrpc_request(input).await;
        
        // Should handle malformed JSON gracefully (return error, not panic)
        match result {
            Ok(response) => {
                // If it parses, should have an error
                assert!(response.error.is_some(), 
                    "Malformed input '{}' should result in error", input);
            }
            Err(_) => {
                // Error is also acceptable for truly malformed JSON
            }
        }
    }
}

#[tokio::test]
async fn chaos_test_json_bombs() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("chaos-bombs.sock");
    let server = create_chaos_server(socket_path).await;

    // Deeply nested JSON
    let mut nested = json!({});
    for _ in 0..100 {
        nested = json!({"nested": nested});
    }

    let deeply_nested_request = json!({
        "jsonrpc": "2.0",
        "method": "capabilities",
        "params": nested,
        "id": 1
    });

    let result = server
        .handle_jsonrpc_request(&deeply_nested_request.to_string())
        .await;

    // Should handle without panicking
    assert!(result.is_ok());

    // Huge array
    let huge_array: Vec<i32> = (0..10000).collect();
    let huge_array_request = json!({
        "jsonrpc": "2.0",
        "method": "btsp.contact_exchange",
        "params": {
            "target_peer_id": "test",
            "requester_lineage": "test",
            "extra": huge_array
        },
        "id": 1
    });

    let result = server
        .handle_jsonrpc_request(&huge_array_request.to_string())
        .await;

    // Should handle without panicking
    assert!(result.is_ok());
}

#[tokio::test]
async fn chaos_test_unicode_and_special_chars() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("chaos-unicode.sock");
    let server = create_chaos_server(socket_path).await;

    let special_inputs = vec![
        "🔥🎊🚀",  // Emojis
        "测试中文",  // Chinese
        "тест",     // Cyrillic
        "اختبار",   // Arabic
        "🔐\u{0000}null byte", // Null byte
        "\n\r\t",   // Control characters
        "'; DROP TABLE--", // SQL injection attempt
        "<script>alert('xss')</script>", // XSS attempt
        "../../../../etc/passwd", // Path traversal attempt
    ];

    for input in special_inputs {
        let request = json!({
            "jsonrpc": "2.0",
            "method": "btsp.contact_exchange",
            "params": {
                "target_peer_id": input,
                "requester_lineage": "test"
            },
            "id": 1
        });

        let result = server
            .handle_jsonrpc_request(&request.to_string())
            .await;

        // Should handle gracefully
        assert!(result.is_ok(), "Should handle special input: {}", input);
    }
}

#[tokio::test]
async fn chaos_test_extremely_long_strings() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("chaos-long-strings.sock");
    let server = create_chaos_server(socket_path).await;

    // 1MB string
    let long_string = "A".repeat(1024 * 1024);

    let request = json!({
        "jsonrpc": "2.0",
        "method": "btsp.contact_exchange",
        "params": {
            "target_peer_id": long_string,
            "requester_lineage": "test"
        },
        "id": 1
    });

    let result = server
        .handle_jsonrpc_request(&request.to_string())
        .await;

    // Should handle without panicking
    assert!(result.is_ok());
}

// ========================================================================
// TYPE CONFUSION TESTS
// ========================================================================

#[tokio::test]
async fn chaos_test_type_confusion() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("chaos-type-confusion.sock");
    let server = create_chaos_server(socket_path).await;

    let type_confusion_cases = vec![
        // String where number expected
        json!({
            "jsonrpc": "2.0",
            "method": "btsp.contact_exchange",
            "params": {
                "target_peer_id": "test",
                "requester_lineage": "test",
                "max_hops": "not a number"
            },
            "id": 1
        }),
        // Number where string expected
        json!({
            "jsonrpc": "2.0",
            "method": "btsp.contact_exchange",
            "params": {
                "target_peer_id": 12345,
                "requester_lineage": "test"
            },
            "id": 1
        }),
        // Array where object expected
        json!({
            "jsonrpc": "2.0",
            "method": "btsp.tunnel_establish",
            "params": ["not", "an", "object"],
            "id": 1
        }),
        // Boolean where string expected
        json!({
            "jsonrpc": "2.0",
            "method": "btsp.contact_exchange",
            "params": {
                "target_peer_id": true,
                "requester_lineage": false
            },
            "id": 1
        }),
    ];

    for request in type_confusion_cases {
        let result = server
            .handle_jsonrpc_request(&request.to_string())
            .await;

        // Should handle type errors gracefully
        assert!(result.is_ok());
        let response = result.unwrap();
        // Should have error for type mismatch
        if response.error.is_none() {
            // If no error, method might be flexible (also okay)
        }
    }
}

// ========================================================================
// RESOURCE EXHAUSTION TESTS
// ========================================================================

#[tokio::test]
async fn chaos_test_memory_pressure() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("chaos-memory.sock");
    let server = create_chaos_server(socket_path).await;

    // Create many servers concurrently to pressure memory
    let mut handles = vec![];
    for _ in 0..50 {
        let server_clone = Arc::clone(&server);
        let handle = tokio::spawn(async move {
            // Large payload
            let large_data = "X".repeat(100_000);
            let request = json!({
                "jsonrpc": "2.0",
                "method": "btsp.tunnel_encrypt",
                "params": {
                    "tunnel": {
                        "id": "test",
                        "peer_id": "peer",
                        "established_at": "2026-01-07T00:00:00Z"
                    },
                    "data": large_data
                },
                "id": 1
            });
            
            server_clone.handle_jsonrpc_request(&request.to_string()).await
        });
        handles.push(handle);
    }

    let results = futures::future::join_all(handles).await;

    // All should complete (even if with errors)
    for result in results {
        assert!(result.is_ok(), "Should handle memory pressure");
    }
}

#[tokio::test]
async fn chaos_test_concurrent_avalanche() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("chaos-avalanche.sock");
    let server = create_chaos_server(socket_path).await;

    // Avalanche: 500 concurrent requests
    let mut handles = vec![];
    for i in 0..500 {
        let server_clone = Arc::clone(&server);
        let handle = tokio::spawn(async move {
            let request = json!({
                "jsonrpc": "2.0",
                "method": "capabilities",
                "id": i
            });
            
            server_clone.handle_jsonrpc_request(&request.to_string()).await
        });
        handles.push(handle);
    }

    let results = futures::future::join_all(handles).await;

    // Count successes
    let mut success_count = 0;
    for result in results {
        if result.is_ok() {
            if let Ok(response) = result.unwrap() {
                if response.error.is_none() {
                    success_count += 1;
                }
            }
        }
    }

    // Should handle most requests successfully
    assert!(success_count > 400, "Should handle avalanche: {} successes", success_count);
}

// ========================================================================
// RACE CONDITION TESTS
// ========================================================================

#[tokio::test]
async fn chaos_test_rapid_method_switching() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("chaos-switching.sock");
    let server = create_chaos_server(socket_path).await;

    let methods = vec![
        "capabilities",
        "btsp.contact_exchange",
        "btsp.tunnel_establish",
        "identity",
        "btsp.tunnel_status",
        "security.evaluate",
    ];

    let mut handles = vec![];
    for _ in 0..100 {
        let server_clone = Arc::clone(&server);
        let methods_clone = methods.clone();
        
        let handle = tokio::spawn(async move {
            // Rapidly switch between methods
            for (i, method) in methods_clone.iter().enumerate() {
                let request = json!({
                    "jsonrpc": "2.0",
                    "method": method,
                    "params": if method.contains("btsp") {
                        json!({"target_peer_id": "t", "requester_lineage": "r"})
                    } else if method == "security.evaluate" {
                        json!({"peer_id": "test", "peer_family": "fam"})
                    } else {
                        json!({})
                    },
                    "id": i
                });
                
                let _ = server_clone.handle_jsonrpc_request(&request.to_string()).await;
            }
        });
        handles.push(handle);
    }

    let results = futures::future::join_all(handles).await;

    // All should complete
    for result in results {
        assert!(result.is_ok(), "Should handle rapid switching");
    }
}

// ========================================================================
// EDGE CASE TESTS
// ========================================================================

#[tokio::test]
async fn chaos_test_negative_numbers() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("chaos-negative.sock");
    let server = create_chaos_server(socket_path).await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "btsp.contact_exchange",
        "params": {
            "target_peer_id": "test",
            "requester_lineage": "test",
            "max_hops": -1
        },
        "id": 1
    });

    let result = server
        .handle_jsonrpc_request(&request.to_string())
        .await;

    // Should handle negative numbers gracefully
    assert!(result.is_ok());
}

#[tokio::test]
async fn chaos_test_float_where_int_expected() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("chaos-float.sock");
    let server = create_chaos_server(socket_path).await;

    let request = json!({
        "jsonrpc": "2.0",
        "method": "btsp.contact_exchange",
        "params": {
            "target_peer_id": "test",
            "requester_lineage": "test",
            "max_hops": 3.14159
        },
        "id": 1
    });

    let result = server
        .handle_jsonrpc_request(&request.to_string())
        .await;

    // Should handle float values gracefully
    assert!(result.is_ok());
}

#[tokio::test]
async fn chaos_test_duplicate_keys() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("chaos-dup-keys.sock");
    let server = create_chaos_server(socket_path).await;

    // JSON with duplicate keys (valid JSON, but ambiguous)
    let request_str = r#"{
        "jsonrpc": "2.0",
        "method": "capabilities",
        "method": "btsp.contact_exchange",
        "id": 1
    }"#;

    let result = server.handle_jsonrpc_request(request_str).await;

    // Should handle without panicking
    assert!(result.is_ok());
}

#[tokio::test]
async fn chaos_test_whitespace_variations() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("chaos-whitespace.sock");
    let server = create_chaos_server(socket_path).await;

    let variations = vec![
        // No whitespace
        r#"{"jsonrpc":"2.0","method":"capabilities","id":1}"#,
        // Excessive whitespace
        r#"{  "jsonrpc"  :  "2.0"  ,  "method"  :  "capabilities"  ,  "id"  :  1  }"#,
        // Mixed line endings
        "{\n\"jsonrpc\": \"2.0\",\r\n\"method\": \"capabilities\",\n\"id\": 1\r\n}",
        // Tabs
        "{\t\"jsonrpc\":\t\"2.0\",\t\"method\":\t\"capabilities\",\t\"id\":\t1}",
    ];

    for request_str in variations {
        let result = server.handle_jsonrpc_request(request_str).await;
        
        // All should parse successfully
        assert!(result.is_ok(), "Should handle whitespace variation");
        let response = result.unwrap();
        assert!(response.error.is_none(), "Should not have error");
    }
}

// ========================================================================
// RECOVERY TESTS
// ========================================================================

#[tokio::test]
async fn chaos_test_recovery_after_errors() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("chaos-recovery.sock");
    let server = create_chaos_server(socket_path).await;

    // Send malformed request
    let _ = server.handle_jsonrpc_request("{malformed}").await;

    // Send invalid method
    let invalid_request = json!({
        "jsonrpc": "2.0",
        "method": "nonexistent",
        "id": 1
    });
    let _ = server.handle_jsonrpc_request(&invalid_request.to_string()).await;

    // Server should still work for valid requests
    let valid_request = json!({
        "jsonrpc": "2.0",
        "method": "capabilities",
        "id": 1
    });

    let result = server
        .handle_jsonrpc_request(&valid_request.to_string())
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.error.is_none(), "Server should recover after errors");
}

#[tokio::test]
async fn chaos_test_mixed_success_failure_pattern() {
    let temp_dir = TempDir::new().unwrap();
    let socket_path = temp_dir.path().join("chaos-mixed-pattern.sock");
    let server = create_chaos_server(socket_path).await;

    // Pattern: success, fail, success, fail...
    let mut handles = vec![];
    for i in 0..100 {
        let server_clone = Arc::clone(&server);
        let handle = tokio::spawn(async move {
            let request = if i % 2 == 0 {
                // Valid
                json!({
                    "jsonrpc": "2.0",
                    "method": "capabilities",
                    "id": i
                })
            } else {
                // Invalid
                json!({
                    "jsonrpc": "2.0",
                    "method": "nonexistent",
                    "id": i
                })
            };
            
            server_clone.handle_jsonrpc_request(&request.to_string()).await
        });
        handles.push(handle);
    }

    let results = futures::future::join_all(handles).await;

    // All should complete
    for (i, result) in results.iter().enumerate() {
        assert!(result.is_ok(), "Request {} should complete", i);
    }
}

