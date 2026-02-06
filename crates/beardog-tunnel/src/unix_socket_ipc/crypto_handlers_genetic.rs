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
//! - `genetic.derive_device_seed` - Derive UNIQUE device seed (not copy!)
//! - `genetic.sign_lineage_certificate` - Sign device enrollment certificate
//! - `genetic.verify_lineage_certificate` - Verify device certificate
//!
//! # Architecture
//!
//! Types are defined in `crypto_handlers_genetic_types` module for maintainability.
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
//! - `derive_device_seed`: < 200μs (HKDF-SHA256)
//! - `sign_lineage_certificate`: < 500μs (Ed25519 sign)
//! - `verify_lineage_certificate`: < 300μs (Ed25519 verify)

use crate::tunnel::hsm::software_hsm::crypto_providers::genetic_crypto::GeneticCryptoProvider;
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use beardog_errors::BearDogError;
use hkdf::Hkdf;
use serde_json::{json, Value};
use sha2::Sha256;
// Removed: ConstantTimeEq - use BLAKE3 keyed MAC instead
use tracing::{debug, info, warn};

// Re-export types from the types module for backward compatibility
pub use super::crypto_handlers_genetic_types::*;

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

    hkdf.expand(domain, &mut okm)
        .map_err(|e| BearDogError::system(format!("HKDF beacon key derivation failed: {}", e)))?;

    // Encode as hex string for JSON-RPC
    let beacon_key_hex = hex::encode(&okm);

    info!("✅ Derived BirdSong beacon key: 32 bytes (HKDF-SHA256, domain-separated)");
    debug!("   Domain: birdsong_beacon_v1, Deterministic: true, Algorithm: ChaCha20-Poly1305");

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
        .mix_entropy(tier3.as_deref(), tier2.as_deref(), tier1.as_deref())
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
    let nonce_bytes = hex::decode(&request.nonce)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid nonce (not hex): {}", e)))?;

    // Compute HMAC-SHA512(nonce, lineage_key)
    use hmac::{Hmac, Mac};
    use sha2::Sha512;
    type HmacSha512 = Hmac<Sha512>;

    let mut mac = HmacSha512::new_from_slice(&lineage_key)
        .map_err(|e| BearDogError::system(format!("Failed to create HMAC: {}", e)))?;
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
        BearDogError::invalid_input(&format!("Invalid verify_challenge_response params: {}", e))
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
    let nonce_bytes = hex::decode(&request.nonce)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid nonce (not hex): {}", e)))?;

    let response_bytes = hex::decode(&request.response)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid response (not hex): {}", e)))?;

    // Compute expected response
    use hmac::{Hmac, Mac};
    use sha2::Sha512;
    type HmacSha512 = Hmac<Sha512>;

    let mut mac = HmacSha512::new_from_slice(&lineage_key)
        .map_err(|e| BearDogError::system(format!("Failed to create HMAC: {}", e)))?;
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
// DEVICE SEED DERIVATION (Deep Debt: DERIVE, not COPY)
// ============================================================================

/// Handle `genetic.derive_device_seed` RPC method
///
/// Derives a UNIQUE device seed from the family's root genesis seed.
/// This is the CORRECT approach for device enrollment - each device gets
/// its own derived seed, not a copy of the root seed.
///
/// # Security Properties
///
/// - **Forward secrecy**: Compromising one device doesn't reveal root seed
/// - **Device isolation**: Each device has unique cryptographic material
/// - **Verifiability**: derivation_proof allows proving the seed was derived correctly
///
/// # Performance
///
/// - Expected: < 200μs (HKDF-SHA256)
///
/// # Example
///
/// ```json
/// {
///     "root_seed": "base64_encoded_genesis_seed...",
///     "device_entropy": "base64_encoded_device_entropy...",
///     "device_id": "pixel8a"
/// }
/// ```
pub async fn handle_derive_device_seed(params: Value) -> Result<Value, BearDogError> {
    debug!("🧬 RPC: genetic.derive_device_seed");

    let request: DeriveDeviceSeedRequest = serde_json::from_value(params).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid derive_device_seed params: {}", e))
    })?;

    // Decode root seed from base64
    let root_seed = BASE64.decode(&request.root_seed).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid root_seed (not base64): {}", e))
    })?;

    if root_seed.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "root_seed must be 32 bytes, got {}",
            root_seed.len()
        )));
    }

    // Decode device entropy from base64
    let device_entropy = BASE64.decode(&request.device_entropy).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid device_entropy (not base64): {}", e))
    })?;

    if device_entropy.len() < 16 {
        return Err(BearDogError::invalid_input(&format!(
            "device_entropy must be at least 16 bytes, got {}",
            device_entropy.len()
        )));
    }

    // Domain separation for device seed derivation
    let domain = b"beardog_device_seed_v1";

    // Create HKDF with root seed as IKM
    let hkdf = Hkdf::<Sha256>::new(Some(&device_entropy), &root_seed);

    // Build info with device ID and optional timestamp
    let mut info = format!("{}:{}", domain.escape_ascii(), request.device_id);
    if let Some(ts) = request.enrollment_timestamp {
        info.push_str(&format!(":{}", ts));
    }

    // Derive 32-byte device seed
    let mut device_seed = [0u8; 32];
    hkdf.expand(info.as_bytes(), &mut device_seed)
        .map_err(|_| BearDogError::internal("HKDF expansion failed".to_string()))?;

    // Create derivation proof: HMAC(root_seed, device_seed || device_id)
    // This allows verifying the seed was derived correctly without revealing root_seed
    use hmac::{Hmac, Mac};
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(&root_seed)
        .map_err(|_| BearDogError::internal("HMAC key creation failed".to_string()))?;
    mac.update(&device_seed);
    mac.update(request.device_id.as_bytes());
    let proof = mac.finalize().into_bytes();

    let device_seed_b64 = BASE64.encode(&device_seed);
    let proof_b64 = BASE64.encode(&proof);

    info!(
        "✅ Derived device seed for '{}' (32 bytes, unique derivation)",
        request.device_id
    );

    Ok(json!(DeriveDeviceSeedResponse {
        device_seed: device_seed_b64,
        device_id: request.device_id,
        kdf: "HKDF-SHA256".to_string(),
        domain: String::from_utf8_lossy(domain).to_string(),
        derivation_proof: proof_b64,
    }))
}

// ============================================================================
// LINEAGE CERTIFICATE HANDLERS (Device Enrollment)
// ============================================================================

/// Handle `genetic.sign_lineage_certificate` RPC method
///
/// Signs a lineage certificate for device enrollment. When a new device joins
/// the family, the enrolling device (parent) signs a certificate proving
/// the new device's lineage.
///
/// # Security Properties
///
/// - Ed25519 signature for authenticity
/// - Includes depth for hierarchical verification
/// - Optional expiration for certificate rotation
///
/// # Performance
///
/// - Expected: < 500μs (Ed25519 sign)
///
/// # Example
///
/// ```json
/// {
///     "parent_seed": "base64_encoded_parent_device_seed...",
///     "parent_device_id": "usb-desktop",
///     "child_public_key": "base64_encoded_ed25519_pubkey...",
///     "child_device_id": "pixel8a",
///     "family_id": "8ff3b864a4bc589a"
/// }
/// ```
pub async fn handle_sign_lineage_certificate(params: Value) -> Result<Value, BearDogError> {
    debug!("🧬 RPC: genetic.sign_lineage_certificate");

    let request: SignLineageCertificateRequest = serde_json::from_value(params).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid sign_lineage_certificate params: {}", e))
    })?;

    // Decode parent seed
    let parent_seed = BASE64.decode(&request.parent_seed).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid parent_seed (not base64): {}", e))
    })?;

    if parent_seed.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "parent_seed must be 32 bytes, got {}",
            parent_seed.len()
        )));
    }

    // Decode child public key
    let child_pubkey_bytes = BASE64.decode(&request.child_public_key).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid child_public_key (not base64): {}", e))
    })?;

    if child_pubkey_bytes.len() != 32 {
        return Err(BearDogError::invalid_input(&format!(
            "child_public_key must be 32 bytes, got {}",
            child_pubkey_bytes.len()
        )));
    }

    // Derive Ed25519 keypair from parent seed
    use ed25519_dalek::{Signer, SigningKey, VerifyingKey};

    let parent_seed_array: [u8; 32] = parent_seed
        .try_into()
        .map_err(|_| BearDogError::internal("Failed to convert parent seed to array".to_string()))?;

    let signing_key = SigningKey::from_bytes(&parent_seed_array);
    let parent_public_key = VerifyingKey::from(&signing_key);

    // Build certificate data
    let issued_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // Version 1, depth 1 for direct children (could be passed as param for deeper hierarchies)
    let version: u8 = 1;
    let depth: u32 = 1; // Direct child of enrolling device

    // Create message to sign:
    // version || parent_id || child_id || child_pubkey || family_id || issued_at || depth
    let mut message = Vec::new();
    message.push(version);
    message.extend(request.parent_device_id.as_bytes());
    message.push(0); // null separator
    message.extend(request.child_device_id.as_bytes());
    message.push(0);
    message.extend(&child_pubkey_bytes);
    message.extend(request.family_id.as_bytes());
    message.push(0);
    message.extend(&issued_at.to_le_bytes());
    message.extend(&depth.to_le_bytes());

    // Sign the message
    let signature = signing_key.sign(&message);

    // Build certificate
    let certificate = LineageCertificate {
        version,
        parent_device_id: request.parent_device_id,
        child_device_id: request.child_device_id,
        child_public_key: request.child_public_key,
        family_id: request.family_id,
        issued_at,
        expires_at: request.expires_at,
        depth,
        parent_signature: BASE64.encode(signature.to_bytes()),
        parent_public_key: BASE64.encode(parent_public_key.to_bytes()),
    };

    // Create certificate ID (SHA256 of serialized certificate)
    use sha2::Digest;
    let cert_json = serde_json::to_string(&certificate)
        .map_err(|e| BearDogError::internal(format!("Failed to serialize certificate: {}", e)))?;
    let cert_id = sha2::Sha256::digest(cert_json.as_bytes());
    let cert_id_hex = hex::encode(cert_id);

    info!(
        "✅ Signed lineage certificate: {} → {} (cert: {}...)",
        certificate.parent_device_id,
        certificate.child_device_id,
        &cert_id_hex[..16]
    );

    Ok(json!(SignLineageCertificateResponse {
        certificate,
        certificate_id: cert_id_hex,
    }))
}

/// Handle `genetic.verify_lineage_certificate` RPC method
///
/// Verifies a lineage certificate's signature and optional chain.
///
/// # Verification Steps
///
/// 1. Verify Ed25519 signature is valid
/// 2. Check certificate not expired
/// 3. Optionally verify family_id matches expected
/// 4. Optionally verify chain up to trust anchors
///
/// # Performance
///
/// - Expected: < 300μs (Ed25519 verify)
///
/// # Example
///
/// ```json
/// {
///     "certificate": { ... },
///     "expected_family_id": "8ff3b864a4bc589a",
///     "trust_anchors": []
/// }
/// ```
pub async fn handle_verify_lineage_certificate(params: Value) -> Result<Value, BearDogError> {
    debug!("🔍 RPC: genetic.verify_lineage_certificate");

    let request: VerifyLineageCertificateRequest = serde_json::from_value(params).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid verify_lineage_certificate params: {}", e))
    })?;

    let cert = &request.certificate;

    // Initialize verification details
    let mut details = CertificateVerificationDetails {
        signature_valid: false,
        not_expired: true,
        family_id_matches: true,
        chain_verified: request.trust_anchors.is_empty(), // No chain to verify if empty
        chain_depth: cert.depth,
        failure_reason: None,
    };

    // 1. Decode and verify signature
    let parent_pubkey_bytes = BASE64.decode(&cert.parent_public_key).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid parent_public_key: {}", e))
    })?;

    let signature_bytes = BASE64.decode(&cert.parent_signature).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid parent_signature: {}", e))
    })?;

    let child_pubkey_bytes = BASE64.decode(&cert.child_public_key).map_err(|e| {
        BearDogError::invalid_input(&format!("Invalid child_public_key: {}", e))
    })?;

    // Verify Ed25519 signature
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    let parent_pubkey_array: [u8; 32] = parent_pubkey_bytes.try_into().map_err(|_| {
        BearDogError::invalid_input("parent_public_key must be 32 bytes")
    })?;

    let signature_array: [u8; 64] = signature_bytes.try_into().map_err(|_| {
        BearDogError::invalid_input("parent_signature must be 64 bytes")
    })?;

    let verifying_key = VerifyingKey::from_bytes(&parent_pubkey_array)
        .map_err(|e| BearDogError::invalid_input(&format!("Invalid public key: {}", e)))?;

    let signature = Signature::from_bytes(&signature_array);

    // Reconstruct message that was signed
    let mut message = Vec::new();
    message.push(cert.version);
    message.extend(cert.parent_device_id.as_bytes());
    message.push(0);
    message.extend(cert.child_device_id.as_bytes());
    message.push(0);
    message.extend(&child_pubkey_bytes);
    message.extend(cert.family_id.as_bytes());
    message.push(0);
    message.extend(&cert.issued_at.to_le_bytes());
    message.extend(&cert.depth.to_le_bytes());

    // Verify signature
    details.signature_valid = verifying_key.verify(&message, &signature).is_ok();

    if !details.signature_valid {
        details.failure_reason = Some("Invalid Ed25519 signature".to_string());
    }

    // 2. Check expiration
    if let Some(expires_at) = cert.expires_at {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        if now > expires_at {
            details.not_expired = false;
            details.failure_reason = Some(format!(
                "Certificate expired at {} (current: {})",
                expires_at, now
            ));
        }
    }

    // 3. Check family_id matches expected (if provided)
    if let Some(expected) = &request.expected_family_id {
        if &cert.family_id != expected {
            details.family_id_matches = false;
            details.failure_reason = Some(format!(
                "Family ID mismatch: expected '{}', got '{}'",
                expected, cert.family_id
            ));
        }
    }

    // 4. Chain verification (if trust anchors provided)
    if !request.trust_anchors.is_empty() {
        // For now, simple check: verify parent's public key matches a trust anchor
        // Full chain verification would walk the tree
        let parent_key_matches = request.trust_anchors.iter().any(|anchor| {
            // Check if this cert's parent public key matches any anchor's child public key
            // (meaning this cert was signed by an already-trusted device)
            anchor.child_public_key == cert.parent_public_key
                || anchor.parent_public_key == cert.parent_public_key
        });

        details.chain_verified = parent_key_matches;

        if !parent_key_matches {
            details.failure_reason =
                Some("Certificate chain doesn't link to any trust anchor".to_string());
        }
    }

    // Determine overall validity
    let valid =
        details.signature_valid && details.not_expired && details.family_id_matches && details.chain_verified;

    if valid {
        info!(
            "✅ Certificate verified: {} → {} (depth: {})",
            cert.parent_device_id, cert.child_device_id, cert.depth
        );
    } else {
        warn!(
            "❌ Certificate verification FAILED: {} → {} (reason: {:?})",
            cert.parent_device_id, cert.child_device_id, details.failure_reason
        );
    }

    Ok(json!(VerifyLineageCertificateResponse { valid, details }))
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
        assert_eq!(
            result.get("key_size_bytes").and_then(|v| v.as_u64()),
            Some(32)
        );
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
    async fn test_derive_lineage_beacon_key_deterministic() -> Result<(), Box<dyn std::error::Error>>
    {
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
    async fn test_derive_lineage_beacon_key_empty_params() -> Result<(), Box<dyn std::error::Error>>
    {
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

    // ========================================================================
    // DEVICE SEED DERIVATION TESTS (Deep Debt: DERIVE, not COPY)
    // ========================================================================

    #[tokio::test]
    async fn test_derive_device_seed() -> Result<(), Box<dyn std::error::Error>> {
        // Seeds must be exactly 32 bytes
        let root_seed = BASE64.encode(b"genesis_root_seed_32byteslong!!!");  // 32 bytes
        let device_entropy = BASE64.encode(b"pixel8a_entropy_data_here!!!_32!");  // 32 bytes

        let params = json!({
            "root_seed": root_seed,
            "device_entropy": device_entropy,
            "device_id": "pixel8a",
        });

        let result = handle_derive_device_seed(params).await?;
        let response: DeriveDeviceSeedResponse = serde_json::from_value(result)?;

        // Verify response
        assert_eq!(response.device_id, "pixel8a");
        assert_eq!(response.kdf, "HKDF-SHA256");
        assert!(!response.device_seed.is_empty());
        assert!(!response.derivation_proof.is_empty());

        // Decode and verify sizes
        let device_seed_bytes = BASE64.decode(&response.device_seed)?;
        let proof_bytes = BASE64.decode(&response.derivation_proof)?;

        assert_eq!(device_seed_bytes.len(), 32, "Device seed should be 32 bytes");
        assert_eq!(proof_bytes.len(), 32, "Derivation proof should be 32 bytes");

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_device_seed_different_devices() -> Result<(), Box<dyn std::error::Error>> {
        // Same root seed but different device entropy should produce different seeds
        // Seeds must be exactly 32 bytes
        let root_seed = BASE64.encode(b"shared_family_root_seed_here!!!!");

        let entropy1 = BASE64.encode(b"usb_desktop_entropy_1234567890!!");
        let entropy2 = BASE64.encode(b"pixel8a_mobile_entropy_987654!!");

        let params1 = json!({
            "root_seed": root_seed.clone(),
            "device_entropy": entropy1,
            "device_id": "usb-desktop",
        });

        let params2 = json!({
            "root_seed": root_seed,
            "device_entropy": entropy2,
            "device_id": "pixel8a",
        });

        let result1 = handle_derive_device_seed(params1).await?;
        let result2 = handle_derive_device_seed(params2).await?;

        let resp1: DeriveDeviceSeedResponse = serde_json::from_value(result1)?;
        let resp2: DeriveDeviceSeedResponse = serde_json::from_value(result2)?;

        // Seeds should be different (DERIVE, not COPY!)
        assert_ne!(
            resp1.device_seed, resp2.device_seed,
            "Different devices should have different derived seeds"
        );

        // Proofs should also be different
        assert_ne!(
            resp1.derivation_proof, resp2.derivation_proof,
            "Derivation proofs should be unique per device"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_device_seed_deterministic() -> Result<(), Box<dyn std::error::Error>> {
        // Same inputs should produce same output (deterministic)
        // Seeds must be exactly 32 bytes
        let root_seed = BASE64.encode(b"deterministic_root_seed_test!!!!");
        let device_entropy = BASE64.encode(b"deterministic_device_entropy!!!!");

        let params = json!({
            "root_seed": root_seed,
            "device_entropy": device_entropy,
            "device_id": "test-device",
        });

        let result1 = handle_derive_device_seed(params.clone()).await?;
        let result2 = handle_derive_device_seed(params).await?;

        let resp1: DeriveDeviceSeedResponse = serde_json::from_value(result1)?;
        let resp2: DeriveDeviceSeedResponse = serde_json::from_value(result2)?;

        assert_eq!(
            resp1.device_seed, resp2.device_seed,
            "Same inputs should produce same device seed"
        );

        Ok(())
    }

    // ========================================================================
    // LINEAGE CERTIFICATE TESTS (Device Enrollment)
    // ========================================================================

    #[tokio::test]
    async fn test_sign_and_verify_lineage_certificate() -> Result<(), Box<dyn std::error::Error>> {
        // Create parent seed (enrolling device) - must be exactly 32 bytes
        let parent_seed = BASE64.encode(b"parent_device_seed_32_bytes!!!!!");

        // Generate child public key (in real scenario, child generates this)
        use ed25519_dalek::SigningKey;
        let child_signing_key = SigningKey::from_bytes(&[42u8; 32]);
        let child_public_key = BASE64.encode(child_signing_key.verifying_key().to_bytes());

        // Sign certificate
        let sign_params = json!({
            "parent_seed": parent_seed,
            "parent_device_id": "usb-desktop",
            "child_public_key": child_public_key,
            "child_device_id": "pixel8a",
            "family_id": "8ff3b864a4bc589a",
        });

        let sign_result = handle_sign_lineage_certificate(sign_params).await?;
        let sign_response: SignLineageCertificateResponse = serde_json::from_value(sign_result)?;

        // Verify certificate
        assert!(!sign_response.certificate_id.is_empty());
        assert_eq!(sign_response.certificate.parent_device_id, "usb-desktop");
        assert_eq!(sign_response.certificate.child_device_id, "pixel8a");
        assert_eq!(sign_response.certificate.family_id, "8ff3b864a4bc589a");
        assert_eq!(sign_response.certificate.version, 1);
        assert_eq!(sign_response.certificate.depth, 1);

        // Now verify the certificate
        let verify_params = json!({
            "certificate": sign_response.certificate,
            "expected_family_id": "8ff3b864a4bc589a",
            "trust_anchors": [],
        });

        let verify_result = handle_verify_lineage_certificate(verify_params).await?;
        let verify_response: VerifyLineageCertificateResponse =
            serde_json::from_value(verify_result)?;

        assert!(verify_response.valid, "Certificate should be valid");
        assert!(verify_response.details.signature_valid);
        assert!(verify_response.details.not_expired);
        assert!(verify_response.details.family_id_matches);

        Ok(())
    }

    #[tokio::test]
    async fn test_verify_lineage_certificate_wrong_family() -> Result<(), Box<dyn std::error::Error>>
    {
        // Must be exactly 32 bytes
        let parent_seed = BASE64.encode(b"parent_device_seed_for_test!!!!!");

        use ed25519_dalek::SigningKey;
        let child_signing_key = SigningKey::from_bytes(&[99u8; 32]);
        let child_public_key = BASE64.encode(child_signing_key.verifying_key().to_bytes());

        // Sign certificate with family_id "aaa"
        let sign_params = json!({
            "parent_seed": parent_seed,
            "parent_device_id": "device-a",
            "child_public_key": child_public_key,
            "child_device_id": "device-b",
            "family_id": "family_aaa",
        });

        let sign_result = handle_sign_lineage_certificate(sign_params).await?;
        let sign_response: SignLineageCertificateResponse = serde_json::from_value(sign_result)?;

        // Verify with WRONG expected family
        let verify_params = json!({
            "certificate": sign_response.certificate,
            "expected_family_id": "family_bbb",  // Wrong!
            "trust_anchors": [],
        });

        let verify_result = handle_verify_lineage_certificate(verify_params).await?;
        let verify_response: VerifyLineageCertificateResponse =
            serde_json::from_value(verify_result)?;

        // Certificate should be invalid due to family mismatch
        assert!(!verify_response.valid);
        assert!(!verify_response.details.family_id_matches);
        assert!(verify_response
            .details
            .failure_reason
            .as_ref()
            .map(|r| r.contains("Family ID mismatch"))
            .unwrap_or(false));

        Ok(())
    }

    #[tokio::test]
    async fn test_verify_lineage_certificate_tampered_signature(
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Must be exactly 32 bytes
        let parent_seed = BASE64.encode(b"parent_device_seed_tamper_t!!!!!");

        use ed25519_dalek::SigningKey;
        let child_signing_key = SigningKey::from_bytes(&[77u8; 32]);
        let child_public_key = BASE64.encode(child_signing_key.verifying_key().to_bytes());

        // Sign certificate
        let sign_params = json!({
            "parent_seed": parent_seed,
            "parent_device_id": "device-x",
            "child_public_key": child_public_key,
            "child_device_id": "device-y",
            "family_id": "test_family",
        });

        let sign_result = handle_sign_lineage_certificate(sign_params).await?;
        let mut sign_response: SignLineageCertificateResponse =
            serde_json::from_value(sign_result)?;

        // Tamper with the certificate - change the child device ID
        sign_response.certificate.child_device_id = "tampered-device".to_string();

        // Verify tampered certificate
        let verify_params = json!({
            "certificate": sign_response.certificate,
            "trust_anchors": [],
        });

        let verify_result = handle_verify_lineage_certificate(verify_params).await?;
        let verify_response: VerifyLineageCertificateResponse =
            serde_json::from_value(verify_result)?;

        // Certificate should be invalid due to signature mismatch
        assert!(!verify_response.valid);
        assert!(!verify_response.details.signature_valid);

        Ok(())
    }

    // ========================================================================
    // EDGE CASE TESTS - Device Seed Derivation
    // ========================================================================

    #[tokio::test]
    async fn test_derive_device_seed_invalid_root_seed_length() {
        // Too short (16 bytes instead of 32)
        let short_seed = BASE64.encode(b"only_16_bytes!!");
        let device_entropy = BASE64.encode(b"device_entropy_data_32_bytes!!!!");

        let params = json!({
            "root_seed": short_seed,
            "device_entropy": device_entropy,
            "device_id": "test-device",
        });

        let result = handle_derive_device_seed(params).await;
        assert!(result.is_err(), "Should reject short root seed");
        let err = result.unwrap_err().to_string();
        assert!(err.contains("32 bytes"), "Error should mention size requirement");
    }

    #[tokio::test]
    async fn test_derive_device_seed_invalid_base64() {
        let params = json!({
            "root_seed": "not-valid-base64!!!",
            "device_entropy": "also-not-valid!!!",
            "device_id": "test-device",
        });

        let result = handle_derive_device_seed(params).await;
        assert!(result.is_err(), "Should reject invalid base64");
    }

    #[tokio::test]
    async fn test_derive_device_seed_entropy_too_short() {
        let root_seed = BASE64.encode(b"valid_root_seed_exactly_32bytes!");
        let short_entropy = BASE64.encode(b"tiny"); // Only 4 bytes

        let params = json!({
            "root_seed": root_seed,
            "device_entropy": short_entropy,
            "device_id": "test-device",
        });

        let result = handle_derive_device_seed(params).await;
        assert!(result.is_err(), "Should reject entropy < 16 bytes");
    }

    #[tokio::test]
    async fn test_derive_device_seed_with_timestamp() -> Result<(), Box<dyn std::error::Error>> {
        let root_seed = BASE64.encode(b"root_seed_with_timestamp_test!!!");
        let device_entropy = BASE64.encode(b"device_entropy_timestamp_test!!!");

        // With timestamp
        let params_with_ts = json!({
            "root_seed": root_seed.clone(),
            "device_entropy": device_entropy.clone(),
            "device_id": "timestamped-device",
            "enrollment_timestamp": 1738713600u64,
        });

        // Without timestamp
        let params_without_ts = json!({
            "root_seed": root_seed,
            "device_entropy": device_entropy,
            "device_id": "timestamped-device",
        });

        let result_with = handle_derive_device_seed(params_with_ts).await?;
        let result_without = handle_derive_device_seed(params_without_ts).await?;

        let resp_with: DeriveDeviceSeedResponse = serde_json::from_value(result_with)?;
        let resp_without: DeriveDeviceSeedResponse = serde_json::from_value(result_without)?;

        // Different timestamps should produce different seeds
        assert_ne!(
            resp_with.device_seed, resp_without.device_seed,
            "Timestamp should affect derivation"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_device_seed_empty_device_id() -> Result<(), Box<dyn std::error::Error>> {
        let root_seed = BASE64.encode(b"root_seed_empty_device_id_test!!");
        let device_entropy = BASE64.encode(b"device_entropy_empty_id_testing!");

        let params = json!({
            "root_seed": root_seed,
            "device_entropy": device_entropy,
            "device_id": "",  // Empty device ID
        });

        // Should still work (empty string is valid, just not recommended)
        let result = handle_derive_device_seed(params).await?;
        let response: DeriveDeviceSeedResponse = serde_json::from_value(result)?;
        assert_eq!(response.device_id, "");
        assert!(!response.device_seed.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_derive_device_seed_unicode_device_id() -> Result<(), Box<dyn std::error::Error>> {
        // Seeds must be exactly 32 bytes
        let root_seed = BASE64.encode(b"root_seed_unicode_device_id!!!!!");  // 32 bytes
        let device_entropy = BASE64.encode(b"device_entropy_unicode_testing!!");  // 32 bytes

        let params = json!({
            "root_seed": root_seed,
            "device_entropy": device_entropy,
            "device_id": "设备-🦀-pixel",  // Unicode device ID
        });

        let result = handle_derive_device_seed(params).await?;
        let response: DeriveDeviceSeedResponse = serde_json::from_value(result)?;
        assert_eq!(response.device_id, "设备-🦀-pixel");

        Ok(())
    }

    // ========================================================================
    // EDGE CASE TESTS - Certificate Signing/Verification
    // ========================================================================

    #[tokio::test]
    async fn test_sign_certificate_invalid_parent_seed_length() {
        let short_seed = BASE64.encode(b"only_16_bytes!!");

        use ed25519_dalek::SigningKey;
        let child_key = SigningKey::from_bytes(&[1u8; 32]);
        let child_pubkey = BASE64.encode(child_key.verifying_key().to_bytes());

        let params = json!({
            "parent_seed": short_seed,
            "parent_device_id": "parent",
            "child_public_key": child_pubkey,
            "child_device_id": "child",
            "family_id": "test",
        });

        let result = handle_sign_lineage_certificate(params).await;
        assert!(result.is_err(), "Should reject short parent seed");
    }

    #[tokio::test]
    async fn test_sign_certificate_invalid_child_pubkey_length() {
        let parent_seed = BASE64.encode(b"valid_parent_seed_exactly_32!!!");
        let invalid_pubkey = BASE64.encode(b"too_short"); // Not 32 bytes

        let params = json!({
            "parent_seed": parent_seed,
            "parent_device_id": "parent",
            "child_public_key": invalid_pubkey,
            "child_device_id": "child",
            "family_id": "test",
        });

        let result = handle_sign_lineage_certificate(params).await;
        assert!(result.is_err(), "Should reject invalid public key length");
    }

    #[tokio::test]
    async fn test_verify_certificate_expired() -> Result<(), Box<dyn std::error::Error>> {
        let parent_seed = BASE64.encode(b"parent_seed_for_expiry_testing!!");

        use ed25519_dalek::SigningKey;
        let child_key = SigningKey::from_bytes(&[55u8; 32]);
        let child_pubkey = BASE64.encode(child_key.verifying_key().to_bytes());

        // Sign certificate with expiration in the past
        let params = json!({
            "parent_seed": parent_seed,
            "parent_device_id": "parent",
            "child_public_key": child_pubkey,
            "child_device_id": "child",
            "family_id": "test-family",
            "expires_at": 1000u64,  // Way in the past (Unix timestamp)
        });

        let sign_result = handle_sign_lineage_certificate(params).await?;
        let sign_response: SignLineageCertificateResponse = serde_json::from_value(sign_result)?;

        // Verify expired certificate
        let verify_params = json!({
            "certificate": sign_response.certificate,
            "trust_anchors": [],
        });

        let verify_result = handle_verify_lineage_certificate(verify_params).await?;
        let verify_response: VerifyLineageCertificateResponse =
            serde_json::from_value(verify_result)?;

        assert!(!verify_response.valid, "Expired certificate should be invalid");
        assert!(!verify_response.details.not_expired);
        assert!(verify_response
            .details
            .failure_reason
            .as_ref()
            .map(|r| r.contains("expired"))
            .unwrap_or(false));

        Ok(())
    }

    #[tokio::test]
    async fn test_verify_certificate_with_trust_anchor() -> Result<(), Box<dyn std::error::Error>> {
        // Create a "root" certificate (trust anchor)
        // Note: root_seed and root_pubkey are defined for documentation purposes
        // showing the expected trust chain structure, even though this test 
        // focuses on parent->child verification with trust_anchors
        let _root_seed = BASE64.encode(b"root_device_seed_for_chain_test!");

        use ed25519_dalek::SigningKey;
        let _root_signing_key = SigningKey::from_bytes(&[10u8; 32]);
        let _root_pubkey = BASE64.encode(_root_signing_key.verifying_key().to_bytes());

        // Root signs itself (depth 0)
        // For simplicity, we'll create a parent that will be our trust anchor

        let parent_seed = BASE64.encode(b"parent_seed_for_trust_anchor!!!!");
        let parent_signing_key = SigningKey::from_bytes(
            &BASE64.decode(&parent_seed)?[..32].try_into().unwrap()
        );
        let parent_pubkey = BASE64.encode(parent_signing_key.verifying_key().to_bytes());

        // Create child and sign with parent
        let child_key = SigningKey::from_bytes(&[88u8; 32]);
        let child_pubkey = BASE64.encode(child_key.verifying_key().to_bytes());

        let sign_params = json!({
            "parent_seed": parent_seed,
            "parent_device_id": "parent-device",
            "child_public_key": child_pubkey,
            "child_device_id": "child-device",
            "family_id": "chain-family",
        });

        let sign_result = handle_sign_lineage_certificate(sign_params).await?;
        let child_cert: SignLineageCertificateResponse = serde_json::from_value(sign_result)?;

        // Create a trust anchor that matches the parent's public key
        let trust_anchor = LineageCertificate {
            version: 1,
            parent_device_id: "genesis".to_string(),
            child_device_id: "parent-device".to_string(),
            child_public_key: parent_pubkey.clone(),
            family_id: "chain-family".to_string(),
            issued_at: 0,
            expires_at: None,
            depth: 0,
            parent_signature: "dummy".to_string(), // Not verified for trust anchors
            parent_public_key: parent_pubkey,
        };

        // Verify with trust anchor
        let verify_params = json!({
            "certificate": child_cert.certificate,
            "trust_anchors": [trust_anchor],
        });

        let verify_result = handle_verify_lineage_certificate(verify_params).await?;
        let verify_response: VerifyLineageCertificateResponse =
            serde_json::from_value(verify_result)?;

        assert!(verify_response.valid, "Certificate should verify against trust anchor");
        assert!(verify_response.details.chain_verified);

        Ok(())
    }

    // ========================================================================
    // CHAOS TESTS - Malformed Inputs
    // ========================================================================

    #[tokio::test]
    async fn test_chaos_empty_params_device_seed() {
        let params = json!({});
        let result = handle_derive_device_seed(params).await;
        assert!(result.is_err(), "Empty params should fail");
    }

    #[tokio::test]
    async fn test_chaos_null_values_device_seed() {
        let params = json!({
            "root_seed": null,
            "device_entropy": null,
            "device_id": null,
        });
        let result = handle_derive_device_seed(params).await;
        assert!(result.is_err(), "Null values should fail");
    }

    #[tokio::test]
    async fn test_chaos_wrong_types_device_seed() {
        let params = json!({
            "root_seed": 12345,  // Number instead of string
            "device_entropy": ["array"],  // Array instead of string
            "device_id": {"object": true},  // Object instead of string
        });
        let result = handle_derive_device_seed(params).await;
        assert!(result.is_err(), "Wrong types should fail");
    }

    #[tokio::test]
    async fn test_chaos_extra_fields_device_seed() -> Result<(), Box<dyn std::error::Error>> {
        let root_seed = BASE64.encode(b"root_seed_extra_fields_testing!!");
        let device_entropy = BASE64.encode(b"device_entropy_extra_fields!!!!");

        // Extra fields should be ignored
        let params = json!({
            "root_seed": root_seed,
            "device_entropy": device_entropy,
            "device_id": "test-device",
            "extra_field": "should be ignored",
            "another_extra": 12345,
        });

        let result = handle_derive_device_seed(params).await?;
        let response: DeriveDeviceSeedResponse = serde_json::from_value(result)?;
        assert!(!response.device_seed.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_chaos_very_long_device_id() -> Result<(), Box<dyn std::error::Error>> {
        let root_seed = BASE64.encode(b"root_seed_long_device_id_test!!!");
        let device_entropy = BASE64.encode(b"device_entropy_long_id_testing!!");

        // Very long device ID (1000 characters)
        let long_id = "a".repeat(1000);

        let params = json!({
            "root_seed": root_seed,
            "device_entropy": device_entropy,
            "device_id": long_id,
        });

        let result = handle_derive_device_seed(params).await?;
        let response: DeriveDeviceSeedResponse = serde_json::from_value(result)?;
        assert_eq!(response.device_id.len(), 1000);

        Ok(())
    }

    #[tokio::test]
    async fn test_chaos_special_characters_family_id() -> Result<(), Box<dyn std::error::Error>> {
        let parent_seed = BASE64.encode(b"parent_seed_special_chars_test!!");

        use ed25519_dalek::SigningKey;
        let child_key = SigningKey::from_bytes(&[66u8; 32]);
        let child_pubkey = BASE64.encode(child_key.verifying_key().to_bytes());

        // Special characters in family_id
        let params = json!({
            "parent_seed": parent_seed,
            "parent_device_id": "parent",
            "child_public_key": child_pubkey,
            "child_device_id": "child",
            "family_id": "family-测试-🔐-\n\t\r",
        });

        let result = handle_sign_lineage_certificate(params).await?;
        let response: SignLineageCertificateResponse = serde_json::from_value(result)?;
        assert!(response.certificate.family_id.contains("测试"));
        assert!(response.certificate.family_id.contains("🔐"));

        Ok(())
    }

    // ========================================================================
    // FAULT INJECTION TESTS - Error Paths
    // ========================================================================

    #[tokio::test]
    async fn test_fault_invalid_signature_bytes() {
        // Construct a certificate with invalid signature bytes
        let cert = LineageCertificate {
            version: 1,
            parent_device_id: "parent".to_string(),
            child_device_id: "child".to_string(),
            child_public_key: BASE64.encode(&[0u8; 32]),
            family_id: "test".to_string(),
            issued_at: 0,
            expires_at: None,
            depth: 1,
            parent_signature: "not-valid-base64!!!".to_string(),
            parent_public_key: BASE64.encode(&[0u8; 32]),
        };

        let verify_params = json!({
            "certificate": cert,
            "trust_anchors": [],
        });

        let result = handle_verify_lineage_certificate(verify_params).await;
        assert!(result.is_err(), "Invalid signature encoding should fail");
    }

    #[tokio::test]
    async fn test_fault_corrupted_public_key() -> Result<(), Box<dyn std::error::Error>> {
        // Use bytes that are definitely NOT a valid Ed25519 public key
        // The low-order point (0, 1) is invalid for Ed25519
        let mut invalid_pubkey = [0u8; 32];
        invalid_pubkey[0] = 0xee; // Set some bytes that won't form a valid point

        let cert = LineageCertificate {
            version: 1,
            parent_device_id: "parent".to_string(),
            child_device_id: "child".to_string(),
            child_public_key: BASE64.encode(&invalid_pubkey),
            family_id: "test".to_string(),
            issued_at: 0,
            expires_at: None,
            depth: 1,
            parent_signature: BASE64.encode(&[0u8; 64]),
            parent_public_key: BASE64.encode(&invalid_pubkey),
        };

        let verify_params = json!({
            "certificate": cert,
            "trust_anchors": [],
        });

        let result = handle_verify_lineage_certificate(verify_params).await;
        
        // Either the verification fails completely (invalid key parse)
        // OR it succeeds but the signature is invalid
        match result {
            Err(_) => {
                // Expected - invalid public key rejected
                Ok(())
            }
            Ok(value) => {
                // If parsing succeeded, signature must be invalid
                let response: VerifyLineageCertificateResponse = serde_json::from_value(value)?;
                assert!(!response.valid, "Corrupted key should result in invalid verification");
                assert!(!response.details.signature_valid);
                Ok(())
            }
        }
    }

    #[tokio::test]
    async fn test_fault_signature_wrong_length() {
        let cert = LineageCertificate {
            version: 1,
            parent_device_id: "parent".to_string(),
            child_device_id: "child".to_string(),
            child_public_key: BASE64.encode(&[0u8; 32]),
            family_id: "test".to_string(),
            issued_at: 0,
            expires_at: None,
            depth: 1,
            parent_signature: BASE64.encode(&[0u8; 32]), // 32 bytes instead of 64
            parent_public_key: BASE64.encode(&[0u8; 32]),
        };

        let verify_params = json!({
            "certificate": cert,
            "trust_anchors": [],
        });

        let result = handle_verify_lineage_certificate(verify_params).await;
        assert!(result.is_err(), "Wrong signature length should fail");
    }

    // ========================================================================
    // CONCURRENCY TESTS
    // ========================================================================

    #[tokio::test]
    async fn test_concurrent_device_seed_derivation() -> Result<(), Box<dyn std::error::Error>> {
        use tokio::task::JoinSet;

        let root_seed = BASE64.encode(b"concurrent_root_seed_testing!!!!");

        let mut tasks = JoinSet::new();

        // Spawn 10 concurrent derivations
        for i in 0..10 {
            let root = root_seed.clone();
            let entropy = BASE64.encode(format!("entropy_for_device_{:02}_padding!!", i).as_bytes());
            let device_id = format!("device-{}", i);

            tasks.spawn(async move {
                let params = json!({
                    "root_seed": root,
                    "device_entropy": entropy,
                    "device_id": device_id,
                });
                handle_derive_device_seed(params).await
            });
        }

        // Collect results
        let mut results = Vec::new();
        while let Some(result) = tasks.join_next().await {
            results.push(result??);
        }

        // All should succeed
        assert_eq!(results.len(), 10);

        // All should have unique seeds
        let seeds: std::collections::HashSet<_> = results
            .iter()
            .map(|r| r.get("device_seed").unwrap().as_str().unwrap())
            .collect();
        assert_eq!(seeds.len(), 10, "All concurrent derivations should be unique");

        Ok(())
    }

    #[tokio::test]
    async fn test_concurrent_certificate_signing() -> Result<(), Box<dyn std::error::Error>> {
        use tokio::task::JoinSet;

        let mut tasks = JoinSet::new();

        // Spawn 5 concurrent certificate signings
        for i in 0..5 {
            // Seeds must be exactly 32 bytes - use fixed base + index
            let parent_seed = BASE64.encode(
                format!("parent_seed_concurrent_{:02}_pad!!!", i).as_bytes()  // 32 bytes
            );

            tasks.spawn(async move {
                use ed25519_dalek::SigningKey;
                let child_key = SigningKey::from_bytes(&[i as u8; 32]);
                let child_pubkey = BASE64.encode(child_key.verifying_key().to_bytes());

                let params = json!({
                    "parent_seed": parent_seed,
                    "parent_device_id": format!("parent-{}", i),
                    "child_public_key": child_pubkey,
                    "child_device_id": format!("child-{}", i),
                    "family_id": "concurrent-family",
                });
                handle_sign_lineage_certificate(params).await
            });
        }

        // Collect results
        let mut results = Vec::new();
        while let Some(result) = tasks.join_next().await {
            results.push(result??);
        }

        // All should succeed
        assert_eq!(results.len(), 5);

        // All should have unique certificate IDs
        let cert_ids: std::collections::HashSet<_> = results
            .iter()
            .map(|r| r.get("certificate_id").unwrap().as_str().unwrap())
            .collect();
        assert_eq!(cert_ids.len(), 5, "All concurrent signings should have unique cert IDs");

        Ok(())
    }

    // ========================================================================
    // E2E ENROLLMENT FLOW TEST
    // ========================================================================

    #[tokio::test]
    async fn test_e2e_complete_enrollment_flow() -> Result<(), Box<dyn std::error::Error>> {
        // ===== STEP 1: Genesis device creates root seed =====
        // (In real scenario, this would be secure random generation)
        let root_seed = BASE64.encode(b"genesis_root_seed_for_e2e_test!!");

        // ===== STEP 2: Genesis device derives its own device seed =====
        let genesis_entropy = BASE64.encode(b"genesis_device_hardware_entropy!");
        let genesis_params = json!({
            "root_seed": root_seed.clone(),
            "device_entropy": genesis_entropy,
            "device_id": "genesis-device",
        });
        let genesis_result = handle_derive_device_seed(genesis_params).await?;
        let genesis_seed: DeriveDeviceSeedResponse = serde_json::from_value(genesis_result)?;

        // ===== STEP 3: USB Tower enrolls (child of genesis) =====
        let usb_entropy = BASE64.encode(b"usb_tower_hardware_entropy_here!");
        let usb_params = json!({
            "root_seed": root_seed.clone(),
            "device_entropy": usb_entropy,
            "device_id": "usb-tower",
        });
        let usb_result = handle_derive_device_seed(usb_params).await?;
        let usb_seed: DeriveDeviceSeedResponse = serde_json::from_value(usb_result)?;

        // Verify USB seed is different from genesis
        assert_ne!(genesis_seed.device_seed, usb_seed.device_seed);

        // Genesis signs certificate for USB Tower
        use ed25519_dalek::SigningKey;
        let usb_signing_key = SigningKey::from_bytes(
            &BASE64.decode(&usb_seed.device_seed)?[..32].try_into().unwrap()
        );
        let usb_pubkey = BASE64.encode(usb_signing_key.verifying_key().to_bytes());

        let cert_params = json!({
            "parent_seed": genesis_seed.device_seed,
            "parent_device_id": "genesis-device",
            "child_public_key": usb_pubkey,
            "child_device_id": "usb-tower",
            "family_id": "test-family-e2e",
        });
        let cert_result = handle_sign_lineage_certificate(cert_params).await?;
        let usb_cert: SignLineageCertificateResponse = serde_json::from_value(cert_result)?;

        // Verify the certificate
        let verify_params = json!({
            "certificate": usb_cert.certificate,
            "expected_family_id": "test-family-e2e",
            "trust_anchors": [],
        });
        let verify_result = handle_verify_lineage_certificate(verify_params).await?;
        let verify_resp: VerifyLineageCertificateResponse = serde_json::from_value(verify_result)?;
        assert!(verify_resp.valid, "USB Tower certificate should be valid");

        // ===== STEP 4: Pixel enrolls (child of USB Tower) =====
        let pixel_entropy = BASE64.encode(b"pixel_8a_hardware_entropy_here!!");
        let pixel_params = json!({
            "root_seed": root_seed,
            "device_entropy": pixel_entropy,
            "device_id": "pixel-8a",
        });
        let pixel_result = handle_derive_device_seed(pixel_params).await?;
        let pixel_seed: DeriveDeviceSeedResponse = serde_json::from_value(pixel_result)?;

        // Verify Pixel seed is different from both genesis and USB
        assert_ne!(pixel_seed.device_seed, genesis_seed.device_seed);
        assert_ne!(pixel_seed.device_seed, usb_seed.device_seed);

        // USB Tower signs certificate for Pixel
        let pixel_signing_key = SigningKey::from_bytes(
            &BASE64.decode(&pixel_seed.device_seed)?[..32].try_into().unwrap()
        );
        let pixel_pubkey = BASE64.encode(pixel_signing_key.verifying_key().to_bytes());

        let pixel_cert_params = json!({
            "parent_seed": usb_seed.device_seed,
            "parent_device_id": "usb-tower",
            "child_public_key": pixel_pubkey,
            "child_device_id": "pixel-8a",
            "family_id": "test-family-e2e",
        });
        let pixel_cert_result = handle_sign_lineage_certificate(pixel_cert_params).await?;
        let pixel_cert: SignLineageCertificateResponse = serde_json::from_value(pixel_cert_result)?;

        // Verify Pixel certificate with USB Tower as trust anchor
        let trust_anchor = usb_cert.certificate.clone();
        let verify_pixel_params = json!({
            "certificate": pixel_cert.certificate,
            "expected_family_id": "test-family-e2e",
            "trust_anchors": [trust_anchor],
        });
        let verify_pixel_result = handle_verify_lineage_certificate(verify_pixel_params).await?;
        let verify_pixel_resp: VerifyLineageCertificateResponse =
            serde_json::from_value(verify_pixel_result)?;

        assert!(verify_pixel_resp.valid, "Pixel certificate should verify against USB Tower");
        assert!(verify_pixel_resp.details.chain_verified);

        // ===== FINAL VERIFICATION =====
        // We now have a complete enrollment chain:
        // Genesis (root) → USB Tower → Pixel 8a
        // All with UNIQUE derived seeds (not copied!)
        println!("✅ E2E Enrollment Flow Complete:");
        println!("   Genesis seed: {}...", &genesis_seed.device_seed[..20]);
        println!("   USB seed:     {}...", &usb_seed.device_seed[..20]);
        println!("   Pixel seed:   {}...", &pixel_seed.device_seed[..20]);
        println!("   USB cert ID:  {}...", &usb_cert.certificate_id[..16]);
        println!("   Pixel cert ID:{}...", &pixel_cert.certificate_id[..16]);

        Ok(())
    }
}
