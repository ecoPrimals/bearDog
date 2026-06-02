// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]

//! # `BearDog` Production Utilities
//!
//! Production readiness checks, secrets management, and disaster recovery for
//! `BearDog` deployments.
//!
//! ## Modules
//!
//! - [`config`] — Runtime production-readiness gate (`production_ready()`)
//! - [`config_management`] — Configuration loading, vault-backed secrets, runtime validation
//! - [`disaster_recovery`] — BLAKE3 content-addressed backup integrity
//!

#![cfg_attr(test, allow(clippy::expect_used))]
#![cfg_attr(test, allow(clippy::unwrap_used))]

// October 27, 2025: Comprehensive test expansion
#[cfg(test)]
mod production_comprehensive_tests;

/// Content-addressed backup integrity (BLAKE3 over backup file trees).
pub mod disaster_recovery;

/// Production configuration loading, secrets backends, and runtime validation.
pub mod config_management;

/// Runtime production-readiness gate.
pub mod config {
    #[cfg(test)]
    use std::sync::atomic::{AtomicBool, Ordering};

    #[cfg(test)]
    static TEST_OVERRIDE_READY: AtomicBool = AtomicBool::new(true);

    /// Returns whether the production environment passes basic readiness checks.
    ///
    /// Currently checks:
    /// - `BEARDOG_MASTER_KEY` is set and at least 32 characters
    ///
    /// Returns `true` when all checks pass, `false` otherwise.
    #[must_use]
    pub fn production_ready() -> bool {
        #[cfg(test)]
        {
            TEST_OVERRIDE_READY.load(Ordering::Relaxed)
        }
        #[cfg(not(test))]
        {
            std::env::var(beardog_config::env_keys::ENV_MASTER_KEY).is_ok_and(|v| v.len() >= 32)
        }
    }

    /// Test-only: set the return value of `production_ready()`.
    #[cfg(test)]
    pub(crate) fn set_test_production_ready(ready: bool) {
        TEST_OVERRIDE_READY.store(ready, Ordering::Relaxed);
    }
}
