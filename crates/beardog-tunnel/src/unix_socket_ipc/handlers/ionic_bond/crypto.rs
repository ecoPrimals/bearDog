// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::btsp_provider::BeardogBtspProvider;
use base64::Engine;
use beardog_config::env_keys;
use beardog_types::ionic_bond::IonicBondProposeParams;
use std::sync::Arc;

/// Sign the terms hash with the primal's unified Ed25519 identity key.
///
/// Returns `(signature_base64, public_key_base64)` using standard base64.
/// Uses the shared primal identity derivation from
/// [`super::super::primal_signing`] so the same public key appears in
/// capability announcements, ionic bonds, and neural registration.
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
    let primal_name = std::env::var(env_keys::ENV_PRIMAL_NAME)
        .unwrap_or_else(|_| env_keys::DEFAULT_PRIMAL_NAME.to_string());
    let node_id = beardog_types::primal_identity::resolve_node_id_from_env_or_ephemeral(None);

    Ok(
        crate::unix_socket_ipc::handlers::primal_signing::sign_with_primal_identity(
            &primal_name,
            &node_id,
            terms_hash.as_bytes(),
        ),
    )
}

/// Decode an Ed25519 field (signature or public key) from standard base64,
/// falling back to hex for backward compatibility with pre-Wave 62 callers.
fn decode_ed25519_field(
    encoded: &str,
    field_name: &str,
    expected_len: usize,
) -> Result<Vec<u8>, String> {
    let b64 = &base64::engine::general_purpose::STANDARD;
    b64.decode(encoded)
        .or_else(|_| hex::decode(encoded))
        .map_err(|e| format!("Invalid {field_name} (expected base64 or hex): {e}"))
        .and_then(|bytes| {
            if bytes.len() == expected_len {
                Ok(bytes)
            } else {
                Err(format!(
                    "{field_name} must be {expected_len} bytes, got {}",
                    bytes.len()
                ))
            }
        })
}

/// Verify an Ed25519 signature over the terms hash.
///
/// Accepts standard base64 (preferred) or hex-encoded public key and
/// signature for backward compatibility.
pub(super) fn verify_ed25519_signature(
    terms_hash: &str,
    signature_enc: &str,
    public_key_enc: &str,
) -> Result<(), String> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    let pub_bytes: [u8; 32] = decode_ed25519_field(public_key_enc, "public_key", 32)?
        .try_into()
        .map_err(|_| "public_key conversion failed".to_string())?;

    let sig_bytes: [u8; 64] = decode_ed25519_field(signature_enc, "signature", 64)?
        .try_into()
        .map_err(|_| "signature conversion failed".to_string())?;

    let verifying_key = VerifyingKey::from_bytes(&pub_bytes)
        .map_err(|e| format!("Invalid Ed25519 public key: {e}"))?;

    let signature = Signature::from_bytes(&sig_bytes);

    verifying_key
        .verify(terms_hash.as_bytes(), &signature)
        .map_err(|e| format!("Ed25519 signature verification failed: {e}"))
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
