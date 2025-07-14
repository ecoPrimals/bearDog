//! Cross-node authorization module
//!
//! **Cryptographic proof of permissions for secure distributed storage operations.**
//!
//! This module was refactored from a large file to improve maintainability.
//! The authorization system enables "friends helpin friends store data securely
//! with mathematical proof of permission."
//!
//! ## Key Features
//!
//! * **Authorization Proofs**: Cryptographic signatures proving operation permissions
//! * **Node Registry**: Trust relationships and capability management  
//! * **Genetic Spawning**: Dynamic BearDog instance creation with inherited capabilities
//! * **Multi-Party Workflows**: Consensus-based approval for sensitive operations
//! * **Ecosystem Integration**: Network effects with ToadStool, SongBird, NestGate, Squirrel

// Re-export public types and functions from submodules
pub use handlers::*;
pub use types::*;

// Module declarations
pub mod handlers;
pub mod types;

#[cfg(test)]
mod tests;

// Core module functionality will be implemented here
