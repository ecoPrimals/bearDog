// SPDX-License-Identifier: AGPL-3.0-or-later

//! Runtime compliance evaluation: the `handlers` submodule orchestrates checks; the `types` submodule defines events and results.

pub use handlers::*;
pub use types::*;

mod handlers;
#[cfg(test)]
mod handlers_tests;
#[cfg(test)]
mod tests;
mod types;
