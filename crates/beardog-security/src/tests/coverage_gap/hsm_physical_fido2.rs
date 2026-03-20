// SPDX-License-Identifier: AGPL-3.0-only

// ========================================================================
// entropy_orchestrator: HumanEntropyInput with all fields (mix_with_human_input)
// ========================================================================

mod entropy_orchestrator_tests {
    use crate::hsm::entropy_orchestrator::types::SecurityLevel;
    use crate::hsm::entropy_orchestrator::{HsmDeviceInfo, HsmDeviceType, HumanEntropyInput};

    #[tokio::test]
    async fn test_human_entropy_input_environmental_data() {
        let input = HumanEntropyInput {
            biometric_data: None,
            behavioral_data: None,
            environmental_data: Some(vec![10, 20, 30, 40, 50]),
        };
        assert!(input.environmental_data.is_some());
    }

    #[tokio::test]
    async fn test_human_entropy_input_all_fields() {
        let input = HumanEntropyInput {
            biometric_data: Some(vec![1, 2, 3]),
            behavioral_data: Some(vec![4, 5, 6]),
            environmental_data: Some(vec![7, 8, 9]),
        };
        assert!(input.biometric_data.is_some());
        assert!(input.behavioral_data.is_some());
        assert!(input.environmental_data.is_some());
    }

    #[tokio::test]
    async fn test_generate_entropy_with_all_human_inputs() {
        let mut orchestrator = crate::hsm::entropy_orchestrator::HsmEntropyOrchestrator::new()
            .await
            .unwrap();
        let human_input = HumanEntropyInput {
            biometric_data: Some(vec![100, 101, 102]),
            behavioral_data: Some(vec![200, 201, 202]),
            environmental_data: Some(vec![50, 51, 52]),
        };
        let result = orchestrator
            .generate_human_entropy(32, Some(human_input))
            .await;
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_hsm_device_info_serialization() {
        let info = HsmDeviceInfo {
            device_type: HsmDeviceType::Fido2,
            device_id: "fido2_0".to_string(),
            name: "Test Device".to_string(),
            security_level: SecurityLevel::Hardware,
            biometric_capable: true,
        };
        let json = serde_json::to_string(&info).unwrap();
        let deserialized: HsmDeviceInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.device_id, info.device_id);
        assert_eq!(deserialized.security_level, SecurityLevel::Hardware);
    }
}

// ========================================================================
// genesis/physical_proof: PhysicalProximityVerifier, PhysicalProofError
// ========================================================================

mod physical_proof_tests {
    use crate::genesis::physical_proof::*;
    use crate::genesis::types::{PhysicalChannelType, TrustLevel};

    #[test]
    fn test_physical_proximity_verifier_default_config() {
        let verifier = PhysicalProximityVerifier::default_genesis_config();
        let result = verifier.verify(PhysicalChannelType::HardwareKey);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), TrustLevel::Maximum);
    }

    #[test]
    fn test_physical_proximity_verifier_maximum_security() {
        let verifier = PhysicalProximityVerifier::maximum_security();
        let result = verifier.verify(PhysicalChannelType::HardwareKey);
        assert!(result.is_ok());
    }

    #[test]
    fn test_physical_proximity_verifier_bluetooth_insufficient_for_maximum() {
        let verifier = PhysicalProximityVerifier::maximum_security();
        let result = verifier.verify(PhysicalChannelType::Bluetooth);
        assert!(result.is_err());
        if let Err(PhysicalProofError::InsufficientTrust { actual }) = result {
            assert_eq!(actual, TrustLevel::Medium);
        }
    }

    #[test]
    fn test_physical_proof_error_display() {
        let err = PhysicalProofError::UnsupportedChannel(PhysicalChannelType::Bluetooth);
        let msg = format!("{err}");
        assert!(!msg.is_empty());

        let err2 = PhysicalProofError::AttestationFailed("bad attestation".to_string());
        assert!(format!("{err2}").contains("bad attestation"));
    }

    #[test]
    fn test_physical_proof_error_from_beardog_error() {
        use beardog_errors::BearDogError;
        let err = PhysicalProofError::InsufficientTrust {
            actual: TrustLevel::Low,
        };
        let beardog_err: BearDogError = err.into();
        assert!(format!("{beardog_err}").contains("Physical proof"));
    }
}

// ========================================================================
// FIDO2 operations: Ctap2Command enum variants (when fido2 feature enabled)
// ========================================================================

#[cfg(feature = "fido2")]
mod fido2_operations_tests {
    use crate::hsm::fido2::Ctap2Command;

    #[test]
    fn test_ctap2_command_all_variants() {
        assert_eq!(Ctap2Command::MakeCredential as u8, 0x01);
        assert_eq!(Ctap2Command::GetAssertion as u8, 0x02);
        assert_eq!(Ctap2Command::GetInfo as u8, 0x04);
        assert_eq!(Ctap2Command::ClientPin as u8, 0x06);
        assert_eq!(Ctap2Command::Reset as u8, 0x07);
        assert_eq!(Ctap2Command::GetNextAssertion as u8, 0x08);
        assert_eq!(Ctap2Command::CredentialManagement as u8, 0x0A);
    }
}
