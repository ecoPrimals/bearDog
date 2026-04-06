// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use crate::birdsong::types::LineageMetadata;
use crate::birdsong::{LineageChain, LineageNode};

#[test]
fn test_physical_channel_trust_levels() {
    assert_eq!(
        PhysicalChannelType::HardwareKey.trust_level(),
        TrustLevel::Maximum
    );
    assert_eq!(
        PhysicalChannelType::QrCodeWithOob.trust_level(),
        TrustLevel::High
    );
    assert_eq!(
        PhysicalChannelType::Bluetooth.trust_level(),
        TrustLevel::Medium
    );
    assert_eq!(PhysicalChannelType::Nfc.trust_level(), TrustLevel::High);
}

#[test]
fn test_trust_level_ordering() {
    assert!(TrustLevel::Maximum > TrustLevel::High);
    assert!(TrustLevel::High > TrustLevel::Medium);
    assert!(TrustLevel::Medium > TrustLevel::Low);
}

#[test]
fn test_trust_level_threshold() {
    assert!(TrustLevel::Maximum.meets_threshold(TrustLevel::High));
    assert!(TrustLevel::High.meets_threshold(TrustLevel::Medium));
    assert!(!TrustLevel::Medium.meets_threshold(TrustLevel::High));
}

#[test]
fn test_witness_signing_message_format() {
    let node_id = "test-node-123";
    let timestamp = 1_735_000_000_u64;
    let pubkey = vec![1u8; 32];

    let message = GenesisWitness::create_signing_message(node_id, timestamp, &pubkey);

    // Should contain node_id + 8 bytes timestamp + 32 bytes pubkey
    assert_eq!(message.len(), node_id.len() + 8 + 32);

    // Check timestamp is big-endian
    let ts_bytes = &message[node_id.len()..node_id.len() + 8];
    assert_eq!(
        u64::from_be_bytes(
            ts_bytes
                .try_into()
                .expect("timestamp slice should be exactly 8 bytes"),
        ),
        timestamp
    );
}

// === PhysicalChannelProof verify tests ===

#[test]
fn test_proof_verify_hardware_key_no_attestation() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::HardwareKey,
        attestation: None,
        verification_codes: None,
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert!(!proof.verify().expect("verify should return Result in test"));
}

#[test]
fn test_proof_verify_hardware_key_with_attestation() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::HardwareKey,
        attestation: Some(vec![1u8; 64]),
        verification_codes: None,
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert!(
        proof
            .verify_with_attestation_mode("permissionless")
            .expect("permissionless attestation verify")
    );
}

#[test]
fn test_proof_verify_qr_code_with_codes() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::QrCodeWithOob,
        attestation: None,
        verification_codes: Some(vec!["CODE123".to_string()]),
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert!(proof.verify().expect("verify should return Result in test"));
}

#[test]
fn test_proof_verify_qr_code_no_codes() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::QrCodeWithOob,
        attestation: None,
        verification_codes: None,
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert!(!proof.verify().expect("verify should return Result in test"));
}

#[test]
fn test_proof_verify_qr_code_empty_codes() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::QrCodeWithOob,
        attestation: None,
        verification_codes: Some(vec![]),
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert!(!proof.verify().expect("verify should return Result in test"));
}

#[test]
fn test_proof_verify_bluetooth_with_pairing() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::Bluetooth,
        attestation: None,
        verification_codes: None,
        pairing_data: Some(vec![1u8; 32]),
        timestamp: 1_735_000_000,
    };
    assert!(proof.verify().expect("verify should return Result in test"));
}

#[test]
fn test_proof_verify_bluetooth_no_pairing() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::Bluetooth,
        attestation: None,
        verification_codes: None,
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert!(!proof.verify().expect("verify should return Result in test"));
}

#[test]
fn test_proof_verify_nfc_with_attestation() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::Nfc,
        attestation: Some(vec![1u8; 32]),
        verification_codes: None,
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert!(proof.verify().expect("verify should return Result in test"));
}

#[test]
fn test_proof_verify_nfc_no_attestation() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::Nfc,
        attestation: None,
        verification_codes: None,
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert!(!proof.verify().expect("verify should return Result in test"));
}

#[test]
fn test_proof_trust_level() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::HardwareKey,
        attestation: None,
        verification_codes: None,
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert_eq!(proof.trust_level(), TrustLevel::Maximum);
}

// === Hardware attestation mode tests ===

#[test]
fn test_attestation_software_mode_short() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::HardwareKey,
        attestation: Some(vec![1u8; 16]), // too short for software mode
        verification_codes: None,
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert!(
        !proof
            .verify_with_attestation_mode("software")
            .expect("software attestation verify")
    );
}

#[test]
fn test_attestation_software_mode_valid() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::HardwareKey,
        attestation: Some(vec![1u8; 64]),
        verification_codes: None,
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert!(
        proof
            .verify_with_attestation_mode("software")
            .expect("software attestation verify")
    );
}

#[test]
fn test_attestation_software_mode_zero_hash() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::HardwareKey,
        attestation: Some(vec![0u8; 64]), // all zeros
        verification_codes: None,
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert!(
        !proof
            .verify_with_attestation_mode("software")
            .expect("software attestation verify")
    );
}

#[test]
fn test_attestation_permissionless_mode() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::HardwareKey,
        attestation: Some(vec![42u8; 1]), // any non-empty
        verification_codes: None,
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert!(
        proof
            .verify_with_attestation_mode("permissionless")
            .expect("permissionless attestation verify")
    );
}

#[test]
fn test_attestation_permissionless_mode_empty() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::HardwareKey,
        attestation: Some(vec![]), // empty
        verification_codes: None,
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert!(
        !proof
            .verify_with_attestation_mode("permissionless")
            .expect("permissionless attestation verify")
    );
}

#[test]
fn test_attestation_hardware_mode() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::HardwareKey,
        attestation: Some(vec![1u8; 128]),
        verification_codes: None,
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    // On linux, this uses verify_tpm_attestation which checks len >= 64
    let result = proof
        .verify_with_attestation_mode("hardware")
        .expect("hardware attestation verify");
    assert!(result);
}

#[test]
fn test_attestation_unknown_mode() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::HardwareKey,
        attestation: Some(vec![1u8; 64]), // valid for software fallback
        verification_codes: None,
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert!(
        proof
            .verify_with_attestation_mode("unknown_mode")
            .expect("unknown attestation mode verify")
    );
}

#[test]
fn test_attestation_empty_attestation() {
    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::HardwareKey,
        attestation: Some(vec![]),
        verification_codes: None,
        pairing_data: None,
        timestamp: 1_735_000_000,
    };
    assert!(!proof.verify().expect("verify should return Result in test"));
}

// === GeneticLineage tests ===

#[test]
fn test_genetic_lineage_depth() {
    let mut nodes = std::collections::HashMap::new();
    nodes.insert(
        "root".to_string(),
        LineageNode {
            node_id: "root".to_string(),
            parent_id: None,
            public_key: vec![1u8; 32],
            depth: 0,
            created_at: chrono::Utc::now(),
            metadata: LineageMetadata::default(),
        },
    );
    nodes.insert(
        "child".to_string(),
        LineageNode {
            node_id: "child".to_string(),
            parent_id: Some("root".to_string()),
            public_key: vec![2u8; 32],
            depth: 1,
            created_at: chrono::Utc::now(),
            metadata: LineageMetadata::default(),
        },
    );

    let lineage = GeneticLineage {
        genetic_id: vec![1u8; 32],
        lineage_chain: LineageChain {
            chain_id: "test-chain".to_string(),
            root_node: LineageNode {
                node_id: "root".to_string(),
                parent_id: None,
                public_key: vec![1u8; 32],
                depth: 0,
                created_at: chrono::Utc::now(),
                metadata: LineageMetadata::default(),
            },
            nodes,
            relationships: vec![],
            created_at: chrono::Utc::now(),
        },
        genesis_witness: GenesisWitness {
            device_id: "witness".to_string(),
            public_key: vec![1u8; 32],
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1_735_000_000,
            signature: vec![0u8; 64],
        },
        birth_timestamp: 1_735_000_000,
        trust_level: TrustLevel::High,
    };

    assert_eq!(lineage.depth(), 2);
}

#[test]
fn test_genetic_lineage_hint() {
    let mut nodes = std::collections::HashMap::new();
    nodes.insert(
        "root".to_string(),
        LineageNode {
            node_id: "root".to_string(),
            parent_id: None,
            public_key: vec![1u8; 32],
            depth: 0,
            created_at: chrono::Utc::now(),
            metadata: LineageMetadata::default(),
        },
    );

    let lineage = GeneticLineage {
        genetic_id: vec![1u8; 32],
        lineage_chain: LineageChain {
            chain_id: "test-chain".to_string(),
            root_node: LineageNode {
                node_id: "root".to_string(),
                parent_id: None,
                public_key: vec![1u8; 32],
                depth: 0,
                created_at: chrono::Utc::now(),
                metadata: LineageMetadata::default(),
            },
            nodes,
            relationships: vec![],
            created_at: chrono::Utc::now(),
        },
        genesis_witness: GenesisWitness {
            device_id: "witness".to_string(),
            public_key: vec![1u8; 32],
            physical_channel: PhysicalChannelType::HardwareKey,
            timestamp: 1_735_000_000,
            signature: vec![0u8; 64],
        },
        birth_timestamp: 1_735_000_000,
        trust_level: TrustLevel::High,
    };

    let hint = lineage.lineage_hint();
    assert_eq!(hint.root_id, "root");
    assert_eq!(hint.min_depth, 0);
    assert_eq!(hint.max_depth, 1);
    assert_eq!(hint.biome_filter, Some("genesis".to_string()));
}
