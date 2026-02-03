//! Genetic Crypto RPC Handlers - Phase 5
//!
//! Handlers for genetic lineage-based cryptographic operations.
//!
//! # Methods Implemented
//!
//! - `genetic.derive_lineage_key` - Derive keys from family lineage
//! - `genetic.derive_lineage_beacon_key` - Derive BirdSong beacon key (TRUE Dark Forest)
//! - `genetic.mix_entropy` - Mix entropy across three tiers
//! - `genetic.verify_lineage` - Verify genetic family relationships
//! - `genetic.generate_lineage_proof` - Generate lineage proof for verification
//! - `genetic.generate_challenge` - Generate challenge for lineage verification
//! - `genetic.respond_to_challenge` - Respond to lineage challenge
//! - `genetic.verify_challenge_response` - Verify challenge response
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
use subtle::ConstantTimeEq;  // For constant-time comparisons
use hkdf::Hkdf;
use sha2::Sha256;

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

/// Request to generate a challenge
#[derive(Debug, Deserialize)]
pub struct GenerateChallengeRequest {
    /// Challenger node ID
    pub challenger_node_id: String,
    /// Target family ID
    pub target_family_id: String,
}

/// Response containing challenge
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateChallengeResponse {
    /// Challenge nonce (hex-encoded, 32 bytes)
    pub nonce: String,
    /// Challenge ID
    pub challenge_id: String,
    /// Challenger node ID
    pub challenger: String,
    /// Target family ID
    pub target: String,
}

/// Request to respond to challenge
#[derive(Debug, Deserialize)]
pub struct RespondToChallengeRequest {
    /// Challenge nonce (hex-encoded)
    pub nonce: String,
    /// Our family seed path
    pub our_family_seed_path: String,
    /// Our node ID
    pub our_node_id: String,
}

/// Response containing challenge response
#[derive(Debug, Serialize, Deserialize)]
pub struct RespondToChallengeResponse {
    /// Challenge response (hex-encoded HMAC-SHA512)
    pub response: String,
    /// Lineage proof
    pub lineage_proof: String,
    /// Seed hash prefix (hex-encoded, 16 bytes)
    pub seed_hash_prefix: String,
    /// Responder node ID
    pub responder_node_id: String,
}

/// Request to verify challenge response
#[derive(Debug, Deserialize)]
pub struct VerifyChallengeResponseRequest {
    /// Challenge nonce (hex-encoded)
    pub nonce: String,
    /// Response to verify (hex-encoded)
    pub response: String,
    /// Responder node ID
    pub responder_node_id: String,
    /// Lineage proof
    pub lineage_proof: String,
    /// Our family seed path
    pub our_family_seed_path: String,
}

/// Response containing verification result
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyChallengeResponseResponse {
    /// Whether response is valid
    pub valid: bool,
    /// Relationship type
    pub relationship: String,
    /// Trust level
    pub trust_level: String,
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

/// Handle `genetic.derive_lineage_beacon_key` RPC method
///
/// Derives a dedicated beacon encryption key from family lineage.
/// This key is used for BirdSong beacons (TRUE Dark Forest - pure noise).
///
/// # Domain Separation
///
/// This method uses domain separation ("birdsong_beacon_v1") to ensure
/// beacon keys are cryptographically distinct from other lineage keys.
///
/// # Determinism
///
/// All family members with the same lineage seed derive the SAME key.
/// This enables family-only beacon decryption (zero metadata leaks).
///
/// # Performance
///
/// - Expected: < 100μs
/// - Method: HKDF-SHA256 with domain separation
///
/// # Security
///
/// - Key Size: 32 bytes (256 bits for ChaCha20-Poly1305)
/// - Algorithm: HKDF-SHA256
/// - Domain: "birdsong_beacon_v1"
/// - Output: Hex-encoded for JSON-RPC
///
/// # Example
///
/// ```json
/// {
///     "lineage_seed": "base64_encoded_seed..."
/// }
/// ```
pub async fn handle_derive_lineage_beacon_key(params: Value) -> Result<Value, BearDogError> {
    debug!("🌑 RPC: genetic.derive_lineage_beacon_key (TRUE Dark Forest)");

    // Extract lineage_seed parameter (optional, will use empty if not provided)
    let lineage_seed_b64 = params
        .get("lineage_seed")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Decode lineage seed from base64 (or use empty seed for testing)
    let lineage_seed = if !lineage_seed_b64.is_empty() {
        BASE64.decode(lineage_seed_b64).map_err(|e| {
            BearDogError::invalid_input(&format!("Invalid lineage_seed (not base64): {}", e))
        })?
    } else {
        // Generate deterministic fallback seed (for testing without family setup)
        vec![0u8; 32]
    };

    // Domain separation for beacon keys (distinct from other genetic keys)
    let domain = b"birdsong_beacon_v1";

    // HKDF-SHA256 key derivation
    // IKM: lineage_seed
    // Salt: None (lineage seed is already high-entropy)
    // Info: domain (for separation)
    let mut okm = [0u8; 32]; // 256 bits for ChaCha20-Poly1305

    let hkdf = Hkdf::<Sha256>::new(None, &lineage_seed);

    hkdf.expand(domain, &mut okm).map_err(|e| {
        BearDogError::system(format!("HKDF beacon key derivation failed: {}", e))
    })?;

    // Encode as hex string for JSON-RPC
    let beacon_key_hex = hex::encode(&okm);

    info!(
        "✅ Derived BirdSong beacon key: 32 bytes (HKDF-SHA256, domain-separated)"
    );
    debug!(
        "   Domain: birdsong_beacon_v1, Deterministic: true, Algorithm: ChaCha20-Poly1305"
    );

    Ok(json!({
        "beacon_key": beacon_key_hex,
        "algorithm": "HKDF-SHA256+ChaCha20-Poly1305",
        "domain": "birdsong_beacon_v1",
        "key_size_bytes": 32,
        "deterministic": true,  // Same lineage = same key
        "purpose": "TRUE Dark Forest beacon encryption (zero metadata)"
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

/// Handle `genetic.generate_challenge` RPC method
///
/// Generates a cryptographic challenge for lineage verification.
///
/// # Performance
///
/// - Expected: < 100μs
/// - Method: Secure random nonce generation
///
/// # Example
///
/// ```json
/// {
///     "challenger_node_id": "usb_node1",
///     "target_family_id": "pixel_tower"
/// }
/// ```
pub async fn handle_generate_challenge(params: Value) -> Result<Value, BearDogError> {
    debug!("🎲 RPC: genetic.generate_challenge");

    let request: GenerateChallengeRequest = serde_json::from_value(params).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid generate_challenge params: {}", e))
    })?;

    // Generate 32-byte nonce
    let mut nonce = [0u8; 32];
    use rand::RngCore;
    rand::thread_rng().fill_bytes(&mut nonce);
    let nonce_hex = hex::encode(&nonce);

    // Generate challenge ID
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
///
/// - Expected: < 500μs
/// - Method: HMAC-SHA512 with lineage key
///
/// # Example
///
/// ```json
/// {
///     "nonce": "hex_encoded_nonce...",
///     "our_family_seed_path": "/path/to/.family.seed",
///     "our_node_id": "pixel_node1"
/// }
/// ```
pub async fn handle_respond_to_challenge(params: Value) -> Result<Value, BearDogError> {
    debug!("🔐 RPC: genetic.respond_to_challenge");

    let request: RespondToChallengeRequest = serde_json::from_value(params).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid respond_to_challenge params: {}", e))
    })?;

    // Read family seed
    let seed_bytes = std::fs::read(&request.our_family_seed_path).map_err(|e| {
        BearDogError::system(format!(
            "Failed to read family seed from {}: {}",
            request.our_family_seed_path, e
        ))
    })?;
    let _seed_b64 = BASE64.encode(&seed_bytes);

    // Derive lineage key
    // NOTE: Role is "responder" to match verifier expectations
    let provider = GeneticCryptoProvider::new_with_lineage(seed_bytes.clone())?;
    let lineage_key = provider
        .derive_lineage_key("family", "responder", b"lineage-challenge-v1")
        .await?;

    // Decode nonce
    let nonce_bytes = hex::decode(&request.nonce).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid nonce (not hex): {}", e))
    })?;

    // Compute HMAC-SHA512(nonce, lineage_key)
    use hmac::{Hmac, Mac};
    use sha2::Sha512;
    type HmacSha512 = Hmac<Sha512>;

    let mut mac = HmacSha512::new_from_slice(&lineage_key).map_err(|e| {
        BearDogError::system(format!("Failed to create HMAC: {}", e))
    })?;
    mac.update(&nonce_bytes);
    let response_bytes = mac.finalize().into_bytes();
    let response_hex = hex::encode(&response_bytes);

    // Generate lineage proof
    // NOTE: Role is "responder" to match verifier expectations
    let mut hasher = blake3::Hasher::new();
    hasher.update(&seed_bytes);
    hasher.update(b"family");
    hasher.update(b"responder");
    hasher.update(b"GENETIC_LINEAGE_PROOF_V1");
    let proof = hasher.finalize();
    let proof_b64 = BASE64.encode(proof.as_bytes());

    // Generate seed hash prefix (16 bytes)
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
///
/// # Performance
///
/// - Expected: < 600μs
/// - Method: Constant-time HMAC comparison + lineage verification
///
/// # Example
///
/// ```json
/// {
///     "nonce": "hex_encoded_nonce...",
///     "response": "hex_encoded_response...",
///     "responder_node_id": "pixel_node1",
///     "lineage_proof": "base64_proof...",
///     "our_family_seed_path": "/path/to/.family.seed"
/// }
/// ```
pub async fn handle_verify_challenge_response(params: Value) -> Result<Value, BearDogError> {
    debug!("🔍 RPC: genetic.verify_challenge_response");

    let request: VerifyChallengeResponseRequest = serde_json::from_value(params).map_err(|e| {
        BearDogError::invalid_input(&format!(
            "Invalid verify_challenge_response params: {}",
            e
        ))
    })?;

    // Read our family seed
    let our_seed_bytes = std::fs::read(&request.our_family_seed_path).map_err(|e| {
        BearDogError::system(format!(
            "Failed to read family seed from {}: {}",
            request.our_family_seed_path, e
        ))
    })?;

    // Derive our lineage key
    let provider = GeneticCryptoProvider::new_with_lineage(our_seed_bytes.clone())?;
    let lineage_key = provider
        .derive_lineage_key("family", "responder", b"lineage-challenge-v1")
        .await?;

    // Decode nonce and response
    let nonce_bytes = hex::decode(&request.nonce).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid nonce (not hex): {}", e))
    })?;

    let response_bytes = hex::decode(&request.response).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid response (not hex): {}", e))
    })?;

    // Compute expected response
    use hmac::{Hmac, Mac};
    use sha2::Sha512;
    type HmacSha512 = Hmac<Sha512>;

    let mut mac = HmacSha512::new_from_slice(&lineage_key).map_err(|e| {
        BearDogError::system(format!("Failed to create HMAC: {}", e))
    })?;
    mac.update(&nonce_bytes);
    let expected_bytes = mac.finalize().into_bytes();

    // Constant-time comparison
    use subtle::ConstantTimeEq;
    let response_valid = response_bytes.ct_eq(&expected_bytes[..]).into();

    // Verify lineage proof
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

    #[tokio::test]
    async fn test_derive_lineage_beacon_key() -> Result<(), Box<dyn std::error::Error>> {
        // Test with a valid lineage seed
        let lineage_seed = BASE64.encode(b"test_lineage_seed_for_beacons");
        let params = json!({
            "lineage_seed": lineage_seed,
        });

        let result = handle_derive_lineage_beacon_key(params).await?;

        // Verify response structure
        assert!(result.get("beacon_key").is_some(), "Should have beacon_key");
        assert_eq!(
            result.get("algorithm").and_then(|v| v.as_str()),
            Some("HKDF-SHA256+ChaCha20-Poly1305")
        );
        assert_eq!(
            result.get("domain").and_then(|v| v.as_str()),
            Some("birdsong_beacon_v1")
        );
        assert_eq!(result.get("key_size_bytes").and_then(|v| v.as_u64()), Some(32));
        assert_eq!(
            result.get("deterministic").and_then(|v| v.as_bool()),
            Some(true)
        );

        // Decode and verify key is 32 bytes
        let beacon_key_hex = result
            .get("beacon_key")
            .and_then(|v| v.as_str())
            .expect("beacon_key should be present");
        let key_bytes = hex::decode(beacon_key_hex)?;
        assert_eq!(key_bytes.len(), 32, "Beacon key should be 32 bytes");

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_lineage_beacon_key_deterministic(
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Same lineage seed should produce same key (deterministic)
        let lineage_seed = BASE64.encode(b"deterministic_test_seed_12345");
        let params = json!({
            "lineage_seed": lineage_seed,
        });

        // Derive key twice
        let result1 = handle_derive_lineage_beacon_key(params.clone()).await?;
        let result2 = handle_derive_lineage_beacon_key(params).await?;

        let key1 = result1.get("beacon_key").and_then(|v| v.as_str()).unwrap();
        let key2 = result2.get("beacon_key").and_then(|v| v.as_str()).unwrap();

        assert_eq!(
            key1, key2,
            "Same lineage seed should produce identical keys"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_lineage_beacon_key_different_seeds(
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Different lineage seeds should produce different keys
        let seed1 = BASE64.encode(b"family_alpha_seed_value_here_");
        let seed2 = BASE64.encode(b"family_beta_seed_different!!!");

        let params1 = json!({ "lineage_seed": seed1 });
        let params2 = json!({ "lineage_seed": seed2 });

        let result1 = handle_derive_lineage_beacon_key(params1).await?;
        let result2 = handle_derive_lineage_beacon_key(params2).await?;

        let key1 = result1.get("beacon_key").and_then(|v| v.as_str()).unwrap();
        let key2 = result2.get("beacon_key").and_then(|v| v.as_str()).unwrap();

        assert_ne!(
            key1, key2,
            "Different lineage seeds should produce different keys"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_lineage_beacon_key_empty_params(
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Empty params should use fallback seed (for testing)
        let params = json!({});

        let result = handle_derive_lineage_beacon_key(params).await?;

        // Should still work (uses zero seed for testing)
        assert!(result.get("beacon_key").is_some());

        let key_hex = result.get("beacon_key").and_then(|v| v.as_str()).unwrap();
        let key_bytes = hex::decode(key_hex)?;
        assert_eq!(key_bytes.len(), 32);

        Ok(())
    }
}
