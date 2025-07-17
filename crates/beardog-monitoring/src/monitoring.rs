//! Monitoring module for BearDog system health and metrics
//!
//! This module provides comprehensive monitoring capabilities including:
//! - System health checking with component-level monitoring
//! - Performance metrics collection and alerting
//! - Native Rust metrics (always free under AGPL)
//! - Licensed Prometheus export for enterprise integrations
//!
//! ## Module Structure
//!
//! - `types`: Core monitoring types and data structures
//! - `health`: Health checking functionality and trait definitions
//! - `service`: Main monitoring service implementation
//! - `metrics`: Metrics collection and export system

pub mod health;
pub mod metrics;
pub mod service;
pub mod types;

// Re-export commonly used types and traits
pub use types::{
    AlertThresholds, ComponentHealth, HealthStatus, InternalMetricsSummary, MetricValue,
    PerformanceMetrics, PrometheusConfig, ResourceMetrics, SystemHealth, SystemMetrics,
};

pub use health::{
    DatabaseHealthChecker, ExternalServiceHealthChecker, HealthChecker, RedisHealthChecker,
};

pub use service::MonitoringService;

pub use metrics::{InternalMetricsCollector, MetricsService};
