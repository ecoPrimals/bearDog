// SPDX-License-Identifier: AGPL-3.0-only

//! Coverage: `GenesisLineageProvider` and genesis ceremony flows.

use beardog_errors::BearDogError;

// ═══════════════════════════════════════════════════════════════════
// birdsong/genesis.rs - GenesisLineageProvider
// ═══════════════════════════════════════════════════════════════════

mod genesis_provider_tests {
    use super::BearDogError;
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
