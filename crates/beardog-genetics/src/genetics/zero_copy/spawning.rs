//! Core Zero-Copy Genetic Spawning Implementation
//!
//! Provides the main ZeroCopyGeneticSpawning engine functionality.

pub use crate::genetics::zero_copy_spawning_legacy::{
    ZeroCopyGeneticSpawning, ZeroCopyGeneticsStats,
};

// Re-export for convenience
pub use ZeroCopyGeneticSpawning as ZeroCopySpawning;
