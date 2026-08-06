// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(
    clippy::wildcard_imports,
    reason = "facade re-export: single public surface for env key constants"
)]

//! Centralized environment variable key constants.
//!
//! All `BEARDOG_*` env var names used across the codebase should be defined
//! here. Callers use these constants instead of inline string literals,
//! enabling rename refactors and grep-based auditing.
//!
//! Organized by domain, matching the `beardog-config/src/domains/` structure.

mod compute;
mod discovery;
mod identity;
mod monitoring;
mod network;
mod paths;
mod runtime;
mod security;

pub use compute::*;
pub use discovery::*;
pub use identity::*;
pub use monitoring::*;
pub use network::*;
pub use paths::*;
pub use runtime::*;
pub use security::*;
