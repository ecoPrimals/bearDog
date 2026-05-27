// SPDX-License-Identifier: AGPL-3.0-or-later

use super::super::super::MethodHandler;
use super::super::*;
use crate::btsp_provider::BeardogBtspProvider;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use ed25519_dalek::{Signer, SigningKey};
use std::sync::Arc;

/// Generate a real Ed25519 signature over `terms_hash` and return
/// `(signature_base64, public_key_base64)` for use in accept params.
pub(super) fn sign_as_acceptor(terms_hash: &str) -> (String, String) {
    let key = SigningKey::from_bytes(&[0x42; 32]);
    let sig = key.sign(terms_hash.as_bytes());
    (
        BASE64.encode(sig.to_bytes()),
        BASE64.encode(key.verifying_key().as_bytes()),
    )
}

/// Sign terms as a countersigner using a distinct key from the proposer.
pub(super) fn sign_as_countersigner(terms_hash: &str) -> (String, String) {
    let key = SigningKey::from_bytes(&[0x99; 32]);
    let sig = key.sign(terms_hash.as_bytes());
    (
        BASE64.encode(sig.to_bytes()),
        BASE64.encode(key.verifying_key().as_bytes()),
    )
}

/// Helper: propose a bond -> return (`proposal_id`, `terms_hash`).
pub(super) async fn propose_bond(
    handler: &IonicBondHandler,
    provider: &Arc<BeardogBtspProvider>,
    proposer: &str,
    target: &str,
) -> (String, String) {
    let params = serde_json::json!({ "proposer": proposer, "target": target });
    let result = handler
        .handle("crypto.ionic_bond.propose", Some(&params), provider)
        .await
        .expect("propose");
    (
        result["proposal_id"].as_str().unwrap().to_string(),
        result["terms_hash"].as_str().unwrap().to_string(),
    )
}

/// Helper: accept a bond with real Ed25519 signature -> return `bond_id`.
pub(super) async fn accept_bond(
    handler: &IonicBondHandler,
    provider: &Arc<BeardogBtspProvider>,
    proposal_id: &str,
    terms_hash: &str,
    acceptor: &str,
) -> String {
    let (sig, pubkey) = sign_as_acceptor(terms_hash);
    let params = serde_json::json!({
        "proposal_id": proposal_id,
        "acceptor": acceptor,
        "acceptor_signature": sig,
        "acceptor_public_key": pubkey,
    });
    let result = handler
        .handle("crypto.ionic_bond.accept", Some(&params), provider)
        .await
        .expect("accept");
    result["bond"]["bond_id"].as_str().unwrap().to_string()
}
