// SPDX-License-Identifier: AGPL-3.0-or-later

//! Cryptographic algorithms

// Allow pedantic lints for algorithm implementations
// These functions have comprehensive documentation and error handling
// The `# Errors` sections would be repetitive (all return "encryption/decryption failed")
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::double_must_use)]

/// Public-key and signature primitives exposed through the crypto service.
pub mod asymmetric;
/// Capability and algorithm discovery helpers.
pub mod discovery;
/// Hashing helpers (e.g. BLAKE3, SHA-2) for the crypto service.
pub mod hashing;
/// Symmetric encryption and AEAD helpers.
pub mod symmetric;

pub use asymmetric::*;
pub use discovery::*;
pub use hashing::*;
pub use symmetric::*;
