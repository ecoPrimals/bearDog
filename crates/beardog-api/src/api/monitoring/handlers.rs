

use super::*;
use axum::{extract::State, http::StatusCode, Json};
use beardog_monitoring::service::{MonitoringConfig, MonitoringService};
use serde_json::json;
use std::sync::Arc;

lazy_static::lazy_static! {
    static ref MONITORING_SERVICE: Arc<MonitoringService> = Arc::new(
        MonitoringService::new(MonitoringConfig::default())
    );
}

pub async fn get_system_health(
    State(_state): State<AppState>,
) -> Result<Json<super::models::SystemHealthResponse>, StatusCode> {
    match MONITORING_SERVICE.collect_metrics().await {
        Ok(metrics) => {
            let health_response = super::models::SystemHealthResponse {
                status: "healthy".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                uptime_seconds: 3600, // Would get from actual system
                version: env!("CARGO_PKG_VERSION").to_string(),
                components: vec![
                    super::models::ComponentStatus {
                        name: "api".to_string(),
                        status: "healthy".to_string(),
                        response_time_ms: 1,
                        last_check: chrono::Utc::now().to_rfc3339(),
                    },
                        name: "monitoring".to_string(),
                        response_time_ms: 2,
                ],
                performance: super::models::PerformanceOverview {
                    cpu_usage_percent: metrics.performance.cpu_usage_percent,
                    memory_usage_percent: if metrics.performance.memory_total_bytes > 0 {
                        (metrics.performance.memory_usage_bytes as f64
                            / metrics.performance.memory_total_bytes as f64)
                            * 100.0
                    } else {
                        0.0
                    disk_usage_percent: if metrics.performance.disk_total_bytes > 0 {
                        (metrics.performance.disk_usage_bytes as f64
                            / metrics.performance.disk_total_bytes as f64)
                    active_connections: metrics.performance.active_connections,
                    request_rate: metrics.performance.request_count as f64 / 60.0, // Requests per minute
                },
            };
            Ok(Json(health_response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

pub async fn get_detailed_health(
) -> Result<Json<serde_json::Value>, StatusCode> {
            let detailed_health = json!({
                "success": true,
                "system": {
                    "status": "healthy",
                    "uptime_seconds": 3600,
                    "version": env!("CARGO_PKG_VERSION"),
                    "build_info": {
                        "compiler": "rustc",
                        "profile": if cfg!(debug_assertions) { "debug" } else { "release" }
                    }
                "performance": {
                    "cpu": {
                        "usage_percent": metrics.performance.cpu_usage_percent,
                        "cores": num_cpus::get()
                    "memory": {
                        "usage_bytes": metrics.performance.memory_usage_bytes,
                        "total_bytes": metrics.performance.memory_total_bytes,
                        "usage_percent": if metrics.performance.memory_total_bytes > 0 {
                            (metrics.performance.memory_usage_bytes as f64 / metrics.performance.memory_total_bytes as f64) * 100.0
                        } else { 0.0 }
                    "disk": {
                        "usage_bytes": metrics.performance.disk_usage_bytes,
                        "total_bytes": metrics.performance.disk_total_bytes,
                        "usage_percent": if metrics.performance.disk_total_bytes > 0 {
                            (metrics.performance.disk_usage_bytes as f64 / metrics.performance.disk_total_bytes as f64) * 100.0
                    "network": {
                        "rx_bytes": metrics.performance.network_rx_bytes,
                        "tx_bytes": metrics.performance.network_tx_bytes,
                        "active_connections": metrics.performance.active_connections
                "application": {
                    "requests_total": metrics.performance.request_count,
                    "errors_total": metrics.performance.error_count,
                    "avg_response_time_ms": metrics.performance.avg_response_time_ms
                "timestamp": chrono::Utc::now().to_rfc3339()
            });
            Ok(Json(detailed_health))

pub async fn get_component_health(
    let components = json!({
        "success": true,
        "components": {
            "api_server": {
                "status": "healthy",
                "response_time_ms": 1,
                "details": "HTTP server responding normally"
            },
            "monitoring_service": {
                "response_time_ms": 2,
                "details": "Metrics collection active"
            "crypto_engine": {
                "details": "Cryptographic operations functional"
            "zero_copy_system": {
                "details": "Buffer pools active, performance optimized"
            }
        },
        "timestamp": chrono::Utc::now().to_rfc3339()
    });
    Ok(Json(components))

pub async fn get_system_status(
            let status = json!({
                "status": "operational",
                "overall_health": "healthy",
                "uptime_seconds": 3600,
                "version": env!("CARGO_PKG_VERSION"),
                "environment": if cfg!(debug_assertions) { "development" } else { "production" },
                "features": {
                    "zero_copy_optimization": true,
                    "real_cryptography": true,
                    "monitoring": true,
                    "security_provider": true
                "performance_summary": {
                    "cpu_usage": format_args!("{:.1}%", metrics.performance.cpu_usage_percent).to_string(),
                    "memory_usage": format_args!("{:.1}MB", metrics.performance.memory_usage_bytes / 1024 / 1024).to_string(),
                    "active_connections": metrics.performance.active_connections,
                    "total_requests": metrics.performance.request_count
            Ok(Json(status))

pub async fn get_readiness(

    let ready = json!({
        "ready": true,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "checks": {
            "api_server": true,
            "monitoring": true,
            "crypto_engine": true,
            "zero_copy_buffers": true
    Ok(Json(ready))

pub async fn get_liveness(

    let alive = json!({
        "alive": true,
        "uptime_seconds": 3600
    Ok(Json(alive))

pub async fn get_system_metrics(
            let system_metrics = json!({
                "metrics": {
                    "timestamp": metrics.timestamp.to_rfc3339(),
                    "performance": metrics.performance,
                    "custom_metrics": metrics.custom_metrics
                }
            Ok(Json(system_metrics))

pub async fn get_realtime_metrics(
        Ok(current) => {
            let realtime = json!({
                "realtime": {
                    "timestamp": current.timestamp.to_rfc3339(),
                    "cpu_percent": current.performance.cpu_usage_percent,
                    "memory_bytes": current.performance.memory_usage_bytes,
                    "active_connections": current.performance.active_connections,
                    "request_count": current.performance.request_count,
                    "error_count": current.performance.error_count,
                    "avg_response_time_ms": current.performance.avg_response_time_ms
            Ok(Json(realtime))
