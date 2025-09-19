// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod ai_optimization;
pub mod benchmarks;
pub mod const_eval;

// Safe implementations - production ready
pub mod buffer_pools_safe;
pub mod concurrent_safe;
pub mod memory_pools_safe;
pub mod simd_safe;
pub mod zero_copy_safe;

// Performance and utility modules
pub mod performance_optimizations;
pub mod simd_crypto_acceleration;
pub mod simd_optimizations;
/// Utility functions and helpers
/// Utility functions and helpers
pub mod utils;
pub mod zero_copy;
pub mod zero_copy_optimized;

// Export safe implementations by default - specific imports to avoid ambiguity
pub use buffer_pools_safe::{PoolStats as BufferPoolStats, SafeBufferPool as BufferPoolSafe};
pub use concurrent_safe::*;
pub use memory_pools_safe::{PoolStats as MemoryPoolStats, SafeMemoryPool};
pub use simd_safe::*;
pub use utils::*;
pub use zero_copy_safe::*;
