// SPDX-License-Identifier: AGPL-3.0-or-later

//! Integration tests for genesis module

use super::*;
use ed25519_dalek::{Signer, SigningKey};
use physical_proof::PhysicalProximityVerifier;
use std::time::{SystemTime, UNIX_EPOCH};
use types::{PhysicalChannelType, TrustLevel};
use witness::{GenesisWitness, GenesisWitnessVerifier};

fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Create a test witness with mock keys (for tests that only inspect struct fields,
/// not cryptographic verification).
fn create_test_witness(device_id: &str, channel: PhysicalChannelType) -> GenesisWitness {
    GenesisWitness::new(
        device_id.to_string(),
        vec![0u8; 32],
        channel,
        current_timestamp(),
        vec![0u8; 64],
    )
}

/// Create a properly signed test witness for cryptographic verification.
///
/// The signature matches the verification message format:
/// `BLAKE3(node_id || timestamp || device_id)`.
fn create_test_witness_for_node(
    device_id: &str,
    channel: PhysicalChannelType,
    node_id: &str,
) -> GenesisWitness {
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
        channel,
        timestamp,
        signature,
    )
}

#[test]
fn test_end_to_end_genesis_verification() {
    // Create a hardware key witness
    let witness = create_test_witness_for_node(
        "solokey-abc123",
        PhysicalChannelType::HardwareKey,
        "new-node-001",
    );

    let witness_verifier = GenesisWitnessVerifier::permissive();
    assert!(witness_verifier.verify(&witness, "new-node-001").is_ok());

    // Verify physical channel
    let physical_verifier = PhysicalProximityVerifier::default_genesis_config();
    let trust_level = physical_verifier
        .verify(witness.physical_channel)
        .expect("Physical verification should pass");

    // Hardware key should give maximum trust
    assert_eq!(trust_level, TrustLevel::Maximum);
    assert!(trust_level.is_sufficient_for_genesis());
}

#[test]
fn test_medium_trust_channel_acceptable() {
    // Create a Bluetooth witness (medium trust)
    let witness = create_test_witness_for_node(
        "phone-bluetooth",
        PhysicalChannelType::Bluetooth,
        "new-node-002",
    );

    let witness_verifier = GenesisWitnessVerifier::permissive();
    assert!(witness_verifier.verify(&witness, "new-node-002").is_ok());

    let physical_verifier = PhysicalProximityVerifier::default_genesis_config();
    let trust_level = physical_verifier
        .verify(witness.physical_channel)
        .expect("Bluetooth should be acceptable");

    assert_eq!(trust_level, TrustLevel::Medium);
    assert!(trust_level.is_sufficient_for_genesis());
}

#[test]
fn test_maximum_security_rejects_medium_trust() {
    // Create a Bluetooth witness (medium trust)
    let witness = create_test_witness("phone-bluetooth", PhysicalChannelType::Bluetooth);

    // Should fail with maximum security config
    let physical_verifier = PhysicalProximityVerifier::maximum_security();
    assert!(physical_verifier.verify(witness.physical_channel).is_err());
}

#[test]
fn test_all_physical_channels() {
    let channels = vec![
        (PhysicalChannelType::HardwareKey, TrustLevel::Maximum),
        (PhysicalChannelType::Nfc, TrustLevel::High),
        (PhysicalChannelType::QrCodeWithOob, TrustLevel::High),
        (PhysicalChannelType::Bluetooth, TrustLevel::Medium),
    ];

    let verifier = PhysicalProximityVerifier::default_genesis_config();

    for (channel, expected_trust) in channels {
        let witness = create_test_witness("test-device", channel);

        let trust_level = verifier
            .verify(witness.physical_channel)
            .unwrap_or_else(|_| panic!("Channel {channel:?} should be acceptable"));

        assert_eq!(
            trust_level, expected_trust,
            "Channel {channel:?} should have trust level {expected_trust:?}"
        );
    }
}

#[test]
fn test_witness_with_trusted_list() {
    let trusted = vec!["solokey-123".to_string()];
    let verifier = GenesisWitnessVerifier::with_trusted_witnesses(trusted);

    let trusted_witness =
        create_test_witness_for_node("solokey-123", PhysicalChannelType::HardwareKey, "test-node");
    assert!(verifier.verify(&trusted_witness, "test-node").is_ok());

    let untrusted_witness =
        create_test_witness_for_node("unknown-key", PhysicalChannelType::HardwareKey, "test-node");
    assert!(verifier.verify(&untrusted_witness, "test-node").is_err());
}
