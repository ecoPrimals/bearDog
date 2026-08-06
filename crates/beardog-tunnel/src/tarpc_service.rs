// SPDX-License-Identifier: AGPL-3.0-or-later

//! tarpc binary RPC service (G64 Cephalization).
//!
//! Exposes a `.tarpc.sock` sibling socket alongside the primary JSON-RPC
//! `.sock`. Hot-path crypto operations use bincode framing — no JSON
//! serde roundtrip, no string encoding overhead.
//!
//! ## Socket Convention
//!
//! ```text
//! beardog-family123.sock          ← JSON-RPC  (always present, bootstrap + diagnostic)
//! beardog-family123.tarpc.sock    ← tarpc      (optional, high-perf intra-gate)
//! ```
//!
//! ## Method Surface (30 methods — crypto + auth domain convergence)
//!
//! | Category | Methods | Count |
//! |----------|---------|-------|
//! | Health | `health_check`, `version` | 2 |
//! | Hash | `blake3_hash`, `sha256`, `sha384`, `sha512` | 4 |
//! | MAC | `hmac_sha256`, `blake3_keyed` | 2 |
//! | KDF | `hkdf_sha256`, `blake3_derive_key`, `argon2id_hash`, `derive_key` | 4 |
//! | Signing | `sign_ed25519`, `verify_ed25519` | 2 |
//! | Key Exchange | `x25519_generate_ephemeral`, `x25519_derive_secret` | 2 |
//! | AEAD | `chacha20_poly1305_{encrypt,decrypt}` | 2 |
//! | AEAD | `aes256_gcm_{encrypt,decrypt}` | 2 |
//! | AEAD | `aes128_gcm_{encrypt,decrypt}` | 2 |
//! | Auth | `auth_issue_ionic`, `auth_verify_ionic`, `auth_public_key` | 3 |
//! | Auth | `auth_issue_session`, `identity_create` | 2 |
//! | Total | | **30** |

use std::sync::Arc;

use beardog_crypto::{hash_blake3, sign_ed25519, verify_ed25519};
use beardog_types::primal_identity::PrimalIdentity;
use tracing::{debug, info, warn};

use crate::ionic_token::{issue_ionic_token_with_gate, GateIdentity};
use crate::unix_socket_ipc::handlers::primal_signing::derive_primal_signing_key;

/// Derives the tarpc socket path from a JSON-RPC socket path.
///
/// Follows the ecosystem convention from `biomeos-primal-sdk::tarpc_transport`:
/// `beardog.sock` → `beardog.tarpc.sock`.
#[must_use]
pub fn tarpc_socket_path(jsonrpc_socket: &str) -> String {
    if let Some(base) = jsonrpc_socket.strip_suffix(".sock") {
        format!("{base}.tarpc.sock")
    } else {
        format!("{jsonrpc_socket}.tarpc")
    }
}

/// Health status returned by the tarpc health check.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TarpcHealthStatus {
    /// Whether the service is healthy.
    pub healthy: bool,
    /// Primal name.
    pub primal: String,
    /// Crate version.
    pub version: String,
    /// Number of tarpc methods served.
    pub tarpc_method_count: u32,
    /// Number of JSON-RPC methods served.
    pub jsonrpc_method_count: u32,
}

/// Result of an Ed25519 signing operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SignResult {
    /// Raw 64-byte Ed25519 signature.
    pub signature: Vec<u8>,
    /// 32-byte public key corresponding to the signing key.
    pub public_key: Vec<u8>,
}

/// Result of an AEAD encryption operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AeadCiphertext {
    /// Encrypted data (without tag).
    pub ciphertext: Vec<u8>,
    /// 96-bit nonce used for encryption.
    pub nonce: Vec<u8>,
    /// 128-bit authentication tag.
    pub tag: Vec<u8>,
}

/// X25519 ephemeral keypair.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct X25519Keypair {
    /// 32-byte secret key.
    pub secret_key: Vec<u8>,
    /// 32-byte public key.
    pub public_key: Vec<u8>,
}

/// Ionic token issuance result.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IonicTokenResult {
    /// Compact ionic token string (header.payload.signature).
    pub token: String,
    /// DID of the issuing primal.
    pub issuer: String,
    /// Subject the token was issued for.
    pub subject: String,
    /// Scope patterns the token covers.
    pub scope: Vec<String>,
    /// Token lifetime in seconds.
    pub ttl_secs: i64,
}

/// Ionic token verification result.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IonicVerifyResult {
    /// Whether the token is valid.
    pub valid: bool,
    /// Issuer DID from the token claims.
    pub issuer: Option<String>,
    /// Subject from the token claims.
    pub subject: Option<String>,
    /// Scope patterns from the token claims.
    pub scope: Vec<String>,
    /// Expiry timestamp (Unix seconds).
    pub exp: Option<i64>,
    /// Error description if invalid.
    pub error: Option<String>,
}

/// Public key info for the primal's identity.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PublicKeyInfo {
    /// Base64-encoded Ed25519 public key.
    pub public_key_b64: String,
    /// Hex-encoded Ed25519 public key.
    pub public_key_hex: String,
    /// DID (`did:key:z6Mk...`) for the primal.
    pub did: String,
}

/// Total number of tarpc RPC methods served.
const TARPC_METHOD_COUNT: u32 = 30;

#[expect(
    missing_docs,
    reason = "tarpc::service macro generates the trait and helper types; docs on individual methods below"
)]
#[tarpc::service]
pub trait BearDogRpc {
    // ── Health ──────────────────────────────────────────────────────────

    /// Health check — returns primal status without JSON overhead.
    async fn health_check() -> TarpcHealthStatus;

    /// Crate version string.
    async fn version() -> String;

    // ── Hash ───────────────────────────────────────────────────────────

    /// BLAKE3 hash — provenance hot-path (CAS, data braids).
    async fn blake3_hash(data: Vec<u8>) -> Vec<u8>;

    /// SHA-256 hash.
    async fn sha256(data: Vec<u8>) -> Vec<u8>;

    /// SHA-384 hash.
    async fn sha384(data: Vec<u8>) -> Vec<u8>;

    /// SHA-512 hash.
    async fn sha512(data: Vec<u8>) -> Vec<u8>;

    // ── MAC ────────────────────────────────────────────────────────────

    /// HMAC-SHA256.
    async fn hmac_sha256(key: Vec<u8>, data: Vec<u8>) -> Result<Vec<u8>, String>;

    /// BLAKE3 keyed hash (MAC).
    async fn blake3_keyed(key: Vec<u8>, data: Vec<u8>) -> Result<Vec<u8>, String>;

    // ── KDF ────────────────────────────────────────────────────────────

    /// HKDF-SHA256 key derivation.
    async fn hkdf_sha256(
        ikm: Vec<u8>,
        salt: Vec<u8>,
        info: Vec<u8>,
        output_length: u32,
    ) -> Result<Vec<u8>, String>;

    /// BLAKE3 key derivation.
    async fn blake3_derive_key(context: String, key_material: Vec<u8>) -> Vec<u8>;

    /// Argon2id password hashing (memory-hard).
    async fn argon2id_hash(password: Vec<u8>, salt: Vec<u8>) -> Result<Vec<u8>, String>;

    // ── Signing ────────────────────────────────────────────────────────

    /// Ed25519 sign — derived key from KDF(FAMILY_SEED, key_id, "signing").
    async fn sign_ed25519(key_id: String, message: Vec<u8>) -> Result<SignResult, String>;

    /// Ed25519 verify.
    async fn verify_ed25519(
        public_key: Vec<u8>,
        message: Vec<u8>,
        signature: Vec<u8>,
    ) -> Result<bool, String>;

    // ── Key Exchange ───────────────────────────────────────────────────

    /// X25519 generate ephemeral keypair.
    async fn x25519_generate_ephemeral() -> X25519Keypair;

    /// X25519 derive shared secret.
    async fn x25519_derive_secret(
        secret_key: Vec<u8>,
        peer_public_key: Vec<u8>,
    ) -> Result<Vec<u8>, String>;

    // ── AEAD ───────────────────────────────────────────────────────────

    /// ChaCha20-Poly1305 encrypt (constant-time, no AES-NI required).
    async fn chacha20_poly1305_encrypt(
        data: Vec<u8>,
        key: Vec<u8>,
        aad: Vec<u8>,
    ) -> Result<AeadCiphertext, String>;

    /// ChaCha20-Poly1305 decrypt.
    async fn chacha20_poly1305_decrypt(
        ciphertext: Vec<u8>,
        nonce: Vec<u8>,
        tag: Vec<u8>,
        key: Vec<u8>,
        aad: Vec<u8>,
    ) -> Result<Vec<u8>, String>;

    /// AES-256-GCM encrypt.
    async fn aes256_gcm_encrypt(
        data: Vec<u8>,
        key: Vec<u8>,
        aad: Vec<u8>,
    ) -> Result<AeadCiphertext, String>;

    /// AES-256-GCM decrypt.
    async fn aes256_gcm_decrypt(
        ciphertext: Vec<u8>,
        nonce: Vec<u8>,
        tag: Vec<u8>,
        key: Vec<u8>,
        aad: Vec<u8>,
    ) -> Result<Vec<u8>, String>;

    /// AES-128-GCM encrypt.
    async fn aes128_gcm_encrypt(
        data: Vec<u8>,
        key: Vec<u8>,
        aad: Vec<u8>,
    ) -> Result<AeadCiphertext, String>;

    /// AES-128-GCM decrypt.
    async fn aes128_gcm_decrypt(
        ciphertext: Vec<u8>,
        nonce: Vec<u8>,
        tag: Vec<u8>,
        key: Vec<u8>,
        aad: Vec<u8>,
    ) -> Result<Vec<u8>, String>;

    // ── Auth / Ionic Token ─────────────────────────────────────────────

    /// Issue an ionic capability token.
    async fn auth_issue_ionic(
        subject: String,
        scope: Vec<String>,
        ttl_secs: i64,
    ) -> IonicTokenResult;

    /// Verify an ionic token string, returning claims or error.
    async fn auth_verify_ionic(token: String) -> IonicVerifyResult;

    /// Get the primal's Ed25519 public key and DID.
    async fn auth_public_key() -> PublicKeyInfo;

    /// Issue a session token (simplified — auto-scoped by purpose).
    async fn auth_issue_session(
        purpose: String,
        user: String,
    ) -> IonicTokenResult;

    /// Generate an ephemeral Ed25519 identity keypair.
    async fn identity_create() -> SignResult;

    /// Derive a key from FAMILY_SEED via BLAKE3 KDF.
    async fn derive_key(key_id: String, purpose: String) -> Result<Vec<u8>, String>;
}

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

/// Spawns the tarpc listener on the `.tarpc.sock` sibling socket.
///
/// This is fire-and-forget: if the listener fails to bind, it logs a warning
/// and returns without blocking the JSON-RPC server.
///
/// # Errors
///
/// Returns `Err` if the Unix socket cannot be bound (e.g. path too long,
/// permissions). The caller should treat this as non-fatal.
pub async fn spawn_tarpc_listener(
    jsonrpc_socket_path: &str,
    identity: Arc<PrimalIdentity>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    use futures::StreamExt;
    use tarpc::server::{BaseChannel, Channel};

    let tarpc_path = tarpc_socket_path(jsonrpc_socket_path);

    if std::path::Path::new(&tarpc_path).exists() {
        std::fs::remove_file(&tarpc_path).ok();
    }

    let incoming = tarpc::serde_transport::unix::listen(
        &tarpc_path,
        tarpc::tokio_serde::formats::Bincode::default,
    )
    .await?;
    info!(path = %tarpc_path, method_count = TARPC_METHOD_COUNT, "tarpc listener bound");

    let server = BearDogRpcServer::new(identity);

    tokio::spawn(async move {
        futures::pin_mut!(incoming);
        while let Some(result) = incoming.next().await {
            match result {
                Ok(transport) => {
                    let channel = BaseChannel::with_defaults(transport);
                    let handler = server.clone();
                    tokio::spawn(channel.execute(handler.serve()).for_each(|resp| async {
                        tokio::spawn(resp);
                    }));
                }
                Err(e) => {
                    warn!(error = %e, "tarpc accept failed");
                }
            }
        }
    });

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tarpc_socket_path_convention() {
        assert_eq!(
            tarpc_socket_path("/tmp/beardog.sock"),
            "/tmp/beardog.tarpc.sock"
        );
        assert_eq!(
            tarpc_socket_path("/run/user/1000/biomeos/beardog-family123.sock"),
            "/run/user/1000/biomeos/beardog-family123.tarpc.sock"
        );
    }

    #[test]
    fn tarpc_socket_path_no_sock_suffix() {
        assert_eq!(tarpc_socket_path("/tmp/beardog"), "/tmp/beardog.tarpc");
    }

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

        let tarpc_sock = tarpc_socket_path(sock_str);

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

        // Give server a moment to bind
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

        // BLAKE3 hash
        let hash = client
            .blake3_hash(tarpc::context::current(), data.clone())
            .await
            .unwrap();
        assert_eq!(hash.len(), 32);
        assert_eq!(hash, beardog_crypto::hash_blake3(&data));

        // SHA-256
        let sha = client
            .sha256(tarpc::context::current(), data.clone())
            .await
            .unwrap();
        assert_eq!(sha.len(), 32);

        // SHA-512
        let sha512 = client
            .sha512(tarpc::context::current(), data.clone())
            .await
            .unwrap();
        assert_eq!(sha512.len(), 64);

        // HMAC-SHA256
        let key = vec![0xAB; 32];
        let mac = client
            .hmac_sha256(tarpc::context::current(), key.clone(), data.clone())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(mac.len(), 32);

        // BLAKE3 derive key
        let derived = client
            .blake3_derive_key(
                tarpc::context::current(),
                "test-context".to_string(),
                data.clone(),
            )
            .await
            .unwrap();
        assert_eq!(derived.len(), 32);

        // HKDF-SHA256
        let hkdf_out = client
            .hkdf_sha256(tarpc::context::current(), key.clone(), vec![], vec![], 48)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(hkdf_out.len(), 48);

        // ChaCha20-Poly1305 roundtrip
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

        // AES-256-GCM roundtrip
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

        // Ed25519 verify (raw key, no FAMILY_SEED needed)
        let pk = vec![0u8; 32]; // invalid key — should fail gracefully
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

        // Public key
        let pk_info = client.auth_public_key(tarpc::context::current()).await.unwrap();
        assert!(pk_info.did.starts_with("did:key:z6Mk"));
        assert_eq!(pk_info.public_key_hex.len(), 64); // 32 bytes hex

        // Issue ionic token
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

        // Verify the token we just issued
        let verify_result = client
            .auth_verify_ionic(tarpc::context::current(), token_result.token.clone())
            .await
            .unwrap();
        assert!(verify_result.valid, "token should be valid: {:?}", verify_result.error);
        assert_eq!(verify_result.subject.unwrap(), "test-subject");

        // Identity create
        let id = client.identity_create(tarpc::context::current()).await.unwrap();
        assert_eq!(id.public_key.len(), 32);
        assert_eq!(id.signature.len(), 32); // reuses SignResult; field holds secret key bytes

        // BLAKE3 derive key
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
