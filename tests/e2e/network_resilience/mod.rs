// SPDX-License-Identifier: AGPL-3.0-only
//! Network Resilience E2E Test Suite
//!
//! Modular test organization for network failure handling, retries, and recovery.
//!
//! ## Test Categories
//!
//! - `retry_tests` - Retry logic with exponential backoff
//! - `timeout_tests` - Timeout handling and detection
//! - `partition_tests` - Network partition scenarios
//! - `recovery_tests` - Recovery from failures
//! - `circuit_breaker_tests` - Circuit breaker patterns
//! - `degradation_tests` - Graceful degradation
//!
//! ## Usage
//!
//! Run all network resilience tests:
//! ```bash
//! cargo test --test network_resilience
//! ```
//!
//! Run specific category:
//! ```bash
//! cargo test --test network_resilience retry_tests::
//! ```

#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

use beardog_errors::BearDogError;
use tracing::{info, warn};

/// E2E metrics for network resilience tests
#[derive(Debug, Clone, Default)]
pub struct NetworkE2EMetrics {
    pub connection_attempts: usize,
    pub successful_connections: usize,
    pub failed_connections: usize,
    pub retries: usize,
    pub recoveries: usize,
    pub timeout_errors: usize,
}

// Module organization
pub mod circuit_breaker_tests;
pub mod degradation_tests;
pub mod partition_tests;
pub mod recovery_tests;
pub mod retry_tests;
pub mod timeout_tests;
