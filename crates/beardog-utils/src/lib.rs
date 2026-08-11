// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used, reason = "expect/unwrap acceptable for invariant failures in tests and bootstrap code"))]

//! # `BearDog` Utilities Crate
//!
//! Safe utility primitives consumed by the `BearDog` ecosystem:
//!
//! - [`utils::safe_ops::SafeOps`] — checked arithmetic and overflow-safe operations
//! - [`utils::safe_memory_enhanced`] — pinned buffers and global buffer pools
//! - [`resolve_uid_from_proc`] — real UID from `/proc/self/status` on Linux

/// Shared helpers: safe arithmetic, pinned memory, buffer pools.
pub mod utils;

pub use utils::*;

/// Approximate `f64` equality for tests (avoids `clippy::float_cmp`).
#[cfg(test)]
pub(crate) mod float_eq {
    #[track_caller]
    pub fn f64(a: f64, b: f64) {
        const EPS: f64 = 1e-9;
        assert!((a - b).abs() < EPS, "expected ~{b}, got {a}");
    }
}
