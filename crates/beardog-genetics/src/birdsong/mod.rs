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
//!                  │   Songbird    │
//!                  │ (Broadcasts)  │
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
//! ```rust,no_run
//! use beardog_genetics::birdsong::BirdSongManager;
//!
//! # async fn example() -> Result<(), beardog_errors::BearDogError> {
//! // Initialize BirdSong manager
//! let manager = BirdSongManager::new().await?;
//!
//! // Generate lineage for new node
//! let lineage = manager.generate_lineage("parent-id", "child-id").await?;
//!
//! // Encrypt broadcast for this lineage
//! let ciphertext = manager.encrypt_for_lineage(
//!     b"Hello family!",
//!     &lineage.lineage_hint
//! ).await?;
//!
//! // Verify and decrypt (if in lineage)
//! let plaintext = manager.decrypt_birdsong(&ciphertext, &lineage.proof).await?;
//! # Ok(())
//! # }
//! ```

pub mod encryption;
pub mod genesis;
pub mod genesis_types;
pub mod key_derivation;
pub mod lineage_chain;
pub mod lineage_proof;
pub mod manager;
pub mod types;

// Re-exports
pub use encryption::BirdSongEncryption;
pub use genesis::GenesisLineageProvider;
pub use genesis_types::{
    GenesisCeremonyResult, GeneticLineage, GenesisWitness, PhysicalChannelProof,
    PhysicalChannelType, TrustLevel,
};
pub use key_derivation::LineageKeyDerivation;
pub use lineage_chain::LineageChainManager;
pub use lineage_proof::LineageProofManager;
pub use manager::BirdSongManager;
pub use types::{
    BirdSongKey, LineageChain, LineageDepth, LineageHint, LineageNode, LineageProof,
    LineageRelationship,
};
