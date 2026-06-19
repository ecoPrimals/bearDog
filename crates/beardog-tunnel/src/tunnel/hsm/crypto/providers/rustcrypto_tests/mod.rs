// SPDX-License-Identifier: AGPL-3.0-or-later

// Comprehensive tests for RustCrypto provider
//
// This module provides comprehensive test coverage for all cryptographic operations
// implemented by the RustCrypto provider, including encryption, signatures, hashing, and KDF.

#![cfg(test)]

pub use super::rustcrypto::RustCryptoProvider;
pub use crate::tunnel::hsm::crypto::algorithms::*;
pub use crate::tunnel::hsm::crypto::provider::{NonceGenerator, UniversalCryptoProvider};

mod aes_128_gcm_tests;
mod aes_256_gcm_tests;
mod asymmetric_tests;
mod chacha20_poly1305_tests;
mod ecdsa_p256_signature_tests;
mod ed25519_signature_tests;
mod hash_tests;
mod kdf_tests;
mod nonce_tests;
mod provider_basics_tests;
