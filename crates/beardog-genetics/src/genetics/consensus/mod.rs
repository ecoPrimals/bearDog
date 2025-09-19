

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod types;
pub mod engine;
pub mod voting;
pub mod trust_network;
pub mod metrics;

#[cfg(test)]
mod tests;

pub use types::*;
pub use engine::GeneticConsensusEngine;
pub use voting::{VotingManager, Vote};
pub use trust_network::TrustNetwork;
pub use metrics::ConsensusMetrics; 
