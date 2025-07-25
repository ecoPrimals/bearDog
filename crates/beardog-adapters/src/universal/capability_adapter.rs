//! Universal Capability Adapter - Refactored
//!
//! BearDog's name-agnostic capability adapter for universal service integration.
//! This adapter allows BearDog to register its security capabilities with any
//! service mesh or orchestrator that implements the Universal Primal Architecture Standard.

use super::{
    commercial_extraction::{CommercialClassification, CommercialExtractionDetector},
    service_registration::{SecurityDomain, ServiceCategory, UniversalServiceRegistration},
};
use crate::BearDogResult;

use chrono::Utc;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;

/// BearDog capability adapter for universal ecosystem integration
#[derive(Debug, Clone)]
pub struct BearDogCapabilityAdapter {
    /// Service registration
    pub service_registration: Arc<RwLock<Option<UniversalServiceRegistration>>>,
    /// Commercial extraction detector
    pub extraction_detector: Arc<RwLock<CommercialExtractionDetector>>,
    /// Service capabilities
    pub capabilities: Vec<ServiceCapability>,
    /// Service mesh connector
    pub mesh_connector: Option<ServiceMeshConnector>,
}

/// Service capability definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCapability {
    /// Capability name
    pub name: String,
    /// Capability description
    pub description: String,
    /// Capability version
    pub version: String,
    /// Required parameters
    pub required_params: Vec<String>,
    /// Optional parameters
    pub optional_params: Vec<String>,
}

/// Service mesh connector
#[derive(Debug, Clone)]
pub struct ServiceMeshConnector {
    /// Endpoint URL
    pub endpoint: String,
    /// Authentication token
    pub auth_token: Option<String>,
    /// Connection timeout
    pub timeout: std::time::Duration,
}

/// Universal service provider trait
#[async_trait::async_trait]
pub trait UniversalServiceProvider {
    /// Register service with mesh
    async fn register_service(&self) -> BearDogResult<String>;
    /// Handle incoming requests
    async fn handle_request(
        &self,
        request: crate::adapters::universal::UniversalRequest,
    ) -> BearDogResult<crate::adapters::universal::UniversalResponse>;
    /// Get service capabilities
    fn get_capabilities(&self) -> Vec<ServiceCapability>;
    /// Health check
    async fn health_check(&self) -> BearDogResult<bool>;
}

impl BearDogCapabilityAdapter {
    /// Create new capability adapter
    pub async fn new() -> BearDogResult<Self> {
        let capabilities = vec![
            ServiceCapability {
                name: "encryption".to_string(),
                description: "AES-256-GCM encryption services".to_string(),
                version: "1.0.0".to_string(),
                required_params: vec!["data".to_string()],
                optional_params: vec!["key_id".to_string(), "metadata".to_string()],
            },
            ServiceCapability {
                name: "signature_verification".to_string(),
                description: "Ed25519 digital signature verification".to_string(),
                version: "1.0.0".to_string(),
                required_params: vec![
                    "message".to_string(),
                    "signature".to_string(),
                    "public_key".to_string(),
                ],
                optional_params: vec![],
            },
            ServiceCapability {
                name: "key_management".to_string(),
                description: "Cryptographic key lifecycle management".to_string(),
                version: "1.0.0".to_string(),
                required_params: vec!["operation".to_string()],
                optional_params: vec!["key_type".to_string(), "metadata".to_string()],
            },
            ServiceCapability {
                name: "commercial_extraction_detection".to_string(),
                description: "Revolutionary human vs commercial extraction detection".to_string(),
                version: "1.0.0".to_string(),
                required_params: vec!["request_data".to_string()],
                optional_params: vec!["context".to_string()],
            },
        ];

        Ok(Self {
            service_registration: Arc::new(RwLock::new(None)),
            extraction_detector: Arc::new(RwLock::new(CommercialExtractionDetector::new())),
            capabilities,
            mesh_connector: None,
        })
    }

    /// Set service mesh connector
    pub fn with_mesh_connector(mut self, connector: ServiceMeshConnector) -> Self {
        self.mesh_connector = Some(connector);
        self
    }

    /// Create service registration
    pub async fn create_service_registration(&self) -> UniversalServiceRegistration {
        UniversalServiceRegistration {
            service_id: format!("beardog-security-{}", Uuid::new_v4()),
            version: Version::parse("1.0.0").expect("Valid version"),
            metadata: super::ServiceMetadata {
                name: "BearDog Security Provider".to_string(),
                description: "Decentralized cryptographic security services".to_string(),
                documentation: Some("https://beardog.security/docs".to_string()),
                license: "Proprietary".to_string(),
                tags: vec![
                    "security".to_string(),
                    "cryptography".to_string(),
                    "decentralized".to_string(),
                ],
                properties: HashMap::new(),
                dependencies: vec![],
            },
            capabilities: self.capabilities.iter().map(|c| c.name.clone()).collect(),
            contact_info: super::ContactInfo {
                email: Some("security@beardog.dev".to_string()),
                support_url: Some("https://beardog.security/support".to_string()),
                repository: Some("https://github.com/ecoprimal/beardog".to_string()),
            },
            registered_at: Utc::now(),
            health_endpoint: Some("/health".to_string()),
            category: ServiceCategory::Security,
            security_domain: SecurityDomain::Cryptography,
        }
    }

    /// Analyze request for commercial extraction
    pub async fn analyze_request_for_extraction(
        &self,
        request: &crate::adapters::UniversalRequest,
    ) -> BearDogResult<CommercialClassification> {
        let mut detector = self.extraction_detector.write().await;
        Ok(detector.analyze_request(request).await)
    }
}

#[async_trait::async_trait]
impl UniversalServiceProvider for BearDogCapabilityAdapter {
    async fn register_service(&self) -> BearDogResult<String> {
        let registration = self.create_service_registration().await;
        let service_id = registration.service_id.clone();

        // Store registration
        {
            let mut reg_lock = self.service_registration.write().await;
            *reg_lock = Some(registration);
        }

        info!("BearDog service registered with ID: {}", service_id);
        Ok(service_id)
    }

    async fn handle_request(
        &self,
        request: crate::adapters::universal::UniversalRequest,
    ) -> BearDogResult<crate::adapters::universal::UniversalResponse> {
        // First, analyze for commercial extraction
        let classification = self.analyze_request_for_extraction(&request).await?;

        match classification {
            CommercialClassification::Commercial {
                confidence,
                risk_level,
            } => {
                warn!(
                    "Commercial extraction detected: confidence={}, risk={:?}",
                    confidence, risk_level
                );
                return Ok(crate::adapters::universal::UniversalResponse {
                    success: false,
                    payload: json!({"error": "Commercial extraction detected", "message": "Access restricted due to commercial extraction patterns"}),
                    metadata: std::collections::HashMap::new(),
                    processing_time_ms: 0,
                    system_id: request.system_id.clone(),
                    operation: request.operation.clone(),
                });
            }
            CommercialClassification::Human { confidence } => {
                info!("Human user detected: confidence={}", confidence);
                // Process normally for humans
            }
            CommercialClassification::Uncertain { human_probability } => {
                info!(
                    "Uncertain classification: human_probability={}",
                    human_probability
                );
                // Allow with monitoring
            }
        }

        // Handle the actual request based on capability
        let capability = request
            .payload
            .get("capability")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        match capability {
            "encryption" => self.handle_encryption_request(&request).await,
            "signature_verification" => self.handle_signature_request(&request).await,
            "key_management" => self.handle_key_management_request(&request).await,
            _ => Ok(crate::adapters::universal::UniversalResponse {
                success: false,
                payload: json!({"error": format!("Unknown capability: {}", capability)}),
                metadata: std::collections::HashMap::new(),
                processing_time_ms: 0,
                system_id: request.system_id.clone(),
                operation: request.operation.clone(),
            }),
        }
    }

    fn get_capabilities(&self) -> Vec<ServiceCapability> {
        self.capabilities.clone()
    }

    async fn health_check(&self) -> BearDogResult<bool> {
        // Basic health check - ensure components are responsive
        let _detector = self.extraction_detector.read().await;
        let _registration = self.service_registration.read().await;
        Ok(true)
    }
}

impl BearDogCapabilityAdapter {
    /// Handle encryption requests
    async fn handle_encryption_request(
        &self,
        request: &crate::adapters::universal::UniversalRequest,
    ) -> BearDogResult<crate::adapters::universal::UniversalResponse> {
        // Placeholder for encryption logic
        Ok(crate::adapters::universal::UniversalResponse {
            success: true,
            payload: json!({"encrypted": true, "algorithm": "AES-256-GCM"}),
            metadata: std::collections::HashMap::new(),
            processing_time_ms: 0,
            system_id: request.system_id.clone(),
            operation: request.operation.clone(),
        })
    }

    /// Handle signature verification requests
    async fn handle_signature_request(
        &self,
        request: &crate::adapters::universal::UniversalRequest,
    ) -> BearDogResult<crate::adapters::universal::UniversalResponse> {
        // Placeholder for signature verification logic
        Ok(crate::adapters::universal::UniversalResponse {
            success: true,
            payload: json!({"verified": true, "algorithm": "Ed25519"}),
            metadata: std::collections::HashMap::new(),
            processing_time_ms: 0,
            system_id: request.system_id.clone(),
            operation: request.operation.clone(),
        })
    }

    /// Handle key management requests
    async fn handle_key_management_request(
        &self,
        request: &crate::adapters::universal::UniversalRequest,
    ) -> BearDogResult<crate::adapters::universal::UniversalResponse> {
        // Placeholder for key management logic
        Ok(crate::adapters::universal::UniversalResponse {
            success: true,
            payload: json!({"key_operation": "completed"}),
            metadata: std::collections::HashMap::new(),
            processing_time_ms: 0,
            system_id: request.system_id.clone(),
            operation: request.operation.clone(),
        })
    }
}
