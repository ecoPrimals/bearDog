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


/// # Monitoring Configuration - Canonical
///
/// **UNIFIED MONITORING CONFIGURATION** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// **CANONICAL** Basic Monitoring Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicMonitoringConfig {
    pub enabled: bool,
    pub metrics_interval: Duration,
    pub log_level: String,
    pub enable_tracing: bool,
}

impl Default for BasicMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            metrics_interval: Duration::from_secs(60),
            log_level: "info".to_string(),
            enable_tracing: true,
        }
    }
}

/// **CANONICAL** Production Monitoring Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Default)]
pub struct ProductionMonitoringConfig {
    pub metrics: MetricsConfig,
    pub alerting: AlertingConfig,
    pub health_checks: HealthCheckConfig,
    pub profiling: ProfilingConfig,
    pub sla_monitoring: SlaMonitoringConfig,
}


/// **CANONICAL** Metrics Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub collection_interval: Duration,
    pub retention_period: Duration,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(30),
            retention_period: Duration::from_secs(86400 * 7), // 7 days
        }
    }
}

/// **CANONICAL** Alerting Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertingConfig {
    pub enabled: bool,
    pub notification_channels: Vec<String>,
    pub alert_thresholds: HashMap<String, f64>,
}

impl Default for AlertingConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            notification_channels: vec!["email".to_string()],
            alert_thresholds: HashMap::new(),
        }
    }
}

/// **CANONICAL** Health Check Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub enabled: bool,
    pub check_interval: Duration,
    pub timeout: Duration,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
        }
    }
}

/// **CANONICAL** Profiling Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilingConfig {
    pub enabled: bool,
    pub sampling_rate: f64,
    pub profile_duration: Duration,
}

impl Default for ProfilingConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sampling_rate: 0.1, // 10%
            profile_duration: Duration::from_secs(60),
        }
    }
}

/// **CANONICAL** SLA Monitoring Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlaMonitoringConfig {
    pub enabled: bool,
    pub targets: HashMap<String, f64>,
    pub response_time_threshold: f64,
    pub availability_threshold: f64,
    pub error_rate_threshold: f64,
}

impl Default for SlaMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            targets: HashMap::new(),
            response_time_threshold: 1000.0, // 1 second
            availability_threshold: 99.9,    // 99.9%
            error_rate_threshold: 5.0,       // 5%
        }
    }
}
