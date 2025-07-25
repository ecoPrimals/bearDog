//! HSM Capability Types
//!
//! This module defines types for describing HSM capabilities and requirements.

use serde::{Deserialize, Serialize};

/// HSM capability requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirements {
    /// Minimum security level required
    pub min_security_level: String,
    /// Whether hardware backing is required
    pub hardware_required: bool,
    /// Whether attestation is required
    pub attestation_required: bool,
}

impl Default for CapabilityRequirements {
    fn default() -> Self {
        Self {
            min_security_level: "software".to_string(),
            hardware_required: false,
            attestation_required: false,
        }
    }
}

/// Android HSM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidHsmConfig {
    /// Whether to use StrongBox if available
    pub prefer_strongbox: bool,
    /// Keystore alias prefix
    pub keystore_alias_prefix: String,
    /// Whether to require hardware backing
    pub require_hardware_backing: bool,
}

impl Default for AndroidHsmConfig {
    fn default() -> Self {
        Self {
            prefer_strongbox: true,
            keystore_alias_prefix: "beardog_".to_string(),
            require_hardware_backing: false,
        }
    }
}

/// Device model information  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceModel {
    /// Device manufacturer
    pub manufacturer: String,
    /// Device model name
    pub model: String,
    /// Operating system version
    pub os_version: String,
}

/// Memory protection level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryProtectionLevel {
    /// No special memory protection
    None,
    /// Basic memory protection
    Basic,
    /// Hardware-enforced memory protection
    Hardware,
}

impl Default for MemoryProtectionLevel {
    fn default() -> Self {
        MemoryProtectionLevel::Basic
    }
}

/// StrongBox capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrongBoxCapabilities {
    /// Whether hardware backing is available
    pub hardware_backed: bool,
    /// Supported key algorithms
    pub supported_algorithms: Vec<String>,
    /// Maximum key size supported
    pub max_key_size: u32,
}

impl Default for StrongBoxCapabilities {
    fn default() -> Self {
        Self {
            hardware_backed: false,
            supported_algorithms: vec!["Ed25519".to_string(), "P256".to_string()],
            max_key_size: 4096,
        }
    }
}

/// Key generation capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyGenerationCapabilities {
    /// Supported key types
    pub supported_key_types: Vec<String>,
    /// Maximum key sizes
    pub max_key_sizes: std::collections::HashMap<String, u32>,
    /// Hardware-backed generation
    pub hardware_backed: bool,
    /// True random number generation
    pub true_rng: bool,
}

impl Default for KeyGenerationCapabilities {
    fn default() -> Self {
        Self {
            supported_key_types: vec!["Ed25519".to_string(), "P256".to_string()],
            max_key_sizes: [
                ("Ed25519".to_string(), 256),
                ("P256".to_string(), 256),
                ("RSA".to_string(), 4096),
            ].into_iter().collect(),
            hardware_backed: false,
            true_rng: true,
        }
    }
}

/// Comprehensive HSM capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmCapabilities {
    /// Key generation capabilities
    pub key_generation: KeyGenerationCapabilities,
    /// Hardware backing available
    pub hardware_backed: bool,
    /// Attestation support
    pub attestation_support: bool,
    /// Maximum concurrent operations
    pub max_concurrent_ops: u32,
}

impl Default for HsmCapabilities {
    fn default() -> Self {
        Self {
            key_generation: KeyGenerationCapabilities::default(),
            hardware_backed: false,
            attestation_support: false,
            max_concurrent_ops: 10,
        }
    }
}

/// HSM performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmMetrics {
    /// Operations per second
    pub ops_per_second: f64,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
    /// Error rate percentage
    pub error_rate_percent: f64,
    /// Uptime percentage
    pub uptime_percent: f64,
}

impl Default for HsmMetrics {
    fn default() -> Self {
        Self {
            ops_per_second: 0.0,
            avg_latency_ms: 0.0,
            error_rate_percent: 0.0,
            uptime_percent: 100.0,
        }
    }
}

/// Tamper resistance levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TamperResistance {
    /// No tamper resistance
    None,
    /// Evidence of tampering
    Evidence,
    /// Active tamper response
    Response,
    /// Hardware tamper protection
    Hardware,
}

impl Default for TamperResistance {
    fn default() -> Self {
        TamperResistance::None
    }
} 