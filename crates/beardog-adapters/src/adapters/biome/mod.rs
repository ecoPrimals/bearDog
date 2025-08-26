

use beardog_errors::BearDogResult;
use beardog_types::{
    canonical::providers::{ProviderConfig, ProviderHealthStatus},
    capabilities::CapabilityType,
    providers::{BaseProvider, PrimalRequest, PrimalResponse},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub struct BiomeOSAdapter {

    pub endpoint: String,

    pub auth_config: BiomeOSAuthConfig,

    pub connection_config: BiomeOSConnectionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeOSAuthConfig {

    pub auth_method: String,

    pub credentials: HashMap<String, String>,

pub struct BiomeOSConnectionConfig {

    pub timeout_seconds: u64,

    pub max_retries: u32,

    pub health_check_interval_seconds: u64,}

impl Default for BiomeOSConnectionConfig {}

    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            max_retries: 3,
            health_check_interval_seconds: 60,
        }
    }

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

    #[must_use] pub fn new(endpoint: &str, auth_config: BiomeOSAuthConfig) -> Self {
            endpoint,
            auth_config,
            connection_config: BiomeOSConnectionConfig::default(),

    async fn handle_container_orchestration(
        &self,
        request: PrimalRequest,
    ) -> BearDogResult<PrimalResponse> {
        tracing::info!("🐳 Processing container orchestration request for BiomeOS");

        let biome_request = self.convert_to_biome_api_request(&request)?;

        let biome_response = self
            .call_biome_api("container/orchestrate", &biome_request)
            .await?;

        self.convert_from_biome_api_response(biome_response)

    async fn handle_process_orchestration(
        tracing::info!("⚙️ Processing process orchestration request for BiomeOS");
            .call_biome_api("process/orchestrate", &biome_request)

    async fn handle_resource_management(
        tracing::info!("💾 Processing resource management request for BiomeOS");
            .call_biome_api("resources/manage", &biome_request)

    async fn handle_environment_configuration(
        tracing::info!("🔧 Processing environment configuration request for BiomeOS");
            .call_biome_api("environment/configure", &biome_request)

    async fn handle_system_services(
        tracing::info!("🔧 Processing system services request for BiomeOS");
            .call_biome_api("system/services", &biome_request)

    fn convert_to_biome_api_request(
        request: &PrimalRequest,
    ) -> BearDogResult<serde_json::Value> {

        let biome_request = serde_json::json!({
            "request_id": request.request_id,
            "capability": format_args!("{:?}", request.capability).to_string(),
            "parameters": request.parameters,
            "metadata": {
                "source": "beardog",
                "timestamp": chrono::Utc::now().to_rfc3339(),
            }
        });
        Ok(biome_request)

    async fn call_biome_api(
        endpoint: &str,
        request: &serde_json::Value,

        tracing::debug!(
            "📡 Making BiomeOS API call to {}/{}",
            self.endpoint,
            endpoint
        );
            "📤 Request: {}",
            serde_json::to_string_pretty(request).unwrap_or_default()

        let mock_response = serde_json::json!({
            "status": "success",
            "result": {
                "operation_id": Uuid::new_v4(),
                "status": "completed",
                "details": "BiomeOS operation completed successfully"
            "📥 Response: {}",
            serde_json::to_string_pretty(&mock_response).unwrap_or_default()
        Ok(mock_response)

    fn convert_from_biome_api_response(
        response: serde_json::Value,

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
            metadata: HashMap::with_capacity(16),
        };
        Ok(universal_response)

    pub fn convert_biome_manifest_to_requests(
        manifest_content: &str,
    ) -> BearDogResult<Vec<PrimalRequest>> {
        tracing::info!("🔄 Converting legacy biome.yaml manifest to capability requests");

        let manifest: serde_yaml::Value = serde_yaml::from_str(manifest_content).map_err(|e| {
            beardog_errors::BearDogError::validation(format!("Failed to parse biome.yaml: {e)"),
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
            requests.len()
        Ok(requests)
    fn convert_primals_section_to_requests(
        primals: &serde_yaml::Value,
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
                                beardog_errors::BearDogError::internal(format!("Failed to convert primal config: {e)"),
                                }
                            })?,
                        );
                        params
                    },
                    data: Vec::new(),
                    metadata: {
                        let mut metadata = HashMap::with_capacity(16);
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
                let mut params = HashMap::with_capacity(16);
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
                let mut metadata = HashMap::with_capacity(16);
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
