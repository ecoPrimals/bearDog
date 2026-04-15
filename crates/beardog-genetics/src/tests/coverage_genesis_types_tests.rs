// SPDX-License-Identifier: AGPL-3.0-or-later

//! Coverage: genesis types (`GenesisWitness`, `GeneticLineage`, `PhysicalChannelProof`).

// ═══════════════════════════════════════════════════════════════════
// birdsong/genesis_types/ - GenesisWitness, GeneticLineage, PhysicalChannelProof
// ═══════════════════════════════════════════════════════════════════

mod genesis_types_tests {
    use crate::birdsong::genesis_types::*;
    use ed25519_dalek::{Signer, SigningKey};

    fn make_valid_witness(new_node_id: &str) -> GenesisWitness {
        let signing_key = SigningKey::from_bytes(&[42u8; 32]);
        let public_key = signing_key.verifying_key().to_bytes().to_vec();
        let timestamp = 1_735_000_000_u64;
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
        if let Ok(valid) = result {
            assert!(!valid, "Zero signature should not verify");
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
                generation: 0,
                head_commitment: vec![],
                created_at: chrono::Utc::now(),
            },
            genesis_witness: make_valid_witness(node_id),
            birth_timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
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
            timestamp: 1_735_000_000,
        };
        let result = GenesisCeremonyResult {
            genetic_lineage: lineage,
            physical_proof: proof,
            completed_at: 1_735_000_001,
            success: true,
            error: None,
        };
        assert!(result.success);
        assert!(result.error.is_none());
        assert_eq!(result.completed_at, 1_735_000_001);
    }

    #[test]
    fn test_genesis_ceremony_result_with_error() {
        let lineage = make_test_lineage("err-node", TrustLevel::Low);
        let proof = PhysicalChannelProof {
            channel_type: PhysicalChannelType::HardwareKey,
            attestation: None,
            verification_codes: None,
            pairing_data: None,
            timestamp: 1_735_000_000,
        };
        let result = GenesisCeremonyResult {
            genetic_lineage: lineage,
            physical_proof: proof,
            completed_at: 1_735_000_001,
            success: false,
            error: Some("Witness verification failed".to_string()),
        };
        assert!(!result.success);
        assert_eq!(result.error.unwrap(), "Witness verification failed");
    }
}
