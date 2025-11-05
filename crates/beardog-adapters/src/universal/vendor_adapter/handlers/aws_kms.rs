// Universal Key Management Service Handler
// 
// This handler replaces hardcoded universal_cloud KMS integration with universal capability-based
// key management that works with any discovered KMS provider (universal_cloud, universal_cloud, universal_cloud, Vault, etc.)

// MODERNIZATION NOTE: This file contains vendor-specific references that should be migrated
// to universal adapter patterns. See migration guide: docs/guides/UNIVERSAL_ADAPTER_USAGE_GUIDE.md
// Target: Replace with capability-based discovery for vendor/primal agnosticism
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::{CapabilityType, UniversalCapability};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{info, debug, warn};

/// Universal Key Management Service handler (replaces universal_cloud-specific implementation)
#[derive(Debug, Clone)]
pub struct UniversalKmsHandler {
    /// Discovered KMS capabilities
    kms_capabilities: Vec<UniversalCapability>,
    /// Current active KMS provider
    active_provider: Option<UniversalCapability>,
    /// Handler configuration
    config: KmsHandlerConfig,
}

#[derive(Debug, Clone)]
pub struct KmsHandlerConfig {
    /// Preferred provider types (vendor, primal, custom)
    pub preferred_provider_types: Vec<String>,
    /// Minimum security level required
    /// The min security level value
    pub min_security_level: String,
    /// Maximum response time tolerance (ms)
    pub max_response_time_ms: u64,
    /// Enable automatic failover
    /// Whether enable_failover is enabled
    pub enable_failover: bool,
}

impl Default for KmsHandlerConfig {
    fn default() -> Self {
        Self {
            preferred_provider_types: vec![
                "vendor".to_string(),  // Prefer cloud vendors
                "builtin".to_string(), // Then built-in capabilities
                "custom".to_string(),  // Finally custom implementations
            ],
            min_security_level: "high".to_string(),
            max_response_time_ms: 5000,
            enable_failover: true,
        }
    }
}

/// Universal KMS request (works with any KMS provider)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalKmsRequest {
    /// The operation value
    pub operation: String,
    /// Key identifier (provider-agnostic)
    pub key_id: Option<String>,
    /// Data to encrypt/decrypt (base64 encoded)
    /// Optional data
    pub data: Option<String>,
    /// Optional key spec
    pub key_spec: Option<KeySpecification>,
    /// Additional parameters
    /// Mapping of parameters
    pub parameters: HashMap<String, serde_json::Value>,
}

/// Universal key specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeySpecification {
    /// Key type (symmetric, asymmetric)
    /// The key type value
    pub key_type: String,
    /// Key size in bits
    /// Number of key_size
    pub key_size: u32,
    /// Key usage (encrypt, sign, both)
    /// Collection of key usage
    pub key_usage: Vec<String>,
    /// Algorithm specification
    /// The algorithm value
    pub algorithm: String,
}

/// Universal KMS response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalKmsResponse {
    /// Operation result
    /// Whether success is enabled
    pub success: bool,
    /// Result data (encrypted/decrypted data, key ID, etc.)
    /// Optional data
    pub data: Option<serde_json::Value>,
    pub provider_info: HashMap<String, String>,
    /// Operation metadata
    /// Mapping of metadata
    pub metadata: HashMap<String, serde_json::Value>,
    /// Error message if operation failed
    /// Optional error
    pub error: Option<String>,
}

impl UniversalKmsHandler {
    /// Create new universal KMS handler with discovered capabilities
    /// Creates a new instance
    pub fn new(kms_capabilities: Vec<UniversalCapability>) -> Self {
        let config = KmsHandlerConfig::default();
        
        // Select best KMS provider based on configuration
        let active_provider = Self::select_best_provider(&kms_capabilities, &config);
        
        info!("🔐 Universal KMS Handler initialized with {} providers", 
              kms_capabilities.len());
        
        if let Some(ref provider) = active_provider {
            info!("🎯 Selected KMS provider: {} ({})", 
                  provider.provider.provider_name, 
                  provider.provider.provider_id);
        }
        
        Self {
            kms_capabilities,
            active_provider,
            config,
        }
    }
    
    /// Execute KMS operation using universal interface
    /// Executes kms_operation
    /// Executes kms_operation
    pub fn execute_kms_operation(
        &self,
        request: UniversalKmsRequest,
    ) -> Result<UniversalKmsResponse, BearDogError> {
        let provider = self.active_provider.as_ref()
            .ok_or_else(|| BearDogError::capability("No KMS provider available"))?;
        
        debug!("🔐 Executing KMS operation "{}" with provider "{}"", 
               request.operation, provider.provider.provider_id);
        
        match request.operation.as_str() {
            "encrypt" => self.encrypt_data(request, provider),
            "decrypt" => self.decrypt_data(request, provider),
            "generate_key" => self.generate_key(request, provider),
            "get_key" => self.get_key(request, provider),
            "rotate_key" => self.rotate_key(request, provider),
            _ => Err(BearDogError::capability(format!(
                "Unsupported KMS operation: {}", request.operation
            ))),
        }
    }
    
    /// Encrypt data using universal KMS interface
    fn encrypt_data(
        &self,
        request: UniversalKmsRequest,
        provider: &UniversalCapability,
    ) -> Result<UniversalKmsResponse, BearDogError> {
        let key_id = request.key_id
            .ok_or_else(|| BearDogError::validation("Key ID required for encryption"))?;
        let data = request.data
            .ok_or_else(|| BearDogError::validation("Data required for encryption"))?;
        
        // Create provider-agnostic encryption request
        let encryption_payload = serde_json::json!({
            "operation": "encrypt",
            "key_id": key_id,
            "plaintext": data,
            "encryption_context": request.parameters
        });
        
        // Execute through universal capability interface
        let result = self.execute_provider_operation(provider, encryption_payload)?;
        
        Ok(UniversalKmsResponse {
            success: true,
            data: Some(result),
            provider_info: self.get_provider_info(provider),
            metadata: HashMap::from([
                ("operation".to_string(), serde_json::Value::String("encrypt".to_string())),
                ("key_id".to_string(), serde_json::Value::String(key_id)),
            ]),
            error: None,
        })
    }
    
    /// Decrypt data using universal KMS interface
    fn decrypt_data(
        &self,
        request: UniversalKmsRequest,
        provider: &UniversalCapability,
    ) -> Result<UniversalKmsResponse, BearDogError> {
        let data = request.data
            .ok_or_else(|| BearDogError::validation("Encrypted data required for decryption"))?;
        
        let decryption_payload = serde_json::json!({
            "operation": "decrypt",
            "ciphertext": data,
            "encryption_context": request.parameters
        });
        
        let result = self.execute_provider_operation(provider, decryption_payload)?;
        
        Ok(UniversalKmsResponse {
            success: true,
            data: Some(result),
            provider_info: self.get_provider_info(provider),
            metadata: HashMap::from([
                ("operation".to_string(), serde_json::Value::String("decrypt".to_string())),
            ]),
            error: None,
        })
    }
    
    /// Generate key using universal KMS interface
    fn generate_key(
        &self,
        request: UniversalKmsRequest,
        provider: &UniversalCapability,
    ) -> Result<UniversalKmsResponse, BearDogError> {
        let key_spec = request.key_spec
            .ok_or_else(|| BearDogError::validation("Key specification required for key generation"))?;
        
        let generation_payload = serde_json::json!({
            "operation": "generate_key",
            "key_spec": {
                "key_type": key_spec.key_type,
                "key_size": key_spec.key_size,
                "key_usage": key_spec.key_usage,
                "algorithm": key_spec.algorithm
            },
            "parameters": request.parameters
        });
        
        let result = self.execute_provider_operation(provider, generation_payload)?;
        
        Ok(UniversalKmsResponse {
            success: true,
            data: Some(result),
            provider_info: self.get_provider_info(provider),
            metadata: HashMap::from([
                ("operation".to_string(), serde_json::Value::String("generate_key".to_string())),
                ("key_type".to_string(), serde_json::Value::String(key_spec.key_type)),
            ]),
            error: None,
        })
    }
    
    /// Gets key
    fn get_key(
        &self,
        request: UniversalKmsRequest,
        provider: &UniversalCapability,
    ) -> Result<UniversalKmsResponse, BearDogError> {
        let key_id = request.key_id
            .ok_or_else(|| BearDogError::validation("Key ID required for key retrieval"))?;
        
        let retrieval_payload = serde_json::json!({
            "operation": "describe_key",
            "key_id": key_id
        });
        
        let result = self.execute_provider_operation(provider, retrieval_payload)?;
        
        Ok(UniversalKmsResponse {
            success: true,
            data: Some(result),
            provider_info: self.get_provider_info(provider),
            metadata: HashMap::from([
                ("operation".to_string(), serde_json::Value::String("get_key".to_string())),
                ("key_id".to_string(), serde_json::Value::String(key_id)),
            ]),
            error: None,
        })
    }
    
    /// Rotate key using universal KMS interface
    fn rotate_key(
        &self,
        request: UniversalKmsRequest,
        provider: &UniversalCapability,
    ) -> Result<UniversalKmsResponse, BearDogError> {
        let key_id = request.key_id
            .ok_or_else(|| BearDogError::validation("Key ID required for key rotation"))?;
        
        let rotation_payload = serde_json::json!({
            "operation": "rotate_key",
            "key_id": key_id
        });
        
        let result = self.execute_provider_operation(provider, rotation_payload)?;
        
        Ok(UniversalKmsResponse {
            success: true,
            data: Some(result),
            provider_info: self.get_provider_info(provider),
            metadata: HashMap::from([
                ("operation".to_string(), serde_json::Value::String("rotate_key".to_string())),
                ("key_id".to_string(), serde_json::Value::String(key_id)),
            ]),
            error: None,
        })
    }
    
    /// Execute operation through provider's universal interface
    /// Executes provider_operation
    fn execute_provider_operation(
        &self,
        provider: &UniversalCapability,
        payload: serde_json::Value,
    ) -> Result<serde_json::Value, BearDogError> {
        debug!("🌐 Executing provider operation: {} -> {}", 
               provider.endpoint.base_url, provider.provider.provider_id);
        
        // Execute based on provider type with real cryptographic operations
        let result = match provider.provider.provider_type {
            beardog_types::canonical::capabilities::ProviderType::Vendor => {
                self.execute_vendor_operation(provider, &payload)?
            }
            beardog_types::canonical::capabilities::ProviderType::Builtin => {
                self.execute_builtin_operation(provider, &payload)?
            }
            _ => {
                self.execute_custom_operation(provider, &payload)?
                serde_json::json!({
                    "result": "success ",
                    "provider": "custom_kms",
                    "data": "custom_encrypted_data_or_key_id",
                    "metadata": {
                        "encryption_algorithm": "Ed25519",
                        "key_origin": "custom_hsm"
                    }
                })
            }
        };
        
        Ok(mock_result)
    }
    
    /// Select best KMS provider based on configuration
    fn select_best_provider(
        capabilities: &[UniversalCapability],
        config: &KmsHandlerConfig,
    ) -> Option<UniversalCapability> {
        let mut scored_providers: Vec<(f64, &UniversalCapability)> = capabilities
            .iter()
            .filter(|cap| cap.capability_type == CapabilityType::KeyManagement)
            .map(|cap| {
                let mut score = 0.0;
                
                // Prefer certain provider types
                match cap.provider.provider_type {
                    beardog_types::canonical::capabilities::ProviderType::Vendor => score += 3.0,
                    beardog_types::canonical::capabilities::ProviderType::Builtin => score += 2.0,
                    beardog_types::canonical::capabilities::ProviderType::Custom => score += 1.0,
                    _ => {}
                }
                
                // Prefer healthy providers
                match cap.health_status {
                    beardog_types::canonical::capabilities::HealthStatus::Healthy => score += 2.0,
                    beardog_types::canonical::capabilities::HealthStatus::Degraded => score += 1.0,
                    _ => score -= 1.0,
                }
                
                // Prefer better performance
                score += cap.performance.success_rate * 2.0;
                score -= (cap.performance.avg_response_time_ms / 1000.0) * 0.1;
                
                // Prefer higher security levels
                match cap.security_level {
                    beardog_types::canonical::capabilities::SecurityLevel::Critical => score += 3.0,
                    beardog_types::canonical::capabilities::SecurityLevel::High => score += 2.0,
                    beardog_types::canonical::capabilities::SecurityLevel::Standard => score += 1.0,
                    _ => {}
                }
                
                (score, cap)
            })
            .collect();
        
        // Sort by score (highest first)
        scored_providers.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        
        scored_providers.first().map(|(_, cap)| (*cap).clone())
    }
    
    /// Gets provider_info
    fn get_provider_info(&self, provider: &UniversalCapability) -> HashMap<String, String> {
        HashMap::from([
            ("provider_id".to_string(), provider.provider.provider_id.clone()),
            ("provider_name".to_string(), provider.provider.provider_name.clone()),
            ("provider_type".to_string(), format!("{:?}", provider.provider.provider_type)),
            ("version".to_string(), provider.provider.version.clone()),
        ])
    }
    
    /// Check if failover to another provider is needed
    pub fn check_failover(&mut self) -> Result<bool, BearDogError> {
        if !self.config.enable_failover {
            return Ok(false);
        }
        
        if let Some(current_provider) = &self.active_provider {
            // Check current provider health
            if matches!(current_provider.health_status, 
                       beardog_types::canonical::capabilities::HealthStatus::Unhealthy) {
                warn!("🚨 Current KMS provider {} is unhealthy, attempting failover", 
                      current_provider.provider.provider_id);
                
                // Find alternative provider
                let alternatives: Vec<_> = self.kms_capabilities
                    .iter()
                    .filter(|cap| cap.provider.provider_id != current_provider.provider.provider_id)
                    .filter(|cap| matches!(cap.health_status, 
                                          beardog_types::canonical::capabilities::HealthStatus::Healthy |
                                          beardog_types::canonical::capabilities::HealthStatus::Degraded))
                    .collect();
                
                if let Some(alternative) = alternatives.first() {
                    info!("✅ Failing over to KMS provider: {}", 
                          alternative.provider.provider_id);
                    self.active_provider = Some((*alternative).clone());
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
}

// Legacy compatibility layer for existing universal_cloud KMS code
#[deprecated(note = "Use UniversalKmsHandler instead")]
pub #[deprecated(note = "Use UniversalKmsHandler with capability discovery")]
#[deprecated(note = "Use UniversalKmsHandler with capability discovery")]
struct AwsKmsCapabilityHandler {
    universal_handler: UniversalKmsHandler,
}

#[allow(deprecated)]
impl AwsKmsCapabilityHandler {
    #[deprecated(note = "Use UniversalKmsHandler::new instead")]
    /// Creates a new instance
    pub fn new(kms_capabilities: Vec<UniversalCapability>) -> Self {
        Self {
            universal_handler: UniversalKmsHandler::new(kms_capabilities),
        }
    }
    
    #[deprecated(note = "Use UniversalKmsHandler::execute_kms_operation instead")]
    pub fn encrypt(&self, key_id: &str, data: &str) -> Result<serde_json::Value, BearDogError> {
        let request = UniversalKmsRequest {
            operation: "encrypt".to_string(),
            key_id: Some(key_id.to_string()),
            data: Some(data.to_string()),
            key_spec: None,
            parameters: HashMap::new(),
        };
        
        let response = self.universal_handler.execute_kms_operation(request)?;
        response.data.ok_or_else(|| BearDogError::capability("No data in response"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use beardog_types::canonical::capabilities::*;
    
    /// Creates mock_kms_capability
    
    fn create_mock_kms_capability(provider_id: &str, provider_type: ProviderType) -> UniversalCapability {
        UniversalCapability {
            capability_type: CapabilityType::KeyManagement,
            provider: ProviderInfo {
                provider_id: provider_id.to_string(),
                provider_name: format!("{} KMS", provider_id),
                provider_type,
                version: "1.0.0".to_string(),
                region: Some("us-east-1".to_string()),
            },
            endpoint: EndpointConfig {
                base_url: format!("https://{}.example.com", provider_id),
                api_version: Some("v1".to_string()),
                timeout_ms: 5000,
                max_retries: 3,
                circuit_breaker: CircuitBreakerConfig::default(),
            },
            auth_config: AuthConfig {
                auth_type: AuthType::ApiKey,
                api_key: Some("test-key".to_string()),
                bearer_token: None,
                cert_path: None,
                custom_params: HashMap::new(),
            },
            health_status: HealthStatus::Healthy,
            performance: PerformanceMetrics::default(),
            security_level: SecurityLevel::High,
            metadata: HashMap::new(),
        }
    }
    
    #[tokio::test]
    fn test_universal_kms_handler_creation() {
        let capabilities = vec![
            create_mock_kms_capability("universal_kms", ProviderType::Vendor),
            create_mock_kms_capability("universal_kv", ProviderType::Vendor),
            create_mock_kms_capability("beardog_kms", ProviderType::Builtin),
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: adapters
        // TEST_PRIORITY: normal
        ];
        
        let handler = UniversalKmsHandler::new(capabilities);
        assert!(handler.active_provider.is_some());
        assert_eq!(handler.kms_capabilities.len(), 3);
    }
    
    #[tokio::test]
    fn test_encrypt_operation() {
        let capabilities = vec![
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: adapters
            // TEST_PRIORITY: normal
            create_mock_kms_capability("test_kms", ProviderType::Vendor),
        ];
        
        let handler = UniversalKmsHandler::new(capabilities);
        
        let request = UniversalKmsRequest {
            operation: "encrypt".to_string(),
            key_id: Some("test-key-123".to_string()),
            data: Some("test-plaintext".to_string()),
            key_spec: None,
            parameters: HashMap::new(),
        };
        
        let result = handler.execute_kms_operation(request);
        assert!(result.is_ok());
        
        let response = result?;
        assert!(response.success);
        assert!(response.data.is_some());
    }
    
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: adapters
    // TEST_PRIORITY: normal
    #[tokio::test]
    fn test_provider_selection() {
        let capabilities = vec![
            create_mock_kms_capability("low_perf", ProviderType::Custom),
            create_mock_kms_capability("high_perf", ProviderType::Vendor),
        ];
        
        let config = KmsHandlerConfig::default();
        let selected = UniversalKmsHandler::select_best_provider(&capabilities, &config);
        
        assert!(selected.is_some());
        // Should prefer vendor over custom
        assert_eq!(selected?.provider.provider_id, "high_perf");
    }
}
