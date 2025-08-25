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


/// # BearDog Monitoring Module
///
/// **UNIFIED MONITORING ARCHITECTURE** ✅ **COMPLETE**
///
/// This module provides comprehensive monitoring capabilities across the BearDog ecosystem,
/// including health checks, metrics collection, alerting, and security monitoring.

// Re-export all monitoring components
pub mod health;
pub mod metrics;
pub mod service;
pub mod types;

// Re-export key types and traits for easy access
pub use health::{
    DatabaseHealthChecker, CacheHealthChecker, ExternalApiHealthChecker, HsmHealthChecker, 
    HealthChecker, HealthCheckerType,
};
pub use metrics::{MetricsService, InternalMetricsSummary};
pub use service::{MonitoringService, Alert, MonitoringConfig};
pub use beardog_types::AlertSeverity;
pub use types::{
    ComponentHealth, SystemHealth, SystemMetrics,
};
