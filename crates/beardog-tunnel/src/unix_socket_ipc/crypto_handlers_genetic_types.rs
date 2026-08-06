// SPDX-License-Identifier: AGPL-3.0-or-later

//! Genetic Crypto RPC Types
//!
//! Request and response types for genetic lineage-based cryptographic operations.
//! Split from `crypto_handlers_genetic.rs` for maintainability (Deep Debt principle).
//!
//! # Types
//!
//! - Key derivation: `DeriveLineageKeyRequest`, `DeriveLineageKeyResponse`
//! - Entropy mixing: `MixEntropyRequest`, `MixEntropyResponse`
//! - Lineage verification: `VerifyLineageRequest`, `VerifyLineageResponse`
//! - Proof generation: `GenerateLineageProofRequest`, `GenerateLineageProofResponse`
//! - Challenge-response: `GenerateChallengeRequest`, `GenerateChallengeResponse`,
//!   `RespondToChallengeRequest`, `RespondToChallengeResponse`,
//!   `VerifyChallengeResponseRequest`, `VerifyChallengeResponseResponse`
//! - Device certificates: `SignLineageCertificateRequest`, `SignLineageCertificateResponse`,
//!   `VerifyLineageCertificateRequest`, `VerifyLineageCertificateResponse`
//! - Device seed derivation: `DeriveDeviceSeedRequest`, `DeriveDeviceSeedResponse`

use serde::{Deserialize, Serialize};

// ============================================================================
// KEY DERIVATION TYPES
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

/// Request to derive lineage beacon key (Dark Forest)
#[derive(Debug, Deserialize)]
pub struct DeriveLineageBeaconKeyRequest {
    /// Our genetic family ID
    pub our_family_id: String,
    /// Peer's genetic family ID
    pub peer_family_id: String,
    /// Beacon key (base64-encoded, for meeting-based encryption)
    pub beacon_key: String,
    /// Context data (purpose, session ID, etc.)
    pub context: String,
    /// Lineage seed (base64-encoded)
    pub lineage_seed: String,
}

/// Response containing the derived beacon key
#[derive(Debug, Serialize, Deserialize)]
pub struct DeriveLineageBeaconKeyResponse {
    /// Derived key (base64-encoded, 32 bytes)
    pub key: String,
    /// Key derivation method used
    pub method: String,
    /// Quality score (0.0-1.0)
    pub quality_score: f64,
}

// ============================================================================
// ENTROPY MIXING TYPES
// ============================================================================

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

/// Response containing mixed entropy with provenance metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct MixEntropyResponse {
    /// Mixed entropy (base64-encoded, 32 bytes)
    pub entropy: String,
    /// Quality score (0.0-1.0, weighted by source provenance)
    pub quality_score: f64,
    /// Number of tiers used
    pub tiers_used: u8,
    /// Which entropy sources contributed to this mix
    pub provenance: EntropyProvenance,
}

/// Provenance metadata for mixed entropy — records which sources contributed.
#[derive(Debug, Serialize, Deserialize)]
pub struct EntropyProvenance {
    /// Tier 3 (human lived experience) was provided
    pub has_human: bool,
    /// Tier 2 (human-supervised machine) was provided
    pub has_supervised: bool,
    /// Tier 1 (machine / OS RNG) was provided or auto-generated
    pub has_machine: bool,
    /// Whether machine entropy was explicitly provided vs auto-generated from OS RNG
    pub machine_explicit: bool,
}

// ============================================================================
// LINEAGE VERIFICATION TYPES
// ============================================================================

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
    /// Chain ID for full chain-based verification via `LineageProofManager`.
    /// When present (along with structured proof JSON), the handler uses the
    /// generation-aware Merkle verification path instead of the simple Blake3
    /// hash check.
    #[serde(default)]
    pub chain_id: Option<String>,
}

/// Response containing lineage verification result
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyLineageResponse {
    /// Whether lineage is valid
    pub valid: bool,
    /// Optional failure reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Verified depth in the lineage chain (only set for chain-based proofs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
    /// Generation counter from the chain (only set for chain-based proofs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<u64>,
}

// ============================================================================
// PROOF GENERATION TYPES
// ============================================================================

/// Request to generate lineage proof
#[derive(Debug, Deserialize)]
pub struct GenerateLineageProofRequest {
    /// Our genetic family ID
    pub our_family_id: String,
    /// Peer's genetic family ID
    pub peer_family_id: String,
    /// Lineage seed (base64-encoded)
    pub lineage_seed: String,
    /// Chain ID for full chain-based proof via `LineageProofManager`.
    /// When present with `node_id`, the handler generates a structured
    /// Merkle-path proof with generation tracking and head commitment.
    #[serde(default)]
    pub chain_id: Option<String>,
    /// Node ID within the chain (required when `chain_id` is set).
    #[serde(default)]
    pub node_id: Option<String>,
}

/// Response containing generated lineage proof
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateLineageProofResponse {
    /// Lineage proof (base64-encoded Blake3 hash for simple proofs,
    /// or JSON-serialized `LineageProof` for chain-based proofs).
    pub proof: String,
    /// Proof generation timestamp
    pub timestamp: u64,
    /// Generation counter (only set for chain-based proofs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<u64>,
    /// Head commitment hex (only set for chain-based proofs).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head_commitment: Option<String>,
    /// Proof mode: `"simple"` (Blake3 hash) or `"chain"` (full Merkle proof).
    #[serde(default = "default_proof_mode")]
    pub mode: String,
}

fn default_proof_mode() -> String {
    "simple".to_string()
}

// ============================================================================
// CHALLENGE-RESPONSE TYPES
// ============================================================================

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
// DEVICE SEED DERIVATION TYPES (Deep Debt: DERIVE, not COPY)
// ============================================================================

/// Request to derive a unique device seed from root genesis seed
///
/// This is the CORRECT approach: each device gets a DERIVED seed,
/// not a COPIED seed. If one device is compromised, others remain secure.
///
/// # Deep Debt Principle
///
/// ```text
/// Genesis:    root_seed = 8ff3b864a4bc589a... (stored encrypted, rarely accessed)
///                         ↓
/// USB Tower:  device_seed = DERIVE(root_seed, device_entropy) = a1b2c3d4...
/// Pixel:      device_seed = DERIVE(root_seed, pixel_entropy) = e5f6g7h8...
/// ```
#[derive(Debug, Deserialize)]
pub struct DeriveDeviceSeedRequest {
    /// Root genesis seed (base64-encoded, 32 bytes)
    /// This is the family's master seed - should be stored encrypted
    pub root_seed: String,
    /// Device-specific entropy (base64-encoded, 32 bytes)
    /// Should include hardware identifiers, device attestation, etc.
    pub device_entropy: String,
    /// Device identifier (e.g., "usb-desktop", "pixel8a")
    pub device_id: String,
    /// Optional enrollment timestamp (Unix seconds, for reproducibility)
    #[serde(default)]
    pub enrollment_timestamp: Option<u64>,
}

/// Response containing the derived device seed
#[derive(Debug, Serialize, Deserialize)]
pub struct DeriveDeviceSeedResponse {
    /// Derived device seed (base64-encoded, 32 bytes)
    /// This is the UNIQUE seed for this device
    pub device_seed: String,
    /// Device ID (echoed back)
    pub device_id: String,
    /// Key derivation function used
    pub kdf: String,
    /// Domain separation used
    pub domain: String,
    /// Proof that this seed was derived from the root (for verification)
    /// This is a commitment: `HMAC(root_seed`, `device_seed` || `device_id`)
    pub derivation_proof: String,
}

// ============================================================================
// LINEAGE CERTIFICATE TYPES (Device Enrollment)
// ============================================================================

/// Request to sign a lineage certificate for device enrollment
///
/// When a new device joins the family, the enrolling device (parent)
/// signs a certificate proving the new device's lineage.
#[derive(Debug, Deserialize)]
pub struct SignLineageCertificateRequest {
    /// Parent device's seed (base64-encoded, 32 bytes)
    /// The enrolling device's derived seed
    pub parent_seed: String,
    /// Parent device ID
    pub parent_device_id: String,
    /// Child device's public key (base64-encoded Ed25519 public key, 32 bytes)
    pub child_public_key: String,
    /// Child device ID
    pub child_device_id: String,
    /// Family ID (shared across all family members)
    pub family_id: String,
    /// Optional: Child's derived seed proof (to verify derivation)
    #[serde(default)]
    pub child_derivation_proof: Option<String>,
    /// Optional: Expiration timestamp (Unix seconds)
    #[serde(default)]
    pub expires_at: Option<u64>,
}

/// A signed lineage certificate proving device enrollment
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LineageCertificate {
    /// Certificate version
    pub version: u8,
    /// Parent device ID (enrolling device)
    pub parent_device_id: String,
    /// Child device ID (enrolled device)
    pub child_device_id: String,
    /// Child's public key (base64-encoded Ed25519)
    pub child_public_key: String,
    /// Family ID
    pub family_id: String,
    /// Certificate issuance timestamp (Unix seconds)
    pub issued_at: u64,
    /// Certificate expiration timestamp (Unix seconds, optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<u64>,
    /// Depth in lineage tree (`parent_depth` + 1)
    pub depth: u32,
    /// Parent's signature of the certificate (base64-encoded Ed25519 signature)
    /// Signs: version || `parent_id` || `child_id` || `child_pubkey` || `family_id` || `issued_at` || depth
    pub parent_signature: String,
    /// Parent's public key (base64-encoded Ed25519 public key, for verification)
    pub parent_public_key: String,
}

/// Response containing the signed certificate
#[derive(Debug, Serialize, Deserialize)]
pub struct SignLineageCertificateResponse {
    /// The signed lineage certificate
    pub certificate: LineageCertificate,
    /// Certificate ID (SHA256 of certificate data, hex-encoded)
    pub certificate_id: String,
}

/// Request to verify a lineage certificate
#[derive(Debug, Deserialize)]
pub struct VerifyLineageCertificateRequest {
    /// The certificate to verify
    pub certificate: LineageCertificate,
    /// Optional: Expected family ID (for additional validation)
    #[serde(default)]
    pub expected_family_id: Option<String>,
    /// Optional: Root certificate(s) for chain verification
    #[serde(default)]
    pub trust_anchors: Vec<LineageCertificate>,
}

/// Response containing certificate verification result
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifyLineageCertificateResponse {
    /// Whether the certificate is valid
    pub valid: bool,
    /// Verification details
    pub details: CertificateVerificationDetails,
}

/// Details about certificate verification
#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateVerificationDetails {
    /// Signature is valid
    pub signature_valid: bool,
    /// Certificate is not expired
    pub not_expired: bool,
    /// Family ID matches expected (if provided)
    pub family_id_matches: bool,
    /// Full chain verified (if trust anchors provided)
    pub chain_verified: bool,
    /// Chain depth
    pub chain_depth: u32,
    /// Failure reason (if invalid)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
}
