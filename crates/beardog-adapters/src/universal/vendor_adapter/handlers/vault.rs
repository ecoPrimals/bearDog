

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use beardog_errors::BearDogError;
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

#[derive(Debug, Clone)]
    base_url: String,

    token: String,

    client: Client,

    config: VaultConfig,
}

pub use beardog_types::canonical::configuration::VaultConfig;

impl Default for VaultConfig {}

    fn default(None,
            kv_mount: "secret".to_string(),
            transit_mount: "transit".to_string(),
            })?;
        Ok(Self {
            instance_id: Uuid::new_v4(),
            base_url: base_url.to_string(),
            token,
            client,
            config: VaultConfig::default(&str,
        token: &str,
        config: VaultConfig,
    ) -> Result<Self, BearDogError> {
            .timeout(Duration::from_secs(config.timeout_seconds))
            config,

    /// Builds headers
    fn build_headers(&self) -> Result<reqwest::header::HeaderMap, BearDogError> {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::CONTENT_TYPE,
            reqwest::header::HeaderValue::from_static("application/json"),
        );
            "X-Vault-Token",
            reqwest::header::HeaderValue::from_str(&self.token).map_err(|e| {
                BearDogError::internal(format!("Invalid vault token header: {}e"),
                }
            })?,
        if let Some(namespace) = &self.config.namespace {
            if !namespace.is_empty() {
                headers.insert(
                    "X-Vault-Namespace",
                    reqwest::header::HeaderValue::from_str(namespace).map_err(|e| {
                        BearDogError::internal(format!("Invalid vault namespace header: {}e"),
                        }
                    })?,
                );
            }
        Ok(Option<&str>,
        data: &[u8],
    ) -> Result<serde_json::Value, BearDogError> {
        let key_name = "beardog-default-key";
        let algorithm = algorithm.unwrap_or_else(|| "aes256-gcm96".to_string());

        self.ensure_transit_key(key_name, &algorithm)?;

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
            .map_err(|e| beardog_errors::BearDogError::network(format!("Vault encrypt request failed: {}e)"),
        if !response.status().is_success() {
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(beardog_errors::BearDogError::network(format!("Vault encrypt failed: {error_text"),
            });
        let response_json: serde_json::Value =
            response
                .json()
                .map_err(|e| beardog_errors::BearDogError::configuration(format!("Failed to parse Vault response: {e}"),
                })?;

        let ciphertext = response_json["data"]["ciphertext"]
            .as_str()
            .ok_or_else(|| beardog_errors::BearDogError::configuration("encrypt",
            "algorithm": algorithm,
            "ciphertext": ciphertext,
            "key_name": key_name
        }))

    /// Handles decrypt
    fn handle_decrypt(&self, ciphertext: &str) -> Result<serde_json::Value, BearDogError> {
        let decrypt_url = format!(
            "{}/v1/{}/decrypt/{}",
            "ciphertext": ciphertext
            .post(&decrypt_url)
                message: format!("Vault decrypt request failed: {e}"),
                message: format!("Vault decrypt failed: {error_text}"),

        let plaintext_b64 = response_json["data"]["plaintext"].as_str().ok_or_else(|| {
            beardog_errors::BearDogError::configuration("Vault response missing plaintext")
        })?;
        let plaintext = STANDARD.decode(plaintext_b64).map_err(|e| {
            beardog_errors::BearDogError::configuration(format!("Failed to decode plaintext: {e}"),
            "operation": "decrypt",
            "plaintext": String::from_utf8_lossy(&str, algorithm: &str) -> Result<(), BearDogError> {
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


    fn can_handle(&self, request: &UniversalVendorRequest) -> Result<f64, BearDogError> {
        match &request.operation {
            CapabilityOperation::Crypto { operation_type, .. } => match operation_type {
                CryptoOperationType::Encrypt | CryptoOperationType::Decrypt => Ok(0.9),
                CryptoOperationType::Sign | CryptoOperationType::Verify => Ok(0.7),
                CryptoOperationType::Hash => Ok(UniversalVendorRequest,
    ) -> Result<UniversalVendorResponse, BearDogError> {
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
                        self.handle_encrypt(algorithm.clone(), data)?
                    }
                    CryptoOperationType::Decrypt => {

                        let ciphertext = String::from_utf8_lossy(data);
                        self.handle_decrypt(&ciphertext)?
                    _ => {
                        return Err(beardog_errors::BearDogError::configuration({}operation_type:?"},
                        });
            _ => {
                return Err(beardog_errors::BearDogError::configuration("Vault handler only supports crypto operations"));
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
    /// Gets metadata
    fn get_metadata(&self) -> CapabilityMetadata {
        let now = Utc::now(self.instance_id,
            capability: self.capability_type(),
            handler_name: "HashiCorp Vault".to_string(),
            handler_version: "1.0.0".to_string(),
            description: "HashiCorp Vault encryption and key management capability handler"
                .to_string(),
                os_requirements: vec![
                    "Linux".to_string(),
                    "macOS".to_string(),
                    "Windows".to_string(),
            tags: vec![
                "hashicorp".to_string(),
                "vault".to_string(),
                "encryption ".to_string(),
                "enterprise".to_string(),
            custom_metadata: {
                let mut metadata = HashMap::with_capacity(now,
            updated_at: now,
    fn health_check(&self) -> Result<CapabilityHealth, BearDogError> {

        let health_url = format!("{}/v1/sys/health", self.base_url);
            .get(&health_url)
            .timeout(std::time::Duration::from_secs(HealthStatus::Healthy,
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
            }),
            Ok(HealthStatus::Degraded,
                health_score: 0.5,
                details: HashMap::with_capacity(Some(format!(
                    "Vault health check returned status: {}",
                    resp.status(HealthStatus::Unhealthy,
                health_score: 0.0,
                error_message: Some(format!("Vault health check failed: {e}")),
    /// Initializes componentialize
    fn initialize(&mut self, config: CapabilityConfig) -> Result<(), BearDogError> {

        if let Some(namespace) = config.parameters.get("namespace") {
            if let Some(namespace_str) = namespace.as_str() {
                self.config.namespace = Some(namespace_str.to_string());
        if let Some(kv_mount) = config.parameters.get("kv_mount") {
            if let Some(kv_mount_str) = kv_mount.as_str() {
                self.config.kv_mount = kv_mount_str.to_string();
        if let Some(transit_mount) = config.parameters.get("transit_mount") {
            if let Some(transit_mount_str) = transit_mount.as_str() {
                self.config.transit_mount = transit_mount_str.to_string();

        let health = self.health_check()?;
        if matches!(health.status, HealthStatus::Unhealthy) {
                message: format!(
                    "Vault initialization failed: {}",
                    health
                        .error_message
                        .unwrap_or_else(|| "Unknown error".to_string())
                ),
        tracing::info!("✅ Vault capability handler initialized successfully");
    fn shutdown(&mut self) -> Result<(), BearDogError> {
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
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[tokio::test]
    fn test_vault_handler_creation() -> Result<(), BearDogError> {
        let vault_url = std::env::var("VAULT_ADDR")
            .unwrap_or_else(|_| "http://127.0.0.1:8200".to_string());
        let handler = VaultCapabilityHandler::new(
            vault_url.clone(),
            "test-token")?;
        assert_eq!(handler.base_url, vault_url);
        assert_eq!(handler.token, "test-token");}


    fn test_vault_handler_capability_check() -> Result<(), BearDogError> {
            "https://vault.example.com".to_string(),
        let encrypt_request = UniversalVendorRequest::new(
            CapabilityType::Encryption,
                operation_type: CryptoOperationType::Encrypt,
                algorithm: Some(None,
                data: b"test data".to_vec(),
        let confidence = handler.can_handle(&encrypt_request).map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Operation failed: {e:?}"))
        assert!(confidence > 0.8);
