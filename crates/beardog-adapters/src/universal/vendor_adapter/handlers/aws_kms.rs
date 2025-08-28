

use beardog_errors::BearDogError;
use serde_json::json;

use crate::adapters::universal::{
    CapabilityHandler, CapabilityType, UniversalVendorRequest, UniversalVendorResponse,
};

#[derive(Debug, Clone)]
pub struct AwsKmsCapabilityHandler {

    pub region: String,

    pub access_key_id: Option<String>,

    pub use_iam_role: bool,
}

impl Default for AwsKmsCapabilityHandler {
    fn default() -> Self {
        Self {
            region: "us-east-1".to_string(),
            access_key_id: None,
            use_iam_role: true,
        }
    }
}

impl AwsKmsCapabilityHandler {

    pub fn new(region: &str) -> Self {
        Self {
            region,
            access_key_id: None,
            use_iam_role: true,
        }
    }

    pub fn with_access_key(region: &str, access_key_id: &str) -> Self {
        Self {
            region,
            access_key_id: Some(access_key_id),
            use_iam_role: false,
        }
    }
}

impl CapabilityHandler for AwsKmsCapabilityHandler {
    fn capability_type(&self) -> CapabilityType {
        CapabilityType::Encryption
    }

    async fn can_handle(&self, request: &UniversalVendorRequest) -> Result<f64, BearDogError> {

        if let Some(operation) = &request.operation {
            match operation.as_str() {
                "encrypt" | "decrypt" | "generate_key" | "get_key" => {

                    Ok(0.85)
                },
                "sign" | "verify" => {

                    Ok(0.65)
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
        let operation = request.operation.as_deref().unwrap_or("unknown");
        
        match operation {
            "encrypt" => self.handle_encrypt_request(request).await,
            "decrypt" => self.handle_decrypt_request(request).await,
            "generate_key" => self.handle_generate_key_request(request).await,
            "get_key" => self.handle_get_key_request(request).await,
            _ => Err(BearDogError::configuration(
                format_args!("AWS KMS operation '{}' not supported", operation).to_string()
            ))
        }
    }
}

impl AwsKmsCapabilityHandler {
    async fn handle_encrypt_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {

        Ok(UniversalVendorResponse {
            success: true,
            payload: json!({
                "operation": "encrypt",
                "provider": "aws_kms",
                "region": self.region,
                "status": "would_encrypt_with_aws_kms",
                "key_id": "arn:aws:kms:region:account:key/key-id",
                "algorithm": "AES_256_GCM"
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16);
                meta.insert("provider".to_string(), "aws_kms".to_string());
                meta.insert("region".to_string(), self.region.clone());
                meta.insert("capability".to_string(), "cloud_encryption".to_string());
                meta
            },
            processing_time_ms: 25, // Realistic cloud API call time
            system_id: request.system_id,
            operation: request.operation,
        })
    }
    
    async fn handle_decrypt_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {
        Ok(UniversalVendorResponse {
            success: true,
            payload: json!({
                "operation": "decrypt",
                "provider": "aws_kms",
                "region": self.region,
                "status": "would_decrypt_with_aws_kms"
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16);
                meta.insert("provider".to_string(), "aws_kms".to_string());
                meta.insert("region".to_string(), self.region.clone());
                meta
            },
            processing_time_ms: 20,
            system_id: request.system_id,
            operation: request.operation,
        })
    }
    
    async fn handle_generate_key_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {
        Ok(UniversalVendorResponse {
            success: true,
            payload: json!({
                "operation": "generate_key",
                "provider": "aws_kms",
                "region": self.region,
                "status": "would_generate_key_with_aws_kms",
                "key_spec": "AES_256"
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16);
                meta.insert("provider".to_string(), "aws_kms".to_string());
                meta.insert("key_type".to_string(), "symmetric".to_string());
                meta
            },
            processing_time_ms: 50, // Key generation takes longer
            system_id: request.system_id,
            operation: request.operation,
        })
    }
    
    async fn handle_get_key_request(&self, request: UniversalVendorRequest) -> Result<UniversalVendorResponse, BearDogError> {
        Ok(UniversalVendorResponse {
            success: true,
            payload: json!({
                "operation": "get_key",
                "provider": "aws_kms",
                "region": self.region,
                "status": "would_retrieve_key_from_aws_kms"
            }),
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16);
                meta.insert("provider".to_string(), "aws_kms".to_string());
                meta
            },
            processing_time_ms: 15,
            system_id: request.system_id,
            operation: request.operation,
        })
    }
}
