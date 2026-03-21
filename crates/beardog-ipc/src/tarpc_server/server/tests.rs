// SPDX-License-Identifier: AGPL-3.0-only

use super::BearDogCryptoServer;
use crate::tarpc_types::BearDogCrypto;
use crate::tarpc_types::*;

#[test]
fn test_server_creation() {
    let server = BearDogCryptoServer::new();
    assert!(!server.version.is_empty());
}

#[test]
fn test_server_default_impl() {
    let server = BearDogCryptoServer::default();
    assert!(!server.version.is_empty());
}

#[test]
fn test_default_primal_name() {
    let server = BearDogCryptoServer::new();
    assert_eq!(server.primal_name, env!("CARGO_PKG_NAME"));
}

#[tokio::test]
async fn crypto_handlers_happy_paths() {
    let server = BearDogCryptoServer::new();
    let cx = tarpc::context::current();

    let kp = server.clone().generate_ed25519(cx).await.unwrap();
    assert_eq!(kp.public_key.len(), 32);
    assert_eq!(kp.private_key.len(), 32);

    let x = server.clone().generate_x25519_ephemeral(cx).await.unwrap();
    assert_eq!(x.public_key.len(), 32);

    let p256 = server.clone().generate_ecdh_p256(cx).await.unwrap();
    assert!(!p256.private_key.is_empty());

    let p384 = server.clone().generate_ecdh_p384(cx).await.unwrap();
    assert!(!p384.private_key.is_empty());

    let sign = server
        .clone()
        .sign_ed25519(
            cx,
            SignRequest {
                data: b"msg".to_vec(),
                private_key: kp.private_key.clone(),
            },
        )
        .await
        .unwrap();
    assert_eq!(sign.signature.len(), 64);

    let ok = server
        .clone()
        .verify_ed25519(
            cx,
            VerifyRequest {
                data: b"msg".to_vec(),
                signature: sign.signature.clone(),
                public_key: kp.public_key.clone(),
            },
        )
        .await
        .unwrap();
    assert!(ok);

    let h = server.clone().blake3_hash(cx, b"x".to_vec()).await.unwrap();
    assert_eq!(h.hash.len(), 32);
    let s = server.clone().sha256_hash(cx, b"x".to_vec()).await.unwrap();
    assert_eq!(s.hash.len(), 32);
    let mac = server
        .clone()
        .hmac_sha256(
            cx,
            HmacRequest {
                data: b"d".to_vec(),
                key: b"secret".to_vec(),
            },
        )
        .await
        .unwrap();
    assert_eq!(mac.hash.len(), 32);

    let tls_req = TlsSecretsRequest {
        shared_secret: vec![7u8; 32],
        transcript_hash: vec![3u8; 32],
        cipher_suite: "test".to_string(),
    };
    let hs = server
        .clone()
        .tls_derive_handshake_secrets(cx, tls_req.clone())
        .await
        .unwrap();
    assert_eq!(hs.client_traffic_secret.len(), 32);
    let app = server
        .clone()
        .tls_derive_application_secrets(cx, tls_req)
        .await
        .unwrap();
    assert_eq!(app.server_traffic_secret.len(), 32);

    let lineage = server
        .clone()
        .genetic_derive_lineage_key(
            cx,
            LineageRequest {
                family_seed: vec![9u8; 32],
                generation: 2,
                context: "ctx".to_string(),
            },
        )
        .await
        .unwrap();
    assert_eq!(lineage.key.len(), 32);

    let mix = server
        .clone()
        .genetic_mix_entropy(
            cx,
            EntropyMixRequest {
                sources: vec![vec![1], vec![2]],
                context: "mix".to_string(),
            },
        )
        .await
        .unwrap();
    assert_eq!(mix.entropy.len(), 32);

    let methods = server.clone().rpc_methods(cx).await;
    assert!(!methods.is_empty());
    let caps = server.clone().primal_capabilities(cx).await;
    assert!(!caps.is_empty());
    let health = server.clone().health(cx).await;
    assert_eq!(health.status, "healthy");
}

#[tokio::test]
async fn crypto_handlers_error_branches() {
    let server = BearDogCryptoServer::new();
    let cx = tarpc::context::current();

    let err = server
        .clone()
        .sign_ed25519(
            cx,
            SignRequest {
                data: vec![],
                private_key: vec![0u8; 31],
            },
        )
        .await
        .unwrap_err();
    assert!(err.message.contains("Ed25519 private key"));

    let err = server
        .clone()
        .verify_ed25519(
            cx,
            VerifyRequest {
                data: vec![],
                signature: vec![0u8; 64],
                public_key: vec![0u8; 31],
            },
        )
        .await
        .unwrap_err();
    assert!(err.message.contains("public key length"));

    let err = server
        .clone()
        .chacha20_poly1305_encrypt(
            cx,
            EncryptRequest {
                plaintext: vec![0u8; 8],
                key: vec![0u8; 16],
                nonce: None,
                aad: None,
            },
        )
        .await
        .unwrap_err();
    assert!(err.message.contains("key length"));

    let err = server
        .clone()
        .chacha20_poly1305_decrypt(
            cx,
            DecryptRequest {
                ciphertext: vec![],
                key: vec![0u8; 32],
                nonce: vec![0u8; 11],
                tag: vec![0u8; 16],
                aad: None,
            },
        )
        .await
        .unwrap_err();
    assert!(err.message.contains("nonce length"));
}

#[tokio::test]
async fn primal_info_reads_family_env() {
    let server = BearDogCryptoServer::with_identity("x", "test-family");
    let info = server.primal_info(tarpc::context::current()).await;
    assert_eq!(info.family, "test-family");
}
