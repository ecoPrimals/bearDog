

use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

use beardog_config::{BearDogConfig, AppConfig};
use beardog_core::BearDogCore;
use beardog_tunnel::tunnel::hsm::{
    HsmManager, HsmManagerConfig, SecurityRequirements, SecurityLevel,
    GenerateKeyRequest, KeyMetadata, KeyUsage, KeyAlgorithm,
};
use beardog_genetics::genetics::{GeneticsAPI, SpawnRequest, SpawnPurpose, ResourceLimits};
use beardog_errors::BearDogError;

#[tokio::main]
async fn main() -> Result<(), BearDogError> {

    tracing_subscriber::fmt::init();
    
    info!("🏠 Starting Distributed BearDog Demo");
    info!("📱 HSM for human identity, local instances for routine ops");

    let local_instances = initialize_local_instances().await?;

    demo_operation_routing(&local_instances).await?;

    demo_hsm_failover(&local_instances).await?;

    demo_offline_genetic_spawning(&local_instances).await?;

    demo_local_coordination(&local_instances).await?;
    
    Ok(())
}

async fn initialize_local_instances() -> Result<Vec<Arc<BearDogCore, BearDogError>>> {
    info!("🔧 Initializing local BearDog instances...");
    
    let mut instances = Vec::new();

    let tower_config = create_local_config("tower", "192.168.1.10:8080").await?;
    let tower_instance = Arc::new(BearDogCore::new(tower_config).await?);
    tower_instance.start().await?;
    instances.push(tower_instance);

    let laptop_config = create_local_config("laptop", "192.168.1.11:8080").await?;
    let laptop_instance = Arc::new(BearDogCore::new(laptop_config).await?);
    laptop_instance.start().await?;
    instances.push(laptop_instance);

    let server_config = create_local_config("server", "192.168.1.12:8080").await?;
    let server_instance = Arc::new(BearDogCore::new(server_config).await?);
    server_instance.start().await?;
    instances.push(server_instance);
    
    info!("✅ {} local instances initialized", instances.len());
    Ok(instances)
}

async fn create_local_config(instance_name: &str, bind_address: &str) -> Result<BearDogConfig, BearDogError> {
    let mut config = BearDogConfig::default();

    config.app.standalone_mode = true;
    config.app.environment = "distributed".to_string();
    config.app.name = format_args!("BearDog-{}", instance_name).to_string();

    config.network.http.bind_address = bind_address.to_string();
    config.network.node_communication.enabled = true;

    Ok(config)
}

async fn demo_operation_routing(instances: &[Arc<BearDogCore>]) -> Result<(), BearDogError> {
    info!("🔀 Demonstrating operation routing...");
    
    let tower_instance = &instances[0];

    let hsm_config = HsmManagerConfig::default();
    let hsm_manager = Arc::new(HsmManager::with_config(hsm_config).await?);

    info!("📄 Scenario 1: Routine file encryption (local software HSM)");
    let file_encryption_req = SecurityRequirements::new(SecurityLevel::Standard);
    
    match hsm_manager.get_best_provider(&file_encryption_req).await {
        Ok(provider) => {
            info!("✅ Routing to local software HSM for file encryption");

            let key_request = GenerateKeyRequest {
                key_id: "file_encryption_key".to_string(),
                algorithm: KeyAlgorithm::Aes256,
                usage: KeyUsage::Encryption,
                metadata: KeyMetadata::default(),
            };
            
            match provider.generate_key(key_request).await {
                Ok(key) => info!("🔑 Generated file encryption key: {}", key.key_id),
                Err(e) => warn!("Failed to generate key: {}", e),
            }
        }
        Err(e) => warn!("No suitable provider for file encryption: {}", e),
    }

    info!("👤 Scenario 2: Human identity operation (mobile HSM preferred)");
    let identity_req = SecurityRequirements::new(SecurityLevel::High);
    
    match hsm_manager.get_best_provider(&identity_req).await {
        Ok(provider) => {
            info!("✅ Would route to mobile HSM for human identity (if available)");
            info!("💡 Fallback to software HSM with user notification");
        }
        Err(e) => {
            warn!("Mobile HSM not available: {}", e);
            info!("🔄 Falling back to software HSM for identity operations");
        }
    }
    
    Ok(())
}

async fn demo_hsm_failover(instances: &[Arc<BearDogCore>]) -> Result<(), BearDogError> {
    info!("🔄 Demonstrating HSM failover...");
    
    let tower_instance = &instances[0];

    info!("📱 Simulating mobile HSM unavailable...");

    info!("✅ Operations continue with local software HSM");
    info!("🔐 Security level: Standard (graceful degradation)");
    info!("⚡ Latency: Low (local operations)");

    sleep(Duration::from_secs(2)).await;
    info!("📱 Mobile HSM back online - upgrading security level");
    info!("🔐 Security level: High (enhanced operations)");
    
    Ok(())
}

async fn demo_offline_genetic_spawning(instances: &[Arc<BearDogCore>]) -> Result<(), BearDogError> {
    info!("🧬 Demonstrating offline genetic spawning...");
    
    let tower_instance = &instances[0];

    let genetics_store = Arc::new(beardog_genetics::genetics::types::InMemoryGeneticsStore::new());
    let genetics_config = beardog_genetics::genetics::GeneticsConfig {
        base_mutation_rate: 0.05,
        max_genetic_diversity: 0.8,
        min_security_threshold: 0.7,
        capability_inheritance_weight: 0.8,
        trait_blending_factor: 0.6,
        enable_directed_evolution: true,
    };
    
    let genetics_api = GeneticsAPI::new(genetics_store, genetics_config);

    let spawn_request = SpawnRequest {
        request_id: uuid::Uuid::new_v4().to_string(),
        requesting_parent: "tower_instance".to_string(),
        co_parents: vec!["laptop_instance".to_string()],
        purpose: SpawnPurpose::LocalProcessing,
        resource_requirements: ResourceLimits {
            max_cpu_percent: 50.0,
            max_memory_mb: 2048,
            max_storage_gb: 10,
            max_network_mbps: 100,
            allowed_jurisdictions: vec!["US".to_string()],
            temporal_windows: vec![],
        },
        workflow_type: beardog_genetics::genetics::BearDogWorkflowType::AutomatedConsensus {
            participating_nodes: vec!["tower_instance".to_string(), "laptop_instance".to_string()],
            consensus_threshold: 0.67,
            max_decision_time: chrono::Duration::minutes(5),
        },
        created_at: chrono::Utc::now(),
        expires_at: chrono::Utc::now() + chrono::Duration::hours(24),
        metadata: std::collections::HashMap::with_capacity(16),
    };

    match genetics_api.spawn_node(spawn_request).await {
        Ok(result) => {
            info!("✅ Genetic spawning successful without HSM");
            info!("🆔 New node ID: {:?}", result.child_node_id);
            info!("🔍 Decision reason: {}", result.decision_reason);
        }
        Err(e) => error!("Genetic spawning failed: {}", e),
    }
    
    Ok(())
}

async fn demo_local_coordination(instances: &[Arc<BearDogCore>]) -> Result<(), BearDogError> {
    info!("🤝 Demonstrating local instance coordination...");

    info!("🏠 Tower: Processing heavy computation locally");

    info!("💻 Laptop: Providing quick access to encrypted data");

    info!("🖥️ Server: Managing backup and storage operations");

    info!("🌐 Local network benefits:");
    info!("  ⚡ Low latency: <1ms between instances");
    info!("  🔒 Local security: No internet dependency");
    info!("  📈 High bandwidth: Gigabit local network");
    info!("  🔄 Auto-sync: Background coordination");

    info!("⚖️ Load balancing across local instances:");
    for (i, instance) in instances.iter().enumerate() {
        let health = instance.health_check().await?;
        info!("  Instance {}: {:?}", i + 1, health.status);
    }
    
    Ok(())
}

async fn demo_performance_comparison() -> Result<(), BearDogError> {
    info!("📊 Performance comparison:");

    info!("🏠 Local Software HSM:");
    info!("  ⚡ Latency: 0.1ms - 1ms");
    info!("  🔄 Throughput: 10,000+ ops/sec");
    info!("  📈 Availability: 99.9%");

    info!("📱 Mobile HSM:");
    info!("  ⚡ Latency: 50ms - 200ms");
    info!("  🔄 Throughput: 100-500 ops/sec");
    info!("  📈 Availability: 95% (user dependent)");

    info!("🎯 Optimal routing:");
    info!("  👤 Human identity → Mobile HSM");
    info!("  📄 File encryption → Local software HSM");
    info!("  🧬 Genetic spawning → Local software HSM");
    info!("  💾 Backup operations → Local software HSM");
    
    Ok(())
} 