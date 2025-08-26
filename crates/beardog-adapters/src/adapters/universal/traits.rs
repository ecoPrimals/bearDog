

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use beardog_errors::BearDogResult;

#[allow(async_fn_in_trait)]
pub trait UniversalEcosystemAdapter: Send + Sync {

    async fn initialize(&self, config: &HashMap<&str, &str>) -> BearDogResult<()>;

    async fn process_request(&self, request: ServiceRequest) -> BearDogResult<ServiceResponse>;

    async fn health_check(&self) -> BearDogResult<HealthStatus>;

    async fn get_capabilities(&self) -> BearDogResult<Vec<Capability>>;

    async fn register(&self, registration: EcosystemRegistration) -> BearDogResult<String>;

    async fn shutdown(&self) -> BearDogResult<()>;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Capability {

    pub id: String,

    pub name: String,

    pub description: String,

    pub category: CapabilityCategory,

    pub attributes: HashMap<String, CapabilityAttribute>,

    pub qos: QualityOfService,

    pub resource_requirements: ResourceRequirements,
}
impl Capability {

    pub fn new(
        id: &str,
        name: &str,
        description: &str,
        category: CapabilityCategory,
    ) -> Self {
        Self {
            id,
            name,
            description,
            category,
            attributes: HashMap::with_capacity(16),
            qos: QualityOfService::default(),
            resource_requirements: ResourceRequirements::default(),
        }
    }

    pub fn with_attributes(
        attributes: HashMap<&str, CapabilityAttribute>,
            attributes,

    pub fn add_attribute(mut self, key: &str, value: CapabilityAttribute) -> Self {
        self.attributes.insert(key, value);
        self

    pub fn with_qos(mut self, qos: QualityOfService) -> Self {
        self.qos = qos;

    pub fn with_resource_requirements(mut self, requirements: ResourceRequirements) -> Self {
        self.resource_requirements = requirements;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityCategory {

    Compute,

    Storage,

    Security,

    AI,

    Communication,

    Monitoring,

    Integration,

    Custom(String),

pub struct CapabilityAttribute {

    pub value: String,

    pub data_type: AttributeDataType,

    pub required: bool,

    pub description: Option<String>,

pub enum AttributeDataType {

    String,

    Integer,

    Float,

    Boolean,

    Array,

    Object,

    Duration,

    Bytes,

pub struct QualityOfService {

    pub avg_response_time_ms: u64,

    pub availability_percent: f64,

    pub throughput: Option<ThroughputMetric>,

    pub scalability: ScalabilityInfo,

pub struct ThroughputMetric {

    pub value: u64,

    pub unit: String,

pub struct ScalabilityInfo {

    pub min_instances: u32,

    pub max_instances: u32,

    pub auto_scaling: bool,

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ResourceRequirements {

    pub cpu: Option<ResourceRequirement>,

    pub memory: Option<ResourceRequirement>,

    pub storage: Option<ResourceRequirement>,

    pub network: Option<ResourceRequirement>,

    pub custom: HashMap<String, ResourceRequirement>,

pub struct ResourceRequirement {

    pub min: u64,

    pub max: Option<u64>,

pub struct Dependency {

    pub version: String,

    pub category: DependencyCategory,

    pub config: Option<HashMap<String, serde_json::Value>>,

pub enum DependencyCategory {

    Database,

    MessageQueue,

    Cache,

    ExternalApi,

    FileSystem,

    Network,

    Primal,

pub struct ServiceEndpoints {

    pub primary: String,

    pub health: String,

    pub metrics: Option<String>,

    pub admin: Option<String>,

    pub events: Option<String>,

    pub custom: HashMap<String, String>,

pub enum HealthStatus {

    Healthy,

    Degraded {

        issues: Vec<String>,

        impact: HealthImpact,
    },

    Unhealthy {

        reason: String,

        recovery_time: Option<DateTime<Utc>>,

    Starting,

    Shutting,

    Warning,

    Critical,

pub enum HealthImpact {

    Low,

    Medium,

    High,

#[derive(Debug, Clone, Serialize, Deserialize)]}

pub struct ServiceRequest {

    pub request_type: String,

    pub source: String,

    pub target: String,

    pub payload: serde_json::Value,

    pub metadata: HashMap<String, String>,

    pub timestamp: DateTime<Utc>,

    pub priority: RequestPriority,

pub enum RequestPriority {

    Normal,

pub struct ServiceResponse {

    pub request_id: String,

    pub success: bool,

    pub error: Option<ServiceError>,}

impl ServiceResponse {

    pub fn success(request_id: &str, payload: serde_json::Value) -> Self {
            request_id,
            success: true,
            payload,
            metadata: HashMap::with_capacity(16),
            timestamp: chrono::Utc::now(),
            error: None,

    pub fn error(request_id: &str, code: &str, message: &str) -> Self {
            success: false,
            payload: serde_json::json!({}),
            error: Some(ServiceError {
                code,
                message,
                details: None,
                retryable: false,
            }),

    pub fn error_with_details(
        request_id: &str,
        code: &str,
        message: &str,
        details: HashMap<&str, serde_json::Value>,
        retryable: bool,
                details: Some(details),
                retryable,

pub struct ServiceError {

    pub code: String,

    pub message: String,

    pub details: Option<HashMap<String, serde_json::Value>>,

    pub retryable: bool,

pub struct EcosystemRegistration {

    pub registration_id: String,

    pub ecosystem_id: String,

    pub instance_id: String,

    pub registered_at: DateTime<Utc>,

    pub expires_at: Option<DateTime<Utc>>,

    pub status: RegistrationStatus,

    pub capabilities: Vec<Capability>,

    pub endpoints: ServiceEndpoints,

pub enum RegistrationStatus {

    Pending,

    Active,

    Inactive,

    Suspended,

    Revoked,

    Registered,

    Standalone,

pub use beardog_types::config::network::NetworkSecurityConfig as NetworkConfig;

pub use beardog_types::config::network::ConnectionPoolConfig;

pub use beardog_types::config::monitoring::MonitoringConfig;

pub use beardog_types::canonical::providers::ProviderMetadata;

impl Default for ServiceEndpoints {}

    fn default() -> Self {
        let host = std::env::var("BEARDOG_SERVICE_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("BEARDOG_SERVICE_PORT").unwrap_or_else(|_| "8080".to_string());
        
            primary: format_args!("http://{}:{}", host, port).to_string(),
            health: format_args!("http://{}:{}/health", host, port).to_string(),
            metrics: Some(format_args!("http://{}:{}/metrics", host, port).to_string()),
            admin: None,
            events: None,
            custom: HashMap::with_capacity(16),
impl Default for QualityOfService {
            avg_response_time_ms: 100,
            availability_percent: 99.9,
            throughput: None,
            scalability: ScalabilityInfo {
                min_instances: 1,
                max_instances: 10,
                auto_scaling: false,
            },

pub struct SecurityContext {

    pub context_id: String,

    pub user_id: String,

    pub device_id: String,

    pub created_at: String,

    pub security_clearance_level: u8,

    pub risk_score: f64,

    pub device_trust_level: f64,

    pub network_zone: String,

    pub allowed_operations: Vec<String>,

    pub denied_operations: Vec<String>,

    pub auth_token: Option<String>,

    pub session_id: Option<String>,

    pub client_ip: Option<String>,

    pub user_agent: Option<String>,}

impl Default for SecurityContext {
            context_id: String::with_capacity(64),
            user_id: String::with_capacity(64),
            device_id: String::with_capacity(64),
            created_at: String::with_capacity(64),
            security_clearance_level: 1,
            risk_score: 0.5,
            device_trust_level: 0.5,
            network_zone: "unknown".to_string(),
            allowed_operations: vec!["read".to_string()],
            denied_operations: vec![],
            auth_token: None,
            session_id: None,
            client_ip: None,
            user_agent: None,
