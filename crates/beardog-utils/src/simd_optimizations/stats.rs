// SPDX-License-Identifier: AGPL-3.0-only

//! Statistics for safe SIMD operations

/// Statistics for safe SIMD operations
///
/// Tracks performance metrics for monitoring and tuning of SIMD-optimized operations.
#[derive(Debug, Clone, Default)]
pub struct SafeSimdStats {
    /// Total number of processing operations completed successfully
    pub operations_completed: u64,
    /// Total number of bytes processed across all operations
    pub bytes_processed: u64,
    /// Number of operations that used parallel processing (Rayon)
    pub parallel_operations: u64,
    /// Number of operations that benefited from compiler vectorization
    pub vectorized_operations: u64,
}
