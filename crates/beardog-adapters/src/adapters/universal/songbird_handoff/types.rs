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


/// Universal SongBird Handoff Types
///
/// **Universal data structures for ecosystem service discovery**
/// This module provides universal types for integrating any ecosystem component
/// with SongBird's discovery and orchestration platform. The types are designed
/// to be domain-agnostic and work with any PrimalProvider implementation.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
// CANONICAL IMPORT: use beardog_types::config::UnifiedMonitoringConfig;
// CANONICAL IMPORT: use beardog_types::config::UnifiedNetworkConfig;
// CANONICAL IMPORT: use beardog_types::config::UnifiedSecurityConfig;
/// Universal primal types for ecosystem standardization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrimalType {
    /// ToadStool compute orchestrator
    ToadStool,
    /// Songbird discovery service
    Songbird,
    /// BearDog security provider
    BearDog,
    /// NestGate data management
    NestGate,
    /// Squirrel storage service
    Squirrel,
    /// BiomeOS operating system
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
/// Universal ecosystem service registration (standardized across all primals)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemServiceRegistration {
    /// Unique service identifier: "primal-{type}-{instance}"
    pub service_id: String,
    /// Primal type from standardized enum
    pub primal_type: PrimalType,
    /// Associated biome identifier (if applicable)
    pub biome_id: Option<String>,
    /// Service capabilities (standardized format)
    pub capabilities: ServiceCapabilities,
    /// API endpoints (standardized format)
    pub endpoints: ServiceEndpoints,
    /// Resource requirements
    pub resource_requirements: ResourceSpec,
    /// Security configuration
    pub security_config: SecurityConfig,
    /// Health check configuration
    pub health_check: HealthCheckConfig,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
    /// Registration timestamp
    pub registered_at: DateTime<Utc>,
/// Universal service capabilities (works with any primal)
pub struct ServiceCapabilities {
    /// Core capabilities (required)
    pub core: Vec<String>,
    /// Extended capabilities (optional)
    pub extended: Vec<String>,
    /// Cross-primal integrations supported
    pub integrations: Vec<String>,
    /// Performance characteristics
    pub performance: PerformanceCapabilities,
/// Universal service endpoints (standardized across all primals)
pub struct ServiceEndpoints {
    /// Health check endpoint
    pub health: String,
    /// Metrics endpoint
    pub metrics: String,
    /// Admin/management endpoint
    pub admin: String,
    /// WebSocket endpoint (if supported)
    pub websocket: Option<String>,
    /// Primary API endpoint
    pub primary: String,
/// Universal resource specification
pub struct ResourceSpec {
    /// CPU requirements (cores)
    pub cpu_cores: Option<f64>,
    /// Memory requirements (MB)
    pub memory_mb: Option<u64>,
    /// Storage requirements (MB)
    pub storage_mb: Option<u64>,
    /// Network bandwidth (Mbps)
    pub network_mbps: Option<u64>,
    /// GPU requirements (if applicable)
    pub gpu_units: Option<u32>,
/// Universal security configuration
// MIGRATED: SecurityConfig -> use beardog_types::config::UnifiedSecurityConfig;,
    /// Operation timed out
    Timeout,
    /// Service is unavailable
    ServiceUnavailable,
    /// Unauthorized access
    Unauthorized,
    /// Access forbidden
    Forbidden,
/// Universal security context for all requests
pub struct SecurityContext {
    /// Authentication token
    pub auth_token: Option<String>,
    /// User/service identity
    pub identity: String,
    /// Permissions/capabilities
    pub permissions: Vec<String>,
    /// Session information
    pub session_id: Option<String>,
/// Universal SongBird handoff configuration
/// Configuration for integrating any ecosystem component with SongBird's
/// discovery and orchestration platform.
pub struct SongBirdHandoffConfig {
    /// SongBird orchestrator endpoint
    pub songbird_endpoint: String,
    /// API authentication key
    pub api_key: String,
    /// Registration timeout in seconds
    pub registration_timeout_seconds: u64,
    /// Heartbeat interval in seconds
    pub heartbeat_interval_seconds: u64,
    /// Maximum registration retry attempts
    pub max_registration_retries: u32,
    /// Enable automatic re-registration on failure
    pub enable_auto_reregistration: bool,
    /// Service discovery tags
    pub discovery_tags: Vec<String>,
    /// Load balancer algorithm preference
    pub load_balancer_algorithm: LoadBalancingAlgorithm,
    /// Enable circuit breaker for fault tolerance
    pub enable_circuit_breaker: bool,
    /// Health check interval in seconds
    pub health_check_interval_seconds: u64,}


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
/// Universal service registration status
/// Tracks the registration state of any ecosystem component with SongBird.
#[derive(Debug, Clone)]
pub struct RegistrationStatus {
    /// Unique registration identifier
    pub registration_id: String,
    /// Current registration state
    pub status: RegistrationState,
    /// Last successful registration timestamp
    pub last_registration: DateTime<Utc>,
    /// Last heartbeat timestamp
    pub last_heartbeat: DateTime<Utc>,
    /// Number of consecutive failures
    pub consecutive_failures: u32,
    /// Next retry attempt time
    pub next_retry: Option<DateTime<Utc>>,
/// Universal registration state enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegistrationState {
    /// Not yet registered with SongBird
    NotRegistered,
    /// Registration in progress
    Registering,
    /// Successfully registered and active
    Active,
    /// Registration failed
    Failed,
    /// Temporarily deregistered
    Deregistered,
    /// Circuit breaker opened due to failures
    CircuitBreakerOpen,
/// Universal load balancing algorithms}


pub enum LoadBalancingAlgorithm {
    /// Round-robin load balancing
    RoundRobin,
    /// Least connections load balancing
    LeastConnections,
    /// Weighted round-robin load balancing
    WeightedRoundRobin,
    /// Resource-based load balancing
    ResourceBased,
    /// Latency-based load balancing
    LatencyBased,
/// Universal service registration result
pub struct ServiceRegistrationResult {
    /// Registration success status
    pub success: bool,
    /// Assigned service identifier
    /// Registration details
    pub registration_details: EcosystemServiceRegistration,
    /// Error message if registration failed
    pub error_message: Option<String>,
/// Universal advertised service structure
pub struct AdvertisedService {
    /// Service registration information
    pub registration: EcosystemServiceRegistration,
    /// Current service health
    pub health: ServiceHealth,
    /// Load balancer configuration
    pub load_balancer_config: LoadBalancerConfig,
    /// Orchestration metadata
    pub orchestration: OrchestrationMetadata,
/// Universal service health status
pub struct ServiceHealth {
    /// Overall health status
    pub status: HealthStatus,
    /// Health check timestamp
    pub last_check: DateTime<Utc>,
    /// Performance metrics
    pub metrics: PerformanceMetrics,
    /// Error details if unhealthy
    pub error_details: Option<String>,
/// Universal health status enumeration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Service is healthy
    Healthy,
    /// Service is degraded but functional
    Degraded,
    /// Service is unhealthy
    Unhealthy,
    /// Health status is unknown
    Unknown,
/// Universal performance metrics}


pub struct PerformanceMetrics {
    /// CPU utilization percentage
    pub cpu_percent: f64,
    /// Memory utilization percentage
    pub memory_percent: f64,
    /// Request latency in milliseconds
    pub latency_ms: u64,
    /// Requests per second
    pub requests_per_second: f64,
    /// Error rate percentage
    pub error_rate_percent: f64,
/// Universal load balancer configuration
// MIGRATED: LoadBalancerConfig -> use beardog_types::config::UnifiedNetworkConfig;


pub struct AffinityRule {
    /// Affinity type
    pub affinity_type: AffinityType,
    /// Target selection
/// Universal affinity type
pub enum AffinityType {
    /// Node affinity
    NodeAffinity,
    /// Service affinity
    ServiceAffinity,
    /// Anti-affinity
    AntiAffinity,
/// Universal service endpoint}


pub struct ServiceEndpoint {
    /// Endpoint URL
    pub url: String,
    /// Endpoint type
    pub endpoint_type: EndpointType,
    /// Protocol used
    pub protocol: String,
    /// Port number
    pub port: u16,
/// Universal endpoint type
pub enum EndpointType {
    Primary,
    Health,
    Metrics,
    Admin,
    /// WebSocket endpoint
    WebSocket,
    /// Custom endpoint type
/// Universal health summary}


pub struct HealthSummary {
    pub overall_status: HealthStatus,
    /// Component health details
    pub components: HashMap<String, ComponentHealth>,
    /// System performance metrics
    pub performance: PerformanceMetrics,
/// Universal component health
pub struct ComponentHealth {
    /// Component name
    pub name: String,
    /// Health status
    /// Health details
    pub details: Option<String>,
    /// Last check timestamp
/// Universal health monitor configuration
// MIGRATED: HealthMonitorConfig -> use beardog_types::config::UnifiedMonitoringConfig;


impl Default for HealthMonitorConfig {
            check_interval_seconds: 30,
            timeout_seconds: 5,
            failure_threshold: 3,
            recovery_threshold: 2,
