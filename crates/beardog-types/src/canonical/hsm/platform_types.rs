use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryProtectionLevel {
    None,

    Low,

    Medium,

    High,

    Maximum,
}

impl Default for MemoryProtectionLevel {
    fn default() -> Self {
        Self::Medium
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HsmType {
    Software,

    Network,

    Usb,

    Pcie,

    Cloud,

    Mobile,

    Tpm,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmartphoneType {
    Android,

    Ios,

    Other(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecureEnclaveType {
    AppleSecureEnclave,

    AndroidStrongBox,

    SamsungKnox,

    QualcommSpu,

    TrustedExecutionEnvironment,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoftwareHsmType {
    SoftHsm,

    OpenSsl,

    BearDogNative,

    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntropyQualityRating {
    None,

    Insufficient,

    Low,

    Basic,

    Medium,

    Good,

    High,

    Excellent,

    Premium,
}

impl Default for EntropyQualityRating {
    fn default() -> Self {
        Self::Basic
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntropyCollectionMethod {
    TouchPressure,

    TypingRhythm,

    MouseMovement,

    Accelerometer,

    CameraNoise,

    MicrophoneNoise,

    HardwareRng,

    SystemEntropy,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyStorageType {
    Software,

    Hardware,

    TrustedExecutionEnvironment,

    SecureElement,

    CloudHsm,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AndroidKeyAlgorithm {
    Rsa,

    Ec,

    Aes,

    Hmac,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrongBoxImplementation {
    Qualcomm,

    Samsung,

    MediaTek,

    Generic,
}

// PerformanceMetrics consolidated to canonical metrics module
pub use crate::metrics::PerformanceMetrics;
