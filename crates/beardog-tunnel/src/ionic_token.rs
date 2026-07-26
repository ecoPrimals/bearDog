// SPDX-License-Identifier: AGPL-3.0-or-later

//! Ed25519-signed ionic capability tokens (JH-1).
//!
//! Wire format: `base64(header).base64(payload).base64(signature)` —
//! three dot-separated standard-base64 segments. The signature covers
//! the first two segments verbatim (i.e. the ASCII bytes of
//! `base64(header).base64(payload)`), matching JWT's signing input.
//!
//! Tokens are issued by [`issue_ionic_token`] using the primal's
//! deterministic Ed25519 identity key and verified by
//! [`verify_ionic_token`] using the corresponding public key.

use base64::Engine;
use chrono::Utc;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};

const B64: &base64::engine::GeneralPurpose = &base64::engine::general_purpose::STANDARD;

// ── Token wire types ────────────────────────────────────────────────────

/// Header of an ionic token (`alg`, `typ`, `ver`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicTokenHeader {
    /// Signing algorithm (always `"EdDSA"`).
    pub alg: String,
    /// Token type (always `"ionic"`).
    pub typ: String,
    /// Wire format version.
    pub ver: u32,
}

impl Default for IonicTokenHeader {
    fn default() -> Self {
        Self {
            alg: "EdDSA".to_owned(),
            typ: "ionic".to_owned(),
            ver: 1,
        }
    }
}

/// Claims payload of an ionic token.
///
/// v2 adds optional cross-gate claims (`gate_id`, `family_id`) so verifiers
/// can identify the issuing gate without an external registry lookup.  Both
/// fields are `#[serde(skip_serializing_if)]` / `#[serde(default)]` for
/// backward compatibility with v1 tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IonicTokenPayload {
    /// Issuer DID (`did:key:z6Mk...` of the signing primal).
    pub iss: String,
    /// Subject — who the token was issued for.
    pub sub: String,
    /// Method scope patterns (e.g. `["crypto.*", "health.*"]` or `["*"]`).
    pub scope: Vec<String>,
    /// Issued-at (Unix timestamp, seconds).
    pub iat: i64,
    /// Expiry (Unix timestamp, seconds).
    pub exp: i64,
    /// Unique token ID (hex, for future revocation tracking).
    pub jti: String,

    // ── Cross-gate identity claims (v2) ──────────────────────────────
    /// Issuing gate's `NODE_ID`.  Allows the verifier to identify which
    /// gate instance created the token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gate_id: Option<String>,

    /// Issuing gate's `FAMILY_ID`.  Paired with a seed fingerprint in
    /// the trust registry, this binds the token to a cryptographic family.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_id: Option<String>,
}

// ── Errors ──────────────────────────────────────────────────────────────

/// Token verification failure.
#[derive(Debug, thiserror::Error)]
pub enum TokenError {
    /// Token string is not three dot-separated base64 segments.
    #[error("malformed token: {0}")]
    Malformed(String),
    /// Ed25519 signature does not verify.
    #[error("invalid Ed25519 signature")]
    InvalidSignature,
    /// Token `exp` is in the past.
    #[error("token expired (exp={exp}, now={now})")]
    Expired {
        /// Token expiry timestamp.
        exp: i64,
        /// Current timestamp at verification time.
        now: i64,
    },
    /// Token header `typ` or `alg` is not recognized.
    #[error("unsupported token format: {0}")]
    UnsupportedFormat(String),
}

// ── Issue ───────────────────────────────────────────────────────────────

/// Gate identity for embedding cross-gate claims in issued tokens.
#[derive(Debug, Clone)]
pub struct GateIdentity {
    /// `NODE_ID` of the issuing gate.
    pub node_id: String,
    /// `FAMILY_ID` of the issuing gate.
    pub family_id: String,
}
/// Issue a signed ionic token.
///
/// `issuer_did` should be the `did:key:z6Mk...` of the signing primal.
/// `subject` identifies who the token is for (a user DID, primal name, etc.).
/// `scopes` are glob patterns like `["crypto.*"]` or `["*"]`.
/// `ttl_secs` is the token lifetime in seconds.
/// `gate` optionally embeds the issuing gate's identity for cross-gate
/// verification.
///
/// Returns the compact wire representation: `header_b64.payload_b64.sig_b64`.
#[must_use]
pub fn issue_ionic_token(
    signing_key: &SigningKey,
    issuer_did: &str,
    subject: &str,
    scopes: &[String],
    ttl_secs: i64,
) -> String {
    issue_ionic_token_with_gate(signing_key, issuer_did, subject, scopes, ttl_secs, None)
}
/// Issue a signed ionic token with optional cross-gate identity claims.
#[must_use]
pub fn issue_ionic_token_with_gate(
    signing_key: &SigningKey,
    issuer_did: &str,
    subject: &str,
    scopes: &[String],
    ttl_secs: i64,
    gate: Option<&GateIdentity>,
) -> String {
    let now = Utc::now().timestamp();

    let header = IonicTokenHeader::default();
    let payload = IonicTokenPayload {
        iss: issuer_did.to_owned(),
        sub: subject.to_owned(),
        scope: scopes.to_vec(),
        iat: now,
        exp: now + ttl_secs,
        jti: generate_jti(),
        gate_id: gate.map(|g| g.node_id.clone()),
        family_id: gate.map(|g| g.family_id.clone()),
    };

    let header_json = serde_json::to_vec(&header).unwrap_or_default();
    let payload_json = serde_json::to_vec(&payload).unwrap_or_default();
    let header_b64 = B64.encode(&header_json);
    let payload_b64 = B64.encode(&payload_json);

    let signing_input = format!("{header_b64}.{payload_b64}");
    let signature = signing_key.sign(signing_input.as_bytes());
    let sig_b64 = B64.encode(signature.to_bytes());

    format!("{signing_input}.{sig_b64}")
}

/// 16-byte random hex token ID for future revocation tracking.
fn generate_jti() -> String {
    use rand::Rng;
    let bytes: [u8; 16] = rand::rng().random();
    hex::encode(bytes)
}

// ── Verify ──────────────────────────────────────────────────────────────

/// Verify an ionic token string and return the payload claims.
///
/// Checks: structure, header type/alg, Ed25519 signature, expiry.
/// Does **not** check scope — that is the caller's responsibility via
/// [`scope_covers_method`].
///
/// # Errors
///
/// Returns [`TokenError`] on any verification failure.
pub fn verify_ionic_token(
    token_str: &str,
    verifying_key: &VerifyingKey,
) -> Result<IonicTokenPayload, TokenError> {
    let parts: Vec<&str> = token_str.splitn(3, '.').collect();
    if parts.len() != 3 {
        return Err(TokenError::Malformed(
            "expected 3 dot-separated segments".to_owned(),
        ));
    }

    let header_bytes = B64
        .decode(parts[0])
        .map_err(|e| TokenError::Malformed(format!("header base64: {e}")))?;
    let payload_bytes = B64
        .decode(parts[1])
        .map_err(|e| TokenError::Malformed(format!("payload base64: {e}")))?;
    let sig_bytes = B64
        .decode(parts[2])
        .map_err(|e| TokenError::Malformed(format!("signature base64: {e}")))?;

    let header: IonicTokenHeader = serde_json::from_slice(&header_bytes)
        .map_err(|e| TokenError::Malformed(format!("header JSON: {e}")))?;

    if header.typ != "ionic" || header.alg != "EdDSA" {
        return Err(TokenError::UnsupportedFormat(format!(
            "typ={}, alg={}",
            header.typ, header.alg
        )));
    }

    let sig_arr: [u8; 64] = sig_bytes
        .try_into()
        .map_err(|_| TokenError::Malformed("signature must be 64 bytes".to_owned()))?;
    let signature = Signature::from_bytes(&sig_arr);

    let signing_input = format!("{}.{}", parts[0], parts[1]);
    verifying_key
        .verify(signing_input.as_bytes(), &signature)
        .map_err(|_| TokenError::InvalidSignature)?;

    let payload: IonicTokenPayload = serde_json::from_slice(&payload_bytes)
        .map_err(|e| TokenError::Malformed(format!("payload JSON: {e}")))?;

    let now = Utc::now().timestamp();
    if payload.exp <= now {
        return Err(TokenError::Expired {
            exp: payload.exp,
            now,
        });
    }

    Ok(payload)
}

// ── Scope matching ──────────────────────────────────────────────────────

/// Check if any scope pattern in `scopes` covers the given `method`.
///
/// Patterns:
/// - `"*"` — matches everything
/// - `"crypto.*"` — matches any method starting with `crypto.`
/// - `"crypto.sign_ed25519"` — exact match
#[must_use]
pub fn scope_covers_method(scopes: &[String], method: &str) -> bool {
    for scope in scopes {
        if scope == "*" {
            return true;
        }
        if let Some(prefix) = scope.strip_suffix(".*") {
            if method.starts_with(prefix) && method.as_bytes().get(prefix.len()) == Some(&b'.') {
                return true;
            }
        } else if scope == method {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_keypair() -> (SigningKey, VerifyingKey) {
        use crate::unix_socket_ipc::handlers::primal_signing::derive_primal_signing_key;
        let sk = derive_primal_signing_key("beardog", "test-node");
        let vk = sk.verifying_key();
        (sk, vk)
    }

    // ── issue / verify roundtrip ──

    #[test]
    fn issue_and_verify_roundtrip() {
        let (sk, vk) = test_keypair();
        let token = issue_ionic_token(
            &sk,
            "did:key:z6MkTest",
            "alice",
            &["crypto.*".to_owned(), "health.*".to_owned()],
            3600,
        );

        let payload = verify_ionic_token(&token, &vk).expect("valid token");
        assert_eq!(payload.iss, "did:key:z6MkTest");
        assert_eq!(payload.sub, "alice");
        assert_eq!(payload.scope, vec!["crypto.*", "health.*"]);
        assert!(payload.exp > payload.iat);
        assert_eq!(payload.jti.len(), 32);
    }

    #[test]
    fn wildcard_scope_roundtrip() {
        let (sk, vk) = test_keypair();
        let token = issue_ionic_token(&sk, "did:key:z6MkTest", "admin", &["*".to_owned()], 60);
        let payload = verify_ionic_token(&token, &vk).expect("valid token");
        assert_eq!(payload.scope, vec!["*"]);
    }

    // ── signature verification ──

    #[test]
    fn wrong_key_rejects() {
        let (sk, _) = test_keypair();
        let token = issue_ionic_token(&sk, "did:key:z6MkTest", "alice", &["*".to_owned()], 3600);

        let other_sk = crate::unix_socket_ipc::handlers::primal_signing::derive_primal_signing_key(
            "other-primal",
            "other-node",
        );
        let other_vk = other_sk.verifying_key();

        let result = verify_ionic_token(&token, &other_vk);
        assert!(matches!(result, Err(TokenError::InvalidSignature)));
    }

    #[test]
    fn tampered_payload_rejects() {
        let (sk, vk) = test_keypair();
        let token = issue_ionic_token(&sk, "did:key:z6MkTest", "alice", &["*".to_owned()], 3600);

        let parts: Vec<&str> = token.splitn(3, '.').collect();
        let mut payload_bytes = B64.decode(parts[1]).unwrap();
        payload_bytes[0] ^= 0xff;
        let tampered = format!("{}.{}.{}", parts[0], B64.encode(&payload_bytes), parts[2]);

        let result = verify_ionic_token(&tampered, &vk);
        assert!(matches!(result, Err(TokenError::InvalidSignature)));
    }

    // ── expiry ──

    #[test]
    fn expired_token_rejects() {
        let (sk, vk) = test_keypair();
        let token = issue_ionic_token(&sk, "did:key:z6MkTest", "alice", &["*".to_owned()], -10);
        let result = verify_ionic_token(&token, &vk);
        assert!(matches!(result, Err(TokenError::Expired { .. })));
    }

    // ── malformed ──

    #[test]
    fn missing_segments_rejects() {
        let (_, vk) = test_keypair();
        let result = verify_ionic_token("only.two", &vk);
        assert!(matches!(result, Err(TokenError::Malformed(_))));
    }

    #[test]
    fn bad_base64_rejects() {
        let (_, vk) = test_keypair();
        let result = verify_ionic_token("!!!.!!!.!!!", &vk);
        assert!(matches!(result, Err(TokenError::Malformed(_))));
    }

    // ── scope matching ──

    #[test]
    fn wildcard_covers_everything() {
        assert!(scope_covers_method(&["*".to_owned()], "crypto.sign"));
        assert!(scope_covers_method(&["*".to_owned()], "anything"));
    }

    #[test]
    fn prefix_scope_matches() {
        let scopes = vec!["crypto.*".to_owned()];
        assert!(scope_covers_method(&scopes, "crypto.sign_ed25519"));
        assert!(scope_covers_method(&scopes, "crypto.blake3_hash"));
        assert!(!scope_covers_method(&scopes, "health.check"));
        assert!(!scope_covers_method(&scopes, "cryptography.other"));
    }

    #[test]
    fn exact_scope_matches() {
        let scopes = vec!["crypto.sign_ed25519".to_owned()];
        assert!(scope_covers_method(&scopes, "crypto.sign_ed25519"));
        assert!(!scope_covers_method(&scopes, "crypto.blake3_hash"));
    }

    #[test]
    fn empty_scope_matches_nothing() {
        assert!(!scope_covers_method(&[], "crypto.sign"));
    }

    #[test]
    fn multiple_scopes_any_match() {
        let scopes = vec!["health.*".to_owned(), "auth.*".to_owned()];
        assert!(scope_covers_method(&scopes, "health.check"));
        assert!(scope_covers_method(&scopes, "auth.mode"));
        assert!(!scope_covers_method(&scopes, "crypto.sign"));
    }

    #[test]
    fn prefix_scope_requires_dot_boundary() {
        let scopes = vec!["crypto.*".to_owned()];
        assert!(!scope_covers_method(&scopes, "cryptography.other"));
        assert!(scope_covers_method(&scopes, "crypto.other"));
    }
}
