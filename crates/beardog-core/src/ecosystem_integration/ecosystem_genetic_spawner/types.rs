// SPDX-License-Identifier: AGPL-3.0-only

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

/// Genetic trait that can be inherited and combined
///
/// Represents a specific trait that can be inherited and combined
/// in the ecosystem genetic optimization process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticTrait {
    /// Unique identifier for this trait
    pub trait_id: String,
    /// Human-readable name of the trait
    pub trait_name: String,
    /// Category classification of the trait
    pub category: TraitCategory,
    /// Strength of the trait (0.0 to 1.0)
    pub strength: f64,
    /// Dominance level when combining with other traits (0.0 to 1.0)
    pub dominance: f64,
    /// Capabilities required for this trait to function
    pub required_capabilities: Vec<EcosystemCapability>,
    /// Additional trait-specific configuration
    pub trait_config: HashMap<String, serde_json::Value>,
}

/// Hybrid node in the ecosystem genetic network
///
/// Represents a node that combines multiple genetic contributions
/// to create optimized ecosystem behavior through trait combination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemHybridNode {
    /// Unique identifier for this hybrid node
    pub node_id: String,
    /// Genetic contributions that make up this node
    pub genetic_contributions: Vec<EcosystemGeneticContribution>,
    /// Combined traits resulting from genetic contributions
    pub combined_traits: Vec<GeneticTrait>,
    /// Resource allocation strategy for this node
    pub resource_allocation: EcosystemResourceAllocation,
    /// Current health status of the node
    pub health_status: NodeHealthStatus,
    /// Performance metrics for this node
    pub performance_metrics: NodePerformanceMetrics,
    /// Timestamp when this node was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Timestamp of last heartbeat (if any)
    pub last_heartbeat: Option<chrono::DateTime<chrono::Utc>>,
    /// Security level classification for this node
    pub security_level: SecurityLevel,
    /// Currently active capabilities on this node
    pub active_capabilities: Vec<EcosystemCapability>,
}

/// Ecosystem genetic blueprint
///
/// Template for creating optimized ecosystem hybrid nodes with
/// predefined genetic contributions and resource allocations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemGeneticBlueprint {
    /// Unique identifier for this blueprint
    pub blueprint_id: String,
    /// Human-readable name for this blueprint
    pub name: String,
    /// Primary genetic contributions that define this blueprint
    pub primary_contributions: Vec<EcosystemGeneticContribution>,
    /// Expected performance characteristics
    pub expected_performance: NodePerformanceMetrics,
    /// Resource requirements for nodes created from this blueprint
    pub resource_requirements: EcosystemResourceAllocation,
    /// Required security level
    pub security_level: SecurityLevel,
    /// Heartbeat interval in seconds
    pub heartbeat_interval_seconds: u64,
    /// Compatibility score with existing ecosystem
    pub compatibility_score: f64,
    /// Expected number of services
    pub expected_services: usize,
}

/// Ecosystem resource allocation
///
/// Defines the complete resource allocation strategy across all
/// system domains including security, compute, networking, storage, and AI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemResourceAllocation {
    /// Security-related resource allocations
    pub security: SecurityResourceAllocation,
    /// Compute resource allocations
    pub compute: ComputeResourceAllocation,
    /// Networking resource allocations
    pub networking: NetworkingResourceAllocation,
    /// Storage resource allocations
    pub storage: StorageResourceAllocation,
    /// AI processing resource allocations
    pub ai: AiResourceAllocation,
}

/// Security resource allocation configuration
///
/// Defines security resource allocations including
/// HSM usage, cryptographic operations, and audit retention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityResourceAllocation {
    /// Number of HSM slots allocated
    pub hsm_slots: u32,
    /// Key storage capacity in megabytes
    pub key_storage_mb: u32,
    /// Cryptographic operations per second capacity
    pub crypto_ops_per_second: u32,
    /// Audit log retention period in days
    pub audit_retention_days: u32,
}

/// Compute resource allocation configuration
///
/// Defines CPU, memory, and compute capacity allocations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResourceAllocation {
    /// Number of CPU cores allocated
    pub cpu_cores: u32,
    /// Memory allocation in gigabytes
    pub memory_gb: u32,
    /// Compute units per second capacity
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
    pub connections_per_second: u32,
}

/// Storage resource allocation configuration
///
/// Defines storage capacity and I/O performance allocations
/// for data storage and retrieval operations within the ecosystem.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResourceAllocation {
    /// Storage capacity allocation in gigabytes
    pub capacity_gb: u32,
    /// Input/output operations per second capacity
    pub iops: u32,
}

/// AI resource allocation configuration
///
/// including GPU compute and model storage requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiResourceAllocation {
    /// Number of `gpu_compute_units`
    pub gpu_compute_units: u32,
    /// Number of `model_storage_gb`
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
    /// Performance requirements for ecosystem operations
    pub performance_requirements: EcosystemPerformanceRequirements,
    /// Whether `high_availability` is enabled
    pub high_availability: bool,
    /// Collection of geographic preferences
    pub geographic_preferences: Vec<String>,
}

/// Resource constraints for ecosystem operations
///
/// Defines maximum resource limits for CPU, memory, storage, and cost.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemResourceConstraints {
    /// Maximum CPU cores allowed
    pub max_cpu_cores: Option<u32>,
    /// Maximum memory in GB allowed
    pub max_memory_gb: Option<u32>,
    /// Maximum storage in GB allowed
    pub max_storage_gb: Option<u32>,
    /// Maximum cost per hour in dollars
    pub max_cost_per_hour: Option<f64>,
}

/// Security requirements for ecosystem operations
///
/// Specifies minimum security levels, compliance standards, encryption, and auditing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemSecurityRequirements {
    /// Minimum required security level
    pub min_security_level: SecurityLevel,
    /// Required compliance standards (GDPR, HIPAA, etc.)
    pub required_compliance: Vec<ComplianceStandard>,
    /// Encryption requirements for data protection
    pub encryption_requirements: EncryptionRequirements,
    /// Auditing and logging requirements
    pub audit_requirements: AuditRequirements,
}

/// Encryption requirements for data protection
///
/// Specifies encryption needs for data at rest, in transit, and key rotation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionRequirements {
    /// Whether to encrypt data at rest
    pub data_at_rest: bool,
    /// Whether to encrypt data in transit
    pub data_in_transit: bool,
    /// How often to rotate encryption keys (in days)
    pub key_rotation_days: u32,
}

/// Auditing and logging requirements
///
/// Defines audit level, log retention, and real-time alerting needs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRequirements {
    /// Required audit level (basic, detailed, comprehensive)
    pub audit_level: AuditLevel,
    /// How long to retain audit logs (in days)
    pub log_retention_days: u32,
    /// Whether to enable real-time alerting
    pub real_time_alerting: bool,
}

/// Performance requirements for ecosystem operations
///
/// Defines minimum throughput, maximum latency, and uptime requirements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemPerformanceRequirements {
    /// Minimum requests per second required
    pub min_throughput_rps: u32,
    /// Maximum acceptable latency in milliseconds
    pub max_latency_ms: u32,
    /// Minimum uptime percentage required (0.0-100.0)
    pub min_uptime_percentage: f64,
}

/// Represents an active primal spawning operation
///
/// Tracks the progress of spawning a new primal instance, including blueprints,
/// resources, and current execution status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemSpawningOperation {
    /// Unique identifier for this spawning operation
    pub operation_id: String,
    /// Requirements for the primal being spawned
    pub requirements: EcosystemSpawningRequirements,
    /// Current status of the spawning operation
    pub status: SpawningStatus,
    /// Current execution stage
    pub current_stage: SpawningStage,
    /// Progress percentage (0.0-100.0)
    pub progress_percentage: f64,
    /// Generated genetic blueprints for selection
    pub genetic_blueprints: Vec<EcosystemGeneticBlueprint>,
    /// Blueprint selected for spawning
    pub selected_blueprint: Option<EcosystemGeneticBlueprint>,
    /// Reserved resources for this operation
    pub resource_reservations: HashMap<String, serde_json::Value>,
    /// Errors encountered during spawning
    pub error_messages: Vec<String>,
    /// When the operation started
    pub started_at: chrono::DateTime<chrono::Utc>,
    /// When the operation completed (if finished)
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Statistics for ecosystem spawning operations
///
/// Tracks success rates, performance metrics, and hybrid node creation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct EcosystemSpawningStatistics {
    /// Total number of spawn operations attempted
    pub total_spawns: u64,
    /// Number of successful spawn operations
    pub successful_spawns: u64,
    /// Number of failed spawn operations
    pub failed_spawns: u64,
    /// Total number of hybrid nodes created
    pub total_hybrid_nodes: u64,
    /// Average time to complete spawning in seconds
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
    /// Security-related traits (encryption, authentication, etc.)
    Security,
    /// Computation and processing traits
    Compute,
    /// Data storage and persistence traits
    Storage,
    /// Networking and communication traits
    Network,
    /// AI and machine learning traits
    AI,
    /// Monitoring and observability traits
    Monitoring,
    /// Custom trait category with user-defined classification
    Custom(String),
}

///
/// Defines the specific capabilities that can be required by genetic traits
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EcosystemCapability {
    /// High-performance computing capability for intensive workloads
    HighPerformanceCompute,
    /// Secure key management and cryptographic operations
    SecureKeyManagement,
    /// Distributed storage across ecosystem nodes
    DistributedStorage,
    /// Network routing and optimization capabilities
    NetworkOptimization,
    /// AI inference and machine learning capabilities
    AIInference,
    /// Real-time monitoring and observability
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
    /// Node is operational but experiencing reduced performance or partial failures
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
    /// ISO 27001 (Information Security Management) compliance
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
    /// Basic audit logging of critical events only
    Basic,
    /// Enhanced audit logging with additional detail
    Enhanced,
    /// Comprehensive audit logging of all events and state changes
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_performance_metrics_default() {
        let m = NodePerformanceMetrics::default();
        assert_eq!(m.cpu_utilization, 0.0);
        assert_eq!(m.requests_per_second, 0.0);
    }

    #[test]
    fn test_ecosystem_spawning_statistics_default() {
        let s = EcosystemSpawningStatistics::default();
        assert_eq!(s.total_spawns, 0);
        assert_eq!(s.successful_spawns, 0);
        assert_eq!(s.failed_spawns, 0);
    }

    #[test]
    fn test_trait_category_serialization() {
        let categories = [
            TraitCategory::Security,
            TraitCategory::Compute,
            TraitCategory::Custom("custom".to_string()),
        ];
        for cat in categories {
            let json = serde_json::to_string(&cat).expect("serialize trait category");
            let _: TraitCategory = serde_json::from_str(&json).expect("deserialize trait category");
        }
    }

    #[test]
    fn test_ecosystem_capability_serialization() {
        let caps = [
            EcosystemCapability::SecureKeyManagement,
            EcosystemCapability::Custom("custom".to_string()),
        ];
        for cap in caps {
            let json = serde_json::to_string(&cap).expect("serialize ecosystem capability");
            let _: EcosystemCapability =
                serde_json::from_str(&json).expect("deserialize ecosystem capability");
        }
    }

    #[test]
    fn test_security_level_ordering() {
        assert!(SecurityLevel::CriticalInfrastructure > SecurityLevel::Development);
        assert!(SecurityLevel::Production >= SecurityLevel::Staging);
    }

    #[test]
    fn test_spawning_status_variants() {
        let _ = SpawningStatus::Initializing;
        let _ = SpawningStatus::Completed;
        let _ = SpawningStatus::Failed;
    }

    #[test]
    fn test_spawning_stage_variants() {
        let _ = SpawningStage::Initialization;
        let _ = SpawningStage::Finalization;
    }

    #[test]
    fn test_node_health_status_variants() {
        let _ = NodeHealthStatus::Healthy;
        let _ = NodeHealthStatus::Critical;
    }
}
