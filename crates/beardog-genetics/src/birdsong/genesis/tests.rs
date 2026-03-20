// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for genesis lineage provider

use super::provider::GenesisLineageProvider;
use crate::birdsong::genesis_types::GenesisWitness;
use crate::birdsong::genesis_types::{PhysicalChannelProof, PhysicalChannelType, TrustLevel};
use beardog_errors::BearDogError;
use std::sync::Arc;

#[tokio::test]
async fn test_genesis_lineage_provider_creation() {
    let provider = GenesisLineageProvider::new().await.unwrap();
    assert_eq!(provider.min_trust_level, TrustLevel::Medium);
}

#[tokio::test]
async fn test_genesis_lineage_provider_with_custom_trust() {
    let provider = GenesisLineageProvider::with_config(TrustLevel::Maximum)
        .await
        .unwrap();
    assert_eq!(provider.min_trust_level, TrustLevel::Maximum);
}

#[tokio::test]
async fn test_add_trusted_witness() {
    let provider = GenesisLineageProvider::new().await.unwrap();
    let pubkey = vec![1u8; 32];

    provider.add_trusted_witness("test-witness", pubkey.clone());

    let stored = provider.trusted_witnesses.read();
    assert_eq!(stored.get("test-witness"), Some(&pubkey));
}

#[tokio::test]
async fn test_generate_genetic_id() {
    let provider = GenesisLineageProvider::new().await.unwrap();

    let witness = GenesisWitness {
        device_id: "test-witness".into(),
        public_key: vec![1u8; 32],
        physical_channel: PhysicalChannelType::HardwareKey,
        timestamp: 1735000000,
        signature: vec![0u8; 64],
    };

    let genetic_id = provider.generate_genetic_id("test-node", &witness).unwrap();

    assert_eq!(genetic_id.len(), 32);

    let genetic_id2 = provider.generate_genetic_id("test-node", &witness).unwrap();
    assert_eq!(genetic_id, genetic_id2);

    let genetic_id3 = provider
        .generate_genetic_id("different-node", &witness)
        .unwrap();
    assert_ne!(genetic_id, genetic_id3);
}

#[tokio::test]
async fn test_create_lineage_from_witness() {
    let provider = GenesisLineageProvider::new().await.unwrap();

    let witness = GenesisWitness {
        device_id: "witness-001".into(),
        public_key: vec![1u8; 32],
        physical_channel: PhysicalChannelType::HardwareKey,
        timestamp: 1735000000,
        signature: vec![0u8; 64],
    };

    let lineage = provider
        .create_lineage_from_witness("new-node", &witness)
        .unwrap();

    assert_eq!(lineage.nodes.len(), 2);
    assert_eq!(lineage.root_node.node_id, "witness-001");
    assert!(lineage.nodes.contains_key("witness-001"));
    assert!(lineage.nodes.contains_key("new-node"));
    assert_eq!(
        lineage.nodes.get("new-node").unwrap().parent_id,
        Some("witness-001".into())
    );
}

#[tokio::test]
async fn test_genesis_with_hardware_entropy() {
    let provider = GenesisLineageProvider::new()
        .await
        .unwrap()
        .with_hardware_entropy(Arc::new(|| {
            Ok(vec![
                0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad,
                0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef,
                0xde, 0xad, 0xbe, 0xef,
            ])
        }));

    let witness = GenesisWitness {
        device_id: "test-device".into(),
        public_key: vec![1u8; 32],
        physical_channel: PhysicalChannelType::HardwareKey,
        timestamp: 1735000000,
        signature: vec![2u8; 64],
    };

    let genetic_id = provider.generate_genetic_id("test-node", &witness).unwrap();

    assert_eq!(genetic_id.len(), 32);

    assert_ne!(genetic_id, vec![0u8; 32]);
}

#[tokio::test]
async fn test_genesis_without_hardware_entropy() {
    let provider = GenesisLineageProvider::new().await.unwrap();

    let witness = GenesisWitness {
        device_id: "test-device".into(),
        public_key: vec![1u8; 32],
        physical_channel: PhysicalChannelType::HardwareKey,
        timestamp: 1735000000,
        signature: vec![2u8; 64],
    };

    let genetic_id = provider.generate_genetic_id("test-node", &witness).unwrap();

    assert_eq!(genetic_id.len(), 32);
    assert_ne!(genetic_id, vec![0u8; 32]);
}

#[tokio::test]
async fn test_hardware_entropy_uniqueness() {
    let provider = GenesisLineageProvider::new()
        .await
        .unwrap()
        .with_hardware_entropy(Arc::new(|| {
            use rand::{RngCore, rngs::OsRng};
            let mut entropy = vec![0u8; 32];
            OsRng.fill_bytes(&mut entropy);
            Ok(entropy)
        }));

    let witness = GenesisWitness {
        device_id: "test-device".into(),
        public_key: vec![1u8; 32],
        physical_channel: PhysicalChannelType::HardwareKey,
        timestamp: 1735000000,
        signature: vec![2u8; 64],
    };

    let id1 = provider.generate_genetic_id("node-1", &witness).unwrap();
    let id2 = provider.generate_genetic_id("node-2", &witness).unwrap();

    assert_ne!(id1, id2);
}

#[tokio::test]
async fn test_hardware_entropy_failure_fallback() {
    let provider = GenesisLineageProvider::new()
        .await
        .unwrap()
        .with_hardware_entropy(Arc::new(|| {
            Err(BearDogError::unavailable(
                "Hardware entropy unavailable".into(),
            ))
        }));

    let witness = GenesisWitness {
        device_id: "test-device".into(),
        public_key: vec![1u8; 32],
        physical_channel: PhysicalChannelType::HardwareKey,
        timestamp: 1735000000,
        signature: vec![2u8; 64],
    };

    let genetic_id = provider.generate_genetic_id("test-node", &witness).unwrap();

    assert_eq!(genetic_id.len(), 32);
    assert_ne!(genetic_id, vec![0u8; 32]);
}

#[tokio::test]
async fn test_hardware_entropy_determinism() {
    let fixed_entropy = vec![0x42u8; 32];
    let provider = GenesisLineageProvider::new()
        .await
        .unwrap()
        .with_hardware_entropy(Arc::new(move || Ok(fixed_entropy.clone())));

    let witness = GenesisWitness {
        device_id: "test-device".into(),
        public_key: vec![1u8; 32],
        physical_channel: PhysicalChannelType::HardwareKey,
        timestamp: 1735000000,
        signature: vec![2u8; 64],
    };

    let id1 = provider.generate_genetic_id("test-node", &witness).unwrap();
    let id2 = provider.generate_genetic_id("test-node", &witness).unwrap();

    assert_eq!(id1, id2);
}

#[tokio::test]
#[serial_test::serial]
async fn test_verify_witness_authority_permissionless_mode() {
    beardog_errors::process_env::set_var("BEARDOG_GENESIS_MODE", "permissionless");
    let provider = GenesisLineageProvider::new().await.unwrap();

    let witness = GenesisWitness {
        device_id: "any-device".into(),
        public_key: vec![1u8; 32],
        physical_channel: PhysicalChannelType::HardwareKey,
        timestamp: 1735000000,
        signature: vec![0u8; 64],
    };

    let result = provider.verify_witness_authority(&witness);
    beardog_errors::process_env::remove_var("BEARDOG_GENESIS_MODE");
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_verify_witness_authority_invalid_pubkey_length() {
    let provider = GenesisLineageProvider::new().await.unwrap();

    let witness = GenesisWitness {
        device_id: "device".into(),
        public_key: vec![1u8; 16],
        physical_channel: PhysicalChannelType::HardwareKey,
        timestamp: 1735000000,
        signature: vec![0u8; 64],
    };

    let result = provider.verify_witness_authority(&witness);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_verify_witness_authority_empty_signature() {
    let provider = GenesisLineageProvider::new().await.unwrap();

    let witness = GenesisWitness {
        device_id: "device".into(),
        public_key: vec![1u8; 32],
        physical_channel: PhysicalChannelType::HardwareKey,
        timestamp: 1735000000,
        signature: vec![],
    };

    let result = provider.verify_witness_authority(&witness);
    assert!(result.is_err());
}

#[tokio::test]
#[serial_test::serial]
async fn test_verify_witness_authority_unknown_mode() {
    beardog_errors::process_env::set_var("BEARDOG_GENESIS_MODE", "unknown_mode");
    let provider = GenesisLineageProvider::new().await.unwrap();

    let witness = GenesisWitness {
        device_id: "device".into(),
        public_key: vec![1u8; 32],
        physical_channel: PhysicalChannelType::HardwareKey,
        timestamp: 1735000000,
        signature: vec![0u8; 64],
    };

    let result = provider.verify_witness_authority(&witness);
    beardog_errors::process_env::remove_var("BEARDOG_GENESIS_MODE");
    assert!(result.is_err());
}

#[tokio::test]
async fn test_verify_witness_authority_pubkey_mismatch() {
    let provider = GenesisLineageProvider::new().await.unwrap();
    provider.add_trusted_witness("device", vec![1u8; 32]);

    let witness = GenesisWitness {
        device_id: "device".into(),
        public_key: vec![2u8; 32],
        physical_channel: PhysicalChannelType::HardwareKey,
        timestamp: 1735000000,
        signature: vec![0u8; 64],
    };

    let result = provider.verify_witness_authority(&witness);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("does not match"));
}

#[tokio::test]
async fn test_conduct_genesis_ceremony_failed_proof() {
    let provider = GenesisLineageProvider::new().await.unwrap();

    let witness = GenesisWitness {
        device_id: "witness-device".into(),
        public_key: vec![1u8; 32],
        physical_channel: PhysicalChannelType::HardwareKey,
        timestamp: 1735000000,
        signature: vec![0u8; 64],
    };

    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::HardwareKey,
        attestation: None,
        verification_codes: None,
        pairing_data: None,
        timestamp: 1735000000,
    };

    let result = provider
        .conduct_genesis_ceremony("node-1", &witness, &proof)
        .await
        .unwrap();

    assert!(!result.success);
    assert!(result.error.is_some());
}

#[tokio::test]
async fn test_conduct_genesis_ceremony_lineage_failure() {
    let provider = GenesisLineageProvider::new().await.unwrap();
    provider.add_trusted_witness("other-device", vec![99u8; 32]);

    let witness = GenesisWitness {
        device_id: "unregistered-device".into(),
        public_key: vec![1u8; 32],
        physical_channel: PhysicalChannelType::QrCodeWithOob,
        timestamp: 1735000000,
        signature: vec![0u8; 64],
    };

    let proof = PhysicalChannelProof {
        channel_type: PhysicalChannelType::QrCodeWithOob,
        attestation: None,
        verification_codes: Some(vec!["CODE123".to_string()]),
        pairing_data: None,
        timestamp: 1735000000,
    };

    let result = provider
        .conduct_genesis_ceremony("node-1", &witness, &proof)
        .await
        .unwrap();

    assert!(!result.success);
    assert!(result.error.is_some());
}

#[tokio::test]
async fn test_get_lineage_not_found() {
    let provider = GenesisLineageProvider::new().await.unwrap();
    assert!(provider.get_lineage("nonexistent").is_none());
}
