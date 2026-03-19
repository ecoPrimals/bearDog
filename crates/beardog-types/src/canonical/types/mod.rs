// SPDX-License-Identifier: AGPL-3.0-only

//! Type-Safe Newtypes for BearDog
//!
//! This module provides strongly-typed wrappers around primitive types
//! to prevent mixing different types of values at compile time.
//!
//! ## Benefits
//!
//! - **Compile-Time Safety**: Cannot mix different ID types
//! - **Zero Runtime Cost**: All wrappers are optimized away
//! - **Self-Documenting**: Type signatures are clearer
//! - **Better APIs**: Type-driven development

/// Type-safe ID newtypes (KeyId, ServiceInstanceId, RegistrationId)
pub mod ids;

// Re-export commonly used types
pub use ids::{KeyId, RegistrationId, ServiceInstanceId};
