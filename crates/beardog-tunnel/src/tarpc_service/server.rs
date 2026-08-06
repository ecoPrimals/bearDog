// SPDX-License-Identifier: AGPL-3.0-or-later

//! Server-side implementation of [`BearDogRpc`].
//!
//! All methods delegate to `beardog-crypto` or `ionic_token` — no logic
//! duplication with JSON-RPC handlers. The `identity` field carries the
//! primal's `PrimalIdentity` for key derivation and DID generation.

use std::sync::Arc;

use beardog_crypto::{hash_blake3, sign_ed25519, verify_ed25519};
use beardog_types::primal_identity::PrimalIdentity;
use tracing::debug;

use super::types::*;
use crate::ionic_token::{issue_ionic_token_with_gate, GateIdentity};
use crate::unix_socket_ipc::handlers::primal_signing::derive_primal_signing_key;

/// Server-side implementation of `BearDogRpc`.
#[derive(Clone)]
pub struct BearDogRpcServer {
    identity: Arc<PrimalIdentity>,
}

impl BearDogRpcServer {
    /// Creates a new tarpc service backed by the given primal identity.
    #[must_use]
    pub fn new(identity: Arc<PrimalIdentity>) -> Self {
        Self { identity }
    }

    fn derive_ed25519_keypair(
        &self,
        key_id: &str,
    ) -> Result<(ed25519_dalek::SigningKey, ed25519_dalek::VerifyingKey), String> {
        let family_seed = beardog_errors::process_env::var(
            beardog_config::env_keys::ENV_FAMILY_SEED_PREFIXED,
        )
        .or_else(|_| beardog_errors::process_env::var(beardog_config::env_keys::ENV_FAMILY_SEED))
        .map_err(|_| "FAMILY_SEED not set — cannot derive signing key".to_string())?;

        let derived = blake3::derive_key(
            &format!("beardog-ed25519:{key_id}:signing"),
            family_seed.as_bytes(),
        );

        let signing_key = ed25519_dalek::SigningKey::from_bytes(&derived);
        let verifying_key = signing_key.verifying_key();
        Ok((signing_key, verifying_key))
    }

    fn require_key_32(key: &[u8], name: &str) -> Result<[u8; 32], String> {
        key.try_into()
            .map_err(|_| format!("{name} must be exactly 32 bytes, got {}", key.len()))
    }

    fn require_key_16(key: &[u8], name: &str) -> Result<[u8; 16], String> {
        key.try_into()
            .map_err(|_| format!("{name} must be exactly 16 bytes, got {}", key.len()))
    }

    fn aad_or_empty(aad: &[u8]) -> Option<&[u8]> {
        if aad.is_empty() { None } else { Some(aad) }
    }
}

impl BearDogRpc for BearDogRpcServer {
    // ── Health ──────────────────────────────────────────────────────────

    async fn health_check(self, _: tarpc::context::Context) -> TarpcHealthStatus {
        debug!("tarpc health_check");
        TarpcHealthStatus {
            healthy: true,
            primal: self.identity.primal_name().to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            tarpc_method_count: TARPC_METHOD_COUNT,
            jsonrpc_method_count: 236,
        }
    }

    async fn version(self, _: tarpc::context::Context) -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }

    // ── Hash ───────────────────────────────────────────────────────────

    async fn blake3_hash(self, _: tarpc::context::Context, data: Vec<u8>) -> Vec<u8> {
        hash_blake3(&data)
    }

    async fn sha256(self, _: tarpc::context::Context, data: Vec<u8>) -> Vec<u8> {
        beardog_crypto::hash_sha256(&data)
    }

    async fn sha384(self, _: tarpc::context::Context, data: Vec<u8>) -> Vec<u8> {
        use sha2::{Digest, Sha384};
        let mut hasher = Sha384::new();
        hasher.update(&data);
        hasher.finalize().to_vec()
    }

    async fn sha512(self, _: tarpc::context::Context, data: Vec<u8>) -> Vec<u8> {
        beardog_crypto::hash_sha512(&data)
    }

    // ── MAC ────────────────────────────────────────────────────────────

    async fn hmac_sha256(
        self,
        _: tarpc::context::Context,
        key: Vec<u8>,
        data: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        beardog_crypto::hmac_sha256(&key, &data).map_err(|e| format!("HMAC failed: {e}"))
    }

    async fn blake3_keyed(
        self,
        _: tarpc::context::Context,
        key: Vec<u8>,
        data: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        let key = Self::require_key_32(&key, "BLAKE3 key")?;
        Ok(beardog_crypto::hash_blake3_keyed(&key, &data))
    }

    // ── KDF ────────────────────────────────────────────────────────────

    async fn hkdf_sha256(
        self,
        _: tarpc::context::Context,
        ikm: Vec<u8>,
        salt: Vec<u8>,
        info: Vec<u8>,
        output_length: u32,
    ) -> Result<Vec<u8>, String> {
        beardog_crypto::hkdf_sha256(&ikm, &salt, &info, output_length as usize)
            .map_err(|e| format!("HKDF failed: {e}"))
    }

    async fn blake3_derive_key(
        self,
        _: tarpc::context::Context,
        context: String,
        key_material: Vec<u8>,
    ) -> Vec<u8> {
        beardog_crypto::derive_key_blake3(&context, &key_material)
    }

    async fn argon2id_hash(
        self,
        _: tarpc::context::Context,
        password: Vec<u8>,
        salt: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        let salt: [u8; 16] = salt
            .try_into()
            .map_err(|_| "Argon2 salt must be exactly 16 bytes".to_string())?;
        beardog_crypto::hash_password_argon2(&password, &salt)
            .map_err(|e| format!("Argon2 failed: {e}"))
    }

    // ── Signing ────────────────────────────────────────────────────────

    async fn sign_ed25519(
        self,
        _: tarpc::context::Context,
        key_id: String,
        message: Vec<u8>,
    ) -> Result<SignResult, String> {
        let (signing_key, verifying_key) = self.derive_ed25519_keypair(&key_id)?;

        let mut expanded = [0u8; 64];
        expanded[..32].copy_from_slice(signing_key.as_bytes());
        expanded[32..].copy_from_slice(verifying_key.as_bytes());

        let signature =
            sign_ed25519(&message, &expanded).map_err(|e| format!("signing failed: {e}"))?;

        Ok(SignResult {
            signature,
            public_key: verifying_key.to_bytes().to_vec(),
        })
    }

    async fn verify_ed25519(
        self,
        _: tarpc::context::Context,
        public_key: Vec<u8>,
        message: Vec<u8>,
        signature: Vec<u8>,
    ) -> Result<bool, String> {
        verify_ed25519(&message, &signature, &public_key)
            .map_err(|e| format!("verification failed: {e}"))
    }

    // ── Key Exchange ───────────────────────────────────────────────────

    async fn x25519_generate_ephemeral(self, _: tarpc::context::Context) -> X25519Keypair {
        use x25519_dalek::{PublicKey, StaticSecret};

        let mut seed = [0u8; 32];
        rand::fill(&mut seed);
        let secret = StaticSecret::from(seed);
        let public = PublicKey::from(&secret);
        X25519Keypair {
            secret_key: secret.as_bytes().to_vec(),
            public_key: public.as_bytes().to_vec(),
        }
    }

    async fn x25519_derive_secret(
        self,
        _: tarpc::context::Context,
        secret_key: Vec<u8>,
        peer_public_key: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        use x25519_dalek::{PublicKey, StaticSecret};

        let sk_bytes = Self::require_key_32(&secret_key, "X25519 secret key")?;
        let pk_bytes = Self::require_key_32(&peer_public_key, "X25519 peer public key")?;

        let secret = StaticSecret::from(sk_bytes);
        let peer = PublicKey::from(pk_bytes);
        let shared = secret.diffie_hellman(&peer);

        Ok(shared.as_bytes().to_vec())
    }

    // ── AEAD ───────────────────────────────────────────────────────────

    async fn chacha20_poly1305_encrypt(
        self,
        _: tarpc::context::Context,
        data: Vec<u8>,
        key: Vec<u8>,
        aad: Vec<u8>,
    ) -> Result<AeadCiphertext, String> {
        let key = Self::require_key_32(&key, "ChaCha20-Poly1305 key")?;
        let (ciphertext, nonce, tag) =
            beardog_crypto::encrypt_chacha20_poly1305(&data, &key, Self::aad_or_empty(&aad))
                .map_err(|e| format!("ChaCha20-Poly1305 encrypt failed: {e}"))?;
        Ok(AeadCiphertext {
            ciphertext,
            nonce,
            tag,
        })
    }

    async fn chacha20_poly1305_decrypt(
        self,
        _: tarpc::context::Context,
        ciphertext: Vec<u8>,
        nonce: Vec<u8>,
        tag: Vec<u8>,
        key: Vec<u8>,
        aad: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        let key = Self::require_key_32(&key, "ChaCha20-Poly1305 key")?;
        beardog_crypto::decrypt_chacha20_poly1305(
            &ciphertext,
            &nonce,
            &tag,
            &key,
            Self::aad_or_empty(&aad),
        )
        .map_err(|e| format!("ChaCha20-Poly1305 decrypt failed: {e}"))
    }

    async fn aes256_gcm_encrypt(
        self,
        _: tarpc::context::Context,
        data: Vec<u8>,
        key: Vec<u8>,
        aad: Vec<u8>,
    ) -> Result<AeadCiphertext, String> {
        let key = Self::require_key_32(&key, "AES-256-GCM key")?;
        let (ciphertext, nonce, tag) =
            beardog_crypto::encrypt_aes_256_gcm(&data, &key, Self::aad_or_empty(&aad))
                .map_err(|e| format!("AES-256-GCM encrypt failed: {e}"))?;
        Ok(AeadCiphertext {
            ciphertext,
            nonce,
            tag,
        })
    }

    async fn aes256_gcm_decrypt(
        self,
        _: tarpc::context::Context,
        ciphertext: Vec<u8>,
        nonce: Vec<u8>,
        tag: Vec<u8>,
        key: Vec<u8>,
        aad: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        let key = Self::require_key_32(&key, "AES-256-GCM key")?;
        beardog_crypto::decrypt_aes_256_gcm(
            &ciphertext,
            &nonce,
            &tag,
            &key,
            Self::aad_or_empty(&aad),
        )
        .map_err(|e| format!("AES-256-GCM decrypt failed: {e}"))
    }

    async fn aes128_gcm_encrypt(
        self,
        _: tarpc::context::Context,
        data: Vec<u8>,
        key: Vec<u8>,
        aad: Vec<u8>,
    ) -> Result<AeadCiphertext, String> {
        let key = Self::require_key_16(&key, "AES-128-GCM key")?;
        let (ciphertext, nonce, tag) =
            beardog_crypto::encrypt_aes_128_gcm(&data, &key, Self::aad_or_empty(&aad))
                .map_err(|e| format!("AES-128-GCM encrypt failed: {e}"))?;
        Ok(AeadCiphertext {
            ciphertext,
            nonce,
            tag,
        })
    }

    async fn aes128_gcm_decrypt(
        self,
        _: tarpc::context::Context,
        ciphertext: Vec<u8>,
        nonce: Vec<u8>,
        tag: Vec<u8>,
        key: Vec<u8>,
        aad: Vec<u8>,
    ) -> Result<Vec<u8>, String> {
        let key = Self::require_key_16(&key, "AES-128-GCM key")?;
        beardog_crypto::decrypt_aes_128_gcm(
            &ciphertext,
            &nonce,
            &tag,
            &key,
            Self::aad_or_empty(&aad),
        )
        .map_err(|e| format!("AES-128-GCM decrypt failed: {e}"))
    }

    // ── Auth / Ionic Token ─────────────────────────────────────────────

    async fn auth_issue_ionic(
        self,
        _: tarpc::context::Context,
        subject: String,
        scope: Vec<String>,
        ttl_secs: i64,
    ) -> IonicTokenResult {
        let primal_name = self.identity.primal_name();
        let node_id = self.identity.node_id();
        let signing_key = derive_primal_signing_key(primal_name, node_id);
        let issuer_did = crate::ionic_token_handlers::did_from_ed25519_public(
            signing_key.verifying_key().as_bytes(),
        );

        let family_id = beardog_errors::process_env::var(
            beardog_config::env_keys::ENV_FAMILY_ID_PREFIXED,
        )
        .or_else(|_| beardog_errors::process_env::var(beardog_config::env_keys::ENV_FAMILY_ID))
        .ok()
        .filter(|v| !v.is_empty() && v != "default" && v != "standalone");

        let gate = family_id.map(|fid| GateIdentity {
            node_id: node_id.to_owned(),
            family_id: fid,
        });

        let token = issue_ionic_token_with_gate(
            &signing_key,
            &issuer_did,
            &subject,
            &scope,
            ttl_secs,
            gate.as_ref(),
        );

        IonicTokenResult {
            token,
            issuer: issuer_did,
            subject,
            scope,
            ttl_secs,
        }
    }

    async fn auth_verify_ionic(
        self,
        _: tarpc::context::Context,
        token: String,
    ) -> IonicVerifyResult {
        let primal_name = self.identity.primal_name();
        let node_id = self.identity.node_id();
        let signing_key = derive_primal_signing_key(primal_name, node_id);
        let verifying_key = signing_key.verifying_key();

        match crate::ionic_token::verify_ionic_token(&token, &verifying_key) {
            Ok(payload) => IonicVerifyResult {
                valid: true,
                issuer: Some(payload.iss),
                subject: Some(payload.sub),
                scope: payload.scope,
                exp: Some(payload.exp),
                error: None,
            },
            Err(e) => IonicVerifyResult {
                valid: false,
                issuer: None,
                subject: None,
                scope: vec![],
                exp: None,
                error: Some(e.to_string()),
            },
        }
    }

    async fn auth_public_key(self, _: tarpc::context::Context) -> PublicKeyInfo {
        let primal_name = self.identity.primal_name();
        let node_id = self.identity.node_id();
        let signing_key = derive_primal_signing_key(primal_name, node_id);
        let vk = signing_key.verifying_key();

        let did = crate::ionic_token_handlers::did_from_ed25519_public(vk.as_bytes());

        PublicKeyInfo {
            public_key_b64: base64::Engine::encode(
                &base64::engine::general_purpose::STANDARD,
                vk.as_bytes(),
            ),
            public_key_hex: hex::encode(vk.as_bytes()),
            did,
        }
    }

    async fn auth_issue_session(
        self,
        _: tarpc::context::Context,
        purpose: String,
        user: String,
    ) -> IonicTokenResult {
        let (scope, ttl_hours): (Vec<&str>, i64) = match purpose.as_str() {
            "jupyterhub" | "notebook" => (
                vec![
                    "crypto.*",
                    "health.*",
                    "capabilities.*",
                    "identity.*",
                    "content.*",
                    "auth.verify_ionic",
                ],
                8,
            ),
            "desktop" => (
                vec!["crypto.*", "health.*", "capabilities.*", "identity.*"],
                24,
            ),
            "admin" => (vec!["*"], 1),
            _ => (vec!["crypto.*", "health.*"], 4),
        };

        let scope_strings: Vec<String> = scope.iter().map(|s| (*s).to_string()).collect();
        let ttl_secs = ttl_hours * 3600;

        self.auth_issue_ionic(
            tarpc::context::current(),
            user,
            scope_strings,
            ttl_secs,
        )
        .await
    }

    async fn identity_create(self, _: tarpc::context::Context) -> SignResult {
        let seed: [u8; 32] = rand::random();
        let sk = ed25519_dalek::SigningKey::from_bytes(&seed);
        let vk = sk.verifying_key();

        SignResult {
            signature: sk.to_bytes().to_vec(),
            public_key: vk.to_bytes().to_vec(),
        }
    }

    async fn derive_key(
        self,
        _: tarpc::context::Context,
        key_id: String,
        purpose: String,
    ) -> Result<Vec<u8>, String> {
        let family_seed = beardog_errors::process_env::var(
            beardog_config::env_keys::ENV_FAMILY_SEED_PREFIXED,
        )
        .or_else(|_| beardog_errors::process_env::var(beardog_config::env_keys::ENV_FAMILY_SEED))
        .map_err(|_| "FAMILY_SEED not set".to_string())?;

        let derived = blake3::derive_key(
            &format!("beardog-{purpose}:{key_id}"),
            family_seed.as_bytes(),
        );
        Ok(derived.to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tarpc_method_count_matches_trait() {
        assert_eq!(TARPC_METHOD_COUNT, 30);
    }

    #[tokio::test]
    async fn tarpc_e2e_health_roundtrip() {
        use futures::StreamExt;
        use tarpc::server::{BaseChannel, Channel};

        let dir = tempfile::tempdir().unwrap();
        let sock = dir.path().join("test.sock");
        let sock_str = sock.to_str().unwrap();

        let tarpc_sock = super::super::tarpc_socket_path(sock_str);

        let incoming = tarpc::serde_transport::unix::listen(
            &tarpc_sock,
            tarpc::tokio_serde::formats::Bincode::default,
        )
        .await
        .unwrap();

        let identity = Arc::new(PrimalIdentity::from_env());
        let server = BearDogRpcServer::new(identity);

        tokio::spawn(async move {
            futures::pin_mut!(incoming);
            while let Some(Ok(transport)) = incoming.next().await {
                let channel = BaseChannel::with_defaults(transport);
                let handler = server.clone();
                tokio::spawn(
                    channel
                        .execute(handler.serve())
                        .for_each(|resp| async { tokio::spawn(resp); }),
                );
            }
        });

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let transport = tarpc::serde_transport::unix::connect(
            &tarpc_sock,
            tarpc::tokio_serde::formats::Bincode::default,
        )
        .await
        .unwrap();
        let client = BearDogRpcClient::new(tarpc::client::Config::default(), transport).spawn();

        let status = client.health_check(tarpc::context::current()).await.unwrap();
        assert!(status.healthy);
        assert_eq!(status.tarpc_method_count, 30);
        assert_eq!(status.version, env!("CARGO_PKG_VERSION"));

        let ver = client.version(tarpc::context::current()).await.unwrap();
        assert_eq!(ver, env!("CARGO_PKG_VERSION"));
    }

    #[tokio::test]
    async fn tarpc_e2e_crypto_roundtrip() {
        use futures::StreamExt;
        use tarpc::server::{BaseChannel, Channel};

        let dir = tempfile::tempdir().unwrap();
        let tarpc_sock = dir.path().join("crypto-test.tarpc.sock");
        let tarpc_sock_str = tarpc_sock.to_str().unwrap();

        let incoming = tarpc::serde_transport::unix::listen(
            tarpc_sock_str,
            tarpc::tokio_serde::formats::Bincode::default,
        )
        .await
        .unwrap();

        let identity = Arc::new(PrimalIdentity::from_env());
        let server = BearDogRpcServer::new(identity);

        tokio::spawn(async move {
            futures::pin_mut!(incoming);
            while let Some(Ok(transport)) = incoming.next().await {
                let channel = BaseChannel::with_defaults(transport);
                let handler = server.clone();
                tokio::spawn(
                    channel
                        .execute(handler.serve())
                        .for_each(|resp| async { tokio::spawn(resp); }),
                );
            }
        });

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let transport = tarpc::serde_transport::unix::connect(
            tarpc_sock_str,
            tarpc::tokio_serde::formats::Bincode::default,
        )
        .await
        .unwrap();
        let client = BearDogRpcClient::new(tarpc::client::Config::default(), transport).spawn();

        let data = b"hello beardog tarpc".to_vec();

        let hash = client
            .blake3_hash(tarpc::context::current(), data.clone())
            .await
            .unwrap();
        assert_eq!(hash.len(), 32);
        assert_eq!(hash, beardog_crypto::hash_blake3(&data));

        let sha = client
            .sha256(tarpc::context::current(), data.clone())
            .await
            .unwrap();
        assert_eq!(sha.len(), 32);

        let sha512 = client
            .sha512(tarpc::context::current(), data.clone())
            .await
            .unwrap();
        assert_eq!(sha512.len(), 64);

        let key = vec![0xAB; 32];
        let mac = client
            .hmac_sha256(tarpc::context::current(), key.clone(), data.clone())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(mac.len(), 32);

        let derived = client
            .blake3_derive_key(
                tarpc::context::current(),
                "test-context".to_string(),
                data.clone(),
            )
            .await
            .unwrap();
        assert_eq!(derived.len(), 32);

        let hkdf_out = client
            .hkdf_sha256(tarpc::context::current(), key.clone(), vec![], vec![], 48)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(hkdf_out.len(), 48);

        let chacha_key = vec![0x42; 32];
        let encrypted = client
            .chacha20_poly1305_encrypt(
                tarpc::context::current(),
                data.clone(),
                chacha_key.clone(),
                vec![],
            )
            .await
            .unwrap()
            .unwrap();
        let decrypted = client
            .chacha20_poly1305_decrypt(
                tarpc::context::current(),
                encrypted.ciphertext,
                encrypted.nonce,
                encrypted.tag,
                chacha_key,
                vec![],
            )
            .await
            .unwrap()
            .unwrap();
        assert_eq!(decrypted, data);

        let aes_key = vec![0x55; 32];
        let enc = client
            .aes256_gcm_encrypt(
                tarpc::context::current(),
                data.clone(),
                aes_key.clone(),
                b"aad".to_vec(),
            )
            .await
            .unwrap()
            .unwrap();
        let dec = client
            .aes256_gcm_decrypt(
                tarpc::context::current(),
                enc.ciphertext,
                enc.nonce,
                enc.tag,
                aes_key,
                b"aad".to_vec(),
            )
            .await
            .unwrap()
            .unwrap();
        assert_eq!(dec, data);

        let pk = vec![0u8; 32];
        let result = client
            .verify_ed25519(
                tarpc::context::current(),
                pk,
                data.clone(),
                vec![0u8; 64],
            )
            .await
            .unwrap();
        assert!(result.is_err() || !result.unwrap());
    }

    #[tokio::test]
    async fn tarpc_e2e_auth_roundtrip() {
        use futures::StreamExt;
        use tarpc::server::{BaseChannel, Channel};

        let dir = tempfile::tempdir().unwrap();
        let tarpc_sock = dir.path().join("auth-test.tarpc.sock");
        let tarpc_sock_str = tarpc_sock.to_str().unwrap();

        let incoming = tarpc::serde_transport::unix::listen(
            tarpc_sock_str,
            tarpc::tokio_serde::formats::Bincode::default,
        )
        .await
        .unwrap();

        let identity = Arc::new(PrimalIdentity::from_env());
        let server = BearDogRpcServer::new(identity);

        tokio::spawn(async move {
            futures::pin_mut!(incoming);
            while let Some(Ok(transport)) = incoming.next().await {
                let channel = BaseChannel::with_defaults(transport);
                let handler = server.clone();
                tokio::spawn(
                    channel
                        .execute(handler.serve())
                        .for_each(|resp| async { tokio::spawn(resp); }),
                );
            }
        });

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let transport = tarpc::serde_transport::unix::connect(
            tarpc_sock_str,
            tarpc::tokio_serde::formats::Bincode::default,
        )
        .await
        .unwrap();
        let client = BearDogRpcClient::new(tarpc::client::Config::default(), transport).spawn();

        let pk_info = client.auth_public_key(tarpc::context::current()).await.unwrap();
        assert!(pk_info.did.starts_with("did:key:z6Mk"));
        assert_eq!(pk_info.public_key_hex.len(), 64);

        let token_result = client
            .auth_issue_ionic(
                tarpc::context::current(),
                "test-subject".to_string(),
                vec!["crypto.*".to_string()],
                3600,
            )
            .await
            .unwrap();
        assert!(!token_result.token.is_empty());
        assert!(token_result.issuer.starts_with("did:key:z6Mk"));
        assert_eq!(token_result.subject, "test-subject");

        let verify_result = client
            .auth_verify_ionic(tarpc::context::current(), token_result.token.clone())
            .await
            .unwrap();
        assert!(verify_result.valid, "token should be valid: {:?}", verify_result.error);
        assert_eq!(verify_result.subject.unwrap(), "test-subject");

        let id = client.identity_create(tarpc::context::current()).await.unwrap();
        assert_eq!(id.public_key.len(), 32);
        assert_eq!(id.signature.len(), 32);

        let dk = client
            .blake3_derive_key(
                tarpc::context::current(),
                "test-context".to_string(),
                b"key-material".to_vec(),
            )
            .await
            .unwrap();
        assert_eq!(dk.len(), 32);
    }
}
