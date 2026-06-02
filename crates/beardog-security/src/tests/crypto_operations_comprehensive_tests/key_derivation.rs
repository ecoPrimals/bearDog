// SPDX-License-Identifier: AGPL-3.0-or-later

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal

use beardog_errors::BearDogError;

use super::*;

#[test]
fn test_pbkdf2_key_derivation() -> Result<(), BearDogError> {
    // Test PBKDF2 key derivation
    let password = b"User password";
    let salt = b"Random salt value";
    let iterations = 100_000;

    let derived_key = derive_key_pbkdf2(password, salt, iterations)?;

    assert_eq!(derived_key.len(), 32, "Derived key should be 32 bytes");

    Ok(())
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
fn test_pbkdf2_deterministic() -> Result<(), BearDogError> {
    // Test that same inputs produce same key
    let password = b"password123";
    let salt = b"salt123";
    let iterations = 10_000;

    let key1 = derive_key_pbkdf2(password, salt, iterations)?;
    let key2 = derive_key_pbkdf2(password, salt, iterations)?;

    assert_eq!(key1, key2, "PBKDF2 should be deterministic");

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    Ok(())
}

#[test]
fn test_pbkdf2_different_salts() -> Result<(), BearDogError> {
    // Test that different salts produce different keys
    let password = b"same password";
    let salt1 = b"salt 1";
    let salt2 = b"salt 2";
    let iterations = 10_000;

    let key1 = derive_key_pbkdf2(password, salt1, iterations)?;
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let key2 = derive_key_pbkdf2(password, salt2, iterations)?;

    assert_ne!(key1, key2, "Different salts should produce different keys");

    Ok(())
}

#[test]
fn test_argon2_key_derivation() -> Result<(), BearDogError> {
    // Test Argon2 key derivation (memory-hard)
    let password = b"User password for Argon2";
    let salt = b"Random salt for Argon2!!";

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let derived_key = derive_key_argon2(password, salt)?;

    assert_eq!(derived_key.len(), 32, "Argon2 key should be 32 bytes");

    Ok(())
}
