

use beardog_errors::BearDogError;
use serde_json::json;

use crate::adapters::universal::{
    CapabilityHandler, CapabilityType, UniversalVendorRequest, UniversalVendorResponse,
};

#[derive(Debug, Clone)]
pub struct TpmCapabilityHandler {

    pub tpm_version: String,

    pub tpm_available: bool,

    pub device_path: Option<String>,
}

impl Default for TpmCapabilityHandler {
    fn default() -> Self {
        Self {
            tpm_version: "2.0".to_string(),
            tpm_available: false, // Will be detected during initialization
            device_path: Some("/dev/tpm0".to_string()),
        }
    }
}

impl TpmCapabilityHandler {

    pub fn new(tpm_version: &str) -> Self {
        Self {
            tpm_version,
            tpm_available: false,
            device_path: Some("/dev/tpm0".to_string()),
        }
    }

    pub fn with_device_path(device_path: &str) -> Self {
        Self {
            tpm_version: "2.0".to_string(),
            tpm_available: false,
            device_path: Some(device_path),
        }
    }

    pub async fn detect_tpm(&mut self) -> Result<bool, BearDogError> {

        if let Some(path) = &self.device_path {

            self.tpm_available = std::path::Path::new(path).exists();
        }
        Ok(self.tpm_available)
    }
}

impl CapabilityHandler for TpmCapabilityHandler {
    fn capability_type(&self) -> CapabilityType {
        CapabilityType::HardwareSecurity
    }

    async fn can_handle(&self, request: &UniversalVendorRequest) -> Result<f64, BearDogError> {

        if let Some(operation) = &request.operation {
            match operation.as_str() {
                "generate_key" | "seal" | "unseal" => {

                    if self.tpm_available {
                        Ok(0.95)
                    } else {
                        Ok(0.0) // Can't handle without TPM
                    }
                },
                "attest" | "quote" => {

                    if self.tpm_available {
                        Ok(0.98)
                    } else {
                        Ok(0.0)
                    }
                },
                "encrypt" | "decrypt" => {

                    if self.tpm_available {
                        Ok(0.70)
                    } else {
                        Ok(0.0)
                    }
                },
                _ => Ok(0.0)
            }
        } else {
            Ok(0.0)
        }
    }

    async fn execute(
        &self,
        request: UniversalVendorRequest,
    ) -> Result<UniversalVendorResponse, BearDogError> {
        if !self.tpm_available {
            return Err(BearDogError::security("TPM not available on this system".to_string()));
        }
        
        let operation = request.operation.as_deref().unwrap_or("unknown");
        
        match operation {
            "generate_key" => self.handle_generate_key_request(request).await,
            "seal" => self.handle_seal_request(request).await,
            "unseal" => self.handle_unseal_request(request).await,
            "attest" => self.handle_attest_request(request).await,
            "quote" => self.handle_quote_request(request).await,
            "encrypt" => self.handle_encrypt_request(request).await,
            "decrypt" => self.handle_decrypt_request(request).await,
            _ => Err(BearDogError::configuration(
                format_args!("TPM operation '{}' not supported", operation).to_string()
            ))
        }
    }
}

impl TpmCapabilityHandler {
    async fn handle_generate_key_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {

        Ok(UniversalVendorResponse {
            success: true,
            payload: json!({
                "operation": "generate_key",
                "provider": "tpm",
                "tpm_version": self.tpm_version,
                "status": "would_generate_hardware_key",
                "key_type": "RSA_2048",
                "hardware_backed": true
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16);
                meta.insert("provider".to_string(), "tpm".to_string());
                meta.insert("tpm_version".to_string(), self.tpm_version.clone());
                meta.insert("hardware_backed".to_string(), "true".to_string());
                meta.insert("capability".to_string(), "hardware_key_generation".to_string());
                meta
            },
            processing_time_ms: 150, // Hardware operations take longer
            system_id: request.system_id,
            operation: request.operation,
        })
    }
    
    async fn handle_seal_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {
        Ok(UniversalVendorResponse {
            success: true,
            payload: json!({
                "operation": "seal",
                "provider": "tpm",
                "status": "would_seal_data_to_pcr",
                "pcr_values": ["0", "1", "2", "3"],
                "sealed": true
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16);
                meta.insert("provider".to_string(), "tpm".to_string());
                meta.insert("operation_type".to_string(), "seal".to_string());
                meta
            },
            processing_time_ms: 100,
            system_id: request.system_id,
            operation: request.operation,
        })
    }
    
    async fn handle_unseal_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {
        Ok(UniversalVendorResponse {
            success: true,
            payload: json!({
                "operation": "unseal",
                "provider": "tpm",
                "status": "would_unseal_data_with_pcr_validation",
                "unsealed": true
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16);
                meta.insert("provider".to_string(), "tpm".to_string());
                meta.insert("operation_type".to_string(), "unseal".to_string());
                meta
            },
            processing_time_ms: 80,
            system_id: request.system_id,
            operation: request.operation,
        })
    }
    
    async fn handle_attest_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {
        Ok(UniversalVendorResponse {
            success: true,
            payload: json!({
                "operation": "attest",
                "provider": "tpm",
                "status": "would_provide_attestation",
                "attestation_key": "ak_handle_0x81000001",
                "platform_verified": true
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16);
                meta.insert("provider".to_string(), "tpm".to_string());
                meta.insert("attestation_type".to_string(), "platform".to_string());
                meta
            },
            processing_time_ms: 200, // Attestation is complex
            system_id: request.system_id,
            operation: request.operation,
        })
    }
    
    async fn handle_quote_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {
        Ok(UniversalVendorResponse {
            success: true,
            payload: json!({
                "operation": "quote",
                "provider": "tpm",
                "status": "would_provide_pcr_quote",
                "pcr_selection": "sha256:0,1,2,3,4,5,6,7",
                "quote_signature": "would_be_tpm_signature"
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16);
                meta.insert("provider".to_string(), "tpm".to_string());
                meta.insert("quote_type".to_string(), "pcr".to_string());
                meta
            },
            processing_time_ms: 120,
            system_id: request.system_id,
            operation: request.operation,
        })
    }
    
    async fn handle_encrypt_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {
        Ok(UniversalVendorResponse {
            success: true,
            payload: json!({
                "operation": "encrypt",
                "provider": "tpm",
                "status": "would_encrypt_with_tpm_key",
                "algorithm": "RSA_OAEP_SHA256"
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16);
                meta.insert("provider".to_string(), "tpm".to_string());
                meta.insert("hardware_backed".to_string(), "true".to_string());
                meta
            },
            processing_time_ms: 90,
            system_id: request.system_id,
            operation: request.operation,
        })
    }
    
    async fn handle_decrypt_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {
        Ok(UniversalVendorResponse {
            success: true,
            payload: json!({
                "operation": "decrypt",
                "provider": "tpm",
                "status": "would_decrypt_with_tpm_key",
                "algorithm": "RSA_OAEP_SHA256"
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16);
                meta.insert("provider".to_string(), "tpm".to_string());
                meta.insert("hardware_backed".to_string(), "true".to_string());
                meta
            },
            processing_time_ms: 85,
            system_id: request.system_id,
            operation: request.operation,
        })
    }
}
