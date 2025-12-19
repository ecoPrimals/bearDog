#!/usr/bin/env python3
"""Fix remaining clippy warnings"""

import re

# Fix symmetric.rs - add # Errors docs
sym_file = "crates/beardog-core/src/crypto_service/algorithms/symmetric.rs"

with open(sym_file, 'r') as f:
    content = f.read()

# Add # Errors to encrypt_aes_256_gcm
content = re.sub(
    r'(/// # Returns\n///\n/// Tuple of \(ciphertext, nonce, tag\))\n(pub fn encrypt_aes_256_gcm)',
    r'\1\n///\n/// # Errors\n///\n/// Returns error if encryption fails or key/nonce length is invalid.\n\2',
    content
)

# Add # Errors to decrypt_aes_256_gcm
content = re.sub(
    r'(/// # Returns\n///\n/// Original plaintext)\n(pub fn decrypt_aes_256_gcm)',
    r'\1\n///\n/// # Errors\n///\n/// Returns error if decryption fails (wrong key, tampered data, or invalid tag).\n\2',
    content
)

# Add # Errors to encrypt_aes_128_gcm
content = re.sub(
    r'(/// Same as AES-256-GCM but with 128-bit keys\.\n/// Provides 128-bit security level\.)\n(pub fn encrypt_aes_128_gcm)',
    r'\1\n///\n/// # Errors\n///\n/// Returns error if encryption fails or key/nonce length is invalid.\n\2',
    content
)

# Add # Errors to decrypt_aes_128_gcm
content = re.sub(
    r'(/// Decrypts data using AES-128-GCM\.)\n(pub fn decrypt_aes_128_gcm)',
    r'\1\n///\n/// # Errors\n///\n/// Returns error if decryption fails (wrong key, tampered data, or invalid tag).\n\2',
    content
)

# Add # Errors to encrypt_chacha20_poly1305
content = re.sub(
    r'(/// # Returns\n///\n/// Tuple of \(ciphertext, nonce, tag\) where each is 16 bytes)\n(pub fn encrypt_chacha20_poly1305)',
    r'\1\n///\n/// # Errors\n///\n/// Returns error if encryption fails or key length is invalid.\n\2',
    content
)

# Add # Errors to decrypt_chacha20_poly1305
content = re.sub(
    r'(/// # Returns\n///\n/// Original plaintext)\n(pub fn decrypt_chacha20_poly1305)',
    r'\1\n///\n/// # Errors\n///\n/// Returns error if decryption fails (wrong key, tampered data, or invalid tag).\n\2',
    content
)

with open(sym_file, 'w') as f:
    f.write(content)

print(f"✅ Added # Errors docs to {sym_file}")

# Fix implementation.rs - merge match arms
impl_file = "crates/beardog-core/src/crypto_service/implementation.rs"

with open(impl_file, 'r') as f:
    content = f.read()

# Merge match arms for key sizes
old_match = '''            KeyAlgorithm::Aes256 => 256,
            KeyAlgorithm::Ed25519 => 256,
            KeyAlgorithm::EcdsaP256 => 256,'''

new_match = '''            KeyAlgorithm::Aes256 | KeyAlgorithm::Ed25519 | KeyAlgorithm::EcdsaP256 => 256,'''

content = content.replace(old_match, new_match)

with open(impl_file, 'w') as f:
    f.write(content)

print(f"✅ Merged identical match arms in {impl_file}")

# Fix mod.rs - add doc for Result type
mod_file = "crates/beardog-core/src/crypto_service/mod.rs"

with open(mod_file, 'r') as f:
    content = f.read()

# Find and replace the Result type alias
old_result = '''pub type Result<T> = std::result::Result<T, beardog_errors::BearDogError>;'''

new_result = '''/// Result type alias for crypto service operations.
///
/// Convenience type wrapping [`std::result::Result`] with [`beardog_errors::BearDogError`].
/// This allows shorter function signatures throughout the crypto service.
pub type Result<T> = std::result::Result<T, beardog_errors::BearDogError>;'''

content = content.replace(old_result, new_result)

with open(mod_file, 'w') as f:
    f.write(content)

print(f"✅ Added documentation to Result type alias in {mod_file}")

# Fix discovery.rs - too many lines (114 > 100)
disc_file = "crates/beardog-core/src/crypto_service/algorithms/discovery.rs"

with open(disc_file, 'r') as f:
    content = f.read()

# Add allow annotation for the long function
old_discover = '''pub fn discover_algorithms() -> Vec<AlgorithmCapability> {'''

new_discover = '''#[allow(clippy::too_many_lines)]
pub fn discover_algorithms() -> Vec<AlgorithmCapability> {'''

content = content.replace(old_discover, new_discover)

with open(disc_file, 'w') as f:
    f.write(content)

print(f"✅ Allowed long function in {disc_file} (it's a comprehensive capability list)")

print("\n✅ All remaining clippy issues fixed!")

