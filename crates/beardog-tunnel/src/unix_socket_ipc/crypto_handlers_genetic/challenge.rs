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
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use beardog_errors::BearDogError;
use serde_json::{json, Value};
use tracing::{debug, info, warn};

/// Handle `genetic.generate_challenge` RPC method
///
/// Generates a cryptographic challenge for lineage verification.
///
/// # Performance
/// - Expected: < 100μs (secure random nonce generation)
pub async fn handle_generate_challenge(params: Value) -> Result<Value, BearDogError> {
    debug!("🎲 RPC: genetic.generate_challenge");

    let request: GenerateChallengeRequest = serde_json::from_value(params).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid generate_challenge params: {}", e))
    })?;

    let mut nonce = [0u8; 32];
    use rand::RngCore;
    rand::thread_rng().fill_bytes(&mut nonce);
    let nonce_hex = hex::encode(&nonce);

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

/// Handle `genetic.respond_to_challenge` RPC method
///
/// Responds to a lineage challenge by generating HMAC proof.
///
/// # Performance
/// - Expected: < 500μs (HMAC-SHA512 with lineage key)
pub async fn handle_respond_to_challenge(params: Value) -> Result<Value, BearDogError> {
    debug!("🔐 RPC: genetic.respond_to_challenge");

    let request: RespondToChallengeRequest = serde_json::from_value(params).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid respond_to_challenge params: {}", e))
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
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid nonce (not hex): {}", e)))?;

    use hmac::{Hmac, Mac};
    use sha2::Sha512;
    type HmacSha512 = Hmac<Sha512>;

    let mut mac = HmacSha512::new_from_slice(&lineage_key)
        .map_err(|e| BearDogError::system(format!("Failed to create HMAC: {}", e)))?;
    mac.update(&nonce_bytes);
    let response_bytes = mac.finalize().into_bytes();
    let response_hex = hex::encode(&response_bytes);

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

/// Handle `genetic.verify_challenge_response` RPC method
///
/// Verifies a challenge response for lineage authentication.
/// Uses constant-time comparison for HMAC verification.
///
/// # Performance
/// - Expected: < 600μs (constant-time HMAC comparison + lineage verification)
pub async fn handle_verify_challenge_response(params: Value) -> Result<Value, BearDogError> {
    debug!("🔍 RPC: genetic.verify_challenge_response");

    let request: VerifyChallengeResponseRequest = serde_json::from_value(params).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid verify_challenge_response params: {}", e))
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
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid nonce (not hex): {}", e)))?;

    let response_bytes = hex::decode(&request.response)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid response (not hex): {}", e)))?;

    use hmac::{Hmac, Mac};
    use sha2::Sha512;
    type HmacSha512 = Hmac<Sha512>;

    let mut mac = HmacSha512::new_from_slice(&lineage_key)
        .map_err(|e| BearDogError::system(format!("Failed to create HMAC: {}", e)))?;
    mac.update(&nonce_bytes);
    let expected_bytes = mac.finalize().into_bytes();

    use subtle::ConstantTimeEq;
    let response_valid = response_bytes.ct_eq(&expected_bytes[..]).into();

    let lineage_proof = BASE64.decode(&request.lineage_proof).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid lineage_proof (not base64): {}", e))
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
