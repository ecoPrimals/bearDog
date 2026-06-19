// SPDX-License-Identifier: AGPL-3.0-or-later

//! Types for Universal HSM Entropy Orchestration

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Human entropy input from various sources
#[derive(Debug, Clone, Default)]
pub struct HumanEntropyInput {
    /// Biometric data (fingerprint, face, voice hash)
    pub biometric_data: Option<Vec<u8>>,

    /// Behavioral data (typing patterns, touch dynamics)
    pub behavioral_data: Option<Vec<u8>>,

    /// Environmental data (location, time, device state)
    pub environmental_data: Option<Vec<u8>>,
}

/// HSM device information for user selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HsmDeviceInfo {
    /// Type of HSM device
    pub device_type: HsmDeviceType,

    /// Unique device identifier
    pub device_id: String,

    /// Human-readable device name
    pub name: String,

    /// Security level of the device
    pub security_level: SecurityLevel,

    /// Whether device supports biometric authentication
    pub biometric_capable: bool,
}

/// Type of HSM device
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HsmDeviceType {
    /// FIDO2/CTAP2 security key (`SoloKeys`, `YubiKey`, etc.)
    Fido2,

    /// Android `StrongBox` (Titan M2, etc.)
    AndroidStrongBox,

    /// iOS Secure Enclave
    IOSSecureEnclave,
}

/// Security level of HSM device
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityLevel {
    /// Software-based security
    Software,

    /// Trusted Execution Environment (TEE)
    Tee,

    /// Hardware security module
    Hardware,

    /// Dedicated security chip (StrongBox/Secure Enclave)
    StrongBox,
}

/// Request for entropy generation
#[derive(Debug, Clone)]
pub struct EntropyGenerationRequest {
    /// Length of entropy to generate (bytes)
    pub length: usize,

    /// Human input to mix with hardware entropy
    pub human_input: Option<HumanEntropyInput>,

    /// Preferred device (None = auto-select best)
    pub preferred_device: Option<String>,

    /// Minimum quality tier required
    pub min_quality_tier: u8,
}

impl Default for EntropyGenerationRequest {
    fn default() -> Self {
        Self {
            length: 256,
            human_input: None,
            preferred_device: None,
            min_quality_tier: 1,
        }
    }
}

/// Result of entropy generation
#[derive(Debug, Clone)]
pub struct EntropyGenerationResult {
    /// Unique seed identifier in entropy hierarchy
    pub seed_id: Uuid,

    /// Quality tier achieved (0 = OS RNG fallback, 1-3 = hardware tiers)
    pub quality_tier: u8,

    /// Quality score (0.0-1.0)
    pub quality_score: f64,

    /// Human-readable label for the entropy path used (e.g. `"os_rng_fallback"`)
    pub device_used: String,

    /// Canonical entropy source identifier (e.g. `"os_rng"` or future hardware source ids)
    pub source: String,

    /// Whether entropy was produced by a hardware-backed RNG
    pub hardware_backed: bool,

    /// Timestamp of generation
    pub timestamp: chrono::DateTime<chrono::Utc>,
}
