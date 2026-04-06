// SPDX-License-Identifier: AGPL-3.0-or-later

//! SHA hashing and Tor v3 onion helpers for Unix socket crypto IPC.
//!
//! Split by domain: SHA-2/SHA-1/SHA3-256 handlers and Tor v3 onion derivation.

mod onion;
mod sha;
#[cfg(test)]
mod tests;

pub use onion::{handle_derive_onion_address, handle_generate_onion_identity};
pub use sha::{handle_sha1, handle_sha3_256, handle_sha256, handle_sha384, handle_sha512};
