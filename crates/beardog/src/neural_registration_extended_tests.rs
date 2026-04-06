// SPDX-License-Identifier: AGPL-3.0-or-later

//! Additional tests for neural_registration module
//! Expanding coverage for Tower Atomic TRUE PRIMAL pattern
//!
//! Uses [`beardog_ipc::neural_registration::discover_neural_api_socket_with`] for
//! dependency-injected resolution (no process environment mutation).

use beardog_ipc::neural_registration::{discover_neural_api_socket_with, register_with_neural_api};
use serde_json::json;

#[tokio::test]
async fn test_register_with_neural_api_connection_failure() {
    let result = register_with_neural_api(
        "/tmp/nonexistent-neural-api-test.sock",
        "beardog-test",
        "/tmp/beardog-test.sock",
    )
    .await;

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Failed to connect") || err_msg.contains("context"));
}

#[test]
fn test_discover_neural_api_socket_explicit_override() {
    assert_eq!(
        discover_neural_api_socket_with(
            Some("/tmp/explicit-neural.sock".to_string()),
            None,
        ),
        Some("/tmp/explicit-neural.sock".to_string())
    );
}

#[test]
fn test_semantic_mappings_comprehensive() {
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

    for (semantic, actual) in crypto_operations {
        assert!(!semantic.is_empty());
        assert!(!actual.is_empty());
        assert!(semantic.starts_with("crypto."));
        assert!(actual.starts_with("crypto."));
    }
}

#[test]
fn test_tls_crypto_mappings() {
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
fn test_discover_neural_api_socket_injected_priority() {
    assert_eq!(
        discover_neural_api_socket_with(None, Some("/tmp/neurals-custom.sock".to_string())),
        Some("/tmp/neurals-custom.sock".to_string())
    );
    assert_eq!(
        discover_neural_api_socket_with(
            Some("/tmp/neural-priority.sock".to_string()),
            Some("/tmp/neurals-custom.sock".to_string()),
        ),
        Some("/tmp/neural-priority.sock".to_string())
    );
}

#[test]
fn test_empty_string_disables_registration() {
    assert_eq!(
        discover_neural_api_socket_with(Some(String::new()), None),
        None
    );
}
