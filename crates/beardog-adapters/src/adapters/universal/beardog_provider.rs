

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod capabilities;
/// Core functionality
/// Core functionality
pub mod core;
pub mod handlers;
// pub mod helpers; // REMOVED - use universal::capability_helpers instead
pub mod provider;

pub use core::BearDogPrimalProvider;
