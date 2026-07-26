// SPDX-License-Identifier: AGPL-3.0-or-later

//! Unified primal identity signing.
//!
//! Derives a single Ed25519 keypair per runtime identity (primal name + node ID).
//! All subsystems that need to sign on behalf of the primal (capability
//! announcements, ionic bonds, contract signing, neural registration) use this
//! shared derivation so verifiers see **one** public key per primal instance.
//!
//! The seed is `SHA-256("primal-identity-key:" || name || ":" || node_id)`.

use base64::Engine;
use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use sha2::{Digest, Sha256};

/// Derive the primal's Ed25519 signing key from its runtime identity.
///
/// Deterministic: same `(primal_name, node_id)` always yields the same key.
#[must_use]
pub fn derive_primal_signing_key(primal_name: &str, node_id: &str) -> SigningKey {
    let mut seed = [0u8; 32];
    let mut h = Sha256::new();
    h.update(b"primal-identity-key:");
    h.update(primal_name.as_bytes());
    h.update(b":");
    h.update(node_id.as_bytes());
    seed.copy_from_slice(&h.finalize());
    SigningKey::from_bytes(&seed)
}

/// Derive the primal's Ed25519 verifying (public) key.
#[must_use]
pub fn derive_primal_verifying_key(primal_name: &str, node_id: &str) -> VerifyingKey {
    derive_primal_signing_key(primal_name, node_id).verifying_key()
}
/// Sign arbitrary bytes with the primal's identity key.
///
/// Returns `(signature_base64, public_key_base64)` using standard base64.
#[must_use]
pub fn sign_with_primal_identity(
    primal_name: &str,
    node_id: &str,
    message: &[u8],
) -> (String, String) {
    let b64 = &base64::engine::general_purpose::STANDARD;
    let signing_key = derive_primal_signing_key(primal_name, node_id);
    let verifying_key = signing_key.verifying_key();
    let signature = signing_key.sign(message);
    (
        b64.encode(signature.to_bytes()),
        b64.encode(verifying_key.as_bytes()),
    )
}

/// Build the canonical bytes for a capability announcement signature.
///
/// The signed payload is `SHA-256(primal || ":" || version || ":" || sorted_methods)`.
/// Hash-then-sign ensures the signed message is a fixed 32 bytes regardless of
/// method count, and the canonical form is unambiguous for any verifier.
#[must_use]
pub fn canonical_announcement_message(
    primal_name: &str,
    version: &str,
    methods: &[impl AsRef<str>],
) -> Vec<u8> {
    let mut sorted: Vec<&str> = methods.iter().map(AsRef::as_ref).collect();
    sorted.sort_unstable();

    let mut h = Sha256::new();
    h.update(primal_name.as_bytes());
    h.update(b":");
    h.update(version.as_bytes());
    h.update(b":");
    for m in &sorted {
        h.update(m.as_bytes());
        h.update(b",");
    }
    h.finalize().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use ed25519_dalek::Verifier;

    #[test]
    fn deterministic_key_derivation() {
        let k1 = derive_primal_signing_key("beardog", "node-1");
        let k2 = derive_primal_signing_key("beardog", "node-1");
        assert_eq!(k1.to_bytes(), k2.to_bytes());
    }

    #[test]
    fn different_names_yield_different_keys() {
        let k1 = derive_primal_signing_key("beardog", "node-1");
        let k2 = derive_primal_signing_key("songbird", "node-1");
        assert_ne!(k1.to_bytes(), k2.to_bytes());
    }

    #[test]
    fn sign_and_verify_roundtrip() {
        let (sig_b64, pk_b64) = sign_with_primal_identity("beardog", "node-1", b"hello");
        let pk_bytes: [u8; 32] = BASE64.decode(&pk_b64).unwrap().try_into().unwrap();
        let sig_bytes: [u8; 64] = BASE64.decode(&sig_b64).unwrap().try_into().unwrap();
        let vk = VerifyingKey::from_bytes(&pk_bytes).unwrap();
        let sig = ed25519_dalek::Signature::from_bytes(&sig_bytes);
        assert!(vk.verify(b"hello", &sig).is_ok());
    }

    #[test]
    fn canonical_message_sorts_methods() {
        let methods_a = vec!["z.foo".to_string(), "a.bar".to_string()];
        let methods_b = vec!["a.bar".to_string(), "z.foo".to_string()];
        assert_eq!(
            canonical_announcement_message("p", "1.0", &methods_a),
            canonical_announcement_message("p", "1.0", &methods_b),
        );
    }

    #[test]
    fn canonical_message_differs_on_version() {
        let m = vec!["a.b".to_string()];
        assert_ne!(
            canonical_announcement_message("p", "1.0", &m),
            canonical_announcement_message("p", "2.0", &m),
        );
    }
}
