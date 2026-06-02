// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_errors::BearDogError;

use super::*;

#[test]
fn test_chacha20_poly1305_encryption_decryption() -> Result<(), BearDogError> {
    // Test ChaCha20-Poly1305 encryption and decryption
    let plaintext = b"Data encrypted with ChaCha20-Poly1305";
    let key = generate_chacha20_key()?;
    let nonce = generate_chacha20_nonce()?;

    let ciphertext = encrypt_chacha20_poly1305(&key, &nonce, plaintext)?;
    assert_ne!(ciphertext.as_slice(), plaintext, "Ciphertext should differ");

    let decrypted = decrypt_chacha20_poly1305(&key, &nonce, &ciphertext)?;
    assert_eq!(decrypted.as_slice(), plaintext, "Decryption should match");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    Ok(())
}

#[test]
fn test_chacha20_poly1305_authentication_failure() -> Result<(), BearDogError> {
    // Test that authentication tag prevents tampering
    let plaintext = b"Authenticated data";
    let key = generate_chacha20_key()?;
    let nonce = generate_chacha20_nonce()?;

    let mut ciphertext = encrypt_chacha20_poly1305(&key, &nonce, plaintext)?;
    // Tamper with ciphertext
    if !ciphertext.is_empty() {
        ciphertext[0] ^= 0xFF;
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    let result = decrypt_chacha20_poly1305(&key, &nonce, &ciphertext);

    assert!(result.is_err(), "Authentication should catch tampering");

    Ok(())
}

#[test]
fn test_chacha20_poly1305_with_aad() -> Result<(), BearDogError> {
    // Test encryption with additional authenticated data
    let plaintext = b"Secret payload";
    let aad = b"Public header data";
    let key = generate_chacha20_key()?;
    let nonce = generate_chacha20_nonce()?;

    let ciphertext = encrypt_chacha20_poly1305_with_aad(&key, &nonce, plaintext, aad)?;
    let decrypted = decrypt_chacha20_poly1305_with_aad(&key, &nonce, &ciphertext, aad)?;

    assert_eq!(
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        decrypted.as_slice(),
        plaintext,
        "AAD should not affect decryption"
    );

    Ok(())
}

#[test]
fn test_chacha20_poly1305_wrong_aad() -> Result<(), BearDogError> {
    // Test that wrong AAD causes authentication failure
    let plaintext = b"Secret data";
    let aad = b"Correct AAD";
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    let wrong_aad = b"Wrong AAD!!";
    let key = generate_chacha20_key()?;
    let nonce = generate_chacha20_nonce()?;

    let ciphertext = encrypt_chacha20_poly1305_with_aad(&key, &nonce, plaintext, aad)?;
    let result = decrypt_chacha20_poly1305_with_aad(&key, &nonce, &ciphertext, wrong_aad);

    assert!(
        result.is_err(),
        "Wrong AAD should cause authentication failure"
    );

    Ok(())
}
