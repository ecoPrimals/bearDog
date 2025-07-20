//! Zero-Copy Cryptographic Operations for BearDog
//!
//! **High-Performance Crypto with Minimal Memory Allocations**
//!
//! This module has been refactored into smaller, focused sub-modules for better maintainability.
//! All original functionality is preserved and re-exported from this module.
//!
//! ## Module Structure
//!
//! - `types`: Configuration and type definitions
//! - `buffer_pool`: Memory-efficient buffer pooling
//! - Main cryptographic engine integrated into the module system
//!
//! ## Key Features
//!
//! - Zero-copy operations for 2-5x performance gains
//! - SIMD-accelerated cryptography with hardware optimization
//! - Advanced memory management with buffer pooling
//! - Streaming architecture for constant memory usage

// Re-export everything from the zero_copy module for backward compatibility
pub use crate::zero_copy::*;
