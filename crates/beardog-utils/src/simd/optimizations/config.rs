// SPDX-License-Identifier: AGPL-3.0-only

//! Configuration for safe SIMD optimization strategies

#[derive(Debug, Clone)]
pub struct SafeSimdConfig {
    /// Whether `enable_auto_vectorization` is enabled
    pub enable_auto_vectorization: bool,
    /// Whether `enable_vectorization_hints` is enabled
    pub enable_vectorization_hints: bool,
    /// Whether `prefer_iterator_chains` is enabled
    pub prefer_iterator_chains: bool,
    /// Whether `use_rayon_parallel` is enabled
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

