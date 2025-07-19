//! Network Configuration
//!
//! This module defines network configurations for production deployments.

use serde::{Deserialize, Serialize};

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Enable network
    pub enabled: bool,
    /// Network interfaces
    pub interfaces: Vec<NetworkInterface>,
}

/// Network interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// Interface name
    pub name: String,
    /// Interface type
    pub interface_type: String,
    /// Interface address
    pub address: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interfaces: Vec::new(),
        }
    }
}
