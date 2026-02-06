//! # BiomeOS Adapter
//!
//! This module provides the adapter for BiomeOS integration, enabling
//! container orchestration, resource management, and environment configuration.

use beardog_errors::BearDogError;
use beardog_types::{
    canonical::providers::{ProviderConfig, ProviderHealthStatus},
    capabilities::CapabilityType,
    providers::{BaseProvider, PrimalRequest, PrimalResponse},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// Re-export canonical types
pub use beardog_types::canonical::configuration::adapters::{
    BiomeOSAuthConfig, BiomeOSConnectionConfig,
};

// ============================================================
// Connection Configuration
// ============================================================

impl Default for BiomeOSConnectionConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            max_retries: 3,
            health_check_interval_seconds: 60,
        }
    }
}

// ============================================================
// BiomeOS Adapter
// ============================================================

/// BiomeOS adapter for container orchestration
pub struct BiomeOSAdapter {
    /// BiomeOS API endpoint
    pub endpoint: String,

    /// Authentication configuration
    pub auth_config: BiomeOSAuthConfig,

    /// Connection configuration
    pub connection_config: BiomeOSConnectionConfig,
}

impl BiomeOSAdapter {
    /// Create a new adapter
    pub fn new(endpoint: &str, auth_config: BiomeOSAuthConfig) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            auth_config,
            connection_config: BiomeOSConnectionConfig::default(),
        }
    }

    /// Create with custom connection config
    pub fn with_config(
        endpoint: &str,
        auth_config: BiomeOSAuthConfig,
        connection_config: BiomeOSConnectionConfig,
    ) -> Self {
        Self {
            endpoint: endpoint.to_string(),
            auth_config,
            connection_config,
        }
    }

    /// Handle container orchestration request
    pub fn handle_container_orchestration(
        &self,
        request: &PrimalRequest,
    ) -> Result<PrimalResponse, BearDogError> {
        tracing::info!("🐳 Processing container orchestration request for BiomeOS");

        let biome_request = self.convert_to_biome_api_request(request)?;
        let biome_response = self.call_biome_api("container/orchestrate", &biome_request)?;
        self.convert_from_biome_api_response(biome_response)
    }

    /// Handle process orchestration request
    pub fn handle_process_orchestration(
        &self,
        request: &PrimalRequest,
    ) -> Result<PrimalResponse, BearDogError> {
        tracing::info!("⚙️ Processing process orchestration request for BiomeOS");

        let biome_request = self.convert_to_biome_api_request(request)?;
        let biome_response = self.call_biome_api("process/orchestrate", &biome_request)?;
        self.convert_from_biome_api_response(biome_response)
    }

    /// Handle resource management request
    pub fn handle_resource_management(
        &self,
        request: &PrimalRequest,
    ) -> Result<PrimalResponse, BearDogError> {
        tracing::info!("💾 Processing resource management request for BiomeOS");

        let biome_request = self.convert_to_biome_api_request(request)?;
        let biome_response = self.call_biome_api("resources/manage", &biome_request)?;
        self.convert_from_biome_api_response(biome_response)
    }

    /// Handle environment configuration request
    pub fn handle_environment_configuration(
        &self,
        request: &PrimalRequest,
    ) -> Result<PrimalResponse, BearDogError> {
        tracing::info!("🔧 Processing environment configuration request for BiomeOS");

        let biome_request = self.convert_to_biome_api_request(request)?;
        let biome_response = self.call_biome_api("environment/configure", &biome_request)?;
        self.convert_from_biome_api_response(biome_response)
    }

    /// Handle system services request
    pub fn handle_system_services(
        &self,
        request: &PrimalRequest,
    ) -> Result<PrimalResponse, BearDogError> {
        tracing::info!("🔧 Processing system services request for BiomeOS");

        let biome_request = self.convert_to_biome_api_request(request)?;
        let biome_response = self.call_biome_api("system/services", &biome_request)?;
        self.convert_from_biome_api_response(biome_response)
    }

    /// Convert primal request to BiomeOS API format
    fn convert_to_biome_api_request(
        &self,
        request: &PrimalRequest,
    ) -> Result<serde_json::Value, BearDogError> {
        Ok(serde_json::json!({
            "request_id": request.request_id,
            "capability": format!("{:?}", request.capability),
            "parameters": request.parameters,
            "metadata": {
                "source": "beardog",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }
        }))
    }

    /// Call BiomeOS API
    fn call_biome_api(
        &self,
        endpoint: &str,
        request: &serde_json::Value,
    ) -> Result<serde_json::Value, BearDogError> {
        tracing::info!(
            "📡 Making BiomeOS API call to {}/{}",
            self.endpoint,
            endpoint
        );
        tracing::debug!(
            "📤 Request: {}",
            serde_json::to_string_pretty(request).unwrap_or_default()
        );

        // Real HTTP implementation using reqwest
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(
                self.connection_config.timeout_seconds as u64,
            ))
            .build()
            .map_err(|e| BearDogError::network(format!("Failed to create HTTP client: {}", e)))?;

        let url = format!("{}/{}", self.endpoint, endpoint);
        let response = client
            .post(&url)
            .json(request)
            .header(
                "Authorization",
                format!("Bearer {}", self.auth_config.api_key),
            )
            .header("Content-Type", "application/json")
            .send()
            .map_err(|e| BearDogError::network(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(BearDogError::network(format!(
                "BiomeOS API error: {}",
                response.status()
            )));
        }

        let response_json: serde_json::Value = response
            .json()
            .map_err(|e| BearDogError::parsing(format!("Failed to parse response: {}", e)))?;

        tracing::debug!(
            "📥 Response: {}",
            serde_json::to_string_pretty(&response_json).unwrap_or_default()
        );

        Ok(response_json)
    }

    /// Convert BiomeOS API response to primal response
    fn convert_from_biome_api_response(
        &self,
        response: serde_json::Value,
    ) -> Result<PrimalResponse, BearDogError> {
        let universal_response = PrimalResponse {
            request_id: Uuid::new_v4().to_string(),
            success: response.get("status").and_then(|s| s.as_str()) == Some("success"),
            data: serde_json::to_vec(
                &response
                    .get("result")
                    .cloned()
                    .unwrap_or(serde_json::Value::Null),
            )
            .unwrap_or_default(),
            metadata: HashMap::new(),
        };

        Ok(universal_response)
    }

    /// Convert legacy biome.yaml manifest to capability requests
    pub fn convert_manifest_to_requests(
        manifest_content: &str,
    ) -> Result<Vec<PrimalRequest>, BearDogError> {
        tracing::info!("🔄 Converting legacy biome.yaml manifest to capability requests");

        let manifest: serde_yaml::Value =
            serde_yaml::from_str(manifest_content).map_err(|e| {
                BearDogError::validation(format!("Failed to parse biome.yaml: {}", e))
            })?;

        let mut requests = Vec::new();

        if let Some(primals) = manifest.get("primals") {
            requests.extend(Self::convert_primals_section_to_requests(primals)?);
        }

        if let Some(resources) = manifest.get("resources") {
            requests.push(Self::convert_resources_section_to_request(resources)?);
        }

        if let Some(networking) = manifest.get("networking") {
            requests.push(Self::convert_networking_section_to_request(networking)?);
        }

        tracing::info!(
            "✅ Converted manifest to {} capability requests",
            requests.len()
        );

        Ok(requests)
    }

    fn convert_primals_section_to_requests(
        primals: &serde_yaml::Value,
    ) -> Result<Vec<PrimalRequest>, BearDogError> {
        let mut requests = Vec::new();

        if let Some(primals_map) = primals.as_mapping() {
            for (_name, config) in primals_map {
                let primal_request = PrimalRequest {
                    request_id: Uuid::new_v4().to_string(),
                    capability: CapabilityType::ContainerOrchestration,
                    operation: "container_orchestration".to_string(),
                    parameters: {
                        let mut params = HashMap::with_capacity(16);
                        params.insert(
                            "config".to_string(),
                            serde_json::to_value(config).map_err(|e| {
                                BearDogError::internal(format!(
                                    "Failed to convert primal config: {}",
                                    e
                                ))
                            })?,
                        );
                        params
                    },
                    data: Vec::new(),
                    metadata: {
                        let mut metadata = HashMap::with_capacity(16);
                        metadata.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
                        metadata
                    },
                };
                requests.push(primal_request);
            }
        }

        Ok(requests)
    }

    fn convert_resources_section_to_request(
        resources: &serde_yaml::Value,
    ) -> Result<PrimalRequest, BearDogError> {
        Ok(PrimalRequest {
            request_id: Uuid::new_v4().to_string(),
            capability: CapabilityType::ResourceManagement,
            operation: "resource_management".to_string(),
            parameters: {
                let mut params = HashMap::with_capacity(16);
                params.insert(
                    "config".to_string(),
                    serde_json::to_value(resources).map_err(|e| {
                        BearDogError::internal(format!(
                            "Failed to convert resources config: {}",
                            e
                        ))
                    })?,
                );
                params
            },
            data: Vec::new(),
            metadata: {
                let mut metadata = HashMap::with_capacity(16);
                metadata.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
                metadata
            },
        })
    }

    fn convert_networking_section_to_request(
        networking: &serde_yaml::Value,
    ) -> Result<PrimalRequest, BearDogError> {
        Ok(PrimalRequest {
            request_id: Uuid::new_v4().to_string(),
            capability: CapabilityType::EnvironmentConfiguration,
            operation: "environment_configuration".to_string(),
            parameters: {
                let mut params = HashMap::with_capacity(16);
                params.insert(
                    "config".to_string(),
                    serde_json::to_value(networking).map_err(|e| {
                        BearDogError::internal(format!(
                            "Failed to convert networking config: {}",
                            e
                        ))
                    })?,
                );
                params
            },
            data: Vec::new(),
            metadata: {
                let mut metadata = HashMap::with_capacity(16);
                metadata.insert("operation".to_string(), "environment_configuration".to_string());
                metadata.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
                metadata
            },
        })
    }
}

impl BaseProvider for BiomeOSAdapter {
    fn provider_id(&self) -> &str {
        "biome_adapter"
    }

    fn get_capabilities(&self) -> Result<Vec<String>, BearDogError> {
        Ok(vec![
            "container_orchestration".to_string(),
            "resource_management".to_string(),
            "environment_configuration".to_string(),
        ])
    }

    fn initialize(&mut self, _config: ProviderConfig) -> Result<(), BearDogError> {
        Ok(())
    }

    fn health_check(&self) -> Result<ProviderHealthStatus, BearDogError> {
        Ok(ProviderHealthStatus {
            healthy: true,
            last_check: chrono::Utc::now(),
            details: Some("Biome adapter healthy".to_string()),
            response_time_ms: Some(10),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_config_default() {
        let config = BiomeOSConnectionConfig::default();
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_retries, 3);
    }

    #[test]
    fn test_adapter_creation() {
        let auth = BiomeOSAuthConfig {
            api_key: "test-key".to_string(),
            ..Default::default()
        };
        let adapter = BiomeOSAdapter::new("http://localhost:8080", auth);
        assert_eq!(adapter.endpoint, "http://localhost:8080");
    }

    #[test]
    fn test_base_provider() {
        let auth = BiomeOSAuthConfig {
            api_key: "test-key".to_string(),
            ..Default::default()
        };
        let adapter = BiomeOSAdapter::new("http://localhost:8080", auth);
        assert_eq!(adapter.provider_id(), "biome_adapter");

        let caps = adapter.get_capabilities().unwrap();
        assert!(caps.contains(&"container_orchestration".to_string()));
    }
}
