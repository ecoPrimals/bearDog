// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::btsp_provider::BeardogBtspProvider;
use beardog_types::ionic_bond::IonicBondProposeParams;
use std::sync::Arc;

/// Sign the terms hash with the primal's Ed25519 identity key.
///
/// Returns `(signature_hex, public_key_hex)`. The key seed is derived
/// deterministically from the primal's runtime identity (`PRIMAL_NAME` +
/// node-id) via SHA-256.
///
/// **BTSP Phase 3 (HSM path):** The `_btsp_provider` parameter is
/// reserved for HSM-backed signing via `HsmKeyProvider::sign()`. When
/// an HSM is available, the signing key should be generated/stored in
/// hardware rather than derived from environment. The current software
/// derivation path is production-safe but not sovereign-grade.
pub(super) fn sign_terms_ed25519(
    _btsp_provider: &Arc<BeardogBtspProvider>,
    terms_hash: &str,
) -> Result<(String, String), String> {
    use ed25519_dalek::{Signer, SigningKey};
    use sha2::{Digest, Sha256};

    let primal_name = std::env::var("PRIMAL_NAME").unwrap_or_else(|_| "beardog".to_string());
    let node_id = beardog_types::primal_identity::resolve_node_id_from_env_or_ephemeral(None);

    let mut seed = [0u8; 32];
    let mut h = Sha256::new();
    h.update(b"ionic-bond-identity-seed:");
    h.update(primal_name.as_bytes());
    h.update(b":");
    h.update(node_id.as_bytes());
    seed.copy_from_slice(&h.finalize());

    let signing_key = SigningKey::from_bytes(&seed);
    let verifying_key = signing_key.verifying_key();
    let signature = signing_key.sign(terms_hash.as_bytes());
    Ok((
        hex::encode(signature.to_bytes()),
        hex::encode(verifying_key.as_bytes()),
    ))
}

/// Verify an Ed25519 signature over the terms hash using the provided
/// hex-encoded public key and signature.
pub(super) fn verify_ed25519_signature(
    terms_hash: &str,
    signature_hex: &str,
    public_key_hex: &str,
) -> Result<(), String> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    let pub_bytes: [u8; 32] = hex::decode(public_key_hex)
        .map_err(|e| format!("Invalid acceptor public key hex: {e}"))?
        .try_into()
        .map_err(|v: Vec<u8>| format!("Acceptor public key must be 32 bytes, got {}", v.len()))?;

    let sig_bytes: [u8; 64] = hex::decode(signature_hex)
        .map_err(|e| format!("Invalid acceptor signature hex: {e}"))?
        .try_into()
        .map_err(|v: Vec<u8>| format!("Acceptor signature must be 64 bytes, got {}", v.len()))?;

    let verifying_key = VerifyingKey::from_bytes(&pub_bytes)
        .map_err(|e| format!("Invalid Ed25519 public key: {e}"))?;

    let signature = Signature::from_bytes(&sig_bytes);

    verifying_key
        .verify(terms_hash.as_bytes(), &signature)
        .map_err(|e| format!("Acceptor signature verification failed: {e}"))
}

pub(super) fn compute_terms_hash(params: &IonicBondProposeParams) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(params.proposer.as_bytes());
    hasher.update(params.target.as_bytes());
    hasher.update(
        serde_json::to_string(&params.trust_model)
            .unwrap_or_default()
            .as_bytes(),
    );
    hasher.update(
        serde_json::to_string(&params.encryption_tier)
            .unwrap_or_default()
            .as_bytes(),
    );
    for cap in &params.allowed_capabilities {
        hasher.update(cap.as_bytes());
    }
    hex::encode(hasher.finalize())
}

/// Compute a deterministic SHA-256 hash of arbitrary contract terms.
///
/// The JSON value is serialized with sorted keys to ensure determinism
/// regardless of field ordering in the caller's payload.
pub(super) fn compute_contract_terms_hash(terms: &serde_json::Value) -> String {
    use sha2::{Digest, Sha256};
    let canonical = canonical_json(terms);
    hex::encode(Sha256::digest(canonical.as_bytes()))
}

/// Recursively sort JSON object keys for deterministic serialization.
pub(super) fn canonical_json(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::Object(map) => {
            let mut sorted: Vec<_> = map.iter().collect();
            sorted.sort_by_key(|(k, _)| *k);
            let entries: Vec<String> = sorted
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}:{}",
                        serde_json::to_string(k).unwrap_or_default(),
                        canonical_json(v)
                    )
                })
                .collect();
            format!("{{{}}}", entries.join(","))
        }
        serde_json::Value::Array(arr) => {
            let entries: Vec<String> = arr.iter().map(canonical_json).collect();
            format!("[{}]", entries.join(","))
        }
        _ => serde_json::to_string(value).unwrap_or_default(),
    }
}
