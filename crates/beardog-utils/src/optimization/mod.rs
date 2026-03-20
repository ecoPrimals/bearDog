// SPDX-License-Identifier: AGPL-3.0-only

//! Clone strategy helpers, copy-on-write patterns, and string interning.

pub mod clone_optimizer;
pub mod clone_patterns;
pub mod string_interner;

/// Re-exports for clone analysis helpers.
pub use clone_optimizer::{CloneOptimizationStrategy, CloneOptimizer};
/// Global [`StringInterner`] accessors used from hot paths.
pub use string_interner::{StringInterner, global_interner, intern_string};
