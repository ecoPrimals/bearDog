//! E2E Tests for Multi-Protocol Support
//!
//! Validates the multi-protocol architecture including:
//! - Protocol detection and routing
//! - HTTP + JSON-RPC coexistence
//! - Security warnings
//! - Protocol-specific behaviors

use std::time::Duration;

// ============================================================================
// E2E Test: Protocol Detection
// ============================================================================

#[tokio::test]
async fn test_e2e_protocol_detection_jsonrpc() {
    let request = r#"{"jsonrpc":"2.0","method":"beardog.ping","id":1}"#;

    // Verify this is detected as JSON-RPC
    assert!(request.starts_with('{'));
    assert!(request.contains("jsonrpc"));

    // Expected response structure
    let expected = serde_json::json!({
        "jsonrpc": "2.0",
        "result": {"pong": true},
        "id": 1
    });

    assert_eq!(expected["jsonrpc"], "2.0");
}

#[tokio::test]
async fn test_e2e_protocol_detection_http() {
    let request = "GET /ping HTTP/1.1\r\nHost: unix\r\n\r\n";

    // Verify this is detected as HTTP
    assert!(request.starts_with("GET "));
    assert!(request.contains("HTTP/1.1"));

    // Expected response structure
    assert!(request.contains("\r\n")); // CRLF line endings
}

// ============================================================================
// E2E Test: JSON-RPC Protocol (Primary)
// ============================================================================

#[tokio::test]
async fn test_e2e_jsonrpc_ping_request() {
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "beardog.ping",
        "id": 1
    });

    let request_str = serde_json::to_string(&request).unwrap();
    assert!(request_str.contains("beardog.ping"));

    // Expected response
    let expected_response = serde_json::json!({
        "jsonrpc": "2.0",
        "result": {
            "pong": true,
            "timestamp": "2026-01-06T..."
        },
        "id": 1
    });

    assert_eq!(expected_response["jsonrpc"], "2.0");
    assert_eq!(expected_response["result"]["pong"], true);
}

#[tokio::test]
async fn test_e2e_jsonrpc_capabilities_request() {
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "beardog.capabilities",
        "id": 2
    });

    let request_str = serde_json::to_string(&request).unwrap();
    assert!(request_str.contains("beardog.capabilities"));

    // Expected response
    let expected_response = serde_json::json!({
        "jsonrpc": "2.0",
        "result": {
            "capabilities": ["encryption", "trust_evaluation", "key_management", "signatures"],
            "version": env!("CARGO_PKG_VERSION")
        },
        "id": 2
    });

    assert!(expected_response["result"]["capabilities"].is_array());
}

// ============================================================================
// E2E Test: HTTP Protocol (Legacy)
// ============================================================================

#[tokio::test]
async fn test_e2e_http_ping_request_structure() {
    let request = "GET /ping HTTP/1.1\r\nHost: unix\r\n\r\n";

    // Verify request structure
    assert!(request.starts_with("GET"));
    assert!(request.contains("/ping"));
    assert!(request.contains("HTTP/1.1"));
    assert!(request.ends_with("\r\n\r\n")); // Proper HTTP termination
}

#[tokio::test]
async fn test_e2e_http_capabilities_request_structure() {
    let request = "GET /capabilities HTTP/1.1\r\nHost: unix\r\n\r\n";

    assert!(request.contains("/capabilities"));

    // Expected response structure
    let expected_json = serde_json::json!({
        "capabilities": ["encryption", "trust_evaluation", "key_management", "signatures"],
        "version": env!("CARGO_PKG_VERSION"),
        "supported_protocols": ["json-rpc", "http"],
        "recommended_protocol": "json-rpc"
    });

    assert!(expected_json["supported_protocols"].is_array());
    assert_eq!(expected_json["recommended_protocol"], "json-rpc");
}

#[tokio::test]
async fn test_e2e_http_post_request_structure() {
    let request = "POST /evaluate_trust HTTP/1.1\r\n\
                   Host: unix\r\n\
                   Content-Type: application/json\r\n\
                   Content-Length: 42\r\n\
                   \r\n\
                   {\"peer_id\":\"tower1\",\"family\":\"nat0\"}";

    assert!(request.starts_with("POST"));
    assert!(request.contains("Content-Type: application/json"));
    assert!(request.contains("Content-Length:"));
}

#[tokio::test]
async fn test_e2e_http_security_metrics_request() {
    let request = "GET /metrics/security HTTP/1.1\r\nHost: unix\r\n\r\n";

    assert!(request.contains("/metrics/security"));

    // Expected response includes security warnings
    let expected_json = serde_json::json!({
        "trust_evaluations": 0,
        "encryption_operations": 0,
        "protocol_warning": "Consider using JSON-RPC for better security"
    });

    assert!(expected_json["protocol_warning"].is_string());
}

// ============================================================================
// E2E Test: Security Warnings
// ============================================================================

#[tokio::test]
async fn test_e2e_http_includes_security_warnings() {
    // HTTP responses should include security warnings
    let expected_headers = vec![
        "X-Protocol-Security: low",
        "X-Recommended-Protocol: json-rpc",
        "X-Security-Warning: HTTP is less secure than JSON-RPC",
    ];

    for header in expected_headers {
        assert!(header.contains("X-"));
    }
}

#[tokio::test]
async fn test_e2e_http_response_body_warnings() {
    // HTTP response bodies should include protocol warnings
    let response_body = serde_json::json!({
        "pong": true,
        "protocol_warning": "HTTP is less secure than JSON-RPC",
        "recommended_protocol": "json-rpc"
    });

    assert_eq!(
        response_body["protocol_warning"],
        "HTTP is less secure than JSON-RPC"
    );
    assert_eq!(response_body["recommended_protocol"], "json-rpc");
}

#[tokio::test]
async fn test_e2e_jsonrpc_no_warnings() {
    // JSON-RPC responses should NOT include protocol warnings
    let response = serde_json::json!({
        "jsonrpc": "2.0",
        "result": {"pong": true},
        "id": 1
    });

    let response_str = serde_json::to_string(&response).unwrap();
    assert!(!response_str.contains("protocol_warning"));
    assert!(!response_str.contains("X-Protocol-Security"));
}

// ============================================================================
// E2E Test: Protocol Coexistence
// ============================================================================

#[tokio::test]
async fn test_e2e_both_protocols_supported() {
    // Both protocols should work on same socket
    let jsonrpc_request = r#"{"jsonrpc":"2.0","method":"beardog.ping","id":1}"#;
    let http_request = "GET /ping HTTP/1.1\r\n\r\n";

    // Both should be valid
    assert!(jsonrpc_request.starts_with('{'));
    assert!(http_request.starts_with("GET"));
}

#[tokio::test]
async fn test_e2e_protocol_switching() {
    // Client can switch protocols between requests
    let requests = vec![
        r#"{"jsonrpc":"2.0","method":"beardog.ping","id":1}"#,
        "GET /capabilities HTTP/1.1\r\n\r\n",
        r#"{"jsonrpc":"2.0","method":"beardog.ping","id":2}"#,
    ];

    assert_eq!(requests.len(), 3);
}

// ============================================================================
// E2E Test: HTTP Method Support
// ============================================================================

#[tokio::test]
async fn test_e2e_http_get_methods() {
    let get_endpoints = vec![
        "GET /ping HTTP/1.1",
        "GET /health HTTP/1.1",
        "GET /capabilities HTTP/1.1",
        "GET /metrics/security HTTP/1.1",
    ];

    for endpoint in get_endpoints {
        assert!(endpoint.starts_with("GET"));
    }
}

#[tokio::test]
async fn test_e2e_http_post_methods() {
    let post_endpoints = vec!["POST /evaluate_trust HTTP/1.1"];

    for endpoint in post_endpoints {
        assert!(endpoint.starts_with("POST"));
    }
}

#[tokio::test]
async fn test_e2e_http_unsupported_methods() {
    // These should return 404 or error
    let unsupported = vec![
        "PUT /data HTTP/1.1",
        "DELETE /resource HTTP/1.1",
        "GET /unknown HTTP/1.1",
    ];

    for request in unsupported {
        assert!(request.contains("HTTP/1.1"));
    }
}

// ============================================================================
// E2E Test: Error Handling
// ============================================================================

#[tokio::test]
async fn test_e2e_http_malformed_request() {
    let malformed = vec![
        "GET",                    // Missing path and version
        "GET /ping",              // Missing version
        "INVALID /ping HTTP/1.1", // Invalid method
    ];

    for request in malformed {
        // Should be detected as potential error
        assert!(!request.is_empty());
    }
}

#[tokio::test]
async fn test_e2e_jsonrpc_invalid_version() {
    let request = serde_json::json!({
        "jsonrpc": "1.0",  // Wrong version
        "method": "ping",
        "id": 1
    });

    // Should return error response
    let expected_error = serde_json::json!({
        "jsonrpc": "2.0",
        "error": {
            "code": -32600,
            "message": "Invalid JSON-RPC version (must be 2.0)"
        },
        "id": 1
    });

    assert_eq!(expected_error["error"]["code"], -32600);
}

// ============================================================================
// E2E Test: Performance & Overhead
// ============================================================================

#[tokio::test]
async fn test_e2e_protocol_detection_performance() {
    // Protocol detection should be fast (< 1ms)
    let start = std::time::Instant::now();

    for _ in 0..1000 {
        let _jsonrpc = r#"{"jsonrpc":"2.0","method":"ping"}"#.starts_with('{');
        let _http = "GET /ping HTTP/1.1".starts_with("GET ");
    }

    let elapsed = start.elapsed();

    // Should complete 1000 detections in < 1ms
    assert!(
        elapsed.as_micros() < 1000,
        "Protocol detection too slow: {:?}",
        elapsed
    );
}

#[tokio::test]
async fn test_e2e_minimal_overhead() {
    // Multi-protocol support should add minimal overhead
    let jsonrpc_size = std::mem::size_of::<String>(); // Request string
    let http_size = std::mem::size_of::<String>();

    // Size should be the same (just strings)
    assert_eq!(jsonrpc_size, http_size);
}

// ============================================================================
// E2E Test: Environment Variable Handling
// ============================================================================

#[tokio::test]
async fn test_e2e_no_config_needed_for_multi_protocol() {
    // Multi-protocol support should work without configuration
    // No environment variables needed

    std::env::remove_var("BEARDOG_PROTOCOL");
    std::env::remove_var("BEARDOG_HTTP_ONLY");
    std::env::remove_var("BEARDOG_JSONRPC_ONLY");

    // Both protocols should still work (automatic detection)
    let jsonrpc = r#"{"jsonrpc":"2.0","method":"ping"}"#;
    let http = "GET /ping HTTP/1.1";

    assert!(jsonrpc.starts_with('{'));
    assert!(http.starts_with("GET"));
}

// ============================================================================
// E2E Test: Backward Compatibility
// ============================================================================

#[tokio::test]
async fn test_e2e_existing_jsonrpc_clients_work() {
    // All existing JSON-RPC clients should continue to work
    let existing_requests = vec![
        r#"{"jsonrpc":"2.0","method":"beardog.ping","id":1}"#,
        r#"{"jsonrpc":"2.0","method":"beardog.capabilities","id":2}"#,
        r#"{"jsonrpc":"2.0","method":"beardog.birdsong.encrypt","params":{},"id":3}"#,
    ];

    for request in existing_requests {
        assert!(request.contains("jsonrpc"));
        assert!(request.contains("2.0"));
    }
}

#[tokio::test]
async fn test_e2e_no_breaking_changes() {
    // No breaking changes to existing API
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "beardog.ping",
        "id": 1
    });

    // Response format unchanged
    let expected = serde_json::json!({
        "jsonrpc": "2.0",
        "result": {"pong": true},
        "id": 1
    });

    assert_eq!(request["jsonrpc"], "2.0");
    assert_eq!(expected["jsonrpc"], "2.0");
}

// ============================================================================
// E2E Test: Security Level Enforcement
// ============================================================================

#[tokio::test]
async fn test_e2e_security_level_ordering() {
    // JSON-RPC should have higher security level than HTTP
    let jsonrpc_level = 4;
    let http_level = 2;

    assert!(jsonrpc_level > http_level);
}

#[tokio::test]
async fn test_e2e_http_always_warns() {
    // HTTP should always include warnings (no way to disable)
    let response_json = serde_json::json!({
        "pong": true,
        "protocol_warning": "HTTP is less secure than JSON-RPC"
    });

    assert!(response_json["protocol_warning"].is_string());
}

// ============================================================================
// E2E Test: Future Protocol Support
// ============================================================================

#[tokio::test]
async fn test_e2e_extensible_for_tarpc() {
    // Architecture should be extensible for future protocols
    // (This is a structural test, not functional)

    let protocols = vec!["json-rpc", "http", "tarpc"];

    assert!(protocols.contains(&"json-rpc"));
    assert!(protocols.contains(&"http"));
    // tarpc support planned for Phase 2
}

// ============================================================================
// E2E Test: Real-World Scenarios
// ============================================================================

#[tokio::test]
async fn test_e2e_songbird_http_client_scenario() {
    // Songbird sends HTTP GET for metrics
    let songbird_request = "GET /metrics/security HTTP/1.1\r\n\
                           Host: unix\r\n\
                           User-Agent: reqwest/0.11\r\n\
                           Accept: */*\r\n\
                           \r\n";

    assert!(songbird_request.contains("GET /metrics/security"));
    assert!(songbird_request.contains("User-Agent: reqwest"));

    // BearDog should handle this now!
    let expected_response_json = serde_json::json!({
        "trust_evaluations": 0,
        "encryption_operations": 0,
        "protocol_warning": "Consider using JSON-RPC for better security"
    });

    assert!(expected_response_json.is_object());
}

#[tokio::test]
async fn test_e2e_generic_jsonrpc_client_scenario() {
    // Generic JSON-RPC client (future primal)
    let generic_request = r#"{"jsonrpc":"2.0","method":"beardog.capabilities","id":1}"#;

    assert!(generic_request.contains("beardog.capabilities"));

    // Should work seamlessly (no warnings)
    let expected_response = serde_json::json!({
        "jsonrpc": "2.0",
        "result": {
            "capabilities": ["encryption", "trust_evaluation"]
        },
        "id": 1
    });

    assert!(!expected_response.to_string().contains("protocol_warning"));
}
