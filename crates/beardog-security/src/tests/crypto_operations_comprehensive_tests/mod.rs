// SPDX-License-Identifier: AGPL-3.0-or-later

//! Comprehensive Cryptographic Operations Tests
//!
//! High-value security tests covering core cryptographic operations
//! including Ed25519 signatures, AES-256-GCM, ChaCha20-Poly1305, and BLAKE3.

mod helpers;

use helpers::*;

#[cfg(test)]
mod aes_gcm;

#[cfg(test)]
mod blake3;

#[cfg(test)]
mod chacha20;

#[cfg(test)]
mod ed25519;

#[cfg(test)]
mod key_derivation;

#[cfg(test)]
mod key_rotation;
