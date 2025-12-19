#!/usr/bin/env python3
"""Add missing # Errors documentation to crypto_service functions."""

import re

# File: algorithms/asymmetric.rs
asymmetric_file = "crates/beardog-core/src/crypto_service/algorithms/asymmetric.rs"

# Patches to apply
patches = [
    # sign_ed25519
    {
        "file": asymmetric_file,
        "before": "/// Ed25519 provides 128-bit security level and is resistant to:\n/// - Side-channel attacks\n/// - Fault attacks\n/// - Weak randomness\npub fn sign_ed25519",
        "after": """/// Ed25519 provides 128-bit security level and is resistant to:
/// - Side-channel attacks
/// - Fault attacks
/// - Weak randomness
///
/// # Errors
///
/// Returns error if secret key format is invalid (not properly expanded 64-byte format).
pub fn sign_ed25519"""
    },
    # generate_ed25519_from_seed
    {
        "file": asymmetric_file,
        "before": "/// Tuple of (`secret_key`, `public_key`) where:\n/// - `secret_key` is 64 bytes (expanded secret key)\n/// - `public_key` is 32 bytes\npub fn generate_ed25519_from_seed",
        "after": """/// Tuple of (`secret_key`, `public_key`) where:
/// - `secret_key` is 64 bytes (expanded secret key)
/// - `public_key` is 32 bytes
///
/// # Errors
///
/// Returns error if keypair generation fails (extremely rare, indicates library issue).
pub fn generate_ed25519_from_seed"""
    },
    # sign_ecdsa_p256
    {
        "file": asymmetric_file,
        "before": "/// ECDSA requires high-quality randomness for each signature.\n/// Weak RNG can lead to private key recovery. Consider using deterministic\n/// ECDSA (RFC 6979) for better security.\npub fn sign_ecdsa_p256",
        "after": """/// ECDSA requires high-quality randomness for each signature.
/// Weak RNG can lead to private key recovery. Consider using deterministic
/// ECDSA (RFC 6979) for better security.
///
/// # Errors
///
/// Returns error if:
/// - Secret key is invalid or out of range for P-256 curve
/// - Signing operation fails
pub fn sign_ecdsa_p256"""
    },
    # verify_ecdsa_p256
    {
        "file": asymmetric_file,
        "before": "/// # Returns\n///\n/// `true` if signature is valid, `false` otherwise\npub fn verify_ecdsa_p256",
        "after": """/// # Returns
///
/// `true` if signature is valid, `false` otherwise
///
/// # Errors
///
/// Returns error if:
/// - Public key format is invalid
/// - Signature format is malformed
pub fn verify_ecdsa_p256"""
    },
    # generate_ecdsa_p256_from_seed
    {
        "file": asymmetric_file,
        "before": "/// Tuple of (`secret_key`, `public_key`) where:\n/// - `secret_key` is 32 bytes\n/// - `public_key` is 33 bytes (compressed format)\npub fn generate_ecdsa_p256_from_seed",
        "after": """/// Tuple of (`secret_key`, `public_key`) where:
/// - `secret_key` is 32 bytes
/// - `public_key` is 33 bytes (compressed format)
///
/// # Errors
///
/// Returns error if seed value is out of range for P-256 curve.
pub fn generate_ecdsa_p256_from_seed"""
    },
]

# Read asymmetric.rs
with open(asymmetric_file, 'r') as f:
    content = f.read()

# Apply patches
for patch in patches:
    if patch["file"] == asymmetric_file:
        content = content.replace(patch["before"], patch["after"])

# Write back
with open(asymmetric_file, 'w') as f:
    f.write(content)

print(f"✅ Added # Errors docs to {asymmetric_file}")

# Fix hmac_sha256 - evolve .expect() to proper error handling
hashing_file = "crates/beardog-core/src/crypto_service/algorithms/hashing.rs"

with open(hashing_file, 'r') as f:
    content = f.read()

# Change hmac_sha256 to return Result and handle error properly
old_hmac = '''/// 32-byte authentication tag
#[must_use]
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any length");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}'''

new_hmac = '''/// 32-byte authentication tag
///
/// # Errors
///
/// This function is infallible in practice (HMAC accepts any key length),
/// but returns Result for API consistency.
#[must_use]
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> Result<Vec<u8>> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    // HMAC can accept any key length, so this should never fail
    let mut mac = HmacSha256::new_from_slice(key)
        .map_err(|e| BearDogError::hsm(format!("HMAC initialization failed: {e}")))?;
    mac.update(data);
    Ok(mac.finalize().into_bytes().to_vec())
}'''

content = content.replace(old_hmac, new_hmac)

with open(hashing_file, 'w') as f:
    f.write(content)

print(f"✅ Evolved hmac_sha256 to proper error handling in {hashing_file}")
print("✅ All clippy error docs added!")

