// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// BiomeOS Universal Adapter
///
/// **TECHNICAL DEBT ELIMINATION**: BiomeOS Universal Adapter Migration
/// This adapter treats BiomeOS like any other primal in the ecosystem, routing
/// requests through the universal adapter system instead of hardcoded YAML parsing.
/// ## Migration Benefits
/// - ✅ **Eliminates 987-line biome_yaml_parser.rs** - Replaces with capability-based requests
/// - ✅ **Consolidates 15+ BiomeOS config structs** - Uses standard capability request/response
/// - ✅ **Universal adapter pattern** - BiomeOS treated like ToadStool, Songbird, etc.
/// - ✅ **Capability-based routing** - No hardcoded BiomeOS integration logic
/// ## Capability Mapping
/// BiomeOS provides these capabilities through the universal adapter:
/// - `ContainerOrchestration` - Pod/container deployment and scaling
/// - `ProcessOrchestration` - Service lifecycle management  
/// - `ResourceManagement` - CPU, memory, storage allocation
/// - `EnvironmentConfiguration` - Environment variables and configuration
/// - `SystemServices` - System-level service management

// Removed async_trait - using native async fn for zero-cost abstractions
use beardog_errors::BearDogResult;
use beardog_types::{
    canonical::providers::{ProviderConfig, ProviderHealthStatus},
    capabilities::CapabilityType,
    providers::{BaseProvider, PrimalRequest, PrimalResponse},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
/// **BiomeOS Universal Adapter** - Treats BiomeOS as a standard primal provider
/// Replaces the hardcoded biome_yaml_parser.rs with capability-based requests
pub struct BiomeOSAdapter {
    /// BiomeOS endpoint for API communication
    pub endpoint: String,
    /// Authentication configuration
    pub auth_config: BiomeOSAuthConfig,
    /// Connection configuration
    pub connection_config: BiomeOSConnectionConfig,
}
/// BiomeOS authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeOSAuthConfig {
    /// Authentication method (token, certificate, etc.)
    pub auth_method: String,
    /// Authentication credentials
    pub credentials: HashMap<String, String>,
/// BiomeOS connection configuration
pub struct BiomeOSConnectionConfig {
    /// Connection timeout in seconds
    pub timeout_seconds: u64,
    /// Retry configuration
    pub max_retries: u32,
    /// Health check interval
    pub health_check_interval_seconds: u64,}


impl Default for BiomeOSConnectionConfig {}


    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            max_retries: 3,
            health_check_interval_seconds: 60,
        }
    }
// Uses native async fn from trait definition - no async_trait needed
impl BaseProvider for BiomeOSAdapter {}


    fn provider_id(&self) -> &str {
        "biome_adapter"}


    async fn get_capabilities(&self) -> BearDogResult<Vec<String>> {
        Ok(vec![
            "container_orchestration".to_string(),
            "resource_management".to_string(),
            "environment_configuration".to_string(),
        ])
    async fn initialize(&mut self, _config: ProviderConfig) -> BearDogResult<()> {
        Ok(())}


    async fn health_check(&self) -> BearDogResult<ProviderHealthStatus> {
        Ok(ProviderHealthStatus {
            is_healthy: true,
            last_check: chrono::Utc::now(),
            details: Some("Biome adapter healthy".to_string()),
            response_time_ms: Some(10),
        })
    async fn shutdown(&mut self) -> BearDogResult<()> {
impl BiomeOSAdapter {
    /// Create new BiomeOS adapter}


    #[must_use] pub fn new(endpoint: String, auth_config: BiomeOSAuthConfig) -> Self {
            endpoint,
            auth_config,
            connection_config: BiomeOSConnectionConfig::default(),
    /// Handle container orchestration requests (replaces biome.yaml parsing)}


    async fn handle_container_orchestration(
        &self,
        request: PrimalRequest,
    ) -> BearDogResult<PrimalResponse> {
        tracing::info!("🐳 Processing container orchestration request for BiomeOS");
        // Convert request to BiomeOS API call
        let biome_request = self.convert_to_biome_api_request(&request)?;
        // Make API call to BiomeOS (replaces YAML parsing)
        let biome_response = self
            .call_biome_api("container/orchestrate", &biome_request)
            .await?;
        // Convert response back to standard format
        self.convert_from_biome_api_response(biome_response)
    /// Handle process orchestration requests
    async fn handle_process_orchestration(
        tracing::info!("⚙️ Processing process orchestration request for BiomeOS");
            .call_biome_api("process/orchestrate", &biome_request)
    /// Handle resource management requests
    async fn handle_resource_management(
        tracing::info!("💾 Processing resource management request for BiomeOS");
            .call_biome_api("resources/manage", &biome_request)
    /// Handle environment configuration requests
    async fn handle_environment_configuration(
        tracing::info!("🔧 Processing environment configuration request for BiomeOS");
            .call_biome_api("environment/configure", &biome_request)
    /// Handle system services requests
    async fn handle_system_services(
        tracing::info!("🔧 Processing system services request for BiomeOS");
            .call_biome_api("system/services", &biome_request)
    /// Convert universal request to BiomeOS API format
    fn convert_to_biome_api_request(
        request: &PrimalRequest,
    ) -> BearDogResult<serde_json::Value> {
        // Convert standard capability request to BiomeOS-specific API format
        let biome_request = serde_json::json!({
            "request_id": request.request_id,
            "capability": format!("{:?}", request.capability),
            "parameters": request.parameters,
            "metadata": {
                "source": "beardog",
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }
        });
        Ok(biome_request)
    /// Make API call to BiomeOS
    async fn call_biome_api(
        endpoint: &str,
        request: &serde_json::Value,
        // In real implementation, this would make HTTP request to BiomeOS API
        tracing::debug!(
            "📡 Making BiomeOS API call to {}/{}",
            self.endpoint,
            endpoint
        );
            "📤 Request: {}",
            serde_json::to_string_pretty(request).unwrap_or_default()
        // Mock response for now - in real implementation this would be actual API call
        let mock_response = serde_json::json!({
            "status": "success",
            "result": {
                "operation_id": Uuid::new_v4(),
                "status": "completed",
                "details": "BiomeOS operation completed successfully"
            "📥 Response: {}",
            serde_json::to_string_pretty(&mock_response).unwrap_or_default()
        Ok(mock_response)
    /// Convert BiomeOS API response to universal format
    fn convert_from_biome_api_response(
        response: serde_json::Value,
        // Convert BiomeOS-specific response to standard format
        let universal_response = PrimalResponse {
            request_id: Uuid::new_v4().to_string(), // In real implementation, extract from response
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
/// **Migration Helper Functions** - Assist in migrating from biome_yaml_parser.rs
    /// Convert legacy BiomeManifest to capability requests
    ///
    /// This function helps migrate existing biome.yaml files to the new adapter pattern
    pub fn convert_biome_manifest_to_requests(
        manifest_content: &str,
    ) -> BearDogResult<Vec<PrimalRequest>> {
        tracing::info!("🔄 Converting legacy biome.yaml manifest to capability requests");
        // Parse the YAML content
        let manifest: serde_yaml::Value = serde_yaml::from_str(manifest_content).map_err(|e| {
            beardog_errors::BearDogError::validation(format!("Failed to parse biome.yaml: {e)"),
        })?;
        let mut requests = Vec::new();
        // Convert different sections of the manifest to capability requests
        if let Some(primals) = manifest.get("primals") {
            requests.extend(Self::convert_primals_section_to_requests(primals)?);
        if let Some(resources) = manifest.get("resources") {
            requests.push(Self::convert_resources_section_to_request(resources)?);
        if let Some(networking) = manifest.get("networking") {
            requests.push(Self::convert_networking_section_to_request(networking)?);
        tracing::info!(
            "✅ Converted manifest to {} capability requests",
            requests.len()
        Ok(requests)
    fn convert_primals_section_to_requests(
        primals: &serde_yaml::Value,
        if let Some(primals_map) = primals.as_mapping() {
            for (_name, config) in primals_map {
                // Create primal request for container orchestration
                let primal_request = PrimalRequest {
                    request_id: Uuid::new_v4().to_string(),
                    capability: CapabilityType::ContainerOrchestration,
                    operation: "container_orchestration".to_string(),
                    parameters: {
                        let mut params = HashMap::new();
                        params.insert(
                            "config".to_string(),
                            serde_json::to_value(config).map_err(|e| {
                                beardog_errors::BearDogError::internal(format!("Failed to convert primal config: {e)"),
                                }
                            })?,
                        );
                        params
                    },
                    data: Vec::new(),
                    metadata: {
                        let mut metadata = HashMap::new();
                        metadata.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
                        metadata.insert(
                            "operation".to_string(),
                            "container_orchestration".to_string(),
                        metadata
                };
                requests.push(primal_request);
    fn convert_resources_section_to_request(
        resources: &serde_yaml::Value,
    ) -> BearDogResult<PrimalRequest> {
        Ok(PrimalRequest {
            request_id: Uuid::new_v4().to_string(),
            capability: CapabilityType::ResourceManagement,
            operation: "resource_management".to_string(),
            parameters: {
                let mut params = HashMap::new();
                params.insert(
                    "config".to_string(),
                    serde_json::to_value(resources).map_err(|e| {
                        beardog_errors::BearDogError::internal(format!("Failed to convert resources config: {e)"),
                        }
                    })?,
                );
                params
            },
            data: Vec::new(),
            metadata: {
                let mut metadata = HashMap::new();
                metadata.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
                metadata.insert("operation".to_string(), "resource_management".to_string());
                metadata
    fn convert_networking_section_to_request(
        networking: &serde_yaml::Value,
            capability: CapabilityType::EnvironmentConfiguration,
            operation: "environment_configuration".to_string(),
                    serde_json::to_value(networking).map_err(|e| {
                            message: format!("Failed to convert networking config: {e}"),
                metadata.insert(
                    "operation".to_string(),
                    "environment_configuration".to_string(),
