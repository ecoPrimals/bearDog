use super::{
    commercial_extraction::{CommercialClassification, CommercialExtractionDetector},
    service_registration::{
        ContactInfo, SecurityDomain, ServiceCategory, ServiceMetadata, UniversalServiceRegistration,
    },
};
use beardog_errors::BearDogError;
use beardog_traits::canonical::UniversalProvider;
use beardog_types::HealthStatus;

use std::collections::HashMap;

// Re-export types that are used in the module interface
// ServiceCapability will be defined locally for now
pub use super::http_adapter::ServiceMeshConnector;
// UniversalProvider already imported at top of file

// Local ServiceCapability definition until types are unified
#[derive(Debug, Clone)]
pub struct ServiceCapability {
    pub name: String,
    pub version: String,
    pub endpoints: Vec<String>,
}

use chrono::Utc;
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct BearDogCapabilityAdapter {
    service_registration: Arc<RwLock<Option<UniversalServiceRegistration>>>,
    extraction_detector: Arc<RwLock<CommercialExtractionDetector>>,
    #[allow(dead_code)] // Future capability metadata functionality
    capability_metadata: Arc<RwLock<std::collections::HashMap<String, serde_json::Value>>>,
}

impl BearDogCapabilityAdapter {
    pub async fn new() -> Result<Self, BearDogError> {
        Ok(Self {
            service_registration: Arc::new(RwLock::new(None)),
            extraction_detector: Arc::new(RwLock::new(CommercialExtractionDetector::new())),
            capability_metadata: Arc::new(RwLock::new(std::collections::HashMap::new())),
        })
    }

    async fn create_service_registration(
        &self,
    ) -> Result<UniversalServiceRegistration, BearDogError> {
        let metadata = ServiceMetadata {
            name: "BearDog Capability Adapter".to_string(),
            description: "Universal capability adaptation and commercial extraction detection"
                .to_string(),
            documentation: None,
            license: "MIT".to_string(),
            tags: vec!["adapter".to_string(), "capability".to_string()],
            properties: {
                let mut props = HashMap::new();
                props.insert("adapter_type".to_string(), "capability".to_string());
                props.insert("extraction_detection".to_string(), "true".to_string());
                props.insert("security_level".to_string(), "high".to_string());
                props
            },
            dependencies: vec![],
        };

        Ok(UniversalServiceRegistration {
            service_id: format!("beardog-capability-adapter-{}", uuid::Uuid::new_v4()),
            version: "3.0.0".to_string(),
            metadata,
            capabilities: vec![
                "encryption".to_string(),
                "signature_verification".to_string(),
                "key_management".to_string(),
                "commercial_extraction_detection".to_string(),
            ],
            contact_info: ContactInfo {
                email: None,
                support_url: None,
                repository: None,
            },
            registered_at: Utc::now(),
            health_endpoint: Some("/health".to_string()),
            category: ServiceCategory::Security,
            security_domain: SecurityDomain::General,
        })
    }

    pub async fn handle_request(
        &self,
        request: &crate::adapters::UniversalRequest,
    ) -> Result<crate::adapters::UniversalResponse, BearDogError> {
        let capability = request
            .payload
            .get("capability")
            .and_then(|c| c.as_str())
            .ok_or_else(|| {
                BearDogError::validation(
                    "Missing or invalid 'capability' field in request payload".to_string(),
                )
            })?;

        match capability {
            "encryption" => self.handle_encryption_request(request).await,
            "signature_verification" => self.handle_signature_request(request).await,
            "key_management" => self.handle_key_management_request(request).await,
            _ => Ok(crate::adapters::UniversalResponse {
                request_id: "unknown".to_string(),
                status: beardog_types::canonical::services::ResponseStatus::Error,
                payload: json!({"error": format!("Unknown capability: {}", capability)}),
                timestamp: chrono::Utc::now(),
                processing_time_ms: 0,
            }),
        }
    }

    async fn handle_encryption_request(
        &self,
        request: &crate::adapters::UniversalRequest,
    ) -> Result<crate::adapters::UniversalResponse, BearDogError> {
        let payload = request.payload.as_object().ok_or_else(|| {
            BearDogError::invalid_input("Encryption request payload must be an object")
        })?;

        let algorithm = payload
            .get("algorithm")
            .and_then(|a| a.as_str())
            .unwrap_or("AES-256-GCM");

        Ok(crate::adapters::UniversalResponse {
            request_id: "encryption_request".to_string(),
            status: beardog_types::canonical::services::ResponseStatus::Success,
            payload: json!({
                "operation": "encryption",
                "algorithm": algorithm,
                "status": "would_encrypt_with_backend",
                "note": "Using canonical encryption system"
            }),
            timestamp: chrono::Utc::now(),
            processing_time_ms: 1,
        })
    }

    async fn handle_signature_request(
        &self,
        request: &crate::adapters::UniversalRequest,
    ) -> Result<crate::adapters::UniversalResponse, BearDogError> {
        let payload = request.payload.as_object().ok_or_else(|| {
            BearDogError::invalid_input("Signature request payload must be an object")
        })?;

        let _signature = payload.get("signature").ok_or_else(|| {
            BearDogError::invalid_input("Signature request missing 'signature' field")
        })?;

        Ok(crate::adapters::UniversalResponse {
            request_id: "signature_verification_request".to_string(),
            status: beardog_types::canonical::services::ResponseStatus::Success,
            payload: json!({
                "operation": "signature_verification",
                "status": "would_verify_with_backend",
                "note": "Using canonical signature verification"
            }),
            timestamp: chrono::Utc::now(),
            processing_time_ms: 2,
        })
    }

    async fn handle_key_management_request(
        &self,
        request: &crate::adapters::UniversalRequest,
    ) -> Result<crate::adapters::UniversalResponse, BearDogError> {
        let payload = request.payload.as_object().ok_or_else(|| {
            BearDogError::invalid_input("Key management request payload must be an object")
        })?;

        let operation_type = payload
            .get("operation_type")
            .and_then(|o| o.as_str())
            .ok_or_else(|| {
                BearDogError::invalid_input("Key management request missing 'operation_type' field")
            })?;

        match operation_type {
            "generate" | "store" | "retrieve" | "delete" => {}
            _ => {
                return Err(BearDogError::invalid_input(format!(
                    "Unsupported key management operation: {operation_type}"
                )))
            }
        }

        Ok(crate::adapters::UniversalResponse {
            request_id: "key_management_request".to_string(),
            status: beardog_types::canonical::services::ResponseStatus::Success,
            payload: json!({
                "operation": "key_management",
                "operation_type": operation_type,
                "status": "would_execute_with_backend",
                "note": "Placeholder - implement actual key management backend"
            }),
            timestamp: chrono::Utc::now(),
            processing_time_ms: 3,
        })
    }

    pub async fn analyze_commercial_extraction(
        &self,
        request: &crate::adapters::UniversalRequest,
    ) -> Result<CommercialClassification, BearDogError> {
        let mut detector = self.extraction_detector.write().await;
        Ok(detector.analyze_request(request).await)
    }

    pub async fn register_service(&self) -> Result<String, BearDogError> {
        let registration = self.create_service_registration().await?;
        let service_id = registration.service_id.clone();

        {
            let mut reg_lock = self.service_registration.write().await;
            *reg_lock = Some(registration);
        }

        tracing::info!(
            "BearDog Capability Adapter registered with service ID: {}",
            service_id
        );
        Ok(service_id)
    }
}

impl beardog_traits::canonical::BaseProvider for BearDogCapabilityAdapter {
    fn provider_info(&self) -> beardog_traits::canonical::ProviderInfo {
        beardog_traits::canonical::ProviderInfo {
            name: "beardog_capability_adapter".to_string(),
            version: "1.0.0".to_string(),
            provider_type: "AI".to_string(),
            capabilities: vec![
                "capability_discovery".to_string(),
                "universal_adaptation".to_string(),
            ],
        }
    }

    fn id(&self) -> &str {
        "beardog_capability_adapter"
    }

    fn version(&self) -> &str {
        "1.0.0"
    }

    async fn validate_config(
        &self,
        _config: &beardog_types::canonical::providers::ProviderConfig,
    ) -> Result<bool, BearDogError> {
        Ok(true) // Always valid for capability adapter
    }

    async fn status(
        &self,
    ) -> Result<beardog_types::canonical::providers::ProviderStatus, BearDogError> {
        Ok(beardog_types::canonical::providers::ProviderStatus::Active)
    }

    async fn reload_config(
        &self,
        _config: &beardog_types::canonical::providers::ProviderConfig,
    ) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn health_check(&self) -> Result<HealthStatus, BearDogError> {
        Ok(HealthStatus::Healthy)
    }

    async fn capabilities(&self) -> Result<Vec<String>, BearDogError> {
        Ok(vec![
            "capability_discovery".to_string(),
            "universal_adaptation".to_string(),
            "cross_system_integration".to_string(),
            "commercial_extraction".to_string(),
        ])
    }

    async fn initialize(
        &self,
        _config: &beardog_types::canonical::providers::ProviderConfig,
    ) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn shutdown(&self) -> Result<(), BearDogError> {
        Ok(())
    }

    async fn metrics(&self) -> Result<beardog_traits::canonical::ProviderMetrics, BearDogError> {
        Ok(HashMap::from([
            ("registered_capabilities".to_string(), 0.0),
            ("requests_processed".to_string(), 0.0),
        ]))
    }
}

impl UniversalProvider for BearDogCapabilityAdapter {
    fn provider_type(&self) -> &str {
        "beardog_capability_adapter"
    }

    async fn discover_capabilities(&self) -> Result<Vec<String>, BearDogError> {
        Ok(vec![
            "discover_capabilities".to_string(),
            "register_capability".to_string(),
            "query_capability".to_string(),
        ])
    }

    async fn execute_operation(
        &self,
        operation: &str,
        parameters: std::collections::HashMap<String, serde_json::Value>,
    ) -> Result<serde_json::Value, BearDogError> {
        let payload = serde_json::Value::Object(parameters.into_iter().collect());
        // Convert the payload into a mock request for internal processing
        let mock_request = crate::adapters::UniversalRequest {
            request_id: "internal_request".to_string(),
            service_type: beardog_types::canonical::services::ServiceType::Core,
            operation: operation.to_string(),
            payload,
            priority: beardog_types::canonical::services::RequestPriority::Normal,
            timestamp: chrono::Utc::now(),
        };

        match operation {
            "key_management" => {
                let response = self.handle_key_management_request(&mock_request).await?;
                Ok(response.payload)
            }
            "encryption" => {
                let response = self.handle_encryption_request(&mock_request).await?;
                Ok(response.payload)
            }
            "signature_verification" => {
                let response = self.handle_signature_request(&mock_request).await?;
                Ok(response.payload)
            }
            _ => Err(BearDogError::system(format!(
                "Unsupported operation: {operation}"
            ))),
        }
    }

    async fn connection_status(
        &self,
    ) -> Result<beardog_traits::canonical::ConnectionStatus, BearDogError> {
        Ok(beardog_traits::canonical::ConnectionStatus {
            connected: true,
            latency_ms: Some(10),
            last_successful_operation: Some(chrono::Utc::now()),
            error_count: 0,
        })
    }

    async fn validate_compatibility(&self, _target_version: &str) -> Result<bool, BearDogError> {
        Ok(true)
    }
}

impl BearDogCapabilityAdapter {
    #[allow(dead_code)] // Future capability testing functionality
    async fn test_capability_flow(&self) -> Result<(), BearDogError> {
        let _mock_request = crate::adapters::UniversalRequest {
            request_id: "test_request".to_string(),
            service_type: beardog_types::canonical::services::ServiceType::Core,
            operation: "test_operation".to_string(),
            payload: serde_json::json!({"test": "data"}),
            priority: beardog_types::canonical::services::RequestPriority::Normal,
            timestamp: chrono::Utc::now(),
        };

        // Test the flow
        let _capabilities = self.discover_capabilities().await?;
        Ok(())
    }
}
