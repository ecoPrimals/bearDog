//! Deployment Validation
//!
//! This module defines validation configurations for production deployments.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Deployment validation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentValidation {
    /// Enable validation
    pub enabled: bool,
    /// Validation checks
    pub checks: Vec<ValidationCheck>,
    /// Validation timeout
    pub timeout: Duration,
}

/// Validation check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationCheck {
    /// Check name
    pub name: String,
    /// Check type
    pub check_type: String,
    /// Check parameters
    pub parameters: std::collections::HashMap<String, String>,
}

impl Default for DeploymentValidation {
    fn default() -> Self {
        Self {
            enabled: true,
            checks: Vec::new(),
            timeout: Duration::from_secs(300),
        }
    }
}

impl DeploymentValidation {
    /// Create production validation configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            checks: vec![
                ValidationCheck {
                    name: "health_check".to_string(),
                    check_type: "health".to_string(),
                    parameters: std::collections::HashMap::new(),
                },
                ValidationCheck {
                    name: "smoke_test".to_string(),
                    check_type: "smoke".to_string(),
                    parameters: std::collections::HashMap::new(),
                },
            ],
            timeout: Duration::from_secs(600),
        }
    }

    /// Create development validation configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            checks: Vec::new(),
            timeout: Duration::from_secs(60),
        }
    }
}
