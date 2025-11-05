//! Statistics for safe SIMD operations

#[derive(Debug, Clone, Default)]
pub struct SafeSimdStats {
    /// Number of `operations_completed`
    pub operations_completed: u64,
    /// Number of `bytes_processed`
    pub bytes_processed: u64,
    /// Number of `parallel_operations`
    pub parallel_operations: u64,
    /// Number of `vectorized_operations`
    pub vectorized_operations: u64,
}

