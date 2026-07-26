// SPDX-License-Identifier: AGPL-3.0-or-later

//! HSM Entropy and Seed Types
//!
//! Provides types for managing entropy sources and ephemeral seeds.

use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Ephemeral seed for cryptographic operations
///
/// A time-limited seed value used for key derivation or initialization.
/// Seeds should be securely erased after use.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EphemeralSeed {
    /// The seed data (should be cryptographically random)
    pub seed_data: Vec<u8>,

    /// When this seed was generated
    pub timestamp: SystemTime,
}

impl EphemeralSeed {
    /// Creates a new ephemeral seed
    #[must_use]
    pub fn new(seed_data: Vec<u8>) -> Self {
        Self {
            seed_data,
            timestamp: SystemTime::now(),
        }
    }

    /// Creates a seed with a specific timestamp (for testing)
    #[must_use]
    pub const fn with_timestamp(seed_data: Vec<u8>, timestamp: SystemTime) -> Self {
        Self {
            seed_data,
            timestamp,
        }
    }

    /// Gets the seed size in bytes
    #[must_use]
    pub const fn size(&self) -> usize {
        self.seed_data.len()
    }

    /// Checks if the seed is empty
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.seed_data.is_empty()
    }
}

impl Default for EphemeralSeed {
    fn default() -> Self {
        Self {
            seed_data: vec![0; 32], // Default 32-byte seed
            timestamp: SystemTime::now(),
        }
    }
}

/// Human entropy collection method
///
/// Represents different ways to collect entropy from human interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HumanEntropyMethod {
    /// Touch screen pattern analysis
    TouchPattern,

    /// Device accelerometer measurements
    Accelerometer,

    /// Device gyroscope measurements
    Gyroscope,

    /// Ambient audio sampling
    AudioAmbient,

    /// Biometric input (fingerprint timing, etc.)
    Biometric,
}

impl HumanEntropyMethod {
    /// Returns a human-readable name for the method
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::TouchPattern => "Touch Pattern",
            Self::Accelerometer => "Accelerometer",
            Self::Gyroscope => "Gyroscope",
            Self::AudioAmbient => "Ambient Audio",
            Self::Biometric => "Biometric",
        }
    }

    /// Returns all available methods
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
            Self::TouchPattern,
            Self::Accelerometer,
            Self::Gyroscope,
            Self::AudioAmbient,
            Self::Biometric,
        ]
    }
}

/// Human entropy capabilities
///
/// Describes what entropy collection methods are available on a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanEntropyCapabilities {
    /// List of available entropy collection methods
    pub available_methods: Vec<HumanEntropyMethod>,

    /// Estimated quality of entropy (0.0 = poor, 1.0 = excellent)
    pub quality_estimate: f64,
}

impl HumanEntropyCapabilities {
    /// Creates new capabilities
    #[must_use]
    pub const fn new(available_methods: Vec<HumanEntropyMethod>, quality_estimate: f64) -> Self {
        Self {
            available_methods,
            quality_estimate: quality_estimate.clamp(0.0, 1.0),
        }
    }

    /// Creates capabilities with no available methods
    #[must_use]
    pub const fn none() -> Self {
        Self {
            available_methods: Vec::new(),
            quality_estimate: 0.0,
        }
    }

    /// Checks if a specific method is available
    #[must_use]
    pub fn has_method(&self, method: HumanEntropyMethod) -> bool {
        self.available_methods.contains(&method)
    }

    /// Adds a method to the available list
    pub fn add_method(&mut self, method: HumanEntropyMethod) {
        if !self.has_method(method) {
            self.available_methods.push(method);
        }
    }

    /// Gets the number of available methods
    #[must_use]
    pub const fn method_count(&self) -> usize {
        self.available_methods.len()
    }
}

impl Default for HumanEntropyCapabilities {
    fn default() -> Self {
        Self::none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ephemeral_seed_creation() {
        let seed = EphemeralSeed::new(vec![1, 2, 3, 4]);
        assert_eq!(seed.size(), 4);
        assert!(!seed.is_empty());
    }

    #[test]
    fn test_ephemeral_seed_default() {
        let seed = EphemeralSeed::default();
        assert_eq!(seed.size(), 32);
        assert!(!seed.is_empty());
    }

    #[test]
    fn test_human_entropy_method_name() {
        assert_eq!(HumanEntropyMethod::TouchPattern.name(), "Touch Pattern");
        assert_eq!(HumanEntropyMethod::Biometric.name(), "Biometric");
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_human_entropy_method_all() {
        let methods = HumanEntropyMethod::all();
        assert_eq!(methods.len(), 5);
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    #[test]
    fn test_human_entropy_capabilities_creation() {
        let caps = HumanEntropyCapabilities::new(
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            vec![
                HumanEntropyMethod::TouchPattern,
                HumanEntropyMethod::Biometric,
                // TEST_CATEGORY: unit
                // TEST_DOMAIN: types
                // TEST_PRIORITY: normal
            ],
            0.8,
        );
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(caps.method_count(), 2);
        assert_eq!(caps.quality_estimate, 0.8);
    }

    #[test]
    fn test_human_entropy_capabilities_has_method() {
        let caps = HumanEntropyCapabilities::new(vec![HumanEntropyMethod::TouchPattern], 0.5);
        assert!(caps.has_method(HumanEntropyMethod::TouchPattern));
        assert!(!caps.has_method(HumanEntropyMethod::Biometric));
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    #[test]
    fn test_human_entropy_capabilities_add_method() {
        let mut caps = HumanEntropyCapabilities::none();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(caps.method_count(), 0);

        caps.add_method(HumanEntropyMethod::TouchPattern);
        assert_eq!(caps.method_count(), 1);

        // Adding same method doesn't duplicate
        caps.add_method(HumanEntropyMethod::TouchPattern);
        assert_eq!(caps.method_count(), 1);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_quality_estimate_clamping() {
        let caps1 = HumanEntropyCapabilities::new(vec![], 1.5);
        assert_eq!(caps1.quality_estimate, 1.0);

        let caps2 = HumanEntropyCapabilities::new(vec![], -0.5);
        assert_eq!(caps2.quality_estimate, 0.0);
    }
}
