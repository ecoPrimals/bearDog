// SPDX-License-Identifier: AGPL-3.0-only

//! Asymmetric cryptography algorithms
//!
//! Provides implementations of public-key cryptography including:
//! - Ed25519 (Edwards-curve Digital Signature Algorithm)
//! - ECDSA P-256 (Elliptic Curve Digital Signature Algorithm)
//! - RSA-PSS (RSA Probabilistic Signature Scheme)
//!
//! All implementations use safe Rust with established cryptography libraries.

use beardog_errors::BearDogError;

type Result<T> = std::result::Result<T, BearDogError>;

/// Ed25519 signature generation
///
/// Ed25519 is a modern elliptic curve signature scheme with:
/// - Fast signing and verification
/// - Small keys (32-byte public, 64-byte secret)
/// - Deterministic signatures (no need for secure random during signing)
/// - High security level
///
/// # Arguments
///
/// * `data` - Data to sign
/// * `secret_key` - 64-byte Ed25519 secret key (includes public key)
///
/// # Returns
///
/// 64-byte Ed25519 signature
///
/// # Security
///
/// Ed25519 provides 128-bit security level and is resistant to:
/// - Side-channel attacks
/// - Fault attacks
/// - Weak randomness
///
/// # Errors
///
/// Returns error if secret key format is invalid (not properly expanded 64-byte format).
pub fn sign_ed25519(data: &[u8], secret_key: &[u8; 64]) -> Result<Vec<u8>> {
    use ed25519_dalek::{Signer, SigningKey};

    // Ed25519 uses 32-byte secret key (first 32 bytes of the 64-byte expanded key)
    let key_bytes: [u8; 32] = secret_key[..32]
        .try_into()
        .map_err(|_| BearDogError::validation("Invalid Ed25519 secret key format"))?;

    let signing_key = SigningKey::from_bytes(&key_bytes);

    // Sign the data (deterministic, no RNG needed)
    let signature = signing_key.sign(data);

    Ok(signature.to_bytes().to_vec())
}

/// Ed25519 signature verification
///
/// Verifies an Ed25519 signature against a public key.
///
/// # Arguments
///
/// * `data` - Original data that was signed
/// * `signature` - 64-byte Ed25519 signature to verify
/// * `public_key` - 32-byte Ed25519 public key
///
/// # Returns
///
/// `true` if signature is valid, `false` otherwise
///
/// # Errors
///
/// Returns error if:
/// - Public key format is invalid
/// - Signature format is invalid (doesn't return false, fails hard)
pub fn verify_ed25519(data: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    // Parse public key (must be exactly 32 bytes)
    let pub_key_bytes: [u8; 32] = public_key.try_into().map_err(|_| {
        BearDogError::validation("Invalid Ed25519 public key length (expected 32 bytes)")
    })?;

    let verifying_key = VerifyingKey::from_bytes(&pub_key_bytes)
        .map_err(|e| BearDogError::hsm(format!("Invalid Ed25519 public key: {e}")))?;

    // Parse signature (must be exactly 64 bytes)
    let sig_bytes: [u8; 64] = signature.try_into().map_err(|_| {
        BearDogError::validation("Invalid Ed25519 signature length (expected 64 bytes)")
    })?;

    let sig = Signature::from_bytes(&sig_bytes);

    // Verify signature
    Ok(verifying_key.verify(data, &sig).is_ok())
}

/// Generate Ed25519 keypair from seed
///
/// Derives a deterministic Ed25519 keypair from a 32-byte seed.
/// Useful for key derivation schemes.
///
/// # Arguments
///
/// * `seed` - 32-byte seed for key derivation
///
/// # Returns
///
/// Tuple of (`secret_key`, `public_key`) where:
/// - `secret_key` is 64 bytes (expanded secret key)
/// - `public_key` is 32 bytes
///
/// # Errors
///
/// Returns error if keypair generation fails (extremely rare, indicates library issue).
pub fn generate_ed25519_from_seed(seed: &[u8; 32]) -> Result<([u8; 64], [u8; 32])> {
    use ed25519_dalek::SigningKey;

    let signing_key = SigningKey::from_bytes(seed);
    let verifying_key = signing_key.verifying_key();

    // Construct 64-byte expanded secret key
    let mut secret_key = [0u8; 64];
    secret_key[..32].copy_from_slice(&signing_key.to_bytes());
    secret_key[32..].copy_from_slice(&verifying_key.to_bytes());

    let public_key = verifying_key.to_bytes();

    Ok((secret_key, public_key))
}

/// ECDSA P-256 signature generation
///
/// ECDSA with the NIST P-256 curve (also known as secp256r1).
/// Widely supported but requires secure randomness during signing.
///
/// # Arguments
///
/// * `data` - Data to sign (will be hashed with SHA-256)
/// * `secret_key` - 32-byte ECDSA P-256 secret key
///
/// # Returns
///
/// DER-encoded ECDSA signature (variable length, typically ~70 bytes)
///
/// # Security Note
///
/// ECDSA requires high-quality randomness for each signature.
/// Weak RNG can lead to private key recovery. Consider using deterministic
/// ECDSA (RFC 6979) for better security.
///
/// # Errors
///
/// Returns error if:
/// - Secret key is invalid or out of range for P-256 curve
/// - Signing operation fails
pub fn sign_ecdsa_p256(data: &[u8], secret_key: &[u8; 32]) -> Result<Vec<u8>> {
    use p256::ecdsa::{signature::Signer, Signature, SigningKey};
    use sha2::{Digest, Sha256};

    let signing_key = SigningKey::from_bytes(secret_key.into())
        .map_err(|e| BearDogError::hsm(format!("Invalid ECDSA P-256 secret key: {e}")))?;

    // Hash the data first (ECDSA signs the hash)
    let mut hasher = Sha256::new();
    hasher.update(data);
    let digest = hasher.finalize();

    // Sign the hash
    let signature: Signature = signing_key.sign(&digest);

    // Return DER-encoded signature
    Ok(signature.to_der().as_bytes().to_vec())
}

/// ECDSA P-256 signature verification
///
/// Verifies an ECDSA P-256 signature.
///
/// # Arguments
///
/// * `data` - Original data that was signed
/// * `signature` - DER-encoded ECDSA signature
/// * `public_key` - 33 or 65 byte ECDSA P-256 public key (compressed or uncompressed)
///
/// # Returns
///
/// `true` if signature is valid, `false` otherwise
///
/// # Errors
///
/// Returns error if:
/// - Public key format is invalid
/// - Signature format is malformed
pub fn verify_ecdsa_p256(data: &[u8], signature: &[u8], public_key: &[u8]) -> Result<bool> {
    use p256::ecdsa::{signature::Verifier, Signature, VerifyingKey};
    use sha2::{Digest, Sha256};

    // Parse public key (accepts both compressed and uncompressed)
    let verifying_key = VerifyingKey::from_sec1_bytes(public_key)
        .map_err(|e| BearDogError::hsm(format!("Invalid ECDSA P-256 public key: {e}")))?;

    // Parse DER signature
    let sig = Signature::from_der(signature)
        .map_err(|e| BearDogError::hsm(format!("Invalid ECDSA signature format: {e}")))?;

    // Hash the data
    let mut hasher = Sha256::new();
    hasher.update(data);
    let digest = hasher.finalize();

    // Verify signature
    Ok(verifying_key.verify(&digest, &sig).is_ok())
}

/// Generate ECDSA P-256 keypair from seed
///
/// # Arguments
///
/// * `seed` - 32-byte seed for key derivation
///
/// # Returns
///
/// Tuple of (`secret_key`, `public_key`) where:
/// - `secret_key` is 32 bytes
/// - `public_key` is 33 bytes (compressed format)
///
/// # Errors
///
/// Returns error if seed value is out of range for P-256 curve.
pub fn generate_ecdsa_p256_from_seed(seed: &[u8; 32]) -> Result<([u8; 32], Vec<u8>)> {
    use p256::ecdsa::SigningKey;

    let signing_key = SigningKey::from_bytes(seed.into())
        .map_err(|e| BearDogError::hsm(format!("Failed to generate ECDSA key: {e}")))?;

    let verifying_key = signing_key.verifying_key();

    // Export public key in compressed format (33 bytes)
    let public_key = verifying_key.to_encoded_point(true).to_bytes().to_vec();

    let secret_key = signing_key.to_bytes();
    let mut secret_array = [0u8; 32];
    secret_array.copy_from_slice(&secret_key);

    Ok((secret_array, public_key))
}

/// RSA-PSS signature generation
///
/// RSA-PSS (Probabilistic Signature Scheme) is a modern RSA signature scheme
/// with provable security. Uses SHA-256 hash and MGF1 mask generation.
///
/// # Arguments
///
/// * `data` - Data to sign
/// * `private_key_der` - DER-encoded PKCS#8 RSA private key
///
/// # Returns
///
/// RSA-PSS signature bytes
///
/// # Security
///
/// RSA-PSS provides better security than traditional PKCS#1 v1.5 signatures:
/// - Probabilistic (uses random salt)
/// - Provable security reduction
/// - Recommended by modern standards
///
/// # Errors
///
/// Returns error if:
/// - Private key is invalid or wrong format
/// - Signing operation fails
/// - Key size is insufficient for the hash/salt combination
pub fn sign_rsa_pss(data: &[u8], private_key_der: &[u8]) -> Result<Vec<u8>> {
    use rsa::pkcs8::DecodePrivateKey;
    use rsa::pss::{BlindedSigningKey, Signature};
    use rsa::signature::{RandomizedSigner, SignatureEncoding};
    use rsa::RsaPrivateKey;
    use sha2::Sha256;

    // Parse PKCS#8 DER-encoded private key
    let private_key = RsaPrivateKey::from_pkcs8_der(private_key_der)
        .map_err(|e| BearDogError::hsm(format!("Invalid RSA private key: {e}")))?;

    // Create PSS signing key with SHA-256
    let signing_key = BlindedSigningKey::<Sha256>::new(private_key);

    // Sign with randomness
    let mut rng = rand::thread_rng();
    let signature: Signature = signing_key.sign_with_rng(&mut rng, data);

    Ok(signature.to_bytes().into())
}

/// RSA-PSS signature verification
///
/// Verifies an RSA-PSS signature using SHA-256 hash.
///
/// # Arguments
///
/// * `data` - Original data that was signed
/// * `signature` - RSA-PSS signature bytes
/// * `public_key_der` - DER-encoded `SubjectPublicKeyInfo` RSA public key
///
/// # Returns
///
/// `true` if signature is valid, `false` otherwise
///
/// # Errors
///
/// Returns error if:
/// - Public key format is invalid
/// - Signature format is malformed
pub fn verify_rsa_pss(data: &[u8], signature: &[u8], public_key_der: &[u8]) -> Result<bool> {
    use rsa::pkcs8::DecodePublicKey;
    use rsa::pss::{Signature, VerifyingKey};
    use rsa::signature::Verifier;
    use rsa::RsaPublicKey;
    use sha2::Sha256;

    // Parse public key
    let public_key = RsaPublicKey::from_public_key_der(public_key_der)
        .map_err(|e| BearDogError::hsm(format!("Invalid RSA public key: {e}")))?;

    // Create verifying key
    let verifying_key = VerifyingKey::<Sha256>::new(public_key);

    // Parse signature
    let sig = Signature::try_from(signature)
        .map_err(|e| BearDogError::hsm(format!("Invalid RSA signature format: {e}")))?;

    // Verify signature
    Ok(verifying_key.verify(data, &sig).is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519_sign_verify() {
        // Generate a test keypair
        let seed = [42u8; 32];
        let (secret_key, public_key) = generate_ed25519_from_seed(&seed).unwrap();

        let data = b"Hello, BearDog Ed25519!";

        // Sign
        let signature = sign_ed25519(data, &secret_key).unwrap();
        assert_eq!(signature.len(), 64);

        // Verify with correct key
        let valid = verify_ed25519(data, &signature, &public_key).unwrap();
        assert!(valid);

        // Verify with wrong data should fail
        let wrong_data = b"Wrong data";
        let invalid = verify_ed25519(wrong_data, &signature, &public_key).unwrap();
        assert!(!invalid);
    }

    #[test]
    fn test_ecdsa_p256_sign_verify() {
        // Generate a test keypair
        let seed = [99u8; 32];
        let (secret_key, public_key) = generate_ecdsa_p256_from_seed(&seed).unwrap();

        let data = b"Hello, BearDog ECDSA!";

        // Sign
        let signature = sign_ecdsa_p256(data, &secret_key).unwrap();

        // Verify with correct key
        let valid = verify_ecdsa_p256(data, &signature, &public_key).unwrap();
        assert!(valid);

        // Verify with wrong data should fail
        let wrong_data = b"Wrong data";
        let invalid = verify_ecdsa_p256(wrong_data, &signature, &public_key).unwrap();
        assert!(!invalid);
    }

    #[test]
    fn test_ed25519_deterministic() {
        // Same seed should produce same keypair
        let seed = [77u8; 32];
        let (sk1, pk1) = generate_ed25519_from_seed(&seed).unwrap();
        let (sk2, pk2) = generate_ed25519_from_seed(&seed).unwrap();

        assert_eq!(sk1, sk2);
        assert_eq!(pk1, pk2);

        // Same key should produce same signature for same data
        let data = b"Deterministic test";
        let sig1 = sign_ed25519(data, &sk1).unwrap();
        let sig2 = sign_ed25519(data, &sk2).unwrap();

        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_rsa_pss_sign_verify() {
        use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
        use rsa::{RsaPrivateKey, RsaPublicKey};

        // Generate a 2048-bit RSA key for testing (smaller for faster tests)
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng, bits).unwrap();
        let public_key = RsaPublicKey::from(&private_key);

        // Encode keys to DER
        let private_key_der = private_key.to_pkcs8_der().unwrap();
        let public_key_der = public_key.to_public_key_der().unwrap();

        let data = b"Hello, BearDog RSA-PSS!";

        // Sign
        let signature = sign_rsa_pss(data, private_key_der.as_bytes()).unwrap();

        // Verify with correct key
        let valid = verify_rsa_pss(data, &signature, public_key_der.as_bytes()).unwrap();
        assert!(valid);

        // Verify with wrong data should fail
        let wrong_data = b"Wrong data";
        let invalid = verify_rsa_pss(wrong_data, &signature, public_key_der.as_bytes()).unwrap();
        assert!(!invalid);
    }
}
