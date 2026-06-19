// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
#![allow(
    missing_docs,
    clippy::unwrap_used,
    clippy::expect_used,
    reason = "benchmarks are internal tooling; panics in bench/test code are acceptable"
)]

pub mod utils;

#[cfg(test)]
mod utils_tests;

pub use beardog_security::{encryption, memory_key_manager};
