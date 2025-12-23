// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

pub mod observability;

pub use observability::{
    Alert, AlertSeverity, MetricsSummary,
    PerformanceSummary, ProductionObservability, SLAStatus,
    SystemHealth,
};

// Define ObservabilityConfig locally since path is not found
#[derive(Debug, Clone)]
pub struct ObservabilityConfig {
    pub metrics_enabled: bool,
    pub health_checks_enabled: bool,
    pub performance_monitoring_enabled: bool,
    pub collection_interval_seconds: u64,
}

// Define ProductionReadinessReport locally since it's not found in beardog_types
#[derive(Debug, Clone)]
pub struct ProductionReadinessReport {
    pub overall_status: String,
    pub readiness_score: f64,
    pub components_ready: u32,
    pub components_total: u32,
    pub timestamp: std::time::SystemTime,
}

// BusinessOperation will be defined locally if needed
#[derive(Debug, Clone)]
pub struct BusinessOperation {
    pub operation_id: String,
    pub operation_type: String,
    pub status: String,
    pub duration_ms: u64,
}
