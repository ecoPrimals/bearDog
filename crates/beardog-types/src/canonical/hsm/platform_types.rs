// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Platform-Specific HSM Types - Canonical Definitions
///
/// This module consolidates platform-specific HSM types that were previously
/// duplicated across multiple crates, particularly in the tunnel crate.

use serde::{Deserialize, Serialize};

/// Canonical memory protection level - unified definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemoryProtectionLevel {
    /// No special memory protection
    None,
    /// Basic memory protection
    Low,
    /// Standard memory protection
    Medium,
    /// Enhanced memory protection with encryption
    High,
    /// Maximum security with hardware-backed protection
    Maximum,
}

impl Default for MemoryProtectionLevel {
    fn default() -> Self {
        Self::Medium
    }
}

/// Canonical HSM type - unified definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HsmType {
    /// Software-based HSM
    Software,
    /// Network-attached HSM
    Network,
    /// USB-connected HSM
    Usb,
    /// PCIe card HSM
    Pcie,
    /// Cloud-based HSM
    Cloud,
    /// Mobile secure element
    Mobile,
    /// TPM (Trusted Platform Module)
    Tpm,
}

/// Canonical smartphone type - unified definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SmartphoneType {
    /// Android device
    Android,
    /// iOS device
    Ios,
    /// Other smartphone OS
    Other(String),
}

/// Canonical secure enclave type - unified definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecureEnclaveType {
    /// Apple Secure Enclave
    AppleSecureEnclave,
    /// Android StrongBox
    AndroidStrongBox,
    /// Samsung Knox
    SamsungKnox,
    /// Qualcomm Secure Processing Unit
    QualcommSpu,
    /// Generic TEE
    TrustedExecutionEnvironment,
}

/// Canonical software HSM type - unified definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoftwareHsmType {
    /// SoftHSM implementation
    SoftHsm,
    /// OpenSSL engine
    OpenSsl,
    /// BearDog native implementation
    BearDogNative,
    /// Custom implementation
    Custom(String),
}

/// Canonical entropy quality rating - unified definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntropyQualityRating {
    /// No entropy quality (legacy)
    None,
    /// Insufficient entropy quality
    Insufficient,
    /// Low entropy quality
    Low,
    /// Basic entropy quality
    Basic,
    /// Medium entropy quality
    Medium,
    /// Good entropy quality
    Good,
    /// High entropy quality
    High,
    /// Excellent entropy quality
    Excellent,
    /// Premium entropy quality
    Premium,
}

impl Default for EntropyQualityRating {
    fn default() -> Self {
        Self::Basic
    }
}

/// Canonical entropy collection method - unified definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EntropyCollectionMethod {
    /// Touch patterns with pressure sensitivity
    TouchPressure,
    /// Typing rhythm analysis
    TypingRhythm,
    /// Mouse movement patterns
    MouseMovement,
    /// Accelerometer data
    Accelerometer,
    /// Camera sensor noise
    CameraNoise,
    /// Microphone ambient noise
    MicrophoneNoise,
    /// Hardware random number generator
    HardwareRng,
    /// System entropy pool
    SystemEntropy,
}

/// Key storage type classification
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyStorageType {
    /// Software keystore
    Software,
    /// Hardware security module
    Hardware,
    /// Trusted execution environment
    TrustedExecutionEnvironment,
    /// Secure element
    SecureElement,
    /// Cloud HSM
    CloudHsm,
}

/// Android key algorithm support
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AndroidKeyAlgorithm {
    /// RSA algorithm
    Rsa,
    /// Elliptic Curve algorithm
    Ec,
    /// AES symmetric key
    Aes,
    /// HMAC algorithm
    Hmac,
}

/// StrongBox implementation type
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StrongBoxImplementation {
    /// Qualcomm implementation
    Qualcomm,
    /// Samsung implementation
    Samsung,
    /// MediaTek implementation
    MediaTek,
    /// Generic implementation
    Generic,
}

/// Performance metrics for HSM operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Operations per second
    pub operations_per_second: f64,
    /// Average latency in milliseconds
    pub average_latency_ms: f64,
    /// Success rate percentage (0.0 to 100.0)
    pub success_rate: f64,
    /// Memory usage in MB
    pub memory_usage_mb: f64,
    /// CPU usage percentage (0.0 to 100.0)
    pub cpu_usage_percent: f64,
    /// Total number of errors
    pub error_count: u64,
    /// Uptime in seconds
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