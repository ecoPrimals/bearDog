//! BearDog Universal PrimalProvider Implementation
//!
//! **BearDog's implementation of universal ecosystem patterns**
//!
//! This module shows how BearDog integrates with the universal ecosystem
//! by implementing the PrimalProvider trait as a security provider.
//! It follows SongBird's established patterns for interoperable ecosystem components.

pub mod capabilities;
pub mod core;
pub mod handlers;
pub mod helpers;
pub mod provider;

// Re-export the main struct for backward compatibility
pub use core::BearDogPrimalProvider; 