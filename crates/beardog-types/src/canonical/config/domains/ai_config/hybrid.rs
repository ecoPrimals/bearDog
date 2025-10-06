//! Hybrid Intelligence Configuration
//!
//! Modern, type-safe configuration for human-AI collaboration with builder pattern.

use super::types::{ConfidenceThreshold, OversightLevel};
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;

type Result<T> = BearDogResult<T>;

/// Hybrid intelligence configuration for human-AI collaboration
///
/// # Examples
///
/// ```ignore
/// use std::time::Duration;
///
/// // Using builder pattern
/// let config = HybridIntelligenceConfig::builder()
///     .oversight_level(OversightLevel::balanced())
///     .auto_decision_threshold(ConfidenceThreshold::high())
///     .human_input_timeout(Duration::from_secs(30))
///     .build()?;
///
/// // Using defaults
/// let config = HybridIntelligenceConfig::default();
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HybridIntelligenceConfig {
    /// Enable hybrid intelligence mode
    enabled: bool,

    /// Human oversight level
    oversight_level: OversightLevel,

    /// Confidence threshold for automatic decisions
    auto_decision_threshold: ConfidenceThreshold,

    /// Enable human feedback learning
    feedback_learning: bool,

    /// Maximum time to wait for human input
    human_input_timeout: Duration,
}

impl HybridIntelligenceConfig {
    /// Create a new builder
    pub fn builder() -> HybridIntelligenceConfigBuilder {
        HybridIntelligenceConfigBuilder::default()
    }

    /// Check if hybrid intelligence is enabled
    pub const fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get the oversight level
    pub const fn oversight_level(&self) -> OversightLevel {
        self.oversight_level
    }

    /// Get the auto decision threshold
    pub const fn auto_decision_threshold(&self) -> ConfidenceThreshold {
        self.auto_decision_threshold
    }

    /// Check if feedback learning is enabled
    pub const fn has_feedback_learning(&self) -> bool {
        self.feedback_learning
    }

    /// Get the human input timeout
    pub const fn human_input_timeout(&self) -> Duration {
        self.human_input_timeout
    }

    /// Check if a decision can be made automatically based on confidence
    pub const fn can_auto_decide(&self, confidence: f64) -> bool {
        confidence >= self.auto_decision_threshold.value()
    }
}

impl Default for HybridIntelligenceConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            oversight_level: OversightLevel::balanced(),
            auto_decision_threshold: ConfidenceThreshold::high(),
            feedback_learning: true,
            human_input_timeout: Duration::from_secs(30),
        }
    }
}

/// Builder for `HybridIntelligenceConfig`
///
/// Provides a fluent API for configuration with validation.
#[derive(Debug, Clone)]
pub struct HybridIntelligenceConfigBuilder {
    enabled: bool,
    oversight_level: OversightLevel,
    auto_decision_threshold: ConfidenceThreshold,
    feedback_learning: bool,
    human_input_timeout: Duration,
}

impl Default for HybridIntelligenceConfigBuilder {
    fn default() -> Self {
        Self {
            enabled: false,
            oversight_level: OversightLevel::balanced(),
            auto_decision_threshold: ConfidenceThreshold::high(),
            feedback_learning: true,
            human_input_timeout: Duration::from_secs(30),
        }
    }
}

impl HybridIntelligenceConfigBuilder {
    /// Enable or disable hybrid intelligence
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set the oversight level
    pub fn oversight_level(mut self, level: OversightLevel) -> Self {
        self.oversight_level = level;
        self
    }

    /// Set the oversight level from a raw f64 (with validation)
    pub fn oversight_level_raw(mut self, level: f64) -> Result<Self> {
        self.oversight_level = OversightLevel::new(level)?;
        Ok(self)
    }

    /// Set the auto decision threshold
    pub fn auto_decision_threshold(mut self, threshold: ConfidenceThreshold) -> Self {
        self.auto_decision_threshold = threshold;
        self
    }

    /// Set the auto decision threshold from a raw f64 (with validation)
    pub fn auto_decision_threshold_raw(mut self, threshold: f64) -> Result<Self> {
        self.auto_decision_threshold = ConfidenceThreshold::new(threshold)?;
        Ok(self)
    }

    /// Enable or disable feedback learning
    pub fn feedback_learning(mut self, enabled: bool) -> Self {
        self.feedback_learning = enabled;
        self
    }

    /// Set the human input timeout
    pub fn human_input_timeout(mut self, timeout: Duration) -> Self {
        self.human_input_timeout = timeout;
        self
    }

    /// Build the configuration
    ///
    /// Performs final validation checks.
    pub fn build(self) -> Result<HybridIntelligenceConfig> {
        // Validate that timeout is reasonable
        if self.human_input_timeout.as_secs() > 300 {
            return Err(BearDogError::validation(
                "Human input timeout should not exceed 5 minutes (300 seconds)",
            ));
        }

        Ok(HybridIntelligenceConfig {
            enabled: self.enabled,
            oversight_level: self.oversight_level,
            auto_decision_threshold: self.auto_decision_threshold,
            feedback_learning: self.feedback_learning,
            human_input_timeout: self.human_input_timeout,
        })
    }

    /// Preset: Full automation mode
    pub fn full_automation() -> Self {
        Self::default()
            .enabled(true)
            .oversight_level(OversightLevel::full_automation())
            .auto_decision_threshold(ConfidenceThreshold::low())
    }

    /// Preset: Balanced mode
    pub fn balanced() -> Self {
        Self::default()
            .enabled(true)
            .oversight_level(OversightLevel::balanced())
            .auto_decision_threshold(ConfidenceThreshold::medium())
    }

    /// Preset: Human-centric mode
    pub fn human_centric() -> Self {
        Self::default()
            .enabled(true)
            .oversight_level(OversightLevel::full_human())
            .auto_decision_threshold(ConfidenceThreshold::high())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = HybridIntelligenceConfig::builder()
            .enabled(true)
            .oversight_level(OversightLevel::balanced())
            .build()
            .unwrap();

        assert!(config.is_enabled());
        assert_eq!(config.oversight_level().value(), 0.5);
    }

    #[test]
    fn test_builder_presets() {
        let auto = HybridIntelligenceConfigBuilder::full_automation()
            .build()
            .unwrap();
        assert_eq!(auto.oversight_level().value(), 0.0);

        let human = HybridIntelligenceConfigBuilder::human_centric()
            .build()
            .unwrap();
        assert_eq!(human.oversight_level().value(), 1.0);
    }

    #[test]
    fn test_timeout_validation() {
        let result = HybridIntelligenceConfig::builder()
            .human_input_timeout(Duration::from_secs(400))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_can_auto_decide() {
        let config = HybridIntelligenceConfig::builder()
            .auto_decision_threshold(ConfidenceThreshold::high())
            .build()
            .unwrap();

        assert!(config.can_auto_decide(0.95));
        assert!(!config.can_auto_decide(0.85));
    }
}
