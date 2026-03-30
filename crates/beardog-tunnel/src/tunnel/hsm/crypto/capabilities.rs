// SPDX-License-Identifier: AGPL-3.0-only

//! Crypto Provider Capabilities
//!
//! Defines what capabilities a crypto provider has, similar to HSM capabilities.

use super::algorithms::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Capabilities of a crypto provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CryptoCapabilities {
    /// Name of the crypto provider
    pub provider_name: String,
    /// Version of the crypto provider
    pub provider_version: String,

    /// Supported symmetric encryption algorithms
    pub symmetric_algorithms: Vec<SymmetricAlgorithm>,
    /// Supported asymmetric encryption algorithms
    pub asymmetric_algorithms: Vec<AsymmetricAlgorithm>,
    /// Supported digital signature algorithms
    pub signature_algorithms: Vec<SignatureAlgorithm>,
    /// Supported hash algorithms
    pub hash_algorithms: Vec<HashAlgorithm>,
    /// Supported key derivation functions
    pub kdf_algorithms: Vec<KdfAlgorithm>,

    /// Performance characteristics
    pub performance_profile: PerformanceProfile,

    /// Side-channel attack resistance level
    pub side_channel_resistance: SideChannelResistance,
    /// Operations guaranteed to execute in constant time
    pub constant_time_ops: Vec<String>,

    /// Platforms this provider runs on
    pub supported_platforms: Vec<Platform>,
    /// Hardware acceleration features used
    pub hardware_acceleration: Vec<HardwareFeature>,
}

impl CryptoCapabilities {
    /// Check if this provider supports a specific symmetric algorithm
    pub fn supports_symmetric(&self, algorithm: &SymmetricAlgorithm) -> bool {
        self.symmetric_algorithms.contains(algorithm)
    }

    /// Check if this provider supports a specific signature algorithm
    pub fn supports_signature(&self, algorithm: &SignatureAlgorithm) -> bool {
        self.signature_algorithms.contains(algorithm)
    }

    /// Check if this provider supports a specific hash algorithm
    pub fn supports_hash(&self, algorithm: &HashAlgorithm) -> bool {
        self.hash_algorithms.contains(algorithm)
    }
}

/// Performance profile of a crypto provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
    /// Throughput in MB/s for various algorithms
    pub throughput_mbps: HashMap<String, f64>,
    /// Latency in microseconds for various algorithms
    pub latency_us: HashMap<String, f64>,
    /// Memory overhead in bytes
    pub memory_overhead_bytes: usize,
}

impl Default for PerformanceProfile {
    fn default() -> Self {
        Self {
            throughput_mbps: HashMap::new(),
            latency_us: HashMap::new(),
            memory_overhead_bytes: 4096,
        }
    }
}

/// Side-channel resistance level
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SideChannelResistance {
    /// No specific side-channel protections
    None,
    /// Partial protections (some algorithms)
    Partial,
    /// Full protections (all algorithms)
    Full,
}

/// Supported platforms
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Platform {
    /// Linux (`x86_64`, aarch64)
    Linux,
    /// macOS (Apple Silicon, Intel)
    MacOs,
    /// Windows (`x86_64`)
    Windows,
    /// Android (ARM, ARM64)
    Android,
    /// iOS (ARM64)
    Ios,
    /// WebAssembly
    Wasm,
    /// Custom platform identifier
    Custom(String),
}

/// Hardware acceleration features
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HardwareFeature {
    /// Intel AES-NI hardware acceleration
    AesNi,
    /// Intel AVX2 SIMD instructions
    Avx2,
    /// Intel AVX-512 SIMD instructions
    Avx512,
    /// ARM NEON SIMD instructions
    Neon,
    /// Custom hardware feature identifier
    Custom(String),
}
