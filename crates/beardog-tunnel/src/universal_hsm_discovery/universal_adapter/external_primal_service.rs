

use super::core_types::*;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

pub trait ExternalPrimalService: Send + Sync {

    async fn register_service(&self, req: RegistrationRequest) -> Result<RegistrationResult, BearDogError>;

    async fn request_capabilities(&self, req: CapabilityRequest) -> Result<CapabilityResponse, BearDogError>;

    async fn conduct_key_ceremony(&self, req: KeyCeremonyRequest) -> Result<KeyCeremonyResult, BearDogError>;

    async fn submit_metrics(&self, metrics: HsmMetricsSnapshot) -> Result<MetricsAckResult, BearDogError>;

    async fn health_check(&self) -> Result<ServiceHealthStatus, BearDogError>;

    async fn request_genetic_spawning(&self, req: GeneticSpawningRequest) -> Result<GeneticSpawningResult, BearDogError>;

    async fn submit_audit_event(&self, event: SecurityAuditEvent) -> Result<AuditAckResult, BearDogError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRequest {

    pub service_id: Uuid,

    pub service_name: String,

    pub service_version: String,

    pub capabilities: Vec<BearDogCapability>,

    pub endpoint: ServiceEndpoint,

    pub credentials: ServiceCredentials,

    pub metadata: HashMap<String, String>,

    pub timestamp: SystemTime,

pub struct RegistrationResult {

    pub status: RegistrationStatus,

    pub service_token: Option<String>,

    pub primal_capabilities: Vec<ExternalPrimalCapability>,

    pub communication_config: CommunicationConfig,

    pub expires_at: SystemTime,

    pub message: String,

pub struct CapabilityRequest {

    pub service_token: String,

    pub requested_capabilities: Vec<String>,

    pub requirements: CapabilityRequirements,

    pub priority: RequestPriority,

    pub timeout: Duration,

pub struct CapabilityResponse {

    pub status: ResponseStatus,

    pub available_capabilities: Vec<ExternalPrimalCapability>,

    pub access_tokens: HashMap<String, String>,

    pub quotas: HashMap<String, u64>,

pub struct KeyCeremonyRequest {

    pub ceremony_id: Uuid,

    pub ceremony_type: KeyCeremonyType,

    pub beardog_role: CeremonyRole,

    pub parameters: HashMap<String, String>,

    pub security_level: SecurityLevel,

    pub ceremony_data: Vec<u8>,

pub struct KeyCeremonyResult {

    pub status: CeremonyStatus,

    pub result_data: Vec<u8>,

    pub primal_signature: Option<Vec<u8>>,

pub struct HsmMetricsSnapshot {

    pub performance_metrics: HashMap<String, f64>,

    pub operation_counters: HashMap<String, u64>,

    pub health_status: String,

    pub error_counts: HashMap<String, u32>,

    pub active_connections: u32,

pub struct MetricsAckResult {

    pub status: AckStatus,

    pub processed_at: SystemTime,

    pub feedback: Option<String>,

    pub next_interval: Option<Duration>,

pub struct ServiceHealthStatus {

    pub status: HealthStatus,

    pub uptime: Duration,

    pub load_metrics: HashMap<String, f64>,

    pub capability_status: HashMap<String, CapabilityHealth>,

    pub last_check: SystemTime,

pub struct GeneticSpawningRequest {

    pub request_id: Uuid,

    pub genetic_parameters: GeneticParameters,

    pub environment: HashMap<String, String>,

    pub resource_requirements: ResourceRequirements,

pub struct GeneticSpawningResult {

    pub status: SpawningStatus,

    pub organism_id: Option<Uuid>,

    pub genetic_data: Vec<u8>,

    pub spawning_metrics: HashMap<String, f64>,

pub struct SecurityAuditEvent {

    pub event_id: Uuid,

    pub event_type: String,

    pub severity: AuditSeverity,

    pub source: String,

    pub details: HashMap<String, String>,

    pub event_data: Vec<u8>,

pub struct AuditAckResult {

    pub processing_id: Uuid,

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegistrationStatus {

    Success,

    Failed,

    Pending,

    Rejected,

pub enum ResponseStatus {

    Partial,

pub enum CeremonyStatus {

    InProgress,

    Cancelled,

pub enum AckStatus {

    Accepted,

    Processing,

// UNIFIED: Use canonical HealthStatus from beardog-types
pub use beardog_types::canonical::HealthStatus;
    Healthy,

    Degraded,

    Unhealthy,

    Unknown,

pub enum SpawningStatus {

    Queued,

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuditSeverity {

    Low,

    Medium,

    High,

    Critical,

pub enum RequestPriority {

    Normal,

pub enum CeremonyRole {

    Initiator,

    Participant,

    Witness,

    Validator,

pub enum KeyCeremonyType {

    KeyGeneration,

    KeyRotation,

    KeyRecovery,

    KeyDestruction,

pub enum SecurityLevel {

    Standard,

    Maximum,

pub struct BearDogCapability {

    pub capability_id: String,

    pub name: String,

    pub version: String,

    pub description: String,

    pub endpoints: Vec<String>,

pub struct ExternalPrimalCapability {

    pub primal_type: String,

    pub access_level: AccessLevel,

    pub cost: Option<f64>,

pub enum AccessLevel {

    Public,

    Restricted,

    Private,

    Premium,

pub struct ServiceEndpoint {

    pub url: String,

    pub protocol: String,

    pub port: u16,

    pub tls_enabled: bool,

pub struct ServiceCredentials {

    pub auth_type: String,

    pub credentials: HashMap<String, String>,

    pub expires_at: Option<SystemTime>,

pub struct CommunicationConfig {

    pub protocols: Vec<String>,

    pub message_format: String,

    pub compression: bool,

    pub heartbeat_interval: Duration,

pub struct CapabilityRequirements {

    pub performance_level: String,

    pub availability: f64,

    pub max_latency: Duration,

    pub security_requirements: Vec<String>,

pub enum CapabilityHealth {

    Unavailable,

pub struct GeneticParameters {

    pub population_size: u32,

    pub mutation_rate: f64,

    pub crossover_rate: f64,

    pub generations: u32,

    pub fitness_threshold: f64,

pub struct ResourceRequirements {

    pub cpu_cores: u32,

    pub memory_bytes: u64,

    pub storage_bytes: u64,

    pub bandwidth_bps: u64,

