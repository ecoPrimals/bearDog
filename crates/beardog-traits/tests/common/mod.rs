// SPDX-License-Identifier: AGPL-3.0-only
#![allow(clippy::unwrap_used)]

//! Shared helpers for integration tests in this crate.

use std::sync::{Mutex, OnceLock};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

/// Serializes tests that mutate the process environment.
pub fn env_lock() -> std::sync::MutexGuard<'static, ()> {
    ENV_LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
}
