//! Services Configuration
//!
//! This module defines service configurations for production deployments.

use serde::{Deserialize, Serialize};

/// Service configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfig {
    /// Service name
    pub name: String,
    /// Service type
    pub service_type: String,
    /// Service port
    pub port: u16,
    /// Service replicas
    pub replicas: u32,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            service_type: "web".to_string(),
            port: 8080,
            replicas: 1,
        }
    }
}
