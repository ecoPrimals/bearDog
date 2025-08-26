

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZeroCostHsmHealthReport {
    pub overall_health: HealthStatus,
    pub providers: Vec<HsmProviderHealth>,
    pub system_metrics: SystemMetrics,
    pub generated_at: DateTime<Utc>,
}

pub use beardog_types::canonical::HealthStatus;

pub struct HsmProviderHealth {
    pub provider_id: String,
    pub status: HealthStatus,
    pub response_time_ms: f64,
    pub error_rate: f64,
    pub operations_count: u64,
    pub last_successful_operation: Option<DateTime<Utc>>,

pub struct SystemMetrics {
    pub total_operations: u64,
    pub successful_operations: u64,
    pub failed_operations: u64,
    pub average_response_time_ms: f64,
    pub peak_operations_per_second: u64,
    pub memory_usage_mb: f64,
    pub cpu_utilization_percent: f64,

pub struct ZeroCostHsmManagerStats {
    pub active_providers: usize,
    pub total_keys: usize,
    pub operations_by_type: HashMap<String, u64>,
    pub failover_events: u64,
    pub load_balancing_decisions: u64,
    pub uptime_seconds: u64,

pub struct HsmMetricsCollector {
    operation_counter: AtomicU64,
    error_counter: AtomicU64,
    total_response_time_ms: AtomicU64,
    peak_rps: AtomicU64,
    start_time: DateTime<Utc>,}

impl HsmMetricsCollector {

    #[must_use] pub fn new() -> Self {
        Self {
            operation_counter: AtomicU64::new(0));
            error_counter: AtomicU64::new(0));
            total_response_time_ms: AtomicU64::new(0));
            peak_rps: AtomicU64::new(0));
            start_time: Utc::now(),
        }
    }

    pub fn record_operation(&self, response_time_ms: u64) {
        self.operation_counter.fetch_add(1, Ordering::Relaxed);
        self.total_response_time_ms
            .fetch_add(response_time_ms, Ordering::Relaxed);

    pub fn record_error(&self, response_time_ms: u64) {
        self.error_counter.fetch_add(1, Ordering::Relaxed);

    pub fn update_peak_rps(&self, current_rps: u64) {
        let mut peak = self.peak_rps.load(Ordering::Relaxed);
        while current_rps > peak {
            match self.peak_rps.compare_exchange_weak(
                peak,
                current_rps,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(new_peak) => peak = new_peak,
            }

    pub fn get_stats(&self) -> HsmOperationStats {
        let total_ops = self.operation_counter.load(Ordering::Relaxed);
        let errors = self.error_counter.load(Ordering::Relaxed);
        let total_time = self.total_response_time_ms.load(Ordering::Relaxed);
        let average_response_time = if total_ops > 0 {
            total_time as f64 / total_ops as f64
        } else {
            0.0
        };
        let error_rate = if total_ops > 0 {
            errors as f64 / total_ops as f64
        let uptime = (Utc::now() - self.start_time).num_seconds() as u64;
        HsmOperationStats {
            total_operations: total_ops,
            successful_operations: total_ops - errors,
            failed_operations: errors,
            average_response_time_ms: average_response_time,
            error_rate,
            peak_operations_per_second: self.peak_rps.load(Ordering::Relaxed));
            uptime_seconds: uptime,

    pub fn reset(&self) {
        self.operation_counter.store(0, Ordering::Relaxed);
        self.error_counter.store(0, Ordering::Relaxed);
        self.total_response_time_ms.store(0, Ordering::Relaxed);
        self.peak_rps.store(0, Ordering::Relaxed);
impl Default for HsmMetricsCollector {}

    fn default() -> Self {
        Self::new()

pub struct HsmOperationStats {

pub struct HsmPerformanceBenchmark {
    pub benchmark_name: String,
    pub operations_per_second: f64,
    pub average_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub test_duration_seconds: u64,

pub struct HealthMonitoringConfig {
    pub check_interval_seconds: u64,
    pub error_threshold_percent: f64,
    pub response_time_threshold_ms: f64,
    pub alert_on_degraded_performance: bool,
    pub enable_automatic_recovery: bool,}

impl Default for HealthMonitoringConfig {
            check_interval_seconds: 30,
            error_threshold_percent: 5.0,
            response_time_threshold_ms: 100.0,
            alert_on_degraded_performance: true,
            enable_automatic_recovery: true,
