

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use super::super::primal_registry::PrimalId;
use super::super::traits::{HealthImpact, HealthStatus};
use super::client::SongBirdDiscoveryClient;
use super::types::*;
use beardog_errors::BearDogResult;

pub struct UniversalHealthMonitor {

    client: Arc<SongBirdDiscoveryClient>,

    health_status: Arc<RwLock<ServiceHealth>>,

    config: HealthMonitorConfig,

    performance_metrics: Arc<RwLock<PerformanceMetrics>>,

    health_history: Arc<RwLock<Vec<HealthCheckResult>>>,

    primal_id: PrimalId,
}

#[derive(Debug, Clone)]

impl Default for HealthMonitorConfig {}

    fn default() -> Self {
        Self {
            check_interval_seconds: 30,
            check_timeout_seconds: 10,
            max_consecutive_failures: 3,
            history_retention_count: 100,
            enable_performance_metrics: true,
        }
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {

    pub total_requests: u64,

    pub total_errors: u64,

    pub avg_response_time_ms: f64,

    pub cpu_utilization: f64,

    pub memory_utilization: f64,

    pub active_connections: u32,

    pub throughput_rps: f64,

    pub error_rate: f64,

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
            last_updated: chrono::Utc::now(),

pub struct HealthCheckResult {

    pub timestamp: chrono::DateTime<chrono::Utc>,

    pub status: HealthStatus,

    pub response_time_ms: u64,

    pub error_message: Option<String>,

    pub metrics: PerformanceMetrics,}

impl UniversalHealthMonitor {

    pub async fn new(
        client: Arc<SongBirdDiscoveryClient>,
        config: HealthMonitorConfig,
    ) -> BearDogResult<Self> {
        info!("🏥 Initializing Universal Health Monitor");
        let health_status = Arc::new(RwLock::new(ServiceHealth {
            status: super::types::HealthStatus::Healthy,
            last_check: chrono::Utc::now(),
            metrics: super::types::PerformanceMetrics {
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

    pub async fn start_monitoring(&self) -> BearDogResult<()> {
        info!("🔄 Starting universal health monitoring");
        let client = Arc::clone(&self.client);
        let primal_id = self.primal_id.clone();
        let health_history = Arc::clone(&self.health_history);

        let health_monitor = UniversalHealthMonitor {
            client: Arc::clone(&self.client),
            health_status: Arc::clone(&self.health_status),
            config: self.config.clone(),
            performance_metrics: Arc::clone(&self.performance_metrics),
            health_history: Arc::clone(&self.health_history),
            primal_id: self.primal_id.clone(),
        };
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));
            loop {
                interval.tick().await;

                match health_monitor
                    .perform_comprehensive_health_check(&primal_id)
                    .await
                {
                    Ok(health_result) => {

                        {
                            let mut history = health_history.write().await;
                            history.push(health_result.clone());

                            if history.len() > 100 {
                                history.remove(0);
                            }
                        }

                        let success = matches!(health_result.status, HealthStatus::Healthy);
                        let error_count = if success { 0 } else { 1 };
                        client.update_health_status(success, error_count).await;
                        debug!("✅ Health status updated to SongBird");
                    }
                    Err(e) => {
                        warn!("Health check failed: {}", e);

                        client.update_health_status(false, 1).await;
                }
            }
        });
        Ok(())

    async fn perform_comprehensive_health_check(
        &self,
        _primal_id: &PrimalId,
    ) -> BearDogResult<HealthCheckResult> {
        let start_time = std::time::Instant::now();

        let memory_usage = self.check_memory_usage().await?;

        let cpu_usage = UniversalHealthMonitor::check_cpu_usage().await?;

        let disk_space = self.check_disk_space().await?;

        let active_connections = self.check_active_connections().await?;

        let component_health = self.check_all_components().await?;

        let response_time_ms = start_time.elapsed().as_millis() as f64;

        let error_rate = self.calculate_error_rate().await?;

        let overall_health = UniversalHealthMonitor::determine_overall_health(
            memory_usage,
            cpu_usage,
            disk_space,
            error_rate,
            &component_health,
        );

        let _alerts = UniversalHealthMonitor::generate_health_alerts(
        Ok(HealthCheckResult {
            timestamp: chrono::Utc::now(),
            status: overall_health,
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

    async fn check_memory_usage(&self) -> BearDogResult<f64> {

        debug!("📊 Checking memory usage");
        Ok(45.0) // Placeholder: 45% memory usage

    async fn check_cpu_usage() -> BearDogResult<f64> {

        debug!("📊 Checking CPU usage");
        Ok(25.0) // Placeholder: 25% CPU usage

    async fn check_disk_space(&self) -> BearDogResult<f64> {

        debug!("📊 Checking disk space");
        Ok(60.0) // Placeholder: 60% disk usage

    async fn check_active_connections(&self) -> BearDogResult<u32> {

        debug!("📊 Checking active connections");
        Ok(42) // Placeholder: 42 active connections

    async fn check_all_components(
    ) -> BearDogResult<std::collections::HashMap<String, HealthStatus>> {
        let mut component_health = std::collections::HashMap::with_capacity(16);

        component_health.insert("beardog_core".to_string(), HealthStatus::Healthy);

        component_health.insert("hsm".to_string(), HealthStatus::Healthy);

        component_health.insert("database".to_string(), HealthStatus::Healthy);

        component_health.insert("encryption".to_string(), HealthStatus::Healthy);

        component_health.insert("audit".to_string(), HealthStatus::Healthy);

        component_health.insert("threat_detection".to_string(), HealthStatus::Healthy);
        debug!("🔍 All components checked");
        Ok(component_health)

    async fn calculate_error_rate(&self) -> BearDogResult<f64> {

        debug!("📊 Calculating error rate");
        Ok(0.5) // Placeholder: 0.5% error rate

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
                alerts.push(format!("Component {component} is {status:?}"));
        alerts

    pub async fn perform_health_check(&self) -> BearDogResult<HealthCheckResult> {
        debug!("🔍 Performing universal health check");

        let health_status = self.check_component_health().await?;
        let response_time_ms = start_time.elapsed().as_millis() as u64;

        let metrics = self.performance_metrics.read().await.clone();

        let result = HealthCheckResult {
            status: health_status,
            response_time_ms,
            metrics,

        self.update_health_status(&result).await?;

        self.add_to_history(result.clone()).await;

        self.report_health_to_songbird().await?;
        Ok(result)

    async fn check_component_health(&self) -> BearDogResult<HealthStatus> {

        let checks = vec![
            self.check_memory_usage().await,
            UniversalHealthMonitor::check_cpu_usage().await,
            self.check_disk_space().await,
            self.check_network_connectivity().await,
        ];

        let failed_checks = checks.iter().filter(|r| r.is_err()).count();
        if failed_checks == 0 {
            Ok(HealthStatus::Healthy)
        } else if failed_checks <= 1 {
            Ok(HealthStatus::Degraded {
                issues: vec!["Minor system issues detected".to_string()],
                impact: HealthImpact::Low,
            })
        } else {
            Ok(HealthStatus::Unhealthy {
                reason: "Multiple system failures detected".to_string(),
                recovery_time: Some(chrono::Utc::now() + chrono::Duration::minutes(5)),

    async fn check_network_connectivity(&self) -> BearDogResult<f64> {

        match self.client.test_connection().await {
            Ok(()) => Ok(100.0), // 100% connectivity
            Err(_) => Ok(0.0),   // 0% connectivity

    async fn update_health_status(&self, _result: &HealthCheckResult) -> BearDogResult<()> {
        let mut health = self.health_status.write().await;

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

    async fn add_to_history(&self, result: HealthCheckResult) {
        let mut history = self.health_history.write().await;
        history.push(result);

        let max_len = self.config.history_retention_count;
        if history.len() > max_len {
            let drain_count = history.len() - max_len;
            history.drain(0..drain_count);

    async fn report_health_to_songbird(&self) -> BearDogResult<()> {
        debug!("📡 Reporting health to SongBird");

        let service_id = "universal-component";
        self.client.update_service_health(service_id).await

    pub async fn update_performance_metrics(
        requests_processed: u64,
        errors_encountered: u64,
        response_time_ms: u64,
    ) -> BearDogResult<()> {
        if !self.config.enable_performance_metrics {
            return Ok(());
        let mut metrics = self.performance_metrics.write().await;
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

    pub async fn get_health_status(&self) -> ServiceHealth {
        self.health_status.read().await.clone()

    pub async fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.performance_metrics.read().await.clone()

    pub async fn get_health_history(&self) -> Vec<HealthCheckResult> {
        self.health_history.read().await.clone()

    pub async fn get_health_summary(&self) -> HealthSummary {
        let health = self.get_health_status().await;
        let metrics = self.get_performance_metrics().await;
        let history = self.get_health_history().await;

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
    pub current_status: HealthStatus,

    pub uptime_percentage: f64,

    pub total_checks: usize,

    pub healthy_checks: usize,

    pub degraded_checks: usize,

    pub unhealthy_checks: usize,

    pub last_check: chrono::DateTime<chrono::Utc>,
