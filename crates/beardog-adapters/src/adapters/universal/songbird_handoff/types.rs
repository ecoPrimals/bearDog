

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrimalType {

    ToadStool,

    Songbird,

    BearDog,

    NestGate,

    Squirrel,

    BiomeOS,
}
impl PrimalType {}

    pub fn as_str(&self) -> &'static str {
        match self {
            PrimalType::ToadStool => "toadstool",
            PrimalType::Songbird => "songbird",
            PrimalType::BearDog => "beardog",
            PrimalType::NestGate => "nestgate",
            PrimalType::Squirrel => "squirrel",
            PrimalType::BiomeOS => "biomeos",
        }
    }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemServiceRegistration {

    pub service_id: String,

    pub primal_type: PrimalType,

    pub biome_id: Option<String>,

    pub capabilities: ServiceCapabilities,

    pub endpoints: ServiceEndpoints,

    pub resource_requirements: ResourceSpec,

    pub security_config: SecurityConfig,

    pub health_check: HealthCheckConfig,

    pub metadata: HashMap<String, String>,

    pub registered_at: DateTime<Utc>,

pub struct ServiceCapabilities {

    pub core: Vec<String>,

    pub extended: Vec<String>,

    pub integrations: Vec<String>,

    pub performance: PerformanceCapabilities,

pub struct ServiceEndpoints {

    pub health: String,

    pub metrics: String,

    pub admin: String,

    pub websocket: Option<String>,

    pub primary: String,

pub struct ResourceSpec {

    pub cpu_cores: Option<f64>,

    pub memory_mb: Option<u64>,

    pub storage_mb: Option<u64>,

    pub network_mbps: Option<u64>,

    pub gpu_units: Option<u32>,

    Timeout,

    ServiceUnavailable,

    Unauthorized,

    Forbidden,

pub struct SecurityContext {

    pub auth_token: Option<String>,

    pub identity: String,

    pub permissions: Vec<String>,

    pub session_id: Option<String>,

// UNIFIED: Use canonical SongBirdHandoffConfig
pub use beardog_types::canonical::configuration::HandoffConfig;

impl Default for SongBirdHandoffConfig {}

    fn default() -> Self {
        Self {
            songbird_endpoint: std::env::var("SONGBIRD_ENDPOINT")
                .unwrap_or_else(|_| "https://songbird.ecosystem.internal".to_string()),
            api_key: std::env::var("SONGBIRD_API_KEY")
                .unwrap_or_else(|_| "default-api-key".to_string()),
            registration_timeout_seconds: 30,
            heartbeat_interval_seconds: 60,
            max_registration_retries: 3,
            enable_auto_reregistration: true,
            discovery_tags: vec!["ecosystem".to_string(), "universal".to_string()],
            load_balancer_algorithm: LoadBalancingAlgorithm::RoundRobin,
            enable_circuit_breaker: true,
            health_check_interval_seconds: 30,

#[derive(Debug, Clone)]
pub struct RegistrationStatus {

    pub registration_id: String,

    pub status: RegistrationState,

    pub last_registration: DateTime<Utc>,

    pub last_heartbeat: DateTime<Utc>,

    pub consecutive_failures: u32,

    pub next_retry: Option<DateTime<Utc>>,

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationState {

    NotRegistered,

    Registering,

    Active,

    Failed,

    Deregistered,

    CircuitBreakerOpen,

pub enum LoadBalancingAlgorithm {

    RoundRobin,

    LeastConnections,

    WeightedRoundRobin,

    ResourceBased,

    LatencyBased,

pub struct ServiceRegistrationResult {

    pub success: bool,

    pub registration_details: EcosystemServiceRegistration,

    pub error_message: Option<String>,

pub struct AdvertisedService {

    pub registration: EcosystemServiceRegistration,

    pub health: ServiceHealth,

    pub load_balancer_config: LoadBalancerConfig,

    pub orchestration: OrchestrationMetadata,

pub struct ServiceHealth {

    pub status: HealthStatus,

    pub last_check: DateTime<Utc>,

    pub metrics: PerformanceMetrics,

    pub error_details: Option<String>,

// UNIFIED: Use canonical HealthStatus from beardog-types
pub use beardog_types::canonical::HealthStatus;
    Healthy,

    Degraded,

    Unhealthy,

    Unknown,

pub struct PerformanceMetrics {

    pub cpu_percent: f64,

    pub memory_percent: f64,

    pub latency_ms: u64,

    pub requests_per_second: f64,

    pub error_rate_percent: f64,

pub struct AffinityRule {

    pub affinity_type: AffinityType,

pub enum AffinityType {

    NodeAffinity,

    ServiceAffinity,

    AntiAffinity,

pub struct ServiceEndpoint {

    pub url: String,

    pub endpoint_type: EndpointType,

    pub protocol: String,

    pub port: u16,

pub enum EndpointType {
    Primary,
    Health,
    Metrics,
    Admin,

    WebSocket,

pub struct HealthSummary {
    pub overall_status: HealthStatus,

    pub components: HashMap<String, ComponentHealth>,

    pub performance: PerformanceMetrics,

pub struct ComponentHealth {

    pub name: String,

    pub details: Option<String>,

impl Default for HealthMonitorConfig {
            check_interval_seconds: 30,
            timeout_seconds: 5,
            failure_threshold: 3,
            recovery_threshold: 2,
