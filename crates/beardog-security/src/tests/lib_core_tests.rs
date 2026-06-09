// SPDX-License-Identifier: AGPL-3.0-or-later
//! Core library function tests (hashing, RNG, KDF, constant-time compare, zero).

use crate::{
    compute_sha256_hash, compute_sha512_hash, constant_time_compare, derive_key_from_password,
    generate_secure_random_bytes, secure_zero_memory,
};

#[test]
fn test_security_lib_accessible() {}

#[test]
fn test_sha256_hash_function() {
    let data = b"test";
    let result = compute_sha256_hash(data);
    assert!(result.is_ok());
    let hash = result.expect("SHA256 hash should succeed for test input");
    assert_eq!(hash.len(), 32);
}

#[test]
fn test_sha256_empty_input() {
    let result = compute_sha256_hash(b"");
    assert!(result.is_ok());
    assert_eq!(result.expect("SHA256 empty input should succeed").len(), 32);
}

#[test]
fn test_sha256_consistency() {
    let data = b"consistent data";
    let hash1 = compute_sha256_hash(data).expect("SHA256 should succeed");
    let hash2 = compute_sha256_hash(data).expect("SHA256 should succeed");
    assert_eq!(hash1, hash2);
}

#[test]
fn test_sha512_hash_function() {
    let data = b"test data";
    let result = compute_sha512_hash(data);
    assert!(result.is_ok());
    let hash = result.expect("SHA512 hash should succeed for test input");
    assert_eq!(hash.len(), 64);
}

#[test]
fn test_sha512_empty_input() {
    let result = compute_sha512_hash(b"");
    assert!(result.is_ok());
    assert_eq!(result.expect("SHA512 empty input should succeed").len(), 64);
}

#[test]
fn test_generate_random_bytes() {
    let bytes = generate_secure_random_bytes(32);
    assert!(bytes.is_ok());
    let random = bytes.expect("secure random bytes should succeed");
    assert_eq!(random.len(), 32);
}

#[test]
fn test_random_bytes_uniqueness() {
    let bytes1 = generate_secure_random_bytes(32).expect("random bytes");
    let bytes2 = generate_secure_random_bytes(32).expect("random bytes");
    assert_ne!(bytes1, bytes2);
}

#[test]
fn test_random_bytes_various_sizes() {
    for size in [16, 24, 32, 48, 64] {
        let bytes = generate_secure_random_bytes(size);
        assert!(bytes.is_ok());
        assert_eq!(bytes.expect("random bytes for size").len(), size);
    }
}

#[test]
fn test_derive_key_from_password() {
    let password = b"test_password";
    let salt = b"test_salt_1234567890";
    let result = derive_key_from_password(password, salt, 1000);
    assert!(result.is_ok());
    let key = result.expect("PBKDF2 key derivation should succeed");
    assert_eq!(key.len(), 32);
}

#[test]
fn test_key_derivation_consistency() {
    let password = b"password";
    let salt = b"salt12345678";
    let key1 = derive_key_from_password(password, salt, 100).expect("derive key");
    let key2 = derive_key_from_password(password, salt, 100).expect("derive key");
    assert_eq!(key1, key2);
}

#[test]
fn test_key_derivation_salt_dependency() {
    let password = b"password";
    let salt1 = b"salt1";
    let salt2 = b"salt2";
    let key1 = derive_key_from_password(password, salt1, 100).expect("derive key");
    let key2 = derive_key_from_password(password, salt2, 100).expect("derive key");
    assert_ne!(key1, key2);
}

#[test]
fn test_constant_time_compare_equal() {
    let a = b"secret_data";
    let b = b"secret_data";
    assert!(constant_time_compare(a, b));
}

#[test]
fn test_constant_time_compare_different() {
    let a = b"secret_data";
    let b = b"secret_diff";
    assert!(!constant_time_compare(a, b));
}

#[test]
fn test_constant_time_compare_different_lengths() {
    let a = b"short";
    let b = b"longer_data";
    assert!(!constant_time_compare(a, b));
}

#[test]
fn test_constant_time_compare_empty() {
    let a = b"";
    let b = b"";
    assert!(constant_time_compare(a, b));
}

#[test]
fn test_secure_zero_memory() {
    let mut data = vec![1u8, 2, 3, 4, 5];
    secure_zero_memory(&mut data);
    assert_eq!(data, vec![0u8, 0, 0, 0, 0]);
}

#[test]
fn test_secure_zero_large_data() {
    let mut data = vec![0xFFu8; 1024];
    secure_zero_memory(&mut data);
    assert!(data.iter().all(|&b| b == 0));
}

#[test]
fn test_crypto_utils_module() {
    use crate::crypto_utils::BearDogCrypto;
    assert!(BearDogCrypto::constant_time_compare(b"a", b"a"));
}

#[test]
fn test_module_exports() {
    assert_eq!(crate::compute_sha256_hash(b"x").expect("SHA256").len(), 32);
    assert_eq!(crate::compute_sha512_hash(b"x").expect("SHA512").len(), 64);
    assert_eq!(
        crate::generate_secure_random_bytes(4)
            .expect("random")
            .len(),
        4
    );
    assert!(crate::constant_time_compare(b"a", b"a"));
    assert!(crate::derive_key_from_password(b"p", b"s", 8).is_ok());
    let mut z = [1u8, 2, 3];
    crate::secure_zero_memory(&mut z);
    assert_eq!(z, [0, 0, 0]);
}

#[test]
fn test_authorization_types_module() {
    use crate::authorization_types::Subject;
    assert!(core::mem::size_of::<Subject>() > 0);
}

#[test]
fn test_encryption_module() {
    use crate::encryption::EncryptionService;
    assert!(core::mem::size_of::<EncryptionService>() > 0);
}

#[test]
fn test_memory_key_manager_module() {
    use crate::memory_key_manager::MemoryKeyManager;
    assert!(core::mem::size_of::<MemoryKeyManager>() > 0);
}

#[test]
fn test_simd_crypto_module() {
    use crate::simd_crypto::SafeCryptoEngine;
    assert!(core::mem::size_of::<SafeCryptoEngine>() > 0);
}

#[tokio::test]
async fn test_async_security() {
    tokio::task::yield_now().await;
}

#[tokio::test]
async fn test_async_hash_operations() {
    let data = b"async test data";
    let hash = compute_sha256_hash(data).expect("SHA256 in async test");
    assert_eq!(hash.len(), 32);
}
