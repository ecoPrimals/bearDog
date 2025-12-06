//! Modular Timeout Configuration
//!
//! This module provides centralized timeout configuration for all BearDog operations.
//! **MIGRATED FROM timeouts.rs** for better maintainability.
//!
//! ## Module Organization
//!
//! - `core` - Core TimeoutConfig struct and conversions
//! - `builder` - Builder pattern for configuration
//! - `defaults` - Default timeout values
//! - `validation` - Configuration validation logic
//! - `migration` - Backward compatibility layer
//!
//! ## Quick Migration Guide
//!
//! All APIs remain identical - this is a pure refactoring for complexity management.
//!
//! Use `timeouts::TimeoutConfig` for the main configuration type.

pub mod builder;
pub mod core;
pub mod defaults;
pub mod migration;
pub mod validation;

// Re-export primary types for convenience
pub use self::builder::TimeoutConfigBuilder;
pub use self::core::TimeoutConfig;
pub use self::defaults::default_timeouts;

#[cfg(test)]
mod tests;
