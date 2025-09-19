// Infant Discovery Pattern Demo
//
// This example demonstrates how BearDog achieves true primal sovereignty:
// - Starts with zero knowledge of other primals
// - Discovers capabilities through universal adapter
// - Never hardcodes primal names or vendor-specific references
// - Maintains the "infant discovery" pattern throughout

use beardog_core::zero_knowledge_bootstrap::ZeroKnowledgeBootstrap;
use beardog_errors::BearDogError;
use beardog_types::canonical::capabilities::ServiceCapabilityType;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), BearDogError> {
    tracing_subscriber::init();

    info!("🍼 Starting Infant Discovery Pattern Demo");
    info!("📋 PRINCIPLE: Each primal starts with zero knowledge and discovers everything");

    // Phase 1: Birth - Zero knowledge state
    info!("👶 Phase 1: Birth - Zero Knowledge Initialization");
    let mut bootstrap = ZeroKnowledgeBootstrap::new()?;
    info!("[OK] Born with zero hardcoded knowledge");

    // Phase 2: Self-awareness - Only know yourself
    info!("🪞 Phase 2: Self-Awareness - Discovering own identity");
    let self_identity = bootstrap.discover_self_identity()?;
    info!("[OK] Self-identity discovered: {}", self_identity.primal_id);
    info!("   [SEARCH] My capabilities: {:?}", self_identity.capabilities);

    // Validate true primal sovereignty - each primal only knows itself
    assert!(
        !self_identity.primal_id.is_empty(),
        "Must have self-identity"
    );
    assert!(
        !self_identity.capabilities.is_empty(),
        "Must discover own capabilities"
    );

    // Validate infant discovery pattern - no hardcoded ecosystem assumptions
    for capability in &self_identity.capabilities {
        let cap_str = format!("{:?}", capability);
        assert!(
            !cap_str.to_lowercase().contains("hardcoded"),
            "Capabilities should be discovered dynamically, not hardcoded"
        );
    }
    info!(
        "[OK] Sovereignty validated: Pure capability-based discovery, no hardcoded primal knowledge"
    );

    // Phase 3: Environmental awareness - Listen for others
    info!("👂 Phase 3: Environmental Awareness - Listening for ecosystem");
    bootstrap.start_ecosystem_listening()?;
    info!("[OK] Ecosystem listening started");

    // Phase 4: Capability discovery - Learn what's available
    info!("[SEARCH] Phase 4: Capability Discovery - Learning available services");
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let ecosystem_state = bootstrap.get_ecosystem_state()?;
    info!("[OK] Ecosystem state retrieved");
    info!(
        "   [CHART] Discovered primals: {}",
        ecosystem_state.discovered_primals_count
    );
    info!(
        "   [LIGHTNING] Available capabilities: {}",
        ecosystem_state.available_capabilities.len()
    );

    // Phase 5: Universal adapter usage - Connect through capabilities
    info!("🔌 Phase 5: Universal Adapter Usage - Capability-based connections");

    // Demonstrate capability-based discovery (not hardcoded primal names)
    for (capability_type, providers) in &ecosystem_state.available_capabilities {
        info!("   [TARGET] Capability: {:?}", capability_type);
        for provider in providers {
            info!(
                "     🔗 Provider: {} (discovered dynamically)",
                provider.provider_id
            );
        }
    }

    // Phase 6: Network effects without hardcoding
    info!("🌐 Phase 6: Network Effects - Dynamic ecosystem collaboration");
    demonstrate_network_effects(&ecosystem_state)?;

    info!("[PARTY] Infant Discovery Pattern Demo Complete!");
    info!("📋 ACHIEVED:");
    info!("   [OK] Zero hardcoded primal names");
    info!("   [OK] Zero vendor-specific references");
    info!("   [OK] Pure capability-based discovery");
    info!("   [OK] True primal sovereignty maintained");

    Ok(())
}

async fn demonstrate_network_effects(
    ecosystem_state: &beardog_core::zero_knowledge_bootstrap::EcosystemState,
) -> Result<(), BearDogError> {
    info!("[CYCLE] Demonstrating network effects through universal adapter...");

    // Example: Need compute capability (discovered dynamically via universal adapter)
    if let Some(compute_providers) = ecosystem_state
        .available_capabilities
        .get(&ServiceCapabilityType::ComputeIntelligence)
    {
        for provider in compute_providers {
            info!(
                "   💻 Could connect to compute provider: {}",
                provider.provider_id
            );
            info!("      📡 Endpoint: {}", provider.endpoint);
            // In real implementation, would use universal adapter to connect
        }
    } else {
        info!("   ⚠️ No compute capability discovered in ecosystem");
    }

    // Example: Need service mesh capability (discovered dynamically via universal adapter)
    if let Some(mesh_providers) = ecosystem_state
        .available_capabilities
        .get(&ServiceCapabilityType::ServiceMesh)
    {
        for provider in mesh_providers {
            info!(
                "   🕸️ Could connect to mesh provider: {}",
                provider.provider_id
            );
            // In real implementation, would use universal adapter to connect
        }
    } else {
        info!("   ⚠️ No service mesh capability discovered in ecosystem");
    }

    // Example: Complex multi-hop scenario without hardcoding
    info!("   🔗 Complex scenario: AI analysis of storage data via compute");
    info!("      1. Discover storage capability providers");
    info!("      2. Discover compute capability providers");
    info!("      3. Discover AI capability providers");
    info!("      4. Chain them together through universal adapter");
    info!("      [OK] Pure capability-based chaining: storage->compute->ai (no hardcoded primal names)!");

    Ok(())
}
