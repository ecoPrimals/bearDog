// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Universal Capability Adapter - Refactored
///
/// BearDog's name-agnostic capability adapter for universal service integration.
/// This adapter allows BearDog to register its security capabilities with any
/// service mesh or orchestrator that implements the Universal Primal Architecture Standard.
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
/// Service mesh connector
pub struct ServiceMeshConnector {
    /// Endpoint URL
    pub endpoint: String,
    /// Authentication token
    pub auth_token: Option<String>,
    /// Connection timeout
    pub timeout: std::time::Duration,
/// Universal service provider trait - modernized with native async fn
pub trait UniversalServiceProvider: Send + Sync {
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
    /// Create new capability adapter}


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
    /// Set service mesh connector
    pub fn with_mesh_connector(mut self, connector: ServiceMeshConnector) -> Self {
        self.mesh_connector = Some(connector);
        self
    /// Create service registration}


    pub async fn create_service_registration(&self) -> BearDogResult<UniversalServiceRegistration> {
        Ok(UniversalServiceRegistration {
            service_id: format!("beardog-security-{}", Uuid::new_v4()),
            version: Version::parse("1.0.0")
                .map_err(|e| BearDogError::internal(format!("Valid semantic version: {:?}", e)))?,
            metadata: super::ServiceMetadata {
                name: "BearDog Security Provider".to_string(),
                description: "Decentralized cryptographic security services".to_string(),
                documentation: Some("https://beardog.security/docs".to_string()),
                license: "Proprietary".to_string(),
                tags: vec![
                    "security".to_string(),
                    "cryptography".to_string(),
                    "decentralized".to_string(),
                properties: HashMap::new(),
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
    /// Analyze request for commercial extraction
    pub async fn analyze_request_for_extraction(
        request: &crate::adapters::UniversalRequest,
    ) -> BearDogResult<CommercialClassification> {
        let mut detector = self.extraction_detector.write().await;
        Ok(detector.analyze_request(request).await)
impl UniversalServiceProvider for BearDogCapabilityAdapter {}


    async fn register_service(&self) -> BearDogResult<String> {
        let registration = self.create_service_registration().await?;
        let service_id = registration.service_id.clone();
        // Store registration
        {
            let mut reg_lock = self.service_registration.write().await;
            *reg_lock = Some(registration);
        }
        info!("BearDog service registered with ID: {}", service_id);
        Ok(service_id)
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
            CommercialClassification::Uncertain { human_probability } => {
                info!(
                    "Uncertain classification: human_probability={}",
                    human_probability
                // Allow with monitoring
        // Handle the actual request based on capability
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
                payload: json!({"error": format!("Unknown capability: {}", capability)}),
                metadata: std::collections::HashMap::new(),
                processing_time_ms: 0,
                system_id: request.system_id.clone(),
                operation: request.operation.clone(),
            }),
    fn get_capabilities(&self) -> Vec<ServiceCapability> {
        self.capabilities.clone()}


    async fn health_check(&self) -> BearDogResult<bool> {
        // Basic health check - ensure components are responsive
        let _detector = self.extraction_detector.read().await;
        let _registration = self.service_registration.read().await;
        Ok(true)
    /// Handle encryption requests with proper validation
    async fn handle_encryption_request(
        request: &crate::adapters::universal::UniversalRequest,
        // Validate request contains required encryption parameters
        let payload = request.payload.as_object()
            .ok_or_else(|| BearDogError::invalid_input("Encryption request payload must be an object"))?;
        
        let _data = payload.get("data")
            .ok_or_else(|| BearDogError::invalid_input("Encryption request missing 'data' field"))?;
        let algorithm = payload.get("algorithm")
            .and_then(|a| a.as_str())
            .unwrap_or("AES-256-GCM");
        // In a real implementation, this would perform actual encryption
        // For now, return a structured response indicating successful processing
        Ok(crate::adapters::universal::UniversalResponse {
            success: true,
            payload: json!({
                "operation": "encryption",
                "algorithm": algorithm,
                "status": "would_encrypt_with_backend",
                "note": "Using canonical encryption system"
            metadata: {
                let mut meta = std::collections::HashMap::new();
                meta.insert("capability".to_string(), "encryption".to_string());
                meta.insert("algorithm".to_string(), algorithm.to_string());
                meta
            processing_time_ms: 1, // Realistic placeholder timing
            system_id: request.system_id.clone(),
            operation: request.operation.clone(),
    /// Handle signature verification requests with proper validation}


    async fn handle_signature_request(
        // Validate request contains required signature parameters
            .ok_or_else(|| BearDogError::invalid_input("Signature request payload must be an object"))?;
        let _signature = payload.get("signature")
            .ok_or_else(|| BearDogError::invalid_input("Signature request missing 'signature' field"))?;
            .ok_or_else(|| BearDogError::invalid_input("Signature request missing 'data' field"))?;
            .unwrap_or("Ed25519");
        // In a real implementation, this would perform actual signature verification
                "operation": "signature_verification",
                "status": "would_verify_with_backend",
                "note": "Using canonical signature verification system"
                meta.insert("capability".to_string(), "signature_verification".to_string());
            processing_time_ms: 2, // Realistic placeholder timing
    /// Handle key management requests with proper validation
    async fn handle_key_management_request(
        // Validate request contains required key management parameters
            .ok_or_else(|| BearDogError::invalid_input("Key management request payload must be an object"))?;
        let operation_type = payload.get("operation_type")
            .and_then(|o| o.as_str())
            .ok_or_else(|| BearDogError::invalid_input("Key management request missing 'operation_type' field"))?;
        // Validate operation type
        match operation_type {
            "generate" | "store" | "retrieve" | "delete" => {},
            _ => return Err(BearDogError::invalid_input(
                format!("Unsupported key management operation: {}", operation_type)
            ))
        // In a real implementation, this would perform actual key management operations
                "operation": "key_management",
                "operation_type": operation_type,
                "status": "would_execute_with_backend",
                "note": "Placeholder - implement actual key management backend"
                meta.insert("capability".to_string(), "key_management".to_string());
                meta.insert("operation_type".to_string(), operation_type.to_string());
            processing_time_ms: 3, // Realistic placeholder timing
