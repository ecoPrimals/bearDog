//! # Property-Based Testing (DEPRECATED)
//!
//! ⚠️ **DEPRECATED**: This module has been consolidated into `property_testing`.
//!
//! ## Migration Path
//!
//! **Old (deprecated)**:
//! ```rust
//! use beardog_utils::property_based_testing::{PropertyBasedTestFramework, PropertyTestConfig};
//! ```
//!
//! **New (canonical)**:
//! ```rust
//! use beardog_utils::property_testing::{PropertyBasedTestFramework, PropertyTestConfig};
//! ```
//!
//! ## Rationale
//!
//! This consolidation eliminates fragmentation from having:
//! - `property_based_testing.rs` (file)
//! - `property_based_testing/` (directory)
//! - `property_testing/` (directory)
//!
//! All functionality is now unified in the `property_testing` module.
//!
//! **Removal Timeline**: v3.3.0 (Q1 2026)

#![allow(unused_imports, unused_variables, dead_code, clippy::all)]

#[deprecated(
    since = "3.0.2",
    note = "Use beardog_utils::property_testing module instead"
)]
pub use crate::property_testing::*;
