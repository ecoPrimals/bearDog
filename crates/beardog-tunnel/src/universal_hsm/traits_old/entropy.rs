//! Human entropy collection types and functionality

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Human entropy data collected from user interactions
#[derive(Debug, Clone)]
pub struct HumanEntropyData {
    /// Source of entropy
    pub source: HumanEntropyMethod,
    /// Entropy bytes collected
    pub entropy_bytes: Vec<u8>,
    /// When the entropy was collected
    pub collected_at: DateTime<Utc>,
    /// Quality score (0.0 - 1.0)
    pub quality_score: f64,
}

/// Human entropy collection capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEntropyCapabilities {
    /// Whether ephemeral seeds are supported
    pub supports_ephemeral_seeds: bool,
    /// Available collection methods
    pub collection_methods: Vec<HumanEntropyMethod>,
    /// Whether realtime entropy collection is supported
    pub realtime_entropy: bool,
    /// Whether quality assessment is available
    pub quality_assessment: bool,
    /// Whether biometric integration is supported
    pub biometric_integration: bool,
    /// Minimum entropy bits required
    pub min_entropy_bits: f64,
    /// Maximum collection rate (Hz)
    pub max_collection_rate: f64,
}

/// Ephemeral seed generated from human entropy
#[derive(Debug, Clone)]
pub struct EphemeralSeed {
    /// Seed identifier
    pub seed_id: String,
    /// Seed bytes
    pub seed_bytes: Vec<u8>,
    /// Source entropy used to generate seed
    pub source_entropy: Vec<u8>,
    /// When the seed expires
    pub expires_at: DateTime<Utc>,
}

impl EphemeralSeed {
    /// Create a new ephemeral seed
    pub fn new(seed_bytes: Vec<u8>, source_entropy: Vec<u8>, lifetime_seconds: i64) -> Self {
        Self {
            seed_id: uuid::Uuid::new_v4().to_string(),
            seed_bytes,
            source_entropy,
            expires_at: Utc::now() + chrono::Duration::seconds(lifetime_seconds),
        }
    }

    /// Check if the seed has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}

/// Methods for collecting human entropy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HumanEntropyMethod {
    /// Touchscreen interactions
    Touchscreen,
    /// Mouse movements
    Mouse,
    /// Keyboard timing
    Keyboard,
    /// Accelerometer data
    Accelerometer,
    /// Gyroscope data
    Gyroscope,
    /// Camera-based biometrics
    Camera,
    /// Microphone-based biometrics
    Microphone,
    /// Custom method
    Custom,
}

impl std::fmt::Display for HumanEntropyMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HumanEntropyMethod::Touchscreen => write!(f, "Touchscreen"),
            HumanEntropyMethod::Mouse => write!(f, "Mouse"),
            HumanEntropyMethod::Keyboard => write!(f, "Keyboard"),
            HumanEntropyMethod::Accelerometer => write!(f, "Accelerometer"),
            HumanEntropyMethod::Gyroscope => write!(f, "Gyroscope"),
            HumanEntropyMethod::Camera => write!(f, "Camera"),
            HumanEntropyMethod::Microphone => write!(f, "Microphone"),
            HumanEntropyMethod::Custom => write!(f, "Custom"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ephemeral_seed_creation() {
        let seed = EphemeralSeed::new(
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            3600, // 1 hour
        );
        
        assert_eq!(seed.seed_bytes, vec![1, 2, 3, 4]);
        assert!(!seed.is_expired());
    }

    #[test]
    fn test_entropy_method_display() {
        assert_eq!(HumanEntropyMethod::Touchscreen.to_string(), "Touchscreen");
        assert_eq!(HumanEntropyMethod::Mouse.to_string(), "Mouse");
    }
}
