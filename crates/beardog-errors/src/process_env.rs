//! Wrappers for [`std::env::set_var`] and [`std::env::remove_var`].
//!
//! In Rust 2024 these functions are `unsafe` because mutating the process environment is not
//! thread-safe. Callers must ensure no concurrent access (e.g. single-threaded tests or
//! deployment setup before spawning worker threads).

#![allow(unsafe_code)]

use std::ffi::OsStr;

/// See [`std::env::set_var`] for safety requirements.
#[inline]
pub fn set_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
    // SAFETY: Caller must ensure environment is not read or written concurrently.
    unsafe { std::env::set_var(key, value) }
}

/// See [`std::env::remove_var`] for safety requirements.
#[inline]
pub fn remove_var<K: AsRef<OsStr>>(key: K) {
    // SAFETY: Caller must ensure environment is not read or written concurrently.
    unsafe { std::env::remove_var(key) }
}
