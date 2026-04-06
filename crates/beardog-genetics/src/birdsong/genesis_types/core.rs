// SPDX-License-Identifier: AGPL-3.0-or-later

//! Primary genesis structs and serde-backed field definitions.

use serde::{Deserialize, Serialize};

use super::PhysicalChannelType;
use super::TrustLevel;

/// A device that witnesses the birth of a new node
///
/// During a genesis ceremony, a witness device (e.g., `SoloKey`, existing node)
/// cryptographically signs the creation of a new node's lineage, establishing
/// trust through physical proximity rather than network infrastructure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisWitness {
    /// Witness device identifier (e.g., "solokey-abc123")
    pub device_id: String,

    /// Witness public key (Ed25519, 32 bytes)
    pub public_key: Vec<u8>,

    /// Physical channel used for witness ceremony
    pub physical_channel: PhysicalChannelType,

    /// Timestamp of genesis ceremony (Unix timestamp, seconds since epoch)
    pub timestamp: u64,

    /// Signature over new node's identity
    /// Signs: `new_node_id` || timestamp || `public_key`
    pub signature: Vec<u8>,
}

/// Genetic cryptographic lineage for a node
///
/// Established during genesis ceremony, this lineage proves the node's
/// ancestry and provides the foundation for deriving cryptographic keys.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticLineage {
    /// Node's genetic identity (derived from parent lineage + new node ID)
    pub genetic_id: Vec<u8>,

    /// Lineage chain from genesis witness to this node
    pub lineage_chain: crate::birdsong::LineageChain,

    /// Genesis witness who created this lineage
    pub genesis_witness: GenesisWitness,

    /// Birth timestamp (Unix timestamp, seconds since epoch)
    pub birth_timestamp: u64,

    /// Trust level derived from physical channel
    pub trust_level: TrustLevel,
}

/// Physical proximity proof for genesis ceremony
///
/// Additional attestation data depending on physical channel type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicalChannelProof {
    /// Channel type
    pub channel_type: PhysicalChannelType,

    /// Hardware attestation (for `HardwareKey`, Nfc)
    pub attestation: Option<Vec<u8>>,

    /// Out-of-band verification codes (for `QrCodeWithOob`)
    pub verification_codes: Option<Vec<String>>,

    /// Bluetooth pairing data (for Bluetooth)
    pub pairing_data: Option<Vec<u8>>,

    /// Timestamp when proof was generated
    pub timestamp: u64,
}

/// Genesis ceremony result
///
/// Contains the newly created genetic lineage and verification information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisCeremonyResult {
    /// Newly created genetic lineage
    pub genetic_lineage: GeneticLineage,

    /// Physical channel proof
    pub physical_proof: PhysicalChannelProof,

    /// Ceremony completion timestamp
    pub completed_at: u64,

    /// Success status
    pub success: bool,

    /// Error message if failed
    pub error: Option<String>,
}
