

use beardog_errors::BearDogResult;
use beardog_security::crypto_utils::BearDogCrypto;

#[tokio::test]
async fn test_ed25519_signature_mathematical_certainty() -> BearDogResult<()> {

    let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair()?;

    let message = b"test message for mathematical certainty";
    let signature = BearDogCrypto::sign_ed25519(&private_key, message)?;

    let is_valid = BearDogCrypto::verify_ed25519_signature(&public_key, message, &signature)?;
    assert!(
        is_valid,
        "Ed25519 signature verification must be mathematically certain"
    );

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

    let nonce1 = BearDogCrypto::generate_secure_nonce(32)?;
    let nonce2 = BearDogCrypto::generate_secure_nonce(32)?;

    assert_ne!(nonce1, nonce2, "Secure nonces must be unique");

    assert_ne!(nonce1, vec![0u8; 32], "Secure nonces must not be all zeros");
    assert_ne!(nonce2, vec![0u8; 32], "Secure nonces must not be all zeros");

    Ok(())
}

#[tokio::test]
async fn test_key_derivation_deterministic() -> BearDogResult<()> {

    let seed = b"test seed for key derivation";
    let context = b"test context";

    let key1 = BearDogCrypto::derive_key_pbkdf2(seed, context, 10000, 32)?;
    let key2 = BearDogCrypto::derive_key_pbkdf2(seed, context, 10000, 32)?;

    assert_eq!(key1, key2, "Key derivation must be deterministic");

    let different_context = b"different context";
    let key3 = BearDogCrypto::derive_key_pbkdf2(seed, different_context, 10000, 32)?;
    assert_ne!(key1, key3, "Different contexts must produce different keys");

    Ok(())
}
