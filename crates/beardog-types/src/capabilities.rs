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


/// `BearDog` Capability Types and Universal Adapter Interface
///
/// Defines what `BearDog` can do internally vs what it routes through the universal adapter

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// `BearDog`'s internal capabilities (what we provide to ecosystem)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BearDogCapability {
    // Security-specific capabilities `BearDog` provides
    CryptographicOperations,
    KeyManagement,
    HardwareSecurityModules,
    ThreatDetection,
    SecurityAudit,
    AccessControl,
    DataEncryption,
    IdentityVerification,
    // In-house ML capabilities for security
    SecurityMLAnalysis,
    ThreatPatternRecognition,
    BehavioralAnomalyDetection,
    CryptographicOptimization,
    RiskAssessment,
}
/// **UNIFIED CAPABILITY TYPE** - Consolidates all capability definitions
/// This replaces the fragmented capability enums found in:
/// - `beardog-core/src/universal_discovery.rs` (CapabilityType)
/// - `beardog-types/src/capabilities.rs` (ExternalCapabilityType)
/// All modules should use this canonical capability enumeration.
pub enum CapabilityType {
    // Computation capabilities
    ComputeOptimization,
    ComputeOrchestration,
    GeneticAlgorithms,
    PerformanceAcceleration,
    ParallelProcessing,
    QuantumComputing,
    TaskScheduling,
    ResourceAllocation,
    WorkflowExecution,
    // Security capabilities
    Encryption,
    Decryption,
    Authentication,
    Authorization,
    ComplianceAuditing,
    // Storage capabilities
    FileSystem,
    ObjectStorage,
    VolumeManagement,
    DataReplication,
    BackupServices,
    StorageServices,
    DataPersistence,
    BackupRecovery,
    FileManagement,
    // Network capabilities
    ServiceDiscovery,
    LoadBalancing,
    NetworkRouting,
    MessageRouting,
    RealTimeStreaming,
    ServiceMesh,
    RequestRouting,
    // AI capabilities
    ModelInference,
    AgentFramework,
    NaturalLanguageProcessing,
    MachineLearning,
    PatternRecognition,
    AIIntelligence,
    Intelligence,
    LargeLanguageModels,
    MachineLearningTraining,
    KnowledgeGraphs,
    SemanticAnalysis,
    // System capabilities
    ResourceManagement,
    ProcessOrchestration,
    ConfigManagement,
    HealthMonitoring,
    MetricsCollection,
    SystemIntegration,
    ProcessManagement,
    ResourceMonitoring,
    EnvironmentConfiguration,
    WorkloadScheduling,
    ContainerOrchestration,
    SystemServices,
    // Hardware and Infrastructure capabilities
    HardwareSecurityModule,
    Network,
    Storage,
    Monitoring,
    Compliance,
    // Utility and fallback capabilities
    Unknown,
    // Specialized capabilities with parameters
    Gaming {
        low_latency: bool,
        simd_acceleration: bool,
    },
    GeneticHealing {
        adaptive: bool,
        evolutionary: bool,
    // Custom capability extension point
    Custom(String),
/// External capability types for universal adapter routing
pub enum ExternalCapabilityType {
    /// AI intelligence capabilities (Squirrel)
    /// Compute orchestration capabilities (ToadStool)
    /// Service mesh capabilities (SongBird)
    /// Storage services (NestGate)
    /// Identity management (Active Directory, LDAP, Okta)
    Identity,
    /// Secrets management (HashiCorp Vault, AWS KMS)
    SecretsManagement,
    /// Monitoring services (Grafana, Prometheus)
    /// Container orchestration (Kubernetes)
/// `BearDog`'s AI architecture - hybrid approach
#[derive(Debug, Clone, Serialize, Deserialize)]}


pub struct `BearDog`AIArchitecture {
    /// In-house ML capabilities `BearDog` handles internally
    pub internal_ml: Vec<SecurityMLCapability>,
    /// AI capabilities routed to Squirrel via universal adapter
    pub external_ai_routing: Vec<ExternalAIRequest>,
    /// Capability boundaries - what stays internal vs external
    pub capability_boundaries: AICapabilityBoundaries,
/// Security-specific ML capabilities `BearDog` handles in-house
pub enum SecurityMLCapability {
    /// Real-time threat pattern recognition
    ThreatPatternML {}


        model_type: String,
        confidence_threshold: f64,
        update_frequency_hours: u32,
    /// Behavioral anomaly detection for access patterns
    BehavioralAnomalyML {
        baseline_period_days: u32,
        sensitivity_level: f64,
        learning_rate: f64,
    /// Cryptographic operation optimization
    CryptographicOptimizationML {
        supported_algorithms: Vec<String>,
        performance_targets: HashMap<String, f64>,
    /// Risk assessment for security decisions
    SecurityRiskAssessmentML {
        risk_factors: Vec<String>,
        decision_thresholds: HashMap<String, f64>,
/// AI requests that get routed to Squirrel via universal adapter
pub enum ExternalAIRequest {
    /// Natural language processing for security logs
    SecurityLogNLP {
        log_content: String,
        analysis_type: NLPAnalysisType,
        context: SecurityContext,
    /// Large-scale pattern analysis beyond `BearDog`'s scope
    LargeScalePatternAnalysis {
        data_scope: PatternAnalysisScope,
        analysis_depth: AnalysisDepth,
        cross_system_correlation: bool,
    /// Knowledge graph queries for threat intelligence
    ThreatIntelligenceQuery {
        query_type: ThreatIntelQueryType,
        indicators: Vec<String>,
        correlation_scope: CorrelationScope,
    /// Advanced ML training that requires Squirrel's resources
    MLTrainingRequest {
        training_data_scope: TrainingScope,
        resource_requirements: ResourceRequirements,
/// Defines what `BearDog` handles internally vs routes externally}


pub struct AICapabilityBoundaries {
    /// Security-focused ML stays internal
    pub internal_boundaries: Vec<String>,
    /// Large-scale AI intelligence routes to Squirrel
    pub external_boundaries: Vec<String>,
    /// Hybrid scenarios that combine both
    pub hybrid_workflows: Vec<HybridAIWorkflow>,
/// Workflow combining `BearDog` internal ML + Squirrel AI
pub struct HybridAIWorkflow {
    /// Name of the hybrid workflow
    pub workflow_name: String,
    /// Steps handled by `BearDog` internal ML
    pub internal_steps: Vec<SecurityMLStep>,
    /// Steps routed to Squirrel via adapter
    pub external_steps: Vec<ExternalAIStep>,
    /// How results are combined
    pub integration_strategy: IntegrationStrategy,
/// `BearDog` internal ML processing step
pub struct SecurityMLStep {
    pub step_name: String,
    pub capability: SecurityMLCapability,
    pub input_requirements: Vec<String>,
    pub output_format: String,
/// External AI step routed through universal adapter
pub struct ExternalAIStep {
    pub capability_type: CapabilityType,
    pub request_type: ExternalAIRequest,
    pub routing_priority: RoutingPriority,
/// How to combine `BearDog` ML + Squirrel AI results
pub enum IntegrationStrategy {
    /// `BearDog` processes first, then sends to Squirrel
    Sequential,
    /// Parallel processing, combine results
    Parallel,
    /// `BearDog` validates Squirrel results
    ValidationBased,
    /// Squirrel enhances `BearDog` analysis
    EnhancementBased,
/// Supporting types}


pub enum NLPAnalysisType {
    ThreatExtraction,
    SentimentAnalysis,
    EntityRecognition,
    PatternIdentification,
}


pub enum PatternAnalysisScope {
    SingleSystem,
    CrossSystem,
    EcosystemWide,
    GlobalThreatIntel,}


pub enum AnalysisDepth {
    Surface,
    Deep,
    Comprehensive,
    Research,
pub enum ThreatIntelQueryType {
    IOCLookup,
    ThreatActorProfile,
    AttackPatternAnalysis,
    VulnerabilityCorrelation,}


pub enum CorrelationScope {
    Local,
    Regional,
    Global,
    Historical,
pub enum TrainingScope {
    SecuritySpecific,
    CrossDomain,}


pub enum RoutingPriority {
    Critical,
    High,
    Normal,
    Low,
    Background,
pub struct ResourceRequirements {
    pub cpu_cores: Option<u32>,
    pub memory_gb: Option<u32>,
    pub gpu_required: bool,
    pub training_time_hours: Option<u32>,
pub use crate::canonical::SecurityContext;}


impl Default for `BearDog`AIArchitecture {}


    fn default() -> Self {
        Self {
            internal_ml: vec![
                SecurityMLCapability::ThreatPatternML {
                    model_type: "RandomForest".to_string(),
                    confidence_threshold: 0.85,
                    update_frequency_hours: 24,
                },
                SecurityMLCapability::BehavioralAnomalyML {
                    baseline_period_days: 30,
                    sensitivity_level: 0.7,
                    learning_rate: 0.01,
            ],
            external_ai_routing: vec![],
            capability_boundaries: AICapabilityBoundaries::default(),
        }
    }
impl Default for AICapabilityBoundaries {
            internal_boundaries: vec![
                "threat_detection".to_string(),
                "access_anomaly".to_string(),
                "crypto_optimization".to_string(),
                "risk_assessment".to_string(),
            external_boundaries: vec![
                "large_language_models".to_string(),
                "knowledge_graphs".to_string(),
                "cross_system_correlation".to_string(),
                "semantic_analysis".to_string(),
            hybrid_workflows: vec![],
// ============================================================================
// UNIVERSAL ADAPTER REQUEST TYPES
/// Identity management request for external providers}


pub struct IdentityRequest {
    pub provider_type: String,
    pub operation: String,
    pub payload: serde_json::Value,
    pub metadata: HashMap<String, String>,
/// Secrets management request for external providers  
pub struct SecretsRequest {
/// Monitoring request for external providers
pub struct MonitoringRequest {
/// Container orchestration request for external providers
pub struct ContainerRequest {
/// Compute orchestration request for external providers (ToadStool)
pub struct ComputeRequest {
/// Service mesh request for external providers (SongBird)
pub struct ServiceMeshRequest {
/// Storage request for external providers (NestGate)
pub struct StorageRequest {
/// Workflow input data for hybrid AI processing
pub struct WorkflowInputData {
    pub workflow_id: String,
    pub input_type: String,
    pub data: serde_json::Value,
    pub context: HashMap<String, String>,
/// Result from hybrid workflow execution
pub struct HybridWorkflowResult {
    pub success: bool,
    pub internal_results: Vec<serde_json::Value>,
    pub external_results: Vec<serde_json::Value>,}


impl CapabilityType {
    /// Convert capability type to string representation}


    pub fn as_capability_string(&self) -> String {
        match self {
            // Computation capabilities
            CapabilityType::ComputeOptimization => "compute_optimization".to_string(),
            CapabilityType::ComputeOrchestration => "compute_orchestration".to_string(),
            CapabilityType::GeneticAlgorithms => "genetic_algorithms".to_string(),
            CapabilityType::PerformanceAcceleration => "performance_acceleration".to_string(),
            CapabilityType::ParallelProcessing => "parallel_processing".to_string(),
            CapabilityType::QuantumComputing => "quantum_computing".to_string(),
            CapabilityType::TaskScheduling => "task_scheduling".to_string(),
            CapabilityType::ResourceAllocation => "resource_allocation".to_string(),
            CapabilityType::WorkflowExecution => "workflow_execution".to_string(),
            // Security capabilities
            CapabilityType::Encryption => "encryption".to_string(),
            CapabilityType::Decryption => "decryption".to_string(),
            CapabilityType::Authentication => "authentication".to_string(),
            CapabilityType::Authorization => "authorization".to_string(),
            CapabilityType::ThreatDetection => "threat_detection".to_string(),
            CapabilityType::ComplianceAuditing => "compliance_auditing".to_string(),
            CapabilityType::KeyManagement => "key_management".to_string(),
            // Storage capabilities
            CapabilityType::FileSystem => "file_system".to_string(),
            CapabilityType::ObjectStorage => "object_storage".to_string(),
            CapabilityType::VolumeManagement => "volume_management".to_string(),
            CapabilityType::DataReplication => "data_replication".to_string(),
            CapabilityType::BackupServices => "backup_services".to_string(),
            CapabilityType::StorageServices => "storage_services".to_string(),
            CapabilityType::DataPersistence => "data_persistence".to_string(),
            CapabilityType::BackupRecovery => "backup_recovery".to_string(),
            CapabilityType::FileManagement => "file_management".to_string(),
            // Network capabilities
            CapabilityType::ServiceDiscovery => "service_discovery".to_string(),
            CapabilityType::LoadBalancing => "load_balancing".to_string(),
            CapabilityType::NetworkRouting => "network_routing".to_string(),
            CapabilityType::MessageRouting => "message_routing".to_string(),
            CapabilityType::RealTimeStreaming => "real_time_streaming".to_string(),
            CapabilityType::ServiceMesh => "service_mesh".to_string(),
            CapabilityType::RequestRouting => "request_routing".to_string(),
            // AI capabilities
            CapabilityType::ModelInference => "model_inference".to_string(),
            CapabilityType::AgentFramework => "agent_framework".to_string(),
            CapabilityType::NaturalLanguageProcessing => "natural_language_processing".to_string(),
            CapabilityType::MachineLearning => "machine_learning".to_string(),
            CapabilityType::PatternRecognition => "pattern_recognition".to_string(),
            CapabilityType::AIIntelligence => "ai_intelligence".to_string(),
            CapabilityType::Intelligence => "intelligence".to_string(),
            CapabilityType::LargeLanguageModels => "large_language_models".to_string(),
            CapabilityType::MachineLearningTraining => "machine_learning_training".to_string(),
            CapabilityType::KnowledgeGraphs => "knowledge_graphs".to_string(),
            CapabilityType::SemanticAnalysis => "semantic_analysis".to_string(),
            // System capabilities
            CapabilityType::ResourceManagement => "resource_management".to_string(),
            CapabilityType::ProcessOrchestration => "process_orchestration".to_string(),
            CapabilityType::ConfigManagement => "config_management".to_string(),
            CapabilityType::HealthMonitoring => "health_monitoring".to_string(),
            CapabilityType::MetricsCollection => "metrics_collection".to_string(),
            CapabilityType::SystemIntegration => "system_integration".to_string(),
            CapabilityType::ProcessManagement => "process_management".to_string(),
            CapabilityType::ResourceMonitoring => "resource_monitoring".to_string(),
            CapabilityType::EnvironmentConfiguration => "environment_configuration".to_string(),
            CapabilityType::WorkloadScheduling => "workload_scheduling".to_string(),
            CapabilityType::ContainerOrchestration => "container_orchestration".to_string(),
            CapabilityType::SystemServices => "system_services".to_string(),
            // Hardware and Infrastructure capabilities
            CapabilityType::HardwareSecurityModule => "hardware_security_module".to_string(),
            CapabilityType::Network => "network".to_string(),
            CapabilityType::Storage => "storage".to_string(),
            CapabilityType::Monitoring => "monitoring".to_string(),
            CapabilityType::Compliance => "compliance".to_string(),
            // Utility and fallback capabilities
            CapabilityType::Unknown => "unknown".to_string(),
            // Specialized capabilities with parameters
            CapabilityType::Gaming {
                low_latency,
                simd_acceleration,
            } => {
                format!("gaming_low_latency_{low_latency}_simd_{simd_acceleration}")
            }
            CapabilityType::GeneticHealing {
                adaptive,
                evolutionary,
                format!("genetic_healing_adaptive_{adaptive}_evolutionary_{evolutionary}")
            // Custom capability extension point
            CapabilityType::Custom(name) => format!("custom_{name}"),
