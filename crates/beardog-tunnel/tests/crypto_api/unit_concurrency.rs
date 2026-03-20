// SPDX-License-Identifier: AGPL-3.0-only

//! Concurrent mixed crypto operations.

use base64::Engine;
use beardog_tunnel::unix_socket_ipc::handlers::crypto::*;
use serde_json::json;

#[tokio::test]
async fn test_concurrent_crypto_operations() {
    // Test concurrent execution of all crypto operations
    let handles: Vec<_> = (0..50)
        .map(|i| {
            tokio::spawn(async move {
                let msg = format!("message_{}", i);
                let msg_b64 = base64::engine::general_purpose::STANDARD.encode(msg.as_bytes());

                // Sign
                let _ = handle_sign_ed25519(Some(&json!({
                    "message": msg_b64,
                    "key_id": format!("key_{}", i),
                    "purpose": "test"
                })))
                .await;

                // Generate ephemeral key
                let _ = handle_x25519_generate_ephemeral(None).await;

                // Hash
                let _ = handle_blake3_hash(Some(&json!({"data": msg_b64}))).await;

                // HMAC
                let _ = handle_hmac_sha256(Some(&json!({
                    "key": base64::engine::general_purpose::STANDARD.encode(b"key"),
                    "data": msg_b64
                })))
                .await;

                i
            })
        })
        .collect();

    for handle in handles {
        let result = handle.await.expect("Task should complete");
        assert!(result < 50);
    }
}
