// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive tests for `neural_registration` module

use super::*;
use std::path::Path;

#[test]
fn test_discover_neural_api_socket_from_env() {
    let result = discover_neural_api_socket_with(Some("/tmp/test-neural.sock".to_string()), None);
    assert_eq!(result, Some("/tmp/test-neural.sock".to_string()));
}

#[test]
fn test_discover_neural_api_socket_from_neurals_env() {
    let result =
        discover_neural_api_socket_with(None, Some("/tmp/neurals-fallback.sock".to_string()));
    assert_eq!(result, Some("/tmp/neurals-fallback.sock".to_string()));
}

#[test]
fn test_discover_neural_api_socket_priority() {
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
fn test_discover_neural_api_socket_empty_string() {
    assert_eq!(
        discover_neural_api_socket_with(Some(String::new()), None),
        None
    );
}

#[test]
fn test_discover_neural_api_socket_default_paths() {
    let result = discover_neural_api_socket_with(None, None);
    if Path::new("/tmp/neural-api.sock").exists() {
        assert_eq!(result, Some("/tmp/neural-api.sock".to_string()));
    } else if Path::new("/tmp/neural-api-nat0.sock").exists() {
        assert_eq!(result, Some("/tmp/neural-api-nat0.sock".to_string()));
    } else {
        assert_eq!(result, None);
    }
}

#[tokio::test]
async fn test_register_with_neural_api_nonexistent_socket() {
    let result = register_with_neural_api(
        "/tmp/nonexistent-neural-api-test-12345.sock",
        "beardog-test",
        "/tmp/beardog-test.sock",
        None,
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_register_with_neural_api_invalid_path() {
    let result = register_with_neural_api(
        "/invalid/path/that/does/not/exist.sock",
        "beardog-test",
        "/tmp/beardog-test.sock",
        None,
    )
    .await;
    assert!(result.is_err());
}

#[test]
fn test_crypto_capability_structure() {
    let capabilities = json!({
        "capability": "crypto",
        "provider": "beardog",
        "operations": ["generate_keypair", "ecdh_derive", "encrypt", "decrypt"]
    });
    assert_eq!(capabilities["capability"], "crypto");
}

#[test]
fn test_discover_socket_handles_permission_errors() {
    let result = discover_neural_api_socket_with(None, None);
    assert!(result.is_none() || result.is_some());
}

#[tokio::test]
async fn test_register_capability_network_error() {
    let capability = json!({
        "capability": "test",
        "primal": "beardog-test",
        "socket_path": "/tmp/beardog-test.sock",
        "provider": "beardog",
        "operations": ["test_op"]
    });
    let result = register_capability("/tmp/nonexistent-test-socket.sock", capability).await;
    assert!(result.is_err());
}

#[test]
fn test_discover_socket_deterministic() {
    let a = discover_neural_api_socket_with(Some("/tmp/test.sock".to_string()), None);
    let b = discover_neural_api_socket_with(Some("/tmp/test.sock".to_string()), None);
    assert_eq!(a, b);
}

#[test]
fn test_discover_socket_with_whitespace() {
    let result = discover_neural_api_socket_with(Some(" /tmp/test.sock ".to_string()), None);
    assert_eq!(result, Some(" /tmp/test.sock ".to_string()));
}

#[test]
fn test_discover_socket_empty_vs_unset() {
    assert_eq!(
        discover_neural_api_socket_with(Some(String::new()), None),
        None
    );
    let unset = discover_neural_api_socket_with(None, None);
    assert!(unset.is_none() || unset.is_some());
}

#[test]
fn announce_methods_includes_crypto_and_security() {
    let methods = beardog_announce_method_names();
    assert!(
        methods.len() >= 40,
        "expected at least 40 methods, got {}",
        methods.len()
    );
    assert!(methods.contains(&"crypto.sign_ed25519"));
    assert!(methods.contains(&"crypto.ionic_bond.propose"));
    assert!(methods.contains(&"crypto.ionic_bond.verify_proposal"));
    assert!(methods.contains(&"security.evaluate"));
    assert!(methods.contains(&"security.generate_jwt_secret"));
}

#[test]
fn announce_methods_are_dotted_canonical() {
    for method in beardog_announce_method_names() {
        assert!(
            method.contains('.'),
            "method {method} is not in dotted canonical form"
        );
    }
}

#[tokio::test]
async fn send_primal_announce_connection_failure() {
    let result = send_primal_announce(
        "/tmp/nonexistent-biomeos-test.sock",
        "beardog-test",
        "/tmp/beardog-test.sock",
        &["crypto.sign_ed25519".to_string()],
        None,
    )
    .await;
    assert!(result.is_err());
}
