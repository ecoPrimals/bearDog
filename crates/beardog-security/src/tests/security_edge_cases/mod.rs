// SPDX-License-Identifier: AGPL-3.0-or-later

//! Security Edge Cases Tests - October 22, 2025
//!
//! High-value tests focusing on edge cases, error paths, and security-critical scenarios
//! to improve test coverage. These tests target areas identified in the coverage audit.

mod helpers;

pub(crate) use helpers::*;

#[cfg(test)]
mod auth;

#[cfg(test)]
mod config;

#[cfg(test)]
mod constant_time;

#[cfg(test)]
mod encryption;

#[cfg(test)]
mod error_recovery;

#[cfg(test)]
mod hashing;

#[cfg(test)]
mod key_management;

#[cfg(test)]
mod random;

#[cfg(test)]
mod secure_zero;

#[cfg(test)]
mod signature;
