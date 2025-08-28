//! ToadStool Client Integration
//! 
//! This module provides the main interface to ToadStool compute orchestration services.

pub use crate::ecosystem_integration::toadstool_client::*;

// Re-export key types for convenience
pub use crate::ecosystem_integration::ecosystem_genetic_spawner::{
    EcosystemPrimalClient, GeneticTrait, TraitCategory, EcosystemCapability,
    ComputeResourceAllocation, EcosystemGeneticBlueprint,
}; 