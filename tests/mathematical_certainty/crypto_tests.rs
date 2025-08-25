// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! Mathematical Certainty Crypto Tests
//!
//! Tests that validate cryptographic operations with mathematical certainty

use beardog_errors::BearDogResult;
use beardog_security::crypto_utils::BearDogCrypto;

#[tokio::test]
async fn test_ed25519_signature_mathematical_certainty() -> BearDogResult<()> {
    // Test Ed25519 signature generation and verification with mathematical certainty
    let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair()?;

    let message = b"test message for mathematical certainty";
    let signature = BearDogCrypto::sign_ed25519(&private_key, message)?;

    // Verify signature is mathematically correct
    let is_valid = BearDogCrypto::verify_ed25519_signature(&public_key, message, &signature)?;
    assert!(
        is_valid,
        "Ed25519 signature verification must be mathematically certain"
    );

    // Verify wrong message fails
    let wrong_message = b"wrong message";
    let is_invalid =
        BearDogCrypto::verify_ed25519_signature(&public_key, wrong_message, &signature)?;
    assert!(
        !is_invalid,
        "Ed25519 signature verification must reject wrong messages"
    );

    Ok(())
}

#[tokio::test]
async fn test_nonce_generation_entropy() -> BearDogResult<()> {
    // Test nonce generation has sufficient entropy
    let nonce1 = BearDogCrypto::generate_secure_nonce(32)?;
    let nonce2 = BearDogCrypto::generate_secure_nonce(32)?;

    // Nonces must be different (extremely high probability)
    assert_ne!(nonce1, nonce2, "Secure nonces must be unique");

    // Nonces must not be all zeros
    assert_ne!(nonce1, vec![0u8; 32], "Secure nonces must not be all zeros");
    assert_ne!(nonce2, vec![0u8; 32], "Secure nonces must not be all zeros");

    Ok(())
}

#[tokio::test]
async fn test_key_derivation_deterministic() -> BearDogResult<()> {
    // Test key derivation is deterministic with same inputs
    let seed = b"test seed for key derivation";
    let context = b"test context";

    let key1 = BearDogCrypto::derive_key_pbkdf2(seed, context, 10000, 32)?;
    let key2 = BearDogCrypto::derive_key_pbkdf2(seed, context, 10000, 32)?;

    // Same inputs must produce same outputs
    assert_eq!(key1, key2, "Key derivation must be deterministic");

    // Different context must produce different keys
    let different_context = b"different context";
    let key3 = BearDogCrypto::derive_key_pbkdf2(seed, different_context, 10000, 32)?;
    assert_ne!(key1, key3, "Different contexts must produce different keys");

    Ok(())
}
