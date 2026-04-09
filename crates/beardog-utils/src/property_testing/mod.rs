// SPDX-License-Identifier: AGPL-3.0-or-later

//! Property-Based Testing Framework - Canonical Location
//!
//! **Build gating:** the `property_testing` module (including [`mock_implementations`](crate::property_testing::mock_implementations)) is
//! compiled only under `cfg(test)` or the `test-utils` Cargo feature on `beardog-utils`.
//! It must not be relied on from production dependency graphs without that feature.
//!
//! **Unified property-based testing framework** - consolidated from scattered implementations.
//!
//! This module provides comprehensive property-based testing capabilities for `BearDog`,
//! with all implementations consolidated into a single, maintainable location.
//!
//! ## Migration Note
//! This replaces the fragmented `property_based_testing` module and standalone file.
//! All property testing functionality is now centralized here.

// Names imported for `super::...` in submodules (submodules do not repeat these `use` lines).
#[expect(
    unused_imports,
    reason = "Parent import surface for property_testing submodules."
)]
use {
    beardog_errors::BearDogError,
    std::collections::HashMap,
    std::fmt::Debug,
    std::time::{Duration, Instant},
    tracing::{debug, info, warn},
};

// Core types and configuration
pub mod types;

// Property implementations (test fixtures in `mock_implementations`; crate gated in lib.rs)
/// REST-style validation properties built on [`PropertyBasedTestFramework`].
pub mod api_properties;
/// TOML/config parsing and defaulting properties.
pub mod config_properties;
/// Crypto-shaped property tests using deterministic test helpers from `mock_implementations`.
pub mod crypto_properties;
/// Deterministic non-crypto helpers for property tests (see module docs in `mock_implementations.rs`).
pub mod mock_implementations;

// Re-export primary types for convenient access
pub use types::*;

// Backward compatibility re-exports
#[deprecated(
    since = "3.0.2",
    note = "Use property_testing module directly instead of property_based_testing"
)]
pub use types::PropertyBasedTestFramework as LegacyFramework;
