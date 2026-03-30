// SPDX-License-Identifier: AGPL-3.0-only

// Unified Monitoring Configuration System
//
// This module consolidates ALL monitoring configuration patterns from across the BearDog
// ecosystem into a single, canonical, maintainable system. It replaces fragmented configs
// from multiple locations.
//
// ## Consolidation Strategy
//
// This unifies monitoring configs from:
// - `beardog-types/src/canonical/monitoring.rs` (15+ config structs)
// - `beardog-monitoring/src/advanced_metrics.rs` (5+ config structs)
// - `beardog-monitoring/src/improved_monitoring.rs` (3+ config structs)
// - Various monitoring configs scattered across other crates
//
// ## Architecture Principles
//
// - **Single Source of Truth**: All monitoring config in one canonical place
// - **Domain Organization**: Logical grouping by monitoring concern
// - **Zero Fragmentation**: No duplicate monitoring config types
// - **Extensible Design**: Easy addition of new monitoring capabilities
// - **Performance Optimized**: Efficient configuration loading and validation

use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
// Removed unused import: use std::time::Duration;

// Domain-specific monitoring configuration modules
/// Alerting module
pub mod alerting;
/// Analytics module
pub mod analytics;
/// Core module
/// Core functionality
pub mod core;
/// Health module
pub mod health;
/// Integration module
pub mod integration;
/// Logging module
pub mod logging;
/// Metrics module
pub mod metrics;
/// Performance telemetry: latency, throughput, and resource saturation tied to monitoring.
pub mod performance;
/// Security module
pub mod security;
/// Tracing module
pub mod tracing;

// Re-export all configuration types for easy access
// Note: Some type names are intentionally duplicated across modules for flexibility
#[allow(ambiguous_glob_reexports)]
pub use alerting::*;
#[allow(ambiguous_glob_reexports)]
pub use analytics::*;
#[allow(ambiguous_glob_reexports)]
pub use core::*;
#[allow(ambiguous_glob_reexports)]
pub use health::*;
#[allow(ambiguous_glob_reexports)]
pub use integration::*;
#[allow(ambiguous_glob_reexports)]
pub use logging::*;
#[allow(ambiguous_glob_reexports)]
pub use metrics::*;
#[allow(ambiguous_glob_reexports)]
pub use performance::*;
#[allow(ambiguous_glob_reexports)]
pub use security::*;
#[allow(ambiguous_glob_reexports)]
pub use tracing::*;

/// Canonical Monitoring Configuration
///
/// **THE** single source of truth for all monitoring configuration in `BearDog`.
/// This configuration consolidates all monitoring concerns into a unified, hierarchical
/// structure that eliminates fragmentation while maintaining domain organization.
///
/// Replaces: `UnifiedMonitoringConfig`, `CanonicalMonitoringConfig`, and all duplicate `MonitoringConfig` instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// **GLOBAL SETTINGS**
    /// Whether monitoring is globally enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// The environment value
    pub environment: MonitoringEnvironment,
    /// Global tags applied to all monitoring data
    /// Mapping of global tags
    pub global_tags: HashMap<String, String>,

    /// **CORE MONITORING DOMAINS**
    /// Health check and wellness monitoring configuration
    /// The health value
    pub health: UnifiedHealthConfig,
    /// Metrics collection and reporting configuration
    /// The metrics value
    pub metrics: UnifiedMetricsConfig,
    /// Alert management and notification configuration
    /// The alerting value
    pub alerting: UnifiedAlertingConfig,
    /// Logging configuration and management
    /// The logging value
    pub logging: UnifiedLoggingConfig,
    /// Distributed tracing configuration
    /// The tracing value
    pub tracing: UnifiedTracingConfig,

    /// **SPECIALIZED MONITORING**
    /// The security value
    pub security: UnifiedSecurityMonitoringConfig,
    /// Performance signals (latency histograms, saturation). **Default:** [`UnifiedPerformanceMonitoringConfig::default()`].
    pub performance: UnifiedPerformanceMonitoringConfig,
    /// Integration
    /// The integration value
    pub integration: UnifiedIntegrationMonitoringConfig,
    /// Analytics
    /// The analytics value
    pub analytics: UnifiedAnalyticsConfig,

    /// **EXPORT AND INTEGRATION**
    /// The exporters value
    pub exporters: MonitoringExportersConfig,
    /// Dashboards
    /// The dashboards value
    pub dashboards: DashboardConfig,
    /// Notifications
    /// The notifications value
    pub notifications: NotificationConfig,
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            environment: MonitoringEnvironment::Development,
            global_tags: HashMap::new(),
            health: UnifiedHealthConfig::default(),
            metrics: UnifiedMetricsConfig::default(),
            alerting: UnifiedAlertingConfig::default(),
            logging: UnifiedLoggingConfig::default(),
            tracing: UnifiedTracingConfig::default(),
            security: UnifiedSecurityMonitoringConfig::default(),
            performance: UnifiedPerformanceMonitoringConfig::default(),
            integration: UnifiedIntegrationMonitoringConfig::default(),
            analytics: UnifiedAnalyticsConfig::default(),
            exporters: MonitoringExportersConfig::default(),
            dashboards: DashboardConfig::default(),
            notifications: NotificationConfig::default(),
        }
    }
}

/// Monitoring environment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonitoringEnvironment {
    /// Development variant
    Development,
    /// Testing variant
    Testing,
    /// Staging variant
    Staging,
    /// Production variant
    Production,
    /// Custom environment with specified name
    Custom(String),
}

/// Monitoring exporters configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MonitoringExportersConfig {
    /// Prometheus
    /// The prometheus value
    pub prometheus: PrometheusExporterConfig,
    /// Grafana
    /// The grafana value
    pub grafana: GrafanaExporterConfig,
    /// Jaeger
    /// The jaeger value
    pub jaeger: JaegerExporterConfig,
    /// Custom
    /// Mapping of custom
    pub custom: HashMap<String, serde_json::Value>,
}

/// Prometheus exporter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusExporterConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Endpoint
    /// The endpoint value
    pub endpoint: String,
    /// Port
    /// Number of port
    pub port: u16,
    /// Metrics Path
    /// The metrics path value
    pub metrics_path: String,
    /// Push Gateway
    /// Optional push gateway
    pub push_gateway: Option<String>,
}

impl Default for PrometheusExporterConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint: std::env::var("BEARDOG_PROMETHEUS_ENDPOINT")
                .or_else(|_| std::env::var("BEARDOG_BIND_ADDRESS"))
                .unwrap_or_else(|_| "0.0.0.0".to_string()), // Standard bind-to-all-interfaces
            port: std::env::var("BEARDOG_PROMETHEUS_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(beardog_config::domains::network_ports::DEFAULT_METRICS_PORT),
            metrics_path: "/metrics".to_string(),
            push_gateway: None,
        }
    }
}

/// Grafana exporter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrafanaExporterConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Url
    /// The url value
    pub url: String,
    /// Api Key
    /// Optional api key
    pub api_key: Option<String>,
    /// Dashboard Config
    pub dashboard_config: HashMap<String, serde_json::Value>,
}

impl Default for GrafanaExporterConfig {
    fn default() -> Self {
        use crate::canonical::config::network::NetworkConfig;
        let network_config = NetworkConfig::default();

        Self {
            enabled: false,
            url: std::env::var("BEARDOG_GRAFANA_URL")
                .or_else(|_| std::env::var("GRAFANA_URL"))
                .unwrap_or_else(|_| {
                    format!(
                        "http://{}:{}",
                        network_config.default_host,
                        beardog_config::domains::network_ports::DEFAULT_GRAFANA_PORT
                    )
                }),
            api_key: None,
            dashboard_config: HashMap::new(),
        }
    }
}

/// Jaeger exporter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JaegerExporterConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Endpoint
    /// The endpoint value
    pub endpoint: String,
    /// Service Name
    /// Name of the service
    pub service_name: String,
    /// Sampling Rate
    /// The sampling rate value
    pub sampling_rate: f64,
}

impl Default for JaegerExporterConfig {
    fn default() -> Self {
        use crate::canonical::config::network::NetworkConfig;
        let network_config = NetworkConfig::default();

        Self {
            enabled: false,
            endpoint: std::env::var("BEARDOG_JAEGER_ENDPOINT")
                .or_else(|_| std::env::var("JAEGER_ENDPOINT"))
                .unwrap_or_else(|_| {
                    format!("http://{}:14268/api/traces", network_config.default_host)
                }),
            service_name: "beardog ".to_string(),
            sampling_rate: 0.1,
        }
    }
}

/// Dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Auto Create
    /// Whether `auto_create` is enabled
    pub auto_create: bool,
    /// Templates
    /// Collection of templates
    pub templates: Vec<String>,
    /// Custom Panels
    /// Mapping of custom panels
    pub custom_panels: HashMap<String, serde_json::Value>,
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            auto_create: true,
            templates: vec!["system".to_string(), "application".to_string()],
            custom_panels: HashMap::new(),
        }
    }
}

/// Notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationConfig {
    /// Enabled
    /// Whether feature is enabled
    pub enabled: bool,
    /// Channels
    /// Collection of channels
    pub channels: Vec<NotificationChannel>,
    /// Rate Limiting
    /// The rate limiting value
    pub rate_limiting: crate::canonical::config::domains::network::RateLimitConfig,
}

impl Default for NotificationConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            channels: Vec::new(),
            rate_limiting: crate::canonical::config::domains::network::RateLimitConfig::default(),
        }
    }
}

/// Notification channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannel {
    /// Name
    /// Name of the item
    pub name: String,
    /// Channel Type
    /// The channel type value
    pub channel_type: NotificationChannelType,
    /// Config
    pub config: HashMap<String, serde_json::Value>,
    /// Filters
    /// Collection of filters
    pub filters: Vec<NotificationFilter>,
}

/// Notification channel types
#[derive(Debug, Clone, Serialize, Deserialize)]
/// Types of notification channel
pub enum NotificationChannelType {
    /// Email variant
    Email,
    /// Slack variant
    Slack,
    /// Discord variant
    Discord,
    /// Webhook variant
    Webhook,
    /// Custom notification provider with specified name
    Custom(String),
}

/// Notification filter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationFilter {
    /// Field
    /// The field value
    pub field: String,
    /// Operator
    /// The operator value
    pub operator: FilterOperator,
    /// Value
    /// The value value
    pub value: String,
}

/// Filter operators
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    /// Equals variant
    Equals,
    /// `NotEquals` variant
    NotEquals,
    /// Contains variant
    Contains,
    /// `NotContains` variant
    NotContains,
    /// `GreaterThan` variant
    GreaterThan,
    /// `LessThan` variant
    LessThan,
}

/// Rate limiting configuration for monitoring (DEPRECATED - use canonical)
///
/// **MIGRATION**: Use `crate::canonical::config::domains::network::RateLimitConfig` instead.
///
/// This type alias will be removed in v3.3.0.
#[deprecated(
    since = "3.1.0",
    note = "Use crate::canonical::config::domains::network::RateLimitConfig instead"
)]
pub type RateLimitConfig = crate::canonical::config::domains::network::RateLimitConfig;

/// Configuration validation trait
pub trait MonitoringConfigValidation {
    /// Validate the monitoring configuration
    /// Validates input
    ///
    /// # Errors
    ///
    /// Returns an error if any subsection (health, metrics, alerting, logging, etc.) fails validation.
    fn validate(&self) -> Result<(), BearDogError>;

    /// Check configuration compatibility
    /// Checks if compatible with
    fn is_compatible_with(&self, other_version: u32) -> bool;
}

impl MonitoringConfigValidation for MonitoringConfig {
    /// Validates input
    fn validate(&self) -> Result<(), BearDogError> {
        // Validate all sub-configurations
        self.health.validate()?;
        self.metrics.validate()?;
        self.alerting.validate()?;
        self.logging.validate()?;
        self.tracing.validate()?;
        self.security.validate()?;
        self.performance.validate()?;
        self.integration.validate()?;
        self.analytics.validate()?;

        Ok(())
    }

    /// Checks if compatible with
    fn is_compatible_with(&self, _other_version: u32) -> bool {
        // Version compatibility logic
        true
    }
}
