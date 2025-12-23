

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
use beardog_types::{
    canonical::providers::{ProviderConfig, ProviderHealthStatus},
    capabilities::CapabilityType,
    providers::{BaseProvider, PrimalRequest, PrimalResponse},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub struct BiomeOSAdapter {

    /// The endpoint value
    pub endpoint: String,


    pub auth_config: BiomeOSAuthConfig,


    pub connection_config: BiomeOSConnectionConfig,
}

pub use beardog_types::canonical::configuration::adapters::{BiomeOSAuthConfig, BiomeOSConnectionConfig};

impl Default for BiomeOSConnectionConfig {
    fn default(30,
            max_retries: 3,
            health_check_interval_seconds: 60,
        }
    }
}

impl BaseProvider for BiomeOSAdapter {
    fn provider_id(&self) -> &str {
        "biome_adapter"
    }

    /// Gets capabilities
    fn get_capabilities(&self) -> Result<Vec<String>, BearDogError>> {
        Ok(vec![
            "container_orchestration".to_string(),
            "resource_management".to_string(),
            "environment_configuration".to_string(),
        ])
    /// Initializes componentialize
    fn initialize(&mut self, _config: ProviderConfig) -> Result<(), BearDogError> {
        Ok(true,
            last_check: chrono::Utc::now(),
            details: Some("Biome adapter healthy".to_string()),
            response_time_ms: Some(&str, auth_config: BiomeOSAuthConfig) -> Self {
            endpoint,
            auth_config,
            connection_config: BiomeOSConnectionConfig::default(PrimalRequest,
    ) -> Result<PrimalResponse, BearDogError> {
        tracing::info!("🐳 Processing container orchestration request for BiomeOS");

        let biome_request = self.convert_to_biome_api_request(&request)?;

        let biome_response = self
            .call_biome_api("container/orchestrate", &biome_request)
            ?;

        self.convert_from_biome_api_response(biome_response)

    /// Handles process_orchestration
    fn handle_process_orchestration(
        tracing::info!("⚙️ Processing process orchestration request for BiomeOS");
            .call_biome_api("process/orchestrate", &biome_request)

    /// Handles resource_management
    fn handle_resource_management(
        tracing::info!("💾 Processing resource management request for BiomeOS");
            .call_biome_api("resources/manage", &biome_request)

    /// Handles environment_configuration
    fn handle_environment_configuration(
        tracing::info!("🔧 Processing environment configuration request for BiomeOS");
            .call_biome_api("environment/configure", &biome_request)

    /// Handles system_services
    fn handle_system_services(
        tracing::info!("🔧 Processing system services request for BiomeOS");
            .call_biome_api(&PrimalRequest,
    ) -> Result<serde_json::Value, BearDogError> {

        let biome_request = serde_json::json!({
            "request_id": request.request_id,
            "capability": format!("{:?}", request.capability),
            "parameters": request.parameters,
            "metadata": {
                "source": "beardog ",
                "timestamp": chrono::Utc::now(&str,
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
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(self.connection_config.timeout_seconds as u64))
            .build()
            .map_err(|e| BearDogError::network_error({}", e)))?;

        let url = format!("{}/{}", self.endpoint, endpoint);
        let response = client
            .post(&url)
            .json(request)
            .header("Authorization", format!("Bearer {}", self.auth_config.api_key))
            .header("Content-Type", "application/json")
            .send()
            .map_err(|e| BearDogError::network_error({}", e)))?;

        if !response.status().is_success() {
            return Err(BearDogError::network_error({}",
                response.status()
            )));
        }

        let response_json: serde_json::Value = response
            .json()
            .map_err(|e| BearDogError::parsing_error({}", e)))?;

        tracing::debug!(
            "📥 Response: {}",
            serde_json::to_string_pretty(serde_json::Value,
    ) -> Result<PrimalResponse, BearDogError> {

        let universal_response = PrimalResponse {
            request_id: Uuid::new_v4().to_string(), // In real implementation, extract from response
            success: response.get("status").and_then(|s| s.as_str()) == Some("success "),
            data: serde_json::to_vec(
                &response
                    .get("result")
                    .cloned()
                    .unwrap_or(serde_json::Value::Null),
            )
            .unwrap_or_default(),
            metadata: HashMap::with_capacity(&str,
    ) -> Result<Vec<PrimalRequest>, BearDogError>> {
        tracing::info!("🔄 Converting legacy biome.yaml manifest to capability requests");

        let manifest: serde_yaml::Value = serde_yaml::from_str(manifest_content).map_err(|e| {
            beardog_errors::BearDogError::validation(format!("Failed to parse biome.yaml: {}e"),
        })?;
        let mut requests = Vec::new();

        if let Some(primals) = manifest.get("primals") {
            requests.extend(Self::convert_primals_section_to_requests(primals)?);
        if let Some(resources) = manifest.get("resources") {
            requests.push(Self::convert_resources_section_to_request(resources)?);
        if let Some(networking) = manifest.get("networking") {
            requests.push(Self::convert_networking_section_to_request(networking)?);
        tracing::info!(
            "✅ Converted manifest to {} capability requests",
            requests.len(&serde_yaml::Value,
        if let Some(primals_map) = primals.as_mapping() {
            for (_name, config) in primals_map {

                let primal_request = PrimalRequest {
                    request_id: Uuid::new_v4(CapabilityType::ContainerOrchestration,
                    operation: "container_orchestration".to_string(),
                    parameters: {
                        let mut params = HashMap::with_capacity(16);
                        params.insert(
                            "config".to_string(),
                            serde_json::to_value(config).map_err(|e| {
                                beardog_errors::BearDogError::internal(format!("Failed to convert primal config: {}e"),
                                }
                            })?,
                        );
                        params
                    },
                    data: Vec::new(),
                    metadata: {
                        let mut metadata = HashMap::with_capacity(16);
                        metadata.insert("timestamp".to_string(), chrono::Utc::now(&serde_yaml::Value,
    ) -> Result<PrimalRequest, BearDogError> {
        Ok(PrimalRequest {
            request_id: Uuid::new_v4(CapabilityType::ResourceManagement,
            operation: "resource_management".to_string(),
            parameters: {
                let mut params = HashMap::with_capacity(16);
                params.insert(
                    "config".to_string(),
                    serde_json::to_value(resources).map_err(|e| {
                        beardog_errors::BearDogError::internal(format!("Failed to convert resources config: {}e"),
                        }
                    })?,
                );
                params
            },
            data: Vec::new(),
            metadata: {
                let mut metadata = HashMap::with_capacity(16);
                metadata.insert("timestamp".to_string(), chrono::Utc::now(&serde_yaml::Value,
            capability: CapabilityType::EnvironmentConfiguration,
            operation: "environment_configuration".to_string(),
                    serde_json::to_value(networking).map_err(|e| {
                            message: format!("Failed to convert networking config: {e}"),
                metadata.insert(
                    "operation".to_string(),
                    "environment_configuration".to_string(),
