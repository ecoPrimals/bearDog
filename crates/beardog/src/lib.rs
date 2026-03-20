// SPDX-License-Identifier: AGPL-3.0-only
#![forbid(unsafe_code)]

//! BearDog - Privacy-Preserving Biometric Security
//!
//! The main entry point for the BearDog ecosystem, providing a unified interface
//! to all BearDog capabilities including cryptography, authentication, and
//! privacy-preserving operations.
//!
//! # Architecture
//!
//! BearDog is organized as a workspace of specialized crates:
//! - `beardog-core`: Core components, lifecycle, and system management
//! - `beardog-errors`: Unified error handling
//! - `beardog-types`: Canonical types and constants
//! - `beardog-ipc`: Inter-process communication and neural registration
//!
//! # Example
//!
//! ```rust,ignore
//! use beardog::BearDogError;
//! use beardog::core::BearDogCore;
//!
//! // Initialize BearDog core
//! let core = BearDogCore::new();
//! ```
//!
//! # Design Principles
//!
//! - **Privacy by Design**: All operations preserve user privacy
//! - **Pure Rust**: Minimal external dependencies, maximum auditability
//! - **Zero Trust**: Runtime capability discovery, no hardcoded assumptions
//! - **Dark Forest**: Selective disclosure of identity and capabilities

pub use beardog_core as core;
pub use beardog_errors as errors;
pub use beardog_errors::BearDogError;
pub use beardog_types::canonical::*;

// Re-export version and mission from centralized constants
pub use beardog_types::constants::domains::ecosystem::version::{MISSION, VERSION};

// Re-export neural_registration from beardog-ipc for convenience
pub use beardog_ipc::neural_registration;

#[cfg(test)]
mod neural_registration_extended_tests;
