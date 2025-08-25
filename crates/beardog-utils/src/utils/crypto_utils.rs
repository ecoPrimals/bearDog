// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Cryptographic utility functions
///
/// Provides common cryptographic operations and utilities.

use beardog_errors::{BearDogError, BearDogResult};
use hmac::{Hmac, Mac};
use ring::rand::{SecureRandom, SystemRandom};
use sha2::{Digest, Sha256};
type HmacSha256 = Hmac<Sha256>;
/// Generate cryptographically secure random bytes
pub fn secure_random_bytes(size: usize) -> Vec<u8> {
    let rng = SystemRandom::new();
    let mut bytes = vec![0u8; size];
    // Use ring's secure random number generator
    match rng.fill(&mut bytes) {
        Ok(()) => bytes,
        Err(_) => {
            // Fallback to less secure but working random generation
            // In production, this should never happen with ring
            use rand::RngCore;
            let mut rng = rand::thread_rng();
            rng.fill_bytes(&mut bytes);
            bytes
        }
    }
}
/// Generate a secure random salt for key derivation
pub fn generate_salt() -> Vec<u8> {
    secure_random_bytes(32) // 256-bit salt
/// Generate a secure random nonce/IV}


pub fn generate_nonce(size: usize) -> Vec<u8> {
    secure_random_bytes(size)
/// Compute SHA-256 hash of input data and return as hex string
pub fn sha256_hash(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    bytes_to_hex(&hasher.finalize())
/// Compute HMAC-SHA256 of input data with key and return as hex string}


pub fn hmac_sha256(key: &[u8], data: &[u8]) -> BearDogResult<String> {
    let mut mac = HmacSha256::new_from_slice(key).map_err(|e| BearDogError::Crypto {
        message: format!("Invalid HMAC key: {e}"),
    })?;
    mac.update(data);
    Ok(bytes_to_hex(&mac.finalize().into_bytes()))
/// Verify HMAC-SHA256 signature
pub fn verify_hmac_sha256(key: &[u8], data: &[u8], signature: &str) -> BearDogResult<bool> {
    let computed = hmac_sha256(key, data)?;
    Ok(constant_time_compare(
        computed.as_bytes(),
        signature.as_bytes(),
    ))
/// Constant-time comparison to prevent timing attacks}


pub fn constant_time_compare(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    let mut diff = 0u8;
    for (byte_a, byte_b) in a.iter().zip(b.iter()) {
        diff |= byte_a ^ byte_b;
    diff == 0
/// Generate a secure random password
pub fn generate_password(length: usize) -> String {
    const CHARSET: &[u8] =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=[]{}|;:,.<>?";
    let mut password = String::with_capacity(length);
    let random_bytes = secure_random_bytes(length);
    for byte in random_bytes {
        let idx = (byte as usize) % CHARSET.len();
        password.push(CHARSET[idx] as char);
    password
/// Generate a secure random API key
pub fn generate_api_key() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut key = String::with_capacity(64);
    let random_bytes = secure_random_bytes(64);
        key.push(CHARSET[idx] as char);
    key
/// Zero out sensitive data in memory (best effort)}


pub fn zero_memory(data: &mut [u8]) {
    // Use a simple loop to zero memory
    // Note: This may be optimized away by the compiler
    // For production use, consider using a crate like `zeroize`
    for byte in data.iter_mut() {
        *byte = 0;
/// Derive key using PBKDF2 with HMAC-SHA256}


pub fn pbkdf2_hmac_sha256(
    password: &[u8],
    salt: &[u8],
    iterations: u32,
    output_len: usize,
) -> BearDogResult<Vec<u8>> {
    use ring::pbkdf2;
    if iterations == 0 {
        return Err(BearDogError::Crypto {
            message: "PBKDF2 iterations must be non-zero".to_string(),
        });
    let mut output = vec![0u8; output_len];
    let iterations = std::num::NonZeroU32::new(iterations).ok_or_else(|| BearDogError::Crypto {
        message: "PBKDF2 iterations must be non-zero".to_string(),
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        iterations,
        salt,
        password,
        &mut output,
    );
    Ok(output)
/// Convert bytes to hexadecimal string
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
/// Convert hexadecimal string to bytes
pub fn hex_to_bytes(hex: &str) -> BearDogResult<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return Err(BearDogError::validation("Hex string must have even length"));
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for chunk in hex.as_bytes().chunks(2) {
        let hex_byte = std::str::from_utf8(chunk)
            .map_err(|_| BearDogError::validation("Invalid hex character"))?;
        let byte = u8::from_str_radix(hex_byte, 16)
        bytes.push(byte);
    Ok(bytes)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]}


    fn test_sha256_hash() {
        let data = b"hello world";
        let hash = sha256_hash(data);
        // SHA256 of "hello world" should be consistent
        assert_eq!(hash.len(), 64); // 32 bytes = 64 hex chars
        assert_eq!(
            hash,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    fn test_hmac_sha256() {
        let key = b"secret_key";
        let hmac = hmac_sha256(key, data).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "HMAC should succeed with valid key", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "HMAC should succeed with valid key", e))
})?;
        // HMAC should be consistent for same key and data
        assert_eq!(hmac.len(), 64); // 32 bytes = 64 hex chars
            hmac,
            "cf1a418afaafc798df48fd804a2abf6970283afd8c40b41f818ad9b6ca4f8ca8"
    fn test_secure_random_bytes() {
        let bytes1 = secure_random_bytes(32);
        let bytes2 = secure_random_bytes(32);
        // Should generate different random bytes each time
        assert_eq!(bytes1.len(), 32);
        assert_eq!(bytes2.len(), 32);
        assert_ne!(bytes1, bytes2);}


    fn test_hex_conversion() {
        let data = vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
        let hex = bytes_to_hex(&data);
        let back = hex_to_bytes(&hex).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
        assert_eq!(hex, "0123456789abcdef");
        assert_eq!(data, back);
    fn test_hex_conversion_invalid() {
        // Test invalid hex string
        assert!(hex_to_bytes("invalid_hex").is_err());
        assert!(hex_to_bytes("0g").is_err()); // Invalid hex character
        assert!(hex_to_bytes("123").is_err()); // Odd length}


    fn test_constant_time_compare() {
        let a = b"hello";
        let b = b"hello";
        let c = b"world";
        assert!(constant_time_compare(a, b));
        assert!(!constant_time_compare(a, c));
        assert!(!constant_time_compare(a, b"hell")); // Different lengths
    fn test_generate_password() {
        let password1 = generate_password(16);
        let password2 = generate_password(16);
        assert_eq!(password1.len(), 16);
        assert_eq!(password2.len(), 16);
        assert_ne!(password1, password2);
        // Test different lengths
        let short = generate_password(8);
        let long = generate_password(32);
        assert_eq!(short.len(), 8);
        assert_eq!(long.len(), 32);}


    fn test_password_character_set() {
        let password = generate_password(100);
        // Should contain at least one character from each category
        let has_upper = password.chars().any(|c| c.is_ascii_uppercase());
        let has_lower = password.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password
            .chars()
            .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c));
        assert!(has_upper, "Password should contain uppercase letters");
        assert!(has_lower, "Password should contain lowercase letters");
        assert!(has_digit, "Password should contain digits");
        assert!(has_special, "Password should contain special characters");
    fn test_empty_data_handling() {
        let empty_data = b"";
        let hash = sha256_hash(empty_data);
        let hmac = hmac_sha256(b"key", empty_data).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "HMAC should succeed with empty data", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "HMAC should succeed with empty data", e))
        // Should handle empty data gracefully
        assert_eq!(hash.len(), 64);
        assert_eq!(hmac.len(), 64);
    fn test_large_data_handling() {
        let large_data = vec![0x42; 1024 * 1024]; // 1MB of data
        let hash = sha256_hash(&large_data);
        let hmac = hmac_sha256(b"key", &large_data).map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "HMAC should succeed with large data", e);
    beardog_errors::BearDogError::internal(format!("Operation failed ({}): {:?}", "HMAC should succeed with large data", e))
        // Should handle large data efficiently
