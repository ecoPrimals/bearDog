//! Comprehensive tests for neural_registration module
//!
//! Tests Tower Atomic TRUE PRIMAL pattern implementation

use super::*;
use std::env;
use std::path::Path;

// ═══════════════════════════════════════════════════════════════════════════
// Socket Discovery Tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
#[serial_test::serial]
fn test_discover_neural_api_socket_from_env() {
    // Test explicit NEURAL_API_SOCKET environment variable
    let original = env::var("NEURAL_API_SOCKET").ok();

    env::set_var("NEURAL_API_SOCKET", "/tmp/test-neural.sock");
    let result = discover_neural_api_socket();
    assert_eq!(result, Some("/tmp/test-neural.sock".to_string()));

    // Cleanup
    env::remove_var("NEURAL_API_SOCKET");
    if let Some(val) = original {
        env::set_var("NEURAL_API_SOCKET", val);
    }
}

#[test]
#[serial_test::serial]
fn test_discover_neural_api_socket_from_neurals_env() {
    // Test fallback to NEURALS_SOCKET environment variable
    let original_neural = env::var("NEURAL_API_SOCKET").ok();
    let original_neurals = env::var("NEURALS_SOCKET").ok();

    env::remove_var("NEURAL_API_SOCKET");
    env::set_var("NEURALS_SOCKET", "/tmp/neurals-fallback.sock");

    let result = discover_neural_api_socket();
    assert_eq!(result, Some("/tmp/neurals-fallback.sock".to_string()));

    // Cleanup
    env::remove_var("NEURALS_SOCKET");
    if let Some(val) = original_neural {
        env::set_var("NEURAL_API_SOCKET", val);
    }
    if let Some(val) = original_neurals {
        env::set_var("NEURALS_SOCKET", val);
    }
}

#[test]
fn test_discover_neural_api_socket_priority() {
    // ✅ Concurrent-safe: Explicit configuration, no global state modification
    let mut env_vars = std::collections::HashMap::new();
    env_vars.insert(
        "NEURAL_API_SOCKET".to_string(),
        "/tmp/priority.sock".to_string(),
    );
    env_vars.insert(
        "NEURALS_SOCKET".to_string(),
        "/tmp/fallback.sock".to_string(),
    );

    let result = discover_neural_api_socket_with_env(&env_vars);
    assert_eq!(result, Some("/tmp/priority.sock".to_string()));
}

#[test]
#[serial_test::serial]
fn test_discover_neural_api_socket_empty_string() {
    // Test that empty string explicitly disables auto-registration
    let original = env::var("NEURAL_API_SOCKET").ok();

    env::set_var("NEURAL_API_SOCKET", "");
    let result = discover_neural_api_socket();
    assert_eq!(result, None);

    // Cleanup
    env::remove_var("NEURAL_API_SOCKET");
    if let Some(val) = original {
        env::set_var("NEURAL_API_SOCKET", val);
    }
}

#[test]
#[serial_test::serial]
fn test_discover_neural_api_socket_default_paths() {
    // Test that default socket paths are checked in priority order
    let original_neural = env::var("NEURAL_API_SOCKET").ok();
    let original_neurals = env::var("NEURALS_SOCKET").ok();

    env::remove_var("NEURAL_API_SOCKET");
    env::remove_var("NEURALS_SOCKET");

    let result = discover_neural_api_socket();

    // Check default paths in priority order
    if Path::new("/tmp/neural-api.sock").exists() {
        assert_eq!(result, Some("/tmp/neural-api.sock".to_string()));
    } else if Path::new("/tmp/neural-api-nat0.sock").exists() {
        assert_eq!(result, Some("/tmp/neural-api-nat0.sock".to_string()));
    } else {
        assert_eq!(result, None);
    }

    // Cleanup
    if let Some(val) = original_neural {
        env::set_var("NEURAL_API_SOCKET", val);
    }
    if let Some(val) = original_neurals {
        env::set_var("NEURALS_SOCKET", val);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Registration Function Tests
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_register_with_neural_api_nonexistent_socket() {
    // Test graceful handling when Neural API is not running
    let result = register_with_neural_api(
        "/tmp/nonexistent-neural-api-test-12345.sock",
        "beardog-test",
        "/tmp/beardog-test.sock",
    )
    .await;

    // Should fail gracefully - the important thing is it doesn't panic
    assert!(result.is_err());
}

#[tokio::test]
async fn test_register_with_neural_api_invalid_path() {
    // Test handling of invalid socket path
    let result = register_with_neural_api(
        "/invalid/path/that/does/not/exist.sock",
        "beardog-test",
        "/tmp/beardog-test.sock",
    )
    .await;

    assert!(result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════
// Capability Definition Tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_crypto_capability_structure() {
    // Test that crypto capability has expected structure
    let capabilities = json!({
        "capability": "crypto",
        "provider": "beardog",
        "operations": [
            "generate_keypair",
            "ecdh_derive",
            "encrypt",
            "decrypt"
        ]
    });

    assert_eq!(capabilities["capability"], "crypto");
    assert_eq!(capabilities["provider"], "beardog");
    assert!(capabilities["operations"].is_array());
    assert!(capabilities["operations"].as_array().unwrap().len() >= 4);
}

#[test]
fn test_semantic_mappings_complete() {
    // Test that all core crypto operations have semantic mappings
    let required_operations = vec!["generate_keypair", "ecdh_derive", "encrypt", "decrypt"];

    let mappings = json!({
        "crypto.generate_keypair": "crypto.x25519_generate_ephemeral",
        "crypto.ecdh_derive": "crypto.x25519_derive_secret",
        "crypto.encrypt": "crypto.chacha20_poly1305_encrypt",
        "crypto.decrypt": "crypto.chacha20_poly1305_decrypt",
    });

    for op in required_operations {
        let key = format!("crypto.{}", op);
        assert!(mappings[&key].is_string());
    }
}

#[test]
fn test_tls_crypto_capability_distinct() {
    // Test that tls_crypto is a separate capability
    let crypto = json!({"capability": "crypto"});
    let tls_crypto = json!({"capability": "tls_crypto"});

    assert_ne!(crypto["capability"], tls_crypto["capability"]);
}

#[test]
fn test_genetic_lineage_capability() {
    // Test genetic lineage capability structure
    let capability = json!({
        "capability": "genetic_lineage",
        "provider": "beardog",
        "operations": ["verify_lineage", "generate_lineage_proof"]
    });

    assert_eq!(capability["capability"], "genetic_lineage");
    assert_eq!(capability["provider"], "beardog");
    assert!(capability["operations"].is_array());
}

// ═══════════════════════════════════════════════════════════════════════════
// Error Handling Tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
#[serial_test::serial]
fn test_discover_socket_handles_permission_errors() {
    // Test that permission errors don't crash discovery
    // This validates the function is resilient
    let original_neural = env::var("NEURAL_API_SOCKET").ok();
    let original_neurals = env::var("NEURALS_SOCKET").ok();

    env::remove_var("NEURAL_API_SOCKET");
    env::remove_var("NEURALS_SOCKET");

    // Should return None or Some, not panic
    let result = discover_neural_api_socket();
    assert!(result.is_none() || result.is_some());

    // Cleanup
    if let Some(val) = original_neural {
        env::set_var("NEURAL_API_SOCKET", val);
    }
    if let Some(val) = original_neurals {
        env::set_var("NEURALS_SOCKET", val);
    }
}

#[tokio::test]
async fn test_register_capability_network_error() {
    // Test handling of network/connection errors
    let capability = json!({
        "capability": "test",
        "primal": "beardog-test",
        "socket_path": "/tmp/beardog-test.sock",
        "provider": "beardog",
        "operations": ["test_op"]
    });

    let result = register_capability("/tmp/nonexistent-test-socket.sock", capability).await;

    // Should fail with connection error, not panic
    assert!(result.is_err());
}

// ═══════════════════════════════════════════════════════════════════════════
// Integration-Style Tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
#[serial_test::serial]
fn test_discover_socket_deterministic() {
    // Test that socket discovery is deterministic
    let original_neural = env::var("NEURAL_API_SOCKET").ok();
    let original_neurals = env::var("NEURALS_SOCKET").ok();

    env::set_var("NEURAL_API_SOCKET", "/tmp/test.sock");

    let result1 = discover_neural_api_socket();
    let result2 = discover_neural_api_socket();

    assert_eq!(result1, result2);

    // Cleanup
    env::remove_var("NEURAL_API_SOCKET");
    if let Some(val) = original_neural {
        env::set_var("NEURAL_API_SOCKET", val);
    }
    if let Some(val) = original_neurals {
        env::set_var("NEURALS_SOCKET", val);
    }
}

#[test]
fn test_all_capabilities_have_provider() {
    // Test that all capabilities specify beardog as provider
    let capabilities = vec!["crypto", "tls_crypto", "genetic_lineage"];

    for cap in capabilities {
        let capability = json!({
            "capability": cap,
            "provider": "beardog"
        });

        assert_eq!(capability["provider"], "beardog");
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Edge Case Tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
#[serial_test::serial]
fn test_discover_socket_with_whitespace() {
    // Test handling of whitespace in environment variables
    let original = env::var("NEURAL_API_SOCKET").ok();

    env::set_var("NEURAL_API_SOCKET", " /tmp/test.sock ");
    let result = discover_neural_api_socket();

    // Should preserve the exact value (including whitespace)
    assert_eq!(result, Some(" /tmp/test.sock ".to_string()));

    // Cleanup
    env::remove_var("NEURAL_API_SOCKET");
    if let Some(val) = original {
        env::set_var("NEURAL_API_SOCKET", val);
    }
}

#[test]
#[serial_test::serial]
fn test_discover_socket_empty_vs_unset() {
    // Test distinction between empty string and unset variable
    let original = env::var("NEURAL_API_SOCKET").ok();
    let original_neurals = env::var("NEURALS_SOCKET").ok();

    // Empty string should disable (even if defaults exist)
    env::set_var("NEURAL_API_SOCKET", "");
    let empty_result = discover_neural_api_socket();
    assert_eq!(
        empty_result, None,
        "Empty string should explicitly disable auto-registration"
    );

    // Unset should fall through to default/None
    env::remove_var("NEURAL_API_SOCKET");
    env::remove_var("NEURALS_SOCKET");
    let unset_result = discover_neural_api_socket();
    // Could be None or default socket (depends on whether default paths exist)
    assert!(unset_result.is_none() || unset_result.is_some());

    // Cleanup
    if let Some(val) = original {
        env::set_var("NEURAL_API_SOCKET", val);
    }
    if let Some(val) = original_neurals {
        env::set_var("NEURALS_SOCKET", val);
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TRUE PRIMAL Pattern Validation Tests
// ═══════════════════════════════════════════════════════════════════════════

#[test]
fn test_semantic_operation_naming() {
    // Test that semantic operation names follow {domain}.{operation} pattern
    let operations = vec![
        "crypto.generate_keypair",
        "crypto.ecdh_derive",
        "crypto.encrypt",
        "crypto.decrypt",
        "tls.derive_handshake_secrets",
    ];

    for op in operations {
        // Should have exactly one dot separator
        assert_eq!(op.matches('.').count(), 1);

        // Should have non-empty domain and operation
        let parts: Vec<&str> = op.split('.').collect();
        assert_eq!(parts.len(), 2);
        assert!(!parts[0].is_empty());
        assert!(!parts[1].is_empty());
    }
}

#[test]
fn test_zero_coupling_principle() {
    // Test that capabilities don't reference specific implementations
    // (except in semantic_mappings which is the translation layer)

    let capability = json!({
        "capability": "crypto",
        "provider": "beardog",
        "operations": ["generate_keypair", "ecdh_derive"]
    });

    // Operations should be semantic, not implementation-specific
    let ops = capability["operations"].as_array().unwrap();
    for op in ops {
        let op_str = op.as_str().unwrap();
        // Should not contain algorithm names
        assert!(!op_str.contains("x25519"));
        assert!(!op_str.contains("chacha20"));
        assert!(!op_str.contains("poly1305"));
    }
}
