//! Health Checks Configuration
//!
//! This module defines health check configurations for production deployments.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Deployment health checks configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentHealthChecks {
    /// Enable health checks
    pub enabled: bool,
    /// Health check endpoints
    pub endpoints: Vec<HealthCheckEndpoint>,
    /// Health check interval
    pub interval: Duration,
}

/// Health check endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckEndpoint {
    /// Endpoint name
    pub name: String,
    /// Endpoint URL
    pub url: String,
    /// Timeout
    pub timeout: Duration,
}

impl Default for DeploymentHealthChecks {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoints: Vec::new(),
            interval: Duration::from_secs(30),
        }
    }
}

impl DeploymentHealthChecks {
    /// Create production health checks configuration
    pub fn production() -> Self {
        Self {
            enabled: true,
            endpoints: vec![HealthCheckEndpoint {
                name: "api_health".to_string(),
                url: "/health".to_string(),
                timeout: Duration::from_secs(5),
            }],
            interval: Duration::from_secs(10),
        }
    }

    /// Create development health checks configuration
    pub fn development() -> Self {
        Self {
            enabled: false,
            endpoints: Vec::new(),
            interval: Duration::from_secs(60),
        }
    }
}
