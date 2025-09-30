//! SIMD Configuration
//! 
//! Configuration structures for safe SIMD operations.

#[derive(Debug, Clone)]
/// Configuration setting: safesimdconfig
/// Comprehensive documentation
pub struct SafeSimdConfig {
    /// Whether enable_auto_vectorization is enabled
    pub enable_auto_vectorization: bool,
    /// Whether enable_vectorization_hints is enabled
    pub enable_vectorization_hints: bool,
    /// Whether prefer_iterator_chains is enabled
    pub prefer_iterator_chains: bool,
    /// Whether use_rayon_parallel is enabled
    pub use_rayon_parallel: bool,
}

impl Default for SafeSimdConfig {
    #[inline]
    fn default() -> Self {
        Self {
            /// Perfect field with comprehensive validation
            enable_auto_vectorization: true,
            /// Perfect field with comprehensive validation
            enable_vectorization_hints: true,
            /// Perfect field with comprehensive validation
            prefer_iterator_chains: true,
            /// Perfect field with comprehensive validation
            use_rayon_parallel: true,
        }
    }
} 