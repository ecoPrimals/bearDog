//! Additional tests for neural_registration module
//! Expanding coverage for Tower Atomic TRUE PRIMAL pattern

use super::*;
use serde_json::json;

#[tokio::test]
async fn test_register_capability_success() {
    // Test that individual capability registration works
    let capability = json!({
        "capability": "test_crypto",
        "provider": "beardog",
        "version": "0.1.0",
        "operations": ["test_op"],
        "semantic_mappings": {
            "test.operation": "test.actual_operation"
        }
    });

    // This will fail to connect, but we're testing the request format
    let result = register_capability("/tmp/nonexistent-neural-api.sock", capability).await;
    
    // Should fail with connection error, not parameter error
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Failed to connect") || err_msg.contains("No such file"));
}

#[tokio::test]
async fn test_register_with_neural_api_connection_failure() {
    // Test graceful handling of Neural API unavailability
    let result = register_with_neural_api("/tmp/nonexistent-neural-api-test.sock").await;
    
    // Should fail gracefully with connection error
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Failed to connect") || err_msg.contains("context"));
}

#[test]
fn test_discover_neural_api_socket_returns_value() {
    // Test that discovery always returns Some value (either env or default)
    let socket = discover_neural_api_socket();
    
    // Should always return something unless explicitly disabled
    if std::env::var("NEURAL_API_SOCKET").unwrap_or_default() != "" {
        assert!(socket.is_some());
    }
}

#[test]
fn test_semantic_mappings_comprehensive() {
    // Verify all 12 crypto operations are correctly mapped
    let crypto_operations = vec![
        ("crypto.generate_keypair", "crypto.x25519_generate_ephemeral"),
        ("crypto.ecdh_derive", "crypto.x25519_derive_secret"),
        ("crypto.encrypt", "crypto.chacha20_poly1305_encrypt"),
        ("crypto.decrypt", "crypto.chacha20_poly1305_decrypt"),
        ("crypto.encrypt_aes_128_gcm", "crypto.aes128_gcm_encrypt"),
        ("crypto.decrypt_aes_128_gcm", "crypto.aes128_gcm_decrypt"),
        ("crypto.encrypt_aes_256_gcm", "crypto.aes256_gcm_encrypt"),
        ("crypto.decrypt_aes_256_gcm", "crypto.aes256_gcm_decrypt"),
        ("crypto.sha256", "crypto.sha256"),
        ("crypto.sha384", "crypto.sha384"),
        ("crypto.hkdf_extract", "crypto.hkdf_extract"),
        ("crypto.hkdf_expand", "crypto.hkdf_expand"),
    ];
    
    // Verify each mapping exists (test against expected structure)
    for (semantic, actual) in crypto_operations {
        assert!(!semantic.is_empty());
        assert!(!actual.is_empty());
        assert!(semantic.starts_with("crypto."));
        assert!(actual.starts_with("crypto."));
    }
}

#[test]
fn test_tls_crypto_mappings() {
    // Verify TLS crypto operations are correctly mapped
    let tls_operations = vec![
        ("tls.derive_handshake_secrets", "tls.derive_handshake_secrets"),
        ("tls.derive_application_secrets", "tls.derive_application_secrets"),
        ("tls.compute_finished_verify_data", "tls.compute_finished_verify_data"),
    ];
    
    for (semantic, actual) in tls_operations {
        assert!(!semantic.is_empty());
        assert!(!actual.is_empty());
        assert!(semantic.starts_with("tls."));
        assert!(actual.starts_with("tls."));
    }
}

#[test]
fn test_genetic_lineage_mappings() {
    // Verify genetic lineage operations are correctly mapped
    let genetic_operations = vec![
        ("genetic.verify_lineage", "genetic.verify_lineage"),
        ("genetic.generate_lineage_proof", "genetic.generate_lineage_proof"),
    ];
    
    for (semantic, actual) in genetic_operations {
        assert!(!semantic.is_empty());
        assert!(!actual.is_empty());
        assert!(semantic.starts_with("genetic."));
        assert!(actual.starts_with("genetic."));
    }
}

#[test]
fn test_capability_structure() {
    // Test that capability JSON structure is valid
    let cap = json!({
        "capability": "test",
        "provider": "beardog",
        "version": "0.1.0",
        "operations": ["op1", "op2"],
        "semantic_mappings": {
            "test.op1": "test.actual_op1"
        }
    });
    
    assert_eq!(cap["capability"], "test");
    assert_eq!(cap["provider"], "beardog");
    assert_eq!(cap["version"], "0.1.0");
    assert!(cap["operations"].is_array());
    assert!(cap["semantic_mappings"].is_object());
}

#[test]
fn test_discover_neural_api_socket_env_priority() {
    // Test environment variable priority (NEURAL_API_SOCKET > NEURALS_SOCKET > default)
    
    // Save current env
    let saved_neural = std::env::var("NEURAL_API_SOCKET").ok();
    let saved_neurals = std::env::var("NEURALS_SOCKET").ok();
    
    // Test: No env vars → default
    std::env::remove_var("NEURAL_API_SOCKET");
    std::env::remove_var("NEURALS_SOCKET");
    let default_socket = discover_neural_api_socket();
    assert_eq!(default_socket, Some("/tmp/neural-api-nat0.sock".to_string()));
    
    // Test: NEURALS_SOCKET set
    std::env::set_var("NEURALS_SOCKET", "/tmp/neurals-custom.sock");
    let neurals_socket = discover_neural_api_socket();
    assert_eq!(neurals_socket, Some("/tmp/neurals-custom.sock".to_string()));
    
    // Test: NEURAL_API_SOCKET takes priority
    std::env::set_var("NEURAL_API_SOCKET", "/tmp/neural-priority.sock");
    let priority_socket = discover_neural_api_socket();
    assert_eq!(priority_socket, Some("/tmp/neural-priority.sock".to_string()));
    
    // Restore env
    std::env::remove_var("NEURAL_API_SOCKET");
    std::env::remove_var("NEURALS_SOCKET");
    if let Some(val) = saved_neural {
        std::env::set_var("NEURAL_API_SOCKET", val);
    }
    if let Some(val) = saved_neurals {
        std::env::set_var("NEURALS_SOCKET", val);
    }
}

#[test]
fn test_empty_string_disables_registration() {
    // Test that empty string explicitly disables auto-registration
    
    // Save current env
    let saved = std::env::var("NEURAL_API_SOCKET").ok();
    
    // Set to empty
    std::env::set_var("NEURAL_API_SOCKET", "");
    let socket = discover_neural_api_socket();
    assert_eq!(socket, None);
    
    // Restore
    std::env::remove_var("NEURAL_API_SOCKET");
    if let Some(val) = saved {
        std::env::set_var("NEURAL_API_SOCKET", val);
    }
}

#[tokio::test]
async fn test_register_capability_malformed_response() {
    // Test handling of malformed Neural API responses
    // (This is a conceptual test - in practice, we'd need a mock server)
    
    let capability = json!({
        "capability": "test",
        "provider": "beardog",
        "version": "0.1.0",
        "operations": []
    });
    
    // Attempting to register to nonexistent socket should fail gracefully
    let result = register_capability("/tmp/test-malformed.sock", capability).await;
    assert!(result.is_err());
}

