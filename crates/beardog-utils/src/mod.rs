// BearDog Utilities
//
// This crate provides utility functions and structures used throughout the BearDog ecosystem.
// All implementations are memory-safe and use modern Rust patterns.

// Safe implementations - production ready
pub mod buffer_pools_safe;
pub mod concurrent_safe;
pub mod memory_pools_safe;
pub mod simd_safe;
pub mod zero_copy_safe;

// Performance optimizations
pub mod performance_optimizations;
pub mod simd_crypto_acceleration;
pub mod simd_optimizations;
pub mod zero_copy;
pub mod zero_copy_optimized;

// Testing utilities
pub mod property_based_testing;
pub mod property_testing;

// Re-export safe implementations
pub use buffer_pools_safe::*;
pub use concurrent_safe::*;
pub use memory_pools_safe::*;
pub use simd_safe::*;
pub use zero_copy_safe::*;

// Performance and testing utilities
pub use performance_optimizations::*;
pub use property_based_testing::PropertyBasedTestFramework;
pub use property_testing::*;
pub use simd_crypto_acceleration::*;
pub use simd_optimizations::*;
pub use zero_copy::*;
pub use zero_copy_optimized::*;
