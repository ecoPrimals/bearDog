//! Idiomatic Rust error patterns
//!
//! This module provides utilities for working with idiomatic Result<T, BearDogError> patterns.
//! All type aliases have been removed in favor of using Result<T, BearDogError> directly.
//!
//! # Examples
//!
//! ```rust
//! use beardog_errors::BearDogError;
//!
//! // System-level operations
//! fn system_operation() -> Result<(), BearDogError> {
//!     Ok(())
//! }
//!
//! // With context using ResultExt
//! use beardog_errors::ResultExt;
//!
//! fn load_system_config() -> Result<Config, BearDogError> {
//!     std::fs::read_to_string("system.conf")
//!         .system_context("Failed to load system configuration")?;
//!     // Parse and return config
//! }
//! ```
