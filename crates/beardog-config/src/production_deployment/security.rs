//! Security Configuration
//!
//! This module defines security configurations for production deployments.

use serde::{Deserialize, Serialize};

/// Deployment security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSecurity {
    /// Enable security
    pub enabled: bool,
    /// Security policies
    pub policies: Vec<SecurityPolicy>,
}

/// Security policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    /// Policy name
    pub name: String,
    /// Policy type
    pub policy_type: String,
    /// Policy rules
    pub rules: Vec<String>,
}

impl Default for DeploymentSecurity {
    fn default() -> Self {
        Self {
            enabled: true,
            policies: Vec::new(),
        }
    }
}

impl DeploymentSecurity {
    /// Create production security configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            policies: vec![SecurityPolicy {
                name: "default".to_string(),
                policy_type: "strict".to_string(),
                rules: vec!["enforce_tls".to_string()],
            }],
        }
    }

    /// Create development security configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            policies: Vec::new(),
        }
    }
}
