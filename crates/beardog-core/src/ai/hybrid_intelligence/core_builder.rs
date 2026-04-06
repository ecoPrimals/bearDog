// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builder pattern for Hybrid Intelligence System
//!
//! Provides a fluent interface for constructing `HybridIntelligenceSystem` instances
//! with custom configuration.

use super::config::HybridIntelligenceConfig;
use super::core::HybridIntelligenceSystem;
use beardog_errors::BearDogError;

/// Builder for constructing `HybridIntelligenceSystem` with custom configuration
///
/// Uses the builder pattern to provide a fluent interface for system construction.
#[derive(Debug, Clone)]
pub struct HybridIntelligenceBuilder {
    config: Option<HybridIntelligenceConfig>,
}

impl HybridIntelligenceBuilder {
    /// Creates a new builder with default configuration
    #[must_use]
    pub const fn new() -> Self {
        Self { config: None }
    }

    /// Sets the system configuration
    #[must_use]
    pub fn with_config(mut self, config: HybridIntelligenceConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Builds the `HybridIntelligenceSystem`
    ///
    /// # Errors
    /// Returns an error if system initialization fails
    pub fn build(self) -> Result<HybridIntelligenceSystem, BearDogError> {
        let config = self.config.unwrap_or_default();
        HybridIntelligenceSystem::new(config)
    }
}

impl Default for HybridIntelligenceBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default() {
        let builder = HybridIntelligenceBuilder::new();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(builder.config.is_none());
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[test]
    fn test_builder_with_config() {
        let config = HybridIntelligenceConfig::default();
        let builder = HybridIntelligenceBuilder::new().with_config(config);
        assert!(builder.config.is_some());
    }
}
