//! Safe SIMD optimizations using compiler auto-vectorization
//!
//! This module provides high-performance data processing using only safe Rust,
//! leveraging compiler auto-vectorization, iterator chains, and parallel processing
//! without any unsafe SIMD intrinsics.
//!
//! # Features
//!
//! - **Zero Unsafe Code**: All optimizations use safe Rust constructs
//! - **Auto-Vectorization**: Compiler automatically vectorizes hot loops
//! - **Parallel Processing**: Optional Rayon-based parallelism
//! - **Iterator Chains**: Efficient functional-style data transformation
//!
//! # Example
//!
//! ```rust,ignore
//! use beardog_utils::simd_optimizations::{SafeSimdOptimizer, SafeSimdConfig};
//!
//! let mut optimizer = SafeSimdOptimizer::new(SafeSimdConfig::default());
//! let data = vec![1u8, 2, 3, 4, 5];
//! let result = optimizer.safe_parallel_process(&data, |x| x * 2)?;
//! ```

// Re-export core types for convenience
pub use advanced::{
    AdvancedSIMDOptimizer, AlignedBuffer, SIMDMetrics, SIMDOperation, SharedSIMDOptimizer,
};
pub use config::SafeSimdConfig;
pub use optimizer::SafeSimdOptimizer;
pub use stats::SafeSimdStats;

// Submodules
mod advanced;
mod config;
mod optimizer;
pub mod safe_utils;
mod stats;

#[cfg(test)]
mod tests;
