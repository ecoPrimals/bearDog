// SPDX-License-Identifier: AGPL-3.0-or-later

//! Handler Comprehensive Test Suite
//!
//! `TEST_CATEGORY`: unit + integration
//! `TEST_DOMAIN`: core/handlers
//! `TEST_PRIORITY`: critical
//!
//! This module provides comprehensive testing for handler mechanisms.
//! Originally consolidated from a single 1,355-line file, now split into focused modules.
//!
//! ## Test Modules
//!
//! - `types` - Shared test helper types and utilities
//! - `registration_tests` - Event handler registration and dispatch tests
//! - `error_chain_tests` - Error handler chaining tests
//! - `async_tests` - Async handler execution tests
//! - `middleware_tests` - Handler middleware tests
//! - `state_tests` - Handler state management tests
//! - `concurrency_tests` - Handler concurrency tests
//! - `timeout_tests` - Handler timeout tests
//! - `recovery_tests` - Handler error recovery tests

pub mod types;
pub mod registration_tests;
pub mod error_chain_tests;
pub mod async_tests;
pub mod middleware_tests;
pub mod state_tests;
pub mod concurrency_tests;
pub mod timeout_tests;
pub mod recovery_tests;

// Re-export commonly used items for convenience
pub use types::*;

