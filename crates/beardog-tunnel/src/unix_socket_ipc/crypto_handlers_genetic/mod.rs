// SPDX-License-Identifier: AGPL-3.0-only

//! Genetic Crypto RPC Handlers - Phase 5
//!
//! Handlers for genetic lineage-based cryptographic operations, organized
//! into domain-focused submodules for maintainability.
//!
//! # Submodules
//!
//! - `lineage` - Key derivation, beacon keys, verification, proofs, entropy mixing
//! - `challenge` - Dark Forest challenge-response protocol
//! - `enrollment` - Device seed derivation and lineage certificates
//!
//! # Deep Debt Evolution (Feb 9, 2026)
//!
//! Smart refactored from a single 2193-line file into domain-focused modules.
//! Each module has co-located tests for locality. Re-exports maintain backward
//! compatibility - all handler functions are accessible from `crypto_handlers_genetic::*`.

// Re-export types from the types module for backward compatibility
pub use super::crypto_handlers_genetic_types::*;

pub mod challenge;
pub mod enrollment;
pub mod lineage;

// Re-export all handler functions for backward compatibility
pub use challenge::{
    handle_generate_challenge, handle_respond_to_challenge, handle_verify_challenge_response,
};
pub use enrollment::{
    handle_derive_device_seed, handle_sign_lineage_certificate, handle_verify_lineage_certificate,
};
pub use lineage::{
    handle_derive_lineage_beacon_key, handle_derive_lineage_key, handle_generate_lineage_proof,
    handle_mix_entropy, handle_verify_lineage,
};
