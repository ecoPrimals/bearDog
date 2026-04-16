// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tor Protocol Crypto Handlers (Phase 2 - Pure Rust Tor)
//!
//! Provides cryptographic operations for Tor protocol implementation:
//! - ntor handshake (circuit key exchange)
//! - Cell encryption/decryption
//! - Tor-specific KDF operations
//!
//! **Architecture**: `BearDog` provides crypto primitives, the protocol peer implements transport
//!
//! **Reference**: <https://spec.torproject.org/tor-spec>
//!
//! Pure Rust implementation using `RustCrypto` (zero C dependencies).

mod cell;
mod constants;
mod kdf;
mod ntor_client;
mod ntor_server;
mod primitives;

pub use cell::{handle_tor_cell_decrypt, handle_tor_cell_encrypt};
pub use kdf::handle_tor_kdf;
pub use ntor_client::{handle_tor_ntor_client_finish, handle_tor_ntor_client_init};
pub use ntor_server::handle_tor_ntor_server_respond;

// `crypto_handlers_tor_tests` uses `use super::*` — mirror the old single-file module surface.
#[cfg(test)]
pub use base64::Engine;
#[cfg(test)]
pub use base64::engine::general_purpose::STANDARD as BASE64;
#[cfg(test)]
pub use beardog_errors::BearDogError;
#[cfg(test)]
pub use primitives::constant_time_compare;
#[cfg(test)]
pub use serde_json::{Value, json};
#[cfg(test)]
pub use x25519_dalek::{PublicKey, StaticSecret};

#[cfg(test)]
#[path = "../crypto_handlers_tor_tests.rs"]
mod tests;
