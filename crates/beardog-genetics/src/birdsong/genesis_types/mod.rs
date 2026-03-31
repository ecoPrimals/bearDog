// SPDX-License-Identifier: AGPL-3.0-only

//! Genesis Bootstrap Types
//!
//! Core data structures for physical genesis ceremonies where new nodes
//! receive cryptographic identity via witnessed birth.
//!
//! ## Type Sources
//!
//! - `PhysicalChannelType`, `TrustLevel`: Re-exported from `beardog-security`
//! - `GenesisWitness`: Extended from `beardog-security` with signature verification
//! - `GeneticLineage`, `PhysicalChannelProof`, `GenesisCeremonyResult`: Defined here

mod core;
mod genetic_lineage;
mod physical_proof;
mod witness;

// Re-export core genesis types from beardog-security (single source of truth)
pub use beardog_security::genesis::{PhysicalChannelType, TrustLevel};

pub use core::{GenesisCeremonyResult, GenesisWitness, GeneticLineage, PhysicalChannelProof};

#[cfg(test)]
#[path = "genesis_types_tests.rs"]
mod tests;
