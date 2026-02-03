//! Canonical Security Level Definitions
//!
//! Unified security level enumeration for all HSM operations across BearDog.
//! This consolidates multiple fragmented definitions into a single source of truth.

use serde::{Deserialize, Serialize};

/// Canonical security level enumeration
///
/// Represents the hardware security backing for cryptographic operations,
/// ordered from lowest to highest security.
///
/// # Security Hierarchy
///
/// ```text
/// Software < TEE < Secure Enclave < HSM < StrongBox
///     0       1          2            3       4
/// ```
///
/// # Platform Mapping
///
/// - **Software**: No hardware security, keys in memory
/// - **TrustedExecutionEnvironment**: ARM TrustZone, Intel SGX
/// - **SecureEnclave**: iOS Secure Enclave, isolated processor
/// - **HardwareSecurityModule**: Dedicated security chip
/// - **StrongBox**: Android StrongBox (Titan M, Qualcomm SPU)
///
/// # Examples
///
/// ```
/// use beardog_tunnel::hsm::types::SecurityLevel;
///
/// let level = SecurityLevel::StrongBox;
/// assert!(level > SecurityLevel::TrustedExecutionEnvironment);
/// assert_eq!(level.security_bits(), 256);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum SecurityLevel {
    /// Software implementation - no hardware security
    ///
    /// Keys stored in process memory, protected only by OS isolation.
    /// Suitable for development and testing only.
    Software = 0,

    /// Trusted Execution Environment (TEE)
    ///
    /// Hardware-isolated execution environment (ARM TrustZone, Intel SGX).
    /// Keys protected from normal OS but accessible to TEE.
    TrustedExecutionEnvironment = 1,

    /// Secure Enclave
    ///
    /// Dedicated security processor isolated from main CPU.
    /// Keys never leave the secure enclave.
    /// Example: Apple Secure Enclave
    SecureEnclave = 2,

    /// Hardware Security Module (HSM)
    ///
    /// Dedicated tamper-resistant security chip.
    /// Keys stored in hardware, never exported.
    /// Example: TPM, YubiKey HSM
    HardwareSecurityModule = 3,

    /// Android StrongBox
    ///
    /// Android's highest security level using dedicated security chip.
    /// Stronger isolation and attestation than standard HSM.
    /// Examples: Google Titan M, Qualcomm SPU
    StrongBox = 4,
}

impl SecurityLevel {
    /// Returns the minimum recommended security bits for this level
    pub const fn security_bits(&self) -> usize {
        match self {
            Self::Software => 128,                     // Minimum acceptable
            Self::TrustedExecutionEnvironment => 192,  // TEE-protected
            Self::SecureEnclave => 256,                // Hardware isolated
            Self::HardwareSecurityModule => 256,       // Dedicated HSM
            Self::StrongBox => 256,                    // Maximum security
        }
    }

    /// Returns whether this level provides hardware backing
    pub const fn is_hardware_backed(&self) -> bool {
        !matches!(self, Self::Software)
    }

    /// Returns whether this level supports key attestation
    pub const fn supports_attestation(&self) -> bool {
        matches!(
            self,
            Self::SecureEnclave | Self::HardwareSecurityModule | Self::StrongBox
        )
    }

    /// Returns whether keys can be extracted from this security level
    pub const fn allows_key_export(&self) -> bool {
        matches!(self, Self::Software)
    }

    /// Returns a human-readable description
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Software => "Software (no hardware security)",
            Self::TrustedExecutionEnvironment => "TEE (ARM TrustZone, Intel SGX)",
            Self::SecureEnclave => "Secure Enclave (isolated processor)",
            Self::HardwareSecurityModule => "HSM (dedicated security chip)",
            Self::StrongBox => "StrongBox (Titan M, Qualcomm SPU)",
        }
    }

    /// Returns the platform identifier
    pub const fn platform_name(&self) -> &'static str {
        match self {
            Self::Software => "software",
            Self::TrustedExecutionEnvironment => "tee",
            Self::SecureEnclave => "secure_enclave",
            Self::HardwareSecurityModule => "hsm",
            Self::StrongBox => "strongbox",
        }
    }
}

impl Default for SecurityLevel {
    fn default() -> Self {
        Self::Software
    }
}

impl std::fmt::Display for SecurityLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_level_ordering() {
        assert!(SecurityLevel::StrongBox > SecurityLevel::HardwareSecurityModule);
        assert!(SecurityLevel::HardwareSecurityModule > SecurityLevel::SecureEnclave);
        assert!(SecurityLevel::SecureEnclave > SecurityLevel::TrustedExecutionEnvironment);
        assert!(SecurityLevel::TrustedExecutionEnvironment > SecurityLevel::Software);
    }

    #[test]
    fn test_security_bits() {
        assert_eq!(SecurityLevel::Software.security_bits(), 128);
        assert_eq!(SecurityLevel::StrongBox.security_bits(), 256);
    }

    #[test]
    fn test_hardware_backed() {
        assert!(!SecurityLevel::Software.is_hardware_backed());
        assert!(SecurityLevel::TrustedExecutionEnvironment.is_hardware_backed());
        assert!(SecurityLevel::StrongBox.is_hardware_backed());
    }

    #[test]
    fn test_attestation_support() {
        assert!(!SecurityLevel::Software.supports_attestation());
        assert!(!SecurityLevel::TrustedExecutionEnvironment.supports_attestation());
        assert!(SecurityLevel::SecureEnclave.supports_attestation());
        assert!(SecurityLevel::StrongBox.supports_attestation());
    }

    #[test]
    fn test_key_export() {
        assert!(SecurityLevel::Software.allows_key_export());
        assert!(!SecurityLevel::TrustedExecutionEnvironment.allows_key_export());
        assert!(!SecurityLevel::StrongBox.allows_key_export());
    }
}
