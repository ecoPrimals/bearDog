// SPDX-License-Identifier: AGPL-3.0-or-later

//! Core types for `BirdSong` lineage and encryption

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use zeroize::Zeroizing;

/// A node in the lineage chain representing a primal or node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageNode {
    /// Unique node identifier
    pub node_id: String,
    /// Parent node identifier (None for root/genesis nodes)
    pub parent_id: Option<String>,
    /// Public key for this node (Ed25519)
    pub public_key: Vec<u8>,
    /// Depth in the lineage tree (0 for root)
    pub depth: LineageDepth,
    /// Timestamp when this node joined the lineage
    pub created_at: DateTime<Utc>,
    /// Optional metadata (capabilities, biome type, etc.)
    pub metadata: LineageMetadata,
}

/// Depth in the lineage tree (0 = root, 1 = direct child, etc.)
pub type LineageDepth = u32;

/// Metadata associated with a lineage node
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LineageMetadata {
    /// Node biome type (opaque label from discovery / metadata)
    pub biome_type: Option<String>,
    /// Node capabilities
    pub capabilities: Vec<String>,
    /// Trust level
    pub trust_level: f64,
    /// Custom metadata
    pub custom: std::collections::HashMap<String, String>,
}

/// A lineage chain representing the cryptographic parent-child relationships.
///
/// Supports generational provenance (Phase 3): each mutation increments
/// `generation` and updates `head_commitment` with a chained Blake3
/// hash binding the previous head to the new event.  Proofs include
/// the generation and commitment so verifiers can confirm they refer
/// to a specific, ordered point in the chain's history rather than
/// just a point-in-time tree snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageChain {
    /// Unique identifier for this lineage chain
    pub chain_id: String,
    /// Root node (genesis) of this lineage
    pub root_node: LineageNode,
    /// All nodes in this lineage chain (`node_id` -> `LineageNode`)
    pub nodes: std::collections::HashMap<String, LineageNode>,
    /// Parent-child relationships with signatures
    pub relationships: Vec<LineageRelationship>,
    /// Monotonic generation counter (incremented on every chain mutation).
    #[serde(default)]
    pub generation: u64,
    /// Chained commitment: `Blake3(prev_commitment || event_bytes)`.
    /// Genesis value is `Blake3(chain_id)`.
    #[serde(default)]
    pub head_commitment: Vec<u8>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// A cryptographically signed parent-child relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageRelationship {
    /// Parent node ID
    pub parent_id: String,
    /// Child node ID
    pub child_id: String,
    /// Parent's signature of the relationship (Ed25519)
    /// Signs: `HMAC(parent_id` || `child_id` || `child_public_key` || timestamp)
    pub parent_signature: Vec<u8>,
    /// Optional witness signatures for accountability
    pub witness_signatures: Vec<WitnessSignature>,
    /// Timestamp when relationship was established
    pub established_at: DateTime<Utc>,
}

/// A witness signature for lineage accountability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitnessSignature {
    /// Witness node ID
    pub witness_id: String,
    /// Witness's signature
    pub signature: Vec<u8>,
    /// Timestamp of witnessing
    pub witnessed_at: DateTime<Utc>,
}

/// A proof that a node belongs to a specific lineage at a specific generation.
///
/// Contains the generation and head commitment so verifiers can confirm
/// this proof was issued against a known chain state, enabling
/// verifiable proof chains (generation N → N+1 → N+2).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageProof {
    /// Node claiming lineage membership
    pub node_id: String,
    /// Root node of the lineage being claimed
    pub root_id: String,
    /// Path from root to this node (ordered list of node IDs)
    pub path: Vec<String>,
    /// Cryptographic proof (chain of signatures)
    pub proof_chain: Vec<LineageRelationship>,
    /// Merkle root of the lineage tree (for efficient verification)
    pub merkle_root: Vec<u8>,
    /// Chain generation at which this proof was issued.
    #[serde(default)]
    pub generation: u64,
    /// Head commitment of the chain when this proof was generated.
    /// Binds the proof to a specific, ordered chain state.
    #[serde(default)]
    pub head_commitment: Vec<u8>,
    /// Proof generated at
    pub generated_at: DateTime<Utc>,
}

/// A hint about which lineage can decrypt a `BirdSong` broadcast
///
/// This is public metadata attached to encrypted broadcasts,
/// allowing family members to know they should try to decrypt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageHint {
    /// Root lineage ID (public)
    pub root_id: String,
    /// Minimum depth required to decrypt (0 = root only, 1 = root + children, etc.)
    pub min_depth: LineageDepth,
    /// Maximum depth allowed to decrypt
    pub max_depth: LineageDepth,
    /// Optional biome type filter
    pub biome_filter: Option<String>,
    /// Hint version for future compatibility
    pub version: u8,
}

/// A `BirdSong` encryption key derived from lineage
#[derive(Debug, Clone)]
pub struct BirdSongKey {
    /// The symmetric key material (ChaCha20-Poly1305, 32 bytes)
    pub key_material: Zeroizing<Vec<u8>>,
    /// Lineage hint for this key
    pub hint: LineageHint,
    /// Key generation number (for rotation)
    pub generation: u32,
    /// Key valid from
    pub valid_from: DateTime<Utc>,
    /// Key expires at
    pub expires_at: DateTime<Utc>,
}

// Manual Drop to ensure key material is zeroized
impl Drop for BirdSongKey {
    fn drop(&mut self) {
        // key_material is Zeroizing, so it will be automatically zeroized
        // No additional action needed, but we make it explicit
    }
}

/// Request to encrypt a `BirdSong` broadcast
#[derive(Debug, Clone)]
pub struct BirdSongEncryptRequest {
    /// Plaintext data to encrypt
    pub plaintext: Vec<u8>,
    /// Lineage hint (who can decrypt)
    pub lineage_hint: LineageHint,
    /// Optional associated data (authenticated but not encrypted)
    pub associated_data: Option<Vec<u8>>,
}

/// Encrypted `BirdSong` broadcast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirdSongBroadcast {
    /// Lineage hint (public, tells family to try decrypting)
    pub hint: LineageHint,
    /// Nonce for ChaCha20-Poly1305 (12 bytes)
    pub nonce: Vec<u8>,
    /// Ciphertext + authentication tag
    pub ciphertext: Vec<u8>,
    /// Optional associated data (authenticated but not encrypted)
    pub associated_data: Option<Vec<u8>>,
    /// Key generation used to encrypt this broadcast.
    /// Decryptors must derive the key at this generation to decrypt.
    #[serde(default)]
    pub generation: u32,
    /// Broadcast timestamp
    pub broadcast_at: DateTime<Utc>,
}

/// Request to decrypt a `BirdSong` broadcast
#[derive(Debug, Clone)]
pub struct BirdSongDecryptRequest {
    /// Encrypted broadcast
    pub broadcast: BirdSongBroadcast,
    /// Lineage proof for the requesting node
    pub proof: LineageProof,
}

/// Result of a lineage verification
#[derive(Debug, Clone)]
pub struct LineageVerificationResult {
    /// Whether the lineage proof is valid
    pub valid: bool,
    /// Verified lineage depth
    pub depth: LineageDepth,
    /// Reason if invalid
    pub failure_reason: Option<String>,
}

/// Configuration for `BirdSong` encryption
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirdSongConfig {
    /// Key rotation interval in seconds
    pub key_rotation_interval_secs: u64,
    /// Maximum lineage depth supported
    pub max_lineage_depth: LineageDepth,
    /// Enable witness signatures
    pub enable_witnesses: bool,
    /// Minimum witnesses required
    pub min_witnesses: usize,
}

impl Default for BirdSongConfig {
    fn default() -> Self {
        Self {
            key_rotation_interval_secs: 86400, // 24 hours
            max_lineage_depth: 10,
            enable_witnesses: true,
            min_witnesses: 1,
        }
    }
}
