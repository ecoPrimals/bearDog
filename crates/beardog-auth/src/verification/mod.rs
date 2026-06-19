// SPDX-License-Identifier: AGPL-3.0-or-later

//! Identity verification and proof systems for decentralized authentication.
//!
//! Handlers implement [`IdentityVerificationHandler`] per [`VerificationMethod`]; see
//! [`default_stub_registry`] for placeholder wiring.

pub use handlers::{
    IdentityVerificationHandler, StubVerificationHandler, VerificationHandlerRegistry,
    default_stub_registry,
};
pub use types::*;

mod handlers;
#[cfg(test)]
mod tests;
mod types;
