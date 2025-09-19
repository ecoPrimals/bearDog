// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};

/// `MemoryProtectionLevel`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryProtectionLevel {
    /// None variant
    /// None
    None,

    /// Low variant
    /// Low
    Low,

    /// Medium variant
    /// Medium
    Medium,

    /// High variant
    /// High
    High,

    /// Maximum variant
    /// Maximum
    Maximum,
}

impl Default for MemoryProtectionLevel {
    fn default() -> Self {
        Self::Medium
    }
}

/// `HsmType`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of hsm
pub enum HsmType {
    /// Software variant
    /// Software
    Software,

    /// Network variant
    /// Network
    Network,

    /// Usb variant
    /// Usb
    Usb,

    /// Pcie variant
    /// Pcie
    Pcie,

    /// Cloud variant
    /// Cloud
    Cloud,

    /// Mobile variant
    /// Mobile
    Mobile,

    /// Tpm variant
    /// Tpm
    Tpm,
}

/// `SmartphoneType`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of smartphone
pub enum SmartphoneType {
    /// Android variant
    /// Android
    Android,

    /// Ios variant
    /// Ios
    Ios,

    Other(String),
}

/// `SecureEnclaveType`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of secure enclave
pub enum SecureEnclaveType {
    /// `AppleSecureEnclave` variant
    /// `AppleSecureEnclave`
    AppleSecureEnclave,

    /// `AndroidStrongBox` variant
    /// `AndroidStrongBox`
    AndroidStrongBox,

    /// `SamsungKnox` variant
    /// `SamsungKnox`
    SamsungKnox,

    /// `QualcommSpu` variant
    /// `QualcommSpu`
    QualcommSpu,

    /// `TrustedExecutionEnvironment` variant
    /// `TrustedExecutionEnvironment`
    TrustedExecutionEnvironment,
}

/// `SoftwareHsmType`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of software hsm
pub enum SoftwareHsmType {
    /// `SoftHsm` variant
    /// `SoftHsm`
    SoftHsm,

    /// `OpenSsl` variant
    /// `OpenSsl`
    OpenSsl,

    /// `BearDogNative` variant
    /// `BearDogNative`
    BearDogNative,

    /// Custom secure enclave type with identifier
    Custom(String),
}

/// `EntropyQualityRating`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntropyQualityRating {
    /// None variant
    /// None
    None,

    /// Insufficient variant
    /// Insufficient
    Insufficient,

    /// Low variant
    /// Low
    Low,

    /// Basic variant
    /// Basic
    Basic,

    /// Medium variant
    /// Medium
    Medium,

    /// Good variant
    /// Good
    Good,

    /// High variant
    /// High
    High,

    /// Excellent variant
    /// Excellent
    Excellent,

    /// Premium variant
    /// Premium
    Premium,
}

impl Default for EntropyQualityRating {
    fn default() -> Self {
        Self::Basic
    }
}

/// `EntropyCollectionMethod`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntropyCollectionMethod {
    /// `TouchPressure` variant
    /// `TouchPressure`
    TouchPressure,

    /// `TypingRhythm` variant
    /// `TypingRhythm`
    TypingRhythm,

    /// `MouseMovement` variant
    /// `MouseMovement`
    MouseMovement,

    /// Accelerometer variant
    /// Accelerometer
    Accelerometer,

    /// `CameraNoise` variant
    /// `CameraNoise`
    CameraNoise,

    /// `MicrophoneNoise` variant
    /// `MicrophoneNoise`
    MicrophoneNoise,

    /// `HardwareRng` variant
    /// `HardwareRng`
    HardwareRng,

    /// `SystemEntropy` variant
    /// `SystemEntropy`
    SystemEntropy,
}

/// `KeyStorageType`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
/// Types of key storage
pub enum KeyStorageType {
    /// Software variant
    /// Software
    Software,

    /// Hardware variant
    /// Hardware
    Hardware,

    /// `TrustedExecutionEnvironment` variant
    /// `TrustedExecutionEnvironment`
    TrustedExecutionEnvironment,

    /// `SecureElement` variant
    /// `SecureElement`
    SecureElement,

    /// `CloudHsm` variant
    /// `CloudHsm`
    CloudHsm,
}

/// `AndroidKeyAlgorithm`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AndroidKeyAlgorithm {
    /// Rsa variant
    /// Rsa
    Rsa,

    /// Ec variant
    /// Ec
    Ec,

    /// Aes variant
    /// Aes
    Aes,

    /// Hmac variant
    /// Hmac
    Hmac,
}

/// `StrongBoxImplementation`
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrongBoxImplementation {
    /// Qualcomm variant
    /// Qualcomm
    Qualcomm,

    /// Samsung variant
    /// Samsung
    Samsung,

    /// `MediaTek` variant
    /// `MediaTek`
    MediaTek,

    /// Generic variant
    /// Generic
    Generic,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PerformanceMetrics {
    /// Operations Per Second
    /// The operations per second value
    pub operations_per_second: f64,
    /// Average Latency Ms
    /// The average latency ms value
    pub average_latency_ms: f64,
    /// Success Rate
    /// The success rate value
    pub success_rate: f64,
    /// Memory Usage Mb
    /// The memory usage mb value
    pub memory_usage_mb: f64,
    /// Cpu Usage Percent
    /// The cpu usage percent value
    pub cpu_usage_percent: f64,
    /// Error Count
    /// Number of error
    pub error_count: u64,
    /// Uptime Seconds
    pub uptime_seconds: u64,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            operations_per_second: 0.0,
            average_latency_ms: 0.0,
            success_rate: 100.0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            error_count: 0,
            uptime_seconds: 0,
        }
    }
}
