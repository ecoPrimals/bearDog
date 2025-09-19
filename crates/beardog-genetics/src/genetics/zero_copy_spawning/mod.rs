

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod analysis;
pub mod lineage;
pub mod pool;
pub mod stats;

pub use analysis::{
    CachedFitnessAnalysis, ChunkAnalysis, FitnessCache, PopulationAnalysis,
};
pub use lineage::{LineageStats, LineageTracker};
pub use pool::GeneticsPool;
pub use stats::{StatsSummary, ZeroCopyGeneticsStats};

pub use spawning::ZeroCopyGeneticSpawning; 
