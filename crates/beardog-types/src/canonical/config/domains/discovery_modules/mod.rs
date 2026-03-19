// SPDX-License-Identifier: AGPL-3.0-only

//! Discovery Configuration Module
//!
//! Domain-driven organization of discovery configuration components.
//!
//! ## Architecture
//! This module follows Domain-Driven Design principles, organizing configuration
//! by domain responsibility rather than arbitrary file size limits.
//!
//! ## Submodules
//! - `registry` - Service registration and tracking
//! - Additional modules to be added: network, quantum, cache, security, load_balancing
//!
//! ## Migration Path
//! This module is being extracted from `discovery_unified.rs` for better
//! maintainability. The original file will remain as a compatibility wrapper.

pub mod registry;

// Re-export for convenience
pub use registry::{EtcdAuth, ServiceRegistryConfig};
