// SPDX-License-Identifier: AGPL-3.0-only

//! Runtime compliance evaluation: [`handlers`] orchestrate checks; [`types`] define events and results.

pub use handlers::*;
pub use types::*;

mod handlers;
#[cfg(test)]
mod handlers_tests;
#[cfg(test)]
mod tests;
mod types;
