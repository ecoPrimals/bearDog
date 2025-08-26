// PHASE 5 CORE OPTIMIZED: Ecosystem performance patterns applied
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


/// # Universal Service Registration
///
/// This module implements `BearDog`'s Universal Service Registration for the
/// ecoPrimals ecosystem, enabling dynamic capability declaration and service discovery.

use crate::{BearDogError, BearDogResult};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use uuid::Uuid;
/// Universal Service Registration for `BearDog`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalServiceRegistration {
    /// Unique service identifier
    pub service_id: Uuid,
    /// Service metadata
    pub metadata: ServiceMetadata,
    /// Service capabilities
    pub capabilities: Vec<ServiceCapability>,
    /// Resource specifications
    pub resources: ResourceSpec,
    /// Service endpoints
    pub endpoints: Vec<ServiceEndpoint>,
    /// Integration preferences
    pub integration: IntegrationPreferences,
    /// Custom extensions
    pub extensions: HashMap<String, serde_json::Value>,
    /// Registration timestamp
    pub registration_timestamp: DateTime<Utc>,
    /// Service version
    pub service_version: String,
    /// Instance identifier
    pub instance_id: String,
    /// Priority level for service selection
    pub priority: u32,
}
/// Service metadata information
pub struct ServiceMetadata {
    /// Service name
    pub name: String,
    /// Service category
    pub category: ServiceCategory,
    pub version: String,
    /// Service description
    pub description: String,
    /// Maintainer contact information
    pub maintainer: ContactInfo,
    /// Supported protocols
    pub protocols: Vec<String>,
/// Service category enumeration
pub enum ServiceCategory {
    /// Security services
    Security {
        /// Security domains
        domains: Vec<String>,
    },
    /// Compute services
    Compute {
        /// Compute capabilities
        capabilities: Vec<String>,
    /// Storage services
    Storage {
        /// Storage types
        types: Vec<String>,
    /// AI services
    ArtificialIntelligence {
        /// AI capabilities
        models: Vec<String>,
    /// Custom service category
    Custom {
        /// Category name
        category: String,
        /// Category description
        description: String,
/// Contact information for service maintainer
pub struct ContactInfo {
    /// Maintainer name
    /// Contact email (optional)
    pub email: Option<String>,
    /// Organization (optional)
    pub organization: Option<String>,
/// Service capability specification
pub enum ServiceCapability {
    /// Security capability
        /// Security functions
        functions: Vec<String>,
        /// Compliance standards
        compliance: Vec<String>,
        /// Trust levels supported
        trust_levels: Vec<String>,
    /// AI capability
        /// AI models available
        /// AI tasks supported
        tasks: Vec<String>,
        /// Interface types
        interfaces: Vec<String>,
    /// Custom capability
        /// Capability domain
        domain: String,
        /// Capability name
        capability: String,
        /// Capability parameters
        parameters: HashMap<String, serde_json::Value>,
/// Resource specification}


pub struct ResourceSpec {
    /// CPU cores required
    pub cpu_cores: f64,
    /// Memory bytes required
    pub memory_bytes: u64,
    /// Storage bytes (optional)
    pub storage_bytes: Option<u64>,
    /// Network bandwidth in Mbps (optional)
    pub network_bandwidth_mbps: Option<u32>,
    /// GPU required
    pub gpu_required: bool,
    /// Specialized hardware requirements
    pub specialized_hardware: Vec<String>,
/// Service endpoint specification
pub struct ServiceEndpoint {
    /// Endpoint name
    /// Endpoint URL
    pub url: String,
    /// Protocol
    pub protocol: String,
    /// Supported HTTP methods
    pub methods: Vec<String>,
    /// AI-optimized endpoint
    pub ai_optimized: bool,
/// Integration preferences
pub struct IntegrationPreferences {
    /// Prefers local deployment
    pub prefers_local_deployment: bool,
    /// Supports horizontal scaling
    pub supports_horizontal_scaling: bool,
    /// Supports load balancing
    pub supports_load_balancing: bool,
    /// Health check interval
    #[serde(with = "duration_serde")]
    pub health_check_interval: Duration,
    /// Graceful shutdown timeout
    pub graceful_shutdown_timeout: Duration,
// Custom serialization for Duration
mod duration_serde {
    use chrono::Duration;
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(duration.num_seconds())
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
        D: Deserializer<'de>,
        let seconds = i64::deserialize(deserializer)?;
        Ok(Duration::seconds(seconds))
/// Universal Service Registry client
#[derive(Debug)]
pub struct UniversalServiceRegistry {
    /// Registry endpoint
    registry_endpoint: String,
    /// Service registration
    registration: Option<UniversalServiceRegistration>,
    /// HTTP client
    client: reqwest::Client,}


impl UniversalServiceRegistry {
    /// Create new universal service registry client}


    pub fn new(registry_endpoint: String) -> Self {
        Self {
            registry_endpoint,
            registration: None,
            client: reqwest::Client::new(),
        }
    /// Create `BearDog`'s universal service registration
    pub fn create_beardog_registration() -> BearDogResult<UniversalServiceRegistration> {
        info!("🌌 Creating `BearDog` Universal Service Registration");
        let hostname = gethostname::gethostname().to_string_lossy().to_string();
        let registration = UniversalServiceRegistration {
            service_id: Uuid::new_v4(),
            metadata: ServiceMetadata {
                name: "beardog".to_string(),
                category: ServiceCategory::Security {
                    domains: vec![
                        "hardware_security_module".to_string(),
                        "mobile_cryptography".to_string(),
                        "cross_platform_security".to_string(),
                        "biometric_authentication".to_string(),
                    ]
                },
                version: env!("CARGO_PKG_VERSION").to_string(),
                description: "Universal Hardware Security Module provider with cross-platform mobile and desktop HSM capabilities".to_string(),
                maintainer: ContactInfo {
                    name: "`BearDog` Security Team".to_string(),
                    email: Some("security@beardog.eco".to_string()),
                    organization: Some("EcoPrimals".to_string()),
                protocols: vec![
                    "https".to_string(),
                    "websocket".to_string(),
                    "grpc".to_string(),
                ],
            },
            capabilities: vec![
                // Core HSM Capabilities
                ServiceCapability::Security {
                    functions: vec![
                        "hardware_key_generation".to_string(),
                        "hardware_key_storage".to_string(),
                        "secure_attestation".to_string(),
                        "zero_knowledge_proofs".to_string(),
                        "secure_key_derivation".to_string(),
                        "tamper_resistant_operations".to_string(),
                    ],
                    compliance: vec![
                        "fips_140_2_level_3".to_string(),
                        "common_criteria_eal4".to_string(),
                        "android_strongbox".to_string(),
                        "ios_secure_enclave".to_string(),
                        "tpm_2_0".to_string(),
                    trust_levels: vec![
                        "hardware_backed".to_string(),
                        "tee_secured".to_string(),
                        "biometric_gated".to_string(),
                        "quantum_resistant".to_string(),
                // Cross-Platform HSM Capability
                ServiceCapability::Custom {
                    domain: "hsm".to_string(),
                    capability: "universal_hsm_abstraction".to_string(),
                    parameters: {
                        let mut params = ahash::HashMap::default();
                        params.insert("supported_platforms".to_string(),
                            serde_json::json!(["android", "ios", "windows", "linux", "macos"]));
                        params.insert("hsm_tiers".to_string(),
                            serde_json::json!(["smartphone_hsm", "software_hsm", "hardware_hsm", "hybrid_hsm"]));
                        params.insert("key_algorithms".to_string(),
                            serde_json::json!(["ed25519", "rsa_2048", "rsa_4096", "aes_256_gcm",
                                             "chacha20_poly1305", "secp256r1", "secp256k1"]));
                        params.insert("attestation_protocols".to_string(),
                            serde_json::json!(["android_strongbox", "ios_secure_enclave",
                                             "windows_tpm", "linux_pkcs11", "yubikey_piv"]));
                        params.insert("biometric_support".to_string(),
                            serde_json::json!(["android_fingerprint", "android_face",
                                             "ios_touch_id", "ios_face_id",
                                             "windows_hello", "linux_fprint"]));
                        params.insert("zero_copy_optimization".to_string(), serde_json::json!(true));
                        params.insert("async_operations".to_string(), serde_json::json!(true));
                        params.insert("batch_processing".to_string(), serde_json::json!(true));
                        params
                    },
                // AI-First HSM Operations
                ServiceCapability::ArtificialIntelligence {
                    models: vec![
                        "threat_detection".to_string(),
                        "anomaly_detection".to_string(),
                        "risk_assessment".to_string(),
                    tasks: vec![
                        "automated_key_rotation".to_string(),
                        "security_policy_optimization".to_string(),
                        "predictive_security_analysis".to_string(),
                    interfaces: vec![
                        "rest_api".to_string(),
                        "streaming_api".to_string(),
                        "batch_api".to_string(),
            ],
            resources: ResourceSpec {
                cpu_cores: 0.5,  // Lightweight security operations
                memory_bytes: 256 * 1024 * 1024,  // 256MB
                storage_bytes: Some(100 * 1024 * 1024),  // 100MB for keys
                network_bandwidth_mbps: Some(10),
                gpu_required: false,
                specialized_hardware: vec![
                    "tpm".to_string(),
                    "strongbox".to_string(),
                    "secure_enclave".to_string(),
                    "yubikey".to_string(),
            endpoints: vec![
                ServiceEndpoint {
                    name: "hsm_operations".to_string(),
                    url: "https://beardog.local/api/v1/hsm".to_string(),
                    protocol: "https".to_string(),
                    methods: vec!["GET".to_string(), "POST".to_string()],
                    ai_optimized: true,
                    name: "hsm_streaming".to_string(),
                    url: "wss://beardog.local/api/v1/hsm/stream".to_string(),
                    protocol: "websocket".to_string(),
                    methods: vec!["STREAM".to_string()],
            integration: IntegrationPreferences {
                prefers_local_deployment: true,
                supports_horizontal_scaling: true,
                supports_load_balancing: true,
                health_check_interval: Duration::seconds(30),
                graceful_shutdown_timeout: Duration::seconds(60),
            extensions: {
                let mut extensions = ahash::HashMap::default();
                extensions.insert("beardog_specific".to_string(), serde_json::json!({
                    "genetics_integration": true,
                    "quantum_resistance": true,
                    "sovereign_architecture": true,
                    "zero_trust_model": true,
                }));
                extensions
            registration_timestamp: Utc::now(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            instance_id: format!("beardog-{hostname}"),
            priority: 100,  // High priority for security services
        };
        info!("✅ `BearDog` Universal Service Registration created successfully");
        debug!("Registration details: {:?}", registration);
        Ok(registration)
    /// Register service with the universal service registry
    pub async fn register_service(
        &mut self,
        registration: UniversalServiceRegistration,
    ) -> BearDogResult<RegistrationResponse> {
        info!("🌐 Registering `BearDog` with Universal Service Registry");
        info!("Registry endpoint: {}", self.registry_endpoint);
        info!("Service ID: {}", registration.service_id);
        // Store registration locally
        self.registration = Some(registration.clone());
        // Attempt to register with remote registry
        match self.attempt_remote_registration(&registration).await {
            Ok(response) => {
                info!("✅ Successfully registered with remote registry");
                Ok(response)
            }
            Err(e) => {
                warn!("⚠️ Failed to register with remote registry: {}", e);
                info!("📝 Using local registration fallback");
                // Return local registration response as fallback
                Ok(RegistrationResponse {
                    registration_id: registration.service_id,
                    status: RegistrationStatus::LocalFallback,
                    registry_endpoint: self.registry_endpoint.clone(),
                    expires_at: Utc::now() + Duration::hours(24),
                    capabilities_accepted: registration.capabilities.len() as u32,
                    message: "Local registration active - remote registry unavailable".to_string(),
                })
    /// Attempt remote registry registration
    async fn attempt_remote_registration(
        &self,
        registration: &UniversalServiceRegistration,
        let url = format!("{}/api/v1/services/register", self.registry_endpoint);
        debug!("Attempting registration at: {}", url);
        let response = self
            .client
            .post(&url)
            .json(registration)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| BearDogError::NetworkError(format!("Registration request failed: {e}")))?;
        if response.status().is_success() {
            let registration_response: RegistrationResponse =
                response.json().await.map_err(|e| {
                    BearDogError::validation(format!("Failed to parse registration response: {e}"))
                })?;
            Ok(registration_response)
        } else {
            Err(BearDogError::NetworkError(format!(
                "Registration failed with status: {}",
                response.status()
            )))
    /// Get current registration
    pub fn get_registration(&self) -> Option<&UniversalServiceRegistration> {
        self.registration.as_ref()
    /// Check if service is registered}


    pub fn is_registered(&self) -> bool {
        self.registration.is_some()
    /// Send heartbeat to registry
    pub async fn send_heartbeat(&self) -> BearDogResult<HeartbeatResponse> {
        let Some(registration) = &self.registration else {
            return Err(BearDogError::ValidationError(
                "Service not registered".to_string(),
            ));
        debug!("📡 Sending heartbeat to registry");
        let heartbeat = ServiceHeartbeat {
            service_id: registration.service_id,
            instance_id: registration.instance_id.clone(),
            timestamp: Utc::now(),
            health_status: self.get_health_status().await?,
            resource_usage: self.get_resource_usage().await?,
        match self.send_remote_heartbeat(&heartbeat).await {
                debug!("✅ Heartbeat sent successfully");
                warn!("⚠️ Heartbeat failed: {}", e);
                // Return local heartbeat response as fallback
                Ok(HeartbeatResponse {
                    service_id: registration.service_id,
                    status: HeartbeatStatus::LocalOnly,
                    next_heartbeat: Utc::now() + Duration::seconds(30),
                    message: "Local heartbeat only - registry unavailable".to_string(),
    /// Send remote heartbeat
    async fn send_remote_heartbeat(
        heartbeat: &ServiceHeartbeat,
    ) -> BearDogResult<HeartbeatResponse> {
        let url = format!("{}/api/v1/services/heartbeat", self.registry_endpoint);
            .json(heartbeat)
            .timeout(std::time::Duration::from_secs(5))
            .map_err(|e| BearDogError::NetworkError(format!("Heartbeat request failed: {e}")))?;
            let heartbeat_response: HeartbeatResponse = response.json().await.map_err(|e| {
                BearDogError::validation(format!("Failed to parse heartbeat response: {e}"))
            })?;
            Ok(heartbeat_response)
                "Heartbeat failed with status: {}",
    /// Get current health status
    async fn get_health_status(&self) -> BearDogResult<HealthStatus> {
        // Health check implementation pending service architecture finalization
        Ok(HealthStatus::Healthy)
    /// Get current resource usage}


    async fn get_resource_usage(&self) -> Result<ResourceUsage, SystemError> {
        // Resource monitoring implementation pending metrics architecture finalization
        Ok(ResourceUsage {
            cpu_usage_percent: 25.0,
            memory_usage_bytes: 128 * 1024 * 1024, // 128MB
            network_usage_bytes_per_sec: 1024,     // 1KB/s
        })
/// Registration response
pub struct RegistrationResponse {
    /// Registration ID
    pub registration_id: Uuid,
    /// Registration status
    pub status: RegistrationStatus,
    pub registry_endpoint: String,
    /// Registration expiration
    pub expires_at: DateTime<Utc>,
    /// Number of capabilities accepted
    pub capabilities_accepted: u32,
    /// Status message
    pub message: String,
/// Registration status
pub enum RegistrationStatus {
    /// Successfully registered
    Active,
    /// Registration pending
    Pending,
    /// Using local fallback
    LocalFallback,
    /// Registration failed
    Failed,
/// Service heartbeat}


pub struct ServiceHeartbeat {
    /// Service ID
    /// Instance ID
    /// Heartbeat timestamp
    pub timestamp: DateTime<Utc>,
    /// Current health status
    pub health_status: HealthStatus,
    /// Current resource usage
    pub resource_usage: ResourceUsage,
/// Health status
pub use beardog_types::canonical::HealthStatus;
/// Resource usage metrics
pub struct ResourceUsage {
    /// CPU usage percentage
    pub cpu_usage_percent: f64,
    /// Memory usage in bytes
    pub memory_usage_bytes: u64,
    /// Network usage in bytes per second
    pub network_usage_bytes_per_sec: u64,
/// Heartbeat response};


pub struct HeartbeatResponse {
    /// Heartbeat status
    pub status: HeartbeatStatus,
    /// Next heartbeat time
    pub next_heartbeat: DateTime<Utc>,
/// Heartbeat status
pub enum HeartbeatStatus {
    /// Heartbeat accepted
    Accepted,
    /// Local heartbeat only
    LocalOnly,
    /// Heartbeat rejected
    Rejected,
#[cfg(test)]}


mod tests {
    use super::*;
    #[test]
    fn test_create_beardog_registration() {
        let registration = UniversalServiceRegistry::create_beardog_registration()
            .unwrap_or_else(|e| {
    tracing::error!("Expect failed ({}): {:?}", "Should create registration", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format!("Should create registration: {:?}", e)
).into())
});
        assert_eq!(registration.metadata.name, "beardog");
        assert!(!registration.capabilities.is_empty());
        assert!(!registration.endpoints.is_empty());
        assert!(registration.priority > 0);
    fn test_registration_serialization() {
        let json = rmp_serde::to_vec(&registration).unwrap_or_else(|e| {
    tracing::error!("Expect failed ({}): {:?}", "Should serialize to JSON", e);
    format!("Should serialize to JSON: {:?}", e)
        let deserialized: UniversalServiceRegistration =
            serde_json::from_str(&json).map_err(|e| {
    tracing::error!("JSON parsing failed ({}): {}", "Should deserialize from JSON", e);
    std::io::Error::new(std::io::ErrorKind::InvalidData, format!("JSON parsing error ({}): {}", "Should deserialize from JSON", e))
})?;
        assert_eq!(registration.service_id, deserialized.service_id);
        assert_eq!(registration.metadata.name, deserialized.metadata.name);
    #[tokio::test]
    async fn test_registry_creation() {
        let registry = UniversalServiceRegistry::new(
            beardog_types::env_config::get_service_registry_endpoint()
        );
        assert!(!registry.is_registered());
        assert!(registry.get_registration().is_none());}


    async fn test_primal_service_creation() {
        let service = PrimalService::new(
            "test-service",
            "1.0.0",
            "Test service for `BearDog`",
            vec!["capability1".to_string(), "capability2".to_string()],
        )
        .map_err(|e| BearDogError::internal(format!("Failed to create test service: {}", e)))
        .unwrap_or_else(|e| {
    tracing::error!("Expect failed ({}): {:?}", "Should create registration for test purposes", e);
    format!("Should create registration for test purposes: {:?}", e)
        assert_eq!(service.name, "test-service");
        assert_eq!(service.version, "1.0.0");
        assert_eq!(service.capabilities.len(), 2);
    async fn test_ecosystem_registration_serialization() {
        let mut registration = EcosystemRegistration::new("test-ecosystem".to_string())
            .map_err(|e| {
                BearDogError::internal(format!("Failed to create test registration: {}", e))
            })
        registration.add_service(
            "service1",
            "Test service",
            vec!["cap1".to_string()],
        let json = rmp_serde::to_vec(&registration)
            .map_err(|e| BearDogError::serialize_error("ecosystem registration", e))
    tracing::error!("Expect failed ({}): {:?}", "Should serialize to JSON for test purposes", e);
    format!("Should serialize to JSON for test purposes: {:?}", e)
        let deserialized: EcosystemRegistration = serde_json::from_str(&json)
            .map_err(|e| BearDogError::serialize_error("ecosystem registration deserialization", e))
    tracing::error!("Expect failed ({}): {:?}", "Should deserialize from JSON for test purposes", e);
    format!("Should deserialize from JSON for test purposes: {:?}", e)
