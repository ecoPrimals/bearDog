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


/// # Canonical HSM Configuration
///
/// **SINGLE SOURCE OF TRUTH** for all HSM configuration-related types.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::time::Duration;

/// **HSM CONFIG** - Main HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmConfig {
    /// HSM provider type
    pub provider_type: HsmProviderType,
    /// Connection configuration
    pub connection: ConnectionConfig,
    /// Security configuration
    pub security: SecurityConfig,
    /// Performance tuning
    pub performance: PerformanceConfig,
    /// Custom provider-specific settings
    pub provider_settings: HashMap<String, serde_json::Value>,
}
/// **HSM PROVIDER TYPE** - Types of HSM providers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HsmProviderType {
    /// Software-based HSM
    Software,
    /// Hardware Security Module
    Hardware,
    /// PKCS#11 provider
    Pkcs11,
    /// Cloud HSM service
    Cloud,
    /// Android StrongBox
    AndroidStrongBox,
    /// iOS Secure Enclave
    IosSecureEnclave,
    /// Network HSM
    NetworkHsm,
    /// USB Token
    UsbToken,
    /// Smart Card
    SmartCard,
    /// TPM (Trusted Platform Module)
    Tpm,
    /// Custom provider
    Custom(String),}


impl fmt::Display for HsmProviderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Software => write!(f, "Software"),
            Self::Hardware => write!(f, "Hardware"),
            Self::Pkcs11 => write!(f, "PKCS#11"),
            Self::Cloud => write!(f, "Cloud"),
            Self::AndroidStrongBox => write!(f, "Android StrongBox"),
            Self::IosSecureEnclave => write!(f, "iOS Secure Enclave"),
            Self::NetworkHsm => write!(f, "Network HSM"),
            Self::UsbToken => write!(f, "USB Token"),
            Self::SmartCard => write!(f, "Smart Card"),
            Self::Tpm => write!(f, "TPM"),
            Self::Custom(name) => write!(f, "Custom({name})"),
        }
    }
}
/// **CONNECTION CONFIG** - HSM connection settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    /// Connection timeout in milliseconds
    pub timeout_ms: u32,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Connection pool size
    pub pool_size: u32,
    /// Keep-alive interval
    pub keep_alive_ms: u32,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 30_000,
            max_retries: 3,
            pool_size: 10,
            keep_alive_ms: 60_000,
        }
    }
}

/// **SECURITY CONFIG** - HSM security settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Authentication required
    pub auth_required: bool,
    /// Encryption enabled
    pub encryption_enabled: bool,
    /// Key validation level
    pub key_validation: String,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            auth_required: true,
            encryption_enabled: true,
            key_validation: "strict".to_string(),
        }
    }
}

/// **PERFORMANCE CONFIG** - HSM performance settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Operation timeout
    pub operation_timeout: Duration,
    /// Batch size for bulk operations
    pub batch_size: u32,
    /// Cache size
    pub cache_size: Option<u32>,
    /// Parallel operation limit
    pub max_parallel_ops: u32,
}
/// **AUTH METHOD** - Authentication methods
pub enum AuthMethod {
    /// No authentication required
    None,
    /// Password-based authentication
    Password(String),
    /// Certificate-based authentication
    Certificate { cert_path: String, key_path: String },
    /// Token-based authentication
    Token(String),
    /// Smart card authentication
    SmartCard { slot_id: u32, pin: Option<String> },
}
/// **SOFTWARE HSM CONFIG** - Software HSM specific configuration
pub struct SoftwareHsmConfig {
    /// Base HSM configuration
    pub base: HsmConfig,
    /// Storage path for keys
    pub storage_path: String,
    /// Encryption key for storage
    pub storage_encryption: bool,
    /// Memory protection
    pub memory_protection: bool,
}
/// **HSM TIER CONFIG** - HSM security tier configuration
pub struct HsmTierConfig {
    /// Security tier level
    pub tier: HsmSecurityTier,
    /// Tamper resistance requirements
    pub tamper_resistance: TamperResistanceLevel,
    /// Attestation requirements
    pub attestation: AttestationLevel,
}
/// **HSM SECURITY TIER** - Security tier levels (unified from fragmented definitions)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum HsmSecurityTier {
    /// Software-only security (lowest)
    /// Hardware-backed security
    /// Common Criteria certified
    CommonCriteria,
    /// FIPS 140-2 Level 2
    Fips140Level2,
    /// FIPS 140-2 Level 3
    Fips140Level3,
    /// FIPS 140-2 Level 4 (highest)
    Fips140Level4,
}
/// **TAMPER RESISTANCE LEVEL** - Levels of tamper resistance
pub enum TamperResistanceLevel {
    /// No tamper resistance
    /// Basic tamper detection
    Basic,
    /// Tamper-evident design
    Evident,
    /// Tamper-resistant design
    Resistant,
    /// Tamper-proof design
    Proof,
}
/// **ATTESTATION LEVEL** - Levels of attestation support
pub enum AttestationLevel {
    /// No attestation support
    /// Basic attestation
    /// Enhanced attestation
    Enhanced,
    /// Remote attestation
    Remote,}


impl Default for HsmConfig {
    fn default() -> Self {
        Self {
            provider_type: HsmProviderType::Software,
            connection: ConnectionConfig::default(),
            security: SecurityConfig::default(),
            performance: PerformanceConfig::default(),
            provider_settings: HashMap::new(),
        }
    }
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            operation_timeout: Duration::from_secs(10),
            batch_size: 100,
            cache_size: Some(1000),
            max_parallel_ops: 4,
        }
    }
}

impl Default for SoftwareHsmConfig {
    fn default() -> Self {
        Self {
            base: HsmConfig::default(),
            storage_path: "/tmp/beardog_hsm".to_string(),
            storage_encryption: true,
            memory_protection: true,
        }
    }
}

/// Attestation configuration for HSM operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationConfig {
    /// Whether attestation is required
    pub require_hardware_attestation: bool,
    /// Accepted attestation levels
    pub accepted_attestation_levels: Vec<String>,
    /// Attestation timeout
    pub attestation_timeout: std::time::Duration,
    // === TUNNEL COMPATIBILITY FIELDS ===
    /// Whether attestation is enabled (compatibility)
    pub enabled: bool,
    /// Challenge timeout in seconds (compatibility)
    pub challenge_timeout_secs: u64,
    /// Whether to cache attestation results (compatibility)
    pub cache_results: bool,
    /// Cache time-to-live in seconds (compatibility)
    pub cache_ttl_secs: u64,
}
