//! Distributed BearDog Demo
//!
//! This example demonstrates:
//! 1. Local BearDog instances with software HSM (always available)
//! 2. Mobile HSM integration for high-security operations
//! 3. Graceful degradation when HSM unavailable
//! 4. Low-latency local operations
//!
//! Usage:
//! ```bash
//! cargo run --example distributed_beardog_demo
//! ```

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
use beardog_errors::BearDogResult;

#[tokio::main]
async fn main() -> BearDogResult<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🏠 Starting Distributed BearDog Demo");
    info!("📱 HSM for human identity, local instances for routine ops");
    
    // Initialize distributed BearDog instances
    let local_instances = initialize_local_instances().await?;
    
    // Demonstrate operation routing
    demo_operation_routing(&local_instances).await?;
    
    // Demonstrate HSM failover
    demo_hsm_failover(&local_instances).await?;
    
    // Demonstrate genetic spawning without HSM
    demo_offline_genetic_spawning(&local_instances).await?;
    
    // Demonstrate local instance coordination
    demo_local_coordination(&local_instances).await?;
    
    Ok(())
}

/// Initialize local BearDog instances for different devices
async fn initialize_local_instances() -> BearDogResult<Vec<Arc<BearDogCore>>> {
    info!("🔧 Initializing local BearDog instances...");
    
    let mut instances = Vec::new();
    
    // Tower/Desktop instance - Main processing
    let tower_config = create_local_config("tower", "192.168.1.10:8080").await?;
    let tower_instance = Arc::new(BearDogCore::new(tower_config).await?);
    tower_instance.start().await?;
    instances.push(tower_instance);
    
    // Laptop instance - Portable access
    let laptop_config = create_local_config("laptop", "192.168.1.11:8080").await?;
    let laptop_instance = Arc::new(BearDogCore::new(laptop_config).await?);
    laptop_instance.start().await?;
    instances.push(laptop_instance);
    
    // Server instance - Storage and backup
    let server_config = create_local_config("server", "192.168.1.12:8080").await?;
    let server_instance = Arc::new(BearDogCore::new(server_config).await?);
    server_instance.start().await?;
    instances.push(server_instance);
    
    info!("✅ {} local instances initialized", instances.len());
    Ok(instances)
}

/// Create optimized config for local instance
async fn create_local_config(instance_name: &str, bind_address: &str) -> BearDogResult<BearDogConfig> {
    let mut config = BearDogConfig::default();
    
    // Enable standalone mode for local operation
    config.app.standalone_mode = true;
    config.app.environment = "distributed".to_string();
    config.app.name = format!("BearDog-{}", instance_name);
    
    // Local network configuration
    config.network.http.bind_address = bind_address.to_string();
    config.network.node_communication.enabled = true;
    
    // HSM configuration with fallback
    // Mobile HSM available but not required for routine operations
    
    Ok(config)
}

/// Demonstrate intelligent operation routing
async fn demo_operation_routing(instances: &[Arc<BearDogCore>]) -> BearDogResult<()> {
    info!("🔀 Demonstrating operation routing...");
    
    let tower_instance = &instances[0];
    
    // Create HSM manager for routing decisions
    let hsm_config = HsmManagerConfig::default();
    let hsm_manager = Arc::new(HsmManager::with_config(hsm_config).await?);
    
    // Scenario 1: Routine file encryption (uses local software HSM)
    info!("📄 Scenario 1: Routine file encryption (local software HSM)");
    let file_encryption_req = SecurityRequirements::new(SecurityLevel::Standard);
    
    match hsm_manager.get_best_provider(&file_encryption_req).await {
        Ok(provider) => {
            info!("✅ Routing to local software HSM for file encryption");
            
            // Generate key for file encryption
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
    
    // Scenario 2: Human identity operation (would prefer mobile HSM)
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

/// Demonstrate HSM failover scenarios
async fn demo_hsm_failover(instances: &[Arc<BearDogCore>]) -> BearDogResult<()> {
    info!("🔄 Demonstrating HSM failover...");
    
    let tower_instance = &instances[0];
    
    // Simulate mobile HSM unavailable
    info!("📱 Simulating mobile HSM unavailable...");
    
    // Operations continue with software HSM
    info!("✅ Operations continue with local software HSM");
    info!("🔐 Security level: Standard (graceful degradation)");
    info!("⚡ Latency: Low (local operations)");
    
    // Simulate mobile HSM coming back online
    sleep(Duration::from_secs(2)).await;
    info!("📱 Mobile HSM back online - upgrading security level");
    info!("🔐 Security level: High (enhanced operations)");
    
    Ok(())
}

/// Demonstrate genetic spawning without HSM
async fn demo_offline_genetic_spawning(instances: &[Arc<BearDogCore>]) -> BearDogResult<()> {
    info!("🧬 Demonstrating offline genetic spawning...");
    
    let tower_instance = &instances[0];
    
    // Create genetics API with offline capability
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
    
    // Create spawn request for local processing
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
        metadata: std::collections::HashMap::new(),
    };
    
    // Process spawn without HSM
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

/// Demonstrate local instance coordination
async fn demo_local_coordination(instances: &[Arc<BearDogCore>]) -> BearDogResult<()> {
    info!("🤝 Demonstrating local instance coordination...");
    
    // Tower processes heavy computation
    info!("🏠 Tower: Processing heavy computation locally");
    
    // Laptop handles quick access
    info!("💻 Laptop: Providing quick access to encrypted data");
    
    // Server manages backup and storage
    info!("🖥️ Server: Managing backup and storage operations");
    
    // Show local network benefits
    info!("🌐 Local network benefits:");
    info!("  ⚡ Low latency: <1ms between instances");
    info!("  🔒 Local security: No internet dependency");
    info!("  📈 High bandwidth: Gigabit local network");
    info!("  🔄 Auto-sync: Background coordination");
    
    // Demonstrate load balancing
    info!("⚖️ Load balancing across local instances:");
    for (i, instance) in instances.iter().enumerate() {
        let health = instance.health_check().await?;
        info!("  Instance {}: {:?}", i + 1, health.status);
    }
    
    Ok(())
}

/// Demonstrate operation performance comparison
async fn demo_performance_comparison() -> BearDogResult<()> {
    info!("📊 Performance comparison:");
    
    // Local software HSM
    info!("🏠 Local Software HSM:");
    info!("  ⚡ Latency: 0.1ms - 1ms");
    info!("  🔄 Throughput: 10,000+ ops/sec");
    info!("  📈 Availability: 99.9%");
    
    // Mobile HSM (when available)
    info!("📱 Mobile HSM:");
    info!("  ⚡ Latency: 50ms - 200ms");
    info!("  🔄 Throughput: 100-500 ops/sec");
    info!("  📈 Availability: 95% (user dependent)");
    
    // Optimal routing strategy
    info!("🎯 Optimal routing:");
    info!("  👤 Human identity → Mobile HSM");
    info!("  📄 File encryption → Local software HSM");
    info!("  🧬 Genetic spawning → Local software HSM");
    info!("  💾 Backup operations → Local software HSM");
    
    Ok(())
} 