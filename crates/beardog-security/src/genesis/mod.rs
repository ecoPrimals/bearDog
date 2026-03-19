// SPDX-License-Identifier: AGPL-3.0-only

//! # Genesis Module - Physical Bootstrap with Cryptographic Witness
//!
//! **"Never let a bird be alone in the dark forest"**
//!
//! This module implements physical genesis bootstrap for new nodes, ensuring they
//! receive cryptographic identity at birth via witnessed ceremony, not vulnerable
//! internet bootstrap.
//!
//! ## Architecture
//!
//! Traditional P2P bootstrap ❌:
//! ```text
//! New Node → Internet → Find server → Hope it's safe
//! ```
//!
//! Physical Genesis ✅:
//! ```text
//! New Node → SoloKey tap → Witnessed genesis → Multi-primal lineage
//!          ✅ Protected from first moment
//!          ✅ Born with cryptographic identity
//!          ✅ Never alone, never vulnerable
//! ```
//!
//! ## Core Concepts
//!
//! - **Genesis Witness**: A trusted device (SoloKey, YubiKey) that witnesses a new node's birth
//! - **Physical Channel**: The physical medium used for genesis (hardware key, QR, NFC, Bluetooth)
//! - **Trust Level**: Security rating based on physical channel type
//! - **Witness Verification**: Cryptographic verification of witness authority
//!
//! ## Modules
//!
//! - [`witness`] - Witness verification and signature checking
//! - [`physical_proof`] - Physical channel attestation and trust levels
//! - [`types`] - Core types for genesis ceremony

pub mod physical_proof;
pub mod types;
pub mod witness;

// Re-exports for ergonomic API
pub use physical_proof::{PhysicalProofError, PhysicalProximityVerifier};
pub use types::{PhysicalChannelType, TrustLevel};
pub use witness::{GenesisWitness, GenesisWitnessVerifier, WitnessVerificationError};

#[cfg(test)]
mod tests;
