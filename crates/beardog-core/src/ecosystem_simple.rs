// Fixed ecosystem_simple.rs - Simplified ecosystem integration
use beardog_errors::BearDogError;
// use crate::core::BearDogCore; // Unused import
use beardog_types::canonical::HealthStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleEcosystemConfig {
    pub enabled_services: Vec<String>,
    pub discovery_timeout_ms: u64,
    pub health_check_interval_ms: u64,
    pub retry_attempts: u32,
}

impl Default for SimpleEcosystemConfig {
    fn default() -> Self {
        Self {
            enabled_services: vec![
                "toadstool".to_string(),
                "songbird".to_string(),
                "squirrel".to_string(),
            ],
            discovery_timeout_ms: 5000,
            health_check_interval_ms: 30000,
            retry_attempts: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemService {
    pub service_name: String,
    pub endpoint: String,
    pub health_status: HealthStatus,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub capabilities: Vec<String>,
}

pub struct SimpleEcosystemManager {
    config: SimpleEcosystemConfig,
    services: HashMap<String, EcosystemService>,
}

impl SimpleEcosystemManager {
    pub fn new(config: SimpleEcosystemConfig) -> Self {
        Self {
            config,
            services: HashMap::new(),
        }
    }

    pub async fn initialize(&mut self) -> Result<(), BearDogError> {
        info!("🌐 Initializing simple ecosystem manager");

        let enabled_services = self.config.enabled_services.clone();
        for service_name in &enabled_services {
            self.discover_service(service_name).await?;
        }

        info!(
            "✅ Simple ecosystem manager initialized with {} services",
            self.services.len()
        );
        Ok(())
    }

    async fn discover_service(&mut self, service_name: &str) -> Result<(), BearDogError> {
        debug!("🔍 Discovering service: {}", service_name);

        let endpoint = match service_name {
            "toadstool" => "http://toadstool.ecosystem:8080",
            "songbird" => "http://songbird.mesh:9090",
            "squirrel" => "http://squirrel.ai:8080",
            _ => {
                return Err(BearDogError::business(format!(
                    "Unknown service: {}",
                    service_name
                )))
            }
        };

        let service = EcosystemService {
            service_name: service_name.to_string(),
            endpoint: endpoint.to_string(),
            health_status: HealthStatus::Healthy,
            last_check: chrono::Utc::now(),
            capabilities: self.get_service_capabilities(service_name),
        };

        self.services.insert(service_name.to_string(), service);
        Ok(())
    }

    fn get_service_capabilities(&self, service_name: &str) -> Vec<String> {
        match service_name {
            "toadstool" => vec!["platform".to_string(), "orchestration".to_string()],
            "songbird" => vec!["mesh".to_string(), "discovery".to_string()],
            "squirrel" => vec!["ai".to_string(), "analytics".to_string()],
            _ => vec![],
        }
    }

    pub async fn health_check_all(&mut self) -> HashMap<String, HealthStatus> {
        let mut results = HashMap::new();

        let service_names: Vec<String> = self.services.keys().cloned().collect();
        for service_name in service_names {
            if let Some(service) = self.services.get(&service_name) {
                let health = self.check_service_health(service).await;
                if let Some(service) = self.services.get_mut(&service_name) {
                    service.health_status = health.clone();
                    service.last_check = chrono::Utc::now();
                }
                results.insert(service_name, health);
            }
        }

        results
    }

    async fn check_service_health(&self, service: &EcosystemService) -> HealthStatus {
        debug!("🏥 Checking health for service: {}", service.service_name);

        // In a real implementation, this would make HTTP health check requests
        match service.service_name.as_str() {
            "toadstool" | "songbird" | "squirrel" => HealthStatus::Healthy,
            _ => HealthStatus::Unhealthy,
        }
    }

    pub fn get_service_status(&self, service_name: &str) -> Option<&EcosystemService> {
        self.services.get(service_name)
    }

    pub fn get_all_services(&self) -> &HashMap<String, EcosystemService> {
        &self.services
    }

    pub async fn execute_on_service(
        &self,
        service_name: &str,
        operation: &str,
        _payload: serde_json::Value,
    ) -> Result<serde_json::Value, BearDogError> {
        let service = self.services.get(service_name).ok_or_else(|| {
            BearDogError::business(format!("Service not found: {}", service_name))
        })?;

        if !matches!(service.health_status, HealthStatus::Healthy) {
            return Err(BearDogError::business(format!(
                "Service {} is not healthy",
                service_name
            )));
        }

        info!("🚀 Executing {} on service {}", operation, service_name);

        // Mock response - in reality would make HTTP request
        Ok(serde_json::json!({
            "service": service_name,
            "operation": operation,
            "status": "success",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }

    pub async fn send_ecosystem_message(
        &self,
        target_service: &str,
        message_type: &str,
        _payload: serde_json::Value,
    ) -> Result<serde_json::Value, BearDogError> {
        let service = self.services.get(target_service).ok_or_else(|| {
            BearDogError::business(format!("Service not found: {}", target_service))
        })?;

        if !matches!(service.health_status, HealthStatus::Healthy) {
            return Err(BearDogError::business(format!(
                "Service {} is not healthy",
                target_service
            )));
        }

        info!(
            "🚀 Sending message {} to service {}",
            message_type, target_service
        );

        // Mock response - in reality would make HTTP request
        Ok(serde_json::json!({
            "service": target_service,
            "message_type": message_type,
            "status": "success",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }

    pub fn get_healthy_services(&self) -> Vec<&str> {
        self.services
            .iter()
            .filter(|(_, service)| matches!(service.health_status, HealthStatus::Healthy))
            .map(|(name, _)| name.as_str())
            .collect()
    }

    pub fn get_ecosystem_metrics(&self) -> HashMap<String, serde_json::Value> {
        let mut metrics = HashMap::new();

        metrics.insert(
            "total_services".to_string(),
            serde_json::json!(self.services.len()),
        );
        metrics.insert(
            "healthy_services".to_string(),
            serde_json::json!(self.get_healthy_services().len()),
        );
        metrics.insert(
            "enabled_services".to_string(),
            serde_json::json!(self.config.enabled_services),
        );
        metrics.insert(
            "last_updated".to_string(),
            serde_json::json!(chrono::Utc::now().to_rfc3339()),
        );

        metrics
    }
}

impl Default for SimpleEcosystemManager {
    fn default() -> Self {
        Self::new(SimpleEcosystemConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_ecosystem_config() {
        let config = SimpleEcosystemConfig::default();
        assert_eq!(config.enabled_services.len(), 3);
        assert!(config.enabled_services.contains(&"toadstool".to_string()));
    }

    #[tokio::test]
    async fn test_simple_ecosystem_manager() {
        let mut manager = SimpleEcosystemManager::default();
        assert!(manager.initialize().await.is_ok());
        assert_eq!(manager.get_all_services().len(), 3);
    }

    #[test]
    fn test_service_capabilities() {
        let manager = SimpleEcosystemManager::default();
        let capabilities = manager.get_service_capabilities("toadstool");
        assert!(capabilities.contains(&"platform".to_string()));
    }
}
