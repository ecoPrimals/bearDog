// SPDX-License-Identifier: AGPL-3.0-or-later

//! Identity verification and proof systems for decentralized authentication.
//!
//! Handlers implement [`IdentityVerificationHandler`] per [`VerificationMethod`].
//! Production deployments register real handlers on [`VerificationHandlerRegistry`];
//! stub helpers are gated behind `test-utils` or `cfg(test)`.

pub use handlers::{IdentityVerificationHandler, VerificationHandlerRegistry};
#[cfg(any(test, feature = "test-utils"))]
pub use handlers::{StubVerificationHandler, default_stub_registry};
pub use types::*;

mod handlers;
#[cfg(test)]
mod tests;
mod types;
