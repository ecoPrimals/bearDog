// SPDX-License-Identifier: AGPL-3.0-or-later

//! Modular Timeout Configuration
//!
//! This module provides centralized timeout configuration for all `BearDog` operations.
//!
//! ## Module Organization
//!
//! - `core` - Core `TimeoutConfig` struct and conversions
//! - `builder` - Builder pattern for configuration
//! - `defaults` - Default timeout values
//! - `validation` - Configuration validation logic
//!
//! Use `timeouts::TimeoutConfig` for the main configuration type.

pub mod builder;
pub mod core;
pub mod defaults;
pub mod validation;

// Re-export primary types for convenience
pub use self::builder::TimeoutConfigBuilder;
pub use self::core::TimeoutConfig;
pub use self::defaults::default_timeouts;

#[cfg(test)]
mod tests;
