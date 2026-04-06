// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Network Monitoring Configuration Module
//!
//! This module contains network monitoring and health check configurations.

use serde::{Deserialize, Serialize};

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfiguration {
    /// Enable health checks
    pub enabled: bool,
    /// Health check interval seconds
    pub interval_seconds: u64,
    /// Health check timeout seconds
    pub timeout_seconds: u64,
    /// Health check endpoint
    pub endpoint: String,
}

/// Network monitoring configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMonitoringConfiguration {
    /// Enable monitoring
    pub enabled: bool,
    /// Metrics collection interval seconds
    pub metrics_interval_seconds: u64,
    /// Enable detailed logging
    pub enable_detailed_logging: bool,
}

/// Service discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceDiscoveryConfiguration {
    /// Enable service discovery
    pub enabled: bool,
    /// Discovery interval seconds
    pub discovery_interval_seconds: u64,
    /// Discovery timeout seconds
    pub discovery_timeout_seconds: u64,
}

impl Default for HealthCheckConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            interval_seconds: 30,
            timeout_seconds: 5,
            endpoint: "/health".to_string(),
        }
    }
}

impl Default for NetworkMonitoringConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_interval_seconds: 60,
            enable_detailed_logging: false,
        }
    }
}

impl Default for ServiceDiscoveryConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            discovery_interval_seconds: 30,
            discovery_timeout_seconds: 5,
        }
    }
}
