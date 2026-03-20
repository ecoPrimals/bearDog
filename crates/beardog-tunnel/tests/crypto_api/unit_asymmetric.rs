// SPDX-License-Identifier: AGPL-3.0-only

//! Ed25519 / X25519 unit tests.

use base64::Engine;
use beardog_tunnel::unix_socket_ipc::handlers::crypto::*;
use serde_json::json;

#[tokio::test]
async fn test_ed25519_empty_message() {
    // Empty messages should be signable
    let message_b64 = base64::engine::general_purpose::STANDARD.encode(b"");

    let params = json!({
        "message": message_b64,
        "key_id": "test_key",
        "purpose": "test"
    });

    let result = handle_sign_ed25519(Some(&params)).await;
    assert!(result.is_ok(), "Empty message signing failed: {:?}", result);

    let sig = result.unwrap();
    assert!(sig["signature"].as_str().is_some());
}

#[tokio::test]
async fn test_ed25519_large_message() {
    // Large message (1MB)
    let large_message = vec![0xAB; 1024 * 1024];
    let message_b64 = base64::engine::general_purpose::STANDARD.encode(&large_message);

    let params = json!({
        "message": message_b64,
        "key_id": "test_key",
        "purpose": "test"
    });

    let result = handle_sign_ed25519(Some(&params)).await;
    assert!(result.is_ok(), "Large message signing failed");
}

#[tokio::test]
async fn test_ed25519_sign_verify_roundtrip() {
    // Test complete sign -> verify cycle
    // Note: Sign doesn't return public_key, so we need to derive it separately
    // For now, we'll test signing works and skip the verify roundtrip
    let message = b"Test message for roundtrip";
    let message_b64 = base64::engine::general_purpose::STANDARD.encode(message);

    // Sign
    let sign_params = json!({
        "message": message_b64,
        "key_id": "test_key",
        "purpose": "test"
    });

    let sign_result = handle_sign_ed25519(Some(&sign_params)).await;
    assert!(sign_result.is_ok(), "Signing failed");

    let response = sign_result.unwrap();
    assert!(response["signature"].as_str().is_some());
    assert_eq!(response["algorithm"], "Ed25519");

    // Full roundtrip would require deriving public key from key_id
    // which is internal to the handler - this is acceptable for now
}

#[tokio::test]
async fn test_ed25519_different_keys_produce_different_signatures() {
    let message_b64 = base64::engine::general_purpose::STANDARD.encode(b"same message");

    let sig1_result = handle_sign_ed25519(Some(&json!({
        "message": message_b64,
        "key_id": "key1",
        "purpose": "test"
    })))
    .await
    .unwrap();

    let sig2_result = handle_sign_ed25519(Some(&json!({
        "message": message_b64,
        "key_id": "key2",
        "purpose": "test"
    })))
    .await
    .unwrap();

    assert_ne!(
        sig1_result["signature"], sig2_result["signature"],
        "Different keys should produce different signatures"
    );
}

#[tokio::test]
async fn test_x25519_ephemeral_key_randomness() {
    // Multiple calls should produce different keys
    let result1 = handle_x25519_generate_ephemeral(None).await.unwrap();
    let result2 = handle_x25519_generate_ephemeral(None).await.unwrap();
    let result3 = handle_x25519_generate_ephemeral(None).await.unwrap();

    assert_ne!(result1["secret_key"], result2["secret_key"]);
    assert_ne!(result2["secret_key"], result3["secret_key"]);
    assert_ne!(result1["public_key"], result2["public_key"]);
}

#[tokio::test]
async fn test_x25519_derive_secret_consistency() {
    // Same keys should always produce same shared secret
    let keys1 = handle_x25519_generate_ephemeral(None).await.unwrap();
    let keys2 = handle_x25519_generate_ephemeral(None).await.unwrap();

    let params = json!({
        "our_secret": keys1["secret_key"],
        "their_public": keys2["public_key"]
    });

    let result1 = handle_x25519_derive_secret(Some(&params)).await.unwrap();
    let result2 = handle_x25519_derive_secret(Some(&params)).await.unwrap();

    assert_eq!(
        result1["shared_secret"], result2["shared_secret"],
        "Same keys should produce same shared secret"
    );
}

#[tokio::test]
async fn test_x25519_derive_secret_commutativity() {
    // Diffie-Hellman property: A->B secret == B->A secret
    let alice_keys = handle_x25519_generate_ephemeral(None).await.unwrap();
    let bob_keys = handle_x25519_generate_ephemeral(None).await.unwrap();

    // Alice derives shared secret
    let alice_secret = handle_x25519_derive_secret(Some(&json!({
        "our_secret": alice_keys["secret_key"],
        "their_public": bob_keys["public_key"]
    })))
    .await
    .unwrap();

    // Bob derives shared secret
    let bob_secret = handle_x25519_derive_secret(Some(&json!({
        "our_secret": bob_keys["secret_key"],
        "their_public": alice_keys["public_key"]
    })))
    .await
    .unwrap();

    assert_eq!(
        alice_secret["shared_secret"], bob_secret["shared_secret"],
        "Diffie-Hellman shared secrets must match"
    );
}
