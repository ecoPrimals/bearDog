// Deployment Configuration
//
// This module contains deployment and release management configuration
// including deployment strategies, rollout procedures, and rollback settings.

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// **DEPLOYMENT CONFIGURATION** - Deployment and release management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// Deployment strategy
    /// The strategy value
    pub strategy: DeploymentStrategy,

    /// Rollout configuration
    /// The rollout value
    pub rollout: RolloutConfig,

    /// Canary deployment settings
    /// The canary value
    pub canary: CanaryConfig,

    /// Blue-green deployment settings
    /// The blue green value
    pub blue_green: BlueGreenConfig,

    /// Rollback configuration
    /// The rollback value
    pub rollback: RollbackConfig,
}

/// Deployment strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStrategy {
    /// Rolling deployment
    Rolling,
    /// Blue-green deployment
    BlueGreen,
    /// Canary deployment
    Canary,
}

/// Rollout configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolloutConfig {
    /// Rollout percentage
    /// The percentage value
    pub percentage: f64,
}

/// Canary configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanaryConfig {
    /// Canary percentage
    /// The percentage value
    pub percentage: f64,
}

/// Blue-green configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BlueGreenConfig {
    /// Enable blue-green deployment
    /// Whether feature is enabled
    pub enabled: bool,
}

/// Rollback configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RollbackConfig {
    /// Enable automatic rollback
    /// Whether automatic is enabled
    pub automatic: bool,
}

impl RolloutConfig {
    /// Default rollout percentage (100%)
    pub const DEFAULT_PERCENTAGE: f64 = 100.0;

    /// Create RolloutConfig with hardcoded defaults
    pub fn with_defaults() -> Self {
        Self {
            percentage: Self::DEFAULT_PERCENTAGE,
        }
    }

    /// Create RolloutConfig from environment variables
    pub fn from_env() -> Self {
        Self {
            percentage: std::env::var("BEARDOG_ROLLOUT_PERCENTAGE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_PERCENTAGE),
        }
    }
}

impl CanaryConfig {
    /// Default canary percentage (10%)
    pub const DEFAULT_PERCENTAGE: f64 = 10.0;

    /// Create CanaryConfig with hardcoded defaults
    pub fn with_defaults() -> Self {
        Self {
            percentage: Self::DEFAULT_PERCENTAGE,
        }
    }

    /// Create CanaryConfig from environment variables
    pub fn from_env() -> Self {
        Self {
            percentage: std::env::var("BEARDOG_CANARY_PERCENTAGE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(Self::DEFAULT_PERCENTAGE),
        }
    }
}

impl Default for DeploymentConfig {
    fn default() -> Self {
        Self {
            strategy: DeploymentStrategy::Rolling,
            rollout: RolloutConfig::default(),
            canary: CanaryConfig::default(),
            blue_green: BlueGreenConfig::default(),
            rollback: RollbackConfig::default(),
        }
    }
}

impl Default for RolloutConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl Default for CanaryConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl DeploymentConfig {
    /// Validate
    /// Validates input
    /// Validates input
    pub fn validate(&self) -> Result<(), BearDogError> {
        Ok(())
    }
}
