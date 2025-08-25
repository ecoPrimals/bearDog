// MODERNIZED: Removed async_trait - now uses native async fn in trait

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


/// External Primal Service
///
/// **CANONICAL EXTERNAL PRIMAL SERVICE** - Complete Tarpc service interface for external primal communication
/// This module provides comprehensive service interfaces for BearDog to communicate with other
/// primals in the ecosystem (Songbird, ToadStool, NestGate, BiomeOS, Squirrel).

use super::core_types::*;
use beardog_errors::{BearDogError, BearDogResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use uuid::Uuid;
/// **CANONICAL EXTERNAL PRIMAL SERVICE TRAIT** - Main interface for external primal communication
/// 
/// This is BearDog's interface to communicate with Songbird, ToadStool, and other primals
/// using Tarpc for high-performance RPC communication.

pub trait ExternalPrimalService: Send + Sync {
    /// Register BearDog service with external primal
    async fn register_service(&self, req: RegistrationRequest) -> BearDogResult<RegistrationResult>;
    
    /// Request capabilities from external primal
    async fn request_capabilities(&self, req: CapabilityRequest) -> BearDogResult<CapabilityResponse>;
    /// Conduct distributed key ceremony with external primal
    async fn conduct_key_ceremony(&self, req: KeyCeremonyRequest) -> BearDogResult<KeyCeremonyResult>;
    /// Submit HSM metrics to external primal for monitoring
    async fn submit_metrics(&self, metrics: HsmMetricsSnapshot) -> BearDogResult<MetricsAckResult>;
    /// Perform health check with external primal
    async fn health_check(&self) -> BearDogResult<ServiceHealthStatus>;
    /// Request genetic spawning from external primal
    async fn request_genetic_spawning(&self, req: GeneticSpawningRequest) -> BearDogResult<GeneticSpawningResult>;
    /// Submit security audit event to external primal
    async fn submit_audit_event(&self, event: SecurityAuditEvent) -> BearDogResult<AuditAckResult>;
}
// ============================================================================
// REQUEST AND RESPONSE TYPES
/// **REGISTRATION REQUEST** - Service registration with external primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRequest {
    /// BearDog service identifier
    pub service_id: Uuid,
    /// Service name
    pub service_name: String,
    /// Service version
    pub service_version: String,
    /// BearDog capabilities offered to external primal
    pub capabilities: Vec<BearDogCapability>,
    /// Service endpoint information
    pub endpoint: ServiceEndpoint,
    /// Authentication credentials
    pub credentials: ServiceCredentials,
    /// Registration metadata
    pub metadata: HashMap<String, String>,
    /// Registration timestamp
    pub timestamp: SystemTime,
/// **REGISTRATION RESULT** - Response to service registration
pub struct RegistrationResult {
    /// Registration status
    pub status: RegistrationStatus,
    /// Assigned service token
    pub service_token: Option<String>,
    /// External primal capabilities offered to BearDog
    pub primal_capabilities: Vec<ExternalPrimalCapability>,
    /// Communication configuration
    pub communication_config: CommunicationConfig,
    /// Registration expiry time
    pub expires_at: SystemTime,
    /// Result message
    pub message: String,
/// **CAPABILITY REQUEST** - Request for specific capabilities from external primal
pub struct CapabilityRequest {
    /// Service token for authentication
    pub service_token: String,
    /// Requested capability types
    pub requested_capabilities: Vec<String>,
    /// Capability requirements
    pub requirements: CapabilityRequirements,
    /// Request priority
    pub priority: RequestPriority,
    /// Request timeout
    pub timeout: Duration,
/// **CAPABILITY RESPONSE** - Response with available capabilities
pub struct CapabilityResponse {
    /// Response status
    pub status: ResponseStatus,
    /// Available capabilities
    pub available_capabilities: Vec<ExternalPrimalCapability>,
    /// Capability access tokens
    pub access_tokens: HashMap<String, String>,
    /// Usage quotas and limits
    pub quotas: HashMap<String, u64>,
    /// Response message
/// **KEY CEREMONY REQUEST** - Request for distributed key ceremony participation
pub struct KeyCeremonyRequest {
    /// Ceremony identifier
    pub ceremony_id: Uuid,
    /// Ceremony type
    pub ceremony_type: KeyCeremonyType,
    /// BearDog's role in the ceremony
    pub beardog_role: CeremonyRole,
    /// Ceremony parameters
    pub parameters: HashMap<String, String>,
    /// Required security level
    pub security_level: SecurityLevel,
    /// Ceremony data
    pub ceremony_data: Vec<u8>,
/// **KEY CEREMONY RESULT** - Result of key ceremony participation
pub struct KeyCeremonyResult {
    /// Ceremony status
    pub status: CeremonyStatus,
    /// Ceremony result data
    pub result_data: Vec<u8>,
    /// Ceremony metadata
    /// External primal's signature
    pub primal_signature: Option<Vec<u8>>,
/// **HSM METRICS SNAPSHOT** - Metrics data for external monitoring
pub struct HsmMetricsSnapshot {
    /// Snapshot timestamp
    /// HSM performance metrics
    pub performance_metrics: HashMap<String, f64>,
    /// HSM operation counters
    pub operation_counters: HashMap<String, u64>,
    /// HSM health status
    pub health_status: String,
    /// HSM error counts
    pub error_counts: HashMap<String, u32>,
    /// Active connections count
    pub active_connections: u32,
/// **METRICS ACK RESULT** - Acknowledgment of metrics submission
pub struct MetricsAckResult {
    /// Acknowledgment status
    pub status: AckStatus,
    /// Processing timestamp
    pub processed_at: SystemTime,
    /// Feedback from external primal
    pub feedback: Option<String>,
    /// Next submission interval recommendation
    pub next_interval: Option<Duration>,
/// **SERVICE HEALTH STATUS** - Health status of external primal service
pub struct ServiceHealthStatus {
    /// Overall health status
    pub status: HealthStatus,
    /// Service uptime
    pub uptime: Duration,
    /// Service load metrics
    pub load_metrics: HashMap<String, f64>,
    /// Service capabilities status
    pub capability_status: HashMap<String, CapabilityHealth>,
    /// Last health check timestamp
    pub last_check: SystemTime,
/// **GENETIC SPAWNING REQUEST** - Request for genetic algorithm spawning
pub struct GeneticSpawningRequest {
    /// Spawning request identifier
    pub request_id: Uuid,
    /// Genetic parameters
    pub genetic_parameters: GeneticParameters,
    /// Environment configuration
    pub environment: HashMap<String, String>,
    /// Resource requirements
    pub resource_requirements: ResourceRequirements,
/// **GENETIC SPAWNING RESULT** - Result of genetic spawning request
pub struct GeneticSpawningResult {
    /// Spawning status
    pub status: SpawningStatus,
    /// Generated organism identifier
    pub organism_id: Option<Uuid>,
    /// Genetic data
    pub genetic_data: Vec<u8>,
    /// Spawning metrics
    pub spawning_metrics: HashMap<String, f64>,
/// **SECURITY AUDIT EVENT** - Security event for external audit
pub struct SecurityAuditEvent {
    /// Event identifier
    pub event_id: Uuid,
    /// Event type
    pub event_type: String,
    /// Event severity
    pub severity: AuditSeverity,
    /// Event timestamp
    /// Event source
    pub source: String,
    /// Event details
    pub details: HashMap<String, String>,
    /// Event data
    pub event_data: Vec<u8>,
/// **AUDIT ACK RESULT** - Acknowledgment of audit event submission
pub struct AuditAckResult {
    /// Event processing identifier
    pub processing_id: Uuid,
    /// Audit feedback
// SUPPORTING ENUMS AND TYPES
/// **REGISTRATION STATUS**
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RegistrationStatus {
    /// Registration successful
    Success,
    /// Registration failed
    Failed,
    /// Registration pending approval
    Pending,
    /// Registration rejected
    Rejected,
/// **RESPONSE STATUS**}


pub enum ResponseStatus {
    /// Request successful
    /// Request failed
    /// Request partially fulfilled
    Partial,
    /// Request rejected
/// **CEREMONY STATUS**
pub enum CeremonyStatus {
    /// Ceremony successful
    /// Ceremony failed
    /// Ceremony in progress
    InProgress,
    /// Ceremony cancelled
    Cancelled,
/// **ACK STATUS**}


pub enum AckStatus {
    /// Acknowledged and accepted
    Accepted,
    /// Acknowledged but rejected
    /// Processing in progress
    Processing,
/// **HEALTH STATUS**
pub enum HealthStatus {
    /// Service is healthy
    Healthy,
    /// Service is degraded
    Degraded,
    /// Service is unhealthy
    Unhealthy,
    /// Service status unknown
    Unknown,
/// **SPAWNING STATUS**}


pub enum SpawningStatus {
    /// Spawning successful
    /// Spawning failed
    /// Spawning in progress
    /// Spawning queued
    Queued,
/// **AUDIT SEVERITY**
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuditSeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
/// **REQUEST PRIORITY**}


pub enum RequestPriority {
    /// Low priority
    /// Normal priority
    Normal,
    /// High priority
    /// Critical priority
/// **CEREMONY ROLE**
pub enum CeremonyRole {
    /// Ceremony initiator
    Initiator,
    /// Ceremony participant
    Participant,
    /// Ceremony witness
    Witness,
    /// Ceremony validator
    Validator,
/// **KEY CEREMONY TYPE**}


pub enum KeyCeremonyType {
    /// Key generation ceremony
    KeyGeneration,
    /// Key rotation ceremony
    KeyRotation,
    /// Key recovery ceremony
    KeyRecovery,
    /// Key destruction ceremony
    KeyDestruction,
/// **SECURITY LEVEL**
pub enum SecurityLevel {
    /// Standard security
    Standard,
    /// High security
    /// Critical security
    /// Maximum security
    Maximum,
// COMPLEX SUPPORTING TYPES
/// **BEARDOG CAPABILITY** - Capability offered by BearDog}


pub struct BearDogCapability {
    /// Capability identifier
    pub capability_id: String,
    /// Capability name
    pub name: String,
    /// Capability version
    pub version: String,
    /// Capability description
    pub description: String,
    /// Capability endpoints
    pub endpoints: Vec<String>,
/// **EXTERNAL PRIMAL CAPABILITY** - Capability offered by external primal
pub struct ExternalPrimalCapability {
    /// Primal type (songbird, toadstool, etc.)
    pub primal_type: String,
    /// Capability access level
    pub access_level: AccessLevel,
    /// Usage cost (if any)
    pub cost: Option<f64>,
/// **ACCESS LEVEL**
pub enum AccessLevel {
    /// Public access
    Public,
    /// Restricted access
    Restricted,
    /// Private access
    Private,
    /// Premium access
    Premium,
/// **SERVICE ENDPOINT**}


pub struct ServiceEndpoint {
    /// Endpoint URL
    pub url: String,
    /// Endpoint protocol
    pub protocol: String,
    /// Endpoint port
    pub port: u16,
    /// TLS configuration
    pub tls_enabled: bool,
/// **SERVICE CREDENTIALS**
pub struct ServiceCredentials {
    /// Authentication type
    pub auth_type: String,
    /// Credentials data
    pub credentials: HashMap<String, String>,
    /// Credential expiry
    pub expires_at: Option<SystemTime>,
/// **COMMUNICATION CONFIG**
pub struct CommunicationConfig {
    /// Preferred protocols
    pub protocols: Vec<String>,
    /// Message format
    pub message_format: String,
    /// Compression enabled
    pub compression: bool,
    /// Heartbeat interval
    pub heartbeat_interval: Duration,
/// **CAPABILITY REQUIREMENTS**
pub struct CapabilityRequirements {
    /// Required performance level
    pub performance_level: String,
    /// Required availability
    pub availability: f64,
    /// Maximum latency
    pub max_latency: Duration,
    /// Security requirements
    pub security_requirements: Vec<String>,
/// **CAPABILITY HEALTH**
pub enum CapabilityHealth {
    /// Capability is healthy
    /// Capability is degraded
    /// Capability is unavailable
    Unavailable,
/// **GENETIC PARAMETERS**}


pub struct GeneticParameters {
    /// Population size
    pub population_size: u32,
    /// Mutation rate
    pub mutation_rate: f64,
    /// Crossover rate
    pub crossover_rate: f64,
    /// Number of generations
    pub generations: u32,
    /// Fitness threshold
    pub fitness_threshold: f64,
/// **RESOURCE REQUIREMENTS**
pub struct ResourceRequirements {
    /// CPU requirements
    pub cpu_cores: u32,
    /// Memory requirements in bytes
    pub memory_bytes: u64,
    /// Storage requirements in bytes
    pub storage_bytes: u64,
    /// Network bandwidth requirements
    pub bandwidth_bps: u64,
    /// Execution timeout
