

use super::types::*;
use super::traits::EcosystemPrimalClient;
use crate::ecosystem_integration::{UniversalHsmProvider, SongbirdServiceDiscovery};
use beardog_errors::{BearDogError, BearDogResult};
use beardog_types::canonical::genetics::GeneticsConfig;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn, error};
use uuid::Uuid;

pub struct EcosystemGeneticSpawner {

    pub universal_hsm: Arc<UniversalHsmProvider>,

    pub songbird_discovery: Arc<RwLock<SongbirdServiceDiscovery>>,

    pub genetics_config: GeneticsConfig,

    pub active_spawns: Arc<RwLock<ahash::HashMap<String, EcosystemSpawningOperation>>>,

    pub hybrid_nodes: Arc<RwLock<ahash::HashMap<String, EcosystemHybridNode>>>,

    pub primal_clients: Arc<RwLock<ahash::HashMap<String, Box<dyn EcosystemPrimalClient + Send + Sync>>>>,

    pub statistics: Arc<RwLock<EcosystemSpawningStatistics>>,
}

impl EcosystemGeneticSpawner {

    pub fn new(
        universal_hsm: Arc<UniversalHsmProvider>,
        songbird_discovery: Arc<RwLock<SongbirdServiceDiscovery>>,
    ) -> Self {
        Self {
            universal_hsm,
            songbird_discovery,
            genetics_config: GeneticsConfig::default(),
            active_spawns: Arc::new(RwLock::new(ahash::HashMap::default())),
            hybrid_nodes: Arc::new(RwLock::new(ahash::HashMap::default())),
            primal_clients: Arc::new(RwLock::new(ahash::HashMap::default())),
            statistics: Arc::new(RwLock::new(EcosystemSpawningStatistics::default())),
        }
    }

    pub async fn register_primal_client(
        &self,
        primal_id: &str,
        client: Box<dyn EcosystemPrimalClient + Send + Sync>,
    ) -> BearDogResult<()> {
        let mut clients = self.primal_clients.write().await;
        clients.insert(primal_id.clone(), client);
        info!("🌐 Registered ecosystem primal client: {}", primal_id);
        Ok(())
    }

    pub async fn spawn_ecosystem_hybrid_node(
        &self,
        requirements: EcosystemSpawningRequirements,
    ) -> BearDogResult<EcosystemHybridNode> {
        let operation_id = Uuid::new_v4().to_string();
        info!("🧬 Starting ecosystem hybrid node spawning: {}", operation_id);

        let mut operation = EcosystemSpawningOperation {
            operation_id: operation_id.clone(),
            requirements: requirements.clone(),
            status: SpawningStatus::Initializing,
            current_stage: SpawningStage::Initialization,
            progress_percentage: 0.0,
            genetic_blueprints: Vec::new(),
            selected_blueprint: None,
            resource_reservations: ahash::HashMap::default(),
            error_messages: Vec::new(),
            started_at: chrono::Utc::now(),
            estimated_completion: None,
        };

        {
            let mut active_spawns = self.active_spawns.write().await;
            active_spawns.insert(operation_id.clone(), operation.clone());
        }

        match self.execute_ecosystem_spawning(&mut operation).await {
            Ok(hybrid_node) => {

                {
                    let mut stats = self.statistics.write().await;
                    stats.successful_spawns += 1;
                    stats.total_hybrid_nodes += 1;
                    stats.last_updated = chrono::Utc::now();
                }

                {
                    let mut nodes = self.hybrid_nodes.write().await;
                    nodes.insert(hybrid_node.node_id.clone(), hybrid_node.clone());
                }

                {
                    let mut active_spawns = self.active_spawns.write().await;
                    active_spawns.remove(&operation_id);
                }

                info!("✅ Ecosystem hybrid node spawning completed: {}", hybrid_node.node_id);
                Ok(hybrid_node)
            }
            Err(e) => {
                error!("❌ Ecosystem hybrid node spawning failed: {}", e);

                {
                    let mut stats = self.statistics.write().await;
                    stats.failed_spawns += 1;
                    stats.last_updated = chrono::Utc::now();
                }

                {
                    let mut active_spawns = self.active_spawns.write().await;
                    active_spawns.remove(&operation_id);
                }

                Err(e)
            }
        }
    }

    async fn execute_ecosystem_spawning(
        &self,
        operation: &mut EcosystemSpawningOperation,
    ) -> BearDogResult<EcosystemHybridNode> {

        self.update_operation_stage(operation, SpawningStage::RequirementAnalysis, 10.0).await?;
        let analyzed_requirements = self.analyze_spawning_requirements(&operation.requirements).await?;

        self.update_operation_stage(operation, SpawningStage::BlueprintGeneration, 30.0).await?;
        let blueprints = self.generate_genetic_blueprints(&analyzed_requirements).await?;
        operation.genetic_blueprints = blueprints;

        self.update_operation_stage(operation, SpawningStage::BlueprintSelection, 50.0).await?;
        let selected_blueprint = self.select_optimal_blueprint(&operation.genetic_blueprints, &operation.requirements).await?;
        operation.selected_blueprint = Some(selected_blueprint.clone());

        self.update_operation_stage(operation, SpawningStage::ResourceAllocation, 70.0).await?;
        let resource_allocation = self.allocate_ecosystem_resources(&selected_blueprint, &operation.requirements).await?;

        self.update_operation_stage(operation, SpawningStage::NodeCreation, 85.0).await?;
        let hybrid_node = self.create_hybrid_node(&selected_blueprint, &resource_allocation).await?;

        self.update_operation_stage(operation, SpawningStage::HealthValidation, 95.0).await?;
        self.validate_hybrid_node_health(&hybrid_node).await?;

        self.update_operation_stage(operation, SpawningStage::Finalization, 100.0).await?;
        operation.status = SpawningStatus::Completed;

        info!("🎉 Ecosystem hybrid node created successfully: {}", hybrid_node.node_id);
        Ok(hybrid_node)
    }

    async fn update_operation_stage(
        &self,
        operation: &mut EcosystemSpawningOperation,
        stage: SpawningStage,
        progress: f64,
    ) -> BearDogResult<()> {
        operation.current_stage = stage.clone();
        operation.progress_percentage = progress;

        {
            let mut active_spawns = self.active_spawns.write().await;
            if let Some(stored_operation) = active_spawns.get_mut(&operation.operation_id) {
                stored_operation.current_stage = stage;
                stored_operation.progress_percentage = progress;
            }
        }

        debug!("🔄 Spawning operation {} progress: {:.1}% - {:?}", 
               operation.operation_id, progress, operation.current_stage);
        Ok(())
    }

    async fn analyze_spawning_requirements(
        &self,
        requirements: &EcosystemSpawningRequirements,
    ) -> BearDogResult<EcosystemSpawningRequirements> {
        debug!("🔍 Analyzing spawning requirements for {} capabilities", 
               requirements.required_capabilities.len());

        Ok(requirements.clone())
    }

    async fn generate_genetic_blueprints(
        &self,
        requirements: &EcosystemSpawningRequirements,
    ) -> BearDogResult<Vec<EcosystemGeneticBlueprint>> {
        debug!("🧬 Generating genetic blueprints");
        
        let mut blueprints = Vec::new();

        let blueprint = EcosystemGeneticBlueprint {
            blueprint_id: Uuid::new_v4().to_string(),
            name: "Hybrid Security-Compute Node".to_string(),
            primary_contributions: Vec::new(),
            expected_performance: NodePerformanceMetrics::default(),
            resource_requirements: self.calculate_resource_requirements(requirements).await?,
            security_level: requirements.security_requirements.min_security_level.clone(),
            estimated_spawn_time: 300, // 5 minutes
            compatibility_score: 0.85,
            expected_services: 5,
        };
        
        blueprints.push(blueprint);
        
        info!("📋 Generated {} genetic blueprints", blueprints.len());
        Ok(blueprints)
    }

    async fn select_optimal_blueprint(
        &self,
        blueprints: &[EcosystemGeneticBlueprint],
        _requirements: &EcosystemSpawningRequirements,
    ) -> BearDogResult<EcosystemGeneticBlueprint> {
        if blueprints.is_empty() {
            return Err(BearDogError::system("No genetic blueprints available"));
        }

        let selected = blueprints[0].clone();
        info!("🎯 Selected blueprint: {}", selected.name);
        Ok(selected)
    }

    async fn calculate_resource_requirements(
        &self,
        requirements: &EcosystemSpawningRequirements,
    ) -> BearDogResult<EcosystemResourceAllocation> {
        Ok(EcosystemResourceAllocation {
            security: SecurityResourceAllocation {
                hsm_slots: 4,
                key_storage_mb: 512,
                crypto_ops_per_second: 10000,
                audit_retention_days: if requirements.high_availability { 365 } else { 90 },
            },
            compute: ComputeResourceAllocation {
                cpu_cores: 8,
                memory_gb: 32,
                compute_units_per_second: 1000000,
            },
            networking: NetworkingResourceAllocation {
                bandwidth_mbps: 1000,
                connections_per_second: 10000,
            },
            storage: StorageResourceAllocation {
                capacity_gb: 1000,
                iops: 10000,
            },
            ai: AiResourceAllocation {
                gpu_compute_units: 4,
                model_storage_gb: 100,
            },
        })
    }

    async fn allocate_ecosystem_resources(
        &self,
        blueprint: &EcosystemGeneticBlueprint,
        requirements: &EcosystemSpawningRequirements,
    ) -> BearDogResult<EcosystemResourceAllocation> {
        debug!("💾 Allocating ecosystem resources");

        let mut allocation = blueprint.resource_requirements.clone();

        if requirements.high_availability {
            allocation.compute.cpu_cores *= 2;
            allocation.compute.memory_gb *= 2;
            allocation.security.hsm_slots *= 2;
        }
        
        info!("✅ Resource allocation completed");
        Ok(allocation)
    }

    async fn create_hybrid_node(
        &self,
        blueprint: &EcosystemGeneticBlueprint,
        resource_allocation: &EcosystemResourceAllocation,
    ) -> BearDogResult<EcosystemHybridNode> {
        let node_id = Uuid::new_v4().to_string();
        debug!("🏗️ Creating hybrid node: {}", node_id);
        
        let hybrid_node = EcosystemHybridNode {
            node_id: node_id.clone(),
            genetic_contributions: blueprint.primary_contributions.clone(),
            combined_traits: Vec::new(),
            resource_allocation: resource_allocation.clone(),
            health_status: NodeHealthStatus::Initializing,
            performance_metrics: NodePerformanceMetrics::default(),
            created_at: chrono::Utc::now(),
            last_heartbeat: Some(chrono::Utc::now()),
            security_level: blueprint.security_level.clone(),
            active_capabilities: Vec::new(),
        };
        
        info!("🎉 Hybrid node created: {}", node_id);
        Ok(hybrid_node)
    }

    async fn validate_hybrid_node_health(&self, node: &EcosystemHybridNode) -> BearDogResult<()> {
        debug!("🏥 Validating hybrid node health: {}", node.node_id);

        if node.health_status == NodeHealthStatus::Critical || node.health_status == NodeHealthStatus::Offline {
            return Err(BearDogError::system(format!(
                "Node {} has critical health status: {:?}",
                node.node_id, node.health_status
            )));
        }
        
        if node.security_level < SecurityLevel::Development {
            return Err(BearDogError::security(format!(
                "Node {} has insufficient security level: {:?}",
                node.node_id, node.security_level
            )));
        }
        
        if node.last_heartbeat.is_none() {
            return Err(BearDogError::system(format!(
                "Node {} has no heartbeat recorded",
                node.node_id
            )));
        }
        
        info!("✅ Comprehensive health validation passed: {}", node.node_id);
        Ok(())
    }

    pub async fn get_spawning_statistics(&self) -> BearDogResult<EcosystemSpawningStatistics> {
        let stats = self.statistics.read().await;
        Ok(stats.clone())
    }

    pub async fn get_active_spawns(&self) -> BearDogResult<Vec<EcosystemSpawningOperation>> {
        let active_spawns = self.active_spawns.read().await;
        Ok(active_spawns.values().cloned().collect())
    }

    pub async fn get_hybrid_nodes(&self) -> BearDogResult<Vec<EcosystemHybridNode>> {
        let nodes = self.hybrid_nodes.read().await;
        Ok(nodes.values().cloned().collect())
    }
} 