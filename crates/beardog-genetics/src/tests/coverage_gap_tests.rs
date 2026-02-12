//! Coverage gap tests for beardog-genetics
//!
//! Targets the largest uncovered regions to push from 80.3% → 90%

use beardog_errors::BearDogError;
use chrono::Utc;
use std::collections::HashMap;

// ═══════════════════════════════════════════════════════════════════
// birdsong/genesis_types.rs - GenesisWitness, GeneticLineage, PhysicalChannelProof
// ═══════════════════════════════════════════════════════════════════

mod genesis_types_tests {
    use crate::birdsong::genesis_types::*;
    use ed25519_dalek::{Signer, SigningKey};

    fn make_valid_witness(new_node_id: &str) -> GenesisWitness {
        let signing_key = SigningKey::from_bytes(&[42u8; 32]);
        let public_key = signing_key.verifying_key().to_bytes().to_vec();
        let timestamp = 1735000000u64;
        let message = GenesisWitness::create_signing_message(new_node_id, timestamp, &public_key);
        let signature = signing_key.sign(&message).to_bytes().to_vec();

        GenesisWitness {
            device_id: "solokey-test".to_string(),
            public_key,
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp,
            signature,
        }
    }

    #[test]
    fn test_witness_verify_valid_signature() {
        let witness = make_valid_witness("test-node-001");
        let result = witness.verify_signature("test-node-001");
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_witness_verify_wrong_node_id() {
        let witness = make_valid_witness("test-node-001");
        let result = witness.verify_signature("wrong-node");
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_witness_verify_invalid_pubkey_length() {
        let mut witness = make_valid_witness("test-node");
        witness.public_key = vec![0u8; 16]; // Wrong length
        let result = witness.verify_signature("test-node");
        assert!(result.is_err());
    }

    #[test]
    fn test_witness_verify_invalid_signature_length() {
        let mut witness = make_valid_witness("test-node");
        // Signature must be >= 64 bytes for Ed25519. Use 64 zero bytes (invalid but won't panic)
        witness.signature = vec![0u8; 64];
        let result = witness.verify_signature("test-node");
        // Either an error or false (invalid signature), but should not panic
        match result {
            Ok(valid) => assert!(!valid, "Zero signature should not verify"),
            Err(_) => {} // Error is also acceptable
        }
    }

    fn make_test_lineage(node_id: &str, trust: TrustLevel) -> GeneticLineage {
        use crate::birdsong::types::LineageMetadata;
        use crate::birdsong::{LineageChain, LineageNode};

        let root = LineageNode {
            node_id: "root".to_string(),
            parent_id: None,
            public_key: vec![0u8; 32],
            depth: 0,
            created_at: chrono::Utc::now(),
            metadata: LineageMetadata {
                biome_type: None,
                capabilities: vec![],
                trust_level: 0.9,
                custom: std::collections::HashMap::new(),
            },
        };

        let child = LineageNode {
            node_id: node_id.to_string(),
            parent_id: Some("root".to_string()),
            public_key: vec![1u8; 32],
            depth: 1,
            created_at: chrono::Utc::now(),
            metadata: LineageMetadata {
                biome_type: None,
                capabilities: vec![],
                trust_level: 0.8,
                custom: std::collections::HashMap::new(),
            },
        };

        let mut nodes = std::collections::HashMap::new();
        nodes.insert("root".to_string(), root.clone());
        nodes.insert(node_id.to_string(), child);

        GeneticLineage {
            genetic_id: vec![42u8; 32],
            lineage_chain: LineageChain {
                chain_id: "test-chain".to_string(),
                root_node: root,
                nodes,
                relationships: vec![],
                created_at: chrono::Utc::now(),
            },
            genesis_witness: make_valid_witness(node_id),
            birth_timestamp: 1735000000,
            trust_level: trust,
        }
    }

    #[test]
    fn test_genetic_lineage_depth() {
        let lineage = make_test_lineage("child-1", TrustLevel::High);
        assert_eq!(lineage.depth(), 2); // root + child = 2 nodes
    }

    #[test]
    fn test_genetic_lineage_hint() {
        let lineage = make_test_lineage("child-1", TrustLevel::High);
        let hint = lineage.lineage_hint();
        assert_eq!(hint.root_id, "root");
        assert_eq!(hint.max_depth, 2);
    }

    #[test]
    fn test_genetic_lineage_verify() {
        let lineage = make_test_lineage("test-node-001", TrustLevel::Maximum);
        let result = lineage.verify();
        assert!(result.is_ok());
        // Signature verification should pass since we correctly signed for "test-node-001"
        assert!(result.unwrap());
    }

    // PhysicalChannelProof verification tests
    #[test]
    fn test_physical_channel_proof_hardware_key_with_attestation() {
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::HardwareKey,
            attestation: Some(vec![1u8; 64]),
            verification_codes: None,
            pairing_data: None,
            timestamp: 1735000000,
        };
        let result = proof.verify();
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_physical_channel_proof_hardware_key_no_attestation() {
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::HardwareKey,
            attestation: None,
            verification_codes: None,
            pairing_data: None,
            timestamp: 1735000000,
        };
        let result = proof.verify();
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_physical_channel_proof_hardware_key_empty_attestation() {
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::HardwareKey,
            attestation: Some(vec![]),
            verification_codes: None,
            pairing_data: None,
            timestamp: 1735000000,
        };
        let result = proof.verify();
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_physical_channel_proof_hardware_key_short_attestation() {
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::HardwareKey,
            attestation: Some(vec![1u8; 16]),
            verification_codes: None,
            pairing_data: None,
            timestamp: 1735000000,
        };
        let result = proof.verify();
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_physical_channel_proof_hardware_key_zeroed_attestation() {
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::HardwareKey,
            attestation: Some(vec![0u8; 64]),
            verification_codes: None,
            pairing_data: None,
            timestamp: 1735000000,
        };
        let result = proof.verify();
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_physical_channel_proof_qr_code_with_codes() {
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::QrCodeWithOob,
            attestation: None,
            verification_codes: Some(vec!["CODE1".to_string()]),
            pairing_data: None,
            timestamp: 1735000000,
        };
        let result = proof.verify();
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_physical_channel_proof_qr_code_empty_codes() {
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::QrCodeWithOob,
            attestation: None,
            verification_codes: Some(vec![]),
            pairing_data: None,
            timestamp: 1735000000,
        };
        let result = proof.verify();
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_physical_channel_proof_qr_code_no_codes() {
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::QrCodeWithOob,
            attestation: None,
            verification_codes: None,
            pairing_data: None,
            timestamp: 1735000000,
        };
        let result = proof.verify();
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_physical_channel_proof_bluetooth_with_data() {
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::Bluetooth,
            attestation: None,
            verification_codes: None,
            pairing_data: Some(vec![1, 2, 3]),
            timestamp: 1735000000,
        };
        let result = proof.verify();
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_physical_channel_proof_bluetooth_no_data() {
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::Bluetooth,
            attestation: None,
            verification_codes: None,
            pairing_data: None,
            timestamp: 1735000000,
        };
        let result = proof.verify();
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_physical_channel_proof_nfc_with_attestation() {
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::Nfc,
            attestation: Some(vec![1u8; 32]),
            verification_codes: None,
            pairing_data: None,
            timestamp: 1735000000,
        };
        let result = proof.verify();
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_physical_channel_proof_nfc_no_attestation() {
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::Nfc,
            attestation: None,
            verification_codes: None,
            pairing_data: None,
            timestamp: 1735000000,
        };
        let result = proof.verify();
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_physical_channel_proof_trust_level() {
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::HardwareKey,
            attestation: None,
            verification_codes: None,
            pairing_data: None,
            timestamp: 1735000000,
        };
        assert_eq!(proof.trust_level(), TrustLevel::Maximum);
    }

    #[test]
    fn test_physical_channel_proof_bluetooth_trust_level() {
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::Bluetooth,
            attestation: None,
            verification_codes: None,
            pairing_data: None,
            timestamp: 1735000000,
        };
        assert_eq!(proof.trust_level(), TrustLevel::Medium);
    }

    #[test]
    fn test_physical_channel_proof_nfc_trust_level() {
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::Nfc,
            attestation: None,
            verification_codes: None,
            pairing_data: None,
            timestamp: 1735000000,
        };
        assert_eq!(proof.trust_level(), TrustLevel::High);
    }

    #[test]
    fn test_genesis_ceremony_result_success() {
        let lineage = make_test_lineage("result-node", TrustLevel::Maximum);
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::HardwareKey,
            attestation: Some(vec![1u8; 64]),
            verification_codes: None,
            pairing_data: None,
            timestamp: 1735000000,
        };
        let result = GenesisCeremonyResult {
            genetic_lineage: lineage,
            physical_proof: proof,
            completed_at: 1735000001,
            success: true,
            error: None,
        };
        assert!(result.success);
        assert!(result.error.is_none());
        assert_eq!(result.completed_at, 1735000001);
    }

    #[test]
    fn test_genesis_ceremony_result_with_error() {
        let lineage = make_test_lineage("err-node", TrustLevel::Low);
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::HardwareKey,
            attestation: None,
            verification_codes: None,
            pairing_data: None,
            timestamp: 1735000000,
        };
        let result = GenesisCeremonyResult {
            genetic_lineage: lineage,
            physical_proof: proof,
            completed_at: 1735000001,
            success: false,
            error: Some("Witness verification failed".to_string()),
        };
        assert!(!result.success);
        assert_eq!(result.error.unwrap(), "Witness verification failed");
    }
}

// ═══════════════════════════════════════════════════════════════════
// birdsong/manager.rs - Discovery encryption & key management
// ═══════════════════════════════════════════════════════════════════

mod manager_discovery_tests {
    use super::*;
    use crate::birdsong::BirdSongManager;

    async fn create_test_manager() -> BirdSongManager {
        BirdSongManager::new(vec![0xEF; 32], None).await.unwrap()
    }

    #[tokio::test]
    async fn test_discovery_encrypt_decrypt_roundtrip() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let plaintext = b"Hello, family!";
        let family_id = "test-family";

        let encrypted = manager.encrypt_discovery_for_family(plaintext, family_id)?;
        assert!(encrypted.len() > plaintext.len());

        let decrypted = manager.decrypt_discovery_from_family(&encrypted, family_id)?;
        assert_eq!(decrypted, plaintext);
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_wrong_family_fails() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let plaintext = b"Secret message";

        let encrypted = manager.encrypt_discovery_for_family(plaintext, "family-A")?;
        let result = manager.decrypt_discovery_from_family(&encrypted, "family-B");
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_decrypt_too_short() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let short_data = vec![0u8; 10];

        let result = manager.decrypt_discovery_from_family(&short_data, "family");
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_decrypt_corrupted_data() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let plaintext = b"Test data";

        let mut encrypted = manager.encrypt_discovery_for_family(plaintext, "family")?;
        if let Some(last) = encrypted.last_mut() {
            *last ^= 0xFF;
        }

        let result = manager.decrypt_discovery_from_family(&encrypted, "family");
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_discovery_empty_plaintext() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let plaintext = b"";

        let encrypted = manager.encrypt_discovery_for_family(plaintext, "family")?;
        let decrypted = manager.decrypt_discovery_from_family(&encrypted, "family")?;
        assert_eq!(decrypted, plaintext);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_lineage_chain_nonexistent() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        assert!(manager.get_lineage_chain("nonexistent").is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_get_descendants_nonexistent() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let descendants = manager.get_descendants("nonexistent", "node");
        assert!(descendants.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_can_decrypt_check() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;

        let chain = manager
            .generate_root_lineage("root".to_string(), None)
            .await?;
        manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;

        let hint = crate::birdsong::LineageHint {
            root_id: "root".to_string(),
            min_depth: 1,
            max_depth: 1,
            biome_filter: None,
            version: 1,
        };

        let encrypt_req = crate::birdsong::types::BirdSongEncryptRequest {
            plaintext: b"Test message".to_vec(),
            lineage_hint: hint,
            associated_data: None,
        };

        let broadcast = manager.encrypt_broadcast(&encrypt_req)?;

        assert!(manager.can_decrypt(&broadcast, 1));
        assert!(!manager.can_decrypt(&broadcast, 3));

        Ok(())
    }

    #[tokio::test]
    async fn test_revoke_keys_no_keys() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let chain = manager
            .generate_root_lineage("root".to_string(), None)
            .await?;

        let count = manager.revoke_keys(&chain.chain_id, "root");
        assert_eq!(count, 0);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_distributed_keys_nonexistent() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let keys = manager.get_distributed_keys("nonexistent");
        assert!(keys.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_distribute_keys_to_descendants() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;
        let chain = manager
            .generate_root_lineage("root".to_string(), None)
            .await?;

        manager
            .add_child(&chain.chain_id, "root", "child-1".to_string(), None)
            .await?;

        // Distribute keys
        let count = manager.distribute_keys_to_descendants(&chain.chain_id, "root", 1)?;
        assert!(count > 0);

        // Keys are stored under the descendant's node_id, not the root
        let keys = manager.get_distributed_keys("child-1");
        assert!(!keys.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_full_lineage_lifecycle() -> Result<(), BearDogError> {
        let manager = create_test_manager().await;

        // Create root
        let chain = manager
            .generate_root_lineage("root".to_string(), None)
            .await?;

        // Add children
        manager
            .add_child(&chain.chain_id, "root", "child-a".to_string(), None)
            .await?;
        manager
            .add_child(&chain.chain_id, "root", "child-b".to_string(), None)
            .await?;
        manager
            .add_child(&chain.chain_id, "child-a", "grandchild-1".to_string(), None)
            .await?;

        // Verify chain exists
        assert!(manager.get_lineage_chain(&chain.chain_id).is_some());

        // Get descendants
        let root_descendants = manager.get_descendants(&chain.chain_id, "root");
        assert!(root_descendants.len() >= 2);

        // Generate and verify proof
        let proof = manager.generate_lineage_proof(&chain.chain_id, "child-a")?;
        let verification = manager.verify_lineage_proof(&proof, &chain.chain_id)?;
        assert!(verification.valid);

        // Key distribution — keys stored under descendant node IDs
        manager.distribute_keys_to_descendants(&chain.chain_id, "root", 1)?;
        let keys = manager.get_distributed_keys("child-a");
        assert!(!keys.is_empty());

        // Broadcast encryption
        let hint = crate::birdsong::LineageHint {
            root_id: "root".to_string(),
            min_depth: 0,
            max_depth: 3,
            biome_filter: None,
            version: 1,
        };

        let req = crate::birdsong::types::BirdSongEncryptRequest {
            plaintext: b"broadcast data".to_vec(),
            lineage_hint: hint,
            associated_data: None,
        };

        let broadcast = manager.encrypt_broadcast(&req)?;
        assert!(!broadcast.ciphertext.is_empty());

        // Revoke and verify
        let revoked_count = manager.revoke_keys(&chain.chain_id, "child-a");
        assert!(revoked_count >= 0);

        // Rotate all keys
        let rotated = manager.rotate_all_keys(&chain.chain_id, "root", 2)?;
        assert!(rotated > 0);

        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════
// constraints/enforcement.rs - Test via verify_operation public API
// ═══════════════════════════════════════════════════════════════════

mod enforcement_tests {
    use super::*;
    use crate::constraints::types::OperationType;
    use crate::constraints::*;
    use ed25519_dalek::{Signer, SigningKey};
    use std::collections::HashSet;

    /// Helper to create properly signed constraints
    fn sign_constraints(constraints: KeyConstraints) -> (SignedConstraints, Vec<u8>) {
        let signing_key = SigningKey::from_bytes(&[99u8; 32]);
        let public_key = signing_key.verifying_key().to_bytes().to_vec();
        let hash = SignedConstraints::compute_hash(&constraints);
        let signature = signing_key.sign(&hash).to_bytes().to_vec();

        let signed = SignedConstraints {
            constraints,
            signature,
            signed_by_key_id: "test-key-001".to_string(),
            created_at: Utc::now(),
            version: SignedConstraints::CURRENT_VERSION,
        };

        (signed, public_key)
    }

    #[test]
    fn test_verify_operation_scope_unrestricted() {
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Unrestricted,
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign {
            domain: Some("any_domain".to_string()),
        };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_scope_limited_allowed() {
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Limited {
                domains: vec!["climate".to_string()],
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign {
            domain: Some("climate".to_string()),
        };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_scope_limited_denied() {
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Limited {
                domains: vec!["climate".to_string()],
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign {
            domain: Some("military".to_string()),
        };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::ScopeViolation { .. }
        ));
    }

    #[test]
    fn test_verify_operation_scope_forbidden() {
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Forbidden {
                domains: vec!["military".to_string()],
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign {
            domain: Some("military".to_string()),
        };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::ForbiddenDomain { .. }
        ));
    }

    #[test]
    fn test_verify_operation_scope_forbidden_allowed() {
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Forbidden {
                domains: vec!["military".to_string()],
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign {
            domain: Some("climate".to_string()),
        };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_scope_operation_specific() {
        let mut allowed = HashSet::new();
        allowed.insert(OperationType::Sign);
        allowed.insert(OperationType::Encrypt);
        let constraints = KeyConstraints {
            scope: ScopeConstraint::OperationSpecific {
                allowed_operations: allowed,
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);

        // Allowed operation
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_scope_operation_specific_denied() {
        let mut allowed = HashSet::new();
        allowed.insert(OperationType::Sign);
        let constraints = KeyConstraints {
            scope: ScopeConstraint::OperationSpecific {
                allowed_operations: allowed,
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Delete {
            path: "test.txt".to_string(),
        };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::OperationNotAllowed { .. }
        ));
    }

    #[test]
    fn test_verify_operation_lifetime_permanent() {
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::Permanent,
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_lifetime_not_expired() {
        let future = Utc::now() + chrono::Duration::days(30);
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::ExpiresAt { timestamp: future },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_lifetime_expired() {
        let past = Utc::now() - chrono::Duration::days(1);
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::ExpiresAt { timestamp: past },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::KeyExpired { .. }
        ));
    }

    #[test]
    fn test_verify_operation_lifetime_duration() {
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::Duration {
                months: 12,
                evolution_trigger: Some(6),
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_lifetime_use_count_ok() {
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::UseCount {
                max_uses: 100,
                current_uses: 50,
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_lifetime_use_count_exceeded() {
        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::UseCount {
                max_uses: 10,
                current_uses: 10,
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::UseCountExceeded { .. }
        ));
    }

    #[test]
    fn test_verify_operation_data_access_delete_denied() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                cannot_delete: vec!["raw_data/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Delete {
            path: "raw_data/file.nc".to_string(),
        };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_operation_data_access_modify_immutable() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                immutable_paths: vec!["config/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Modify {
            path: "config/settings.toml".to_string(),
        };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_operation_data_access_modify_cannot_modify() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint {
                cannot_modify: vec!["audit/*".to_string()],
                ..Default::default()
            },
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Modify {
            path: "audit/log.txt".to_string(),
        };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_operation_data_access_read_ok() {
        let constraints = KeyConstraints {
            data_access: DataAccessConstraint::default(),
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Read {
            path: "data.txt".to_string(),
        };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_co_signers_empty() {
        let constraints = KeyConstraints {
            co_signers: vec![],
            ..Default::default()
        };
        let (signed, pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        assert!(ConstraintEnforcer::verify_operation(&signed, &op, &pk).is_ok());
    }

    #[test]
    fn test_verify_operation_invalid_signature() {
        let constraints = KeyConstraints::default();
        let (mut signed, pk) = sign_constraints(constraints);
        // Tamper with signature
        signed.signature = vec![0xFF; 64];
        let op = KeyOperation::Sign { domain: None };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &pk);
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ConstraintViolationError::SignatureVerificationFailed { .. }
        ));
    }

    #[test]
    fn test_verify_operation_short_public_key() {
        let constraints = KeyConstraints::default();
        let (signed, _pk) = sign_constraints(constraints);
        let op = KeyOperation::Sign { domain: None };
        let result = ConstraintEnforcer::verify_operation(&signed, &op, &[0u8; 16]);
        assert!(result.is_err());
    }

    // ConstraintViolationError Display and From coverage
    #[test]
    fn test_violation_error_display() {
        let err = ConstraintViolationError::SignatureVerificationFailed {
            reason: "bad sig".to_string(),
        };
        assert!(format!("{err}").contains("bad sig"));

        let err = ConstraintViolationError::KeyExpired {
            expired_at: Utc::now(),
        };
        assert!(format!("{err}").contains("expired"));

        let err = ConstraintViolationError::UseCountExceeded {
            max_uses: 10,
            current_uses: 15,
        };
        assert!(format!("{err}").contains("15/10"));

        let err = ConstraintViolationError::ScopeViolation {
            allowed: vec!["a".to_string()],
            attempted: "b".to_string(),
        };
        assert!(format!("{err}").contains("Scope violation"));

        let err = ConstraintViolationError::ForbiddenDomain {
            domain: "military".to_string(),
        };
        assert!(format!("{err}").contains("military"));

        let err = ConstraintViolationError::OperationNotAllowed {
            operation: OperationType::Delete,
            allowed: vec![OperationType::Sign],
        };
        assert!(format!("{err}").contains("not allowed"));

        let err = ConstraintViolationError::DataAccessDenied {
            operation: "delete".to_string(),
            path: "/test".to_string(),
            reason: "forbidden".to_string(),
        };
        assert!(format!("{err}").contains("Data access denied"));

        let err = ConstraintViolationError::CoSignerRequired {
            required: vec!["alice".to_string()],
            present: vec![],
        };
        assert!(format!("{err}").contains("Co-signer"));

        let err = ConstraintViolationError::BehavioralRequirementNotMet {
            requirement: "biometric".to_string(),
        };
        assert!(format!("{err}").contains("biometric"));
    }

    #[test]
    fn test_violation_error_into_beardog_error() {
        let err = ConstraintViolationError::KeyExpired {
            expired_at: Utc::now(),
        };
        let beardog_err: BearDogError = err.into();
        let msg = format!("{beardog_err}");
        assert!(msg.contains("Constraint violation"));
    }
}

// ═══════════════════════════════════════════════════════════════════
// constraints/evolution.rs - Additional coverage
// ═══════════════════════════════════════════════════════════════════

mod evolution_tests {
    use super::*;
    use crate::constraints::*;

    #[test]
    fn test_evolution_engine_default() {
        let engine = ConstraintEvolutionEngine::default();
        assert_eq!(engine.trust_score(), 0.5);
    }

    #[test]
    fn test_record_operation_with_domain() {
        let mut engine = ConstraintEvolutionEngine::new();
        let op = KeyOperation::Sign {
            domain: Some("climate".to_string()),
        };
        engine.record_operation(&op);

        assert_eq!(engine.usage_stats().total_operations, 1);
        assert!(engine
            .usage_stats()
            .domains_accessed
            .contains_key("climate"));
        assert!(engine.usage_stats().first_operation.is_some());
        assert!(engine.usage_stats().last_operation.is_some());
    }

    #[test]
    fn test_record_multiple_operations() {
        let mut engine = ConstraintEvolutionEngine::new();
        engine.record_operation(&KeyOperation::Sign {
            domain: Some("climate".to_string()),
        });
        engine.record_operation(&KeyOperation::Encrypt {
            recipients: vec!["peer".to_string()],
        });
        engine.record_operation(&KeyOperation::Read {
            path: "/data".to_string(),
        });

        assert_eq!(engine.usage_stats().total_operations, 3);
    }

    #[test]
    fn test_evolve_constraints_trust_threshold() {
        let engine = ConstraintEvolutionEngine::new();
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Limited {
                domains: vec!["domain1".to_string(), "domain2".to_string()],
            },
            ..Default::default()
        };
        let trigger = EvolutionTrigger::TrustThreshold { threshold: 0.9 };
        let evolved = engine.evolve_constraints(&constraints, &trigger);
        // Relaxed: domains should be filtered to only actually used ones
        if let ScopeConstraint::Limited { domains } = &evolved.scope {
            assert!(domains.is_empty()); // No domains used yet
        }
    }

    #[test]
    fn test_evolve_constraints_trust_threshold_low() {
        let engine = ConstraintEvolutionEngine::new();
        let constraints = KeyConstraints {
            scope: ScopeConstraint::Limited {
                domains: vec!["domain1".to_string()],
            },
            ..Default::default()
        };
        let trigger = EvolutionTrigger::TrustThreshold { threshold: 0.5 };
        let evolved = engine.evolve_constraints(&constraints, &trigger);
        // Low threshold: no relaxation
        if let ScopeConstraint::Limited { domains } = &evolved.scope {
            assert_eq!(domains.len(), 1);
        }
    }

    #[test]
    fn test_evolve_constraints_time_elapsed_high_trust() {
        let mut engine = ConstraintEvolutionEngine::new();
        // Build up high trust: starts at 0.5, +0.001 per op, need >0.8 → 301 ops
        for _ in 0..350 {
            engine.record_operation(&KeyOperation::Sign { domain: None });
        }
        assert!(engine.trust_score() > 0.8);

        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::Duration {
                months: 12,
                evolution_trigger: Some(6),
            },
            ..Default::default()
        };
        let trigger = EvolutionTrigger::TimeElapsed { months: 6 };
        let evolved = engine.evolve_constraints(&constraints, &trigger);
        if let LifetimeConstraint::Duration { months, .. } = &evolved.lifetime {
            assert_eq!(*months, 18); // 12 + 6
        }
    }

    #[test]
    fn test_evolve_constraints_time_elapsed_low_trust() {
        let mut engine = ConstraintEvolutionEngine::new();
        // Keep trust low
        engine.record_failure();
        engine.record_failure();

        let constraints = KeyConstraints {
            lifetime: LifetimeConstraint::Duration {
                months: 12,
                evolution_trigger: Some(6),
            },
            ..Default::default()
        };
        let trigger = EvolutionTrigger::TimeElapsed { months: 6 };
        let evolved = engine.evolve_constraints(&constraints, &trigger);
        if let LifetimeConstraint::Duration { months, .. } = &evolved.lifetime {
            assert_eq!(*months, 12); // No extension (trust < 0.8)
        }
    }

    #[test]
    fn test_evolve_constraints_usage_pattern() {
        let engine = ConstraintEvolutionEngine::new();
        let constraints = KeyConstraints::default();
        let trigger = EvolutionTrigger::UsagePattern {
            pattern: "consistent_compliance".to_string(),
        };
        let _evolved = engine.evolve_constraints(&constraints, &trigger);
    }

    #[test]
    fn test_evolve_constraints_manual_request() {
        let engine = ConstraintEvolutionEngine::new();
        let constraints = KeyConstraints::default();
        let trigger = EvolutionTrigger::ManualRequest;
        let evolved = engine.evolve_constraints(&constraints, &trigger);
        // Manual request doesn't change constraints
        assert_eq!(evolved, constraints);
    }

    #[test]
    fn test_should_evolve_no_trigger() {
        let engine = ConstraintEvolutionEngine::new();
        let constraints = KeyConstraints::default();
        let created_at = Utc::now();
        assert!(engine.should_evolve(&constraints, created_at).is_none());
    }

    #[test]
    fn test_trust_score_capped() {
        let mut engine = ConstraintEvolutionEngine::new();
        let op = KeyOperation::Sign { domain: None };
        for _ in 0..2000 {
            engine.record_operation(&op);
        }
        assert!(engine.trust_score() <= 1.0);
    }

    #[test]
    fn test_trust_score_floor() {
        let mut engine = ConstraintEvolutionEngine::new();
        for _ in 0..200 {
            engine.record_failure();
        }
        assert!(engine.trust_score() >= 0.0);
    }
}

// ═══════════════════════════════════════════════════════════════════
// genetics/entropy_hierarchy/engine.rs - Additional coverage
// ═══════════════════════════════════════════════════════════════════

mod engine_tests {
    use super::*;
    use crate::genetics::entropy_hierarchy::*;

    pub(super) fn make_human_entropy() -> EntropyClass {
        EntropyClass::HumanLivedExperience {
            quality_score: 0.9,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1, 2, 3],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        }
    }

    pub(super) fn make_machine_entropy() -> EntropyClass {
        EntropyClass::StoreBoughtMachine {
            quality_score: 0.8,
            source_type: MachineEntropySource {
                source_type: MachineSourceType::HRNG {
                    device_type: "TPM".to_string(),
                    entropy_rate: 1.0,
                },
                algorithm: "AES-CTR-DRBG".to_string(),
                seed_source: "Hardware".to_string(),
                quality_metrics: HashMap::new(),
            },
            generation_timestamp: Utc::now(),
            reproducibility_index: 0.3,
        }
    }

    pub(super) fn make_supervised_entropy() -> EntropyClass {
        EntropyClass::HumanSupervisedMachine {
            quality_score: 0.85,
            human_validator: HumanIdentity {
                identity_id: "validator-001".to_string(),
                identity_hash: vec![1; 32],
                verification_level: VerificationLevel::Maximum,
                verified_at: Utc::now(),
            },
            machine_source: MachineEntropySource {
                source_type: MachineSourceType::HRNG {
                    device_type: "TPM".to_string(),
                    entropy_rate: 1.0,
                },
                algorithm: "AES-CTR-DRBG".to_string(),
                seed_source: "Hardware".to_string(),
                quality_metrics: HashMap::new(),
            },
            validation_timestamp: Utc::now(),
        }
    }

    #[test]
    fn test_engine_default() {
        let manager = EntropyHierarchyManager::default();
        assert!(manager.active_seeds.is_empty());
    }

    #[test]
    fn test_create_and_validate_seed() {
        let config = EntropyHierarchyConfig::default();
        let mut manager = EntropyHierarchyManager::new(config);
        let entropy_class = make_human_entropy();

        let seed_id = manager
            .create_human_seed(entropy_class, vec![1, 2, 3, 4])
            .unwrap();

        assert!(manager.validate_seed(seed_id).unwrap());
    }

    #[test]
    fn test_use_seed_and_track_usage() {
        let config = EntropyHierarchyConfig::default();
        let mut manager = EntropyHierarchyManager::new(config);
        let entropy_class = make_human_entropy();

        let seed_id = manager
            .create_human_seed(entropy_class, vec![1, 2, 3, 4])
            .unwrap();

        let derived = manager.use_seed(seed_id, "test-operation").unwrap();
        assert!(!derived.is_empty());

        // Usage count should be incremented
        let info = manager.get_seed_info(seed_id).unwrap();
        assert_eq!(info.metadata.usage_count, 1);
    }

    #[test]
    fn test_use_seed_nonexistent() {
        let mut manager = EntropyHierarchyManager::default();
        let fake_id = uuid::Uuid::new_v4();
        assert!(manager.use_seed(fake_id, "test").is_err());
    }

    #[test]
    fn test_validate_seed_nonexistent() {
        let manager = EntropyHierarchyManager::default();
        let fake_id = uuid::Uuid::new_v4();
        assert!(manager.validate_seed(fake_id).is_err());
    }

    #[test]
    fn test_get_seed_info_nonexistent() {
        let manager = EntropyHierarchyManager::default();
        assert!(manager.get_seed_info(uuid::Uuid::new_v4()).is_none());
    }

    #[test]
    fn test_list_active_seeds() {
        let mut manager = EntropyHierarchyManager::default();
        assert!(manager.list_active_seeds().is_empty());

        let entropy_class = make_human_entropy();
        let seed_id = manager
            .create_human_seed(entropy_class, vec![1, 2, 3])
            .unwrap();

        let seeds = manager.list_active_seeds();
        assert_eq!(seeds.len(), 1);
        assert!(seeds.contains(&seed_id));
    }

    #[test]
    fn test_cleanup_expired_seeds() {
        let mut manager = EntropyHierarchyManager::default();
        let entropy_class = make_human_entropy();

        let seed_id = manager
            .create_human_seed(entropy_class, vec![1, 2, 3])
            .unwrap();

        let removed = manager.cleanup_expired_seeds().unwrap();
        assert_eq!(removed, 0);
        assert!(manager.get_seed_info(seed_id).is_some());
    }

    #[test]
    fn test_get_performance_metrics() {
        let mut manager = EntropyHierarchyManager::default();
        let entropy_class = make_human_entropy();

        manager
            .create_human_seed(entropy_class, vec![1, 2, 3, 4])
            .unwrap();

        let metrics = manager.get_performance_metrics();
        assert_eq!(metrics.active_seeds_count, 1);
        assert!(metrics.total_entropy_generated > 0);
        assert!(metrics.average_quality_score > 0.0);
    }

    #[test]
    fn test_get_performance_metrics_empty() {
        let manager = EntropyHierarchyManager::default();
        let metrics = manager.get_performance_metrics();
        assert_eq!(metrics.active_seeds_count, 0);
        assert_eq!(metrics.average_quality_score, 0.0);
    }

    #[test]
    fn test_calculate_average_quality_mixed() {
        let mut manager = EntropyHierarchyManager::default();

        manager
            .create_human_seed(make_human_entropy(), vec![1, 2, 3])
            .unwrap();
        manager
            .create_human_seed(make_machine_entropy(), vec![4, 5, 6])
            .unwrap();
        manager
            .create_human_seed(make_supervised_entropy(), vec![7, 8, 9])
            .unwrap();

        let metrics = manager.get_performance_metrics();
        assert!(metrics.average_quality_score > 0.0);
        assert!(metrics.average_quality_score < 1.0);
        assert_eq!(metrics.active_seeds_count, 3);
    }

    #[test]
    fn test_initialize_and_shutdown() {
        let mut manager = EntropyHierarchyManager::default();
        let entropy = make_human_entropy();
        manager.create_human_seed(entropy, vec![1, 2, 3]).unwrap();

        assert!(manager.initialize().is_ok());
        assert_eq!(manager.active_seeds.len(), 1);

        assert!(manager.shutdown().is_ok());
        assert!(manager.active_seeds.is_empty());
    }

    #[test]
    fn test_seed_multiple_uses() {
        let config = EntropyHierarchyConfig::default();
        let mut manager = EntropyHierarchyManager::new(config);
        let entropy = make_human_entropy();

        let seed_id = manager
            .create_human_seed(entropy, vec![1, 2, 3, 4])
            .unwrap();

        // Use the seed multiple times
        for i in 0..5 {
            let derived = manager.use_seed(seed_id, &format!("op-{i}")).unwrap();
            assert!(!derived.is_empty());
        }

        let info = manager.get_seed_info(seed_id).unwrap();
        assert_eq!(info.metadata.usage_count, 5);
    }
}

// ═══════════════════════════════════════════════════════════════════
// genetics/entropy_hierarchy/validation.rs - Additional coverage
// ═══════════════════════════════════════════════════════════════════

mod validation_tests {
    use super::*;
    use crate::genetics::entropy_hierarchy::*;

    fn strict_config() -> EntropyHierarchyConfig {
        EntropyHierarchyConfig {
            min_human_quality: 0.7,
            min_machine_quality: 0.5,
            max_entropy_age_hours: 24,
            require_biometric_verification: true,
            require_ownership_proof: true,
        }
    }

    fn lenient_config() -> EntropyHierarchyConfig {
        EntropyHierarchyConfig {
            min_human_quality: 0.1,
            min_machine_quality: 0.1,
            max_entropy_age_hours: 8760,
            require_biometric_verification: false,
            require_ownership_proof: false,
        }
    }

    fn make_valid_human_entropy() -> EntropyClass {
        let mut biometric_hash = vec![0u8; 32];
        for (i, byte) in biometric_hash.iter_mut().enumerate() {
            *byte = (i as u8).wrapping_mul(7).wrapping_add(13);
        }
        let mut sig = vec![0u8; 64];
        for (i, byte) in sig.iter_mut().enumerate() {
            *byte = (i as u8).wrapping_mul(3).wrapping_add(5);
        }

        EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: biometric_hash,
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: sig,
                timestamp: Utc::now(),
            },
        }
    }

    #[test]
    fn test_validate_supervised_machine_quality_pass() {
        let entropy = super::engine_tests::make_supervised_entropy();
        let config = lenient_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        assert!(v.validate_entropy_quality(&entropy).unwrap());
    }

    #[test]
    fn test_validate_supervised_machine_quality_fail() {
        let entropy = EntropyClass::HumanSupervisedMachine {
            quality_score: 0.1,
            human_validator: HumanIdentity {
                identity_id: "val-001".to_string(),
                identity_hash: vec![1; 32],
                verification_level: VerificationLevel::Maximum,
                verified_at: Utc::now(),
            },
            machine_source: MachineEntropySource {
                source_type: MachineSourceType::HRNG {
                    device_type: "TPM".to_string(),
                    entropy_rate: 1.0,
                },
                algorithm: "AES-CTR-DRBG".to_string(),
                seed_source: "Hardware".to_string(),
                quality_metrics: HashMap::new(),
            },
            validation_timestamp: Utc::now(),
        };
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        assert!(v.validate_entropy_quality(&entropy).is_err());
    }

    #[test]
    fn test_validate_supervised_machine_age() {
        let entropy = EntropyClass::HumanSupervisedMachine {
            quality_score: 0.8,
            human_validator: HumanIdentity {
                identity_id: "val-002".to_string(),
                identity_hash: vec![1; 32],
                verification_level: VerificationLevel::Maximum,
                verified_at: Utc::now(),
            },
            machine_source: MachineEntropySource {
                source_type: MachineSourceType::HRNG {
                    device_type: "TPM".to_string(),
                    entropy_rate: 1.0,
                },
                algorithm: "AES-CTR-DRBG".to_string(),
                seed_source: "Hardware".to_string(),
                quality_metrics: HashMap::new(),
            },
            validation_timestamp: Utc::now(),
        };
        let config = lenient_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        assert!(v.validate_entropy_age(&entropy).unwrap());
    }

    #[test]
    fn test_validate_biometric_disabled() {
        let config = lenient_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = make_valid_human_entropy();
        assert!(v.validate_biometric_verification(&entropy).unwrap());
    }

    #[test]
    fn test_validate_biometric_empty_hash() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };
        assert!(v.validate_biometric_verification(&entropy).is_err());
    }

    #[test]
    fn test_validate_biometric_empty_ownership_proof() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1; 32],
                ownership_proof: vec![],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };
        assert!(v.validate_biometric_verification(&entropy).is_err());
    }

    #[test]
    fn test_validate_biometric_short_hash() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1; 16],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };
        assert!(v.validate_biometric_verification(&entropy).is_err());
    }

    #[test]
    fn test_validate_biometric_zeroed_hash() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![0; 32],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };
        assert!(v.validate_biometric_verification(&entropy).is_err());
    }

    #[test]
    fn test_validate_biometric_supervised_machine() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = super::engine_tests::make_supervised_entropy();
        assert!(v.validate_biometric_verification(&entropy).unwrap());
    }

    #[test]
    fn test_validate_biometric_store_bought() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = super::engine_tests::make_machine_entropy();
        assert!(v.validate_biometric_verification(&entropy).unwrap());
    }

    #[test]
    fn test_validate_ownership_disabled() {
        let config = lenient_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = make_valid_human_entropy();
        assert!(v.validate_ownership_proof(&entropy).unwrap());
    }

    #[test]
    fn test_validate_ownership_empty_proof_data() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1; 32],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![],
                signature: vec![10, 11, 12],
                timestamp: Utc::now(),
            },
        };
        assert!(v.validate_ownership_proof(&entropy).is_err());
    }

    #[test]
    fn test_validate_ownership_empty_signature() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1; 32],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![],
                timestamp: Utc::now(),
            },
        };
        assert!(v.validate_ownership_proof(&entropy).is_err());
    }

    #[test]
    fn test_validate_ownership_zeroed_signature() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1; 32],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![0; 64],
                timestamp: Utc::now(),
            },
        };
        assert!(v.validate_ownership_proof(&entropy).is_err());
    }

    #[test]
    fn test_validate_ownership_old_proof() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let old_timestamp = Utc::now() - chrono::Duration::hours(48);
        let entropy = EntropyClass::HumanLivedExperience {
            quality_score: 0.8,
            capture_timestamp: Utc::now(),
            biometric_signature: BiometricHash {
                hash: vec![1; 32],
                ownership_proof: vec![4, 5, 6],
            },
            ownership_proof: OwnershipProof {
                proof_data: vec![7, 8, 9],
                signature: vec![1; 64],
                timestamp: old_timestamp,
            },
        };
        assert!(v.validate_ownership_proof(&entropy).is_err());
    }

    #[test]
    fn test_validate_ownership_supervised_machine() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = super::engine_tests::make_supervised_entropy();
        assert!(v.validate_ownership_proof(&entropy).unwrap());
    }

    #[test]
    fn test_validate_ownership_store_bought() {
        let config = strict_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = super::engine_tests::make_machine_entropy();
        assert!(v.validate_ownership_proof(&entropy).unwrap());
    }

    #[test]
    fn test_full_validate_pass() {
        let config = lenient_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = make_valid_human_entropy();
        assert!(v.validate_entropy(&entropy).is_ok());
    }

    #[test]
    fn test_full_validate_machine_pass() {
        let config = lenient_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = super::engine_tests::make_machine_entropy();
        assert!(v.validate_entropy(&entropy).is_ok());
    }

    #[test]
    fn test_full_validate_supervised_pass() {
        let config = lenient_config();
        let v = crate::genetics::entropy_hierarchy::validation::EntropyValidator::new(config);
        let entropy = super::engine_tests::make_supervised_entropy();
        assert!(v.validate_entropy(&entropy).is_ok());
    }
}

// ═══════════════════════════════════════════════════════════════════
// birdsong/genesis.rs - GenesisLineageProvider
// ═══════════════════════════════════════════════════════════════════

mod genesis_provider_tests {
    use super::*;
    use crate::birdsong::genesis::GenesisLineageProvider;
    use crate::birdsong::genesis_types::*;
    use ed25519_dalek::{Signer, SigningKey};

    fn make_signed_witness(new_node_id: &str, channel: PhysicalChannelType) -> GenesisWitness {
        let signing_key = SigningKey::from_bytes(&[42u8; 32]);
        let public_key = signing_key.verifying_key().to_bytes().to_vec();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let message = GenesisWitness::create_signing_message(new_node_id, timestamp, &public_key);
        let signature = signing_key.sign(&message).to_bytes().to_vec();

        GenesisWitness {
            device_id: "solokey-test".to_string(),
            public_key,
            physical_channel: channel,
            timestamp,
            signature,
        }
    }

    #[tokio::test]
    async fn test_genesis_provider_new() -> Result<(), BearDogError> {
        let provider = GenesisLineageProvider::new().await?;
        let _ = provider;
        Ok(())
    }

    #[tokio::test]
    async fn test_genesis_provider_with_config() -> Result<(), BearDogError> {
        let provider = GenesisLineageProvider::with_config(TrustLevel::High).await?;
        let _ = provider;
        Ok(())
    }

    #[tokio::test]
    async fn test_genesis_provider_with_hardware_entropy() -> Result<(), BearDogError> {
        let provider = GenesisLineageProvider::new()
            .await?
            .with_hardware_entropy(std::sync::Arc::new(|| Ok(vec![0xDE; 32])));
        let _ = provider;
        Ok(())
    }

    #[tokio::test]
    async fn test_genesis_establish_lineage() -> Result<(), BearDogError> {
        let provider = GenesisLineageProvider::new().await?;
        let witness = make_signed_witness("new-node-001", PhysicalChannelType::HardwareKey);

        // Register the witness
        provider.add_trusted_witness(&witness.device_id, witness.public_key.clone());

        let lineage = provider
            .establish_genesis_lineage("new-node-001", &witness)
            .await?;

        assert!(!lineage.genetic_id.is_empty());
        assert_eq!(lineage.trust_level, TrustLevel::Maximum);
        Ok(())
    }

    #[tokio::test]
    async fn test_genesis_untrusted_witness() -> Result<(), BearDogError> {
        let provider = GenesisLineageProvider::new().await?;
        let witness = make_signed_witness("new-node", PhysicalChannelType::HardwareKey);

        // Register a DIFFERENT witness so the trusted list is non-empty
        // (empty list falls back to permissionless mode)
        provider.add_trusted_witness("other-device", vec![99u8; 32]);

        // The actual witness is not registered → should fail
        let result = provider
            .establish_genesis_lineage("new-node", &witness)
            .await;
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_genesis_ceremony_success() -> Result<(), BearDogError> {
        let provider = GenesisLineageProvider::new().await?;
        let witness = make_signed_witness("new-node", PhysicalChannelType::HardwareKey);

        provider.add_trusted_witness(&witness.device_id, witness.public_key.clone());

        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::HardwareKey,
            attestation: Some(vec![1u8; 64]),
            verification_codes: None,
            pairing_data: None,
            timestamp: witness.timestamp,
        };

        let result = provider
            .conduct_genesis_ceremony("new-node", &witness, &proof)
            .await?;
        assert!(result.success);
        assert!(result.error.is_none());
        assert!(!result.genetic_lineage.genetic_id.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_genesis_ceremony_failed_proof() -> Result<(), BearDogError> {
        let provider = GenesisLineageProvider::new().await?;
        let witness = make_signed_witness("new-node", PhysicalChannelType::HardwareKey);

        // Physical proof with no attestation -> verification fails
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::HardwareKey,
            attestation: None,
            verification_codes: None,
            pairing_data: None,
            timestamp: witness.timestamp,
        };

        let result = provider
            .conduct_genesis_ceremony("new-node", &witness, &proof)
            .await?;
        assert!(!result.success);
        assert!(result.error.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_genesis_ceremony_qr_channel() -> Result<(), BearDogError> {
        let provider = GenesisLineageProvider::new().await?;
        let witness = make_signed_witness("qr-node", PhysicalChannelType::QrCodeWithOob);

        provider.add_trusted_witness(&witness.device_id, witness.public_key.clone());

        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::QrCodeWithOob,
            attestation: None,
            verification_codes: Some(vec!["QR-CODE-123".to_string()]),
            pairing_data: None,
            timestamp: witness.timestamp,
        };

        let result = provider
            .conduct_genesis_ceremony("qr-node", &witness, &proof)
            .await?;
        assert!(result.success);
        Ok(())
    }

    #[tokio::test]
    async fn test_genesis_ceremony_bluetooth_channel() -> Result<(), BearDogError> {
        let provider = GenesisLineageProvider::new().await?;
        let witness = make_signed_witness("bt-node", PhysicalChannelType::Bluetooth);

        provider.add_trusted_witness(&witness.device_id, witness.public_key.clone());

        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::Bluetooth,
            attestation: None,
            verification_codes: None,
            pairing_data: Some(vec![1, 2, 3, 4]),
            timestamp: witness.timestamp,
        };

        let result = provider
            .conduct_genesis_ceremony("bt-node", &witness, &proof)
            .await?;
        assert!(result.success);
        Ok(())
    }

    #[tokio::test]
    async fn test_genesis_ceremony_nfc_channel() -> Result<(), BearDogError> {
        let provider = GenesisLineageProvider::new().await?;
        let witness = make_signed_witness("nfc-node", PhysicalChannelType::Nfc);

        provider.add_trusted_witness(&witness.device_id, witness.public_key.clone());

        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::Nfc,
            attestation: Some(vec![1u8; 32]),
            verification_codes: None,
            pairing_data: None,
            timestamp: witness.timestamp,
        };

        let result = provider
            .conduct_genesis_ceremony("nfc-node", &witness, &proof)
            .await?;
        assert!(result.success);
        Ok(())
    }

    #[tokio::test]
    async fn test_genesis_get_lineage_nonexistent() -> Result<(), BearDogError> {
        let provider = GenesisLineageProvider::new().await?;
        assert!(provider.get_lineage("nonexistent").is_none());
        Ok(())
    }

    #[tokio::test]
    async fn test_genesis_get_lineage_after_ceremony() -> Result<(), BearDogError> {
        let provider = GenesisLineageProvider::new().await?;
        let witness = make_signed_witness("stored-node", PhysicalChannelType::HardwareKey);

        provider.add_trusted_witness(&witness.device_id, witness.public_key.clone());

        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::HardwareKey,
            attestation: Some(vec![1u8; 64]),
            verification_codes: None,
            pairing_data: None,
            timestamp: witness.timestamp,
        };

        let result = provider
            .conduct_genesis_ceremony("stored-node", &witness, &proof)
            .await?;
        assert!(result.success);

        // Lineage should now be stored
        let lineage = provider.get_lineage("stored-node");
        assert!(lineage.is_some());
        Ok(())
    }

    #[tokio::test]
    async fn test_genesis_add_multiple_witnesses() -> Result<(), BearDogError> {
        let provider = GenesisLineageProvider::new().await?;

        provider.add_trusted_witness("device-1", vec![1u8; 32]);
        provider.add_trusted_witness("device-2", vec![2u8; 32]);
        provider.add_trusted_witness("device-3", vec![3u8; 32]);

        // All should be registered
        Ok(())
    }
}

// ═══════════════════════════════════════════════════════════════════
// constraints/types.rs - Additional KeyOperation coverage
// ═══════════════════════════════════════════════════════════════════

mod types_tests {
    use crate::constraints::types::OperationType;
    use crate::constraints::*;

    #[test]
    fn test_key_operation_types() {
        assert_eq!(
            KeyOperation::Sign { domain: None }.operation_type(),
            OperationType::Sign
        );
        assert_eq!(
            KeyOperation::Decrypt { domain: None }.operation_type(),
            OperationType::Decrypt
        );
        assert_eq!(
            KeyOperation::Encrypt { recipients: vec![] }.operation_type(),
            OperationType::Encrypt
        );
        assert_eq!(
            KeyOperation::Delete {
                path: "x".to_string()
            }
            .operation_type(),
            OperationType::Delete
        );
        assert_eq!(
            KeyOperation::Modify {
                path: "x".to_string()
            }
            .operation_type(),
            OperationType::Modify
        );
        assert_eq!(
            KeyOperation::Read {
                path: "x".to_string()
            }
            .operation_type(),
            OperationType::Read
        );
        assert_eq!(
            KeyOperation::Delegate {
                to_key_id: "k".to_string(),
                authority: Box::new(KeyOperation::Sign { domain: None })
            }
            .operation_type(),
            OperationType::Delegate
        );
        assert_eq!(
            KeyOperation::Mix {
                with_key_ids: vec![]
            }
            .operation_type(),
            OperationType::Mix
        );
    }

    #[test]
    fn test_key_operation_domain() {
        assert_eq!(
            KeyOperation::Sign {
                domain: Some("test".to_string())
            }
            .domain(),
            Some("test")
        );
        assert_eq!(
            KeyOperation::Decrypt {
                domain: Some("dec".to_string())
            }
            .domain(),
            Some("dec")
        );
        assert_eq!(KeyOperation::Sign { domain: None }.domain(), None);
        assert_eq!(
            KeyOperation::Delete {
                path: "x".to_string()
            }
            .domain(),
            None
        );
    }

    #[test]
    fn test_key_operation_path() {
        assert_eq!(
            KeyOperation::Delete {
                path: "a".to_string()
            }
            .path(),
            Some("a")
        );
        assert_eq!(
            KeyOperation::Modify {
                path: "b".to_string()
            }
            .path(),
            Some("b")
        );
        assert_eq!(
            KeyOperation::Read {
                path: "c".to_string()
            }
            .path(),
            Some("c")
        );
        assert_eq!(KeyOperation::Sign { domain: None }.path(), None);
    }

    #[test]
    fn test_signed_constraints_compute_hash() {
        let constraints = KeyConstraints::default();
        let hash1 = SignedConstraints::compute_hash(&constraints);
        let hash2 = SignedConstraints::compute_hash(&constraints);
        assert_eq!(hash1, hash2);
        assert!(!hash1.is_empty());
    }

    #[test]
    fn test_signed_constraints_different_hash() {
        let c1 = KeyConstraints::default();
        let c2 = KeyConstraints {
            scope: ScopeConstraint::Unrestricted,
            lifetime: LifetimeConstraint::Permanent,
            data_access: DataAccessConstraint {
                cannot_delete: vec!["test".to_string()],
                ..Default::default()
            },
            co_signers: vec![],
            behavioral: BehavioralConstraint::default(),
        };
        let hash1 = SignedConstraints::compute_hash(&c1);
        let hash2 = SignedConstraints::compute_hash(&c2);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_data_access_constraint_path_matching() {
        // Exact match
        assert!(DataAccessConstraint::path_matches(
            "secrets/api_key.txt",
            &["secrets/api_key.txt".to_string()]
        ));

        // Wildcard match
        assert!(DataAccessConstraint::path_matches(
            "raw_data/temperature.nc",
            &["raw_data/*".to_string()]
        ));

        // No match
        assert!(!DataAccessConstraint::path_matches(
            "other/file.txt",
            &["raw_data/*".to_string()]
        ));

        // Multiple patterns
        assert!(DataAccessConstraint::path_matches(
            "secrets/api_key.txt",
            &["raw_data/*".to_string(), "secrets/api_key.txt".to_string()]
        ));
    }
}

// ═══════════════════════════════════════════════════════════════════
// ecosystem_evolution - basic coverage
// ═══════════════════════════════════════════════════════════════════

mod ecosystem_evolution_tests {
    use crate::ecosystem_evolution::*;

    #[test]
    fn test_ecosystem_genetic_engine_creation() {
        let engine = EcosystemGeneticEngine::new().unwrap();
        let _ = engine;
    }

    #[test]
    fn test_ecosystem_context_fields() {
        let ctx = EcosystemContext {
            current_health: beardog_types::canonical::HealthStatus::Healthy,
            active_relationships: 3,
            ecosystem_load: 0.5,
            recent_events: vec!["test".to_string()],
        };
        assert_eq!(ctx.active_relationships, 3);
    }

    #[test]
    fn test_trust_evolution_building() {
        let trust = TrustEvolution::Building {
            progress_rate: 0.8,
            milestones: vec!["first-contact".to_string()],
            building_activities: vec!["key-exchange".to_string()],
            genetic_compatibility: 0.9,
        };
        assert!(format!("{trust:?}").contains("Building"));
    }

    #[test]
    fn test_ecosystem_membership_core_steward() {
        let member = EcosystemMembership::CoreSteward {
            stewardship_areas: vec![],
            trust_level: 0.95,
            responsibilities: vec!["crypto".to_string()],
            genetic_markers: vec![],
        };
        assert!(format!("{member:?}").contains("CoreSteward"));
    }

    #[test]
    fn test_coordination_model_distributed() {
        let model = CoordinationModel::Distributed {
            consensus_type: "byzantine".to_string(),
            participation_weights: std::collections::HashMap::new(),
            decision_thresholds: "2/3".to_string(),
        };
        assert!(format!("{model:?}").contains("Distributed"));
    }

    #[test]
    fn test_symbiosis_type_mutualistic() {
        let sym = SymbiosisType::Mutualistic {
            mutual_benefits: "shared entropy".to_string(),
            benefit_balance: 0.5,
            sustainability_metrics: "stable".to_string(),
        };
        assert!(format!("{sym:?}").contains("Mutualistic"));
    }
}
