// SPDX-License-Identifier: AGPL-3.0-only

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
//! - **100% Safe**: Fully memory-safe in all utilities

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
//! - `property_testing`: Property-based testing framework (test / `test-utils` only)
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

mod crypto_safe_accel;

/// Experimental hooks for workload-aware tuning and simple learning-style optimizers.
pub mod ai_optimization;
/// Micro-benchmark helpers used across Beardog utility code.
pub mod benchmarks;
/// Compile-time evaluation helpers (constants and static configuration).
pub mod const_eval;
/// Environment-driven configuration helpers (paths, flags, safe parsing).
pub mod env_config;
/// Higher-level composition of utility building blocks (clone interning, string patterns, etc.).
pub mod optimization;

// Safe implementations - production ready
/// Pre-allocated byte buffers with accounting suitable for hot paths.
pub mod buffer_pools_safe;
/// Concurrency primitives and patterns built on `parking_lot` / `crossbeam`.
pub mod concurrent_safe;
/// Generic memory pools for reusable allocations without per-call heap churn.
pub mod memory_pools_safe;
/// Portable SIMD entry points with scalar fallbacks (see submodules for specifics).
pub mod simd_safe;

/// Testing utilities for truly concurrent, deterministic tests
///
/// Provides mock time sources, event synchronization, and barriers
/// to eliminate `sleep()`-based testing patterns.
///
/// Built only for `cargo test` or when the `test-utils` Cargo feature is enabled.
#[cfg(any(test, feature = "test-utils"))]
pub mod testing;
/// Aggressive performance-oriented helpers (benchmarks, hot-path helpers).
pub mod ultimate_performance;
/// Defense-in-depth checks around high-risk or high-risk utility code paths.
pub mod ultimate_safety;
/// Safe façade over zero-copy buffers (public API re-exported below).
pub mod zero_copy_safe;

// Performance and utility modules
/// General-purpose micro-optimizations (caching, fast paths).
pub mod performance_optimizations;
/// Cryptographic workloads accelerated via SIMD where available.
pub mod simd_crypto_acceleration;
/// SIMD-backed numeric and buffer operations with feature detection.
pub mod simd_optimizations;
/// Shared helpers: parsing, env, crypto utilities, and small safe primitives.
pub mod utils;
/// Zero-copy buffer strategies and ID management (see module docs for invariants).
pub mod zero_copy;
/// Experimental zero-copy layouts layered on the stable `zero_copy` APIs.
pub mod zero_copy_optimized;

// Testing frameworks - canonical location (includes mock crypto/config fixtures)
/// Property-based testing adapters (QuickCheck-style) for Beardog types.
///
/// Built only for `cargo test` or when the `test-utils` Cargo feature is enabled.
#[cfg(any(test, feature = "test-utils"))]
pub mod property_testing;

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
