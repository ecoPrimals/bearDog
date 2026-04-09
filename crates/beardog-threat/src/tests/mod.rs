// SPDX-License-Identifier: AGPL-3.0-or-later

//! `BearDog` Threat Tests Module
//!
//! Comprehensive test coverage for threat detection functionality

#[allow(
    unused_imports,
    clippy::module_inception,
    clippy::manual_range_contains,
    clippy::assertions_on_constants,
    clippy::useless_vec,
    clippy::absurd_extreme_comparisons,
    unused_comparisons,
    reason = "integration threat tests: exhaustive patterns (multi-lint expect causes unfulfilled_lint_expectations)"
)]
#[cfg(test)]
mod threat_detection_tests;
