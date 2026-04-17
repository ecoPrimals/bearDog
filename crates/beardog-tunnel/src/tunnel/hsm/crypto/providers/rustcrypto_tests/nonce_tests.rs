// SPDX-License-Identifier: AGPL-3.0-or-later
//! Nonce generation tests.

#![cfg(test)]

use super::*;

#[test]
fn test_generate_nonce() {
    let provider = RustCryptoProvider::new();
    let nonce = provider.generate_nonce(12);
    assert_eq!(nonce.len(), 12);
}

#[test]
fn test_generate_nonce_different_sizes() {
    let provider = RustCryptoProvider::new();
    let sizes = vec![8, 12, 16, 24, 32];

    for size in sizes {
        let nonce = provider.generate_nonce(size);
        assert_eq!(nonce.len(), size);
    }
}

#[test]
fn test_generate_nonce_uniqueness() {
    let provider = RustCryptoProvider::new();
    let nonce1 = provider.generate_nonce(12);
    let nonce2 = provider.generate_nonce(12);

    // Nonces should be different (with very high probability)
    assert_ne!(nonce1, nonce2);
}

#[test]
fn test_generate_nonce_not_all_zeros() {
    let provider = RustCryptoProvider::new();
    let nonce = provider.generate_nonce(12);

    // At least one byte should be non-zero
    assert!(nonce.iter().any(|&b| b != 0));
}
