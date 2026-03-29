// SPDX-License-Identifier: AGPL-3.0-only

//! Lineage Key Derivation, Beacon Keys, Verification, and Entropy Mixing
//!
//! Handles genetic lineage-based key derivation, proof generation/verification,
//! Dark Forest beacon key derivation, and entropy tier mixing.
//!
//! # Handlers
//!
//! - `handle_derive_lineage_key` - Derive keys from family lineage (Blake3 KDF)
//! - `handle_derive_lineage_beacon_key` - Derive BirdSong beacon key (HKDF-SHA256)
//! - `handle_mix_entropy` - Mix entropy across three tiers
//! - `handle_verify_lineage` - Verify genetic family relationships
//! - `handle_generate_lineage_proof` - Generate lineage proof (Blake3 + HMAC)

use super::*;
use crate::tunnel::hsm::software_hsm::crypto_providers::genetic_crypto::GeneticCryptoProvider;
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_errors::BearDogError;
use hkdf::Hkdf;
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::Sha256;
use tracing::{debug, info, warn};

/// Handle `genetic.derive_lineage_key` RPC method
///
/// Derives a cryptographic key from genetic family lineage.
///
/// # Performance
/// - Expected: < 500μs (Blake3 KDF with lineage mixing)
pub async fn handle_derive_lineage_key(params: &Value) -> Result<Value, BearDogError> {
    debug!("🧬 RPC: genetic.derive_lineage_key");

    let request: DeriveLineageKeyRequest =
        DeriveLineageKeyRequest::deserialize(params).map_err(|e| {
            BearDogError::invalid_input(&format!("Invalid derive_lineage_key params: {e}"))
        })?;

    let lineage_seed = BASE64.decode(&request.lineage_seed).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid lineage_seed (not base64): {e}"))
    })?;

    let provider = GeneticCryptoProvider::new_with_lineage(lineage_seed)?;
    let key = provider
        .derive_lineage_key(
            &request.our_family_id,
            &request.peer_family_id,
            request.context.as_bytes(),
        )
        .await?;

    let key_b64 = BASE64.encode(&key);

    info!(
        "✅ Derived lineage key: {} <-> {} (32 bytes)",
        request.our_family_id, request.peer_family_id
    );

    Ok(json!(DeriveLineageKeyResponse {
        key: key_b64,
        method: "Blake3-Lineage-KDF".to_string(),
        quality_score: 0.8,
    }))
}

/// Handle `genetic.derive_lineage_beacon_key` RPC method
///
/// Derives a dedicated beacon encryption key from family lineage.
/// Uses HKDF-SHA256 with domain separation ("birdsong_beacon_v1").
/// All family members with the same lineage seed derive the SAME key.
///
/// # Performance
/// - Expected: < 100μs (HKDF-SHA256)
pub async fn handle_derive_lineage_beacon_key(params: &Value) -> Result<Value, BearDogError> {
    debug!("🌑 RPC: genetic.derive_lineage_beacon_key (TRUE Dark Forest)");

    let lineage_seed_b64 = params
        .get("lineage_seed")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    if lineage_seed_b64.is_empty() {
        return Err(BearDogError::invalid_input(
            "lineage_seed is required (empty seed would produce a predictable key)",
        ));
    }

    let lineage_seed = BASE64.decode(lineage_seed_b64).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid lineage_seed (not base64): {e}"))
    })?;

    if lineage_seed.len() < 16 {
        return Err(BearDogError::invalid_input(&format!(
            "lineage_seed too short: {} bytes (minimum 16)",
            lineage_seed.len()
        )));
    }

    if lineage_seed.iter().all(|&b| b == 0) {
        return Err(BearDogError::invalid_input(
            "lineage_seed is all zeros (would produce a predictable key)",
        ));
    }

    let domain = b"birdsong_beacon_v1";
    let mut okm = [0u8; 32];

    let hkdf = Hkdf::<Sha256>::new(None, &lineage_seed);
    hkdf.expand(domain, &mut okm)
        .map_err(|e| BearDogError::system(format!("HKDF beacon key derivation failed: {e}")))?;

    let beacon_key_hex = hex::encode(okm);

    info!("✅ Derived BirdSong beacon key: 32 bytes (HKDF-SHA256, domain-separated)");
    debug!("   Domain: birdsong_beacon_v1, Deterministic: true, Algorithm: ChaCha20-Poly1305");

    Ok(json!({
        "beacon_key": beacon_key_hex,
        "algorithm": "HKDF-SHA256+ChaCha20-Poly1305",
        "domain": "birdsong_beacon_v1",
        "key_size_bytes": 32,
        "deterministic": true,
        "purpose": "TRUE Dark Forest beacon encryption (zero metadata)"
    }))
}

/// Handle `genetic.mix_entropy` RPC method
///
/// Mixes entropy from multiple tiers for enhanced security.
///
/// # Performance
/// - Expected: < 200μs (Blake3 entropy mixing)
pub async fn handle_mix_entropy(params: &Value) -> Result<Value, BearDogError> {
    debug!("🌱 RPC: genetic.mix_entropy");

    let request: MixEntropyRequest = MixEntropyRequest::deserialize(params)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid mix_entropy params: {e}")))?;

    let tier3 = request
        .tier3_human
        .as_ref()
        .map(|s| BASE64.decode(s))
        .transpose()
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid tier3_human: {e}")))?;

    let tier2 = request
        .tier2_supervised
        .as_ref()
        .map(|s| BASE64.decode(s))
        .transpose()
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid tier2_supervised: {e}")))?;

    let tier1 = request
        .tier1_machine
        .as_ref()
        .map(|s| BASE64.decode(s))
        .transpose()
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid tier1_machine: {e}")))?;

    let tiers_used = u8::from(tier3.is_some()) + u8::from(tier2.is_some()) + 1;

    let provider = GeneticCryptoProvider::new()?;
    let (mixed, quality) = provider
        .mix_entropy(tier3.as_deref(), tier2.as_deref(), tier1.as_deref())
        .await?;

    let entropy_b64 = BASE64.encode(&mixed);

    info!(
        "✅ Mixed entropy: {} tiers, quality {:.2}",
        tiers_used, quality
    );

    Ok(json!(MixEntropyResponse {
        entropy: entropy_b64,
        quality_score: quality,
        tiers_used,
    }))
}

/// Handle `genetic.verify_lineage` RPC method
///
/// Verifies genetic family lineage relationship.
///
/// # Performance
/// - Expected: < 300μs (Blake3 proof verification)
pub async fn handle_verify_lineage(params: &Value) -> Result<Value, BearDogError> {
    debug!("🔍 RPC: genetic.verify_lineage");

    let request: VerifyLineageRequest = VerifyLineageRequest::deserialize(params)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid verify_lineage params: {e}")))?;

    let lineage_proof = BASE64
        .decode(&request.lineage_proof)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid lineage_proof: {e}")))?;

    let lineage_seed = BASE64
        .decode(&request.lineage_seed)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid lineage_seed: {e}")))?;

    let provider = GeneticCryptoProvider::new_with_lineage(lineage_seed)?;
    let is_valid = provider
        .verify_lineage(
            &request.our_family_id,
            &request.peer_family_id,
            &lineage_proof,
        )
        .await?;

    let response = if is_valid {
        info!(
            "✅ Lineage verified: {} <-> {}",
            request.our_family_id, request.peer_family_id
        );
        VerifyLineageResponse {
            valid: true,
            reason: None,
        }
    } else {
        warn!(
            "❌ Lineage verification failed: {} <-> {}",
            request.our_family_id, request.peer_family_id
        );
        VerifyLineageResponse {
            valid: false,
            reason: Some("Lineage proof verification failed".to_string()),
        }
    };

    Ok(json!(response))
}

/// Handle `genetic.generate_lineage_proof` RPC method
///
/// Generates a cryptographic proof of genetic lineage.
///
/// # Performance
/// - Expected: < 400μs (Blake3 + HMAC)
pub async fn handle_generate_lineage_proof(params: &Value) -> Result<Value, BearDogError> {
    debug!("🔐 RPC: genetic.generate_lineage_proof");

    let request: GenerateLineageProofRequest = GenerateLineageProofRequest::deserialize(params)
        .map_err(|e| {
            BearDogError::invalid_input(&format!("Invalid generate_lineage_proof params: {e}"))
        })?;

    let lineage_seed = BASE64
        .decode(&request.lineage_seed)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid lineage_seed: {e}")))?;

    let mut hasher = blake3::Hasher::new();
    hasher.update(&lineage_seed);
    hasher.update(request.our_family_id.as_bytes());
    hasher.update(request.peer_family_id.as_bytes());
    hasher.update(b"GENETIC_LINEAGE_PROOF_V1");
    let proof = hasher.finalize();

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| BearDogError::system(format!("Failed to get timestamp: {e}")))?
        .as_secs();

    let proof_b64 = BASE64.encode(proof.as_bytes());

    info!(
        "✅ Generated lineage proof: {} <-> {}",
        request.our_family_id, request.peer_family_id
    );

    Ok(json!(GenerateLineageProofResponse {
        proof: proof_b64,
        timestamp,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_derive_lineage_key_roundtrip() -> Result<(), Box<dyn std::error::Error>> {
        let lineage_seed = BASE64.encode(b"test_lineage_seed_32bytes_long!!");
        let params = json!({
            "our_family_id": "beardog-family",
            "peer_family_id": "songbird-family",
            "context": "test-session",
            "lineage_seed": lineage_seed,
        });

        let result = handle_derive_lineage_key(&params).await?;
        let response: DeriveLineageKeyResponse = serde_json::from_value(result)?;

        assert!(!response.key.is_empty(), "Key should not be empty");
        assert_eq!(response.method, "Blake3-Lineage-KDF");
        assert!(response.quality_score > 0.0);

        let key_bytes = BASE64.decode(&response.key)?;
        assert_eq!(key_bytes.len(), 32, "Key should be 32 bytes");

        Ok(())
    }

    #[tokio::test]
    async fn test_mix_entropy_tier1_only() -> Result<(), Box<dyn std::error::Error>> {
        let params = json!({
            "tier3_human": null,
            "tier2_supervised": null,
            "tier1_machine": null,
        });

        let result = handle_mix_entropy(&params).await?;
        let response: MixEntropyResponse = serde_json::from_value(result)?;

        assert!(!response.entropy.is_empty());
        assert_eq!(response.tiers_used, 1);
        assert!(response.quality_score >= 0.4);

        let entropy_bytes = BASE64.decode(&response.entropy)?;
        assert_eq!(entropy_bytes.len(), 32);

        Ok(())
    }

    #[tokio::test]
    async fn test_mix_entropy_all_tiers() -> Result<(), Box<dyn std::error::Error>> {
        let tier3 = BASE64.encode(b"human_lived_experience_entropy!");
        let tier2 = BASE64.encode(b"human_supervised_entropy_data");
        let tier1 = BASE64.encode(b"machine_generated_entropy_val");

        let params = json!({
            "tier3_human": tier3,
            "tier2_supervised": tier2,
            "tier1_machine": tier1,
        });

        let result = handle_mix_entropy(&params).await?;
        let response: MixEntropyResponse = serde_json::from_value(result)?;

        assert_eq!(response.tiers_used, 3);
        assert!(response.quality_score > 0.6);

        Ok(())
    }

    #[tokio::test]
    async fn test_generate_and_verify_lineage() -> Result<(), Box<dyn std::error::Error>> {
        let lineage_seed = BASE64.encode(b"test_lineage_seed_for_roundtrip");

        let gen_params = json!({
            "our_family_id": "beardog-family",
            "peer_family_id": "songbird-family",
            "lineage_seed": lineage_seed.clone(),
        });

        let gen_result = handle_generate_lineage_proof(&gen_params).await?;
        let gen_response: GenerateLineageProofResponse = serde_json::from_value(gen_result)?;

        let verify_params = json!({
            "our_family_id": "beardog-family",
            "peer_family_id": "songbird-family",
            "lineage_proof": gen_response.proof,
            "lineage_seed": lineage_seed,
        });

        let verify_result = handle_verify_lineage(&verify_params).await?;
        let verify_response: VerifyLineageResponse = serde_json::from_value(verify_result)?;

        assert!(verify_response.valid, "Lineage should verify");
        assert!(verify_response.reason.is_none());

        Ok(())
    }

    #[tokio::test]
    async fn test_verify_lineage_invalid_proof() -> Result<(), Box<dyn std::error::Error>> {
        let lineage_seed = BASE64.encode(b"test_lineage_seed_for_invalid_");
        let invalid_proof = BASE64.encode(b"this_is_not_a_valid_proof_data");

        let params = json!({
            "our_family_id": "beardog-family",
            "peer_family_id": "songbird-family",
            "lineage_proof": invalid_proof,
            "lineage_seed": lineage_seed,
        });

        let result = handle_verify_lineage(&params).await?;
        let response: VerifyLineageResponse = serde_json::from_value(result)?;

        assert!(!response.valid, "Invalid proof should not verify");
        assert!(response.reason.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_lineage_key_invalid_base64() {
        let params = json!({
            "our_family_id": "beardog-family",
            "peer_family_id": "songbird-family",
            "context": "test",
            "lineage_seed": "not-valid-base64!!!",
        });

        let result = handle_derive_lineage_key(&params).await;
        assert!(result.is_err(), "Invalid base64 should fail");
    }

    #[tokio::test]
    async fn test_derive_lineage_beacon_key() -> Result<(), Box<dyn std::error::Error>> {
        let lineage_seed = BASE64.encode(b"test_lineage_seed_for_beacons");
        let params = json!({ "lineage_seed": lineage_seed });

        let result = handle_derive_lineage_beacon_key(&params).await?;

        assert!(result.get("beacon_key").is_some());
        assert_eq!(
            result.get("algorithm").and_then(|v| v.as_str()),
            Some("HKDF-SHA256+ChaCha20-Poly1305")
        );
        assert_eq!(
            result.get("domain").and_then(|v| v.as_str()),
            Some("birdsong_beacon_v1")
        );
        assert_eq!(
            result.get("key_size_bytes").and_then(|v| v.as_u64()),
            Some(32)
        );
        assert_eq!(
            result.get("deterministic").and_then(|v| v.as_bool()),
            Some(true)
        );

        let key_hex = result
            .get("beacon_key")
            .and_then(|v| v.as_str())
            .expect("beacon_key hex in test");
        let key_bytes = hex::decode(key_hex)?;
        assert_eq!(key_bytes.len(), 32);

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_lineage_beacon_key_deterministic() -> Result<(), Box<dyn std::error::Error>>
    {
        let lineage_seed = BASE64.encode(b"deterministic_test_seed_12345");
        let params = json!({ "lineage_seed": lineage_seed });

        let result1 = handle_derive_lineage_beacon_key(&params).await?;
        let result2 = handle_derive_lineage_beacon_key(&params).await?;

        let key1 = result1
            .get("beacon_key")
            .and_then(|v| v.as_str())
            .expect("beacon_key hex in test");
        let key2 = result2
            .get("beacon_key")
            .and_then(|v| v.as_str())
            .expect("beacon_key hex in test");

        assert_eq!(
            key1, key2,
            "Same lineage seed should produce identical keys"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_lineage_beacon_key_different_seeds()
    -> Result<(), Box<dyn std::error::Error>> {
        let seed1 = BASE64.encode(b"family_alpha_seed_value_here_");
        let seed2 = BASE64.encode(b"family_beta_seed_different!!!");

        let result1 = handle_derive_lineage_beacon_key(&json!({ "lineage_seed": seed1 })).await?;
        let result2 = handle_derive_lineage_beacon_key(&json!({ "lineage_seed": seed2 })).await?;

        let key1 = result1
            .get("beacon_key")
            .and_then(|v| v.as_str())
            .expect("beacon_key hex in test");
        let key2 = result2
            .get("beacon_key")
            .and_then(|v| v.as_str())
            .expect("beacon_key hex in test");

        assert_ne!(
            key1, key2,
            "Different lineage seeds should produce different keys"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_lineage_beacon_key_empty_params_rejected() {
        let result = handle_derive_lineage_beacon_key(&json!({})).await;
        assert!(result.is_err(), "Empty lineage_seed should be rejected");
    }

    #[tokio::test]
    async fn test_derive_lineage_beacon_key_zero_seed_rejected() {
        let zero_seed = BASE64.encode([0u8; 32]);
        let result = handle_derive_lineage_beacon_key(&json!({ "lineage_seed": zero_seed })).await;
        assert!(result.is_err(), "All-zero lineage_seed should be rejected");
    }

    #[tokio::test]
    async fn test_derive_lineage_beacon_key_short_seed_rejected() {
        let short_seed = BASE64.encode([1u8; 8]);
        let result = handle_derive_lineage_beacon_key(&json!({ "lineage_seed": short_seed })).await;
        assert!(result.is_err(), "Short lineage_seed should be rejected");
    }
}
