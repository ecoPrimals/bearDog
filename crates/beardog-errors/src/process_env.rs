// SPDX-License-Identifier: AGPL-3.0-only

#![allow(unsafe_code)] // Workspace `deny(unsafe_code)`; Rust 2024 requires `unsafe` for env mutation.

//! Safe wrappers for [`std::env::set_var`] and [`std::env::remove_var`].
//!
//! In Rust 2024 these functions are `unsafe` because mutating the process environment is not
//! thread-safe on POSIX systems. These wrappers centralize the `unsafe` behind a single
//! documented call-site so the rest of the codebase stays free of ad-hoc env mutation.
//!
//! # Safety Contract
//!
//! Callers **must** ensure no concurrent reads or writes to the process environment.
//! Typical safe usage: single-threaded test setup (via `#[serial_test::serial]`),
//! or process bootstrap before spawning any worker threads.

use std::ffi::OsStr;

/// Set an environment variable.
///
/// # Safety (caller obligation)
///
/// Must be called when no other thread reads or writes the environment.
/// Prefer `#[serial_test::serial]` in tests.
///
/// See [`std::env::set_var`] for the full safety requirements.
#[inline]
pub fn set_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
    // SAFETY: Caller guarantees exclusive environment access (see module doc).
    unsafe { std::env::set_var(key, value) }
}

/// Remove an environment variable.
///
/// # Safety (caller obligation)
///
/// Must be called when no other thread reads or writes the environment.
///
/// See [`std::env::remove_var`] for the full safety requirements.
#[inline]
pub fn remove_var<K: AsRef<OsStr>>(key: K) {
    // SAFETY: Caller guarantees exclusive environment access (see module doc).
    unsafe { std::env::remove_var(key) }
}
