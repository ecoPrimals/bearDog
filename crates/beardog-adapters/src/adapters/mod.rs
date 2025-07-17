//! Adapter Modules for Ecosystem Integration
//!
//! **Comprehensive adapter architecture for universal ecosystem integration**
//!
//! This module provides various adapter types for integrating any primal with different
//! ecosystem components and external systems. It includes universal patterns for
//! ecosystem-wide interoperability.

// Universal ecosystem adapters (refactored for universal patterns)
pub mod nestgate;
pub mod songbird;

// Universal ecosystem adapter architecture
pub mod universal;

// Re-export universal types for convenience
pub use universal::*;
