// SPDX-License-Identifier: AGPL-3.0-or-later

//! Hashing, MAC, and key derivation operations.
//!
//! # Encoding Contract (LD-01)
//!
//! All methods expect binary inputs as **standard Base64** (RFC 4648 §4,
//! `+/=` alphabet). `BearDog` decodes to raw bytes, processes, and returns
//! results as Base64.
//!
//! # Submodules
//!
//! - [`blake3`] — BLAKE3 cryptographic hash
//! - [`cipher`] — TLS 1.3 cipher-suite-aware digest (SHA-256/SHA-384)
//! - [`hmac`] — HMAC-SHA256 compute and constant-time verify
//! - [`hkdf`] — HKDF-SHA256 key derivation

mod blake3;
mod cipher;
mod hkdf;
mod hmac;

pub use blake3::handle_blake3_hash;
pub use cipher::handle_hash_for_cipher;
pub use hkdf::handle_hkdf_sha256;
pub use hmac::{handle_hmac_sha256, handle_hmac_verify};
