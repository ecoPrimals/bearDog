// Module documentation
//
// This module provides functionality for the BearDog ecosystem.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Genetic contribution from a primal component to the ecosystem
///
/// Represents the genetic material and characteristics that a primal component
/// contributes to the ecosystem's evolutionary optimization process. This includes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemGeneticContribution {
    /// Unique identifier of the contributing primal component
    pub primal_id: String,
    /// Human-readable name of the contributing primal component
    /// Name of the primal
    pub primal_name: String,
    /// List of genetic traits contributed by this primal component
    /// Collection of contributed traits
    pub contributed_traits: Vec<GeneticTrait>,
    /// The contribution weight value
    pub contribution_weight: f64,
    /// Compatibility score with other ecosystem components (0.0 to 1.0)
    /// The compatibility score value
    pub compatibility_score: f64,
    /// Additional metadata about the primal component and its capabilities
    /// Mapping of primal metadata
    pub primal_metadata: HashMap<String, serde_json::Value>,
}

///
/// Represents a specific trait that can be inherited and combined
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticTrait {
    pub trait_id: String,
    /// Human-readable name of the trait
    /// Name of the trait
    pub trait_name: String,
    /// Category classification of the trait
    /// The category value
    pub category: TraitCategory,
    /// Strength of the trait (0.0 to 1.0)
    /// The strength value
    pub strength: f64,
    /// Dominance level when combining with other traits (0.0 to 1.0)
    /// The dominance value
    pub dominance: f64,
    /// Collection of required capabilities
    pub required_capabilities: Vec<EcosystemCapability>,
    pub trait_config: HashMap<String, serde_json::Value>,
}

/// Hybrid node in the ecosystem genetic network
///
/// Represents a node that combines multiple genetic contributions
/// to create optimized ecosystem behavior. Manages trait combination,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemHybridNode {
    pub node_id: String,
    /// List of genetic contributions that make up this node
    /// Collection of genetic contributions
    pub genetic_contributions: Vec<EcosystemGeneticContribution>,
    /// Combined traits resulting from genetic contributions
    /// Collection of combined traits
    pub combined_traits: Vec<GeneticTrait>,
    /// The resource allocation value
    pub resource_allocation: EcosystemResourceAllocation,
    /// Current status of the health
    pub health_status: NodeHealthStatus,
    pub performance_metrics: NodePerformanceMetrics,
    /// The created at value
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Optional last heartbeat
    pub last_heartbeat: Option<chrono::DateTime<chrono::Utc>>,
    /// The security level value
    pub security_level: SecurityLevel,
    /// Collection of active capabilities
    pub active_capabilities: Vec<EcosystemCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemGeneticBlueprint {
    pub blueprint_id: String,
    /// Name of the item
    pub name: String,
    /// Collection of primary contributions
    pub primary_contributions: Vec<EcosystemGeneticContribution>,
    pub expected_performance: NodePerformanceMetrics,
    /// The resource requirements value
    pub resource_requirements: EcosystemResourceAllocation,
    /// The security level value
    pub security_level: SecurityLevel,
    /// Number of heartbeat_interval_seconds
    pub heartbeat_interval_seconds: u64,
    /// The compatibility score value
    pub compatibility_score: f64,
    /// Number of expected_services
    pub expected_services: usize,
}

///
/// Defines the complete resource allocation strategy across all
/// system domains including security, compute, networking, storage, and AI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemResourceAllocation {
    /// Security-related resource allocations
    /// The security value
    pub security: SecurityResourceAllocation,
    /// Compute resource allocations
    /// The compute value
    pub compute: ComputeResourceAllocation,
    /// Networking resource allocations
    /// The networking value
    pub networking: NetworkingResourceAllocation,
    /// Storage resource allocations
    /// The storage value
    pub storage: StorageResourceAllocation,
    /// AI processing resource allocations
    /// The ai value
    pub ai: AiResourceAllocation,
}

/// Security resource allocation configuration
///
/// HSM usage, cryptographic operations, and audit retention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityResourceAllocation {
    /// Number of HSM slots allocated
    /// Number of hsm_slots
    pub hsm_slots: u32,
    /// Key storage capacity in megabytes
    /// Number of key_storage_mb
    pub key_storage_mb: u32,
    /// Cryptographic operations per second capacity
    /// Number of crypto_ops_per_second
    pub crypto_ops_per_second: u32,
    /// Audit log retention period in days
    /// Number of audit_retention_days
    pub audit_retention_days: u32,
}

/// Compute resource allocation configuration
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResourceAllocation {
    /// Number of CPU cores allocated
    /// Number of cpu_cores
    pub cpu_cores: u32,
    /// Memory allocation in gigabytes
    /// Number of memory_gb
    pub memory_gb: u32,
    /// Compute units per second capacity
    /// Number of compute_units_per_second
    pub compute_units_per_second: u64,
}

/// Networking resource allocation configuration
///
/// Defines network bandwidth and connection capacity allocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkingResourceAllocation {
    /// Bandwidth allocation in megabits per second
    pub bandwidth_mbps: u32,
    /// Maximum connections per second
    /// Number of connections_per_second
    pub connections_per_second: u32,
}

/// Storage resource allocation configuration
///
/// and retrieval operations within the ecosystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResourceAllocation {
    /// Storage capacity allocation in gigabytes
    /// Number of capacity_gb
    pub capacity_gb: u32,
    /// Input/output operations per second capacity
    /// Number of iops
    pub iops: u32,
}

/// AI resource allocation configuration
///
/// including GPU compute and model storage requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiResourceAllocation {
    /// Number of gpu_compute_units
    pub gpu_compute_units: u32,
    /// Number of model_storage_gb
    pub model_storage_gb: u32,
}

///
/// including resource utilization, response times, and error rates.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePerformanceMetrics {
    /// CPU utilization percentage (0.0 to 100.0)
    /// The cpu utilization value
    pub cpu_utilization: f64,
    /// Memory utilization percentage (0.0 to 100.0)
    /// The memory utilization value
    pub memory_utilization: f64,
    /// Network utilization percentage (0.0 to 100.0)
    /// The network utilization value
    pub network_utilization: f64,
    /// Storage utilization percentage (0.0 to 100.0)
    /// The storage utilization value
    pub storage_utilization: f64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Request processing rate per second
    /// The requests per second value
    pub requests_per_second: f64,
    /// Error rate percentage (0.0 to 100.0)
    /// The error rate value
    pub error_rate: f64,
}

///
/// ecosystem components including capabilities, resources, security,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemSpawningRequirements {
    /// List of required ecosystem capabilities
    /// Collection of required capabilities
    pub required_capabilities: Vec<EcosystemCapability>,
    /// Resource allocation constraints and limits
    /// The resource constraints value
    pub resource_constraints: EcosystemResourceConstraints,
    /// Security requirements and policies
    /// The security requirements value
    pub security_requirements: EcosystemSecurityRequirements,
    pub performance_requirements: EcosystemPerformanceRequirements,
    /// Whether high_availability is enabled
    pub high_availability: bool,
    /// Collection of geographic preferences
    pub geographic_preferences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemResourceConstraints {
    /// Optional max cpu cores
    pub max_cpu_cores: Option<u32>,
    /// Optional max memory gb
    pub max_memory_gb: Option<u32>,
    /// Optional max storage gb
    pub max_storage_gb: Option<u32>,
    /// Optional max cost per hour
    pub max_cost_per_hour: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemSecurityRequirements {
    /// The min security level value
    pub min_security_level: SecurityLevel,
    /// Collection of required compliance
    pub required_compliance: Vec<ComplianceStandard>,
    /// The encryption requirements value
    pub encryption_requirements: EncryptionRequirements,
    /// The audit requirements value
    pub audit_requirements: AuditRequirements,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionRequirements {
    /// Whether data_at_rest is enabled
    pub data_at_rest: bool,
    /// Whether data_in_transit is enabled
    pub data_in_transit: bool,
    /// Number of key_rotation_days
    pub key_rotation_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRequirements {
    /// The audit level value
    pub audit_level: AuditLevel,
    /// Number of log_retention_days
    pub log_retention_days: u32,
    pub real_time_alerting: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemPerformanceRequirements {
    /// Number of min_throughput_rps
    pub min_throughput_rps: u32,
    /// Number of max_latency_ms
    pub max_latency_ms: u32,
    pub min_uptime_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemSpawningOperation {
    pub operation_id: String,
    /// The requirements value
    pub requirements: EcosystemSpawningRequirements,
    /// Current status of the component
    pub status: SpawningStatus,
    /// The current stage value
    pub current_stage: SpawningStage,
    /// The progress percentage value
    pub progress_percentage: f64,
    /// Collection of genetic blueprints
    pub genetic_blueprints: Vec<EcosystemGeneticBlueprint>,
    /// Optional selected blueprint
    pub selected_blueprint: Option<EcosystemGeneticBlueprint>,
    /// Mapping of resource reservations
    pub resource_reservations: HashMap<String, serde_json::Value>,
    /// Collection of error messages
    pub error_messages: Vec<String>,
    /// The started at value
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// Optional completed at
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EcosystemSpawningStatistics {
    /// Total number of spawn operations attempted
    /// Number of total_spawns
    pub total_spawns: u64,
    /// Number of successful spawn operations
    /// Number of successful_spawns
    pub successful_spawns: u64,
    /// Number of failed spawn operations  
    /// Number of failed_spawns
    pub failed_spawns: u64,
    /// Total number of hybrid nodes created
    pub total_hybrid_nodes: u64,
    pub avg_spawn_time_seconds: f64,
    /// Timestamp of last statistics update
    /// The last updated value
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl Default for NodePerformanceMetrics {
    fn default() -> Self {
        Self {
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            network_utilization: 0.0,
            storage_utilization: 0.0,
            avg_response_time_ms: 0.0,
            requests_per_second: 0.0,
            error_rate: 0.0,
        }
    }
}

impl Default for EcosystemSpawningStatistics {
    fn default() -> Self {
        Self {
            total_spawns: 0,
            successful_spawns: 0,
            failed_spawns: 0,
            total_hybrid_nodes: 0,
            avg_spawn_time_seconds: 0.0,
            last_updated: chrono::Utc::now(),
        }
    }
}

// Enums used by the structs above
///
/// Organizes genetic traits into functional categories to enable
/// targeted optimization and trait combination strategies within
/// the ecosystem genetic algorithm.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TraitCategory {
    Security,
    Compute,
    Storage,
    Network,
    AI,
    Monitoring,
    /// Custom trait category with user-defined classification
    Custom(String),
}

///
/// Defines the specific capabilities that can be required by genetic traits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EcosystemCapability {
    HighPerformanceCompute,
    SecureKeyManagement,
    DistributedStorage,
    NetworkOptimization,
    AIInference,
    RealTimeMonitoring,
    /// Custom ecosystem capability with user-defined functionality
    Custom(String),
}

/// Health status of ecosystem nodes
///
/// Represents the operational health state of nodes within the ecosystem,
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum NodeHealthStatus {
    /// Node is starting up and not yet ready
    Initializing,
    /// Node is operating normally with all systems functional
    Healthy,
    Degraded,
    /// Node has critical issues requiring immediate attention
    Critical,
    /// Node is offline and not responding
    Offline,
    /// Node is undergoing planned maintenance
    Maintenance,
}

///
/// Defines security levels with increasing strictness from development
/// to critical infrastructure, enabling appropriate security controls.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecurityLevel {
    /// Development environment with minimal security controls
    Development,
    /// Testing environment with basic security measures
    Testing,
    /// Staging environment with production-like security
    Staging,
    /// Production environment with full security controls
    Production,
    /// High-security environment with enhanced protection
    HighSecurity,
    /// Critical infrastructure with maximum security measures
    CriticalInfrastructure,
}

/// Compliance standards supported by the ecosystem
///
/// Defines industry-standard compliance frameworks that the ecosystem
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ComplianceStandard {
    /// SOC 2 (Service Organization Control 2) compliance
    SOC2,
    ISO27001,
    /// HIPAA (Health Insurance Portability and Accountability Act) compliance
    HIPAA,
    /// PCI DSS (Payment Card Industry Data Security Standard) compliance
    PciDss,
    /// GDPR (General Data Protection Regulation) compliance
    GDPR,
    /// Custom compliance standard with user-defined requirements
    Custom(String),
}

///
/// Defines the depth and frequency of audit logging from basic
/// event logging to real-time comprehensive monitoring.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuditLevel {
    Basic,
    /// Enhanced audit logging with additional detail
    Enhanced,
    Comprehensive,
    /// Real-time audit logging with immediate event processing
    RealTime,
}

/// Current status of a spawning operation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpawningStatus {
    /// Spawning operation is initializing
    Initializing,
    /// Spawning operation is currently in progress
    InProgress,
    /// Spawning operation completed successfully
    Completed,
    /// Spawning operation failed
    Failed,
    /// Spawning operation was cancelled
    Cancelled,
    /// Spawning operation is paused
    Paused,
}

/// Stage of the spawning process
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SpawningStage {
    /// Initial setup and preparation
    Initialization,
    /// Analyzing requirements and constraints
    RequirementAnalysis,
    /// Generating possible blueprints
    BlueprintGeneration,
    /// Selecting optimal blueprint
    BlueprintSelection,
    /// Allocating necessary resources
    ResourceAllocation,
    /// Creating the actual node
    NodeCreation,
    /// Validating node health and functionality
    HealthValidation,
    /// Final cleanup and completion
    Finalization,
}
