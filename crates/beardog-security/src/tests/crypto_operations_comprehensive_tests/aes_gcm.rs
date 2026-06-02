// SPDX-License-Identifier: AGPL-3.0-or-later

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal

use beardog_errors::BearDogError;

use super::*;

#[test]
fn test_aes_256_gcm_encryption_decryption() -> Result<(), BearDogError> {
    // Test basic AES-256-GCM encryption and decryption
    let plaintext = b"Sensitive data to encrypt with AES-256-GCM";
    let key = generate_aes_256_key()?;
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let nonce = generate_aes_nonce()?;

    let ciphertext = encrypt_aes_256_gcm(&key, &nonce, plaintext)?;
    assert_ne!(
        ciphertext.as_slice(),
        plaintext,
        "Ciphertext should differ from plaintext"
    );

    let decrypted = decrypt_aes_256_gcm(&key, &nonce, &ciphertext)?;
    assert_eq!(
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: important
        decrypted.as_slice(),
        plaintext,
        "Decrypted data should match original"
    );

    Ok(())
}

#[test]
fn test_aes_256_gcm_wrong_key() -> Result<(), BearDogError> {
    // Test that decryption fails with wrong key
    let plaintext = b"Secret data";
    let key = generate_aes_256_key()?;
    let wrong_key = generate_aes_256_key()?;
    let nonce = generate_aes_nonce()?;
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: important

    let ciphertext = encrypt_aes_256_gcm(&key, &nonce, plaintext)?;
    let result = decrypt_aes_256_gcm(&wrong_key, &nonce, &ciphertext);

    assert!(result.is_err(), "Decryption with wrong key should fail");

    Ok(())
}

#[test]
fn test_aes_256_gcm_wrong_nonce() -> Result<(), BearDogError> {
    // Test that decryption fails with wrong nonce
    let plaintext = b"Secret data";
    let key = generate_aes_256_key()?;
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let nonce = generate_aes_nonce()?;
    let wrong_nonce = generate_aes_nonce()?;

    let ciphertext = encrypt_aes_256_gcm(&key, &nonce, plaintext)?;
    let result = decrypt_aes_256_gcm(&key, &wrong_nonce, &ciphertext);

    assert!(result.is_err(), "Decryption with wrong nonce should fail");

    Ok(())
}

#[test]
fn test_aes_256_gcm_tampered_ciphertext() -> Result<(), BearDogError> {
    // Test that decryption fails with tampered ciphertext
    let plaintext = b"Secret data";
    let key = generate_aes_256_key()?;
    let nonce = generate_aes_nonce()?;

    let mut ciphertext = encrypt_aes_256_gcm(&key, &nonce, plaintext)?;
    // Tamper with ciphertext
    if !ciphertext.is_empty() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        ciphertext[0] ^= 0xFF;
    }

    let result = decrypt_aes_256_gcm(&key, &nonce, &ciphertext);

    assert!(
        result.is_err(),
        "Decryption of tampered ciphertext should fail"
    );

    Ok(())
}

#[test]
fn test_aes_256_gcm_empty_plaintext() -> Result<(), BearDogError> {
    // Test encryption/decryption of empty data
    let plaintext = b"";
    let key = generate_aes_256_key()?;
    let nonce = generate_aes_nonce()?;

    let ciphertext = encrypt_aes_256_gcm(&key, &nonce, plaintext)?;
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let decrypted = decrypt_aes_256_gcm(&key, &nonce, &ciphertext)?;

    assert_eq!(
        decrypted.as_slice(),
        plaintext,
        "Empty plaintext should round-trip"
    );

    Ok(())
}

#[test]
fn test_aes_256_gcm_large_plaintext() -> Result<(), BearDogError> {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    // Test encryption/decryption of large data (1MB)
    let plaintext = vec![0x42u8; 1024 * 1024]; // 1MB
    let key = generate_aes_256_key()?;
    let nonce = generate_aes_nonce()?;

    let ciphertext = encrypt_aes_256_gcm(&key, &nonce, &plaintext)?;
    let decrypted = decrypt_aes_256_gcm(&key, &nonce, &ciphertext)?;

    assert_eq!(
        decrypted, plaintext,
        "Large plaintext should round-trip correctly"
    );

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    Ok(())
}
