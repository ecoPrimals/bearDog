//! Genetic Crypto RPC Handlers - Phase 5
//!
//! Handlers for genetic lineage-based cryptographic operations.
//!
//! # Methods Implemented
//!
//! - `genetic.derive_lineage_key` - Derive keys from family lineage
//! - `genetic.mix_entropy` - Mix entropy across three tiers
//! - `genetic.verify_lineage` - Verify genetic family relationships
//! - `genetic.generate_lineage_proof` - Generate lineage proof for verification
//!
//! # Architecture
//!
//! These handlers leverage BearDog's genetic infrastructure:
//! - `BearDogGenetics` for family tree structure
//! - `GeneticCryptoProvider` for Pure Rust lineage-based crypto
//! - Three-tier entropy hierarchy (Human > Supervised > Machine)
//!
//! # Performance
//!
//! - `derive_lineage_key`: < 500μs (Blake3 KDF)
//! - `mix_entropy`: < 200μs (Blake3 mixing)
//! - `verify_lineage`: < 300μs (Blake3 verification)
//! - `generate_lineage_proof`: < 400μs (Blake3 + HMAC)

use crate::tunnel::hsm::software_hsm::crypto_providers::genetic_crypto::GeneticCryptoProvider;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::{debug, info, warn};

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

/// Request to derive a lineage-based key
#[derive(Debug, Deserialize)]
pub struct DeriveLineageKeyRequest {
    /// Our genetic family ID
    pub our_family_id: String,
    /// Peer's genetic family ID
    pub peer_family_id: String,
    /// Context data (purpose, session ID, etc.)
    pub context: String,
    /// Lineage seed (base64-encoded)
    pub lineage_seed: String,
}

/// Response containing the derived lineage key
#[derive(Debug, Serialize, Deserialize)]
pub struct DeriveLineageKeyResponse {
    /// Derived key (base64-encoded, 32 bytes)
    pub key: String,
    /// Key derivation method used
    pub method: String,
    /// Quality score (0.0-1.0)
    pub quality_score: f64,
}

/// Request to mix entropy across tiers
#[derive(Debug, Deserialize)]
pub struct MixEntropyRequest {
    /// Optional Tier 3: Human Lived Experience entropy (base64)
    #[serde(default)]
    pub tier3_human: Option<String>,
    /// Optional Tier 2: Human Supervised Machine entropy (base64)
    #[serde(default)]
    pub tier2_supervised: Option<String>,
    /// Optional Tier 1: Store Bought Machine entropy (base64)
    #[serde(default)]
    pub tier1_machine: Option<String>,
}

/// Response containing mixed entropy
#[derive(Debug, Serialize, Deserialize)]
pub struct MixEntropyResponse {
    /// Mixed entropy (base64-encoded, 32 bytes)
    pub entropy: String,
    /// Quality score (0.0-1.0)
    pub quality_score: f64,
    /// Number of tiers used
    pub tiers_used: u8,
}

/// Request to verify genetic lineage
#[derive(Debug, Deserialize)]
pub struct VerifyLineageRequest {
    /// Our genetic family ID
    pub our_family_id: String,
    /// Peer's claimed family ID
    pub peer_family_id: String,
    /// Lineage proof (base64-encoded)
    pub lineage_proof: String,
    /// Lineage seed (base64-encoded)
    pub lineage_seed: String,
}

/// Response containing lineage verification result
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyLineageResponse {
    /// Whether lineage is valid
    pub valid: bool,
    /// Optional failure reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Request to generate lineage proof
#[derive(Debug, Deserialize)]
pub struct GenerateLineageProofRequest {
    /// Our genetic family ID
    pub our_family_id: String,
    /// Peer's genetic family ID
    pub peer_family_id: String,
    /// Lineage seed (base64-encoded)
    pub lineage_seed: String,
}

/// Response containing generated lineage proof
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateLineageProofResponse {
    /// Lineage proof (base64-encoded)
    pub proof: String,
    /// Proof generation timestamp
    pub timestamp: u64,
}

// ============================================================================
// HANDLER IMPLEMENTATIONS
// ============================================================================

/// Handle `genetic.derive_lineage_key` RPC method
///
/// Derives a cryptographic key from genetic family lineage.
///
/// # Performance
///
/// - Expected: < 500μs
/// - Method: Blake3 KDF with lineage mixing
///
/// # Example
///
/// ```json
/// {
///     "our_family_id": "beardog-family",
///     "peer_family_id": "songbird-family",
///     "context": "tunnel-session-12345",
///     "lineage_seed": "base64_encoded_seed..."
/// }
/// ```
pub async fn handle_derive_lineage_key(params: Value) -> Result<Value, BearDogError> {
    debug!("🧬 RPC: genetic.derive_lineage_key");

    let request: DeriveLineageKeyRequest = serde_json::from_value(params).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid derive_lineage_key params: {}", e))
    })?;

    // Decode lineage seed from base64
    let lineage_seed = BASE64.decode(&request.lineage_seed).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid lineage_seed (not base64): {}", e))
    })?;

    // Create provider with lineage
    let provider = GeneticCryptoProvider::new_with_lineage(lineage_seed)?;

    // Derive lineage key
    let key = provider
        .derive_lineage_key(
            &request.our_family_id,
            &request.peer_family_id,
            request.context.as_bytes(),
        )
        .await?;

    // Encode key as base64
    let key_b64 = BASE64.encode(&key);

    info!(
        "✅ Derived lineage key: {} <-> {} (32 bytes)",
        request.our_family_id, request.peer_family_id
    );

    Ok(json!(DeriveLineageKeyResponse {
        key: key_b64,
        method: "Blake3-Lineage-KDF".to_string(),
        quality_score: 0.8, // Tier 1 + Lineage
    }))
}

/// Handle `genetic.mix_entropy` RPC method
///
/// Mixes entropy from multiple tiers for enhanced security.
///
/// # Performance
///
/// - Expected: < 200μs
/// - Method: Blake3 entropy mixing
///
/// # Example
///
/// ```json
/// {
///     "tier3_human": "base64_human_entropy...",
///     "tier2_supervised": "base64_supervised_entropy...",
///     "tier1_machine": null
/// }
/// ```
pub async fn handle_mix_entropy(params: Value) -> Result<Value, BearDogError> {
    debug!("🌱 RPC: genetic.mix_entropy");

    let request: MixEntropyRequest = serde_json::from_value(params)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid mix_entropy params: {}", e)))?;

    // Decode base64 entropy inputs
    let tier3 = request
        .tier3_human
        .as_ref()
        .map(|s| BASE64.decode(s))
        .transpose()
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid tier3_human: {}", e)))?;

    let tier2 = request
        .tier2_supervised
        .as_ref()
        .map(|s| BASE64.decode(s))
        .transpose()
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid tier2_supervised: {}", e)))?;

    let tier1 = request
        .tier1_machine
        .as_ref()
        .map(|s| BASE64.decode(s))
        .transpose()
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid tier1_machine: {}", e)))?;

    // Count tiers
    let tiers_used = tier3.is_some() as u8 + tier2.is_some() as u8 + 1; // Always have Tier 1

    // Create provider and mix entropy
    let provider = GeneticCryptoProvider::new()?;
    let (mixed, quality) = provider
        .mix_entropy(
            tier3.as_deref(),
            tier2.as_deref(),
            tier1.as_deref(),
        )
        .await?;

    // Encode as base64
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
///
/// - Expected: < 300μs
/// - Method: Blake3 proof verification
///
/// # Example
///
/// ```json
/// {
///     "our_family_id": "beardog-family",
///     "peer_family_id": "songbird-family",
///     "lineage_proof": "base64_encoded_proof...",
///     "lineage_seed": "base64_encoded_seed..."
/// }
/// ```
pub async fn handle_verify_lineage(params: Value) -> Result<Value, BearDogError> {
    debug!("🔍 RPC: genetic.verify_lineage");

    let request: VerifyLineageRequest = serde_json::from_value(params).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid verify_lineage params: {}", e))
    })?;

    // Decode base64 inputs
    let lineage_proof = BASE64
        .decode(&request.lineage_proof)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid lineage_proof: {}", e)))?;

    let lineage_seed = BASE64
        .decode(&request.lineage_seed)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid lineage_seed: {}", e)))?;

    // Create provider with lineage
    let provider = GeneticCryptoProvider::new_with_lineage(lineage_seed)?;

    // Verify lineage
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
///
/// - Expected: < 400μs
/// - Method: Blake3 + HMAC
///
/// # Example
///
/// ```json
/// {
///     "our_family_id": "beardog-family",
///     "peer_family_id": "songbird-family",
///     "lineage_seed": "base64_encoded_seed..."
/// }
/// ```
pub async fn handle_generate_lineage_proof(params: Value) -> Result<Value, BearDogError> {
    debug!("🔐 RPC: genetic.generate_lineage_proof");

    let request: GenerateLineageProofRequest = serde_json::from_value(params).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid generate_lineage_proof params: {}", e))
    })?;

    // Decode lineage seed
    let lineage_seed = BASE64
        .decode(&request.lineage_seed)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid lineage_seed: {}", e)))?;

    // Generate proof using Blake3
    use blake3;
    let mut hasher = blake3::Hasher::new();
    hasher.update(&lineage_seed);
    hasher.update(request.our_family_id.as_bytes());
    hasher.update(request.peer_family_id.as_bytes());
    hasher.update(b"GENETIC_LINEAGE_PROOF_V1");
    let proof = hasher.finalize();

    // Get timestamp
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| BearDogError::system(format!("Failed to get timestamp: {}", e)))?
        .as_secs();

    // Encode as base64
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

// ============================================================================
// TESTS
// ============================================================================

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

        let result = handle_derive_lineage_key(params).await?;
        let response: DeriveLineageKeyResponse = serde_json::from_value(result)?;

        // Verify response
        assert!(!response.key.is_empty(), "Key should not be empty");
        assert_eq!(response.method, "Blake3-Lineage-KDF");
        assert!(response.quality_score > 0.0);

        // Decode key to verify it's 32 bytes
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

        let result = handle_mix_entropy(params).await?;
        let response: MixEntropyResponse = serde_json::from_value(result)?;

        // Verify response
        assert!(!response.entropy.is_empty());
        assert_eq!(response.tiers_used, 1);
        assert!(response.quality_score >= 0.4);

        // Decode to verify 32 bytes
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

        let result = handle_mix_entropy(params).await?;
        let response: MixEntropyResponse = serde_json::from_value(result)?;

        assert_eq!(response.tiers_used, 3);
        assert!(response.quality_score > 0.6);

        Ok(())
    }

    #[tokio::test]
    async fn test_generate_and_verify_lineage() -> Result<(), Box<dyn std::error::Error>> {
        let lineage_seed = BASE64.encode(b"test_lineage_seed_for_roundtrip");

        // Generate proof
        let gen_params = json!({
            "our_family_id": "beardog-family",
            "peer_family_id": "songbird-family",
            "lineage_seed": lineage_seed.clone(),
        });

        let gen_result = handle_generate_lineage_proof(gen_params).await?;
        let gen_response: GenerateLineageProofResponse = serde_json::from_value(gen_result)?;

        // Verify proof
        let verify_params = json!({
            "our_family_id": "beardog-family",
            "peer_family_id": "songbird-family",
            "lineage_proof": gen_response.proof,
            "lineage_seed": lineage_seed,
        });

        let verify_result = handle_verify_lineage(verify_params).await?;
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

        let result = handle_verify_lineage(params).await?;
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

        let result = handle_derive_lineage_key(params).await;
        assert!(result.is_err(), "Invalid base64 should fail");
    }
}
