// SPDX-License-Identifier: AGPL-3.0-only

//! Dark Forest Challenge-Response Protocol
//!
//! Handles cryptographic challenge generation and verification for
//! lineage-based authentication between family members.
//!
//! # Handlers
//!
//! - `handle_generate_challenge` - Generate random challenge nonce
//! - `handle_respond_to_challenge` - Respond with HMAC proof using family seed
//! - `handle_verify_challenge_response` - Verify challenge response (constant-time)

use super::*;
use crate::tunnel::hsm::software_hsm::crypto_providers::genetic_crypto::GeneticCryptoProvider;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_errors::BearDogError;
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::{debug, info, warn};

/// # Errors
///
/// Returns an error if serialization fails.
/// Handle `genetic.generate_challenge` RPC method
///
/// Generates a cryptographic challenge for lineage verification.
///
/// # Performance
/// - Expected: < 100μs (secure random nonce generation)
pub async fn handle_generate_challenge(params: &Value) -> Result<Value, BearDogError> {
    debug!("🎲 RPC: genetic.generate_challenge");

    let request: GenerateChallengeRequest =
        GenerateChallengeRequest::deserialize(params).map_err(|e| {
            BearDogError::invalid_input(&format!("Invalid generate_challenge params: {e}"))
        })?;

    let mut nonce = [0u8; 32];
    use rand::RngCore;
    rand::thread_rng().fill_bytes(&mut nonce);
    let nonce_hex = hex::encode(nonce);

    use uuid::Uuid;
    let challenge_id = Uuid::new_v4().to_string();

    info!(
        "✅ Generated challenge: {} → {}",
        request.challenger_node_id, request.target_family_id
    );

    Ok(json!(GenerateChallengeResponse {
        nonce: nonce_hex,
        challenge_id,
        challenger: request.challenger_node_id,
        target: request.target_family_id,
    }))
}

/// # Errors
///
/// Returns an error if serialization fails.
/// Handle `genetic.respond_to_challenge` RPC method
///
/// Responds to a lineage challenge by generating HMAC proof.
///
/// # Performance
/// - Expected: < 500μs (HMAC-SHA512 with lineage key)
pub async fn handle_respond_to_challenge(params: &Value) -> Result<Value, BearDogError> {
    debug!("🔐 RPC: genetic.respond_to_challenge");

    let request: RespondToChallengeRequest = RespondToChallengeRequest::deserialize(params)
        .map_err(|e| {
            BearDogError::invalid_input(&format!("Invalid respond_to_challenge params: {e}"))
        })?;

    let seed_bytes = std::fs::read(&request.our_family_seed_path).map_err(|e| {
        BearDogError::system(format!(
            "Failed to read family seed from {}: {}",
            request.our_family_seed_path, e
        ))
    })?;
    let _seed_b64 = BASE64.encode(&seed_bytes);

    let provider = GeneticCryptoProvider::new_with_lineage(seed_bytes.clone())?;
    let lineage_key = provider
        .derive_lineage_key("family", "responder", b"lineage-challenge-v1")
        .await?;

    let nonce_bytes = hex::decode(&request.nonce)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid nonce (not hex): {e}")))?;

    use hmac::{Hmac, Mac};
    use sha2::Sha512;
    type HmacSha512 = Hmac<Sha512>;

    let mut mac = HmacSha512::new_from_slice(&lineage_key)
        .map_err(|e| BearDogError::system(format!("Failed to create HMAC: {e}")))?;
    mac.update(&nonce_bytes);
    let response_bytes = mac.finalize().into_bytes();
    let response_hex = hex::encode(response_bytes);

    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed_bytes);
    hasher.update(b"family");
    hasher.update(b"responder");
    hasher.update(b"GENETIC_LINEAGE_PROOF_V1");
    let proof = hasher.finalize();
    let proof_b64 = BASE64.encode(proof.as_bytes());

    let seed_hash = blake3::hash(&seed_bytes);
    let prefix_hex = hex::encode(&seed_hash.as_bytes()[..16]);

    info!(
        "✅ Generated challenge response for node: {}",
        request.our_node_id
    );

    Ok(json!(RespondToChallengeResponse {
        response: response_hex,
        lineage_proof: proof_b64,
        seed_hash_prefix: prefix_hex,
        responder_node_id: request.our_node_id,
    }))
}

/// # Errors
///
/// Returns an error if serialization fails.
/// Handle `genetic.verify_challenge_response` RPC method
///
/// Verifies a challenge response for lineage authentication.
/// Uses constant-time comparison for HMAC verification.
///
/// # Performance
/// - Expected: < 600μs (constant-time HMAC comparison + lineage verification)
pub async fn handle_verify_challenge_response(params: &Value) -> Result<Value, BearDogError> {
    debug!("🔍 RPC: genetic.verify_challenge_response");

    let request: VerifyChallengeResponseRequest =
        VerifyChallengeResponseRequest::deserialize(params).map_err(|e| {
            BearDogError::invalid_input(&format!("Invalid verify_challenge_response params: {e}"))
        })?;

    let our_seed_bytes = std::fs::read(&request.our_family_seed_path).map_err(|e| {
        BearDogError::system(format!(
            "Failed to read family seed from {}: {}",
            request.our_family_seed_path, e
        ))
    })?;

    let provider = GeneticCryptoProvider::new_with_lineage(our_seed_bytes.clone())?;
    let lineage_key = provider
        .derive_lineage_key("family", "responder", b"lineage-challenge-v1")
        .await?;

    let nonce_bytes = hex::decode(&request.nonce)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid nonce (not hex): {e}")))?;

    let response_bytes = hex::decode(&request.response)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid response (not hex): {e}")))?;

    use hmac::{Hmac, Mac};
    use sha2::Sha512;
    type HmacSha512 = Hmac<Sha512>;

    let mut mac = HmacSha512::new_from_slice(&lineage_key)
        .map_err(|e| BearDogError::system(format!("Failed to create HMAC: {e}")))?;
    mac.update(&nonce_bytes);
    let expected_bytes = mac.finalize().into_bytes();

    use subtle::ConstantTimeEq;
    let response_valid = response_bytes.ct_eq(&expected_bytes[..]).into();

    let lineage_proof = BASE64.decode(&request.lineage_proof).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid lineage_proof (not base64): {e}"))
    })?;

    let proof_valid = provider
        .verify_lineage("family", "responder", &lineage_proof)
        .await?;

    let valid = response_valid && proof_valid;
    let (relationship, trust_level) = if valid {
        ("verified_sibling", "family")
    } else {
        ("unrelated", "none")
    };

    if valid {
        info!(
            "✅ Challenge response verified: {}",
            request.responder_node_id
        );
    } else {
        warn!(
            "❌ Challenge response FAILED: {}",
            request.responder_node_id
        );
    }

    Ok(json!(VerifyChallengeResponseResponse {
        valid,
        relationship: relationship.to_string(),
        trust_level: trust_level.to_string(),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn generate_challenge_parses_and_returns_hex_nonce() {
        let params = json!({
            "challenger_node_id": "node-a",
            "target_family_id": "fam-1",
        });
        let v = handle_generate_challenge(&params).await.expect("generate");
        let nonce = v.get("nonce").and_then(|x| x.as_str()).expect("nonce");
        assert_eq!(nonce.len(), 64);
        assert!(v.get("challenge_id").and_then(|x| x.as_str()).is_some());
    }

    #[tokio::test]
    async fn generate_challenge_rejects_malformed_params() {
        let err = handle_generate_challenge(&json!("not-an-object"))
            .await
            .unwrap_err();
        let msg = format!("{err}");
        assert!(
            msg.contains("Invalid generate_challenge") || msg.contains("invalid"),
            "{msg}"
        );
    }

    #[tokio::test]
    async fn respond_and_verify_roundtrip_with_temp_seed() {
        let dir = tempfile::tempdir().expect("tempdir");
        let seed_path = dir.path().join("family.seed");
        std::fs::write(&seed_path, b"test-seed-bytes-for-lineage").expect("write seed");

        let gen_params = json!({
            "challenger_node_id": "c1",
            "target_family_id": "fam-x",
        });
        let challenge = handle_generate_challenge(&gen_params)
            .await
            .expect("generate challenge");
        let nonce = challenge["nonce"].as_str().expect("nonce");

        let respond_params = json!({
            "nonce": nonce,
            "our_family_seed_path": seed_path.to_string_lossy(),
            "our_node_id": "responder-1",
        });
        let resp = handle_respond_to_challenge(&respond_params)
            .await
            .expect("respond");

        let verify_params = json!({
            "nonce": nonce,
            "response": resp["response"],
            "responder_node_id": "responder-1",
            "lineage_proof": resp["lineage_proof"],
            "our_family_seed_path": seed_path.to_string_lossy(),
        });
        let verified = handle_verify_challenge_response(&verify_params)
            .await
            .expect("verify");
        assert_eq!(verified["valid"], true);
        assert_eq!(verified["relationship"], "verified_sibling");
    }

    #[tokio::test]
    async fn respond_fails_on_invalid_hex_nonce() {
        let dir = tempfile::tempdir().expect("tempdir");
        let seed_path = dir.path().join("family.seed");
        std::fs::write(&seed_path, b"x").expect("write seed");

        let err = handle_respond_to_challenge(&json!({
            "nonce": "not-hex",
            "our_family_seed_path": seed_path.to_string_lossy(),
            "our_node_id": "n",
        }))
        .await
        .unwrap_err();
        assert!(format!("{err}").contains("hex") || format!("{err}").contains("nonce"));
    }

    #[tokio::test]
    async fn verify_fails_on_tampered_response() {
        let dir = tempfile::tempdir().expect("tempdir");
        let seed_path = dir.path().join("family.seed");
        std::fs::write(&seed_path, b"same-seed").expect("write seed");

        let challenge = handle_generate_challenge(&json!({
            "challenger_node_id": "c",
            "target_family_id": "f",
        }))
        .await
        .expect("gen");
        let nonce = challenge["nonce"].as_str().expect("nonce");

        let resp = handle_respond_to_challenge(&json!({
            "nonce": nonce,
            "our_family_seed_path": seed_path.to_string_lossy(),
            "our_node_id": "r",
        }))
        .await
        .expect("respond");

        let mut bad_hex = resp["response"].as_str().expect("response hex").to_string();
        let last = bad_hex.pop().unwrap();
        bad_hex.push(if last == '0' { '1' } else { '0' });

        let out = handle_verify_challenge_response(&json!({
            "nonce": nonce,
            "response": bad_hex,
            "responder_node_id": "r",
            "lineage_proof": resp["lineage_proof"],
            "our_family_seed_path": seed_path.to_string_lossy(),
        }))
        .await
        .expect("verify");
        assert_eq!(out["valid"], false);
        assert_eq!(out["relationship"], "unrelated");
    }

    #[tokio::test]
    async fn verify_rejects_invalid_lineage_proof_base64() {
        let dir = tempfile::tempdir().expect("tempdir");
        let seed_path = dir.path().join("family.seed");
        std::fs::write(&seed_path, b"seed").expect("write seed");

        let challenge = handle_generate_challenge(&json!({
            "challenger_node_id": "c",
            "target_family_id": "f",
        }))
        .await
        .expect("gen");
        let nonce = challenge["nonce"].as_str().expect("nonce");

        let resp = handle_respond_to_challenge(&json!({
            "nonce": nonce,
            "our_family_seed_path": seed_path.to_string_lossy(),
            "our_node_id": "r",
        }))
        .await
        .expect("respond");

        let err = handle_verify_challenge_response(&json!({
            "nonce": nonce,
            "response": resp["response"],
            "responder_node_id": "r",
            "lineage_proof": "@@@not-base64@@@",
            "our_family_seed_path": seed_path.to_string_lossy(),
        }))
        .await
        .unwrap_err();
        assert!(format!("{err}").contains("base64") || format!("{err}").contains("lineage"));
    }
}
