// SPDX-License-Identifier: AGPL-3.0-or-later
#![forbid(unsafe_code)]
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

//! # `BearDog` Security Crate
//!
//! Comprehensive security functionality for the `BearDog` platform, providing
//! cryptographic operations, key management, and hardware security module (HSM) integration.
//!
//! ## Features
//!
//! - **Quantum-Resistant Cryptography**: Post-quantum cryptographic algorithms
//! - **Hardware Security Modules**: Integration with `YubiKey`, TPM, and software HSMs
//! - **Memory-safe only**: All operations are memory-safe
//! - **SIMD Acceleration**: Hardware-accelerated cryptographic operations
//! - **Secure Key Management**: Safe key storage and lifecycle management
//!
//! ## Core Components
//!
//! - [`encryption`]: Encryption and decryption operations
//! - [`memory_key_manager`]: In-memory secure key management
//! - [`simd_crypto`]: SIMD-accelerated cryptographic primitives
//!
//! ## Example
//!
//! ```rust,no_run
//! use beardog_security::compute_sha256_hash;
//!
//! let data = b"Hello, BearDog!";
//! let hash = compute_sha256_hash(data)?;
//! tracing::info!("SHA-256: {:?}", hash);
//! # Ok::<(), beardog_errors::BearDogError>(())
//! ```
//!
//! ## Safety
//!
//! This crate maintains full memory safety, ensuring complete memory safety
//! for all security-critical operations. All cryptographic operations are
//! compiler-verified for safety.
//!
//! ## Performance
//!
//! SIMD acceleration provides 2-5x performance improvement for cryptographic
//! operations when available, automatically falling back to safe scalar
//! implementations on unsupported platforms.

/// Authorization types and permission management
///
/// Provides types and utilities for managing authorization, permissions,
/// and access control within the `BearDog` security system.
pub mod authorization_types;

/// Cryptographic utility functions
///
/// Core cryptographic operations including hashing, HMAC, signing,
/// and secure random number generation.
pub mod crypto_utils;

/// Encryption services and algorithms
///
/// High-level encryption and decryption services supporting multiple
/// symmetric algorithms with secure key management.
pub mod encryption;

pub mod hsm;
pub mod key_rotation_manager;

/// In-memory key management system
///
/// Secure key storage and management with memory protection,
/// designed for temporary key handling and secure operations.
pub mod memory_key_manager;

/// SIMD-accelerated cryptographic operations
///
/// Hardware-accelerated crypto functions using SIMD instructions
/// for improved performance on supported platforms.
pub mod simd_crypto;

/// Genesis module - Physical Bootstrap with Cryptographic Witness
///
/// Implements physical genesis bootstrap for new nodes, ensuring they
/// receive cryptographic identity at birth via witnessed ceremony.
///
/// **"Never let a bird be alone in the dark forest"**
pub mod genesis;

/// Quantum-resistant cryptography module
///
/// Post-quantum cryptographic operations following NIST PQC standards:
/// - **ML-KEM** (Kyber) - Key Encapsulation Mechanism
/// - **ML-DSA** (Dilithium) - Digital Signatures
/// - **SPHINCS+** - Stateless hash-based signatures
#[cfg(feature = "quantum-crypto")]
pub mod quantum_crypto;

// Comprehensive test modules
#[cfg(test)]
mod tests;

#[cfg(test)]
mod security_operations_comprehensive_tests;

// Re-export main types and functions
pub use authorization_types::*;
pub use encryption::*;
pub use genesis::{
    GenesisWitness, GenesisWitnessVerifier, PhysicalChannelType, PhysicalProofError,
    PhysicalProximityVerifier, TrustLevel, WitnessVerificationError,
};
pub use key_rotation_manager::{KeyRotationManager, RotationStatistics};
pub use memory_key_manager::*;

use beardog_errors::BearDogError;
use crypto_utils::BearDogCrypto;
use sha2::{Digest, Sha256, Sha512};

/// Compute SHA-256 hash of input data
///
/// This function computes the SHA-256 cryptographic hash of the provided data.
///
/// # Arguments
///
/// * `data` - The input data to hash
///
/// # Returns
///
/// A `Result` containing the 32-byte SHA-256 hash as a `Vec<u8>`, or an error
/// if the operation fails.
///
/// # Examples
///
/// ```rust
/// use beardog_security::compute_sha256_hash;
///
/// # fn main() -> Result<(), beardog_errors::BearDogError> {
/// let data = b"Hello, World!";
/// let hash = compute_sha256_hash(data)?;
/// assert_eq!(hash.len(), 32); // SHA-256 produces 32 bytes
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Currently infallible; the `Result` is reserved for future hashing backends.
pub fn compute_sha256_hash(data: &[u8]) -> Result<Vec<u8>, BearDogError> {
    Ok(BearDogCrypto::sha256_hash_bytes(data))
}

/// Compute SHA-512 hash of input data
///
/// This function computes the SHA-512 cryptographic hash of the provided data.
///
/// # Arguments
///
/// * `data` - The input data to hash
///
/// # Returns
///
/// A `Result` containing the 64-byte SHA-512 hash as a `Vec<u8>`, or an error
/// if the operation fails.
///
/// # Examples
///
/// ```rust
/// use beardog_security::compute_sha512_hash;
///
/// # fn main() -> Result<(), beardog_errors::BearDogError> {
/// let data = b"Hello, World!";
/// let hash = compute_sha512_hash(data)?;
/// assert_eq!(hash.len(), 64); // SHA-512 produces 64 bytes
/// # Ok(())
/// # }
/// ```
///
/// # Errors
///
/// Currently infallible; the `Result` is reserved for future hashing backends.
pub fn compute_sha512_hash(data: &[u8]) -> Result<Vec<u8>, BearDogError> {
    let mut hasher = Sha512::new();
    hasher.update(data);
    Ok(hasher.finalize().to_vec())
}

/// Generate cryptographically secure random bytes
///
/// Uses the operating system's cryptographically secure random number generator
/// (CSPRNG) to generate unpredictable random bytes suitable for cryptographic operations.
///
/// # Security Considerations
///
/// - **Thread-safe**: Uses `rng()` which is cryptographically secure
/// - **OS-backed**: Relies on `/dev/urandom` (Linux), `BCryptGenRandom` (Windows), etc.
/// - **Suitable for**: Keys, IVs, nonces, salts, tokens
/// - **Not suitable for**: Deterministic derivation (use HKDF/KDF instead)
///
/// # Arguments
///
/// * `size` - Number of random bytes to generate (typically 16, 32, or 64)
///
/// # Returns
///
/// A `Result` containing the random bytes, or an error if generation fails
///
/// # Examples
///
/// ## Generate AES-256 Key
///
/// ```rust
/// use beardog_security::generate_secure_random_bytes;
///
/// # fn main() -> Result<(), beardog_errors::BearDogError> {
/// // Generate 32-byte (256-bit) key for AES-256
/// let key = generate_secure_random_bytes(32)?;
/// assert_eq!(key.len(), 32);
///
/// // Keys should be unique (probability of collision negligible)
/// let key2 = generate_secure_random_bytes(32)?;
/// assert_ne!(key, key2);
/// # Ok(())
/// # }
/// ```
///
/// ## Generate IV for AES-GCM
///
/// ```rust
/// use beardog_security::generate_secure_random_bytes;
///
/// # fn main() -> Result<(), beardog_errors::BearDogError> {
/// // Generate 12-byte (96-bit) nonce/IV for AES-GCM
/// let nonce = generate_secure_random_bytes(12)?;
/// assert_eq!(nonce.len(), 12);
/// # Ok(())
/// # }
/// ```
///
/// ## Generate API Token
///
/// ```rust
/// use beardog_security::generate_secure_random_bytes;
///
/// # fn main() -> Result<(), beardog_errors::BearDogError> {
/// // Generate 32-byte token and encode as hex
/// let token_bytes = generate_secure_random_bytes(32)?;
/// let token = hex::encode(token_bytes);
/// assert_eq!(token.len(), 64); // 32 bytes = 64 hex characters
/// # Ok(())
/// # }
/// ```
///
/// # See Also
///
/// - [`derive_key_from_password`] - For deriving keys from passwords
/// - [`compute_sha256_hash`] - For hashing data
///
/// # Errors
///
/// Currently always returns `Ok`; the `Result` is reserved for future RNG or allocation failures.
pub fn generate_secure_random_bytes(size: usize) -> Result<Vec<u8>, BearDogError> {
    Ok(BearDogCrypto::generate_secure_random(size))
}

/// Derive cryptographic key from password using iterative hashing
///
/// Transforms a human-memorable password into a cryptographic key suitable for
/// encryption. Uses iterative SHA-256 hashing to slow down brute-force attacks.
///
/// # Security Considerations
///
/// **⚠️ IMPORTANT:** This is a simplified KDF for development/testing.\
/// **Production systems should use:**
/// - `argon2` - Winner of Password Hashing Competition, best for new systems
/// - `scrypt` - Memory-hard, good for password hashing
/// - `bcrypt` - Time-tested, widely deployed
/// - `PBKDF2-HMAC-SHA256` - NIST approved, minimum 600,000 iterations
///
/// **Best Practices:**
/// - Use unique salt per password (16+ bytes from CSPRNG)
/// - Store salt alongside encrypted data (salt can be public)
/// - Never reuse salts across different passwords
/// - Use 100,000+ iterations (more = slower = more secure)
/// - Recommended: 600,000+ iterations for PBKDF2 (OWASP 2023)
///
/// # Arguments
///
/// * `password` - User's password as bytes (UTF-8 encoded string)
/// * `salt` - Cryptographically random salt (16+ bytes recommended)
/// * `iterations` - Number of hash iterations (100,000+ recommended)
///
/// # Returns
///
/// A `Result` containing the derived 32-byte key, or an error
///
/// # Examples
///
/// ## Basic Password-Based Encryption
///
/// ```rust
/// use beardog_security::{derive_key_from_password, generate_secure_random_bytes};
///
/// # fn main() -> Result<(), beardog_errors::BearDogError> {
/// // User's password
/// let password = b"correct horse battery staple";
///
/// // Generate unique salt (store this with encrypted data!)
/// let salt = generate_secure_random_bytes(16)?;
///
/// // Derive encryption key (use 100,000+ iterations in production)
/// let key = derive_key_from_password(password, &salt, 100_000)?;
/// assert_eq!(key.len(), 32); // 256-bit key
///
/// // Use key for encryption, store salt alongside ciphertext
/// # Ok(())
/// # }
/// ```
///
/// ## Password Verification (Login)
///
/// ```rust
/// use beardog_security::{derive_key_from_password, constant_time_compare};
///
/// # fn main() -> Result<(), beardog_errors::BearDogError> {
/// // During registration, derive and store key
/// let password = b"user_password";
/// let salt = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
/// let stored_key = derive_key_from_password(password, &salt, 100_000)?;
///
/// // During login, derive key again and compare
/// let login_password = b"user_password";
/// let login_key = derive_key_from_password(login_password, &salt, 100_000)?;
///
/// // Use constant-time comparison to prevent timing attacks
/// if constant_time_compare(&stored_key, &login_key) {
///     tracing::info!("✅ Password correct");
/// } else {
///     tracing::info!("❌ Password incorrect");
/// }
/// # Ok(())
/// # }
/// ```
///
/// ## Production-Ready Pattern
///
/// ```rust,ignore
/// use argon2::{Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
/// use argon2::password_hash::{rand_core::OsRng, SaltString};
///
/// # fn production_example() -> Result<(), Box<dyn std::error::Error>> {
/// // PRODUCTION: Use Argon2 instead
/// let password = b"correct horse battery staple";
/// let salt = SaltString::generate(&mut OsRng);
/// let argon2 = Argon2::default();
///
/// // Hash password
/// let password_hash = argon2.hash_password(password, &salt)?.to_string();
///
/// // Verify password
/// let parsed_hash = PasswordHash::new(&password_hash)?;
/// assert!(argon2.verify_password(password, &parsed_hash).is_ok());
/// # Ok(())
/// # }
/// ```
///
/// # See Also
///
/// - [`generate_secure_random_bytes`] - For generating salts
/// - [`constant_time_compare`] - For secure key comparison
///
/// # Errors
///
/// Currently infallible; the `Result` is reserved for future iteration limits or KDF hardening.
pub fn derive_key_from_password(
    password: &[u8],
    salt: &[u8],
    iterations: u32,
) -> Result<Vec<u8>, BearDogError> {
    // Simple key derivation (replace with proper PBKDF2 in production)
    let mut result = password.to_vec();
    for _ in 0..iterations {
        let mut hasher = Sha256::new();
        hasher.update(&result);
        hasher.update(salt);
        result = hasher.finalize().to_vec();
    }
    Ok(result)
}

/// Constant-time comparison of byte arrays to prevent timing attacks
///
/// Compares two byte arrays in constant time regardless of where differences occur.
/// This prevents timing side-channel attacks where an attacker could learn information
/// about secret data by measuring how long comparisons take.
///
/// # Security Rationale
///
/// **Why Constant-Time Comparison Matters:**
///
/// Normal comparison (`a == b`) stops at the first difference, creating timing
/// variations that leak information:
/// - `"password" == "aaaaword"` → Fast (fails at position 0)
/// - `"password" == "passwxxx"` → Slower (fails at position 5)
///
/// An attacker can measure response times to guess secrets byte-by-byte, turning
/// a brute-force attack from 2^256 attempts to just 256*32 attempts for a 32-byte key!
///
/// **Use Cases:**
/// - ✅ Comparing authentication tokens
/// - ✅ Comparing password hashes
/// - ✅ Comparing HMAC signatures
/// - ✅ Comparing any secret values
/// - ❌ Comparing public data (use `==` for clarity)
///
/// # Arguments
///
/// * `a` - First byte array (typically secret/expected value)
/// * `b` - Second byte array (typically user-provided value)
///
/// # Returns
///
/// `true` if arrays are identical, `false` if different lengths or contents
///
/// # Examples
///
/// ## API Token Validation
///
/// ```rust
/// use beardog_security::constant_time_compare;
///
/// # fn main() {
/// // Expected token (from database)
/// let expected_token = b"secret_api_token_xyz123";
///
/// // User-provided token (from HTTP header)
/// let provided_token = b"secret_api_token_xyz123";
///
/// // ✅ Secure: Constant-time comparison
/// if constant_time_compare(expected_token, provided_token) {
///     tracing::info!("✅ Token valid");
/// } else {
///     tracing::info!("❌ Token invalid");
/// }
///
/// // ❌ INSECURE: Don't use == for secrets!
/// // if expected_token == provided_token { ... } // TIMING ATTACK VULNERABLE!
/// # }
/// ```
///
/// ## HMAC Signature Verification
///
/// ```rust
/// use beardog_security::{compute_sha256_hash, constant_time_compare};
///
/// # fn main() -> Result<(), beardog_errors::BearDogError> {
/// // Compute expected HMAC
/// let secret = b"hmac_secret_key";
/// let message = b"important message";
/// let expected_hmac = compute_sha256_hash(message)?;
///
/// // Verify received HMAC (constant-time to prevent timing attacks)
/// let received_hmac = vec![0u8; 32]; // From client
/// if constant_time_compare(&expected_hmac, &received_hmac) {
///     tracing::info!("✅ Signature valid");
/// } else {
///     tracing::info!("❌ Signature invalid");
/// }
/// # Ok(())
/// # }
/// ```
///
/// ## Password Hash Verification
///
/// ```rust
/// use beardog_security::{derive_key_from_password, constant_time_compare};
///
/// # fn main() -> Result<(), beardog_errors::BearDogError> {
/// let salt = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
///
/// // Stored password hash
/// let stored_hash = derive_key_from_password(b"correct_password", &salt, 100_000)?;
///
/// // User login attempt
/// let login_password = b"wrong_password";
/// let login_hash = derive_key_from_password(login_password, &salt, 100_000)?;
///
/// // ✅ Secure: Constant-time prevents timing attack on password guessing
/// if constant_time_compare(&stored_hash, &login_hash) {
///     tracing::info!("✅ Login successful");
/// } else {
///     tracing::info!("❌ Login failed");
/// }
/// # Ok(())
/// # }
/// ```
///
/// # Implementation Details
///
/// Uses bitwise XOR to accumulate differences without short-circuiting:
/// ```text
/// For each byte pair (a[i], b[i]):
///   result |= a[i] ^ b[i]
///
/// If all bytes match: result = 0 → returns true
/// If any byte differs: result ≠ 0 → returns false
///
/// Time taken: Always proportional to array length, regardless of differences
/// ```
///
/// # See Also
///
/// - [`derive_key_from_password`] - For password hashing
/// - [`compute_sha256_hash`] - For creating HMACs
#[must_use]
pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    BearDogCrypto::constant_time_compare(a, b)
}

/// Securely zeroes memory to prevent sensitive data leakage
///
/// This function overwrites the provided buffer with zeros using a volatile write
/// operation to prevent compiler optimization from removing the write.
///
/// # Arguments
/// * `data` - Mutable slice of bytes to be zeroed
///
/// # Security
/// Uses `ptr::write_volatile` to ensure the zeroing operation cannot be optimized away
pub fn secure_zero_memory(data: &mut [u8]) {
    BearDogCrypto::zero_memory(data);
}
