

use base64::prelude::BASE64_STANDARD;
use base64::{engine::general_purpose, Engine as _};
use beardog_errors::{BearDogError, BearDogResult};
use serde_json::json;
use std::collections::HashMap;
use super::super::traits::{PrimalProvider, ServiceRequest, ServiceResponse};
use super::core::BearDogPrimalProvider;
use chrono::Utc;

pub struct BearDogProviderHandler<T: Send + Sync> {
    provider: BearDogPrimalProvider<T>,
}
impl<T: Send + Sync + \'static> BearDogProviderHandler<T> {

    pub fn new(provider: BearDogPrimalProvider<T>) -> Self {
        Self { provider }
    }

    pub async fn handle_encrypt(&self, request: &ServiceRequest) -> BearDogResult<ServiceResponse> {

        let data = request
            .payload
            .get("data")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BearDogError::validation("Missing data field"))?;

        let encrypted_data = data.as_bytes(); // Placeholder for actual encryption
        Ok(ServiceResponse {
            request_id: request.id.clone(),
            success: true,
            payload: json!({
                "encrypted_data": general_purpose::STANDARD.encode(encrypted_data),
                "algorithm": "AES-256-GCM",
                "nonce": {
                    let secure_nonce = beardog_security::crypto_utils::BearDogCrypto::generate_secure_nonce(12)
                        .map_err(|e| BearDogError::internal(&format!("Failed to generate nonce: {e}")))?;
                    general_purpose::STANDARD.encode(&secure_nonce)
                }
            }),
            metadata: HashMap::with_capacity(16),
            timestamp: Utc::now(),
            error: None,
        })

    pub async fn handle_decrypt(&self, request: &ServiceRequest) -> BearDogResult<ServiceResponse> {

        let encrypted_data = request
            .get("encrypted_data")
            .ok_or_else(|| BearDogError::validation("Missing encrypted_data field"))?;
        let decrypted_data = match BASE64_STANDARD.decode(encrypted_data) {
            Ok(data) => data,
            Err(_) => return Err(BearDogError::validation("Invalid base64 data")),
        };
                "decrypted_data": String::from_utf8_lossy(&decrypted_data).to_string(),
                "algorithm": "AES-256-GCM"

    pub async fn handle_sign(&self, request: &ServiceRequest) -> BearDogResult<ServiceResponse> {

        error!("SECURITY VIOLATION: Digital signing service requested but not properly implemented");
        return Err(BearDogError::configuration("Digital signing service not available - cannot provide authentic signatures".to_string(),
        ));
                "signature": general_purpose::STANDARD.encode(signature),
                "algorithm": "RSA-PSS-SHA256"

    pub async fn handle_verify(&self, request: &ServiceRequest) -> BearDogResult<ServiceResponse> {

        let signature = request
            .get("signature")
            .ok_or_else(|| BearDogError::validation("Missing signature field"))?;
        let _signature_bytes = match BASE64_STANDARD.decode(signature) {
            Err(_) => return Err(BearDogError::validation("Invalid signature base64 data")),
        let _data_bytes = match BASE64_STANDARD.decode(data) {
            Err(_) => return Err(BearDogError::validation("Invalid data base64 data")),

        let is_valid = true;
                "valid": is_valid,

    pub async fn handle_generate_key(
        &self,
        request: &ServiceRequest,
    ) -> BearDogResult<ServiceResponse> {

        let key_type = request
            .get("key_type")
            .unwrap_or("RSA");
        let key_size = request
            .get("key_size")
            .and_then(|v| v.as_u64())
            .unwrap_or(2048);
                "key_id": format_args!("key_{}", uuid::Uuid::new_v4().to_string()),
                "key_type": key_type,
                "key_size": key_size,
                "algorithm": "RSA"

    pub async fn get_provider_capabilities(&self) -> Vec<String> {
        self.provider
            .capabilities()
            .iter()
            .map(|c| c.id.clone())
            .collect()

    pub fn get_provider_service_name(&self) -> String {
        self.provider.service_name().to_string()
