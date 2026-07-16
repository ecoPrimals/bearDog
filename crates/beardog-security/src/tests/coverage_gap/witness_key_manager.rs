// SPDX-License-Identifier: AGPL-3.0-or-later

// ========================================================================
// genesis/witness: ~81% → test uncovered error paths
// ========================================================================

mod witness_tests {
    use crate::genesis::types::PhysicalChannelType;
    use crate::genesis::witness::*;
    use ed25519_dalek::{Signer, SigningKey};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    fn create_signed_witness(device_id: &str, node_id: &str) -> GenesisWitness {
        let signing_key = SigningKey::from_bytes(&[42u8; 32]);
        let public_key = signing_key.verifying_key().to_bytes().to_vec();
        let timestamp = current_timestamp();

        let mut hasher = blake3::Hasher::new();
        hasher.update(node_id.as_bytes());
        hasher.update(&timestamp.to_le_bytes());
        hasher.update(device_id.as_bytes());
        let message = hasher.finalize();

        let signature = signing_key.sign(message.as_bytes()).to_bytes().to_vec();

        GenesisWitness::new(
            device_id.to_string(),
            public_key,
            PhysicalChannelType::HardwareKey,
            timestamp,
            signature,
        )
    }

    #[test]
    fn test_witness_trust_level() {
        let witness = GenesisWitness::new(
            "device-1".to_string(),
            vec![0u8; 32],
            PhysicalChannelType::HardwareKey,
            current_timestamp(),
            vec![0u8; 64],
        );
        assert_eq!(
            witness.trust_level(),
            crate::genesis::types::TrustLevel::Maximum
        );
    }

    #[test]
    fn test_witness_nfc_trust_level() {
        let witness = GenesisWitness::new(
            "nfc-1".to_string(),
            vec![0u8; 32],
            PhysicalChannelType::Nfc,
            current_timestamp(),
            vec![0u8; 64],
        );
        assert_eq!(
            witness.trust_level(),
            crate::genesis::types::TrustLevel::High
        );
    }

    #[test]
    fn test_witness_bluetooth_trust_level() {
        let witness = GenesisWitness::new(
            "bt-1".to_string(),
            vec![0u8; 32],
            PhysicalChannelType::Bluetooth,
            current_timestamp(),
            vec![0u8; 64],
        );
        assert_eq!(
            witness.trust_level(),
            crate::genesis::types::TrustLevel::Medium
        );
    }

    #[test]
    fn test_witness_verification_error_from_beardog_error() {
        use beardog_errors::BearDogError;

        let err = WitnessVerificationError::InvalidSignature;
        let beardog_err: BearDogError = err.into();
        let msg = format!("{beardog_err}");
        assert!(msg.contains("Witness verification failed"));
    }

    #[test]
    fn test_witness_verification_error_display() {
        let err = WitnessVerificationError::SignatureExpired {
            age_secs: 100_000,
            max_secs: 86400,
        };
        let msg = format!("{err}");
        assert!(msg.contains("expired"));
        assert!(msg.contains("100000"));

        let err2 = WitnessVerificationError::InvalidPublicKey("wrong format".to_string());
        assert!(format!("{err2}").contains("wrong format"));

        let err3 = WitnessVerificationError::UnauthorizedWitness {
            device_id: "bad-device".to_string(),
        };
        assert!(format!("{err3}").contains("bad-device"));

        let err4 = WitnessVerificationError::CryptoError("crypto fail".to_string());
        assert!(format!("{err4}").contains("crypto fail"));
    }

    #[test]
    fn test_verifier_default_is_permissive() {
        let verifier = GenesisWitnessVerifier::default();
        let witness = create_signed_witness("any-dev", "node-1");
        assert!(verifier.verify(&witness, "node-1").is_ok());
    }

    #[test]
    fn test_verify_empty_node_id() {
        let verifier = GenesisWitnessVerifier::permissive();
        let witness = GenesisWitness::new(
            "dev-1".to_string(),
            vec![0u8; 32],
            PhysicalChannelType::HardwareKey,
            current_timestamp(),
            vec![0u8; 64],
        );
        // Empty node ID should fail signature validation
        assert!(verifier.verify(&witness, "").is_err());
    }

    #[test]
    fn test_verify_empty_device_id() {
        let verifier = GenesisWitnessVerifier::permissive();
        let witness = GenesisWitness::new(
            String::new(), // empty device ID
            vec![0u8; 32],
            PhysicalChannelType::HardwareKey,
            current_timestamp(),
            vec![0u8; 64],
        );
        assert!(verifier.verify(&witness, "node-1").is_err());
    }

    #[test]
    fn test_witness_serde_roundtrip() {
        let witness = GenesisWitness::new(
            "serde-device".to_string(),
            vec![1u8; 32],
            PhysicalChannelType::QrCodeWithOob,
            current_timestamp(),
            vec![2u8; 64],
        );
        let json = serde_json::to_string(&witness).unwrap();
        let deserialized: GenesisWitness = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.device_id, "serde-device");
        assert_eq!(deserialized.public_key.len(), 32);
        assert_eq!(deserialized.signature.len(), 64);
    }
}

// ========================================================================
// memory_key_manager/mod: ~92% → cover remaining paths
// ========================================================================

mod key_manager_tests {
    use crate::memory_key_manager::{KeyMetadata, MemoryKeyConfig, MemoryKeyManager};

    #[test]
    fn test_key_manager_generate_and_exists() {
        let config = MemoryKeyConfig::default();
        let manager = MemoryKeyManager::new(config).unwrap();

        let key_id = manager.generate_key().unwrap();
        assert!(manager.key_exists(&key_id).unwrap());
        assert!(!manager.key_exists("nonexistent").unwrap());
    }

    #[test]
    fn test_key_manager_store_get_delete() {
        let config = MemoryKeyConfig::default();
        let mut manager = MemoryKeyManager::new(config).unwrap();

        let metadata = KeyMetadata {
            id: "test-key".to_string(),
            created_at: chrono::Utc::now(),
            key_type: "AES-256".to_string(),
        };

        let payload = vec![1u8, 2, 3, 4];
        let key_id = manager.store_key(&payload, metadata).unwrap();

        let retrieved = manager.get_key(&key_id).unwrap();
        assert_eq!(retrieved, payload);

        manager.delete_key(&key_id).unwrap();
        assert!(manager.get_key(&key_id).is_err());
    }

    #[test]
    fn test_key_manager_get_nonexistent() {
        let config = MemoryKeyConfig::default();
        let manager = MemoryKeyManager::new(config).unwrap();
        assert!(manager.get_key("does-not-exist").is_err());
    }

    #[test]
    fn test_key_manager_list_keys() {
        let config = MemoryKeyConfig::default();
        let manager = MemoryKeyManager::new(config).unwrap();
        let keys = manager.list_keys().unwrap();
        assert!(keys.is_empty());
    }

    #[test]
    fn test_key_metadata_construction() {
        let meta = KeyMetadata {
            id: "key-001".to_string(),
            created_at: chrono::Utc::now(),
            key_type: "RSA-2048".to_string(),
        };
        assert_eq!(meta.id, "key-001");
        assert_eq!(meta.key_type, "RSA-2048");
    }
}
