//! Network configuration
//!
//! This module contains network-related configuration types.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// Enable networking
    pub enabled: bool,
    /// Core network settings
    pub core: NetworkCoreConfig,
    /// Security settings
    pub security: NetworkSecurityConfig,
    /// Performance settings
    pub performance: NetworkPerformanceConfig,
    /// Service discovery
    pub service_discovery: ServiceDiscoveryConfig,
}

/// Core network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkCoreConfig {
    /// Port to bind to
    pub port: u16,
    /// Host to bind to
    pub host: String,
    /// Connection timeout
    pub timeout: Duration,
}

/// Network security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurityConfig {
    /// Enable TLS
    pub enable_tls: bool,
    /// Certificate path
    pub cert_path: Option<String>,
    /// Private key path
    pub key_path: Option<String>,
}

/// Network performance configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkPerformanceConfig {
    /// Maximum connections
    pub max_connections: usize,
    /// Keep alive timeout
    pub keep_alive_timeout: Duration,
}

/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfig {
    /// Enable service discovery
    pub enabled: bool,
    /// Discovery method
    pub method: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            core: NetworkCoreConfig::default(),
            security: NetworkSecurityConfig::default(),
            performance: NetworkPerformanceConfig::default(),
            service_discovery: ServiceDiscoveryConfig::default(),
        }
    }
}

impl Default for NetworkCoreConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "0.0.0.0".to_string(),
            timeout: Duration::from_secs(30),
        }
    }
}

impl Default for NetworkSecurityConfig {
    fn default() -> Self {
        Self {
            enable_tls: false,
            cert_path: None,
            key_path: None,
        }
    }
}

impl Default for NetworkPerformanceConfig {
    fn default() -> Self {
        Self {
            max_connections: 1000,
            keep_alive_timeout: Duration::from_secs(60),
        }
    }
}

impl Default for ServiceDiscoveryConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            method: "static".to_string(),
        }
    }
} 