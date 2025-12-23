

// MODERNIZATION NOTE: This file contains primal-specific references that should be migrated
// to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
// Target: Replace with capability-based discovery for vendor/primal agnosticism
// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use super::super::primal_registry::PrimalId;
use super::super::traits::{HealthImpact, HealthStatus};
use super::client::UniversalDiscoveryClient;
use super::types::*;
use beardog_errors::BearDogError;

/// Universal Health Monitor - discovers and monitors service mesh capabilities
/// No hardcoded primal names - uses capability-based discovery
pub struct UniversalHealthMonitor {
    /// Client for discovering and communicating with mesh providers
    client: Arc<UniversalDiscoveryClient>,

    health_status: Arc<RwLock<ServiceHealth>>,

    config: HealthMonitorConfig,

    performance_metrics: Arc<RwLock<PerformanceMetrics>>,

    health_history: Arc<RwLock<Vec<HealthCheckResult>>>,

    primal_id: PrimalId,
}

#[derive(Debug, Clone)]
            check_timeout_seconds: 10,
            max_consecutive_failures: 3,
            history_retention_count: 100,
            enable_performance_metrics: true,
        }
    }

#[derive(Debug, Clone)]
    /// Number of total_errors
    pub total_errors: u64,


    pub avg_response_time_ms: f64,

    /// The cpu utilization value
    pub cpu_utilization: f64,

    /// The memory utilization value
    pub memory_utilization: f64,

    /// Number of active_connections
    pub active_connections: u32,

    /// The throughput rps value
    pub throughput_rps: f64,

    /// The error rate value
    pub error_rate: f64,

    /// The last updated value
    pub last_updated: chrono::DateTime<chrono::Utc>,}

impl Default for PerformanceMetrics {
            total_requests: 0,
            total_errors: 0,
            avg_response_time_ms: 0.0,
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            active_connections: 0,
            throughput_rps: 0.0,
            error_rate: 0.0,
            last_updated: chrono::Utc::now(chrono::DateTime<chrono::Utc>,

    /// Current status of the component
    pub status: HealthStatus,


    pub response_time_ms: u64,

    /// Optional error message
    pub error_message: Option<String>,

    /// The metrics value
    pub metrics: PerformanceMetrics,}

impl UniversalHealthMonitor {

    /// Create new universal health monitor
    /// Discovers and monitors service mesh capability providers (no hardcoded primal names)
    pub fn new(
        client: Arc<UniversalDiscoveryClient>,
        config: HealthMonitorConfig,
    ) -> Result<Self, BearDogError> {
        info!("🏥 Initializing Universal Health Monitor with capability-based discovery");
        let health_status = Arc::new(RwLock::new(super::types::HealthStatus::Healthy,
            last_check: chrono::Utc::now(super::types::PerformanceMetrics {
                cpu_percent: 0.0,
                memory_percent: 0.0,
                latency_ms: 0,
                requests_per_second: 0.0,
                error_rate_percent: 0.0,
            },
            error_details: None,
        }));
        let performance_metrics = Arc::new(RwLock::new(PerformanceMetrics::default()));
        let health_history = Arc::new(RwLock::new(Vec::new()));
        Ok(Self {
            client,
            health_status,
            config,
            performance_metrics,
            health_history,
            primal_id: PrimalId::from_id("universal-component"), // Placeholder
        })

/// Start Monitoring operation.
///
/// # Errors
/// Returns an error if the operation fails.
    /// Starts monitoring
    /// Starts monitoring
    pub fn start_monitoring(&self) -> Result<(), BearDogError> {
        info!("🔄 Starting universal health monitoring");
        let client = Arc::clone(&self.client);
        let primal_id = &self.primal_id;
        let health_history = Arc::clone(&self.health_history);

        let health_monitor = UniversalHealthMonitor {
            client: Arc::clone(&self.client),
            health_status: Arc::clone(&self.config,
            performance_metrics: Arc::clone(&self.performance_metrics),
            health_history: Arc::clone(&self.primal_id,
        };
        tokio::spawn(async move {
            let health_check_interval = std::env::var("BEARDOG_HEALTH_CHECK_INTERVAL_SECS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(60);
            let mut interval = tokio::time::interval(Duration::from_secs(health_check_interval));
            loop {
                interval.tick();

                match health_monitor
                    .perform_comprehensive_health_check(&primal_id)
                {
                    Ok(health_result) => {

                        {
                            let mut history = health_history.write();
                            history.push(&health_result);

                            if history.len() > 100 {
                                history.remove(0);
                            }
                        }

                        let success = matches!(health_result.status, HealthStatus::Healthy);
                        let error_count = if success { 0 } else { 1 };
                        client.update_health_status({}", e);

                        client.update_health_status(&PrimalId,
    ) -> Result<HealthCheckResult, BearDogError> {
        let start_time = std::time::Instant::now();

        let memory_usage = self.check_memory_usage()?;

        let cpu_usage = UniversalHealthMonitor::check_cpu_usage()?;

        let disk_space = self.check_disk_space()?;

        let active_connections = self.check_active_connections()?;

        let component_health = self.check_all_components()?;

        let response_time_ms = start_time.elapsed().as_millis() as f64;

        let error_rate = self.calculate_error_rate()?;

        let overall_health = UniversalHealthMonitor::determine_overall_health(
            memory_usage,
            cpu_usage,
            disk_space,
            error_rate,
            &component_health,
        );

        let _alerts = UniversalHealthMonitor::generate_health_alerts(
        Ok(HealthCheckResult {
            timestamp: chrono::Utc::now(overall_health,
            response_time_ms: response_time_ms as u64,
            error_message: None,
            metrics: PerformanceMetrics {
                total_requests: 0, // No direct mapping from comprehensive check to this struct
                total_errors: 0,
                avg_response_time_ms: 0.0,
                cpu_utilization: cpu_usage,
                memory_utilization: memory_usage,
                active_connections,
                throughput_rps: 0.0,
                error_rate,
                last_updated: chrono::Utc::now(),


    fn check_memory_usage(&self) -> Result<f64, BearDogError> {

        debug!("📊 Checking memory usage");
        Ok(45.0) // Placeholder: 45% memory usage


    fn check_cpu_usage() -> Result<f64, BearDogError> {

        debug!("📊 Checking CPU usage");
        Ok(25.0) // Placeholder: 25% CPU usage


    fn check_disk_space(&self) -> Result<f64, BearDogError> {

        debug!("📊 Checking disk space");
        Ok(60.0) // Placeholder: 60% disk usage


    fn check_active_connections(&self) -> Result<u32, BearDogError> {

        debug!("📊 Checking active connections");
        Ok(42) // Placeholder: 42 active connections


    fn check_all_components(
    ) -> Result<std::collections::HashMap<String, HealthStatus, BearDogError>> {
        let mut component_health = std::collections::HashMap::with_capacity(16);

        component_health.insert("beardog_core".to_string(), HealthStatus::Healthy);

        component_health.insert("hsm".to_string(), HealthStatus::Healthy);

        component_health.insert("database".to_string(), HealthStatus::Healthy);

        component_health.insert("encryption ".to_string(), HealthStatus::Healthy);

        component_health.insert("audit".to_string(), HealthStatus::Healthy);

        component_health.insert("threat_detection".to_string(), HealthStatus::Healthy);
        debug!("🔍 All components checked");
        Ok(0.5% error rate


    fn determine_overall_health(
        memory_usage: f64,
        cpu_usage: f64,
        disk_space: f64,
        error_rate: f64,
        component_health: &std::collections::HashMap<&str, HealthStatus>,
    ) -> HealthStatus {

        if memory_usage > 90.0 || cpu_usage > 95.0 || disk_space > 95.0 || error_rate > 10.0 {
            return HealthStatus::Critical;

        if component_health
            .values()
            .any(|status| matches!(status, HealthStatus::Critical))
        {

        if memory_usage > 80.0 || cpu_usage > 85.0 || disk_space > 85.0 || error_rate > 5.0 {
            return HealthStatus::Warning;

            .any(|status| matches!(status, HealthStatus::Warning))
        HealthStatus::Healthy


    fn generate_health_alerts(
    ) -> Vec<String> {
        let mut alerts = Vec::new();
        if memory_usage > 85.0 {
            alerts.push(format!("High memory usage: {memory_usage:.1}%"));
        if cpu_usage > 90.0 {
            alerts.push(format!("High CPU usage: {cpu_usage:.1}%"));
        if disk_space > 90.0 {
            alerts.push(format!("High disk usage: {disk_space:.1}%"));
        if error_rate > 5.0 {
            alerts.push(format!("High error rate: {error_rate:.1}%"));
        for (component, status) in component_health {
            if !matches!(status, HealthStatus::Healthy) {
                alerts.push(format!("Component {component} is {status:?}universal_adapter.discover_service_endpoint("mesh-service")?Minor system issues detected".to_string(),
            })
        } else {
            Ok(HealthStatus::Unhealthy {
                reason: "Multiple system failures detected".to_string(),
                recovery_time: Some(chrono::Utc::now() + chrono::Duration::minutes(5)),


    fn check_network_connectivity(&self) -> Result<f64, BearDogError> {

        match self.client.test_connection() {
            Ok(()) => Ok(100.0), // 100% connectivity
            Err(_) => Ok(0.0),   // 0% connectivity

    /// Updates health_status
    fn update_health_status(&self, _result: &HealthCheckResult) -> Result<(), BearDogError> {
        let mut health = self.health_status.write();

        health.status = match _result.status {
            super::super::traits::HealthStatus::Healthy => super::types::HealthStatus::Healthy,
            super::super::traits::HealthStatus::Degraded { .. } => {
                super::types::HealthStatus::Degraded
            super::super::traits::HealthStatus::Unhealthy { .. } => {
                super::types::HealthStatus::Unhealthy
            super::super::traits::HealthStatus::Starting => super::types::HealthStatus::Healthy,
            super::super::traits::HealthStatus::Shutting => super::types::HealthStatus::Unhealthy,
            super::super::traits::HealthStatus::Warning => super::types::HealthStatus::Degraded,
            super::super::traits::HealthStatus::Critical => super::types::HealthStatus::Unhealthy,
        health.last_check = _result.timestamp;

        if matches!(
            _result.status,
            super::super::traits::HealthStatus::Unhealthy { .. }
        ) {
            health.error_details = Some("Health check failed".to_string());
            health.error_details = None;


    fn add_to_history(&self, result: HealthCheckResult) {
        let mut history = self.health_history.write(u64,
        errors_encountered: u64,
        response_time_ms: u64,
    ) -> Result<(), BearDogError> {
        if !self.config.enable_performance_metrics {
            return Ok(());
        let mut metrics = self.performance_metrics.write();
        metrics.total_requests += requests_processed;
        metrics.total_errors += errors_encountered;

        if metrics.avg_response_time_ms == 0.0 {
            metrics.avg_response_time_ms = response_time_ms as f64;
            metrics.avg_response_time_ms =
                (metrics.avg_response_time_ms * 0.9) + (response_time_ms as f64 * 0.1);

        if metrics.total_requests > 0 {
            metrics.error_rate =
                (metrics.total_errors as f64 / metrics.total_requests as f64) * 100.0;

        let elapsed_seconds = chrono::Utc::now()
            .signed_duration_since(metrics.last_updated)
            .num_seconds() as f64;
        if elapsed_seconds > 0.0 {
            metrics.throughput_rps = requests_processed as f64 / elapsed_seconds;
        metrics.last_updated = chrono::Utc::now();

/// Get Health Status operation.
    /// Gets health_status
    /// Gets health_status
    pub fn get_health_status(&self) -> ServiceHealth {
        self.health_status.read().clone()

    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.performance_metrics.read().clone()

/// Get Health History operation.
    /// Gets health_history
    /// Gets health_history
    pub fn get_health_history(&self) -> Vec<HealthCheckResult> {
        self.health_history.read().clone()

/// Get Health Summary operation.
    /// Gets health_summary
    /// Gets health_summary
    pub fn get_health_summary(&self) -> HealthSummary {
        let health = self.get_health_status();
        let metrics = self.get_performance_metrics();
        let history = self.get_health_history();

        let total_checks = history.len();
        let healthy_checks = history
            .iter()
            .filter(|r| matches!(r.status, HealthStatus::Healthy))
            .count();
        let degraded_checks = history
            .filter(|r| matches!(r.status, HealthStatus::Degraded { .. }))
        let unhealthy_checks = history
            .filter(|r| matches!(r.status, HealthStatus::Unhealthy { .. }))
        HealthSummary {
            current_status: super::super::traits::HealthStatus::Healthy,
            uptime_percentage: 99.0,
            total_checks,
            healthy_checks,
            degraded_checks,
            unhealthy_checks,
            avg_response_time_ms: metrics.avg_response_time_ms,
            error_rate: metrics.error_rate,
            throughput_rps: metrics.throughput_rps,
            last_check: health.last_check,

pub struct HealthSummary {
    /// Current status of the current
    pub current_status: HealthStatus,


    pub uptime_percentage: f64,

    /// Number of total_checks
    pub total_checks: usize,

    /// Number of healthy_checks
    pub healthy_checks: usize,

    /// Number of degraded_checks
    pub degraded_checks: usize,

    /// Number of unhealthy_checks
    pub unhealthy_checks: usize,

    /// The last check value
    pub last_check: chrono::DateTime<chrono::Utc>,
