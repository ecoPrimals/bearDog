//! Core types for genesis ceremony
//!
//! Defines the fundamental types used in physical genesis bootstrap.
//!
//! These types are re-exported by beardog-genetics for use in lineage establishment.

use serde::{Deserialize, Serialize};

/// Physical channel type used for genesis ceremony
///
/// Different physical channels provide different levels of trust and security.
/// Hardware keys (SoloKey, YubiKey) provide the highest trust level.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PhysicalChannelType {
    /// Hardware security key (SoloKey, YubiKey)
    ///
    /// Trust level: ⭐⭐⭐⭐⭐ (Maximum)
    ///
    /// Provides cryptographic attestation, tamper resistance, and
    /// secure key storage. Recommended for all production genesis ceremonies.
    HardwareKey,

    /// QR code with out-of-band verification
    ///
    /// Trust level: ⭐⭐⭐⭐ (High)
    ///
    /// QR code displayed on trusted device, scanned by new node,
    /// with additional out-of-band verification step.
    QrCodeWithOob,

    /// NFC tap
    ///
    /// Trust level: ⭐⭐⭐⭐ (High)
    ///
    /// Near-field communication tap between devices.
    /// Requires physical proximity (< 10cm).
    Nfc,

    /// Bluetooth pairing
    ///
    /// Trust level: ⭐⭐⭐ (Medium)
    ///
    /// Bluetooth Low Energy pairing with PIN verification.
    /// More vulnerable to relay attacks than NFC.
    Bluetooth,
}

impl PhysicalChannelType {
    /// Get the trust level for this physical channel
    pub fn trust_level(self) -> TrustLevel {
        match self {
            Self::HardwareKey => TrustLevel::Maximum,
            Self::QrCodeWithOob | Self::Nfc => TrustLevel::High,
            Self::Bluetooth => TrustLevel::Medium,
        }
    }

    /// Check if this channel requires out-of-band verification
    pub fn requires_oob(self) -> bool {
        matches!(self, Self::QrCodeWithOob)
    }

    /// Get human-readable description of this channel
    pub fn description(self) -> &'static str {
        match self {
            Self::HardwareKey => "Hardware Security Key (SoloKey/YubiKey)",
            Self::QrCodeWithOob => "QR Code with Out-of-Band Verification",
            Self::Nfc => "NFC Tap (Near-Field Communication)",
            Self::Bluetooth => "Bluetooth Pairing",
        }
    }

    /// Whether this channel supports hardware attestation
    pub fn supports_attestation(self) -> bool {
        matches!(self, Self::HardwareKey | Self::Nfc)
    }
}

/// Trust level assigned based on physical channel type
///
/// Trust levels determine how much confidence we have in the genesis ceremony.
/// Higher trust levels may grant more privileges or skip additional verification steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub enum TrustLevel {
    /// ⭐ - Low trust (not used for genesis, reserved for future)
    Low = 1,

    /// ⭐⭐⭐ - Medium trust (Bluetooth)
    #[default]
    Medium = 3,

    /// ⭐⭐⭐⭐ - High trust (QR+OOB, NFC)
    High = 4,

    /// ⭐⭐⭐⭐⭐ - Maximum trust (Hardware key)
    Maximum = 5,
}

impl TrustLevel {
    /// Get human-readable description of trust level
    pub fn description(self) -> &'static str {
        match self {
            Self::Low => "Low (⭐)",
            Self::Medium => "Medium (⭐⭐⭐)",
            Self::High => "High (⭐⭐⭐⭐)",
            Self::Maximum => "Maximum (⭐⭐⭐⭐⭐)",
        }
    }

    /// Check if this trust level is sufficient for genesis ceremony
    ///
    /// Minimum trust level for genesis is Medium (⭐⭐⭐).
    pub fn is_sufficient_for_genesis(self) -> bool {
        self >= Self::Medium
    }

    /// Get star rating as string
    pub fn stars(self) -> &'static str {
        match self {
            Self::Low => "⭐",
            Self::Medium => "⭐⭐⭐",
            Self::High => "⭐⭐⭐⭐",
            Self::Maximum => "⭐⭐⭐⭐⭐",
        }
    }

    /// Check if this trust level meets or exceeds the threshold
    pub fn meets_threshold(self, threshold: TrustLevel) -> bool {
        self >= threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_physical_channel_trust_levels() {
        assert_eq!(
            PhysicalChannelType::HardwareKey.trust_level(),
            TrustLevel::Maximum
        );
        assert_eq!(PhysicalChannelType::Nfc.trust_level(), TrustLevel::High);
        assert_eq!(
            PhysicalChannelType::QrCodeWithOob.trust_level(),
            TrustLevel::High
        );
        assert_eq!(
            PhysicalChannelType::Bluetooth.trust_level(),
            TrustLevel::Medium
        );
    }

    #[test]
    fn test_trust_level_ordering() {
        assert!(TrustLevel::Maximum > TrustLevel::High);
        assert!(TrustLevel::High > TrustLevel::Medium);
    }

    #[test]
    fn test_trust_level_genesis_sufficiency() {
        assert!(TrustLevel::Maximum.is_sufficient_for_genesis());
        assert!(TrustLevel::High.is_sufficient_for_genesis());
        assert!(TrustLevel::Medium.is_sufficient_for_genesis());
    }

    #[test]
    fn test_physical_channel_oob_requirement() {
        assert!(!PhysicalChannelType::HardwareKey.requires_oob());
        assert!(PhysicalChannelType::QrCodeWithOob.requires_oob());
        assert!(!PhysicalChannelType::Nfc.requires_oob());
        assert!(!PhysicalChannelType::Bluetooth.requires_oob());
    }
}
