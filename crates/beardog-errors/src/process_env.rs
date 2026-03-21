// SPDX-License-Identifier: AGPL-3.0-only

//! Process environment overlay for tests and single-process configuration.
//!
//! Rust 2024 classifies [`std::env::set_var`] and [`std::env::remove_var`] as soundness-critical:
//! the process environment is not thread-safe on POSIX. Mutating the real environment also races
//! with concurrent [`std::env::var`] in other threads.
//!
//! This module keeps **all** mutation in a `std::sync::Mutex`-protected overlay map.
//! The `var`, `var_os`, and `vars` functions in this module
//! consult the overlay first, then fall back to the OS environment. Callers avoid the
//! soundness-critical std APIs for tests and in-process configuration.
//!
//! **Subprocesses**: [`std::process::Command`] inherits the **OS** environment only. Values set
//! only in the overlay are not visible to child processes unless you pass them explicitly with
//! [`std::process::Command::env`] or similar.
//!
//! Production deployments that rely on a real externally-set environment still work: unset keys
//! in the overlay defer to [`std::env::var`].

use std::collections::HashMap;
use std::env::VarError;
use std::ffi::{OsStr, OsString};
use std::sync::{Mutex, OnceLock};

type Overlay = HashMap<String, Option<String>>;

fn overlay() -> &'static Mutex<Overlay> {
    static CELL: OnceLock<Mutex<Overlay>> = OnceLock::new();
    CELL.get_or_init(|| Mutex::new(HashMap::new()))
}

fn key_string(key: &OsStr) -> String {
    key.to_string_lossy().into_owned()
}

/// Set a value in the process environment overlay (thread-safe).
///
/// Does not call [`std::env::set_var`].
pub fn set_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
    let k = key_string(key.as_ref());
    let v = key_string(value.as_ref());
    let mut g = overlay()
        .lock()
        .expect("process_env overlay mutex poisoned");
    g.insert(k, Some(v));
}

/// Mark a key as unset in the overlay (thread-safe).
///
/// A key removed here masks the OS value for `var` / `var_os` / `vars`.
/// Does not call [`std::env::remove_var`].
pub fn remove_var<K: AsRef<OsStr>>(key: K) {
    let k = key_string(key.as_ref());
    let mut g = overlay()
        .lock()
        .expect("process_env overlay mutex poisoned");
    g.insert(k, None);
}

/// Read an environment variable: overlay first, then [`std::env::var`].
#[inline]
pub fn var<K: AsRef<OsStr>>(key: K) -> Result<String, VarError> {
    match var_os(key.as_ref()) {
        Some(s) => s.into_string().map_err(VarError::NotUnicode),
        None => Err(VarError::NotPresent),
    }
}

/// Alias for `var` (same semantics as [`std::env::var`] with overlay).
#[inline]
pub fn get_var<K: AsRef<OsStr>>(key: K) -> Result<String, VarError> {
    var(key)
}

/// Read an environment variable as [`OsString`]: overlay first, then [`std::env::var_os`].
pub fn var_os<K: AsRef<OsStr>>(key: K) -> Option<OsString> {
    let k = key_string(key.as_ref());
    let g = overlay()
        .lock()
        .expect("process_env overlay mutex poisoned");
    if let Some(opt) = g.get(&k) {
        return opt.as_ref().map(OsString::from);
    }
    drop(g);
    std::env::var_os(key)
}

/// Iterate environment variables: OS vars merged with the overlay (overlay wins; removals apply).
pub fn vars() -> impl Iterator<Item = (String, String)> {
    let overlay_snapshot = overlay()
        .lock()
        .expect("process_env overlay mutex poisoned")
        .clone();
    let mut combined: HashMap<String, String> = std::env::vars().collect();
    for (k, v_opt) in overlay_snapshot {
        match v_opt {
            Some(v) => {
                combined.insert(k, v);
            }
            None => {
                combined.remove(&k);
            }
        }
    }
    combined.into_iter()
}
