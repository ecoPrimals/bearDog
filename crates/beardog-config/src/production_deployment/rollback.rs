//! Rollback Configuration
//!
//! This module defines rollback configurations for production deployments.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Rollback configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackConfig {
    /// Enable rollback
    pub enabled: bool,
    /// Rollback timeout
    pub timeout: Duration,
    /// Rollback strategy
    pub strategy: RollbackStrategy,
}

/// Rollback strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RollbackStrategy {
    /// Automatic rollback
    Automatic,
    /// Manual rollback
    Manual,
    /// Conditional rollback
    Conditional,
}

impl Default for RollbackConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(300),
            strategy: RollbackStrategy::Automatic,
        }
    }
}

impl RollbackConfig {
    /// Create production rollback configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            timeout: Duration::from_secs(600),
            strategy: RollbackStrategy::Automatic,
        }
    }

    /// Create development rollback configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            timeout: Duration::from_secs(60),
            strategy: RollbackStrategy::Manual,
        }
    }
}
