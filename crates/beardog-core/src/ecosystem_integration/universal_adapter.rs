// Fixed universal_adapter.rs - Universal ecosystem adapter
use beardog_errors::BearDogError;
use crate::BearDogCore;
use beardog_types::canonical::HealthStatus;
use tracing::{debug, info, warn, error};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio::time::{timeout, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterConfig {
    pub adapter_id: String,
    pub target_system: String,
    pub endpoint: String,
    pub timeout_ms: u64,
    pub retry_attempts: u32,
    pub authentication: Option<AuthConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub auth_type: String,
    pub credentials: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterResponse {
    pub success: bool,
    pub data: serde_json::Value,
    pub metadata: HashMap<String, String>,
    pub execution_time_ms: u64,
}

pub struct UniversalAdapter {
    config: AdapterConfig,
    client: reqwest::Client,
}

impl UniversalAdapter {
    pub fn new(config: AdapterConfig) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(config.timeout_ms))
            .build()
            .unwrap_or_default();

        Self { config, client }
    }

    pub async fn execute_request(
        &self,
        operation: &str,
        payload: serde_json::Value,
    ) -> Result<AdapterResponse, BearDogError> {
        let start_time = std::time::Instant::now();
        
        info!("🔄 Executing adapter request: {} -> {}", operation, self.config.target_system);

        let response = self.make_http_request(operation, payload).await?;
        let execution_time = start_time.elapsed().as_millis() as u64;

        Ok(AdapterResponse {
            success: true,
            data: response,
            metadata: self.get_response_metadata(),
            execution_time_ms: execution_time,
        })
    }

    async fn make_http_request(
        &self,
        operation: &str,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, BearDogError> {
        let url = format!("{}/{}", self.config.endpoint, operation);
        
        let mut request_builder = self.client.post(&url).json(&payload);

        // Add authentication if configured
        if let Some(auth) = &self.config.authentication {
            request_builder = self.add_authentication(request_builder, auth)?;
        }

        let response = request_builder
            .send()
            .await
            .map_err(|e| BearDogError::network(format!("HTTP request failed: {}", e)))?;

        if response.status().is_success() {
            let json_response = response
                .json::<serde_json::Value>()
                .await
                .map_err(|e| BearDogError::network(format!("Failed to parse JSON response: {}", e)))?;
            
            Ok(json_response)
        } else {
            Err(BearDogError::network(format!(
                "HTTP request failed with status: {}",
                response.status()
            )))
        }
    }

    fn add_authentication(
        &self,
        request_builder: reqwest::RequestBuilder,
        auth: &AuthConfig,
    ) -> Result<reqwest::RequestBuilder, BearDogError> {
        match auth.auth_type.as_str() {
            "bearer" => {
                if let Some(token) = auth.credentials.get("token") {
                    Ok(request_builder.bearer_auth(token))
                } else {
                    Err(BearDogError::security("Bearer token not found".to_string()))
                }
            }
            "basic" => {
                let username = auth.credentials.get("username")
                    .ok_or_else(|| BearDogError::security("Username not found".to_string()))?;
                let password = auth.credentials.get("password")
                    .ok_or_else(|| BearDogError::security("Password not found".to_string()))?;
                
                Ok(request_builder.basic_auth(username, Some(password)))
            }
            _ => Err(BearDogError::security(format!(
                "Unsupported authentication type: {}",
                auth.auth_type
            ))),
        }
    }

    fn get_response_metadata(&self) -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("adapter_id".to_string(), self.config.adapter_id.clone());
        metadata.insert("target_system".to_string(), self.config.target_system.clone());
        metadata.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
        metadata
    }

    pub async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        debug!("🏥 Performing adapter health check");
        
        match self.make_http_request("health", serde_json::json!({})).await {
            Ok(_) => Ok(HealthStatus::Healthy),
            Err(_) => Ok(HealthStatus::Unhealthy),
        }
    }
}

pub struct ProductionUniversalAdapter {
    adapters: HashMap<String, UniversalAdapter>,
}

impl ProductionUniversalAdapter {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }

    pub fn add_adapter(&mut self, system_name: String, adapter: UniversalAdapter) {
        self.adapters.insert(system_name, adapter);
    }

    pub async fn execute_on_system(
        &self,
        system_name: &str,
        operation: &str,
        payload: serde_json::Value,
    ) -> Result<AdapterResponse, BearDogError> {
        let adapter = self.adapters.get(system_name)
            .ok_or_else(|| BearDogError::business(format!("No adapter found for system: {}", system_name)))?;

        adapter.execute_request(operation, payload).await
    }

    pub async fn health_check_all(&self) -> HashMap<String, HealthStatus> {
        let mut results = HashMap::new();
        
        for (system_name, adapter) in &self.adapters {
            let health = adapter.health_check().await.unwrap_or(HealthStatus::Unhealthy);
            results.insert(system_name.clone(), health);
        }
        
        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adapter_config_creation() {
        let config = AdapterConfig {
            adapter_id: "test-adapter".to_string(),
            target_system: "test-system".to_string(),
            endpoint: "http://localhost:8080".to_string(),
            timeout_ms: 5000,
            retry_attempts: 3,
            authentication: None,
        };

        assert_eq!(config.adapter_id, "test-adapter");
        assert_eq!(config.timeout_ms, 5000);
    }

    #[test]
    fn test_production_adapter_creation() {
        let adapter = ProductionUniversalAdapter::new();
        assert_eq!(adapter.adapters.len(), 0);
    }
}
