// SPDX-License-Identifier: AGPL-3.0-or-later

// Production monitoring and alerting system
//
// This module provides comprehensive production monitoring capabilities including
// real-time metrics collection, alerting, performance analysis, and system health monitoring.
// All monitoring operates with sovereignty compliance and zero hardcoded assumptions.

mod collectors;
mod config;
mod data;

pub use collectors::{AlertManager, PerformanceMonitor, SystemMetricsCollector, SystemMonitor};
pub use config::{CanonicalMonitoringConfig, MonitoringConfig, PerformanceConfig, SystemConfig};
pub use data::{
    Alert, AlertSeverity, AlertStatus, AlertThresholds, HealthStatus, MetricType, MetricsSummary,
    OperationStatus, PerformanceSummary, ResourceUtilization, SystemMetrics, SystemOverview,
};
