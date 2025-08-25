// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Zero-Copy Genetic Spawning Operations - Modular Organization
///
/// **High-Performance Genetic Processing with Minimal Allocations**
/// This module provides zero-copy genetic operations that minimize memory
/// allocations and data copying during genetic spawning, recombination,
/// and analysis operations. The module has been split into logical
/// sub-modules for better maintainability.
/// ## Module Organization
/// - `pool` - Genetic data structure pooling and reuse
/// - `stats` - Performance and utilization metrics  
/// - `lineage` - Genetic lineage tracking
/// - `analysis` - Fitness analysis and population metrics
/// - `spawning` - Main genetic spawning engine (to be added)
/// ## Key Optimizations
/// - Genetic data structure pooling
/// - In-place genetic mutations
/// - Streaming genetic analysis
/// - SIMD-optimized fitness calculations
/// - Copy-on-write genetic lineage tracking

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
// Use the modular spawning implementation
pub use spawning::ZeroCopyGeneticSpawning; 
