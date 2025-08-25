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


/// # Canonical Monitoring Configuration
///
/// **MONITORING CONFIGURATION UNIFICATION** ✅
/// This module consolidates ALL monitoring configuration types from across the codebase:
/// 
/// ## **Fragmentation Eliminated:**
/// - `beardog-monitoring/src/types.rs` - MetricCollectionConfig, AlertProcessingConfig
/// - `beardog-monitoring/src/improved_monitoring.rs` - MonitoringConfig, MetricCollectionConfig
/// - `beardog-monitoring/src/monitoring/service.rs` - MonitoringConfig
/// - `beardog-monitoring/src/monitoring/types.rs` - PrometheusConfig
/// - `beardog-monitoring/src/monitoring/metrics.rs` - PrometheusConfig
/// - `beardog-types/src/monitoring/mod.rs` - HealthCheckConfig
/// - `beardog-types/src/canonical/monitoring.rs` - HealthCheckConfig, AlertConfig
/// - `beardog-types/src/canonical/network.rs` - HealthCheckConfig (duplicate)
/// 
/// ## **Design Benefits:**
/// - **Single Source of Truth**: All monitoring configs defined once
/// - **Hierarchical Organization**: Logical grouping by monitoring domain
/// - **Zero Duplication**: Eliminates 10+ duplicate configuration structs
/// - **Canonical Access**: `use beardog_types::canonical::configuration::monitoring::*`

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// ============================================================================
// UNIFIED MONITORING CONFIGURATION
// ============================================================================

/// **CANONICAL MONITORING CONFIGURATION** - Main monitoring settings
/// Consolidates all monitoring configurations from across the codebase
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct UnifiedMonitoringConfig {
    /// Health check configuration
    pub health_checks: HealthCheckConfig,
    /// Metrics collection configuration
    pub metrics: MetricCollectionConfig,
    /// Alert processing configuration
    pub alerts: AlertProcessingConfig,
    /// Prometheus integration configuration
    pub prometheus: PrometheusConfig,
    /// Security monitoring configuration
    pub security_monitoring: SecurityMonitoringConfig,
    /// Performance monitoring configuration
    pub performance_monitoring: PerformanceMonitoringConfig,
}

/// **CANONICAL HEALTH CHECK CONFIGURATION**
/// Consolidates HealthCheckConfig from multiple locations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    /// Health check interval
    pub interval: Duration,
    /// Health check timeout
    pub timeout: Duration,
    /// Number of failed checks before marking as unhealthy
    pub failure_threshold: u32,
    /// Number of successful checks before marking as healthy
    pub success_threshold: u32,
    /// Health check endpoints
    pub endpoints: Vec<String>,
    /// Custom health check parameters
    pub custom_checks: HashMap<String, serde_json::Value>,
}

/// **CANONICAL METRIC COLLECTION CONFIGURATION**
/// Consolidates MetricCollectionConfig from multiple locations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricCollectionConfig {
    /// Metric collection interval
    pub collection_interval: Duration,
    /// Metric retention period
    pub retention_period: Duration,
    /// Maximum number of metrics to store
    pub max_metrics: u64,
    /// Metric aggregation settings
    pub aggregation: MetricAggregationConfig,
    /// Custom metric collectors
    pub custom_collectors: HashMap<String, serde_json::Value>,
}

/// **CANONICAL ALERT PROCESSING CONFIGURATION**
/// Consolidates AlertProcessingConfig from multiple locations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertProcessingConfig {
    /// Alert processing interval
    pub processing_interval: Duration,
    /// Alert escalation timeout
    pub escalation_timeout: Duration,
    /// Maximum number of alerts to process per batch
    pub max_batch_size: u32,
    /// Alert routing rules
    pub routing_rules: Vec<AlertRoutingRule>,
    /// Notification channels
    pub notification_channels: HashMap<String, NotificationChannelConfig>,
}

/// **CANONICAL PROMETHEUS CONFIGURATION**
/// Consolidates PrometheusConfig from multiple locations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrometheusConfig {
    /// Prometheus server endpoint
    pub endpoint: String,
    /// Prometheus scrape interval
    pub scrape_interval: Duration,
    /// Prometheus query timeout
    pub query_timeout: Duration,
    /// Prometheus retention period
    pub retention_period: Duration,
    /// Custom Prometheus configuration
    pub custom_config: HashMap<String, serde_json::Value>,
}

/// **CANONICAL SECURITY MONITORING CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMonitoringConfig {
    /// Enable security event monitoring
    pub enabled: bool,
    /// Security event collection interval
    pub collection_interval: Duration,
    /// Security alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
    /// Threat detection configuration
    pub threat_detection: ThreatDetectionConfig,
}

/// **CANONICAL PERFORMANCE MONITORING CONFIGURATION**
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMonitoringConfig {
    /// Enable performance monitoring
    pub enabled: bool,
    /// Performance metric collection interval
    pub collection_interval: Duration,
    /// Performance baseline configuration
    pub baseline_config: PerformanceBaselineConfig,
    /// Performance alert thresholds
    pub alert_thresholds: HashMap<String, f64>,
}

// ============================================================================
// SUPPORTING CONFIGURATION TYPES
// ============================================================================

/// Metric aggregation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricAggregationConfig {
    /// Aggregation method (sum, avg, max, min)
    pub method: String,
    /// Aggregation window size
    pub window_size: Duration,
    /// Enable downsampling
    pub enable_downsampling: bool,
}

/// Alert routing rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRoutingRule {
    /// Rule name
    pub name: String,
    /// Rule condition
    pub condition: String,
    /// Target notification channel
    pub channel: String,
    /// Rule priority
    pub priority: u32,
}

/// Notification channel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationChannelConfig {
    /// Channel type (email, slack, webhook, etc.)
    pub channel_type: String,
    /// Channel endpoint
    pub endpoint: String,
    /// Channel configuration
    pub config: HashMap<String, serde_json::Value>,
}

/// Threat detection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionConfig {
    /// Enable ML-based threat detection
    pub enable_ml_detection: bool,
    /// Threat detection sensitivity
    pub sensitivity: f64,
    /// Threat detection rules
    pub rules: Vec<ThreatDetectionRule>,
}

/// Performance baseline configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBaselineConfig {
    /// Baseline calculation window
    pub calculation_window: Duration,
    /// Baseline update interval
    pub update_interval: Duration,
    /// Baseline deviation threshold
    pub deviation_threshold: f64,
}

/// Threat detection rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetectionRule {
    /// Rule name
    pub name: String,
    /// Rule pattern
    pub pattern: String,
    /// Rule severity
    pub severity: String,
    /// Rule action
    pub action: String,
}

// ============================================================================
// DEFAULT IMPLEMENTATIONS
// ============================================================================


impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            failure_threshold: 3,
            success_threshold: 2,
            endpoints: vec!["/health".to_string()],
            custom_checks: HashMap::new(),
        }
    }
}

impl Default for MetricCollectionConfig {
    fn default() -> Self {
        Self {
            collection_interval: Duration::from_secs(60),
            retention_period: Duration::from_secs(86400), // 24 hours
            max_metrics: 100000,
            aggregation: MetricAggregationConfig::default(),
            custom_collectors: HashMap::new(),
        }
    }
}

impl Default for AlertProcessingConfig {
    fn default() -> Self {
        Self {
            processing_interval: Duration::from_secs(30),
            escalation_timeout: Duration::from_secs(300), // 5 minutes
            max_batch_size: 100,
            routing_rules: Vec::new(),
            notification_channels: HashMap::new(),
        }
    }
}

impl Default for PrometheusConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:9090".to_string(),
            scrape_interval: Duration::from_secs(15),
            query_timeout: Duration::from_secs(30),
            retention_period: Duration::from_secs(86400 * 7), // 7 days
            custom_config: HashMap::new(),
        }
    }
}

impl Default for SecurityMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(10),
            alert_thresholds: HashMap::new(),
            threat_detection: ThreatDetectionConfig::default(),
        }
    }
}

impl Default for PerformanceMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(60),
            baseline_config: PerformanceBaselineConfig::default(),
            alert_thresholds: HashMap::new(),
        }
    }
}

impl Default for MetricAggregationConfig {
    fn default() -> Self {
        Self {
            method: "avg".to_string(),
            window_size: Duration::from_secs(300), // 5 minutes
            enable_downsampling: true,
        }
    }
}

impl Default for ThreatDetectionConfig {
    fn default() -> Self {
        Self {
            enable_ml_detection: true,
            sensitivity: 0.8,
            rules: Vec::new(),
        }
    }
}

impl Default for PerformanceBaselineConfig {
    fn default() -> Self {
        Self {
            calculation_window: Duration::from_secs(3600), // 1 hour
            update_interval: Duration::from_secs(86400), // 24 hours
            deviation_threshold: 2.0, // 2 standard deviations
        }
    }
} 