// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// # Canonical Provider Configuration Module
///
/// This module provides canonical configuration types for providers,
/// external services, monitoring, and related settings.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **CANONICAL PROVIDER STATUS** - Provider operational state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProviderStatus {
    Active,
    Inactive,
    Error,
    Maintenance,
}

/// **CANONICAL PROVIDER CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Provider identifier
    pub provider_id: String,
    /// Provider type
    pub provider_type: String,
    /// Provider endpoint
    pub endpoint: Option<String>,
    /// Provider configuration parameters
    pub config_params: HashMap<String, serde_json::Value>,
    /// Whether provider is enabled
    pub enabled: bool,
    /// Provider timeout in seconds
    pub timeout_seconds: u64,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Retry delay in milliseconds
    pub retry_delay_ms: u64,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            provider_id: "default".to_string(),
            provider_type: "generic".to_string(),
            endpoint: None,
            config_params: HashMap::new(),
            enabled: true,
            timeout_seconds: 30,
            max_retries: 3,
            retry_delay_ms: 1000,
        }
    }
}

/// **CANONICAL PROVIDER DEFAULTS**
pub struct ProviderDefaults {
    /// Default timeout
    pub timeout: Duration,
    /// Default retries
    pub retries: u32,
    /// Default retry delay
    pub retry_delay: Duration,
}

/// **CANONICAL EXTERNAL SERVICES CONFIGURATION** - External service endpoints
pub struct ExternalServicesConfig {
    /// SongBird service endpoint
    pub songbird_endpoint: Option<String>,
    /// Storage service endpoint
    pub storage_endpoint: Option<String>,
    /// Authentication service endpoint
    pub auth_endpoint: Option<String>,
    /// Monitoring service endpoint
    pub monitoring_endpoint: Option<String>,
}

/// **CANONICAL MONITORING CONFIGURATION** - Monitoring and observability settings
pub struct MonitoringConfig {
    /// Whether monitoring is enabled
    pub enabled: bool,
    /// Metrics collection settings
    pub metrics: MetricsConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
    /// Tracing configuration
    pub tracing: TracingConfig,
}

/// **CANONICAL METRICS CONFIGURATION**
pub struct MetricsConfig {
    /// Whether metrics collection is enabled
    pub enabled: bool,
    /// Metrics endpoint
    pub endpoint: String,
    /// Collection interval
    pub interval: Duration,
}

/// **CANONICAL LOGGING CONFIGURATION**
pub struct LoggingConfig {
    /// Log level
    pub level: String,
    /// Log format
    pub format: String,
    /// Log output destination
    pub output: String,
}

/// **CANONICAL TRACING CONFIGURATION**
pub struct TracingConfig {
    /// Whether tracing is enabled
    pub enabled: bool,
    /// Tracing endpoint
    pub endpoint: String,
    /// Sample rate (0.0 to 1.0)
    pub sample_rate: f64,
}
