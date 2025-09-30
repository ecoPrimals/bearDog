/// Core component implementations and lifecycle management
// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

/// Main BearDog Core system
pub mod beardog_core;
pub mod components;
/// Genetic algorithm optimization components
pub mod genetic_optimizer;
/// Service lifecycle management and state transitions
pub mod lifecycle;

// Re-export the main BearDogCore struct
pub use beardog_core::BearDogCore;

// Re-export genetic optimizer components
// pub use genetic_optimizer::*;
