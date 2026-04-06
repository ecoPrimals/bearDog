// SPDX-License-Identifier: AGPL-3.0-or-later

//! Recovery Comprehensive Test Suite
//!
//! `TEST_CATEGORY`: unit + integration
//! `TEST_DOMAIN`: security/recovery
//! `TEST_PRIORITY`: critical
//!
//! This module provides comprehensive testing for recovery mechanisms.
//! Originally consolidated from a single 1,121-line file, now split into focused modules.

pub mod challenge_tests;
pub mod ephemeral_tests;
pub mod error_handling_tests;
pub mod federation_tests;
pub mod policy_tests;
pub mod session_tests;
pub mod shard_tests;
pub mod social_tests;
pub mod types;

// Re-export commonly used items for convenience
