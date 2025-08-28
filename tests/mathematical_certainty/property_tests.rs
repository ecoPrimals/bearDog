

use beardog_errors::BearDogError;
use beardog_security::crypto_utils::BearDogCrypto;

#[tokio::test]
async fn test_signature_verification_property() -> Result<(), BearDogError> {

    for i in 0..10 {
        let (private_key, public_key) = BearDogCrypto::generate_ed25519_keypair()?;
        let message = format_args!("test message {}", i).to_string().into_bytes();

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
async fn test_nonce_uniqueness_property() -> Result<(), BearDogError> {

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
async fn test_key_derivation_consistency_property() -> Result<(), BearDogError> {

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
