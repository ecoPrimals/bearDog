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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub operations_per_second: f64,

    pub average_latency_ms: f64,

    pub success_rate: f64,

    pub memory_usage_mb: f64,

    pub cpu_usage_percent: f64,

    pub error_count: u64,

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
