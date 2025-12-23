

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


pub mod capabilities;
/// Core functionality
pub mod core;
pub mod handlers;

// **REMOVED**: Legacy helpers module - use universal adapter patterns instead
// 
// This module contained primal-specific helpers that have been replaced with
// capability-based discovery via the Universal Capability Adapter.
// 
// Migration path: Use `crates/beardog-adapters/src/universal/capability_helpers.rs`
// 
// Removed: October 2, 2025 (v3.1.0)
// 
// #[deprecated(
//     since = "3.0.1",
//     note = "Use universal adapter patterns from `universal::capability_helpers` instead."
// )]
// pub mod helpers;

pub mod provider;

pub use core::BearDogPrimalProvider;
