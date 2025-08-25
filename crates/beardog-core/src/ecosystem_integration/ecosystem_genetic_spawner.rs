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


/// Ecosystem Genetic Spawner - Cross-Primal Genetic Algorithms
///
/// This module implements BearDog's revolutionary cross-ecosystem genetic spawning
/// capabilities, enabling the creation of hybrid nodes that combine security genetics
/// from BearDog with compute genetics from ToadStool and other ecosystem primals.

use super::{UniversalHsmProvider, SongbirdServiceDiscovery};
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::crypto::{KeyType, KeyMetadata};
use beardog_types::canonical::genetics::GeneticsConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};
use uuid::Uuid;

/// Cross-ecosystem genetic spawning capabilities
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EcosystemCapability {
    /// BearDog security genetics
    SecurityGenetics,
    /// ToadStool compute genetics  
    ComputeGenetics,
    /// Songbird service mesh genetics
    ServiceMeshGenetics,
    /// NestGate storage genetics
    StorageGenetics,
    /// Squirrel AI genetics
    AiGenetics,
    /// Hybrid multi-primal capabilities
    MultiPrimalAuthentication,
    /// Cross-ecosystem threat detection
    EcosystemThreatDetection,
    /// Universal resource orchestration
    UniversalResourceOrchestration,
    /// Cross-primal data synchronization
    CrossPrimalDataSync,
    /// Ecosystem-wide compliance
    EcosystemCompliance,
}

/// Genetic contribution from an ecosystem primal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemGeneticContribution {
    /// Source primal (BearDog, ToadStool, etc.)
    pub primal_id: String,
    /// Primal name
    pub primal_name: String,
    /// Genetic traits contributed
    pub contributed_traits: Vec<GeneticTrait>,
    /// Contribution weight (0.0 to 1.0)
    pub contribution_weight: f64,
    /// Genetic compatibility score with other primals
    pub compatibility_score: f64,
    /// Primal-specific metadata
    pub primal_metadata: HashMap<String, serde_json::Value>,
}

/// Individual genetic trait from ecosystem components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticTrait {
    /// Trait identifier
    pub trait_id: String,
    /// Human-readable trait name
    pub trait_name: String,
    /// Trait category (security, compute, storage, etc.)
    pub category: TraitCategory,
    /// Trait strength (0.0 to 1.0)
    pub strength: f64,
    /// Trait dominance in genetic recombination
    pub dominance: f64,
    /// Required capabilities for this trait
    pub required_capabilities: Vec<EcosystemCapability>,
    /// Trait-specific configuration
    pub trait_config: HashMap<String, serde_json::Value>,
}

/// Genetic trait categories
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraitCategory {
    Security,
    Compute,
    Storage,
    Networking,
    AI,
    Monitoring,
    Compliance,
    ResourceManagement,
}

/// Hybrid node resulting from cross-ecosystem genetic spawning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemHybridNode {
    /// Unique node identifier
    pub node_id: String,
    /// Node name
    pub node_name: String,
    /// Genetic blueprint used to create this node
    pub genetic_blueprint: EcosystemGeneticBlueprint,
    /// Capabilities this node possesses
    pub capabilities: Vec<EcosystemCapability>,
    /// Resource allocation across primals
    pub resource_allocation: EcosystemResourceAllocation,
    /// Node health status
    pub health_status: NodeHealthStatus,
    /// Performance metrics
    pub performance_metrics: NodePerformanceMetrics,
    /// Creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last health check
    pub last_health_check: chrono::DateTime<chrono::Utc>,
}

/// Genetic blueprint for cross-ecosystem hybrid nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemGeneticBlueprint {
    /// Blueprint identifier
    pub blueprint_id: String,
    /// Parent genetic contributions
    pub parent_contributions: Vec<EcosystemGeneticContribution>,
    /// Resulting hybrid traits
    pub hybrid_traits: Vec<GeneticTrait>,
    /// Overall fitness score
    pub fitness_score: f64,
    /// Generation in evolutionary process
    pub generation: u32,
    /// Mutation applied
    pub mutation_applied: bool,
    /// Crossover points
    pub crossover_points: Vec<usize>,
    /// Genetic diversity score
    pub diversity_score: f64,
}

/// Resource allocation across ecosystem primals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemResourceAllocation {
    /// BearDog security resources
    pub beardog_security: SecurityResourceAllocation,
    /// ToadStool compute resources
    pub toadstool_compute: ComputeResourceAllocation,
    /// Songbird networking resources
    pub songbird_networking: NetworkingResourceAllocation,
    /// NestGate storage resources
    pub nestgate_storage: StorageResourceAllocation,
    /// Squirrel AI resources
    pub squirrel_ai: AiResourceAllocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityResourceAllocation {
    pub hsm_slots: u32,
    pub key_storage_mb: u64,
    pub crypto_operations_per_sec: u32,
    pub audit_log_retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputeResourceAllocation {
    pub cpu_cores: u32,
    pub memory_gb: u64,
    pub gpu_units: u32,
    pub storage_gb: u64,
    pub network_bandwidth_mbps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkingResourceAllocation {
    pub connection_pool_size: u32,
    pub load_balancer_instances: u32,
    pub service_mesh_endpoints: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageResourceAllocation {
    pub primary_storage_gb: u64,
    pub backup_storage_gb: u64,
    pub cache_storage_gb: u64,
    pub replication_factor: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiResourceAllocation {
    pub model_inference_slots: u32,
    pub training_compute_units: u32,
    pub ai_memory_gb: u64,
}

/// Node health status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum NodeHealthStatus {
    Optimal,
    Healthy,
    Degraded,
    Critical,
    Offline,
}

/// Node performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePerformanceMetrics {
    pub cpu_utilization: f64,
    pub memory_utilization: f64,
    pub network_throughput_mbps: f64,
    pub storage_iops: u64,
    pub response_time_ms: f64,
    pub error_rate: f64,
    pub uptime_percentage: f64,
}

/// Spawning requirements for ecosystem hybrid nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemSpawningRequirements {
    /// Target capabilities for the hybrid node
    pub target_capabilities: Vec<EcosystemCapability>,
    /// Minimum fitness score required
    pub min_fitness_score: f64,
    /// Resource constraints
    pub resource_constraints: EcosystemResourceConstraints,
    /// Security requirements
    pub security_requirements: EcosystemSecurityRequirements,
    /// Performance requirements
    pub performance_requirements: EcosystemPerformanceRequirements,
    /// Spawning timeout
    pub spawning_timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemResourceConstraints {
    pub max_total_cost: f64,
    pub max_beardog_resources: SecurityResourceAllocation,
    pub max_toadstool_resources: ComputeResourceAllocation,
    pub max_songbird_resources: NetworkingResourceAllocation,
    pub max_nestgate_resources: StorageResourceAllocation,
    pub max_squirrel_resources: AiResourceAllocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemSecurityRequirements {
    pub min_security_level: SecurityLevel,
    pub required_compliance_standards: Vec<ComplianceStandard>,
    pub encryption_requirements: EncryptionRequirements,
    pub audit_requirements: AuditRequirements,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SecurityLevel {
    Basic,
    Standard,
    High,
    Critical,
    Maximum,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComplianceStandard {
    FIPS140_2,
    CommonCriteria,
    SOC2,
    ISO27001,
    GDPR,
    HIPAA,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionRequirements {
    pub min_key_size: u32,
    pub required_algorithms: Vec<String>,
    pub hardware_backed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditRequirements {
    pub audit_level: AuditLevel,
    pub retention_days: u32,
    pub real_time_monitoring: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AuditLevel {
    Basic,
    Detailed,
    Comprehensive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemPerformanceRequirements {
    pub min_response_time_ms: f64,
    pub min_throughput_ops_per_sec: u32,
    pub max_error_rate: f64,
    pub min_availability_percentage: f64,
}

/// Main ecosystem genetic spawner
#[derive(Debug)]
pub struct EcosystemGeneticSpawner {
    /// Universal HSM provider for security genetics
    universal_hsm: Arc<UniversalHsmProvider>,
    /// Songbird service discovery for ecosystem coordination
    songbird_discovery: Arc<RwLock<SongbirdServiceDiscovery>>,
    /// Genetic algorithm configuration
    genetics_config: GeneticsConfig,
    /// Active spawning operations
    active_spawns: Arc<RwLock<HashMap<String, EcosystemSpawningOperation>>>,
    /// Spawned hybrid nodes
    hybrid_nodes: Arc<RwLock<HashMap<String, EcosystemHybridNode>>>,
    /// Ecosystem primal clients
    primal_clients: Arc<RwLock<HashMap<String, Box<dyn EcosystemPrimalClient + Send + Sync>>>>,
    /// Spawning statistics
    statistics: Arc<RwLock<EcosystemSpawningStatistics>>,
}

/// Spawning operation tracking
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
    GatheringGenetics,
    GeneratingBlueprints,
    EvaluatingFitness,
    AllocatingResources,
    SpawningNode,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpawningStage {
    Initialization,
    GeneticCollection,
    BlueprintGeneration,
    FitnessEvaluation,
    ResourceAllocation,
    NodeCreation,
    HealthValidation,
    Finalization,
}

/// Statistics for ecosystem genetic spawning
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EcosystemSpawningStatistics {
    pub total_spawns: u64,
    pub successful_spawns: u64,
    pub failed_spawns: u64,
    pub active_spawns: u64,
    pub total_hybrid_nodes: u64,
    pub average_spawn_time_ms: f64,
    pub average_fitness_score: f64,
    pub primal_usage_stats: HashMap<String, u64>,
    pub capability_usage_stats: HashMap<String, u64>,
    pub resource_utilization_stats: HashMap<String, f64>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Trait for ecosystem primal clients
/// MODERNIZED: Native async fn implementation - no async_trait overhead
#[allow(async_fn_in_trait)]
pub trait EcosystemPrimalClient {
    /// Get primal identifier
    fn get_primal_id(&self) -> &str;
    
    /// Get available genetic traits from this primal
    async fn get_genetic_traits(&self) -> BearDogResult<Vec<GeneticTrait>>;
    
    /// Request resource allocation for hybrid node
    async fn allocate_resources(&self, requirements: &serde_json::Value) -> BearDogResult<serde_json::Value>;
    
    /// Create primal-specific component for hybrid node
    async fn create_hybrid_component(&self, blueprint: &EcosystemGeneticBlueprint) -> BearDogResult<serde_json::Value>;
    
    /// Health check for primal connectivity
    async fn health_check(&self) -> BearDogResult<bool>;
    
    /// Get current resource utilization
    async fn get_resource_utilization(&self) -> BearDogResult<HashMap<String, f64>>;
}

impl EcosystemGeneticSpawner {
    /// Create a new ecosystem genetic spawner
    pub fn new(
        universal_hsm: Arc<UniversalHsmProvider>,
        songbird_discovery: Arc<RwLock<SongbirdServiceDiscovery>>,
    ) -> Self {
        Self {
            universal_hsm,
            songbird_discovery,
            genetics_config: GeneticsConfig::default(),
            active_spawns: Arc::new(RwLock::new(HashMap::new())),
            hybrid_nodes: Arc::new(RwLock::new(HashMap::new())),
            primal_clients: Arc::new(RwLock::new(HashMap::new())),
            statistics: Arc::new(RwLock::new(EcosystemSpawningStatistics::default())),
        }
    }

    /// Register an ecosystem primal client
    pub async fn register_primal_client(
        &self,
        primal_id: String,
        client: Box<dyn EcosystemPrimalClient + Send + Sync>,
    ) -> BearDogResult<()> {
        let mut clients = self.primal_clients.write().await;
        clients.insert(primal_id.clone(), client);
        info!("🌐 Registered ecosystem primal client: {}", primal_id);
        Ok(())
    }

    /// Spawn a cross-ecosystem hybrid node
    pub async fn spawn_ecosystem_hybrid_node(
        &self,
        requirements: EcosystemSpawningRequirements,
    ) -> BearDogResult<EcosystemHybridNode> {
        let operation_id = Uuid::new_v4().to_string();
        info!("🧬 Starting ecosystem hybrid node spawning: {}", operation_id);

        // Create spawning operation
        let mut operation = EcosystemSpawningOperation {
            operation_id: operation_id.clone(),
            requirements: requirements.clone(),
            status: SpawningStatus::Initializing,
            current_stage: SpawningStage::Initialization,
            progress_percentage: 0.0,
            genetic_blueprints: Vec::new(),
            selected_blueprint: None,
            resource_reservations: HashMap::new(),
            error_messages: Vec::new(),
            started_at: chrono::Utc::now(),
            estimated_completion: None,
        };

        // Store operation
        {
            let mut active_spawns = self.active_spawns.write().await;
            active_spawns.insert(operation_id.clone(), operation.clone());
        }

        // Execute spawning process
        match self.execute_ecosystem_spawning(&mut operation).await {
            Ok(hybrid_node) => {
                // Update statistics
                {
                    let mut stats = self.statistics.write().await;
                    stats.successful_spawns += 1;
                    stats.total_hybrid_nodes += 1;
                    stats.last_updated = chrono::Utc::now();
                }

                // Store hybrid node
                {
                    let mut nodes = self.hybrid_nodes.write().await;
                    nodes.insert(hybrid_node.node_id.clone(), hybrid_node.clone());
                }

                // Remove from active spawns
                {
                    let mut active_spawns = self.active_spawns.write().await;
                    active_spawns.remove(&operation_id);
                }

                info!("✅ Ecosystem hybrid node spawning completed: {}", hybrid_node.node_id);
                Ok(hybrid_node)
            }
            Err(e) => {
                error!("❌ Ecosystem hybrid node spawning failed: {}", e);
                
                // Update statistics
                {
                    let mut stats = self.statistics.write().await;
                    stats.failed_spawns += 1;
                    stats.last_updated = chrono::Utc::now();
                }

                // Update operation status
                operation.status = SpawningStatus::Failed;
                operation.error_messages.push(e.to_string());
                
                {
                    let mut active_spawns = self.active_spawns.write().await;
                    active_spawns.insert(operation_id, operation);
                }

                Err(e)
            }
        }
    }

    /// Execute the ecosystem spawning process
    async fn execute_ecosystem_spawning(
        &self,
        operation: &mut EcosystemSpawningOperation,
    ) -> BearDogResult<EcosystemHybridNode> {
        // Stage 1: Gather genetic contributions from ecosystem primals
        operation.current_stage = SpawningStage::GeneticCollection;
        operation.progress_percentage = 10.0;
        let genetic_contributions = self.gather_genetic_contributions(&operation.requirements).await?;

        // Stage 2: Generate genetic blueprints
        operation.current_stage = SpawningStage::BlueprintGeneration;
        operation.progress_percentage = 30.0;
        let blueprints = self.generate_genetic_blueprints(&genetic_contributions, &operation.requirements).await?;
        operation.genetic_blueprints = blueprints;

        // Stage 3: Evaluate fitness and select best blueprint
        operation.current_stage = SpawningStage::FitnessEvaluation;
        operation.progress_percentage = 50.0;
        let selected_blueprint = self.select_best_blueprint(&operation.genetic_blueprints, &operation.requirements).await?;
        operation.selected_blueprint = Some(selected_blueprint.clone());

        // Stage 4: Allocate resources across ecosystem
        operation.current_stage = SpawningStage::ResourceAllocation;
        operation.progress_percentage = 70.0;
        let resource_allocation = self.allocate_ecosystem_resources(&selected_blueprint, &operation.requirements).await?;

        // Stage 5: Create hybrid node
        operation.current_stage = SpawningStage::NodeCreation;
        operation.progress_percentage = 90.0;
        let hybrid_node = self.create_hybrid_node(selected_blueprint, resource_allocation).await?;

        // Stage 6: Validate node health
        operation.current_stage = SpawningStage::HealthValidation;
        operation.progress_percentage = 95.0;
        self.validate_node_health(&hybrid_node).await?;

        // Stage 7: Finalize
        operation.current_stage = SpawningStage::Finalization;
        operation.status = SpawningStatus::Completed;
        operation.progress_percentage = 100.0;

        Ok(hybrid_node)
    }

    /// Gather genetic contributions from ecosystem primals
    async fn gather_genetic_contributions(
        &self,
        requirements: &EcosystemSpawningRequirements,
    ) -> BearDogResult<Vec<EcosystemGeneticContribution>> {
        let mut contributions = Vec::new();
        let clients = self.primal_clients.read().await;

        for (primal_id, client) in clients.iter() {
            match client.get_genetic_traits().await {
                Ok(traits) => {
                    let contribution = EcosystemGeneticContribution {
                        primal_id: primal_id.clone(),
                        primal_name: self.get_primal_display_name(primal_id).unwrap_or_else(|| primal_id.clone()),
                        contributed_traits: traits,
                        contribution_weight: 1.0 / clients.len() as f64, // Equal weight for now
                        compatibility_score: self.calculate_primal_compatibility(primal_id, &traits).await.unwrap_or(0.8),
                        primal_metadata: HashMap::new(),
                    };
                    contributions.push(contribution);
                }
                Err(e) => {
                    warn!("Failed to get genetic traits from {}: {}", primal_id, e);
                }
            }
        }

        if contributions.is_empty() {
            return Err(BearDogError::system_error("No genetic contributions available".to_string()));
        }

        info!("🧬 Gathered genetic contributions from {} primals", contributions.len());
        Ok(contributions)
    }

    /// Generate genetic blueprints through recombination
    async fn generate_genetic_blueprints(
        &self,
        contributions: &[EcosystemGeneticContribution],
        requirements: &EcosystemSpawningRequirements,
    ) -> BearDogResult<Vec<EcosystemGeneticBlueprint>> {
        let mut blueprints = Vec::new();
        
        // Generate multiple blueprint candidates
        for generation in 0..self.genetics_config.max_generations {
            let blueprint = self.create_genetic_blueprint(contributions, generation).await?;
            let fitness = self.calculate_fitness_score(&blueprint, requirements).await?;
            
            if fitness >= requirements.min_fitness_score {
                blueprints.push(blueprint);
            }
            
            // Stop early if we have enough good candidates
            if blueprints.len() >= 5 {
                break;
            }
        }

        if blueprints.is_empty() {
            return Err(BearDogError::system_error(
                "No genetic blueprints met minimum fitness requirements".to_string(),
            ));
        }

        info!("🧬 Generated {} genetic blueprints", blueprints.len());
        Ok(blueprints)
    }

    /// Create a genetic blueprint through recombination
    async fn create_genetic_blueprint(
        &self,
        contributions: &[EcosystemGeneticContribution],
        generation: u32,
    ) -> BearDogResult<EcosystemGeneticBlueprint> {
        let blueprint_id = Uuid::new_v4().to_string();
        
        // Combine traits from different primals
        let mut hybrid_traits = Vec::new();
        for contribution in contributions {
            // Select best traits from each primal
            let mut primal_traits = contribution.contributed_traits.clone();
            primal_traits.sort_by(|a, b| {
                b.strength.partial_cmp(&a.strength)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });
            
            // Take top traits based on contribution weight
            let trait_count = (primal_traits.len() as f64 * contribution.contribution_weight).ceil() as usize;
            hybrid_traits.extend(primal_traits.into_iter().take(trait_count));
        }

        // Apply mutation if configured
        let mutation_applied = if self.genetics_config.mutation_rate > 0.0 {
            self.apply_mutation(&mut hybrid_traits).await?
        } else {
            false
        };

        let blueprint = EcosystemGeneticBlueprint {
            blueprint_id,
            parent_contributions: contributions.to_vec(),
            hybrid_traits,
            fitness_score: 0.0, // Will be calculated separately
            generation,
            mutation_applied,
            crossover_points: vec![0, contributions.len()], // Simple crossover
            diversity_score: self.calculate_diversity_score(contributions).await?,
        };

        Ok(blueprint)
    }

    /// Calculate fitness score for a genetic blueprint
    async fn calculate_fitness_score(
        &self,
        blueprint: &EcosystemGeneticBlueprint,
        requirements: &EcosystemSpawningRequirements,
    ) -> BearDogResult<f64> {
        let mut fitness_score = 0.0;
        let mut total_weight = 0.0;

        // Score based on capability coverage
        for target_capability in &requirements.target_capabilities {
            let coverage = self.calculate_capability_coverage(blueprint, target_capability).await?;
            fitness_score += coverage * 0.4; // 40% weight for capability coverage
            total_weight += 0.4;
        }

        // Score based on genetic diversity
        fitness_score += blueprint.diversity_score * 0.3; // 30% weight for diversity
        total_weight += 0.3;

        // Score based on trait strength
        let avg_trait_strength = blueprint.hybrid_traits.iter()
            .map(|t| t.strength)
            .sum::<f64>() / blueprint.hybrid_traits.len() as f64;
        fitness_score += avg_trait_strength * 0.3; // 30% weight for trait strength
        total_weight += 0.3;

        Ok(fitness_score / total_weight)
    }

    /// Calculate capability coverage for a blueprint
    async fn calculate_capability_coverage(
        &self,
        blueprint: &EcosystemGeneticBlueprint,
        capability: &EcosystemCapability,
    ) -> BearDogResult<f64> {
        let relevant_traits = blueprint.hybrid_traits.iter()
            .filter(|trait_| trait_.required_capabilities.contains(capability))
            .collect::<Vec<_>>();

        if relevant_traits.is_empty() {
            return Ok(0.0);
        }

        // Average strength of relevant traits
        let coverage = relevant_traits.iter()
            .map(|t| t.strength)
            .sum::<f64>() / relevant_traits.len() as f64;

        Ok(coverage)
    }

    /// Calculate genetic diversity score
    async fn calculate_diversity_score(
        &self,
        contributions: &[EcosystemGeneticContribution],
    ) -> BearDogResult<f64> {
        if contributions.len() <= 1 {
            return Ok(0.0);
        }

        // Diversity increases with number of different primals
        let unique_primals = contributions.iter()
            .map(|c| &c.primal_id)
            .collect::<std::collections::HashSet<_>>()
            .len();

        let diversity_score = unique_primals as f64 / 5.0; // Normalize to 5 max primals
        Ok(diversity_score.min(1.0))
    }

    /// Apply genetic mutation to traits
    async fn apply_mutation(&self, traits: &mut [GeneticTrait]) -> BearDogResult<bool> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut mutation_applied = false;

        for trait_ in traits.iter_mut() {
            if rng.gen::<f64>() < self.genetics_config.mutation_rate {
                // Mutate trait strength slightly
                let mutation_factor = rng.gen_range(0.9..1.1);
                trait_.strength = (trait_.strength * mutation_factor).min(1.0).max(0.0);
                mutation_applied = true;
            }
        }

        Ok(mutation_applied)
    }

    /// Select the best genetic blueprint
    async fn select_best_blueprint(
        &self,
        blueprints: &[EcosystemGeneticBlueprint],
        requirements: &EcosystemSpawningRequirements,
    ) -> BearDogResult<EcosystemGeneticBlueprint> {
        if blueprints.is_empty() {
            return Err(BearDogError::system_error("No blueprints available for selection".to_string()));
        }

        // Calculate fitness scores for all blueprints
        let mut scored_blueprints = Vec::new();
        for blueprint in blueprints {
            let fitness = self.calculate_fitness_score(blueprint, requirements).await?;
            scored_blueprints.push((blueprint, fitness));
        }

        // Sort by fitness score (descending)
        scored_blueprints.sort_by(|a, b| {
            b.1.partial_cmp(&a.1)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let best_blueprint = scored_blueprints[0].0.clone();
        info!("🧬 Selected blueprint with fitness score: {:.3}", scored_blueprints[0].1);

        Ok(best_blueprint)
    }

    /// Allocate resources across ecosystem for hybrid node
    async fn allocate_ecosystem_resources(
        &self,
        blueprint: &EcosystemGeneticBlueprint,
        requirements: &EcosystemSpawningRequirements,
    ) -> BearDogResult<EcosystemResourceAllocation> {
        // Create intelligent resource allocation based on blueprint requirements
        let allocation = EcosystemResourceAllocation {
            beardog_security: SecurityResourceAllocation {
                hsm_slots: self.calculate_hsm_slots(blueprint, requirements),
                key_storage_mb: self.calculate_key_storage(blueprint, requirements),
                crypto_operations_per_sec: self.calculate_crypto_ops(blueprint, requirements),
                audit_log_retention_days: self.calculate_audit_retention(blueprint, requirements),
            },
            toadstool_compute: ComputeResourceAllocation {
                cpu_cores: 4,
                memory_gb: 8,
                gpu_units: 1,
                storage_gb: 100,
                network_bandwidth_mbps: 1000,
            },
            songbird_networking: NetworkingResourceAllocation {
                connection_pool_size: 100,
                load_balancer_instances: 2,
                service_mesh_endpoints: 10,
            },
            nestgate_storage: StorageResourceAllocation {
                primary_storage_gb: 500,
                backup_storage_gb: 1000,
                cache_storage_gb: 50,
                replication_factor: 3,
            },
            squirrel_ai: AiResourceAllocation {
                model_inference_slots: 2,
                training_compute_units: 1,
                ai_memory_gb: 4,
            },
        };

        info!("🧬 Allocated ecosystem resources for hybrid node");
        Ok(allocation)
    }

    /// Create the hybrid node from blueprint and resources
    async fn create_hybrid_node(
        &self,
        blueprint: EcosystemGeneticBlueprint,
        resource_allocation: EcosystemResourceAllocation,
    ) -> BearDogResult<EcosystemHybridNode> {
        let node_id = Uuid::new_v4().to_string();
        let node_name = format!("hybrid-{}", &node_id[..8]);

        // Extract capabilities from genetic traits
        let mut capabilities = Vec::new();
        for trait_ in &blueprint.hybrid_traits {
            capabilities.extend(trait_.required_capabilities.clone());
        }
        capabilities.sort();
        capabilities.dedup();

        let hybrid_node = EcosystemHybridNode {
            node_id,
            node_name,
            genetic_blueprint: blueprint,
            capabilities,
            resource_allocation,
            health_status: NodeHealthStatus::Healthy,
            performance_metrics: NodePerformanceMetrics {
                cpu_utilization: 0.0,
                memory_utilization: 0.0,
                network_throughput_mbps: 0.0,
                storage_iops: 0,
                response_time_ms: 0.0,
                error_rate: 0.0,
                uptime_percentage: 100.0,
            },
            created_at: chrono::Utc::now(),
            last_health_check: chrono::Utc::now(),
        };

        info!("🧬 Created ecosystem hybrid node: {}", hybrid_node.node_id);
        Ok(hybrid_node)
    }

    /// Validate hybrid node health
    async fn validate_node_health(&self, node: &EcosystemHybridNode) -> BearDogResult<()> {
        // Comprehensive health validation
        
        // 1. Basic health status check
        if node.health_status != NodeHealthStatus::Healthy {
            return Err(BearDogError::system_error(format!(
                "Node {} is not in healthy state: {:?}",
                node.node_id, node.health_status
            )));
        }
        
        // 2. Resource utilization check
        if node.resource_utilization > 0.95 {
            warn!("⚠️ Node {} has high resource utilization: {:.1}%", 
                  node.node_id, node.resource_utilization * 100.0);
        }
        
        // 3. Security validation
        if node.security_level < SecurityLevel::Testing {
            return Err(BearDogError::system_error(format!(
                "Node {} has insufficient security level: {:?}",
                node.node_id, node.security_level
            )));
        }
        
        // 4. Connectivity check
        if node.last_heartbeat.is_none() {
            return Err(BearDogError::system_error(format!(
                "Node {} has no heartbeat recorded",
                node.node_id
            )));
        }
        
        info!("✅ Comprehensive health validation passed: {}", node.node_id);
        Ok(())
    }

    /// Get spawning statistics
    pub async fn get_spawning_statistics(&self) -> BearDogResult<EcosystemSpawningStatistics> {
        let stats = self.statistics.read().await;
        Ok(stats.clone())
    }

    /// Get active spawning operations
    pub async fn get_active_spawns(&self) -> BearDogResult<Vec<EcosystemSpawningOperation>> {
        let active_spawns = self.active_spawns.read().await;
        Ok(active_spawns.values().cloned().collect())
    }

    /// Get hybrid nodes
    pub async fn get_hybrid_nodes(&self) -> BearDogResult<Vec<EcosystemHybridNode>> {
        let nodes = self.hybrid_nodes.read().await;
        Ok(nodes.values().cloned().collect())
    }

    /// Get display name for a primal, with fallback to ID
    fn get_primal_display_name(&self, primal_id: &str) -> Option<String> {
        // Map known primal IDs to display names
        match primal_id {
            "songbird" => Some("SongBird Service Mesh".to_string()),
            "nestgate" => Some("NestGate Storage".to_string()),
            "biome" => Some("BiomeOS Platform".to_string()),
            "squirrel" => Some("Squirrel Analytics".to_string()),
            "toadstool" => Some("ToadStool Network".to_string()),
            _ => None,
        }
    }

    /// Calculate compatibility score between primal and traits
    async fn calculate_primal_compatibility(
        &self,
        _primal_id: &str,
        traits: &[EcosystemGeneticTrait],
    ) -> BearDogResult<f64> {
        // Simple compatibility calculation based on trait strength
        if traits.is_empty() {
            return Ok(0.5);
        }
        
        let avg_strength: f64 = traits.iter().map(|t| t.strength).sum::<f64>() / traits.len() as f64;
        let compatibility = (avg_strength * 0.8).min(1.0).max(0.1);
        
        Ok(compatibility)
    }
    
    /// Calculate HSM slots based on blueprint complexity
    fn calculate_hsm_slots(&self, blueprint: &EcosystemBlueprint, requirements: &EcosystemSpawningRequirements) -> u32 {
        let base_slots = 2;
        let complexity_factor = match blueprint.security_level {
            SecurityLevel::Development => 1,
            SecurityLevel::Testing => 2,
            SecurityLevel::Production => 3,
        };
        let requirement_factor = if requirements.high_availability { 2 } else { 1 };
        (base_slots * complexity_factor * requirement_factor).min(16)
    }
    
    /// Calculate key storage based on expected usage
    fn calculate_key_storage(&self, blueprint: &EcosystemBlueprint, requirements: &EcosystemSpawningRequirements) -> u32 {
        let base_storage = 100; // MB
        let service_factor = blueprint.expected_services.max(1) as u32;
        let requirement_factor = if requirements.high_availability { 2 } else { 1 };
        (base_storage * service_factor * requirement_factor).min(10240) // Max 10GB
    }
    
    /// Calculate crypto operations per second
    fn calculate_crypto_ops(&self, blueprint: &EcosystemBlueprint, requirements: &EcosystemSpawningRequirements) -> u32 {
        let base_ops = 1000;
        let service_factor = blueprint.expected_services.max(1) as u32;
        let requirement_factor = if requirements.high_availability { 3 } else { 2 };
        base_ops * service_factor * requirement_factor
    }
    
    /// Calculate audit log retention days
    fn calculate_audit_retention(&self, _blueprint: &EcosystemBlueprint, requirements: &EcosystemSpawningRequirements) -> u32 {
        if requirements.high_availability {
            365 // 1 year for high availability
        } else {
            90  // 90 days for standard
        }
    }
} 