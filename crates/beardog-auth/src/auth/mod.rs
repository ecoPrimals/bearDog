// SPDX-License-Identifier: AGPL-3.0-only

//! Authentication Module - Identity and Access Management
//!
//! Provides authentication, authorization, and consensus mechanisms for the
//! BearDog ecosystem including:
//!
//! - **Handlers**: Authentication request processing
//! - **Node Registry**: Cross-node authentication management
//! - **Proof Verification**: Cryptographic proof validation
//! - **Genetics**: Lineage-based authentication
//! - **Workflow**: Multi-step authentication flows

pub use handlers::AuthenticationHandler;
pub use types::ConsensusResult as HandlerConsensusResult;
pub use types::ConsensusResult as TypesConsensusResult;

pub use types::{authorization::*, genetics::*, node_registry::*, spawning::*, workflow::*};

mod consensus;
mod core; // CrossNodeAuthEngine implementation
mod ecosystem;
mod genetics;
mod verification;

pub mod handlers;
pub mod node_registry;
pub mod proof_verifier;
#[cfg(test)]
mod tests;
pub mod types;
