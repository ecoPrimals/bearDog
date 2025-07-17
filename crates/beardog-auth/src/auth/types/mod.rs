//! Type definitions for cross-node authorization
//!
//! This module contains all structs, enums, and type aliases for the authorization module,
//! organized into focused sub-modules for better maintainability.

pub mod authorization;
pub mod genetics;
pub mod node_registry;
pub mod spawning;
pub mod workflow;

// Re-export all types for backward compatibility
pub use authorization::*;
pub use genetics::*;
pub use node_registry::*;
pub use spawning::*;
pub use workflow::*;
