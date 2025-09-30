//! # Universal Capability-Based Adapter - Modular Structure
//!
//! This is the revolutionary adapter that replaces ALL hardcoded vendor and primal
//! integrations with dynamic capability-based discovery. It enables true primal
//! sovereignty where "each primal only knows itself and discovers others via the
//! universal adapter."
//!
//! ## 🎯 **Refactoring Complete**
//! 
//! The original 995-line file has been split into focused modules:
//! - `core.rs` - Core adapter structure and initialization (200 lines)
//! - `discovery.rs` - Capability discovery logic (250 lines)
//! - `connection.rs` - Connection management (200 lines)
//! - `execution.rs` - Request execution and response handling (200 lines)
//! - `types.rs` - Type definitions and data structures (150 lines)
//!
//! **Total**: ~1000 lines across 5 focused files (avg: 200 lines per file)
//! **Improvement**: Better maintainability and separation of concerns

// Import all modular components
pub mod core;
pub mod discovery;
pub mod connection;
pub mod execution;
pub mod types;

// Re-export all public types for backward compatibility
pub use core::*;
pub use discovery::*;
pub use connection::*;
pub use execution::*;
pub use types::*; 