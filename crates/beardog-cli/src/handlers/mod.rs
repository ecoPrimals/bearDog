// Handler module exports

pub mod birdsong; // BirdSong lineage-based encryption
pub mod cross_primal; // Cross-primal secure messaging (Workflow 3)
pub mod decrypt;
pub mod encrypt;
pub mod entropy;
pub mod hsm;
pub mod hsm_agnostic; // Key revocation
pub mod kdf; // Key derivation functions
pub mod key;
pub mod key_delegate; // Key delegation
pub mod key_derive; // Key derivation
pub mod key_export; // Key export/import (inter-primal sharing)
pub mod key_lineage; // Key lineage tracking
pub mod key_mix; // Key mixing
pub mod key_revoke;
pub mod key_store; // Key storage utilities
pub mod status;
pub mod streaming; // Streaming encryption/decryption for large files

// Test modules
#[cfg(test)]
mod entropy_tests;
#[cfg(test)]
mod hsm_tests;
#[cfg(test)]
mod status_tests;
