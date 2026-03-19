// SPDX-License-Identifier: AGPL-3.0-only

// Health Monitoring Configuration
//
// Provider health check and monitoring configuration.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Health monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthConfig {
    /// Health checks enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Health check interval
    /// The check interval value
    pub check_interval: Duration,

    /// Health check timeout
    pub check_timeout: Duration,

    /// Number of `failure_threshold`
    pub failure_threshold: u32,

    /// Number of `success_threshold`
    pub success_threshold: u32,

    /// Health check endpoint or path
    /// Optional endpoint
    pub endpoint: Option<String>,

    /// Expected health check response
    /// The expected response value
    pub expected_response: HealthCheckResponse,

    /// Health check method
    /// The method value
    pub method: HealthCheckMethod,

    /// Custom health checks
    /// Collection of custom checks
    pub custom_checks: Vec<CustomHealthCheck>,
}

impl Default for HealthConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: Duration::from_secs(30),
            check_timeout: Duration::from_secs(5),
            failure_threshold: 3,
            success_threshold: 2,
            endpoint: None,
            expected_response: HealthCheckResponse::Ok,
            method: HealthCheckMethod::Http,
            custom_checks: Vec::new(),
        }
    }
}

/// Expected health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthCheckResponse {
    /// HTTP 200 OK response
    Ok,

    /// Specific HTTP status code
    StatusCode(u16),

    /// Response body contains specific text
    BodyContains(String),

    /// JSON response with specific structure
    JsonResponse(serde_json::Value),

    /// Custom validation function
    Custom(String),
}

/// Health check methods
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthCheckMethod {
    /// Http variant
    Http,
    /// Tcp variant
    Tcp,
    /// Grpc variant
    Grpc,
    /// Custom health check type
    Custom(String),
}

/// Custom health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomHealthCheck {
    /// Check name
    /// Name of the item
    pub name: String,

    /// Check description
    /// Optional description
    pub description: Option<String>,

    /// Check interval (overrides global interval)
    /// Optional interval
    pub interval: Option<Duration>,

    /// Check timeout (overrides global timeout)
    pub timeout: Option<Duration>,

    /// Check enabled
    /// Whether feature is enabled
    pub enabled: bool,

    /// Check parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, serde_json::Value>,
}
