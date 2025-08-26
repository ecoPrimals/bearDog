

use crate::{BearDogError, BearDogResult};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalServiceRegistration {

    pub service_id: Uuid,

    pub metadata: ServiceMetadata,

    pub capabilities: Vec<ServiceCapability>,

    pub resources: ResourceSpec,

    pub endpoints: Vec<ServiceEndpoint>,

    pub integration: IntegrationPreferences,

    pub extensions: HashMap<String, serde_json::Value>,

    pub registration_timestamp: DateTime<Utc>,

    pub service_version: String,

    pub instance_id: String,

    pub priority: u32,
}

pub struct ServiceMetadata {

    pub name: String,

    pub category: ServiceCategory,
    pub version: String,

    pub description: String,

    pub maintainer: ContactInfo,

    pub protocols: Vec<String>,

pub enum ServiceCategory {

    Security {

        domains: Vec<String>,
    },

    Compute {

        capabilities: Vec<String>,

    Storage {

        types: Vec<String>,

    ArtificialIntelligence {

        models: Vec<String>,

    Custom {

        category: String,

        description: String,

pub struct ContactInfo {

    pub email: Option<String>,

    pub organization: Option<String>,

pub enum ServiceCapability {

        functions: Vec<String>,

        compliance: Vec<String>,

        trust_levels: Vec<String>,

        tasks: Vec<String>,

        interfaces: Vec<String>,

        domain: String,

        capability: String,

        parameters: HashMap<String, serde_json::Value>,

pub struct ResourceSpec {

    pub cpu_cores: f64,

    pub memory_bytes: u64,

    pub storage_bytes: Option<u64>,

    pub network_bandwidth_mbps: Option<u32>,

    pub gpu_required: bool,

    pub specialized_hardware: Vec<String>,

pub struct ServiceEndpoint {

    pub url: String,

    pub protocol: String,

    pub methods: Vec<String>,

    pub ai_optimized: bool,

pub struct IntegrationPreferences {

    pub prefers_local_deployment: bool,

    pub supports_horizontal_scaling: bool,

    pub supports_load_balancing: bool,

    #[serde(with = "duration_serde")]
    pub health_check_interval: Duration,

    pub graceful_shutdown_timeout: Duration,

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

#[derive(Debug)]
pub struct UniversalServiceRegistry {

    registry_endpoint: String,

    registration: Option<UniversalServiceRegistration>,

    client: reqwest::Client,}

impl UniversalServiceRegistry {

    pub fn new(registry_endpoint: &str) -> Self {
        Self {
            registry_endpoint,
            registration: None,
            client: reqwest::Client::new(),
        }

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

    pub async fn register_service(
        &mut self,
        registration: UniversalServiceRegistration,
    ) -> BearDogResult<RegistrationResponse> {
        info!("🌐 Registering `BearDog` with Universal Service Registry");
        info!("Registry endpoint: {}", self.registry_endpoint);
        info!("Service ID: {}", registration.service_id);

        self.registration = Some(registration.clone());

        match self.attempt_remote_registration(&registration).await {
            Ok(response) => {
                info!("✅ Successfully registered with remote registry");
                Ok(response)
            }
            Err(e) => {
                warn!("⚠️ Failed to register with remote registry: {}", e);
                info!("📝 Using local registration fallback");

                Ok(RegistrationResponse {
                    registration_id: registration.service_id,
                    status: RegistrationStatus::LocalFallback,
                    registry_endpoint: self.registry_endpoint.clone(),
                    expires_at: Utc::now() + Duration::hours(24),
                    capabilities_accepted: registration.capabilities.len() as u32,
                    message: "Local registration active - remote registry unavailable".to_string(),
                })

    async fn attempt_remote_registration(
        &self,
        registration: &UniversalServiceRegistration,
        let url = format_args!("{}/api/v1/services/register", self.registry_endpoint).to_string();
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

    pub fn get_registration(&self) -> Option<&UniversalServiceRegistration> {
        self.registration.as_ref()

    pub fn is_registered(&self) -> bool {
        self.registration.is_some()

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

                Ok(HeartbeatResponse {
                    service_id: registration.service_id,
                    status: HeartbeatStatus::LocalOnly,
                    next_heartbeat: Utc::now() + Duration::seconds(30),
                    message: "Local heartbeat only - registry unavailable".to_string(),

    async fn send_remote_heartbeat(
        heartbeat: &ServiceHeartbeat,
    ) -> BearDogResult<HeartbeatResponse> {
        let url = format_args!("{}/api/v1/services/heartbeat", self.registry_endpoint).to_string();
            .json(heartbeat)
            .timeout(std::time::Duration::from_secs(5))
            .map_err(|e| BearDogError::NetworkError(format!("Heartbeat request failed: {e}")))?;
            let heartbeat_response: HeartbeatResponse = response.json().await.map_err(|e| {
                BearDogError::validation(format!("Failed to parse heartbeat response: {e}"))
            })?;
            Ok(heartbeat_response)
                "Heartbeat failed with status: {}",

    async fn get_health_status(&self) -> BearDogResult<HealthStatus> {

        Ok(HealthStatus::Healthy)

    async fn get_resource_usage(&self) -> Result<ResourceUsage, SystemError> {

        Ok(ResourceUsage {
            cpu_usage_percent: 25.0,
            memory_usage_bytes: 128 * 1024 * 1024, // 128MB
            network_usage_bytes_per_sec: 1024,     // 1KB/s
        })

pub struct RegistrationResponse {

    pub registration_id: Uuid,

    pub status: RegistrationStatus,
    pub registry_endpoint: String,

    pub expires_at: DateTime<Utc>,

    pub capabilities_accepted: u32,

    pub message: String,

pub enum RegistrationStatus {

    Active,

    Pending,

    LocalFallback,

    Failed,

pub struct ServiceHeartbeat {

    pub timestamp: DateTime<Utc>,

    pub health_status: HealthStatus,

    pub resource_usage: ResourceUsage,

pub use beardog_types::canonical::HealthStatus;

pub struct ResourceUsage {

    pub cpu_usage_percent: f64,

    pub memory_usage_bytes: u64,

    pub network_usage_bytes_per_sec: u64,

pub struct HeartbeatResponse {

    pub status: HeartbeatStatus,

    pub next_heartbeat: DateTime<Utc>,

pub enum HeartbeatStatus {

    Accepted,

    LocalOnly,

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
    format_args!("Should create registration: {:?}", e).to_string()
).into())
});
        assert_eq!(registration.metadata.name, "beardog");
        assert!(!registration.capabilities.is_empty());
        assert!(!registration.endpoints.is_empty());
        assert!(registration.priority > 0);
    fn test_registration_serialization() {
        let json = rmp_serde::to_vec(&registration).unwrap_or_else(|e| {
    tracing::error!("Expect failed ({}): {:?}", "Should serialize to JSON", e);
    format_args!("Should serialize to JSON: {:?}", e).to_string()
        let deserialized: UniversalServiceRegistration =
            serde_json::from_str(&json).map_err(|e| {
    tracing::error!("JSON parsing failed ({}): {}", "Should deserialize from JSON", e);
    std::io::Error::new(std::io::ErrorKind::InvalidData, format_args!("JSON parsing error ({}): {}", "Should deserialize from JSON", e).to_string())
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
        .map_err(|e| BearDogError::internal(format_args!("Failed to create test service: {}", e).to_string()))
        .unwrap_or_else(|e| {
    tracing::error!("Expect failed ({}): {:?}", "Should create registration for test purposes", e);
    format_args!("Should create registration for test purposes: {:?}", e).to_string()
        assert_eq!(service.name, "test-service");
        assert_eq!(service.version, "1.0.0");
        assert_eq!(service.capabilities.len(), 2);
    async fn test_ecosystem_registration_serialization() {
        let mut registration = EcosystemRegistration::new("test-ecosystem".to_string())
            .map_err(|e| {
                BearDogError::internal(format_args!("Failed to create test registration: {}", e).to_string())
            })
        registration.add_service(
            "service1",
            "Test service",
            vec!["cap1".to_string()],
        let json = rmp_serde::to_vec(&registration)
            .map_err(|e| BearDogError::serialize_error("ecosystem registration", e))
    tracing::error!("Expect failed ({}): {:?}", "Should serialize to JSON for test purposes", e);
    format_args!("Should serialize to JSON for test purposes: {:?}", e).to_string()
        let deserialized: EcosystemRegistration = serde_json::from_str(&json)
            .map_err(|e| BearDogError::serialize_error("ecosystem registration deserialization", e))
    tracing::error!("Expect failed ({}): {:?}", "Should deserialize from JSON for test purposes", e);
    format_args!("Should deserialize from JSON for test purposes: {:?}", e).to_string()
