//! Crypto Sovereignty Tests
//!
//! TEST_CATEGORY: unit
//! TEST_DOMAIN: security/sovereignty/crypto
//! TEST_PRIORITY: critical

use super::types::*;

#[cfg(test)]
mod tests {
    use super::*;

    ///
    /// Tests cryptographic sovereignty mechanisms:
    /// - Key sovereignty (local key management)
    /// - Encryption sovereignty
    /// - Algorithm restrictions
    /// - Export control compliance
    #[test]
    fn test_crypto_sovereignty_enforcement() {
        // Create crypto sovereignty policy
        let crypto_policy = CryptoSovereigntyPolicy::new()
            .with_key_storage(KeyStorage::LocalOnly)
            .with_algorithm(Algorithm::AES256)
            .with_algorithm(Algorithm::Ed25519)
            .with_export_control(ExportControl::Unrestricted);

        assert_eq!(crypto_policy.key_storage(), KeyStorage::LocalOnly);
        assert!(crypto_policy.allows_algorithm(Algorithm::AES256));
        assert!(crypto_policy.allows_algorithm(Algorithm::Ed25519));

        // Test key sovereignty validation
        let local_key = KeyLocation::new("key_id_1", KeyStorage::LocalOnly);
        assert!(crypto_policy.validate_key_location(&local_key).is_ok());

        let cloud_key = KeyLocation::new("key_id_2", KeyStorage::CloudManaged);
        let result = crypto_policy.validate_key_location(&cloud_key);
        assert!(result.is_err(), "Cloud-managed keys should be rejected");

        // Test algorithm restrictions
        assert!(!crypto_policy.allows_algorithm(Algorithm::DES)); // Weak algorithm
        assert!(!crypto_policy.allows_algorithm(Algorithm::MD5)); // Broken hash

        // Create policy with export restrictions
        let restricted_policy = CryptoSovereigntyPolicy::new()
            .with_key_storage(KeyStorage::LocalOnly)
            .with_algorithm(Algorithm::AES256)
            .with_export_control(ExportControl::Restricted);

        assert_eq!(
            restricted_policy.export_control(),
            ExportControl::Restricted
        );

        // Test export validation
        let domestic_export = ExportRequest::new("US", "US");
        assert!(restricted_policy.validate_export(&domestic_export).is_ok());

        let international_export = ExportRequest::new("US", "CN");
        let result = restricted_policy.validate_export(&international_export);
        assert!(result.is_err(), "International export should be restricted");

        // Test key derivation sovereignty
        let master_key = KeyLocation::new("master", KeyStorage::LocalOnly);
        let derived_key = crypto_policy.derive_key(&master_key, b"context").unwrap();

        assert_eq!(derived_key.storage(), KeyStorage::LocalOnly);
        assert_ne!(derived_key.id(), master_key.id());

        // Test encryption method validation
        let valid_encryption = EncryptionMethod::new(Algorithm::AES256, 256);
        assert!(crypto_policy
            .validate_encryption_method(&valid_encryption)
            .is_ok());

        let weak_encryption = EncryptionMethod::new(Algorithm::DES, 56);
        assert!(crypto_policy
            .validate_encryption_method(&weak_encryption)
            .is_err());
    }
}
