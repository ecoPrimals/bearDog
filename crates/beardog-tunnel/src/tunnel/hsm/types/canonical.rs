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


/// Canonical HSM Types - Unified Type System
///
/// This module provides the canonical, modernized HSM types that replace
/// the fragmented type definitions across the tunnel crate.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
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
impl Default for MemoryProtectionLevel {}


    fn default() -> Self {
        Self::Medium
    }
/// Canonical performance metrics - unified definition
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
    pub uptime_seconds: u64,}


impl Default for PerformanceMetrics {
        Self {
            operations_per_second: 0.0,
            average_latency_ms: 0.0,
            success_rate: 100.0,
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            error_count: 0,
            uptime_seconds: 0,
        }
/// Canonical attestation level - unified definition
pub enum AttestationLevel {
    /// No attestation
    /// Basic software attestation
    Software,
    /// Hardware-backed attestation
    Hardware,
    /// Verified boot attestation
    VerifiedBoot,
    /// Strong hardware attestation
    StrongBox,
/// Canonical HSM type - unified definition}


pub enum HsmType {
    /// Software-based HSM
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
/// Canonical smartphone type - unified definition
pub enum SmartphoneType {
    /// Android device
    Android,
    /// iOS device
    Ios,
    /// Other smartphone OS
    Other(String),
/// Canonical secure enclave type - unified definition}


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
/// Canonical software HSM type - unified definition
pub enum SoftwareHsmType {
    /// SoftHSM implementation
    SoftHsm,
    /// OpenSSL engine
    OpenSsl,
    /// BearDog native implementation
    BearDogNative,
    /// Custom implementation
    Custom(String),
/// Canonical entropy quality rating - unified definition}


pub enum EntropyQualityRating {
    /// No entropy quality (legacy)
    /// Insufficient entropy quality
    Insufficient,
    /// Low entropy quality
    /// Basic entropy quality
    Basic,
    /// Medium entropy quality
    /// Good entropy quality
    Good,
    /// High entropy quality
    /// Excellent entropy quality
    Excellent,
    /// Premium entropy quality
    Premium,}


impl Default for EntropyQualityRating {
        Self::Basic
/// Canonical entropy collection method - unified definition
pub enum EntropyCollectionMethod {
    /// Touch patterns with pressure sensitivity
    TouchPatterns { pressure_sensitive: bool },
    /// Advanced touch patterns with multiple sensors
    TouchPatternsAdvanced {
        pressure_sensitive: bool,
        multi_touch: bool,
        gesture_recognition: bool,
    },
    /// Device motion sensors
    DeviceMotion,
    /// Environmental sensors
    EnvironmentalSensors { sensor_types: Vec<String> },
    /// Biometric data
    Biometric,
    /// Behavioral patterns
    Behavioral,
/// Canonical HSM interface type - unified definition
pub enum HsmInterfaceType {
    /// PKCS#11 interface
    Pkcs11 { library_path: String },
    /// Network HSM interface
    NetworkHsm { endpoint: String, protocol: String },
    /// USB HSM interface
    UsbHsm { device_path: String },
    /// Smart card interface
    SmartCard { reader_name: String },
    /// TPM interface
    Tpm { version: String },
    /// AWS KMS interface
    AwsKms { region: String },
    /// Azure Key Vault interface
    AzureKeyVault { vault_url: String },
    /// Google Cloud KMS interface
    GcpKms {
        project_id: String,
        location: String,
    /// Android StrongBox interface
    AndroidStrongBox { security_level: u8 },
    /// iOS Secure Enclave interface
    IosSecureEnclave { enclave_version: String },
    /// SoftHSM interface
    SoftHsm { config_path: String },
    /// OpenSSL engine interface
    OpenSsl { engine_path: String },
    /// BearDog native interface
    BearDogNative { instance_id: String },
    /// Windows CNG interface
    WindowsCng { provider_name: String },
    /// macOS Keychain interface
    MacOsKeychain { keychain_path: String },
    /// Custom API interface
    CustomApi {
        api_endpoint: String,
        auth_method: String,
    /// Proprietary driver interface
    ProprietaryDriver { driver_path: String, config: String },
/// Canonical HSM connection info - unified definition
pub struct HsmConnectionInfo {
    /// Connection timeout in seconds
    pub timeout: u64,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Connection pool size
    pub pool_size: u32,
    /// Additional connection parameters
    pub parameters: HashMap<String, String>,}


impl Default for HsmConnectionInfo {
            timeout: 30,
            max_retries: 3,
            pool_size: 10,
            parameters: HashMap::new(),
/// Canonical discovered HSM - unified definition}


pub struct DiscoveredHsm {
    /// HSM name/identifier
    pub name: String,
    /// HSM type
    pub hsm_type: HsmType,
    /// Connection endpoint
    pub endpoint: String,
    /// Supports human entropy
    pub supports_human_entropy: bool,
    /// Discovery timestamp
    pub discovered_at: DateTime<Utc>,
    /// Vendor information
    pub vendor: Option<String>,
    /// Model information
    pub model: Option<String>,
    /// Version information
    pub version: Option<String>,
    /// Interface type
    pub interface_type: Option<HsmInterfaceType>,
    /// Connection information
    pub connection_info: Option<HsmConnectionInfo>,
/// Canonical discovery config - unified definition
pub struct DiscoveryConfig {
    /// Enable cloud HSM discovery
    pub enable_cloud_discovery: bool,
    /// Enable PKCS#11 discovery
    pub enable_pkcs11_discovery: bool,
    /// Enable smartphone HSM discovery
    pub enable_smartphone_discovery: bool,
    /// Discovery timeout in seconds
    pub discovery_timeout_seconds: u64,
    /// Enable capability detection
    pub enable_capability_detection: bool,
    /// Auto discovery enabled (legacy field)
    pub auto_discovery_enabled: bool,
    /// General timeout (legacy field)
    pub timeout: Option<u64>,}


impl Default for DiscoveryConfig {
            enable_cloud_discovery: true,
            enable_pkcs11_discovery: true,
            enable_smartphone_discovery: true,
            discovery_timeout_seconds: 300,
            enable_capability_detection: true,
            auto_discovery_enabled: true,
            timeout: Some(300),
/// Canonical key metadata - unified definition}


impl Default for KeyMetadata {
            key_id: String::new(),
            key_name: None,
            key_type: None,
            algorithm: None,
            key_size: None,
            created_at: Some(Utc::now()),
            creation_time: Some(Utc::now()),
            expires_at: None,
            last_used: None,
            usage_count: Some(0),
            is_exportable: Some(false),
            is_hardware_backed: Some(false),
            tags: HashMap::new(),
// Legacy type aliases removed - use canonical types directly
/// Re-export canonical types for easy migration
pub use self::{
    AttestationLevel as CanonicalAttestationLevel, DiscoveredHsm as CanonicalDiscoveredHsm,
    DiscoveryConfig as CanonicalDiscoveryConfig,
    EntropyCollectionMethod as CanonicalEntropyCollectionMethod,
    EntropyQualityRating as CanonicalEntropyQualityRating,
    HsmConnectionInfo as CanonicalHsmConnectionInfo, HsmInterfaceType as CanonicalHsmInterfaceType,
    HsmType as CanonicalHsmType, KeyMetadata as CanonicalKeyMetadata,
    MemoryProtectionLevel as CanonicalMemoryProtectionLevel,
    PerformanceMetrics as CanonicalPerformanceMetrics,
    SecureEnclaveType as CanonicalSecureEnclaveType, SmartphoneType as CanonicalSmartphoneType,
    SoftwareHsmType as CanonicalSoftwareHsmType,
};
