//! Core BearDog Security Manager functionality
//!
//! This crate provides the main BearDog orchestration engine and core functionality.

pub mod core;
pub mod ecosystem_integration;
pub mod licensing;

// Re-export commonly used types
pub use core::BearDogCore;
pub use ecosystem_integration::*;
pub use licensing::*;
