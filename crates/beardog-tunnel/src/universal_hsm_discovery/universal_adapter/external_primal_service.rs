

// Module documentation
//
// This module provides functionality for the BearDog ecosystem.


use super::core_types::*;
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

pub trait ExternalPrimalService: Send + Sync {


    fn register_service(&self, req: RegistrationRequest) -> Result<RegistrationResult, BearDogError>;


    fn request_capabilities(&self, req: CapabilityRequest) -> Result<CapabilityResponse, BearDogError>;


    fn conduct_key_ceremony(&self, req: KeyCeremonyRequest) -> Result<KeyCeremonyResult, BearDogError>;


    fn submit_metrics(&self, metrics: HsmMetricsSnapshot) -> Result<MetricsAckResult, BearDogError>;


    fn health_check(&self) -> Result<ServiceHealthStatus, BearDogError>;


    fn request_genetic_spawning(&self, req: GeneticSpawningRequest) -> Result<GeneticSpawningResult, BearDogError>;


    fn submit_audit_event(&self, event: SecurityAuditEvent) -> Result<AuditAckResult, BearDogError>;
}

#[derive(Debug, Clone)]
    /// Name of the service
    pub service_name: String,

    /// The service version value
    pub service_version: String,

    /// Collection of capabilities
    pub capabilities: Vec<BearDogCapability>,

    /// The endpoint value
    pub endpoint: ServiceEndpoint,

    /// The credentials value
    pub credentials: ServiceCredentials,

    /// Mapping of metadata
    pub metadata: HashMap<String, String>,


    pub timestamp: SystemTime,

pub struct RegistrationResult {

    /// Current status of the component
    pub status: RegistrationStatus,

    /// Optional service token
    pub service_token: Option<String>,

    /// Collection of primal capabilities
    pub primal_capabilities: Vec<ExternalPrimalCapability>,


    pub communication_config: CommunicationConfig,

    /// The expires at value
    pub expires_at: SystemTime,

    /// The message value
    pub message: String,

pub struct CapabilityRequest {

    /// The service token value
    pub service_token: String,

    /// Collection of requested capabilities
    pub requested_capabilities: Vec<String>,

    /// The requirements value
    pub requirements: CapabilityRequirements,

    /// The priority value
    pub priority: RequestPriority,


    pub timeout: Duration,

pub struct CapabilityResponse {

    /// Current status of the component
    pub status: ResponseStatus,

    /// Collection of available capabilities
    pub available_capabilities: Vec<ExternalPrimalCapability>,

    /// Mapping of access tokens
    pub access_tokens: HashMap<String, String>,

    /// Mapping of quotas
    pub quotas: HashMap<String, u64>,

pub struct KeyCeremonyRequest {


    pub ceremony_id: Uuid,

    /// The ceremony type value
    pub ceremony_type: KeyCeremonyType,

    /// The beardog role value
    pub beardog_role: CeremonyRole,

    /// Mapping of parameters
    pub parameters: HashMap<String, String>,

    /// The security level value
    pub security_level: SecurityLevel,

    /// Collection of ceremony data
    pub ceremony_data: Vec<u8>,

pub struct KeyCeremonyResult {

    /// Current status of the component
    pub status: CeremonyStatus,

    /// Collection of result data
    pub result_data: Vec<u8>,

    /// Optional primal signature
    pub primal_signature: Option<Vec<u8>>,

pub struct HsmMetricsSnapshot {


    pub performance_metrics: HashMap<String, f64>,

    /// Number of operationers
    pub operation_counters: HashMap<String, u64>,

    /// Current status of the health
    pub health_status: String,

    /// Number of errors
    pub error_counts: HashMap<String, u32>,

    /// Number of active_connections
    pub active_connections: u32,

pub struct MetricsAckResult {

    /// Current status of the component
    pub status: AckStatus,

    /// The processed at value
    pub processed_at: SystemTime,

    /// Optional feedback
    pub feedback: Option<String>,

    /// Optional next interval
    pub next_interval: Option<Duration>,

pub struct ServiceHealthStatus {

    /// Current status of the component
    pub status: HealthStatus,


    pub uptime: Duration,

    /// Mapping of load metrics
    pub load_metrics: HashMap<String, f64>,

    /// Current status of the capability
    pub capability_status: HashMap<String, CapabilityHealth>,

    /// The last check value
    pub last_check: SystemTime,

pub struct GeneticSpawningRequest {


    pub request_id: Uuid,

    /// The genetic parameters value
    pub genetic_parameters: GeneticParameters,

    /// Mapping of environment
    pub environment: HashMap<String, String>,

    /// The resource requirements value
    pub resource_requirements: ResourceRequirements,

pub struct GeneticSpawningResult {

    /// Current status of the component
    pub status: SpawningStatus,


    pub organism_id: Option<Uuid>,

    /// Collection of genetic data
    pub genetic_data: Vec<u8>,

    /// Mapping of spawning metrics
    pub spawning_metrics: HashMap<String, f64>,

pub struct SecurityAuditEvent {


    pub event_id: Uuid,

    /// The event type value
    pub event_type: String,

    /// The severity value
    pub severity: AuditSeverity,

    /// The source value
    pub source: String,

    /// Mapping of details
    pub details: HashMap<String, String>,

    /// Collection of event data
    pub event_data: Vec<u8>,

pub struct AuditAckResult {


    pub processing_id: Uuid,

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegistrationStatus {


    /// Successful completion state
    Success,


    /// Error or failure state
    Failed,


    /// Operation in progress
    Pending,


    /// State indicating rejected
    Rejected,

pub enum ResponseStatus {


    /// Represents partial variant
    Partial,

pub enum CeremonyStatus {


    /// Operation in progress
    InProgress,


    /// State indicating cancelled
    Cancelled,

pub enum AckStatus {


    /// State indicating accepted
    Accepted,


    /// Currently processing
    Processing,

pub use beardog_types::canonical::HealthStatus;
    /// Represents healthy variant
    Healthy,


    /// State indicating degraded
    Degraded,


    /// Represents unhealthy variant
    Unhealthy,


    /// Unknown or undefined state
    Unknown,

pub enum SpawningStatus {


    /// State indicating queued
    Queued,

#[derive(Debug, Clone)]
    /// Name of the item
    pub name: String,

    /// The version value
    pub version: String,

    /// The description value
    pub description: String,

    /// Collection of endpoints
    pub endpoints: Vec<String>,

pub struct ExternalPrimalCapability {

    /// The primal type value
    pub primal_type: String,

    /// The access level value
    pub access_level: AccessLevel,

    /// Optional cost
    pub cost: Option<f64>,

pub enum AccessLevel {


    /// Represents public variant
    Public,


    /// State indicating restricted
    Restricted,


    /// Represents private variant
    Private,


    /// Represents premium variant
    Premium,

pub struct ServiceEndpoint {

    /// The url value
    pub url: String,

    /// The protocol value
    pub protocol: String,

    /// Number of port
    pub port: u16,

    /// Whether tls is enabled
    pub tls_enabled: bool,

pub struct ServiceCredentials {

    /// The auth type value
    pub auth_type: String,

    /// Mapping of credentials
    pub credentials: HashMap<String, String>,

    /// Optional expires at
    pub expires_at: Option<SystemTime>,

pub struct CommunicationConfig {

    /// Collection of protocols
    pub protocols: Vec<String>,


    pub message_format: String,

    /// Whether compression is enabled
    pub compression: bool,

    /// The heartbeat interval value
    pub heartbeat_interval: Duration,

pub struct CapabilityRequirements {


    pub performance_level: String,

    /// The availability value
    pub availability: f64,

    /// The max latency value
    pub max_latency: Duration,

    /// Collection of security requirements
    pub security_requirements: Vec<String>,

pub enum CapabilityHealth {


    /// Represents unavailable variant
    Unavailable,

pub struct GeneticParameters {

    /// Number of population_size
    pub population_size: u32,

    /// The mutation rate value
    pub mutation_rate: f64,

    /// The crossover rate value
    pub crossover_rate: f64,

    /// Number of generations
    pub generations: u32,

    /// The fitness threshold value
    pub fitness_threshold: f64,

pub struct ResourceRequirements {

    /// Number of cpu_cores
    pub cpu_cores: u32,

    /// Number of memory_bytes
    pub memory_bytes: u64,

    /// Number of storage_bytes
    pub storage_bytes: u64,


    pub bandwidth_bps: u64,

