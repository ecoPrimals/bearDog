//! Adapter Modules for Ecosystem Integration
//!
//! **Comprehensive adapter architecture for universal ecosystem integration**
//!
//! This module provides various adapter types for integrating BearDog with different
//! ecosystem components and external systems. It includes both legacy integrations
//! and new universal patterns for ecosystem-wide interoperability.

// Legacy ecosystem adapters
pub mod nestgate;
pub mod songbird;

// Universal ecosystem adapter architecture
pub mod universal;

// Re-export universal types for convenience
pub use universal::*;
