// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub use handlers::AuthenticationHandler;
pub use types::ConsensusResult as HandlerConsensusResult;
pub use types::ConsensusResult as TypesConsensusResult;

pub use types::{authorization::*, genetics::*, node_registry::*, spawning::*, workflow::*};

pub mod handlers;
pub mod node_registry;
pub mod proof_verifier;
#[cfg(test)]
mod tests;
pub mod types;
