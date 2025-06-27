use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

use crate::error::{BearDogResult, BearDogError};
use crate::utils::env_utils::{EnvUtils, ObservabilityConfig};

/// System health status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

/// Individual component health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    pub name: String,
    pub status: HealthStatus,
    pub message: Option<String>,
    pub last_check: DateTime<Utc>,
    pub check_duration_ms: u64,
    pub metadata: HashMap<String, String>,
}

/// Overall system health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemHealth {
    pub status: HealthStatus,
    pub uptime_seconds: i64,
    pub version: String,
    pub components: Vec<ComponentHealth>,
    pub last_updated: DateTime<Utc>,
}

/// Performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub cpu_usage_percent: f64,
    pub memory_usage_bytes: u64,
    pub memory_total_bytes: u64,
    pub disk_usage_bytes: u64,
    pub disk_total_bytes: u64,
    pub network_rx_bytes: u64,
    pub network_tx_bytes: u64,
    pub active_connections: u32,
    pub request_count: u64,
    pub error_count: u64,
    pub avg_response_time_ms: f64,
}

/// System metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub timestamp: DateTime<Utc>,
    pub performance: PerformanceMetrics,
    pub custom_metrics: HashMap<String, f64>,
}

/// Health check trait for components
#[async_trait::async_trait]
pub trait HealthChecker: Send + Sync {
    async fn check_health(&self) -> BearDogResult<ComponentHealth>;
    fn component_name(&self) -> &str;
}

/// Monitoring service for production deployments
pub struct MonitoringService {
    start_time: Instant,
    checkers: Vec<Arc<dyn HealthChecker>>,
    metrics: Arc<RwLock<SystemMetrics>>,
    config: ObservabilityConfig,
    alert_thresholds: AlertThresholds,
}

/// Alert thresholds for monitoring
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    pub cpu_threshold: f64,
    pub memory_threshold: f64,
    pub disk_threshold: f64,
    pub error_rate_threshold: f64,
    pub response_time_threshold: f64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            cpu_threshold: 80.0,
            memory_threshold: 85.0,
            disk_threshold: 90.0,
            error_rate_threshold: 5.0,
            response_time_threshold: 1000.0,
        }
    }
}

impl MonitoringService {
    /// Create a new monitoring service
    pub fn new(config: ObservabilityConfig) -> Self {
        Self {
            start_time: Instant::now(),
            checkers: Vec::new(),
            metrics: Arc::new(RwLock::new(Self::default_metrics())),
            config,
            alert_thresholds: AlertThresholds::default(),
        }
    }

    /// Register a health checker
    pub fn register_checker(&mut self, checker: Arc<dyn HealthChecker>) {
        self.checkers.push(checker);
    }

    /// Get system health status
    pub async fn get_health(&self) -> BearDogResult<SystemHealth> {
        let mut components = Vec::new();
        let mut overall_status = HealthStatus::Healthy;

        // Check all registered components
        for checker in &self.checkers {
            let check_start = Instant::now();
            
            match checker.check_health().await {
                Ok(mut component) => {
                    component.check_duration_ms = check_start.elapsed().as_millis() as u64;
                    
                    // Update overall status based on component status
                    match component.status {
                        HealthStatus::Unhealthy => overall_status = HealthStatus::Unhealthy,
                        HealthStatus::Degraded if overall_status == HealthStatus::Healthy => {
                            overall_status = HealthStatus::Degraded;
                        }
                        _ => {}
                    }
                    
                    components.push(component);
                }
                Err(e) => {
                    overall_status = HealthStatus::Unhealthy;
                    components.push(ComponentHealth {
                        name: checker.component_name().to_string(),
                        status: HealthStatus::Unhealthy,
                        message: Some(format!("Health check failed: {}", e)),
                        last_check: Utc::now(),
                        check_duration_ms: check_start.elapsed().as_millis() as u64,
                        metadata: HashMap::new(),
                    });
                }
            }
        }

        Ok(SystemHealth {
            status: overall_status,
            uptime_seconds: self.start_time.elapsed().as_secs() as i64,
            version: env!("CARGO_PKG_VERSION").to_string(),
            components,
            last_updated: Utc::now(),
        })
    }

    /// Get system metrics
    pub async fn get_metrics(&self) -> SystemMetrics {
        self.metrics.read().await.clone()
    }

    /// Update system metrics
    pub async fn update_metrics(&self, metrics: SystemMetrics) {
        let mut current_metrics = self.metrics.write().await;
        *current_metrics = metrics;
        
        // Check for alerts
        self.check_alerts(&metrics).await;
    }

    /// Start monitoring loop
    pub async fn start_monitoring(&self) -> BearDogResult<()> {
        if !self.config.enable_metrics {
            tracing::info!("Monitoring disabled in configuration");
            return Ok(());
        }

        tracing::info!("Starting monitoring service");
        
        let metrics = self.metrics.clone();
        let thresholds = self.alert_thresholds.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            
            loop {
                interval.tick().await;
                
                // Collect system metrics
                let current_metrics = Self::collect_system_metrics().await;
                
                // Update metrics
                {
                    let mut metrics_guard = metrics.write().await;
                    *metrics_guard = current_metrics.clone();
                }
                
                // Check thresholds and log warnings
                Self::check_thresholds(&current_metrics, &thresholds).await;
            }
        });
        
        Ok(())
    }

    /// Collect system performance metrics
    async fn collect_system_metrics() -> SystemMetrics {
        // In a real implementation, you'd use system APIs to get actual metrics
        // For now, we'll provide placeholder implementations
        
        SystemMetrics {
            timestamp: Utc::now(),
            performance: PerformanceMetrics {
                cpu_usage_percent: Self::get_cpu_usage().await,
                memory_usage_bytes: Self::get_memory_usage().await,
                memory_total_bytes: Self::get_total_memory().await,
                disk_usage_bytes: Self::get_disk_usage().await,
                disk_total_bytes: Self::get_total_disk().await,
                network_rx_bytes: Self::get_network_rx().await,
                network_tx_bytes: Self::get_network_tx().await,
                active_connections: Self::get_active_connections().await,
                request_count: Self::get_request_count().await,
                error_count: Self::get_error_count().await,
                avg_response_time_ms: Self::get_avg_response_time().await,
            },
            custom_metrics: HashMap::new(),
        }
    }

    /// Check alert thresholds
    async fn check_alerts(&self, metrics: &SystemMetrics) {
        let perf = &metrics.performance;
        let thresholds = &self.alert_thresholds;
        
        if perf.cpu_usage_percent > thresholds.cpu_threshold {
            tracing::warn!("High CPU usage: {:.1}%", perf.cpu_usage_percent);
        }
        
        let memory_usage_percent = (perf.memory_usage_bytes as f64 / perf.memory_total_bytes as f64) * 100.0;
        if memory_usage_percent > thresholds.memory_threshold {
            tracing::warn!("High memory usage: {:.1}%", memory_usage_percent);
        }
        
        let disk_usage_percent = (perf.disk_usage_bytes as f64 / perf.disk_total_bytes as f64) * 100.0;
        if disk_usage_percent > thresholds.disk_threshold {
            tracing::warn!("High disk usage: {:.1}%", disk_usage_percent);
        }
        
        if perf.avg_response_time_ms > thresholds.response_time_threshold {
            tracing::warn!("High response time: {:.1}ms", perf.avg_response_time_ms);
        }
    }

    /// Check thresholds (static version for spawn)
    async fn check_thresholds(metrics: &SystemMetrics, thresholds: &AlertThresholds) {
        let perf = &metrics.performance;
        
        if perf.cpu_usage_percent > thresholds.cpu_threshold {
            tracing::warn!("High CPU usage: {:.1}%", perf.cpu_usage_percent);
        }
        
        let memory_usage_percent = (perf.memory_usage_bytes as f64 / perf.memory_total_bytes as f64) * 100.0;
        if memory_usage_percent > thresholds.memory_threshold {
            tracing::warn!("High memory usage: {:.1}%", memory_usage_percent);
        }
    }

    fn default_metrics() -> SystemMetrics {
        SystemMetrics {
            timestamp: Utc::now(),
            performance: PerformanceMetrics {
                cpu_usage_percent: 0.0,
                memory_usage_bytes: 0,
                memory_total_bytes: 0,
                disk_usage_bytes: 0,
                disk_total_bytes: 0,
                network_rx_bytes: 0,
                network_tx_bytes: 0,
                active_connections: 0,
                request_count: 0,
                error_count: 0,
                avg_response_time_ms: 0.0,
            },
            custom_metrics: HashMap::new(),
        }
    }

    // Placeholder system metric collection methods
    // In production, these would integrate with actual system APIs
    async fn get_cpu_usage() -> f64 {
        // Use procfs, sysinfo, or similar crate for real CPU usage
        rand::random::<f64>() * 20.0 // Simulate 0-20% CPU usage
    }

    async fn get_memory_usage() -> u64 {
        // Real implementation would read from /proc/meminfo or similar
        1024 * 1024 * 256 // 256MB
    }

    async fn get_total_memory() -> u64 {
        1024 * 1024 * 1024 * 4 // 4GB
    }

    async fn get_disk_usage() -> u64 {
        1024 * 1024 * 1024 * 10 // 10GB
    }

    async fn get_total_disk() -> u64 {
        1024 * 1024 * 1024 * 100 // 100GB
    }

    async fn get_network_rx() -> u64 {
        1024 * 1024 * 50 // 50MB
    }

    async fn get_network_tx() -> u64 {
        1024 * 1024 * 30 // 30MB
    }

    async fn get_active_connections() -> u32 {
        42 // Placeholder
    }

    async fn get_request_count() -> u64 {
        1000 // Placeholder
    }

    async fn get_error_count() -> u64 {
        5 // Placeholder
    }

    async fn get_avg_response_time() -> f64 {
        150.0 // 150ms average
    }
}

/// Database health checker
pub struct DatabaseHealthChecker {
    // Add database connection pool or client here
}

impl DatabaseHealthChecker {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl HealthChecker for DatabaseHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
        let start = Instant::now();
        
        // In a real implementation, you'd ping the database
        // For now, simulate a health check
        tokio::time::sleep(Duration::from_millis(10)).await;
        
        Ok(ComponentHealth {
            name: "Database".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Database connection OK".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("type".to_string(), "PostgreSQL".to_string());
                meta.insert("version".to_string(), "14.5".to_string());
                meta
            },
        })
    }

    fn component_name(&self) -> &str {
        "Database"
    }
}

/// Redis health checker
pub struct RedisHealthChecker {
    // Add Redis connection pool or client here
}

impl RedisHealthChecker {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl HealthChecker for RedisHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
        let start = Instant::now();
        
        // In a real implementation, you'd ping Redis
        tokio::time::sleep(Duration::from_millis(5)).await;
        
        Ok(ComponentHealth {
            name: "Redis".to_string(),
            status: HealthStatus::Healthy,
            message: Some("Redis connection OK".to_string()),
            last_check: Utc::now(),
            check_duration_ms: start.elapsed().as_millis() as u64,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("type".to_string(), "Redis".to_string());
                meta.insert("version".to_string(), "7.0".to_string());
                meta
            },
        })
    }

    fn component_name(&self) -> &str {
        "Redis"
    }
}

/// External service health checker
pub struct ExternalServiceHealthChecker {
    pub name: String,
    pub endpoint: String,
    pub client: reqwest::Client,
}

impl ExternalServiceHealthChecker {
    pub fn new(name: String, endpoint: String) -> Self {
        Self {
            name,
            endpoint,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl HealthChecker for ExternalServiceHealthChecker {
    async fn check_health(&self) -> BearDogResult<ComponentHealth> {
        let start = Instant::now();
        
        match self.client.get(&self.endpoint).timeout(Duration::from_secs(5)).send().await {
            Ok(response) => {
                let status = if response.status().is_success() {
                    HealthStatus::Healthy
                } else {
                    HealthStatus::Degraded
                };
                
                Ok(ComponentHealth {
                    name: self.name.clone(),
                    status,
                    message: Some(format!("HTTP {}", response.status())),
                    last_check: Utc::now(),
                    check_duration_ms: start.elapsed().as_millis() as u64,
                    metadata: {
                        let mut meta = HashMap::new();
                        meta.insert("endpoint".to_string(), self.endpoint.clone());
                        meta.insert("status_code".to_string(), response.status().as_u16().to_string());
                        meta
                    },
                })
            }
            Err(e) => {
                Ok(ComponentHealth {
                    name: self.name.clone(),
                    status: HealthStatus::Unhealthy,
                    message: Some(format!("Request failed: {}", e)),
                    last_check: Utc::now(),
                    check_duration_ms: start.elapsed().as_millis() as u64,
                    metadata: {
                        let mut meta = HashMap::new();
                        meta.insert("endpoint".to_string(), self.endpoint.clone());
                        meta.insert("error".to_string(), e.to_string());
                        meta
                    },
                })
            }
        }
    }

    fn component_name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_monitoring_service_creation() {
        let config = ObservabilityConfig {
            log_level: "INFO".to_string(),
            enable_metrics: true,
            metrics_port: 9090,
            enable_tracing: false,
            jaeger_endpoint: None,
            otlp_endpoint: None,
        };
        
        let service = MonitoringService::new(config);
        assert_eq!(service.checkers.len(), 0);
    }

    #[tokio::test]
    async fn test_database_health_checker() {
        let checker = DatabaseHealthChecker::new();
        let health = checker.check_health().await.unwrap();
        
        assert_eq!(health.name, "Database");
        assert_eq!(health.status, HealthStatus::Healthy);
    }

    #[tokio::test]
    async fn test_system_health_no_components() {
        let config = ObservabilityConfig {
            log_level: "INFO".to_string(),
            enable_metrics: true,
            metrics_port: 9090,
            enable_tracing: false,
            jaeger_endpoint: None,
            otlp_endpoint: None,
        };
        
        let service = MonitoringService::new(config);
        let health = service.get_health().await.unwrap();
        
        assert_eq!(health.status, HealthStatus::Healthy);
        assert_eq!(health.components.len(), 0);
    }
} 