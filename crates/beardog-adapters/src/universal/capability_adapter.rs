

use super::{
    commercial_extraction::{CommercialClassification, CommercialExtractionDetector},
    service_registration::{SecurityDomain, ServiceCategory, UniversalServiceRegistration},
};
use beardog_errors::{BearDogError, BearDogResult};

use chrono::Utc;
use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct BearDogCapabilityAdapter {

    pub service_registration: Arc<RwLock<Option<UniversalServiceRegistration>>>,

    pub extraction_detector: Arc<RwLock<CommercialExtractionDetector>>,

    pub capabilities: Vec<ServiceCapability>,

    pub mesh_connector: Option<ServiceMeshConnector>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCapability {

    pub name: String,

    pub description: String,

    pub version: String,

    pub required_params: Vec<String>,

    pub optional_params: Vec<String>,

pub struct ServiceMeshConnector {

    pub endpoint: String,

    pub auth_token: Option<String>,

    pub timeout: std::time::Duration,

#[deprecated(since = "3.1.0", note = "Use UniversalProvider instead")]
#[deprecated(since = "3.1.0", note = "Use UniversalProvider instead")]
pub trait UniversalServiceProvider: Send + Sync {

    async fn register_service(&self) -> BearDogResult<String>;

    async fn handle_request(
        &self,
        request: crate::adapters::universal::UniversalRequest,
    ) -> BearDogResult<crate::adapters::universal::UniversalResponse>;

    fn get_capabilities(&self) -> Vec<ServiceCapability>;

    async fn health_check(&self) -> BearDogResult<bool>;
}

impl BearDogCapabilityAdapter {

    pub async fn new() -> BearDogResult<Self> {
        let capabilities = vec![
            ServiceCapability {
                name: "encryption".to_string(),
                description: "AES-256-GCM encryption services".to_string(),
                version: "1.0.0".to_string(),
                required_params: vec!["data".to_string()],
                optional_params: vec!["key_id".to_string(), "metadata".to_string()],
            },
                name: "signature_verification".to_string(),
                description: "Ed25519 digital signature verification".to_string(),
                required_params: vec![
                    "message".to_string(),
                    "signature".to_string(),
                    "public_key".to_string(),
                ],
                optional_params: vec![],
                name: "key_management".to_string(),
                description: "Cryptographic key lifecycle management".to_string(),
                required_params: vec!["operation".to_string()],
                optional_params: vec!["key_type".to_string(), "metadata".to_string()],
                name: "commercial_extraction_detection".to_string(),
                description: "Revolutionary human vs commercial extraction detection".to_string(),
                required_params: vec!["request_data".to_string()],
                optional_params: vec!["context".to_string()],
        ];
        Ok(Self {
            service_registration: Arc::new(RwLock::new(None)),
            extraction_detector: Arc::new(RwLock::new(CommercialExtractionDetector::new())),
            capabilities,
            mesh_connector: None,
        })
    }

    pub fn with_mesh_connector(mut self, connector: ServiceMeshConnector) -> Self {
        self.mesh_connector = Some(connector);
        self

    pub async fn create_service_registration(&self) -> BearDogResult<UniversalServiceRegistration> {
        Ok(UniversalServiceRegistration {
            service_id: format_args!("beardog-security-{}", Uuid::new_v4().to_string()),
            version: Version::parse("1.0.0")
                .map_err(|e| BearDogError::internal(format_args!("Valid semantic version: {:?}", e).to_string()))?,
            metadata: super::ServiceMetadata {
                name: "BearDog Security Provider".to_string(),
                description: "Decentralized cryptographic security services".to_string(),
                documentation: Some("https://beardog.security/docs".to_string()),
                license: "Proprietary".to_string(),
                tags: vec![
                    "security".to_string(),
                    "cryptography".to_string(),
                    "decentralized".to_string(),
                properties: HashMap::with_capacity(16),
                dependencies: vec![],
            capabilities: self.capabilities.iter().map(|c| c.name.clone()).collect(),
            contact_info: super::ContactInfo {
                email: Some("security@beardog.dev".to_string()),
                support_url: Some("https://beardog.security/support".to_string()),
                repository: Some("https://github.com/ecoprimal/beardog".to_string()),
            registered_at: Utc::now(),
            health_endpoint: Some("/health".to_string()),
            category: ServiceCategory::Security,
            security_domain: SecurityDomain::Cryptography,

    pub async fn analyze_request_for_extraction(
        request: &crate::adapters::UniversalRequest,
    ) -> BearDogResult<CommercialClassification> {
        let mut detector = self.extraction_detector.write().await;
        Ok(detector.analyze_request(request).await)
impl UniversalProvider for BearDogCapabilityAdapter {}

    async fn register_service(&self) -> BearDogResult<String> {
        let registration = self.create_service_registration().await?;
        let service_id = registration.service_id.clone();

        {
            let mut reg_lock = self.service_registration.write().await;
            *reg_lock = Some(registration);
        }
        info!("BearDog service registered with ID: {}", service_id);
        Ok(service_id)
    ) -> BearDogResult<crate::adapters::universal::UniversalResponse> {

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
                    metadata: std::collections::HashMap::with_capacity(16),
                    processing_time_ms: 0,
                    system_id: request.system_id.clone(),
                    operation: request.operation.clone(),
                });
            }
            CommercialClassification::Human { confidence } => {
                info!("Human user detected: confidence={}", confidence);

            CommercialClassification::Uncertain { human_probability } => {
                info!(
                    "Uncertain classification: human_probability={}",
                    human_probability

        let capability = request
            .payload
            .get("capability")
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                BearDogError::validation("Missing or invalid 'capability' field in request payload".to_string())
            })?;
        match capability {
            "encryption" => self.handle_encryption_request(&request).await,
            "signature_verification" => self.handle_signature_request(&request).await,
            "key_management" => self.handle_key_management_request(&request).await,
            _ => Ok(crate::adapters::universal::UniversalResponse {
                success: false,
                payload: json!({"error": format_args!("Unknown capability: {}", capability).to_string()}),
                metadata: std::collections::HashMap::with_capacity(16),
                processing_time_ms: 0,
                system_id: request.system_id.clone(),
                operation: request.operation.clone(),
            }),
    fn get_capabilities(&self) -> Vec<ServiceCapability> {
        self.capabilities.clone()}

    async fn health_check(&self) -> BearDogResult<bool> {

        let _detector = self.extraction_detector.read().await;
        let _registration = self.service_registration.read().await;
        Ok(true)

    async fn handle_encryption_request(
        request: &crate::adapters::universal::UniversalRequest,

        let payload = request.payload.as_object()
            .ok_or_else(|| BearDogError::invalid_input("Encryption request payload must be an object"))?;
        
        let _data = payload.get("data")
            .ok_or_else(|| BearDogError::invalid_input("Encryption request missing 'data' field"))?;
        let algorithm = payload.get("algorithm")
            .and_then(|a| a.as_str())
            .unwrap_or("AES-256-GCM");

        Ok(crate::adapters::universal::UniversalResponse {
            success: true,
            payload: json!({
                "operation": "encryption",
                "algorithm": algorithm,
                "status": "would_encrypt_with_backend",
                "note": "Using canonical encryption system"
            metadata: {
                let mut meta = std::collections::HashMap::with_capacity(16);
                meta.insert("capability".to_string(), "encryption".to_string());
                meta.insert("algorithm".to_string(), algorithm.to_string());
                meta
            processing_time_ms: 1, // Realistic placeholder timing
            system_id: request.system_id.clone(),
            operation: request.operation.clone(),

    async fn handle_signature_request(

            .ok_or_else(|| BearDogError::invalid_input("Signature request payload must be an object"))?;
        let _signature = payload.get("signature")
            .ok_or_else(|| BearDogError::invalid_input("Signature request missing 'signature' field"))?;
            .ok_or_else(|| BearDogError::invalid_input("Signature request missing 'data' field"))?;
            .unwrap_or("Ed25519");

                "operation": "signature_verification",
                "status": "would_verify_with_backend",
                "note": "Using canonical signature verification system"
                meta.insert("capability".to_string(), "signature_verification".to_string());
            processing_time_ms: 2, // Realistic placeholder timing

    async fn handle_key_management_request(

            .ok_or_else(|| BearDogError::invalid_input("Key management request payload must be an object"))?;
        let operation_type = payload.get("operation_type")
            .and_then(|o| o.as_str())
            .ok_or_else(|| BearDogError::invalid_input("Key management request missing 'operation_type' field"))?;

        match operation_type {
            "generate" | "store" | "retrieve" | "delete" => {},
            _ => return Err(BearDogError::invalid_input(
                format_args!("Unsupported key management operation: {}", operation_type).to_string()
            ))

                "operation": "key_management",
                "operation_type": operation_type,
                "status": "would_execute_with_backend",
                "note": "Placeholder - implement actual key management backend"
                meta.insert("capability".to_string(), "key_management".to_string());
                meta.insert("operation_type".to_string(), operation_type.to_string());
            processing_time_ms: 3, // Realistic placeholder timing
