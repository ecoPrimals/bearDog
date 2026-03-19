// SPDX-License-Identifier: AGPL-3.0-only

//! Crypto Provider Implementations
//!
//! Concrete implementations of the UniversalCryptoProvider trait for various crypto libraries.

pub mod rustcrypto;

#[cfg(test)]
mod rustcrypto_tests;

// Re-exports
pub use rustcrypto::RustCryptoProvider;
