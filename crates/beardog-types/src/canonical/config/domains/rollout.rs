//! Canonical Rollout Configuration
//!
//! This module provides the unified, canonical deployment rollout configuration
//! for all BearDog components (production deployment, feature flags, canary releases).
//!
//! ## Design Principles:
//! - **Single Source of Truth**: One RolloutConfig across all domains
//! - **Gradual Rollout Support**: Percentage-based and target-group-based strategies
//! - **Rollback Safety**: Configurable thresholds for automatic rollback
//! - **Strategy Flexibility**: Multiple rollout strategies (all-at-once, blue-green, canary, etc.)
//!
//! ## Rollout Strategies:
//! - **AllAtOnce**: Deploy to all targets simultaneously (fastest, highest risk)
//! - **RollingUpdate**: Deploy incrementally across targets
//! - **BlueGreen**: Switch between two environments
//! - **Canary**: Small subset first, then gradual expansion
//! - **TargetGroupBased**: Deploy to specific groups in sequence
//!
//! ## Usage:
//!
//! ```rust
//! use beardog_types::canonical::config::domains::rollout::{
//!     CanonicalRolloutConfig, RolloutStrategy
//! };
//!
//! let config = CanonicalRolloutConfig {
//!     enabled: true,
//!     strategy: RolloutStrategy::Canary,
//!     percentage: 10.0,
//!     rollback_threshold_percentage: 5.0,
//!     ..Default::default()
//! };
//! ```

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Canonical rollout configuration for all BearDog deployments
///
/// This is the single, unified rollout configuration.
/// All domain-specific RolloutConfigs should migrate to this.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CanonicalRolloutConfig {
    /// Whether gradual rollout is enabled
    pub enabled: bool,

    /// Rollout strategy to use
    pub strategy: RolloutStrategy,

    /// Rollout percentage (0.0-100.0)
    pub percentage: f64,

    /// Target groups to deploy to (for TargetGroupBased strategy)
    pub target_groups: Vec<String>,

    /// Automatic rollback if error percentage exceeds this threshold
    pub rollback_threshold_percentage: f64,

    /// Minimum time to wait between rollout stages
    pub stage_delay: Duration,

    /// Maximum time to wait for a rollout stage to complete
    pub stage_timeout: Duration,

    /// Whether to automatically proceed to next stage on success
    pub auto_promote: bool,

    /// Minimum success rate required to proceed (0.0-100.0)
    pub min_success_rate: f64,

    /// Health check interval during rollout
    pub health_check_interval: Duration,
}

/// Type alias for convenience (follows BearDog naming conventions)
pub type RolloutConfig = CanonicalRolloutConfig;

/// Rollout strategy selection
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RolloutStrategy {
    /// Deploy to all targets simultaneously (fastest, highest risk)
    AllAtOnce,

    /// Deploy incrementally across targets in rolling fashion
    RollingUpdate,

    /// Switch between blue and green environments
    BlueGreen,

    /// Deploy to small subset first, then expand gradually
    Canary,

    /// Deploy to specific target groups in sequence
    TargetGroupBased,

    /// Custom rollout strategy (application-defined)
    Custom,
}

impl Default for CanonicalRolloutConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            strategy: RolloutStrategy::RollingUpdate,
            percentage: 100.0,
            target_groups: Vec::new(),
            rollback_threshold_percentage: 10.0, // Rollback if >10% errors
            stage_delay: Duration::from_secs(60), // 1 minute between stages
            stage_timeout: Duration::from_secs(600), // 10 minutes per stage
            auto_promote: false, // Require manual promotion by default
            min_success_rate: 95.0, // Require 95% success to proceed
            health_check_interval: Duration::from_secs(10),
        }
    }
}

impl Default for RolloutStrategy {
    fn default() -> Self {
        Self::RollingUpdate
    }
}

impl CanonicalRolloutConfig {
    /// Create a new rollout config with percentage-based canary strategy
    pub fn canary(percentage: f64) -> Self {
        Self {
            strategy: RolloutStrategy::Canary,
            percentage: percentage.clamp(0.0, 100.0),
            ..Default::default()
        }
    }

    /// Create a new rollout config with target group strategy
    pub fn target_groups(groups: Vec<String>) -> Self {
        Self {
            strategy: RolloutStrategy::TargetGroupBased,
            target_groups: groups,
            ..Default::default()
        }
    }

    /// Create a new rollout config for immediate deployment
    pub fn all_at_once() -> Self {
        Self {
            strategy: RolloutStrategy::AllAtOnce,
            percentage: 100.0,
            auto_promote: true,
            ..Default::default()
        }
    }

    /// Validate the rollout configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.percentage < 0.0 || self.percentage > 100.0 {
            return Err("percentage must be between 0.0 and 100.0".to_string());
        }
        if self.rollback_threshold_percentage < 0.0 || self.rollback_threshold_percentage > 100.0 {
            return Err("rollback_threshold_percentage must be between 0.0 and 100.0".to_string());
        }
        if self.min_success_rate < 0.0 || self.min_success_rate > 100.0 {
            return Err("min_success_rate must be between 0.0 and 100.0".to_string());
        }
        if self.strategy == RolloutStrategy::TargetGroupBased && self.target_groups.is_empty() {
            return Err("target_groups cannot be empty for TargetGroupBased strategy".to_string());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = CanonicalRolloutConfig::default();
        assert!(config.enabled);
        assert_eq!(config.strategy, RolloutStrategy::RollingUpdate);
        assert_eq!(config.percentage, 100.0);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_canary_constructor() {
        let config = CanonicalRolloutConfig::canary(10.0);
        assert_eq!(config.strategy, RolloutStrategy::Canary);
        assert_eq!(config.percentage, 10.0);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_canary_percentage_clamping() {
        let config = CanonicalRolloutConfig::canary(150.0);
        assert_eq!(config.percentage, 100.0); // Clamped to max

        let config = CanonicalRolloutConfig::canary(-50.0);
        assert_eq!(config.percentage, 0.0); // Clamped to min
    }

    #[test]
    fn test_target_groups_constructor() {
        let groups = vec!["group1".to_string(), "group2".to_string()];
        let config = CanonicalRolloutConfig::target_groups(groups.clone());
        assert_eq!(config.strategy, RolloutStrategy::TargetGroupBased);
        assert_eq!(config.target_groups, groups);
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_all_at_once_constructor() {
        let config = CanonicalRolloutConfig::all_at_once();
        assert_eq!(config.strategy, RolloutStrategy::AllAtOnce);
        assert_eq!(config.percentage, 100.0);
        assert!(config.auto_promote);
    }

    #[test]
    fn test_validation_invalid_percentage() {
        let mut config = CanonicalRolloutConfig::default();
        config.percentage = 150.0;
        assert!(config.validate().is_err());

        config.percentage = -10.0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_target_groups() {
        let mut config = CanonicalRolloutConfig::default();
        config.strategy = RolloutStrategy::TargetGroupBased;
        assert!(config.validate().is_err()); // Empty target_groups

        config.target_groups = vec!["group1".to_string()];
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_serialization() {
        let config = CanonicalRolloutConfig {
            enabled: true,
            strategy: RolloutStrategy::Canary,
            percentage: 25.0,
            target_groups: vec!["staging".to_string()],
            rollback_threshold_percentage: 5.0,
            stage_delay: Duration::from_secs(30),
            stage_timeout: Duration::from_secs(300),
            auto_promote: true,
            min_success_rate: 99.0,
            health_check_interval: Duration::from_secs(5),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: CanonicalRolloutConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, config);
    }

    #[test]
    fn test_strategy_serialization() {
        let strategy = RolloutStrategy::BlueGreen;
        let json = serde_json::to_string(&strategy).unwrap();
        assert_eq!(json, "\"blue_green\"");

        let deserialized: RolloutStrategy = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, strategy);
    }

    #[test]
    fn test_type_alias() {
        let _config: RolloutConfig = CanonicalRolloutConfig::default();
    }
}

