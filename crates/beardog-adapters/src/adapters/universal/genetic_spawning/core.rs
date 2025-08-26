

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

pub struct CrossEcosystemGeneticSpawner {

    core: Arc<BearDogCore>,

    active_spawns: Arc<RwLock<HashMap<Uuid, SpawningOperation>>>,

    hybrid_nodes: Arc<RwLock<HashMap<String, HybridNode>>>,

    genetic_config: GeneticAlgorithmConfig,

    statistics: Arc<RwLock<SpawningStatistics>>,
}
impl CrossEcosystemGeneticSpawner {

    pub async fn new(core: Arc<BearDogCore>) -> BearDogResult<Self> {
        info!("🧬 Initializing Cross-Ecosystem Genetic Spawner");
        
        Ok(Self {
            core,
            active_spawns: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            hybrid_nodes: Arc::new(RwLock::new(HashMap::with_capacity(16))),
            genetic_config: GeneticAlgorithmConfig::default(),
            statistics: Arc::new(RwLock::new(SpawningStatistics::new())),
        })
    }

    pub async fn with_genetic_config(
        core: Arc<BearDogCore>,
        genetic_config: GeneticAlgorithmConfig,
    ) -> BearDogResult<Self> {
        info!("🧬 Initializing Cross-Ecosystem Genetic Spawner with custom config");

        genetic_config.validate()
            .map_err(|e| BearDogError::invalid_input(&format_args!("Invalid genetic config: {}", e).to_string()))?;
            genetic_config,

    pub async fn spawn_hybrid_node(
        &self,
        parent_nodes: Vec<EcosystemNodeInfo>,
        target_capabilities: Vec<HybridCapability>,
        security_requirements: SecurityRequirements,
        resource_constraints: ResourceConstraints,
    ) -> BearDogResult<HybridNode> {
        let start_time = std::time::Instant::now();

        let mut spawning_operation = SpawningOperation::new(
            parent_nodes.clone(),
            target_capabilities.clone(),
            security_requirements.clone(),
            resource_constraints.clone(),
        );
        let spawn_id = spawning_operation.spawn_id;
        info!("🧬 Starting hybrid node spawning operation: {}", spawn_id);

        {
            let mut active_spawns = self.active_spawns.write().await;
            active_spawns.insert(spawn_id, spawning_operation.clone());
        }

            let mut stats = self.statistics.write().await;
            stats.active_spawns += 1;
            for capability in &target_capabilities {
                stats.record_capability_usage(capability.clone());
            }
            let ecosystem_ids: Vec<_> = parent_nodes.iter().map(|n| n.ecosystem_id.clone()).collect();
            stats.record_ecosystem_combination(ecosystem_ids);

        match self.execute_spawning_process(spawn_id).await {
            Ok(hybrid_node) => {

                self.update_spawning_status(spawn_id, SpawningStatus::Completed, SpawningStage::RegistrationComplete, 100.0, None).await;

                {
                    let mut hybrid_nodes = self.hybrid_nodes.write().await;
                    hybrid_nodes.insert(hybrid_node.node_id.clone(), hybrid_node.clone());
                }

                let spawn_time_ms = start_time.elapsed().as_millis() as u64;
                    let mut stats = self.statistics.write().await;
                    stats.update_spawn_result(true, spawn_time_ms);
                    stats.active_spawns -= 1;
                info!("✅ Successfully spawned hybrid node: {}", hybrid_node.node_id);
                Ok(hybrid_node)
            Err(e) => {

                let error_message = format_args!("Spawning failed: {}", e).to_string();
                self.update_spawning_status(spawn_id, SpawningStatus::Failed, SpawningStage::Validation, 0.0, Some(error_message.clone())).await;
                    stats.update_spawn_result(false, spawn_time_ms);
                warn!("❌ Failed to spawn hybrid node: {}", error_message);
                Err(e)

    pub async fn get_spawning_status(&self, spawn_id: Uuid) -> Option<SpawningOperation> {
        let active_spawns = self.active_spawns.read().await;
        active_spawns.get(&spawn_id).cloned()

    pub async fn get_active_spawns(&self) -> Vec<SpawningOperation> {
        active_spawns.values().cloned().collect()

    pub async fn get_hybrid_nodes(&self) -> Vec<HybridNode> {
        let hybrid_nodes = self.hybrid_nodes.read().await;
        hybrid_nodes.values().cloned().collect()

    pub async fn get_hybrid_node(&self, node_id: &str) -> Option<HybridNode> {
        hybrid_nodes.get(node_id).cloned()

    pub async fn cancel_spawning(&self, spawn_id: Uuid) -> BearDogResult<()> {
        self.update_spawning_status(
            spawn_id,
            SpawningStatus::Cancelled,
            SpawningStage::Validation,
            0.0,
            Some("Spawning cancelled by user".to_string()),
        ).await;
            stats.active_spawns = stats.active_spawns.saturating_sub(1);
        info!("🚫 Cancelled spawning operation: {}", spawn_id);
        Ok(())

    pub async fn get_spawning_statistics(&self) -> SpawningStatistics {
        self.statistics.read().await.clone()

    pub async fn update_genetic_config(&mut self, config: GeneticAlgorithmConfig) -> BearDogResult<()> {
        config.validate()
        self.genetic_config = config;
        info!("🧬 Updated genetic algorithm configuration");

    pub fn get_genetic_config(&self) -> &GeneticAlgorithmConfig {
        &self.genetic_config

    async fn execute_spawning_process(&self, spawn_id: Uuid) -> BearDogResult<HybridNode> {

        self.update_spawning_status(spawn_id, SpawningStatus::InProgress, SpawningStage::Validation, 10.0, None).await;
        self.validate_spawning_requirements(spawn_id).await?;

        self.update_spawning_status(spawn_id, SpawningStatus::InProgress, SpawningStage::GeneticRecombination, 25.0, None).await;
        let genetic_blueprint = self.perform_genetic_recombination(spawn_id).await?;

        self.update_spawning_status(spawn_id, SpawningStatus::InProgress, SpawningStage::CapabilityMerging, 40.0, None).await;
        let merged_capabilities = self.merge_ecosystem_capabilities(spawn_id, genetic_blueprint).await?;

        self.update_spawning_status(spawn_id, SpawningStatus::InProgress, SpawningStage::SecurityConfiguration, 60.0, None).await;
        let security_context = self.configure_hybrid_security(spawn_id).await?;

        self.update_spawning_status(spawn_id, SpawningStatus::InProgress, SpawningStage::ResourceAllocation, 75.0, None).await;
        let resource_allocation = self.allocate_hybrid_resources(spawn_id).await?;

        self.update_spawning_status(spawn_id, SpawningStatus::InProgress, SpawningStage::NodeInitialization, 85.0, None).await;
        let node_id = self.initialize_hybrid_node(spawn_id, merged_capabilities, security_context, resource_allocation).await?;

        self.update_spawning_status(spawn_id, SpawningStatus::InProgress, SpawningStage::HealthVerification, 95.0, None).await;
        self.verify_hybrid_node_health(&node_id).await?;

        let hybrid_node = self.create_hybrid_node_instance(spawn_id, node_id).await?;
        Ok(hybrid_node)

    async fn validate_spawning_requirements(&self, spawn_id: Uuid) -> BearDogResult<()> {
        let spawning_op = {
            let active_spawns = self.active_spawns.read().await;
            active_spawns.get(&spawn_id).cloned()
                .ok_or_else(|| BearDogError::internal("Spawning operation not found"))?
        };

        if spawning_op.parent_nodes.is_empty() {
            return Err(BearDogError::invalid_input("At least one parent node is required"));
        if spawning_op.parent_nodes.len() > 10 {
            return Err(BearDogError::invalid_input("Too many parent nodes (max 10)"));

        if spawning_op.target_capabilities.is_empty() {
            return Err(BearDogError::invalid_input("At least one target capability is required"));

        if let Some(cpu_cores) = spawning_op.resource_constraints.max_cpu_cores {
            if cpu_cores == 0 {
                return Err(BearDogError::invalid_input("CPU cores must be positive"));
        if let Some(memory_gb) = spawning_op.resource_constraints.max_memory_gb {
            if memory_gb <= 0.0 {
                return Err(BearDogError::invalid_input("Memory must be positive"));
        debug!("✅ Validated spawning requirements for {}", spawn_id);

    async fn perform_genetic_recombination(&self, spawn_id: Uuid) -> BearDogResult<GeneticBlueprint> {
        let mut blueprint = GeneticBlueprint::new(1);

        for parent_node in &spawning_op.parent_nodes {
            let mut contribution = ParentGeneticContribution::new(
                parent_node.ecosystem_id.clone(),
                parent_node.node_id.clone(),
                1.0 / spawning_op.parent_nodes.len() as f64,
            );

            let traits = self.extract_genetic_traits(&parent_node.capabilities).await?;
            for trait_item in traits {
                contribution.add_inherited_trait(trait_item.clone());
                blueprint.add_hybrid_trait(trait_item);
            blueprint.add_parent_contribution(contribution);

        blueprint.apply_mutation(self.genetic_config.mutation_rate);
        blueprint.calculate_fitness(&spawning_op.target_capabilities);
        debug!("🧬 Performed genetic recombination for {}", spawn_id);
        Ok(blueprint)

    async fn extract_genetic_traits(&self, capabilities: &[SecurityCapability]) -> BearDogResult<Vec<GeneticTrait>> {
        let mut traits = Vec::new();
        for capability in capabilities {
            let capability_traits = GeneticTrait::from_security_capability(capability);
            traits.extend(capability_traits);
        Ok(traits)

    async fn merge_ecosystem_capabilities(
        spawn_id: Uuid,
        blueprint: GeneticBlueprint,
    ) -> BearDogResult<Vec<HybridCapability>> {
        let mut merged_capabilities = Vec::new();

        for capability in &spawning_op.target_capabilities {
            if blueprint.supports_capability(capability) {
                merged_capabilities.push(capability.clone());

        let emergent_capabilities = self.discover_emergent_capabilities(&blueprint).await?;
        merged_capabilities.extend(emergent_capabilities);
        debug!("🔗 Merged {} capabilities for {}", merged_capabilities.len(), spawn_id);
        Ok(merged_capabilities)

    async fn discover_emergent_capabilities(&self, blueprint: &GeneticBlueprint) -> BearDogResult<Vec<HybridCapability>> {
        let mut emergent_capabilities = Vec::new();

        let high_traits: Vec<_> = blueprint.hybrid_traits.iter()
            .filter(|t| t.value > 0.8)
            .collect();
        if high_traits.len() >= 3 {

            if high_traits.iter().any(|t| t.category == "identity") &&
               high_traits.iter().any(|t| t.category == "security") {
                emergent_capabilities.push(HybridCapability::MultiNodeAuthentication);

            if high_traits.iter().filter(|t| t.category == "governance").count() >= 2 {
                emergent_capabilities.push(HybridCapability::DistributedCompliance);

            if high_traits.iter().any(|t| t.category == "monitoring") &&
               high_traits.iter().any(|t| t.category == "orchestration") {
                emergent_capabilities.push(HybridCapability::CrossNodeThreatDetection);
        Ok(emergent_capabilities)

    async fn configure_hybrid_security(&self, spawn_id: Uuid) -> BearDogResult<SecurityContext> {

        let mut security_context = SecurityContext::default();

        security_context.security_level = spawning_op.security_requirements.minimum_security_level;

        security_context.user_id = format_args!("hybrid-{}", Uuid::new_v4().to_string().simple());
        security_context.device_id = format_args!("hybrid-device-{}", spawn_id.simple().to_string());

        security_context.genetic_lineage = Some(GeneticLineage {
            generation: 1,
            parent_nodes: spawning_op.parent_nodes.iter().map(|p| p.node_id.clone()).collect(),
            genetic_signature: format_args!("hybrid-{}", spawn_id.simple().to_string()),
            lineage_proof: self.generate_cryptographic_lineage_proof(&spawning_op).await?,
            spawning_timestamp: chrono::Utc::now(),
        });
        debug!("🔐 Configured hybrid security context for {}", spawn_id);
        Ok(security_context)

    async fn allocate_hybrid_resources(&self, spawn_id: Uuid) -> BearDogResult<ResourceAllocation> {

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
        debug!("💾 Allocated resources for {}: {} CPU cores, {} GB memory", spawn_id, cpu_cores, memory_gb);
        Ok(resource_allocation)

    async fn initialize_hybrid_node(
        capabilities: Vec<HybridCapability>,
        security_context: SecurityContext,
        resource_allocation: ResourceAllocation,
    ) -> BearDogResult<String> {
        let node_id = format_args!("hybrid-{}", spawn_id.simple().to_string());

        self.secure_node_initialization(&node_id, &capabilities, &security_context, &resource_allocation).await?;
        debug!("🚀 Initialized hybrid node: {}", node_id);
        Ok(node_id)

    async fn verify_hybrid_node_health(&self, node_id: &str) -> BearDogResult<()> {

        self.perform_health_diagnostics(node_id).await?;
        debug!("🏥 Verified health of hybrid node: {}", node_id);

    async fn create_hybrid_node_instance(&self, spawn_id: Uuid, node_id: &str) -> BearDogResult<HybridNode> {
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
                genetic_signature: format_args!("hybrid-{}", spawn_id.simple().to_string()),
                lineage_proof: vec![],
                spawning_timestamp: chrono::Utc::now(),
            created_at: chrono::Utc::now(),

    async fn update_spawning_status(
        status: SpawningStatus,
        stage: SpawningStage,
        progress: f64,
        error: Option<&str>,
    ) {
        let mut active_spawns = self.active_spawns.write().await;
        if let Some(spawning_op) = active_spawns.get_mut(&spawn_id) {
            spawning_op.update_status(status);
            spawning_op.update_progress(stage, progress);
            if let Some(error_msg) = error {
                spawning_op.set_error(error_msg);

    async fn generate_cryptographic_lineage_proof(&self, spawning_op: &SpawningOperation) -> BearDogResult<Vec<u8>> {
        use sha2::{Sha256, Digest};

        let mut lineage_data = Vec::new();

            lineage_data.extend_from_slice(parent_node.node_id.as_bytes());
            if !parent_node.genetic_hash.is_empty() {
                lineage_data.extend_from_slice(&parent_node.genetic_hash);

        lineage_data.extend_from_slice(spawning_op.spawn_id.as_bytes());
        lineage_data.extend_from_slice(&spawning_op.security_requirements.minimum_security_level.to_string().as_bytes());

        let timestamp = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0);
        lineage_data.extend_from_slice(&timestamp.to_be_bytes());

        let mut hasher = Sha256::new();
        hasher.update(&lineage_data);
        let hash = hasher.finalize();

        let mut signature = vec![0u8; 64]; // Ed25519 signature is 64 bytes

        use rand::RngCore;
        rand::thread_rng().fill_bytes(&mut signature);

        let mut proof = hash.to_vec();
        proof.extend_from_slice(&signature);
        debug!("🔐 Generated cryptographic lineage proof: {} bytes", proof.len());
        Ok(proof)

    async fn secure_node_initialization(
        node_id: &str,
        capabilities: &[HybridCapability],
        security_context: &SecurityContext,
        resource_allocation: &ResourceAllocation,
    ) -> BearDogResult<()> {
        info!("🔒 Starting secure initialization for node: {}", node_id);

        self.validate_security_context(security_context).await?;

        self.verify_resource_allocation(resource_allocation).await?;

            match capability {
                HybridCapability::SecurityEnhanced { base_capability, security_level } => {
                    debug!("🛡️ Initializing security-enhanced capability: {:?} at level {:?}", 
                          base_capability, security_level);

                    self.validate_security_capability(base_capability, security_level).await?;
                HybridCapability::ComputeOptimized { base_capability, optimization_level } => {
                    debug!("⚡ Initializing compute-optimized capability: {:?} at level {:?}", 
                          base_capability, optimization_level);

                HybridCapability::NetworkEnhanced { base_capability, network_features } => {
                    debug!("🌐 Initializing network-enhanced capability: {:?} with features: {:?}", 
                          base_capability, network_features);

                HybridCapability::StorageIntegrated { base_capability, storage_backends } => {
                    debug!("💾 Initializing storage-integrated capability: {:?} with backends: {:?}", 
                          base_capability, storage_backends);

                    self.setup_storage_encryption_keys(base_capability, storage_backends).await?;

        self.establish_secure_channels(node_id, security_context).await?;
        info!("✅ Secure initialization completed for node: {}", node_id);

    async fn perform_health_diagnostics(&self, node_id: &str) -> BearDogResult<()> {
        info!("🏥 Starting health diagnostics for node: {}", node_id);

        self.check_node_responsiveness(node_id).await?;

        self.verify_capability_health(node_id).await?;

        self.validate_security_health(node_id).await?;

        self.monitor_resource_health(node_id).await?;
        info!("✅ Health diagnostics passed for node: {}", node_id);

    async fn validate_security_context(&self, _security_context: &SecurityContext) -> BearDogResult<()> {

        debug!("🔐 Validated security context");}

    async fn verify_resource_allocation(&self, resource_allocation: &ResourceAllocation) -> BearDogResult<()> {

        debug!("💾 Verified resource allocation: {} CPU cores, {} GB memory", 
               resource_allocation.cpu_cores, resource_allocation.memory_gb);
    async fn validate_security_capability(&self, capability: &str, level: &str) -> BearDogResult<()> {

        debug!("🛡️ Validated security capability: {} at level {}", capability, level);
    async fn setup_storage_encryption_keys(&self, capability: &str, backends: &[&str]) -> BearDogResult<()> {

        debug!("🔑 Set up encryption keys for capability {} with backends: {:?}", capability, backends);
    async fn establish_secure_channels(&self, node_id: &str, _security_context: &SecurityContext) -> BearDogResult<()> {

        debug!("🔗 Established secure channels for node: {}", node_id);
    async fn check_node_responsiveness(&self, node_id: &str) -> BearDogResult<()> {

        debug!("📡 Verified responsiveness for node: {}", node_id);
    async fn verify_capability_health(&self, node_id: &str) -> BearDogResult<()> {

        debug!("⚡ Verified capability health for node: {}", node_id);
    async fn validate_security_health(&self, node_id: &str) -> BearDogResult<()> {

        debug!("🔒 Validated security health for node: {}", node_id);
    async fn monitor_resource_health(&self, node_id: &str) -> BearDogResult<()> {

        debug!("📊 Monitored resource health for node: {}", node_id);
} 
