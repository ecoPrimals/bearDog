

use beardog_errors::BearDogError;
use serde_json::json;

use crate::adapters::universal::{
    CapabilityHandler, CapabilityType, UniversalVendorRequest, UniversalVendorResponse,
};

#[derive(Debug, Clone)]
    /// Whether tpm_available is enabled
    pub tpm_available: bool,

    /// Optional device path
    pub device_path: Option<String>,
}

impl Default for TpmCapabilityHandler {
    fn default() -> Self {
        Self {
            tpm_version: "2.0".to_string(), // Will be detected during initialization
            device_path: Some("/dev/tpm0".to_string()),
        }
    }
}

impl TpmCapabilityHandler {

/// New operation.
    /// Creates a new instance
    pub fn new(tpm_version: &str) -> Self {
        Self {
            tpm_version,
            tpm_available: false,
            device_path: Some("/dev/tpm0".to_string()),
        }
    }

/// With Device Path operation.
    /// Creates instance with device path
    pub fn with_device_path(device_path: &str) -> Self {
        Self {
            tpm_version: "2.0".to_string(),
            device_path: Some(device_path),
        }
    }

/// Detect Tpm operation.
///
/// # Errors
/// Returns an error if the operation fails.
    pub fn detect_tpm(&mut self) -> Result<bool, BearDogError> {

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


    fn can_handle(&self, request: &UniversalVendorRequest) -> Result<f64, BearDogError> {

        if let Some(UniversalVendorRequest,
    ) -> Result<UniversalVendorResponse, BearDogError> {
        if !self.tpm_available {
            return Err(BearDogError::security("TPM not available on this system".to_string()));
        }
        
        let operation = request.operation.as_deref().unwrap_or("unknown");
        
        match operation {
            "generate_key" => self.handle_generate_key_request(request),
            "seal" => self.handle_seal_request(request),
            "unseal" => self.handle_unseal_request(request),
            "attest" => self.handle_attest_request(request),
            "quote" => self.handle_quote_request(request),
            "encrypt" => self.handle_encrypt_request(request),
            "decrypt" => self.handle_decrypt_request(request),
            _ => Err(BearDogError::configuration(
                format!("TPM operation "{}" not supported", operation)
            ))
        }
    }
}

impl TpmCapabilityHandler {
    /// Handles generate_key_request
    fn handle_generate_key_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {

        Ok(true,
            payload: json!({
                "operation": "generate_key",
                "provider": "tpm",
                "tpm_version": self.tpm_version,
                "status": "would_generate_hardware_key",
                "key_type": "RSA_2048",
                "hardware_backed": true
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16), // Hardware operations take longer
            system_id: request.system_id,
            operation: request.operation,
        })
    }
    
    /// Handles seal_request
    
    fn handle_seal_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {
        Ok(true,
            payload: json!({
                "operation": "seal",
                "provider": "tpm",
                "status": "would_seal_data_to_pcr",
                "pcr_values": ["0", "1", "2", "3"],
                "sealed": true
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16),
            system_id: request.system_id,
            operation: request.operation,
        })
    }
    
    /// Handles unseal_request
    
    fn handle_unseal_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {
        Ok(true,
            payload: json!({
                "operation": "unseal",
                "provider": "tpm",
                "status": "would_unseal_data_with_pcr_validation",
                "unsealed": true
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16),
            system_id: request.system_id,
            operation: request.operation,
        })
    }
    
    /// Handles attest_request
    
    fn handle_attest_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {
        Ok(true,
            payload: json!({
                "operation": "attest",
                "provider": "tpm",
                "status": "would_provide_attestation",
                "attestation_key": "ak_handle_0x81000001",
                "platform_verified": true
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16), // Attestation is complex
            system_id: request.system_id,
            operation: request.operation,
        })
    }
    
    /// Handles quote_request
    
    fn handle_quote_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {
        Ok(true,
            payload: json!({
                "operation": "quote",
                "provider": "tpm",
                "status": "would_provide_pcr_quote",
                "pcr_selection": "sha256:0,1,2,3,4,5,6,7",
                "quote_signature": "would_be_tpm_signature"
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16),
            system_id: request.system_id,
            operation: request.operation,
        })
    }
    
    /// Handles encrypt_request
    
    fn handle_encrypt_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {
        Ok(true,
            payload: json!({
                "operation": "encrypt",
                "provider": "tpm",
                "status": "would_encrypt_with_tpm_key",
                "algorithm": "RSA_OAEP_SHA256"
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16),
            system_id: request.system_id,
            operation: request.operation,
        })
    }
    
    /// Handles decrypt_request
    
    fn handle_decrypt_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {
        Ok(true,
            payload: json!({
                "operation": "decrypt",
                "provider": "tpm",
                "status": "would_decrypt_with_tpm_key",
                "algorithm": "RSA_OAEP_SHA256"
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16),
            system_id: request.system_id,
            operation: request.operation,
        })
    }
}
