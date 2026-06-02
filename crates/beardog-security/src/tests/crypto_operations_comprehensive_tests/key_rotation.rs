// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_errors::BearDogError;

use super::*;

#[test]
fn test_key_rotation_workflow() -> Result<(), BearDogError> {
    // Test complete key rotation workflow
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let old_key = generate_aes_256_key()?;
    let new_key = generate_aes_256_key()?;
    let nonce = generate_aes_nonce()?;
    let plaintext = b"Data encrypted with old key";

    // Encrypt with old key
    let ciphertext_old = encrypt_aes_256_gcm(&old_key, &nonce, plaintext)?;

    // Decrypt with old key
    let decrypted = decrypt_aes_256_gcm(&old_key, &nonce, &ciphertext_old)?;

    // Re-encrypt with new key
    let ciphertext_new = encrypt_aes_256_gcm(&new_key, &nonce, &decrypted)?;

    // Verify new encryption works
    let final_decrypted = decrypt_aes_256_gcm(&new_key, &nonce, &ciphertext_new)?;

    assert_eq!(
        final_decrypted.as_slice(),
        plaintext,
        "Key rotation should preserve data"
    );

    Ok(())
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: critical
#[test]
fn test_multi_key_decryption_capability() -> Result<(), BearDogError> {
    // Test that we can try multiple keys until one works
    let keys = vec![
        generate_aes_256_key()?,
        generate_aes_256_key()?,
        generate_aes_256_key()?,
    ];
    let correct_key = &keys[1]; // Use middle key
    let nonce = generate_aes_nonce()?;
    let plaintext = b"Data encrypted with one of multiple keys";

    let ciphertext = encrypt_aes_256_gcm(correct_key, &nonce, plaintext)?;

    // Try each key until one works
    let mut decrypted = None;
    for key in &keys {
        if let Ok(data) = decrypt_aes_256_gcm(key, &nonce, &ciphertext) {
            decrypted = Some(data);
            break;
        }
    }

    assert!(decrypted.is_some(), "Should find correct key");
    let decrypted_data = decrypted
        .ok_or_else(|| BearDogError::security("Failed to decrypt with any key".to_string()))?;
    assert_eq!(
        decrypted_data.as_slice(),
        plaintext,
        "Decryption should work"
    );

    Ok(())
}
