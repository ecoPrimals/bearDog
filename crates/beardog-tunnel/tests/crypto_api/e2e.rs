// SPDX-License-Identifier: AGPL-3.0-only

//! End-to-end handler flow (JSON-RPC style).

use base64::Engine;
use serde_json::json;

#[tokio::test]
async fn test_e2e_sign_ed25519() {
    use beardog_tunnel::unix_socket_ipc::handlers::crypto::*;

    let message_b64 = base64::engine::general_purpose::STANDARD.encode(b"test message");

    let params = json!({
        "message": message_b64,
        "key_id": "test_key",
        "purpose": "test"
    });

    let result = handle_sign_ed25519(Some(&params)).await;

    assert!(result.is_ok(), "E2E sign failed: {:?}", result);
    let response = result.unwrap();
    assert!(response["signature"].as_str().is_some());
    assert_eq!(response["algorithm"], "Ed25519");
}

#[tokio::test]
async fn test_e2e_verify_ed25519() {
    use beardog_tunnel::unix_socket_ipc::handlers::crypto::*;

    // For verify test, we need to provide a known public key
    // Since sign doesn't return it, we'll test verify with independent key generation
    use beardog_core::crypto_service::algorithms::asymmetric;

    let message = b"test message";
    let message_b64 = base64::engine::general_purpose::STANDARD.encode(message);

    // Generate a test keypair
    let seed = [0x42u8; 32];
    let (secret_key, public_key) = asymmetric::generate_ed25519_from_seed(&seed).unwrap();

    // Sign
    let signature = asymmetric::sign_ed25519(message, &secret_key).unwrap();

    // Verify via handler
    let verify_params = json!({
        "message": message_b64,
        "signature": base64::engine::general_purpose::STANDARD.encode(&signature),
        "public_key": base64::engine::general_purpose::STANDARD.encode(public_key)
    });

    let verify_result = handle_verify_ed25519(Some(&verify_params)).await;

    assert!(verify_result.is_ok(), "E2E verify failed");
    assert_eq!(verify_result.unwrap()["valid"], true);
}

#[tokio::test]
async fn test_e2e_x25519_generate() {
    use beardog_tunnel::unix_socket_ipc::handlers::crypto::*;

    let result = handle_x25519_generate_ephemeral(None).await;

    assert!(result.is_ok(), "E2E X25519 generate failed");
    let response = result.unwrap();
    assert!(response["secret_key"].as_str().is_some());
    assert!(response["public_key"].as_str().is_some());
}

#[tokio::test]
async fn test_e2e_x25519_derive_secret() {
    use beardog_tunnel::unix_socket_ipc::handlers::crypto::*;

    // Generate two keypairs
    let keys1 = handle_x25519_generate_ephemeral(None).await.unwrap();
    let keys2 = handle_x25519_generate_ephemeral(None).await.unwrap();

    // Derive shared secret
    let params = json!({
        "our_secret": keys1["secret_key"],
        "their_public": keys2["public_key"]
    });

    let result = handle_x25519_derive_secret(Some(&params)).await;

    assert!(result.is_ok(), "E2E X25519 derive failed");
    assert!(result.unwrap()["shared_secret"].as_str().is_some());
}

#[tokio::test]
async fn test_e2e_chacha20_encrypt_decrypt() {
    use beardog_tunnel::unix_socket_ipc::handlers::crypto::*;

    let plaintext_b64 = base64::engine::general_purpose::STANDARD.encode(b"secret message");
    let key = base64::engine::general_purpose::STANDARD.encode(vec![0x42; 32]);

    // Encrypt
    let encrypt_params = json!({
        "plaintext": plaintext_b64,
        "key": key
    });

    let encrypt_result = handle_chacha20_poly1305_encrypt(Some(&encrypt_params)).await;

    assert!(encrypt_result.is_ok(), "E2E encrypt failed");
    let encrypt_response = encrypt_result.unwrap();
    let ciphertext = encrypt_response["ciphertext"].as_str().unwrap().to_string();
    let nonce = encrypt_response["nonce"].as_str().unwrap().to_string();
    let tag = encrypt_response["tag"].as_str().unwrap().to_string();

    // Decrypt
    let decrypt_params = json!({
        "ciphertext": ciphertext,
        "key": key,
        "nonce": nonce,
        "tag": tag
    });

    let decrypt_result = handle_chacha20_poly1305_decrypt(Some(&decrypt_params)).await;

    assert!(decrypt_result.is_ok(), "E2E decrypt failed");
    assert_eq!(decrypt_result.unwrap()["plaintext"], plaintext_b64);
}

#[tokio::test]
async fn test_e2e_blake3_hash() {
    use beardog_tunnel::unix_socket_ipc::handlers::crypto::*;

    let data_b64 = base64::engine::general_purpose::STANDARD.encode(b"data to hash");
    let params = json!({"data": data_b64});

    let result = handle_blake3_hash(Some(&params)).await;

    assert!(result.is_ok(), "E2E blake3 failed");
    let response = result.unwrap();
    let hash = response["hash"].as_str().unwrap();
    assert_eq!(
        base64::engine::general_purpose::STANDARD
            .decode(hash)
            .unwrap()
            .len(),
        32
    );
}

#[tokio::test]
async fn test_e2e_hmac_sha256() {
    use beardog_tunnel::unix_socket_ipc::handlers::crypto::*;

    let key_b64 = base64::engine::general_purpose::STANDARD.encode(b"secret");
    let data_b64 = base64::engine::general_purpose::STANDARD.encode(b"message");

    let params = json!({
        "key": key_b64,
        "data": data_b64
    });

    let result = handle_hmac_sha256(Some(&params)).await;

    assert!(result.is_ok(), "E2E HMAC failed");
    let response = result.unwrap();
    let mac = response["mac"].as_str().unwrap();
    assert_eq!(
        base64::engine::general_purpose::STANDARD
            .decode(mac)
            .unwrap()
            .len(),
        32
    );
}

#[tokio::test]
async fn test_e2e_full_tls_handshake_simulation() {
    // Simulate a simplified TLS handshake flow using all crypto operations
    use beardog_tunnel::unix_socket_ipc::handlers::crypto::*;

    // 1. Server generates ephemeral ECDH keypair
    let server_keys = handle_x25519_generate_ephemeral(None).await.unwrap();

    // 2. Client generates ephemeral ECDH keypair
    let client_keys = handle_x25519_generate_ephemeral(None).await.unwrap();

    // 3. Derive shared secret (both sides)
    let shared_secret = handle_x25519_derive_secret(Some(&json!({
        "our_secret": client_keys["secret_key"],
        "their_public": server_keys["public_key"]
    })))
    .await
    .unwrap();

    // 4. Hash the handshake transcript
    let transcript = "ClientHello+ServerHello";
    let transcript_hash = handle_blake3_hash(Some(&json!({
        "data": base64::engine::general_purpose::STANDARD.encode(transcript)
    })))
    .await
    .unwrap();

    // 5. Generate HMAC for verify_data
    let verify_data = handle_hmac_sha256(Some(&json!({
        "key": shared_secret["shared_secret"],
        "data": transcript_hash["hash"]
    })))
    .await
    .unwrap();

    // 6. Sign certificate with Ed25519
    let cert_signature = handle_sign_ed25519(Some(&json!({
        "message": base64::engine::general_purpose::STANDARD.encode(b"server certificate"),
        "key_id": "server_cert_key",
        "purpose": "tls"
    })))
    .await
    .unwrap();

    // 7. Encrypt application data
    let app_data = handle_chacha20_poly1305_encrypt(Some(&json!({
        "plaintext": base64::engine::general_purpose::STANDARD.encode(b"Hello, client!"),
        "key": shared_secret["shared_secret"]
    })))
    .await
    .unwrap();

    // All operations succeeded
    assert!(shared_secret["shared_secret"].as_str().is_some());
    assert!(transcript_hash["hash"].as_str().is_some());
    assert!(verify_data["mac"].as_str().is_some());
    assert!(cert_signature["signature"].as_str().is_some());
    assert!(app_data["ciphertext"].as_str().is_some());
}
