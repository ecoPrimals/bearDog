//! Zero-Copy Genetic Spawning Operations - Modular Organization
//!
//! **High-Performance Genetic Processing with Minimal Allocations**
//!
//! This module provides zero-copy genetic operations that minimize memory
//! allocations and data copying during genetic spawning, recombination,
//! and analysis operations. The module has been split into logical
//! sub-modules for better maintainability.
//!
//! ## Module Organization
//!
//! - `pool` - Genetic data structure pooling and reuse
//! - `stats` - Performance and utilization metrics  
//! - `lineage` - Genetic lineage tracking
//! - `analysis` - Fitness analysis and population metrics
//! - `spawning` - Main genetic spawning engine (to be added)
//!
//! ## Key Optimizations
//!
//! - Genetic data structure pooling
//! - In-place genetic mutations
//! - Streaming genetic analysis
//! - SIMD-optimized fitness calculations
//! - Copy-on-write genetic lineage tracking

pub mod analysis;
pub mod lineage;
pub mod pool;
pub mod stats;

// Re-export commonly used types for convenience
pub use analysis::{
    CachedFitnessAnalysis, ChunkAnalysis, FitnessCache, PopulationAnalysis,
};
pub use lineage::{LineageStats, LineageTracker};
pub use pool::GeneticsPool;
pub use stats::{StatsSummary, ZeroCopyGeneticsStats};

// Import the main spawning engine from the legacy file
// (This will be moved to spawning.rs in a future refactor)
pub use super::zero_copy_spawning_legacy::ZeroCopyGeneticSpawning; 