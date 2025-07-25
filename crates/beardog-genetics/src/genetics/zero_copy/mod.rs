//! Zero-Copy Genetic Operations Module
//!
//! High-performance genetic processing with minimal allocations.
//! This module is organized into focused sub-modules for maintainability.

pub mod analysis;
pub mod lineage;
pub mod pool;

pub use analysis::*;
pub use lineage::{LineageStats, LineageTracker};
pub use pool::GeneticsPool;
