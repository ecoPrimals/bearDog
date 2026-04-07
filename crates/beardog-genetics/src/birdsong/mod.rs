// SPDX-License-Identifier: AGPL-3.0-or-later

//! BirdSong Lineage & Encryption System
//!
//! Privacy-preserving federation through lineage-based broadcast encryption.
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │              BirdSong Lineage System                    │
//! ├─────────────────────────────────────────────────────────┤
//! │  ┌──────────────┐  ┌─────────────┐  ┌───────────────┐  │
//! │  │ Lineage      │  │ Lineage     │  │  BirdSong     │  │
//! │  │ Chain        │→ │ Proofs      │→ │  Encryption   │  │
//! │  │ (parent→child)  (verify)         (family keys)   │  │
//! │  └──────────────┘  └─────────────┘  └───────────────┘  │
//! └─────────────────────────────────────────────────────────┘
//!                          ▲
//!                          │
//!                 BirdSong Protocol
//!                          │
//!                          ▼
//!                  ┌───────────────┐
//!                  │  Broadcaster  │
//!                  │  (Mesh peer)  │
//!                  └───────────────┘
//! ```
//!
//! ## Key Concepts
//!
//! - **Lineage Chain**: Cryptographic proof of parent-child relationships
//! - **Lineage Proof**: Verifiable proof that a node belongs to a family tree
//! - **BirdSong Encryption**: Broadcast encryption where only family can decrypt
//! - **Key Derivation**: HKDF-based key generation from lineage hierarchy
//!
//! ## Example
//!
//! ```rust,ignore
//! use beardog_genetics::birdsong::{BirdSongManager, BirdSongConfig};
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! let secret = b"family-master-secret".to_vec();
//! let manager = BirdSongManager::new(secret, None).await?;
//!
//! // Encrypt a broadcast payload
//! let ciphertext = manager.encrypt(b"Hello family!").await?;
//!
//! // Decrypt (if in lineage)
//! let plaintext = manager.decrypt(&ciphertext).await?;
//! # Ok(())
//! # }
//! ```

// Dark Forest Beacon Genetics (Phase 1 - Feb 2026)
pub mod beacon_seed;

// Lineage & Encryption (Existing)
pub mod encryption;
pub mod genesis;
pub mod genesis_types;
pub mod key_derivation;
pub mod lineage_chain;
pub mod lineage_id;
pub mod lineage_proof;
pub mod manager;
pub mod types;

// Re-exports - Beacon (Dark Forest)
pub use beacon_seed::{BeaconCiphertext, BeaconId, BeaconSeed};

// Re-exports - Lineage (Existing)
pub use encryption::BirdSongEncryption;
pub use genesis::GenesisLineageProvider;
pub use genesis_types::{
    GenesisCeremonyResult, GenesisWitness, GeneticLineage, PhysicalChannelProof,
    PhysicalChannelType, TrustLevel,
};
pub use key_derivation::LineageKeyDerivation;
pub use lineage_chain::LineageChainManager;
pub use lineage_id::LineageID;
pub use lineage_proof::LineageProofManager;
pub use manager::BirdSongManager;
pub use types::{
    BirdSongKey, LineageChain, LineageDepth, LineageHint, LineageNode, LineageProof,
    LineageRelationship,
};
