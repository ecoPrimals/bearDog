//! Deployment Strategies
//!
//! This module defines various deployment strategies for production environments,
//! including blue-green, rolling, canary, and immediate deployment strategies.

use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::environment::EnvironmentConfig;

/// Deployment strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStrategy {
    /// Blue-green deployment
    BlueGreen {
        /// Blue environment configuration
        blue: EnvironmentConfig,
        /// Green environment configuration
        green: EnvironmentConfig,
        /// Switch configuration
        switch: SwitchConfig,
    },
    /// Rolling deployment
    Rolling {
        /// Batch size
        batch_size: u32,
        /// Batch delay
        batch_delay: Duration,
        /// Health check timeout
        health_check_timeout: Duration,
    },
    /// Canary deployment
    Canary {
        /// Canary percentage
        canary_percentage: f64,
        /// Canary duration
        canary_duration: Duration,
        /// Success criteria
        success_criteria: CanarySuccessCriteria,
    },
    /// Immediate deployment
    Immediate {
        /// Pre-deployment checks
        pre_checks: Vec<PreDeploymentCheck>,
        /// Post-deployment checks
        post_checks: Vec<PostDeploymentCheck>,
    },
}

/// Switch configuration for blue-green deployment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchConfig {
    /// Switch type
    pub switch_type: SwitchType,
    /// Switch timeout
    pub timeout: Duration,
    /// Validation checks
    pub validation: SwitchValidation,
}

/// Switch type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwitchType {
    /// Automatic switch
    Automatic,
    /// Manual switch
    Manual,
    /// Scheduled switch
    Scheduled { schedule: String },
}

/// Switch validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchValidation {
    /// Enable validation
    pub enabled: bool,
    /// Validation checks
    pub checks: Vec<String>,
    /// Validation timeout
    pub timeout: Duration,
}

/// Canary success criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanarySuccessCriteria {
    /// Maximum error rate
    pub max_error_rate: f64,
    /// Minimum response time
    pub min_response_time: Duration,
    /// Success threshold
    pub success_threshold: f64,
}

/// Pre-deployment check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreDeploymentCheck {
    /// Check name
    pub name: String,
    /// Check type
    pub check_type: CheckType,
    /// Check timeout
    pub timeout: Duration,
    /// Check parameters
    pub parameters: std::collections::HashMap<String, String>,
}

/// Post-deployment check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostDeploymentCheck {
    /// Check name
    pub name: String,
    /// Check type
    pub check_type: CheckType,
    /// Check timeout
    pub timeout: Duration,
    /// Check parameters
    pub parameters: std::collections::HashMap<String, String>,
}

/// Check type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CheckType {
    /// Health check
    Health,
    /// Smoke test
    SmokeTest,
    /// Integration test
    Integration,
    /// Custom check
    Custom { script: String },
}

/// Switch strategy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SwitchStrategy {
    /// Immediate switch
    Immediate,
    /// Gradual switch
    Gradual { duration: Duration },
    /// Weighted switch
    Weighted { weight: f64 },
}

impl Default for DeploymentStrategy {
    fn default() -> Self {
        Self::Rolling {
            batch_size: 1,
            batch_delay: Duration::from_secs(30),
            health_check_timeout: Duration::from_secs(60),
        }
    }
}

impl Default for SwitchConfig {
    fn default() -> Self {
        Self {
            switch_type: SwitchType::Automatic,
            timeout: Duration::from_secs(300),
            validation: SwitchValidation::default(),
        }
    }
}

impl Default for SwitchValidation {
    fn default() -> Self {
        Self {
            enabled: true,
            checks: vec!["health".to_string(), "smoke".to_string()],
            timeout: Duration::from_secs(60),
        }
    }
}

impl Default for CanarySuccessCriteria {
    fn default() -> Self {
        Self {
            max_error_rate: 0.01,
            min_response_time: Duration::from_millis(100),
            success_threshold: 0.95,
        }
    }
}
