//! # `BearDog` Utilities Crate
//!
//! Essential utilities and optimizations for the `BearDog` platform, providing
//! zero-copy operations, SIMD acceleration, memory pooling, and AI-powered
//! optimizations.
//!
//! ## Features
//!
//! - **Zero-Copy Operations**: Minimize memory allocations and copies
//! - **SIMD Acceleration**: Hardware-accelerated operations for performance
//! - **Memory Pooling**: Efficient buffer and memory management
//! - **Property Testing**: QuickCheck-based property testing framework
//! - **AI Optimization**: Intelligent performance optimization
//! - **100% Safe**: Zero unsafe code in all utilities

#![deny(unsafe_code)]
// Production code must use proper error handling - deny panicking methods
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
// Allow expect in tests - test panics are appropriate failure modes
#![cfg_attr(test, allow(clippy::expect_used))]
#![cfg_attr(test, allow(clippy::unwrap_used))]
//!
//! ## Core Modules
//!
//! ### Performance
//! - [`zero_copy`]: Zero-copy buffer management and operations
//! - [`simd_optimizations`]: SIMD-accelerated operations
//! - [`performance_optimizations`]: General performance patterns
//! - [`memory_pools_safe`]: Safe memory pooling
//!
//! ### Utilities
//! - [`utils`]: Common utility functions and helpers
//! - [`const_eval`]: Compile-time evaluations
//! - [`benchmarks`]: Performance benchmarking utilities
//!
//! ### Testing
//! - [`property_testing`]: Property-based testing framework
//!
//! ## Example
//!
//! BearDog utilities provide safe, high-performance operations
//! for cryptography, SIMD, zero-copy, and more.
//!
//! See module documentation for specific usage examples.
//!
//! ## Zero-Copy Patterns
//!
//! The zero-copy module provides efficient buffer management:
//!
//! ```rust,ignore
//! use beardog_utils::zero_copy::SafeZeroCopyBuffer;
//!
//! let buffer = SafeZeroCopyBuffer::new(1024);
//! // Use buffer without copies
//! ```
//!
//! ## Performance
//!
//! SIMD operations provide significant performance improvements when available.
//! All operations automatically fall back to safe scalar implementations.

pub mod ai_optimization;
pub mod benchmarks;
pub mod const_eval;
pub mod env_config;
pub mod optimization;

// Safe implementations - production ready
pub mod buffer_pools_safe;
pub mod concurrent_safe;
pub mod memory_pools_safe;
pub mod simd_safe;

/// Testing utilities for truly concurrent, deterministic tests
///
/// Provides mock time sources, event synchronization, and barriers
/// to eliminate `sleep()`-based testing patterns.
#[cfg(test)]
pub mod testing;
pub mod ultimate_performance;
pub mod ultimate_safety;
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

// Testing frameworks - canonical location
pub mod property_testing;

// Deprecated modules - for backward compatibility
#[deprecated(since = "3.0.2", note = "Use property_testing module instead")]
pub mod property_based_testing;

// Export safe implementations by default - specific imports to avoid ambiguity
pub use buffer_pools_safe::{PoolStats as BufferPoolStats, SafeBufferPool as BufferPoolSafe};
pub use concurrent_safe::*;
pub use memory_pools_safe::{PoolStats as MemoryPoolStats, SafeMemoryPool};
pub use simd_safe::*;
pub use utils::*;
pub use zero_copy_safe::*;

// Test modules
#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod tests;

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
#[path = "tests/ultimate_safety_comprehensive.rs"]
mod ultimate_safety_comprehensive_tests;

#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod performance_optimizations_tests;
#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
#[path = "tests/ultimate_performance_comprehensive.rs"]
mod ultimate_performance_comprehensive_tests;
#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod ultimate_performance_tests;
#[allow(unused_imports, clippy::nonminimal_bool, dead_code)]
#[cfg(test)]
mod ultimate_safety_tests;
