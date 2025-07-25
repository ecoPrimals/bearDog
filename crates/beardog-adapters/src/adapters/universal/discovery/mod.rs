//! Ecosystem Discovery Service
//!
//! **Cross-ecosystem service discovery and integration**
//!
//! This module provides modular ecosystem discovery capabilities.

pub mod config;
pub mod health;
pub mod service_types;
pub mod core;

// Re-export main types
pub use config::*;
pub use health::*;
pub use service_types::*;
pub use core::*; 