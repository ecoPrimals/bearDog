// SPDX-License-Identifier: AGPL-3.0-or-later

//! # SIMD Optimization Module
//! 
//! This module provides safe SIMD operations and optimizations for BearDog.
//! 
//! ## Organization
//! 
//! - `config`: SIMD configuration and feature detection
//! - `safe_ops`: Safe SIMD operations with runtime feature detection
//! - `optimizations`: General SIMD optimizations and algorithms
//! - `crypto`: Cryptographic SIMD acceleration
//! 
//! All modules provide memory-safe-only SIMD abstractions with automatic
//! fallback to scalar operations when SIMD instructions are unavailable.

pub mod config;
pub mod safe_ops;
pub mod optimizations;
pub mod crypto;

// Re-export commonly used types for convenience
pub use config::SafeSimdConfig;
pub use safe_ops::*;
pub use optimizations::*;
pub use crypto::*; 