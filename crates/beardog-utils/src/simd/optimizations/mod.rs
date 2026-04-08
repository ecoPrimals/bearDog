// SPDX-License-Identifier: AGPL-3.0-or-later

//! SIMD optimizations module
//
// This module provides functionality for the BearDog ecosystem.

// Re-export core types for convenience
pub use config::SafeSimdConfig;
pub use stats::SafeSimdStats;
pub use optimizer::SafeSimdOptimizer;
pub use advanced::{AdvancedSIMDOptimizer, AlignedBuffer, SIMDMetrics, SIMDOperation, SharedSIMDOptimizer};

// Submodules
mod config;
mod stats;
mod optimizer;
pub mod safe_utils;
mod advanced;

#[cfg(test)]
mod tests;

