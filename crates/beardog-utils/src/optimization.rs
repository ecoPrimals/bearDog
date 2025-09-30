// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod clone_optimizer;
pub mod string_interner;

pub use clone_optimizer::{CloneOptimizationStrategy, CloneOptimizer};
pub use string_interner::{global_interner, intern_string, StringInterner};
