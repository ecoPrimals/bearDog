//! HSM Tier Types
//!
//! Type definitions for HSM tiers and platform-specific implementations.
//!
//! MODERNIZATION NOTE: This file contains vendor-specific references that should be migrated
//! to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
//! Target: Replace with capability-based discovery for vendor/primal agnosticism

use serde::{Deserialize, Serialize};
use std::fmt;

/// Smartphone type variants
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SmartphoneType {
    /// iOS device
    Ios {
        /// iOS version
        ios_version: String,
        /// Secure Enclave version
        secure_enclave_version: String,
    },
    /// Android device
    Android {
        /// Device manufacturer
        manufacturer: String,
        /// Android version
        android_version: String,
        /// StrongBox version (if available)
        strongbox_version: Option<String>,
    },
}

/// Secure enclave types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecureEnclaveType {
    /// iOS Secure Enclave
    IosSecureEnclave {
        /// Chip type (A-series, M-series)
        chip_type: String,
        /// Biometric support available
        biometric_support: bool,
        /// Key attestation support
        key_attestation: bool,
    },
    /// Android StrongBox
    AndroidStrongBox {
        /// StrongBox implementation type
        implementation: StrongBoxImplementation,
        /// Hardware-backed security
        hardware_backed: bool,
        /// Attestation level
        attestation_level: AttestationLevel,
    },
    /// Samsung Knox
    SamsungKnox {
        /// Knox version
        knox_version: String,
        /// FIPS certification
        fips_certified: bool,
    },
    /// Qualcomm SPU
    QualcommSpu {
        /// SPU version
        spu_version: String,
        /// Trusted execution environment
        tee_enabled: bool,
    },
    /// Generic Trusted Execution Environment
    TrustedExecutionEnvironment {
        /// TEE type
        tee_type: String,
        /// Security level
        security_level: u8,
    },
}

/// StrongBox implementation types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrongBoxImplementation {
    /// Qualcomm implementation
    Qualcomm,
    /// MediaTek implementation
    MediaTek,
    /// Samsung implementation
    Samsung,
    /// Generic implementation
    Generic,
}

/// Attestation levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttestationLevel {
    /// Software-only attestation
    Software,
    /// Trusted Execution Environment attestation
    TrustedExecutionEnvironment,
    /// Hardware attestation
    Hardware,
    /// StrongBox attestation (highest level)
    StrongBox,
}

/// Software HSM types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoftwareHsmType {
    /// SoftHSM v2
    SoftHsm,
    /// OpenSSL-based implementation
    OpenSsl,
    /// Rust crypto implementation
    RustCrypto,
    /// BearDog native implementation
    BearDogNative,
    /// Custom software HSM
    Custom(String),
}

/// Key storage types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyStorageType {
    /// Hardware-backed storage
    Hardware,
    /// Software-encrypted storage
    Encrypted,
    /// In-memory storage
    Memory,
    /// File-based storage
    File,
    /// Database storage
    Database,
}

/// Memory protection levels
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryProtectionLevel {
    /// No protection
    None,
    /// Basic protection
    Low,
    /// Standard protection
    Medium,
    /// Enhanced protection
    High,
    /// Maximum protection
    Maximum,
}

/// Android key algorithms
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AndroidKeyAlgorithm {
    /// RSA encryption
    Rsa,
    /// Elliptic Curve Cryptography
    Ec,
    /// AES encryption
    Aes,
    /// HMAC
    Hmac,
}

/// HSM tier enumeration
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum HsmTier {
    /// Software-only (Tier 0)
    Software,
    /// Basic mobile security (Tier 1)
    Mobile,
    /// Enhanced mobile security (Tier 2)
    SecureEnclave,
    /// Hardware security module (Tier 3)
    Hardware,
    /// Cloud HSM (Tier 2-3)
    Cloud,
}

impl HsmTier {
    /// Get the security level of this tier (0-3)
    pub fn security_level(&self) -> u8 {
        match self {
            Self::Software => 0,
            Self::Mobile => 1,
            Self::SecureEnclave => 2,
            Self::Hardware => 3,
            Self::Cloud => 2,
        }
    }

    /// Check if this tier meets minimum security requirements
    pub fn meets_requirement(&self, required: &HsmTier) -> bool {
        self.security_level() >= required.security_level()
    }
}

impl fmt::Display for HsmTier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Software => write!(f, "Software HSM (Tier 0)"),
            Self::Mobile => write!(f, "Mobile HSM (Tier 1)"),
            Self::SecureEnclave => write!(f, "Secure Enclave (Tier 2)"),
            Self::Hardware => write!(f, "Hardware HSM (Tier 3)"),
            Self::Cloud => write!(f, "Cloud HSM (Tier 2-3)"),
        }
    }
}

impl fmt::Display for AttestationLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Software => write!(f, "Software"),
            Self::TrustedExecutionEnvironment => write!(f, "TEE"),
            Self::Hardware => write!(f, "Hardware"),
            Self::StrongBox => write!(f, "StrongBox"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hsm_tier_security_levels() {
        assert_eq!(HsmTier::Software.security_level(), 0);
        assert_eq!(HsmTier::Mobile.security_level(), 1);
        assert_eq!(HsmTier::SecureEnclave.security_level(), 2);
        assert_eq!(HsmTier::Hardware.security_level(), 3);
        assert_eq!(HsmTier::Cloud.security_level(), 2);
    }

    #[test]
    fn test_hsm_tier_meets_requirement() {
        let software = HsmTier::Software;
        let mobile = HsmTier::Mobile;
        let hardware = HsmTier::Hardware;

        assert!(hardware.meets_requirement(&software));
        assert!(hardware.meets_requirement(&mobile));
        assert!(hardware.meets_requirement(&hardware));

        assert!(!software.meets_requirement(&hardware));
        assert!(!mobile.meets_requirement(&hardware));
    }

    #[test]
    fn test_hsm_tier_ordering() {
        assert!(HsmTier::Software < HsmTier::Mobile);
        assert!(HsmTier::Mobile < HsmTier::SecureEnclave);
        assert!(HsmTier::SecureEnclave < HsmTier::Hardware);
    }

    #[test]
    fn test_hsm_tier_display() {
        assert_eq!(HsmTier::Software.to_string(), "Software HSM (Tier 0)");
        assert_eq!(HsmTier::Hardware.to_string(), "Hardware HSM (Tier 3)");
    }

    #[test]
    fn test_smartphone_type_variants() {
        let ios = SmartphoneType::Ios {
            ios_version: "17.0".to_string(),
            secure_enclave_version: "A17".to_string(),
        };

        let android = SmartphoneType::Android {
            manufacturer: "Google".to_string(),
            android_version: "14".to_string(),
            strongbox_version: Some("1.0".to_string()),
        };

        match ios {
            SmartphoneType::Ios { ios_version, .. } => {
                assert_eq!(ios_version, "17.0");
            }
            _ => panic!("Expected iOS variant"),
        }

        match android {
            SmartphoneType::Android { manufacturer, .. } => {
                assert_eq!(manufacturer, "Google");
            }
            _ => panic!("Expected Android variant"),
        }
    }

    #[test]
    fn test_secure_enclave_type_variants() {
        let ios_enclave = SecureEnclaveType::IosSecureEnclave {
            chip_type: "A17 Pro".to_string(),
            biometric_support: true,
            key_attestation: true,
        };

        match ios_enclave {
            SecureEnclaveType::IosSecureEnclave {
                biometric_support, ..
            } => {
                assert!(biometric_support);
            }
            _ => panic!("Expected iOS Secure Enclave variant"),
        }
    }

    #[test]
    fn test_attestation_level_display() {
        assert_eq!(AttestationLevel::Software.to_string(), "Software");
        assert_eq!(AttestationLevel::StrongBox.to_string(), "StrongBox");
    }

    #[test]
    fn test_strongbox_implementation_variants() {
        let implementations = [
            StrongBoxImplementation::Qualcomm,
            StrongBoxImplementation::MediaTek,
            StrongBoxImplementation::Samsung,
            StrongBoxImplementation::Generic,
        ];
        assert_eq!(implementations.len(), 4);
    }

    #[test]
    fn test_software_hsm_type_custom() {
        let custom = SoftwareHsmType::Custom("MyCustomHSM".to_string());
        match custom {
            SoftwareHsmType::Custom(name) => assert_eq!(name, "MyCustomHSM"),
            _ => panic!("Expected Custom variant"),
        }
    }

    #[test]
    fn test_key_storage_types() {
        let types = [
            KeyStorageType::Hardware,
            KeyStorageType::Encrypted,
            KeyStorageType::Memory,
            KeyStorageType::File,
            KeyStorageType::Database,
        ];
        assert_eq!(types.len(), 5);
    }

    #[test]
    fn test_memory_protection_levels() {
        let levels = vec![
            MemoryProtectionLevel::None,
            MemoryProtectionLevel::Low,
            MemoryProtectionLevel::Medium,
            MemoryProtectionLevel::High,
            MemoryProtectionLevel::Maximum,
        ];
        assert_eq!(levels.len(), 5);
    }
}
