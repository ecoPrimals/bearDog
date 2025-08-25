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


/// # Integration Monitoring Configuration - Canonical
///
/// **UNIFIED INTEGRATION MONITORING CONFIGURATION** for the BearDog ecosystem

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// **CANONICAL** Integration Monitoring Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationMonitoringConfig {
    pub enabled: bool,
    pub collection_interval: Duration,
    pub retention_period: Duration,
    pub alert_thresholds: AlertThresholds,
}

impl Default for IntegrationMonitoringConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            collection_interval: Duration::from_secs(60),
            retention_period: Duration::from_secs(86400 * 7), // 7 days
            alert_thresholds: AlertThresholds::default(),
        }
    }
}

/// **CANONICAL** Performance Thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceThresholds {
    pub response_time_ms: u32,
    pub error_rate_percent: f64,
    pub throughput_per_second: u32,
}

impl Default for PerformanceThresholds {
    fn default() -> Self {
        Self {
            response_time_ms: 1000,
            error_rate_percent: 5.0,
            throughput_per_second: 100,
        }
    }
}

/// **CANONICAL** Integration Audit Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationAuditConfig {
    pub enabled: bool,
    pub log_requests: bool,
    pub include_payload: bool,
}

impl Default for IntegrationAuditConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            log_requests: true,
            include_payload: false,
        }
    }
}

/// **CANONICAL** Alert Thresholds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertThresholds {
    pub error_count_threshold: u32,
    pub latency_threshold_ms: u32,
    pub availability_threshold_percent: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            error_count_threshold: 10,
            latency_threshold_ms: 2000,
            availability_threshold_percent: 99.0,
        }
    }
}
