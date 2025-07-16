//! Core genetic spawning functionality
//!
//! This module contains the main CrossEcosystemGeneticSpawner implementation
//! and the core spawning logic for creating hybrid nodes.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

use super::config::{GeneticAlgorithmConfig, SpawningStatistics};
use super::genetics::{GeneticBlueprint, GeneticTrait, HybridCapability, ParentGeneticContribution};
use super::operations::{SpawningOperation, SpawningStage, SpawningStatus};
use crate::adapters::universal::manager::HybridNode;
use crate::adapters::universal::traits::*;
use crate::{BearDogCore, BearDogError, BearDogResult};

/// Cross-Ecosystem Genetic Spawner
///
/// Revolutionary system that enables genetic algorithms to create hybrid nodes
/// with capabilities from multiple ecosystem components. This enables
/// unprecedented capabilities impossible with individual components.
pub struct CrossEcosystemGeneticSpawner {
    /// Core BearDog instance
    core: Arc<BearDogCore>,
    
    /// Active spawning operations
    active_spawns: Arc<RwLock<HashMap<Uuid, SpawningOperation>>>,
    
    /// Spawned hybrid nodes
    hybrid_nodes: Arc<RwLock<HashMap<String, HybridNode>>>,
    
    /// Genetic algorithm configuration
    genetic_config: GeneticAlgorithmConfig,
    
    /// Spawning statistics
    statistics: Arc<RwLock<SpawningStatistics>>,
}

impl CrossEcosystemGeneticSpawner {
    /// Create a new cross-ecosystem genetic spawner
    pub async fn new(core: Arc<BearDogCore>) -> BearDogResult<Self> {
        info!("🧬 Initializing Cross-Ecosystem Genetic Spawner");
        
        Ok(Self {
            core,
            active_spawns: Arc::new(RwLock::new(HashMap::new())),
            hybrid_nodes: Arc::new(RwLock::new(HashMap::new())),
            genetic_config: GeneticAlgorithmConfig::default(),
            statistics: Arc::new(RwLock::new(SpawningStatistics::new())),
        })
    }
    
    /// Create with custom genetic algorithm configuration
    pub async fn with_genetic_config(
        core: Arc<BearDogCore>,
        genetic_config: GeneticAlgorithmConfig,
    ) -> BearDogResult<Self> {
        info!("🧬 Initializing Cross-Ecosystem Genetic Spawner with custom config");
        
        // Validate configuration
        genetic_config.validate()
            .map_err(|e| BearDogError::invalid_input(&format!("Invalid genetic config: {}", e)))?;
        
        Ok(Self {
            core,
            active_spawns: Arc::new(RwLock::new(HashMap::new())),
            hybrid_nodes: Arc::new(RwLock::new(HashMap::new())),
            genetic_config,
            statistics: Arc::new(RwLock::new(SpawningStatistics::new())),
        })
    }
    
    /// Spawn a hybrid node with capabilities from multiple ecosystems
    pub async fn spawn_hybrid_node(
        &self,
        parent_nodes: Vec<EcosystemNodeInfo>,
        target_capabilities: Vec<HybridCapability>,
        security_requirements: SecurityRequirements,
        resource_constraints: ResourceConstraints,
    ) -> BearDogResult<HybridNode> {
        let start_time = std::time::Instant::now();
        
        // Create spawning operation
        let mut spawning_operation = SpawningOperation::new(
            parent_nodes.clone(),
            target_capabilities.clone(),
            security_requirements.clone(),
            resource_constraints.clone(),
        );
        
        let spawn_id = spawning_operation.spawn_id;
        
        info!("🧬 Starting hybrid node spawning operation: {}", spawn_id);
        
        // Store spawning operation
        {
            let mut active_spawns = self.active_spawns.write().await;
            active_spawns.insert(spawn_id, spawning_operation.clone());
        }
        
        // Update statistics
        {
            let mut stats = self.statistics.write().await;
            stats.active_spawns += 1;
            for capability in &target_capabilities {
                stats.record_capability_usage(capability.clone());
            }
            let ecosystem_ids: Vec<_> = parent_nodes.iter().map(|n| n.ecosystem_id.clone()).collect();
            stats.record_ecosystem_combination(ecosystem_ids);
        }
        
        // Execute spawning process
        match self.execute_spawning_process(spawn_id).await {
            Ok(hybrid_node) => {
                // Mark spawning as completed
                self.update_spawning_status(spawn_id, SpawningStatus::Completed, SpawningStage::RegistrationComplete, 100.0, None).await;
                
                // Store hybrid node
                {
                    let mut hybrid_nodes = self.hybrid_nodes.write().await;
                    hybrid_nodes.insert(hybrid_node.node_id.clone(), hybrid_node.clone());
                }
                
                // Update statistics
                let spawn_time_ms = start_time.elapsed().as_millis() as u64;
                {
                    let mut stats = self.statistics.write().await;
                    stats.update_spawn_result(true, spawn_time_ms);
                    stats.active_spawns -= 1;
                }
                
                info!("✅ Successfully spawned hybrid node: {}", hybrid_node.node_id);
                Ok(hybrid_node)
            }
            Err(e) => {
                // Mark spawning as failed
                let error_message = format!("Spawning failed: {}", e);
                self.update_spawning_status(spawn_id, SpawningStatus::Failed, SpawningStage::Validation, 0.0, Some(error_message.clone())).await;
                
                // Update statistics
                let spawn_time_ms = start_time.elapsed().as_millis() as u64;
                {
                    let mut stats = self.statistics.write().await;
                    stats.update_spawn_result(false, spawn_time_ms);
                    stats.active_spawns -= 1;
                }
                
                warn!("❌ Failed to spawn hybrid node: {}", error_message);
                Err(e)
            }
        }
    }
    
    /// Get status of a spawning operation
    pub async fn get_spawning_status(&self, spawn_id: Uuid) -> Option<SpawningOperation> {
        let active_spawns = self.active_spawns.read().await;
        active_spawns.get(&spawn_id).cloned()
    }
    
    /// Get all active spawning operations
    pub async fn get_active_spawns(&self) -> Vec<SpawningOperation> {
        let active_spawns = self.active_spawns.read().await;
        active_spawns.values().cloned().collect()
    }
    
    /// Get all spawned hybrid nodes
    pub async fn get_hybrid_nodes(&self) -> Vec<HybridNode> {
        let hybrid_nodes = self.hybrid_nodes.read().await;
        hybrid_nodes.values().cloned().collect()
    }
    
    /// Get a specific hybrid node
    pub async fn get_hybrid_node(&self, node_id: &str) -> Option<HybridNode> {
        let hybrid_nodes = self.hybrid_nodes.read().await;
        hybrid_nodes.get(node_id).cloned()
    }
    
    /// Cancel a spawning operation
    pub async fn cancel_spawning(&self, spawn_id: Uuid) -> BearDogResult<()> {
        self.update_spawning_status(
            spawn_id,
            SpawningStatus::Cancelled,
            SpawningStage::Validation,
            0.0,
            Some("Spawning cancelled by user".to_string()),
        ).await;
        
        // Update statistics
        {
            let mut stats = self.statistics.write().await;
            stats.active_spawns = stats.active_spawns.saturating_sub(1);
        }
        
        info!("🚫 Cancelled spawning operation: {}", spawn_id);
        Ok(())
    }
    
    /// Get spawning statistics
    pub async fn get_spawning_statistics(&self) -> SpawningStatistics {
        self.statistics.read().await.clone()
    }
    
    /// Update genetic algorithm configuration
    pub async fn update_genetic_config(&mut self, config: GeneticAlgorithmConfig) -> BearDogResult<()> {
        config.validate()
            .map_err(|e| BearDogError::invalid_input(&format!("Invalid genetic config: {}", e)))?;
        
        self.genetic_config = config;
        info!("🧬 Updated genetic algorithm configuration");
        Ok(())
    }
    
    /// Get current genetic algorithm configuration
    pub fn get_genetic_config(&self) -> &GeneticAlgorithmConfig {
        &self.genetic_config
    }
    
    /// Execute the complete spawning process
    async fn execute_spawning_process(&self, spawn_id: Uuid) -> BearDogResult<HybridNode> {
        // Stage 1: Validation
        self.update_spawning_status(spawn_id, SpawningStatus::InProgress, SpawningStage::Validation, 10.0, None).await;
        self.validate_spawning_requirements(spawn_id).await?;
        
        // Stage 2: Genetic Recombination
        self.update_spawning_status(spawn_id, SpawningStatus::InProgress, SpawningStage::GeneticRecombination, 25.0, None).await;
        let genetic_blueprint = self.perform_genetic_recombination(spawn_id).await?;
        
        // Stage 3: Capability Merging
        self.update_spawning_status(spawn_id, SpawningStatus::InProgress, SpawningStage::CapabilityMerging, 40.0, None).await;
        let merged_capabilities = self.merge_ecosystem_capabilities(spawn_id, genetic_blueprint).await?;
        
        // Stage 4: Security Configuration
        self.update_spawning_status(spawn_id, SpawningStatus::InProgress, SpawningStage::SecurityConfiguration, 60.0, None).await;
        let security_context = self.configure_hybrid_security(spawn_id).await?;
        
        // Stage 5: Resource Allocation
        self.update_spawning_status(spawn_id, SpawningStatus::InProgress, SpawningStage::ResourceAllocation, 75.0, None).await;
        let resource_allocation = self.allocate_hybrid_resources(spawn_id).await?;
        
        // Stage 6: Node Initialization
        self.update_spawning_status(spawn_id, SpawningStatus::InProgress, SpawningStage::NodeInitialization, 85.0, None).await;
        let node_id = self.initialize_hybrid_node(spawn_id, merged_capabilities, security_context, resource_allocation).await?;
        
        // Stage 7: Health Verification
        self.update_spawning_status(spawn_id, SpawningStatus::InProgress, SpawningStage::HealthVerification, 95.0, None).await;
        self.verify_hybrid_node_health(&node_id).await?;
        
        // Create final hybrid node
        let hybrid_node = self.create_hybrid_node_instance(spawn_id, node_id).await?;
        
        Ok(hybrid_node)
    }
    
    /// Validate spawning requirements
    async fn validate_spawning_requirements(&self, spawn_id: Uuid) -> BearDogResult<()> {
        let spawning_op = {
            let active_spawns = self.active_spawns.read().await;
            active_spawns.get(&spawn_id).cloned()
                .ok_or_else(|| BearDogError::internal("Spawning operation not found"))?
        };
        
        // Validate parent nodes
        if spawning_op.parent_nodes.is_empty() {
            return Err(BearDogError::invalid_input("At least one parent node is required"));
        }
        
        if spawning_op.parent_nodes.len() > 10 {
            return Err(BearDogError::invalid_input("Too many parent nodes (max 10)"));
        }
        
        // Validate target capabilities
        if spawning_op.target_capabilities.is_empty() {
            return Err(BearDogError::invalid_input("At least one target capability is required"));
        }
        
        // Validate resource constraints
        if let Some(cpu_cores) = spawning_op.resource_constraints.max_cpu_cores {
            if cpu_cores == 0 {
                return Err(BearDogError::invalid_input("CPU cores must be positive"));
            }
        }
        
        if let Some(memory_gb) = spawning_op.resource_constraints.max_memory_gb {
            if memory_gb <= 0.0 {
                return Err(BearDogError::invalid_input("Memory must be positive"));
            }
        }
        
        debug!("✅ Validated spawning requirements for {}", spawn_id);
        Ok(())
    }
    
    /// Perform genetic recombination
    async fn perform_genetic_recombination(&self, spawn_id: Uuid) -> BearDogResult<GeneticBlueprint> {
        let spawning_op = {
            let active_spawns = self.active_spawns.read().await;
            active_spawns.get(&spawn_id).cloned()
                .ok_or_else(|| BearDogError::internal("Spawning operation not found"))?
        };
        
        let mut blueprint = GeneticBlueprint::new(1);
        
        // Extract genetic traits from parent nodes
        for parent_node in &spawning_op.parent_nodes {
            let mut contribution = ParentGeneticContribution::new(
                parent_node.ecosystem_id.clone(),
                parent_node.node_id.clone(),
                1.0 / spawning_op.parent_nodes.len() as f64,
            );
            
            // Extract traits from parent capabilities
            let traits = self.extract_genetic_traits(&parent_node.capabilities).await?;
            for trait_item in traits {
                contribution.add_inherited_trait(trait_item.clone());
                blueprint.add_hybrid_trait(trait_item);
            }
            
            blueprint.add_parent_contribution(contribution);
        }
        
        // Apply genetic algorithm
        blueprint.apply_mutation(self.genetic_config.mutation_rate);
        blueprint.calculate_fitness(&spawning_op.target_capabilities);
        
        debug!("🧬 Performed genetic recombination for {}", spawn_id);
        Ok(blueprint)
    }
    
    /// Extract genetic traits from security capabilities
    async fn extract_genetic_traits(&self, capabilities: &[SecurityCapability]) -> BearDogResult<Vec<GeneticTrait>> {
        let mut traits = Vec::new();
        
        for capability in capabilities {
            let capability_traits = GeneticTrait::from_security_capability(capability);
            traits.extend(capability_traits);
        }
        
        Ok(traits)
    }
    
    /// Merge ecosystem capabilities
    async fn merge_ecosystem_capabilities(
        &self,
        spawn_id: Uuid,
        blueprint: GeneticBlueprint,
    ) -> BearDogResult<Vec<HybridCapability>> {
        let spawning_op = {
            let active_spawns = self.active_spawns.read().await;
            active_spawns.get(&spawn_id).cloned()
                .ok_or_else(|| BearDogError::internal("Spawning operation not found"))?
        };
        
        let mut merged_capabilities = Vec::new();
        
        // Add requested capabilities that are supported by the blueprint
        for capability in &spawning_op.target_capabilities {
            if blueprint.supports_capability(capability) {
                merged_capabilities.push(capability.clone());
            }
        }
        
        // Discover emergent capabilities
        let emergent_capabilities = self.discover_emergent_capabilities(&blueprint).await?;
        merged_capabilities.extend(emergent_capabilities);
        
        debug!("🔗 Merged {} capabilities for {}", merged_capabilities.len(), spawn_id);
        Ok(merged_capabilities)
    }
    
    /// Discover emergent capabilities from genetic blueprint
    async fn discover_emergent_capabilities(&self, blueprint: &GeneticBlueprint) -> BearDogResult<Vec<HybridCapability>> {
        let mut emergent_capabilities = Vec::new();
        
        // Check for emergent capabilities based on trait combinations
        let high_traits: Vec<_> = blueprint.hybrid_traits.iter()
            .filter(|t| t.value > 0.8)
            .collect();
        
        if high_traits.len() >= 3 {
            // High diversity suggests multi-node authentication capability
            if high_traits.iter().any(|t| t.category == "identity") &&
               high_traits.iter().any(|t| t.category == "security") {
                emergent_capabilities.push(HybridCapability::MultiNodeAuthentication);
            }
            
            // Strong governance traits suggest distributed compliance
            if high_traits.iter().filter(|t| t.category == "governance").count() >= 2 {
                emergent_capabilities.push(HybridCapability::DistributedCompliance);
            }
            
            // High monitoring + orchestration suggests cross-node threat detection
            if high_traits.iter().any(|t| t.category == "monitoring") &&
               high_traits.iter().any(|t| t.category == "orchestration") {
                emergent_capabilities.push(HybridCapability::CrossNodeThreatDetection);
            }
        }
        
        Ok(emergent_capabilities)
    }
    
    /// Configure hybrid security
    async fn configure_hybrid_security(&self, spawn_id: Uuid) -> BearDogResult<SecurityContext> {
        let spawning_op = {
            let active_spawns = self.active_spawns.read().await;
            active_spawns.get(&spawn_id).cloned()
                .ok_or_else(|| BearDogError::internal("Spawning operation not found"))?
        };
        
        // Create enhanced security context for hybrid node
        let mut security_context = SecurityContext::default();
        
        // Apply highest security level from requirements
        security_context.security_level = spawning_op.security_requirements.minimum_security_level;
        
        // Generate unique identifiers for hybrid node
        security_context.user_id = format!("hybrid-{}", Uuid::new_v4().simple());
        security_context.device_id = format!("hybrid-device-{}", spawn_id.simple());
        
        // Create genetic lineage information
        security_context.genetic_lineage = Some(GeneticLineage {
            generation: 1,
            parent_nodes: spawning_op.parent_nodes.iter().map(|p| p.node_id.clone()).collect(),
            genetic_signature: format!("hybrid-{}", spawn_id.simple()),
            lineage_proof: vec![], // TODO: Generate cryptographic proof
            spawning_timestamp: chrono::Utc::now(),
        });
        
        debug!("🔐 Configured hybrid security context for {}", spawn_id);
        Ok(security_context)
    }
    
    /// Allocate resources for the hybrid node
    async fn allocate_hybrid_resources(&self, spawn_id: Uuid) -> BearDogResult<ResourceAllocation> {
        let spawning_op = {
            let active_spawns = self.active_spawns.read().await;
            active_spawns.get(&spawn_id).cloned()
                .ok_or_else(|| BearDogError::internal("Spawning operation not found"))?
        };
        
        // Calculate resource requirements based on target capabilities
        let cpu_cores = spawning_op.resource_constraints.max_cpu_cores.unwrap_or(4);
        let memory_gb = spawning_op.resource_constraints.max_memory_gb.unwrap_or(8.0);
        let storage_gb = spawning_op.resource_constraints.max_storage_gb.unwrap_or(100.0);
        let network_bandwidth_mbps = spawning_op.resource_constraints.max_network_bandwidth_mbps.unwrap_or(1000.0);
        
        let resource_allocation = ResourceAllocation {
            cpu_cores,
            memory_gb,
            storage_gb,
            network_bandwidth_mbps,
            allocated_at: chrono::Utc::now(),
            allocation_id: Uuid::new_v4(),
        };
        
        debug!("💾 Allocated resources for {}: {} CPU cores, {} GB memory", spawn_id, cpu_cores, memory_gb);
        Ok(resource_allocation)
    }
    
    /// Initialize the hybrid node
    async fn initialize_hybrid_node(
        &self,
        spawn_id: Uuid,
        capabilities: Vec<HybridCapability>,
        security_context: SecurityContext,
        resource_allocation: ResourceAllocation,
    ) -> BearDogResult<String> {
        let node_id = format!("hybrid-{}", spawn_id.simple());
        
        // TODO: Implement actual node initialization logic
        // This would involve:
        // 1. Creating the hybrid node process/container
        // 2. Configuring the node with merged capabilities
        // 3. Setting up security and resource constraints
        // 4. Establishing communication channels
        
        debug!("🚀 Initialized hybrid node: {}", node_id);
        Ok(node_id)
    }
    
    /// Verify hybrid node health
    async fn verify_hybrid_node_health(&self, node_id: &str) -> BearDogResult<()> {
        // TODO: Implement actual health verification
        // This would check:
        // 1. Node responsiveness
        // 2. Capability functionality
        // 3. Security configuration
        // 4. Resource utilization
        
        debug!("🏥 Verified health of hybrid node: {}", node_id);
        Ok(())
    }
    
    /// Create the final hybrid node instance
    async fn create_hybrid_node_instance(&self, spawn_id: Uuid, node_id: String) -> BearDogResult<HybridNode> {
        let spawning_op = {
            let active_spawns = self.active_spawns.read().await;
            active_spawns.get(&spawn_id).cloned()
                .ok_or_else(|| BearDogError::internal("Spawning operation not found"))?
        };
        
        let hybrid_node = HybridNode {
            node_id,
            ecosystem_ids: spawning_op.parent_nodes.iter().map(|p| p.ecosystem_id.clone()).collect(),
            hybrid_capabilities: spawning_op.target_capabilities,
            security_context: SecurityContext::default(), // This would be the configured security context
            endpoints: SecurityEndpoints::default(),
            resource_allocation: ResourceAllocation {
                cpu_cores: 4,
                memory_gb: 8.0,
                storage_gb: 100.0,
                network_bandwidth_mbps: 1000.0,
                allocated_at: chrono::Utc::now(),
                allocation_id: Uuid::new_v4(),
            },
            genetic_lineage: GeneticLineage {
                generation: 1,
                parent_nodes: spawning_op.parent_nodes.iter().map(|p| p.node_id.clone()).collect(),
                genetic_signature: format!("hybrid-{}", spawn_id.simple()),
                lineage_proof: vec![],
                spawning_timestamp: chrono::Utc::now(),
            },
            created_at: chrono::Utc::now(),
        };
        
        Ok(hybrid_node)
    }
    
    /// Update spawning operation status
    async fn update_spawning_status(
        &self,
        spawn_id: Uuid,
        status: SpawningStatus,
        stage: SpawningStage,
        progress: f64,
        error: Option<String>,
    ) {
        let mut active_spawns = self.active_spawns.write().await;
        if let Some(spawning_op) = active_spawns.get_mut(&spawn_id) {
            spawning_op.update_status(status);
            spawning_op.update_progress(stage, progress);
            if let Some(error_msg) = error {
                spawning_op.set_error(error_msg);
            }
        }
    }
} 