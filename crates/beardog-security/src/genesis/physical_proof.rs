//! Physical proximity proof verification
//!
//! Verifies that genesis ceremony occurred over a physical channel
//! and assigns trust levels accordingly.

use super::types::{PhysicalChannelType, TrustLevel};
use beardog_errors::BearDogError;

/// Errors that can occur during physical proof verification
#[derive(Debug, thiserror::Error)]
pub enum PhysicalProofError {
    /// Physical channel type is not supported
    #[error("Physical channel type not supported: {0:?}")]
    UnsupportedChannel(PhysicalChannelType),

    /// Trust level is insufficient for genesis
    #[error("Insufficient trust level: {actual:?} (minimum required: Medium)")]
    InsufficientTrust {
        /// The actual trust level provided
        actual: TrustLevel,
    },

    /// Physical attestation failed verification
    #[error("Physical attestation verification failed: {0}")]
    AttestationFailed(String),
}

impl From<PhysicalProofError> for BearDogError {
    fn from(err: PhysicalProofError) -> Self {
        BearDogError::security(format!("Physical proof verification failed: {err}"))
    }
}

/// Physical proximity verifier
///
/// Verifies that genesis ceremony occurred over a legitimate physical channel
/// and assigns appropriate trust levels.
///
/// # Trust Levels
///
/// - **Hardware Key** (SoloKey, YubiKey): ⭐⭐⭐⭐⭐ Maximum
/// - **QR Code + OOB**: ⭐⭐⭐⭐ High
/// - **NFC Tap**: ⭐⭐⭐⭐ High
/// - **Bluetooth**: ⭐⭐⭐ Medium
pub struct PhysicalProximityVerifier {
    /// Minimum trust level required for genesis (default: Medium)
    minimum_trust: TrustLevel,
}

impl PhysicalProximityVerifier {
    /// Create a new physical proximity verifier
    ///
    /// # Arguments
    ///
    /// * `minimum_trust` - Minimum trust level required for genesis
    pub fn new(minimum_trust: TrustLevel) -> Self {
        Self { minimum_trust }
    }

    /// Create verifier with default settings (Medium trust minimum)
    pub fn default_genesis_config() -> Self {
        Self::new(TrustLevel::Medium)
    }

    /// Create verifier requiring maximum trust (Hardware key only)
    pub fn maximum_security() -> Self {
        Self::new(TrustLevel::Maximum)
    }

    /// Verify physical channel and return trust level
    ///
    /// Performs verification of the physical channel used for genesis:
    /// 1. Validate channel type is supported
    /// 2. Determine trust level based on channel
    /// 3. Verify trust level meets minimum requirement
    /// 4. Optional: Verify channel-specific attestation
    ///
    /// # Arguments
    ///
    /// * `channel` - The physical channel type used
    ///
    /// # Returns
    ///
    /// Ok(TrustLevel) if channel is valid and meets requirements
    pub fn verify(&self, channel: PhysicalChannelType) -> Result<TrustLevel, PhysicalProofError> {
        // 1. Get trust level for channel
        let trust_level = channel.trust_level();

        // 2. Verify minimum trust requirement
        if trust_level < self.minimum_trust {
            return Err(PhysicalProofError::InsufficientTrust {
                actual: trust_level,
            });
        }

        // 3. Verify genesis sufficiency
        if !trust_level.is_sufficient_for_genesis() {
            return Err(PhysicalProofError::InsufficientTrust {
                actual: trust_level,
            });
        }

        // 4. Channel-specific attestation (Phase 1: basic validation)
        self.verify_channel_attestation(channel)?;

        Ok(trust_level)
    }

    /// Verify channel-specific attestation
    ///
    /// For Phase 1 (Week 1-2): Basic validation  
    /// For Phase 2 (Week 5): Add hardware attestation verification
    fn verify_channel_attestation(
        &self,
        channel: PhysicalChannelType,
    ) -> Result<(), PhysicalProofError> {
        match channel {
            PhysicalChannelType::HardwareKey => {
                // Phase 1: Accept all hardware keys
                // Phase 2: Verify hardware attestation (FIDO2 attestation)
                Ok(())
            }
            PhysicalChannelType::QrCodeWithOob => {
                // Phase 1: Accept if OOB verified
                // Phase 2: Verify OOB verification proof
                Ok(())
            }
            PhysicalChannelType::Nfc => {
                // Phase 1: Accept NFC
                // Phase 2: Verify NFC secure element attestation
                Ok(())
            }
            PhysicalChannelType::Bluetooth => {
                // Phase 1: Accept Bluetooth
                // Phase 2: Verify Bluetooth pairing proof
                Ok(())
            }
        }
    }

    /// Check if a channel is acceptable for genesis
    pub fn is_channel_acceptable(&self, channel: PhysicalChannelType) -> bool {
        self.verify(channel).is_ok()
    }

    /// Get minimum trust level required
    pub fn minimum_trust_level(&self) -> TrustLevel {
        self.minimum_trust
    }
}

impl Default for PhysicalProximityVerifier {
    fn default() -> Self {
        Self::default_genesis_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_accepts_medium_and_above() {
        let verifier = PhysicalProximityVerifier::default_genesis_config();

        // Maximum trust should pass
        assert!(verifier.verify(PhysicalChannelType::HardwareKey).is_ok());

        // High trust should pass
        assert!(verifier.verify(PhysicalChannelType::Nfc).is_ok());
        assert!(verifier.verify(PhysicalChannelType::QrCodeWithOob).is_ok());

        // Medium trust should pass
        assert!(verifier.verify(PhysicalChannelType::Bluetooth).is_ok());
    }

    #[test]
    fn test_maximum_security_requires_hardware_key() {
        let verifier = PhysicalProximityVerifier::maximum_security();

        // Only hardware key should pass
        assert!(verifier.verify(PhysicalChannelType::HardwareKey).is_ok());

        // All others should fail
        assert!(verifier.verify(PhysicalChannelType::Nfc).is_err());
        assert!(verifier.verify(PhysicalChannelType::QrCodeWithOob).is_err());
        assert!(verifier.verify(PhysicalChannelType::Bluetooth).is_err());
    }

    #[test]
    fn test_trust_level_returned() {
        let verifier = PhysicalProximityVerifier::default_genesis_config();

        // Verify correct trust levels are returned
        assert_eq!(
            verifier.verify(PhysicalChannelType::HardwareKey).unwrap(),
            TrustLevel::Maximum
        );
        assert_eq!(
            verifier.verify(PhysicalChannelType::Nfc).unwrap(),
            TrustLevel::High
        );
        assert_eq!(
            verifier.verify(PhysicalChannelType::Bluetooth).unwrap(),
            TrustLevel::Medium
        );
    }

    #[test]
    fn test_channel_acceptability_check() {
        let verifier = PhysicalProximityVerifier::default_genesis_config();

        assert!(verifier.is_channel_acceptable(PhysicalChannelType::HardwareKey));
        assert!(verifier.is_channel_acceptable(PhysicalChannelType::Nfc));
        assert!(verifier.is_channel_acceptable(PhysicalChannelType::QrCodeWithOob));
        assert!(verifier.is_channel_acceptable(PhysicalChannelType::Bluetooth));
    }

    #[test]
    fn test_high_trust_requirement() {
        let verifier = PhysicalProximityVerifier::new(TrustLevel::High);

        // High and maximum should pass
        assert!(verifier.verify(PhysicalChannelType::HardwareKey).is_ok());
        assert!(verifier.verify(PhysicalChannelType::Nfc).is_ok());

        // Medium should fail
        assert!(verifier.verify(PhysicalChannelType::Bluetooth).is_err());
    }
}
