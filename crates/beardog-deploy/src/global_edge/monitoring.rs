

use super::types::*;
use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{interval, Duration};
use tracing::{info, warn, error, debug};

pub struct GlobalHealthMonitor {
    config: MonitoringConfig,
    health_checks: Arc<RwLock<Vec<HealthCheck>>>,
    alert_thresholds: AlertThresholds,
    incidents: Arc<RwLock<Vec<Incident>>>,
    monitoring_active: Arc<RwLock<bool>>,
}

impl GlobalHealthMonitor {

    pub fn new(config: MonitoringConfig) -> Self {
        Self {
            config,
            health_checks: Arc::new(RwLock::new(Vec::new())),
            alert_thresholds: AlertThresholds::default(),
            incidents: Arc::new(RwLock::new(Vec::new())),
            monitoring_active: Arc::new(RwLock::new(false)),
        }
    }

    pub async fn start_monitoring(&self) -> Result<(), BearDogError> {
        info!("Starting global health monitoring system");

        {
            let mut active = self.monitoring_active.write().await;
            *active = true;
        }

        let health_checks = Arc::clone(&self.health_checks);
        let incidents = Arc::clone(&self.incidents);
        let monitoring_active = Arc::clone(&self.monitoring_active);
        let check_interval = self.config.check_interval;
        let alert_thresholds = self.alert_thresholds.clone();
        
        tokio::spawn(async move {
            let mut interval = interval(check_interval);
            
            while *monitoring_active.read().await {
                interval.tick().await;
                
                debug!("Running health checks");
                let checks = health_checks.read().await;
                
                for health_check in checks.iter() {
                    if let Err(e) = Self::perform_health_check(
                        health_check, 
                        &incidents, 
                        &alert_thresholds
                    ).await {
                        warn!("Health check failed: {}", e);
                    }
                }
            }
        });
        
        info!("Global health monitoring started successfully");
        Ok(())
    }

    pub async fn add_region_health_check(&self, region_id: &str, endpoint_url: &str) -> Result<(), BearDogError> {
        let health_check = HealthCheck {
            check_id: format_args!("health_check_{}", region_id).to_string(),
            region_id,
            endpoint_url,
            check_type: "http_get".to_string(),
            expected_status: 200,
            timeout_ms: 5000,
            last_check: 0,
            status: HealthStatus::Unknown,
        };
        
        let mut checks = self.health_checks.write().await;
        checks.push(health_check);
        
        info!("Added health check for region");
        Ok(())
    }

    async fn perform_health_check(
        health_check: &HealthCheck,
        incidents: &Arc<RwLock<Vec<Incident>>>,
        alert_thresholds: &AlertThresholds,
    ) -> Result<(), BearDogError> {
        debug!("Performing health check for region: {}", health_check.region_id);

        let is_healthy = Self::simulate_health_check(health_check).await?;
        
        if !is_healthy {

            let incident = Incident {
                incident_id: format_args!("incident_{}_{}", 
                    health_check.region_id, 
                    chrono::Utc::now().to_string().timestamp()
                ),
                region_id: health_check.region_id.clone(),
                severity: "high".to_string(),
                title: format_args!("Health check failed for region {}", health_check.region_id).to_string(),
                description: format_args!("Health check endpoint {} is not responding", health_check.endpoint_url).to_string(),
                status: "open".to_string(),
                created_at: chrono::Utc::now().timestamp() as u64,
                updated_at: chrono::Utc::now().timestamp() as u64,
            };
            
            let mut incidents_guard = incidents.write().await;
            incidents_guard.push(incident);
            
            warn!("Health check failed for region: {}", health_check.region_id);
        }
        
        Ok(())
    }

    async fn simulate_health_check(_health_check: &HealthCheck) -> Result<bool, BearDogError> {

        Ok(rand::random::<f64>() < 0.95)
    }

    pub async fn get_global_health(&self) -> Result<GlobalHealthStatus, BearDogError> {
        let checks = self.health_checks.read().await;
        let incidents = self.incidents.read().await;
        
        let total_checks = checks.len();
        let healthy_checks = checks.iter()
            .filter(|check| check.status == HealthStatus::Healthy)
            .count();
        
        let open_incidents = incidents.iter()
            .filter(|incident| incident.status == "open")
            .count();
        
        let overall_status = if healthy_checks == total_checks && open_incidents == 0 {
            "healthy".to_string()
        } else if healthy_checks as f64 / total_checks as f64 > 0.8 {
            "degraded".to_string()
        } else {
            "unhealthy".to_string()
        };
        
        Ok(GlobalHealthStatus {
            overall_status,
            total_regions: total_checks,
            healthy_regions: healthy_checks,
            unhealthy_regions: total_checks - healthy_checks,
            open_incidents,
            last_updated: chrono::Utc::now().timestamp() as u64,
        })
    }

    pub async fn get_incidents(&self) -> Vec<Incident> {
        self.incidents.read().await.clone()
    }

    pub async fn resolve_incident(&self, incident_id: &str) -> Result<(), BearDogError> {
        let mut incidents = self.incidents.write().await;
        
        if let Some(incident) = incidents.iter_mut().find(|i| i.incident_id == incident_id) {
            incident.status = "resolved".to_string();
            incident.updated_at = chrono::Utc::now().timestamp() as u64;
            info!("Resolved incident: {}", incident_id);
            Ok(())
        } else {
            Err(BearDogError::not_found(format_args!("Incident not found: {}", incident_id).to_string()))
        }
    }

    pub async fn shutdown(&self) -> Result<(), BearDogError> {
        info!("Shutting down global health monitoring system");

        {
            let mut active = self.monitoring_active.write().await;
            *active = false;
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
        
        info!("Global health monitoring shutdown complete");
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct GlobalHealthStatus {
    pub overall_status: String,
    pub total_regions: usize,
    pub healthy_regions: usize,
    pub unhealthy_regions: usize,
    pub open_incidents: usize,
    pub last_updated: u64,
} 