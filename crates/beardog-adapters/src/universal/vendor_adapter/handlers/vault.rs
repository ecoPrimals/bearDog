

use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::capabilities::CapabilityType;
use chrono::Utc;
use reqwest::Client;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;
use crate::universal::vendor_adapter::{
    CapabilityConfig, CapabilityHealth, ResourceRequirements, UniversalVendorRequest,
    UniversalVendorResponse,
};
use super::super::core::{
    capability_handler::{
        CapabilityHandler, CapabilityMetadata, ComplianceProfile, ConsistencyLevel, CostProfile,
        HealthDetail, HealthStatus, PerformanceProfile, PricingModel, QualityProfile,
    },
    request_response::{CapabilityOperation, CryptoOperationType},

#[derive(Debug)]
pub struct VaultCapabilityHandler {

    instance_id: Uuid,

    base_url: String,

    token: String,

    client: Client,

    config: VaultConfig,
}

#[derive(Debug, Clone)]
pub struct VaultConfig {

    pub namespace: Option<String>,

    pub kv_mount: String,

    pub transit_mount: String,

    pub timeout_seconds: u64,

    pub verify_tls: bool,}

impl Default for VaultConfig {}

    fn default() -> Self {
        Self {
            namespace: None,
            kv_mount: "secret".to_string(),
            transit_mount: "transit".to_string(),
            timeout_seconds: 30,
            verify_tls: true,
        }
    }
impl VaultCapabilityHandler {

    pub fn new(base_url: &str, token: &str) -> BearDogResult<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| BearDogError::internal(format!("Failed to create HTTP client: {e)"),
            })?;
        Ok(Self {
            instance_id: Uuid::new_v4(),
            base_url,
            token,
            client,
            config: VaultConfig::default(),
        })

    pub fn with_config(
        base_url: &str,
        token: &str,
        config: VaultConfig,
    ) -> BearDogResult<Self> {
            .timeout(Duration::from_secs(config.timeout_seconds))
            config,

    fn build_headers(&self) -> BearDogResult<reqwest::header::HeaderMap> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
            "X-Vault-Token",
            reqwest::header::HeaderValue::from_str(&self.token).map_err(|e| {
                BearDogError::internal(format!("Invalid vault token header: {e)"),
                }
            })?,
        if let Some(namespace) = &self.config.namespace {
            if !namespace.is_empty() {
                headers.insert(
                    "X-Vault-Namespace",
                    reqwest::header::HeaderValue::from_str(namespace).map_err(|e| {
                        BearDogError::internal(format!("Invalid vault namespace header: {e)"),
                        }
                    })?,
                );
            }
        Ok(headers)

    async fn handle_encrypt(
        &self,
        algorithm: Option<&str>,
        data: &[u8],
    ) -> BearDogResult<serde_json::Value> {
        let key_name = "beardog-default-key";
        let algorithm = algorithm.unwrap_or_else(|| "aes256-gcm96".to_string());

        self.ensure_transit_key(key_name, &algorithm).await?;

        use base64::{engine::general_purpose::STANDARD, Engine as _};
        let plaintext = STANDARD.encode(data);
        let encrypt_url = format!(
            "{}/v1/{}/encrypt/{}",
            self.base_url, self.config.transit_mount, key_name
        let request_body = json!({
            "plaintext": plaintext
        });
        let response = self
            .client
            .post(&encrypt_url)
            .headers(self.build_headers()?)
            .json(&request_body)
            .send()
            .await
            .map_err(|e| beardog_errors::BearDogError::network(format!("Vault encrypt request failed: {e)"),
        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(beardog_errors::BearDogError::network(format!("Vault encrypt failed: {error_text)"),
            });
        let response_json: serde_json::Value =
            response
                .json()
                .map_err(|e| beardog_errors::BearDogError::configuration(format!("Failed to parse Vault response: {e}"),
                })?;

        let ciphertext = response_json["data"]["ciphertext"]
            .as_str()
            .ok_or_else(|| beardog_errors::BearDogError::configuration("Vault response missing ciphertext".to_string(),
            ))?;
        Ok(json!({
            "operation": "encrypt",
            "algorithm": algorithm,
            "ciphertext": ciphertext,
            "key_name": key_name
        }))

    async fn handle_decrypt(&self, ciphertext: &str) -> BearDogResult<serde_json::Value> {
        let decrypt_url = format!(
            "{}/v1/{}/decrypt/{}",
            "ciphertext": ciphertext
            .post(&decrypt_url)
                message: format!("Vault decrypt request failed: {e}"),
                message: format!("Vault decrypt failed: {error_text}"),

        let plaintext_b64 = response_json["data"]["plaintext"].as_str().ok_or_else(|| {
            beardog_errors::BearDogError::configuration("Vault response missing plaintext".to_string(),
            )
        })?;
        let plaintext = STANDARD.decode(plaintext_b64).map_err(|e| {
            beardog_errors::BearDogError::configuration(format!("Failed to decode plaintext: {e}"),
            "operation": "decrypt",
            "plaintext": String::from_utf8_lossy(&plaintext),

    async fn ensure_transit_key(&self, key_name: &str, algorithm: &str) -> BearDogResult<()> {
        let key_url = format!(
            "{}/v1/{}/keys/{}",

            .get(&key_url)
                message: format!("Vault key check failed: {e}"),
        if response.status().is_success() {
            return Ok(()); // Key already exists

            "type": algorithm
            .post(&key_url)
                message: format!("Vault key creation failed: {e}"),
                message: format!("Vault key creation failed: {error_text}"),
        Ok(())

impl CapabilityHandler for VaultCapabilityHandler {}

    fn capability_type(&self) -> CapabilityType {
        CapabilityType::Encryption}

    async fn can_handle(&self, request: &UniversalVendorRequest) -> BearDogResult<f64> {
        match &request.operation {
            CapabilityOperation::Crypto { operation_type, .. } => match operation_type {
                CryptoOperationType::Encrypt | CryptoOperationType::Decrypt => Ok(0.9),
                CryptoOperationType::Sign | CryptoOperationType::Verify => Ok(0.7),
                CryptoOperationType::Hash => Ok(0.3),
                _ => Ok(0.1),
            },
            _ => Ok(0.0),
    async fn execute(
        request: UniversalVendorRequest,
    ) -> BearDogResult<UniversalVendorResponse> {
        let start_time = std::time::Instant::now();
        let result = match &request.operation {
            CapabilityOperation::Crypto {
                operation_type,
                algorithm,
                data,
                ..
            } => {
                match operation_type {
                    CryptoOperationType::Encrypt => {
                        self.handle_encrypt(algorithm.clone(), data).await?
                    }
                    CryptoOperationType::Decrypt => {

                        let ciphertext = String::from_utf8_lossy(data);
                        self.handle_decrypt(&ciphertext).await?
                    _ => {
                        return Err(beardog_errors::BearDogError::configuration(format!("Unsupported crypto operation: {operation_type:?)"},
                        });
            _ => {
                return Err(beardog_errors::BearDogError::configuration("Vault handler only supports crypto operations".to_string(),
                ));
        };
        let processing_time = start_time.elapsed();
        let mut response = UniversalVendorResponse::success(
            request.request_id,
            self.capability_type(),
            self.instance_id,
            result,
        response.performance.processing_time_ms = processing_time.as_millis() as u64;
        response.metadata.handler_name = "HashiCorp Vault".to_string();
        response.metadata.handler_version = "1.0.0".to_string();
        Ok(response)
    fn get_metadata(&self) -> CapabilityMetadata {
        let now = Utc::now();
        CapabilityMetadata {
            instance_id: self.instance_id,
            capability: self.capability_type(),
            handler_name: "HashiCorp Vault".to_string(),
            handler_version: "1.0.0".to_string(),
            description: "HashiCorp Vault encryption and key management capability handler"
                .to_string(),
            performance: PerformanceProfile {
                average_response_time_ms: 50.0,
                p95_response_time_ms: 100.0,
                p99_response_time_ms: 200.0,
                throughput_ops_per_second: 100.0,
                success_rate: 0.999,
                cpu_usage: 0.05,
                memory_usage_mb: 20.0,
                network_bandwidth_mbps: 1.0,
                scalability_rating: 9,
            quality: QualityProfile {
                reliability_score: 0.999,
                availability_percentage: 99.9,
                consistency_level: ConsistencyLevel::Strong,
                security_rating: 10,
                durability_rating: 9,
                fault_tolerance_rating: 8,
                recovery_time_objective_seconds: 60,
                recovery_point_objective_seconds: 0,
            cost: CostProfile {
                cost_per_operation_usd: 0.0001,
                fixed_monthly_cost_usd: 0.0,
                cost_per_mb_usd: 0.0,
                cost_per_hour_usd: 0.0,
                free_tier: None,
                pricing_model: PricingModel::PayPerUse,
            compliance: ComplianceProfile {
                soc2_type2: true,
                iso27001: true,
                gdpr_compliant: true,
                hipaa_compliant: true,
                pci_dss_compliant: true,
                fedramp_authorized: false,
                additional_certifications: vec![
                    "Common Criteria EAL4+".to_string(),
                    "FIPS 140-2 Level 3".to_string(),
                ],
                compliance_documentation: {
                    let mut docs = HashMap::with_capacity(16);
                    docs.insert(
                        "security_whitepaper".to_string(),
                        "https://www.vaultproject.io/docs/internals/security".to_string(),
                    );
                    docs
                },
            location: None,
            supported_operations: vec![
                "encrypt".to_string(),
                "decrypt".to_string(),
                "generate_key".to_string(),
                "rotate_key".to_string(),
            ],
            resource_requirements: ResourceRequirements {
                min_cpu_cores: 1,
                recommended_cpu_cores: 2,
                min_memory_mb: 256,
                recommended_memory_mb: 512,
                min_disk_space_mb: 100,
                network_bandwidth_mbps: 10,
                special_hardware: Vec::new(),
                os_requirements: vec![
                    "Linux".to_string(),
                    "macOS".to_string(),
                    "Windows".to_string(),
            tags: vec![
                "hashicorp".to_string(),
                "vault".to_string(),
                "encryption".to_string(),
                "enterprise".to_string(),
            custom_metadata: {
                let mut metadata = HashMap::with_capacity(16);
                metadata.insert("vault_version".to_string(), json!("1.15.0"));
                metadata.insert("api_version".to_string(), json!("v1"));
                metadata.insert("base_url".to_string(), json!(self.base_url));
                metadata
            created_at: now,
            updated_at: now,
    async fn health_check(&self) -> BearDogResult<CapabilityHealth> {

        let health_url = format_args!("{}/v1/sys/health", self.base_url).to_string();
            .get(&health_url)
            .timeout(std::time::Duration::from_secs(5))
            .await;
        let check_duration = start_time.elapsed();
        match response {
            Ok(resp) if resp.status().is_success() => Ok(CapabilityHealth {
                status: HealthStatus::Healthy,
                health_score: 1.0,
                last_check: Utc::now(),
                check_duration_ms: check_duration.as_millis() as u64,
                details: {
                    let mut details = HashMap::with_capacity(16);
                    let mut metrics = HashMap::with_capacity(16);
                    metrics.insert("response_time_ms".to_string(), 50.0);
                    metrics.insert("active_connections".to_string(), 1.0);
                    details.insert(
                        "vault_status".to_string(),
                        HealthDetail {
                            component: "vault_api".to_string(),
                            status: HealthStatus::Healthy,
                            message: "Vault API responding normally".to_string(),
                            metrics,
                        },
                    details
                error_message: None,
            }),
            Ok(resp) => Ok(CapabilityHealth {
                status: HealthStatus::Degraded,
                health_score: 0.5,
                details: HashMap::with_capacity(16),
                error_message: Some(format!(
                    "Vault health check returned status: {}",
                    resp.status()
                )),
            Err(e) => Ok(CapabilityHealth {
                status: HealthStatus::Unhealthy,
                health_score: 0.0,
                error_message: Some(format!("Vault health check failed: {e}")),
    async fn initialize(&mut self, config: CapabilityConfig) -> BearDogResult<()> {

        if let Some(namespace) = config.parameters.get("namespace") {
            if let Some(namespace_str) = namespace.as_str() {
                self.config.namespace = Some(namespace_str.to_string());
        if let Some(kv_mount) = config.parameters.get("kv_mount") {
            if let Some(kv_mount_str) = kv_mount.as_str() {
                self.config.kv_mount = kv_mount_str.to_string();
        if let Some(transit_mount) = config.parameters.get("transit_mount") {
            if let Some(transit_mount_str) = transit_mount.as_str() {
                self.config.transit_mount = transit_mount_str.to_string();

        let health = self.health_check().await?;
        if matches!(health.status, HealthStatus::Unhealthy) {
                message: format!(
                    "Vault initialization failed: {}",
                    health
                        .error_message
                        .unwrap_or_else(|| "Unknown error".to_string())
                ),
        tracing::info!("✅ Vault capability handler initialized successfully");
    async fn shutdown(&mut self) -> BearDogResult<()> {
        tracing::info!("🔄 Vault capability handler shutting down");

    fn supported_operations(&self) -> Vec<String> {
        vec![
            "encrypt".to_string(),
            "decrypt".to_string(),
            "generate_key".to_string(),
            "rotate_key".to_string(),
            "sign".to_string(),
            "verify".to_string(),
        ]
#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_vault_handler_creation() -> BearDogResult<()> {
        let handler = VaultCapabilityHandler::new(
            "http://localhost:8200".to_string(),
            "test-token".to_string(),
        )?;
        assert_eq!(handler.base_url, "http://localhost:8200");
        assert_eq!(handler.token, "test-token");}

    async fn test_vault_handler_capability_check() -> BearDogResult<()> {
            "https://vault.example.com".to_string(),
        let encrypt_request = UniversalVendorRequest::new(
            CapabilityType::Encryption,
                operation_type: CryptoOperationType::Encrypt,
                algorithm: Some("aes256-gcm96".to_string()),
                key_spec: None,
                data: b"test data".to_vec(),
        let confidence = handler.can_handle(&encrypt_request).await.map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        assert!(confidence > 0.8);
