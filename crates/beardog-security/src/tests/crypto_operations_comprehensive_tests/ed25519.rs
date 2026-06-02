// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_errors::BearDogError;

use super::*;

#[test]
fn test_ed25519_keypair_generation() -> Result<(), BearDogError> {
    // Test that we can generate a valid Ed25519 keypair
    let keypair = generate_ed25519_keypair()?;

    assert!(keypair.public.len() == 32, "Public key should be 32 bytes");
    assert!(keypair.secret.len() == 32, "Secret key should be 32 bytes");

    Ok(())
}

#[test]
fn test_ed25519_signature_verification_valid() -> Result<(), BearDogError> {
    // Test signing and verification with valid signature
    let keypair = generate_ed25519_keypair()?;
    let message = b"Test message for Ed25519 signature";

    let signature = sign_ed25519(&keypair.secret, message)?;
    let is_valid = verify_ed25519_signature(&keypair.public, message, &signature)?;

    assert!(is_valid, "Valid signature should verify successfully");

    Ok(())
}

#[test]
fn test_ed25519_signature_verification_invalid_message() -> Result<(), BearDogError> {
    // Test that verification fails with wrong message
    let keypair = generate_ed25519_keypair()?;
    let message = b"Original message";
    let wrong_message = b"Tampered message";

    let signature = sign_ed25519(&keypair.secret, message)?;
    let is_valid = verify_ed25519_signature(&keypair.public, wrong_message, &signature)?;

    assert!(
        !is_valid,
        "Signature should fail verification with wrong message"
    );

    Ok(())
}

#[test]
fn test_ed25519_signature_verification_invalid_signature() -> Result<(), BearDogError> {
    // Test that verification fails with tampered signature
    let keypair = generate_ed25519_keypair()?;
    let message = b"Test message";

    let mut signature = sign_ed25519(&keypair.secret, message)?;
    // Tamper with signature
    signature[0] ^= 0xFF;

    let is_valid = verify_ed25519_signature(&keypair.public, message, &signature)?;

    assert!(!is_valid, "Tampered signature should fail verification");

    Ok(())
}

#[test]
fn test_ed25519_multiple_signatures() -> Result<(), BearDogError> {
    // Test multiple signatures with same keypair
    let keypair = generate_ed25519_keypair()?;
    let messages = vec![
        b"Message 1".as_slice(),
        b"Message 2".as_slice(),
        b"Message 3".as_slice(),
    ];

    for message in messages {
        let signature = sign_ed25519(&keypair.secret, message)?;
        let is_valid = verify_ed25519_signature(&keypair.public, message, &signature)?;
        assert!(is_valid, "Each signature should verify independently");
    }

    Ok(())
}
