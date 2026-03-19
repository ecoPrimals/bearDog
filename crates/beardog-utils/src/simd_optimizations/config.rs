// SPDX-License-Identifier: AGPL-3.0-only

//! Configuration for safe SIMD optimization strategies

/// Configuration for safe SIMD optimization strategies
///
/// Controls which optimization techniques are enabled for data processing,
/// allowing fine-tuning of performance vs. compatibility trade-offs.
#[derive(Debug, Clone)]
pub struct SafeSimdConfig {
    /// Enable compiler auto-vectorization hints for hot loops
    pub enable_auto_vectorization: bool,
    /// Provide explicit vectorization hints to the compiler via iterator patterns
    pub enable_vectorization_hints: bool,
    /// Use chained iterators for better compiler optimization opportunities
    pub prefer_iterator_chains: bool,
    /// Enable Rayon-based parallel processing for large datasets
    pub use_rayon_parallel: bool,
}

impl Default for SafeSimdConfig {
    fn default() -> Self {
        Self {
            enable_auto_vectorization: true,
            enable_vectorization_hints: true,
            prefer_iterator_chains: true,
            use_rayon_parallel: true,
        }
    }
}
