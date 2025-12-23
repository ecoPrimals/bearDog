//! Type-safe newtypes and validated values for AI configuration
//!
//! This module provides compile-time and runtime validation through the type system.

use beardog_errors::BearDogError;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;

type Result<T> = Result<T>;

/// Human oversight level (0.0 = full automation, 1.0 = full human control)
///
/// Validated to be between 0.0 and 1.0 inclusive.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "f64", into = "f64")]
pub struct OversightLevel(f64);

impl OversightLevel {
    /// Create a new oversight level with validation
    pub fn new(level: f64) -> Result<Self> {
        if !(0.0..=1.0).contains(&level) {
            return Err(BearDogError::Business {
                message: format!("Oversight level must be between 0.0 and 1.0, got {}", level),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }
        Ok(Self(level))
    }

    /// Get the raw value
    pub const fn value(&self) -> f64 {
        self.0
    }

    /// Full automation (0.0)
    pub const fn full_automation() -> Self {
        Self(0.0)
    }

    /// Balanced (0.5)
    pub const fn balanced() -> Self {
        Self(0.5)
    }

    /// Full human control (1.0)
    pub const fn full_human() -> Self {
        Self(1.0)
    }
}

impl TryFrom<f64> for OversightLevel {
    type Error = BearDogError;
    fn try_from(value: f64) -> Result<Self> {
        Self::new(value)
    }
}

impl From<OversightLevel> for f64 {
    fn from(level: OversightLevel) -> f64 {
        level.0
    }
}

/// Learning rate for model training
///
/// Validated to be positive and non-zero.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "f64", into = "f64")]
pub struct LearningRate(f64);

impl LearningRate {
    /// Create a new learning rate with validation
    pub fn new(rate: f64) -> Result<Self> {
        if rate <= 0.0 || !rate.is_finite() {
            return Err(BearDogError::Business {
                message: format!("Learning rate must be positive and finite, got {}", rate),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }
        Ok(Self(rate))
    }

    /// Get the raw value
    pub const fn value(&self) -> f64 {
        self.0
    }

    /// Standard learning rate (0.001)
    pub const fn standard() -> Self {
        Self(0.001)
    }

    /// Fast learning rate (0.01)
    pub const fn fast() -> Self {
        Self(0.01)
    }

    /// Slow learning rate (0.0001)
    pub const fn slow() -> Self {
        Self(0.0001)
    }
}

impl TryFrom<f64> for LearningRate {
    type Error = BearDogError;
    fn try_from(value: f64) -> Result<Self> {
        Self::new(value)
    }
}

impl From<LearningRate> for f64 {
    fn from(rate: LearningRate) -> f64 {
        rate.0
    }
}

/// Batch size for training (guaranteed non-zero)
pub type BatchSize = NonZeroUsize;

/// Confidence threshold (0.0 to 1.0)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "f64", into = "f64")]
pub struct ConfidenceThreshold(f64);

impl ConfidenceThreshold {
    /// Create a new confidence threshold with validation
    pub fn new(threshold: f64) -> Result<Self> {
        if !(0.0..=1.0).contains(&threshold) {
            return Err(BearDogError::Business {
                message: format!(
                    "Confidence threshold must be between 0.0 and 1.0, got {}",
                    threshold
                ),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }
        Ok(Self(threshold))
    }

    /// Get the raw value
    pub const fn value(&self) -> f64 {
        self.0
    }

    /// High confidence (0.9)
    pub const fn high() -> Self {
        Self(0.9)
    }

    /// Medium confidence (0.7)
    pub const fn medium() -> Self {
        Self(0.7)
    }

    /// Low confidence (0.5)
    pub const fn low() -> Self {
        Self(0.5)
    }
}

impl TryFrom<f64> for ConfidenceThreshold {
    type Error = BearDogError;
    fn try_from(value: f64) -> Result<Self> {
        Self::new(value)
    }
}

impl From<ConfidenceThreshold> for f64 {
    fn from(threshold: ConfidenceThreshold) -> f64 {
        threshold.0
    }
}

/// Validation split ratio (0.0 to 1.0)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "f64", into = "f64")]
pub struct ValidationSplit(f64);

impl ValidationSplit {
    /// Create a new validation split with validation
    pub fn new(split: f64) -> Result<Self> {
        if !(0.0..=1.0).contains(&split) {
            return Err(BearDogError::Business {
                message: format!(
                    "Validation split must be between 0.0 and 1.0, got {}",
                    split
                ),
                category: beardog_errors::BusinessErrorCategory::Validation,
            });
        }
        Ok(Self(split))
    }

    /// Get the raw value
    pub const fn value(&self) -> f64 {
        self.0
    }

    /// Standard split (0.2 = 20%)
    pub const fn standard() -> Self {
        Self(0.2)
    }

    /// Small split (0.1 = 10%)
    pub const fn small() -> Self {
        Self(0.1)
    }

    /// Large split (0.3 = 30%)
    pub const fn large() -> Self {
        Self(0.3)
    }
}

impl TryFrom<f64> for ValidationSplit {
    type Error = BearDogError;
    fn try_from(value: f64) -> Result<Self> {
        Self::new(value)
    }
}

impl From<ValidationSplit> for f64 {
    fn from(split: ValidationSplit) -> f64 {
        split.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oversight_level_validation() {
        assert!(OversightLevel::new(0.0).is_ok());
        assert!(OversightLevel::new(0.5).is_ok());
        assert!(OversightLevel::new(1.0).is_ok());
        assert!(OversightLevel::new(-0.1).is_err());
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(OversightLevel::new(1.1).is_err());
    }

    #[test]
    fn test_learning_rate_validation() {
        assert!(LearningRate::new(0.001).is_ok());
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert!(LearningRate::new(0.0).is_err());
        assert!(LearningRate::new(-0.1).is_err());
        assert!(LearningRate::new(f64::INFINITY).is_err());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_confidence_threshold_validation() {
        assert!(ConfidenceThreshold::new(0.5).is_ok());
        assert!(ConfidenceThreshold::new(-0.1).is_err());
        assert!(ConfidenceThreshold::new(1.1).is_err());
    }
}
