// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive error-path tests for monitoring (split by domain: metrics vs health/alerts).

#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_comparisons,
    clippy::all
)]

mod alerts_health;
mod common;
mod metrics;
