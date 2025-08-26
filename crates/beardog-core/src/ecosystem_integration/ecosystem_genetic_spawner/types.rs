

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EcosystemCapability {

    SecurityGenetics,

    ComputeGenetics,

    ServiceMeshGenetics,

    StorageGenetics,

    AiGenetics,

    MultiPrimalAuthentication,

    EcosystemThreatDetection,

    UniversalResourceOrchestration,

    CrossPrimalDataSync,

    EcosystemCompliance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemGeneticContribution {

    pub primal_id: String,

    pub primal_name: String,

    pub contributed_traits: Vec<GeneticTrait>,

    pub contribution_weight: f64,

    pub compatibility_score: f64,

    pub primal_metadata: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticTrait {

    pub trait_id: String,

    pub trait_name: String,

    pub category: TraitCategory,

    pub strength: f64,

    pub dominance: f64,

    pub required_capabilities: Vec<EcosystemCapability>,

    pub trait_config: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraitCategory {
    Security,
    Compute,
    Storage,
    Networking,
    AI,
    Monitoring,
    Compliance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemHybridNode {

    pub node_id: String,

    pub genetic_contributions: Vec<EcosystemGeneticContribution>,

    pub combined_traits: Vec<GeneticTrait>,

    pub resource_allocation: EcosystemResourceAllocation,

    pub health_status: NodeHealthStatus,

    pub performance_metrics: NodePerformanceMetrics,

    pub created_at: chrono::DateTime<chrono::Utc>,

    pub last_heartbeat: Option<chrono::DateTime<chrono::Utc>>,

    pub security_level: SecurityLevel,

    pub active_capabilities: Vec<EcosystemCapability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemGeneticBlueprint {

    pub blueprint_id: String,

    pub name: String,

    pub primary_contributions: Vec<EcosystemGeneticContribution>,

    pub expected_performance: NodePerformanceMetrics,

    pub resource_requirements: EcosystemResourceAllocation,

    pub security_level: SecurityLevel,

    pub estimated_spawn_time: u64,

    pub compatibility_score: f64,

    pub expected_services: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemResourceAllocation {

    pub security: SecurityResourceAllocation,

    pub compute: ComputeResourceAllocation,

    pub networking: NetworkingResourceAllocation,

    pub storage: StorageResourceAllocation,

    pub ai: AiResourceAllocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityResourceAllocation {

    pub hsm_slots: u32,

    pub key_storage_mb: u32,

    pub crypto_ops_per_second: u32,

    pub audit_retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResourceAllocation {

    pub cpu_cores: u32,

    pub memory_gb: u32,

    pub compute_units_per_second: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkingResourceAllocation {

    pub bandwidth_mbps: u32,

    pub connections_per_second: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResourceAllocation {

    pub capacity_gb: u32,

    pub iops: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiResourceAllocation {

    pub gpu_compute_units: u32,

    pub model_storage_gb: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeHealthStatus {
    Healthy,
    Degraded,
    Critical,
    Offline,
    Initializing,
    Terminating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePerformanceMetrics {

    pub cpu_utilization: f64,

    pub memory_utilization: f64,

    pub network_utilization: f64,

    pub storage_utilization: f64,

    pub avg_response_time_ms: f64,

    pub requests_per_second: f64,

    pub error_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemSpawningRequirements {

    pub required_capabilities: Vec<EcosystemCapability>,

    pub resource_constraints: EcosystemResourceConstraints,

    pub security_requirements: EcosystemSecurityRequirements,

    pub performance_requirements: EcosystemPerformanceRequirements,

    pub high_availability: bool,

    pub geographic_preferences: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemResourceConstraints {

    pub max_cpu_cores: Option<u32>,

    pub max_memory_gb: Option<u32>,

    pub max_storage_gb: Option<u32>,

    pub max_cost_per_hour: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemSecurityRequirements {

    pub min_security_level: SecurityLevel,

    pub required_compliance: Vec<ComplianceStandard>,

    pub encryption_requirements: EncryptionRequirements,

    pub audit_requirements: AuditRequirements,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SecurityLevel {
    Development,
    Testing,
    Production,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStandard {
    SOC2,
    HIPAA,
    PCI_DSS,
    GDPR,
    FedRAMP,
    ISO27001,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionRequirements {

    pub data_at_rest: bool,

    pub data_in_transit: bool,

    pub key_rotation_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRequirements {

    pub audit_level: AuditLevel,

    pub log_retention_days: u32,

    pub real_time_alerting: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditLevel {
    Basic,
    Standard,
    Comprehensive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemPerformanceRequirements {

    pub min_rps: u32,

    pub max_latency_ms: u32,

    pub min_uptime_percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemSpawningOperation {

    pub operation_id: String,

    pub requirements: EcosystemSpawningRequirements,

    pub status: SpawningStatus,

    pub current_stage: SpawningStage,

    pub progress_percentage: f64,

    pub genetic_blueprints: Vec<EcosystemGeneticBlueprint>,

    pub selected_blueprint: Option<EcosystemGeneticBlueprint>,

    pub resource_reservations: HashMap<String, serde_json::Value>,

    pub error_messages: Vec<String>,

    pub started_at: chrono::DateTime<chrono::Utc>,

    pub estimated_completion: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpawningStatus {
    Initializing,
    AnalyzingRequirements,
    GeneratingBlueprints,
    SelectingBlueprint,
    AllocatingResources,
    CreatingHybridNode,
    Finalizing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpawningStage {
    Initialization,
    RequirementAnalysis,
    BlueprintGeneration,
    BlueprintSelection,
    ResourceAllocation,
    NodeCreation,
    HealthValidation,
    Finalization,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EcosystemSpawningStatistics {

    pub total_spawn_attempts: u64,

    pub successful_spawns: u64,

    pub failed_spawns: u64,

    pub total_hybrid_nodes: u64,

    pub avg_spawn_time_seconds: f64,

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