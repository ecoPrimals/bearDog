// SPDX-License-Identifier: AGPL-3.0-or-later

//! Recovery Ephemeral Tests

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    /// TEST 3: Ephemeral Key Generation
    ///
    /// Tests ephemeral key generation for recovery:
    /// - Generate temporary keys
    /// - Verify key expiration
    /// - Test key rotation
    /// - Validate key properties
    #[test]
    fn test_ephemeral_key_generation() {
        // Generate ephemeral key with 1 hour expiration
        let expiration = std::time::Duration::from_secs(3600);
        let ephemeral_key = EphemeralKey::generate(expiration);

        // Verify key properties
        assert_eq!(ephemeral_key.key_data().len(), 32); // 256-bit key
        assert!(!ephemeral_key.is_expired());

        // Test key validity
        assert!(ephemeral_key.is_valid());

        // Generate multiple keys - should be unique due to cryptographic randomness
        let key1 = EphemeralKey::generate(expiration);
        let key2 = EphemeralKey::generate(expiration);
        // Modern pattern: Keys should be cryptographically unique, not timing-dependent
        let keys_equal = key1.key_data() == key2.key_data();
        assert!(!keys_equal, "Keys should be cryptographically unique");

        // Test expired key - modern pattern: 1 nanosecond instantly expired
        let short_expiration = std::time::Duration::from_nanos(1);
        let expiring_key = EphemeralKey::generate(short_expiration);

        // No sleep needed - CPU cycles already passed 1 nanosecond
        assert!(expiring_key.is_expired());
        assert!(!expiring_key.is_valid());

        // Test key rotation - modern pattern: rotation logic doesn't need timing
        let original_key = EphemeralKey::generate(expiration);
        let rotated_key = original_key.rotate();

        // Rotated key should be different due to rotation logic, not timing
        let keys_same = original_key.key_data() == rotated_key.key_data();
        assert!(!keys_same, "Rotated key should be different from original");
        assert!(!rotated_key.is_expired());

        // Test key derivation
        let derived = ephemeral_key.derive_key(b"context_data");
        assert_eq!(derived.len(), 32);
        assert_ne!(derived, ephemeral_key.key_data());
    }
}
