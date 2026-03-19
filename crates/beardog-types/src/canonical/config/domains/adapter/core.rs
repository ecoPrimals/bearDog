// SPDX-License-Identifier: AGPL-3.0-only

//! Core adapter configuration types and implementations

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

/// Core adapter configuration settings
///
/// ## Performance Note
/// Uses `Arc<str>` for `adapter_id` to enable fast, cheap cloning (10x faster).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoreAdapterConfig {
    /// Adapter identifier (Arc for fast cloning)
    #[serde(
        serialize_with = "crate::canonical::config::utils::serialize_arc_str",
        deserialize_with = "crate::canonical::config::utils::deserialize_arc_str"
    )]
    pub adapter_id: Arc<str>,

    /// Adapter type (primal, universal, capability, etc.)
    pub adapter_type: AdapterType,

    /// Maximum concurrent connections
    pub max_connections: usize,

    /// Connection timeout
    pub connection_timeout: Duration,

    /// Enable adapter registry
    pub registry_enabled: bool,

    /// Adapter metadata
    pub metadata: HashMap<String, String>,
}

/// Adapter type enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdapterType {
    /// Primal-specific adapter
    Primal,
    /// Universal adapter for cross-primal communication
    Universal,
    /// Capability-based adapter
    Capability,
    /// Vendor-specific adapter
    Vendor,
    /// Service mesh adapter
    ServiceMesh,
    /// Custom adapter type
    Custom(String),
}

/// Authorization level enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthLevel {
    /// No authentication required
    None,
    /// Basic authentication
    Basic,
    /// Token-based authentication
    Token,
    /// Certificate-based authentication
    Certificate,
    /// Multi-factor authentication
    MultiFactor,
}

/// Performance optimization configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OptimizationConfig {
    /// Enable performance optimization
    pub enabled: bool,

    /// Optimization level (1-5)
    pub level: u8,

    /// Enable SIMD optimizations
    pub simd_enabled: bool,

    /// Enable zero-copy optimizations
    pub zero_copy_enabled: bool,

    /// Buffer size for optimizations
    pub buffer_size: usize,

    /// Enable adaptive optimization
    pub adaptive_enabled: bool,

    /// Optimization metrics collection
    pub metrics_enabled: bool,
}

impl CoreAdapterConfig {
    /// Default adapter ID
    pub const DEFAULT_ADAPTER_ID: &'static str = "default-adapter";

    /// Default maximum connections
    pub const DEFAULT_MAX_CONNECTIONS: usize = 100;

    /// Default connection timeout in seconds
    pub const DEFAULT_CONNECTION_TIMEOUT_SECS: u64 = 30;

    /// Create CoreAdapterConfig with hardcoded defaults
    ///
    /// This method is deterministic and safe for concurrent use.
    /// No environment variables are read.
    pub fn with_defaults() -> Self {
        Self {
            adapter_id: Arc::from(Self::DEFAULT_ADAPTER_ID),
            adapter_type: AdapterType::Universal,
            max_connections: Self::DEFAULT_MAX_CONNECTIONS,
            connection_timeout: Duration::from_secs(Self::DEFAULT_CONNECTION_TIMEOUT_SECS),
            registry_enabled: true,
            metadata: HashMap::new(),
        }
    }

    /// Create CoreAdapterConfig from environment variables
    ///
    /// Reads configuration from environment, falling back to defaults.
    /// This makes the intent explicit and allows testing without env pollution.
    pub fn from_env() -> Self {
        Self {
            adapter_id: Arc::from(
                std::env::var("BEARDOG_ADAPTER_ID")
                    .unwrap_or_else(|_| Self::DEFAULT_ADAPTER_ID.to_string())
                    .as_str(),
            ),
            adapter_type: AdapterType::Universal,
            max_connections: std::env::var("BEARDOG_ADAPTER_MAX_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(Self::DEFAULT_MAX_CONNECTIONS),
            connection_timeout: Duration::from_secs(
                std::env::var("BEARDOG_ADAPTER_CONNECTION_TIMEOUT_SECS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(Self::DEFAULT_CONNECTION_TIMEOUT_SECS),
            ),
            registry_enabled: true,
            metadata: HashMap::new(),
        }
    }
}

impl Default for CoreAdapterConfig {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            enabled: std::env::var("BEARDOG_ADAPTER_OPTIMIZATION_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            level: std::env::var("BEARDOG_ADAPTER_OPTIMIZATION_LEVEL")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            simd_enabled: std::env::var("BEARDOG_ADAPTER_SIMD_ENABLED")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(true),
            zero_copy_enabled: true,
            buffer_size: std::env::var("BEARDOG_OPTIMIZATION_BUFFER_SIZE")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(8192),
            adaptive_enabled: true,
            metrics_enabled: true,
        }
    }
}
