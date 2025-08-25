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


//! Property-Based Testing for Mathematical Certainty
//!
//! Uses property-based testing to validate cryptographic and system properties

use beardog_errors::BearDogResult;
use beardog_security::crypto_utils::BearDogCrypto;

#[tokio::test]
async fn test_signature_verification_property() -> BearDogResult<()> {
    // Property: For any valid keypair and message, signature verification should succeed
    for i in 0..10 {
        let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair()?;
        let message = format!("test message {}", i).into_bytes();

        let signature = BearDogCrypto::sign_ed25519(&private_key, &message)?;
        let is_valid = BearDogCrypto::verify_ed25519_signature(&public_key, &message, &signature)?;

        assert!(
            is_valid,
            "Property violated: valid signature should always verify"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_nonce_uniqueness_property() -> BearDogResult<()> {
    // Property: Generated nonces should be unique across multiple generations
    let mut nonces = std::collections::HashSet::new();

    for _ in 0..100 {
        let nonce = BearDogCrypto::generate_secure_nonce(16)?;
        assert!(
            nonces.insert(nonce),
            "Property violated: nonces should be unique"
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_key_derivation_consistency_property() -> BearDogResult<()> {
    // Property: Key derivation should be consistent across multiple calls
    let test_cases = vec![
        (b"seed1".as_slice(), b"context1".as_slice()),
        (b"seed2".as_slice(), b"context2".as_slice()),
        (
            b"different_seed".as_slice(),
            b"different_context".as_slice(),
        ),
    ];

    for (seed, context) in test_cases {
        let key1 = BearDogCrypto::derive_key_pbkdf2(seed, context, 10000, 32)?;
        let key2 = BearDogCrypto::derive_key_pbkdf2(seed, context, 10000, 32)?;

        assert_eq!(
            key1, key2,
            "Property violated: key derivation should be deterministic"
        );
    }

    Ok(())
}
