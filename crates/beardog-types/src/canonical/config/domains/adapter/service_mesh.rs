// SPDX-License-Identifier: AGPL-3.0-only

//! Service mesh and handoff configuration

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

/// Service mesh and handoff configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct ServiceMeshConfig {
    /// Enable service mesh integration
    pub enabled: bool,

    /// Mesh discovery configuration
    pub discovery: MeshDiscoveryConfig,

    /// Handoff retry configuration
    pub handoff_retry: HandoffRetryConfig,

    /// Mesh security configuration
    pub security: MeshSecurityConfig,

    /// Health monitoring configuration
    pub health_monitor: HealthMonitorConfig,
}

/// Mesh discovery specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MeshDiscoveryConfig {
    /// Discovery protocol (Arc for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub protocol: Arc<str>,

    /// Discovery port
    pub port: u16,

    /// Discovery timeout
    pub timeout: Duration,

    /// Enable mTLS for discovery
    pub mtls_enabled: bool,
}

/// Handoff retry specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HandoffRetryConfig {
    /// Maximum handoff attempts
    pub max_attempts: u32,

    /// Handoff timeout
    pub timeout: Duration,

    /// Enable circuit breaker
    pub circuit_breaker_enabled: bool,

    /// Circuit breaker threshold
    pub circuit_breaker_threshold: u32,
}

/// Mesh security specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MeshSecurityConfig {
    /// Enable TLS
    pub tls_enabled: bool,

    /// TLS version (Arc for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub tls_version: Arc<str>,

    /// Certificate path
    pub cert_path: Option<String>,

    /// Key path
    pub key_path: Option<String>,

    /// CA path
    pub ca_path: Option<String>,
}

/// Health monitoring specific configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HealthMonitorConfig {
    /// Health check interval
    pub check_interval: Duration,

    /// Health check timeout
    pub check_timeout: Duration,

    /// Enable predictive health monitoring
    pub predictive_enabled: bool,

    /// Health check endpoints
    pub endpoints: Vec<String>,
}

impl Default for MeshDiscoveryConfig {
    fn default() -> Self {
        Self {
            protocol: Arc::from(
                std::env::var("BEARDOG_MESH_PROTOCOL")
                    .unwrap_or_else(|_| "https".to_string())
                    .as_str(),
            ),
            port: std::env::var("BEARDOG_MESH_DISCOVERY_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8443),
            timeout: Duration::from_secs(
                std::env::var("BEARDOG_MESH_DISCOVERY_TIMEOUT_SECS")
                    .ok()
                    .and_then(|t| t.parse().ok())
                    .unwrap_or(5),
            ),
            mtls_enabled: true,
        }
    }
}

impl Default for HandoffRetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: std::env::var("BEARDOG_HANDOFF_RETRY_MAX_ATTEMPTS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            timeout: Duration::from_secs(
                std::env::var("BEARDOG_HANDOFF_RETRY_TIMEOUT_SECS")
                    .ok()
                    .and_then(|t| t.parse().ok())
                    .unwrap_or(10),
            ),
            circuit_breaker_enabled: true,
            circuit_breaker_threshold: std::env::var("BEARDOG_ADAPTER_CIRCUIT_BREAKER_THRESHOLD")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
        }
    }
}

impl Default for MeshSecurityConfig {
    fn default() -> Self {
        Self {
            tls_enabled: true,
            tls_version: Arc::from("1.3"),
            cert_path: None,
            key_path: None,
            ca_path: None,
        }
    }
}

impl Default for HealthMonitorConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(
                std::env::var("BEARDOG_HEALTH_CHECK_INTERVAL_SECS")
                    .ok()
                    .and_then(|t| t.parse().ok())
                    .unwrap_or(30),
            ),
            check_timeout: Duration::from_secs(
                std::env::var("BEARDOG_HEALTH_CHECK_TIMEOUT_SECS")
                    .ok()
                    .and_then(|t| t.parse().ok())
                    .unwrap_or(5),
            ),
            predictive_enabled: false,
            endpoints: vec!["/health".to_string()],
        }
    }
}
