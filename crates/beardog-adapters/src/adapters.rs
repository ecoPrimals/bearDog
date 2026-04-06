// SPDX-License-Identifier: AGPL-3.0-or-later

//! Optional adapter submodule tree; enable via pub mod adapters in lib.rs if needed.
//! The primary supported API remains the crate root.

/// Nested universal adapter experiments; see the universal submodule.
pub mod universal;

pub use universal::*;
