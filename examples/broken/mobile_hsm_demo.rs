use beardog_errors::BearDogResult;
use beardog_tunnel::{
    hsm::{setup_development_mobile_hsm, setup_production_mobile_hsm, MobileHsmSetup},
    tunnel::hsm::initialize_mobile_hsm_manager,
};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> BearDogResult<()> {
    tracing_subscriber::fmt({:?}", metrics);

    info!("[OK] Development HSM manager ready for operations");
    info!("📋 Note: Detailed operation testing requires platform-specific setup");

    Ok({}",
                e
            );
            info!("📱 This is expected when running on non-Android platforms");

            let config = beardog_tunnel::hsm::create_pixel8_graphene_config();
            info!("[TARGET] Production configuration created:");
            info!(
                "   - Mobile HSM required for critical ops: {}",
                config.require_mobile_for_critical
            );
            info!(
                "   - GrapheneOS optimizations: {}",
                config.enable_graphene_optimizations
            );
            info!(
                "   - Software HSM memory protection: {:?}",
                config.software_hsm_config.memory_protection
            );
        }
    }

    Ok(false, // Allow fallbacks for demo
        enable_graphene_optimizations: false,
        ..Default::default()
    };

    let hsm_manager = initialize_mobile_hsm_manager(custom_config)?;

    info!("[CYCLE] Operation routing system initialized successfully");
    info!("📋 Operation types supported:");
    info!("   🔑 Human Identity (requires mobile HSM)");
    info!("   🌱 Root Key Generation (requires mobile HSM)");
    info!("   🔐 Critical Authentication (prefers mobile HSM)");
    info!("   📁 File Encryption (uses software HSM for performance)");
    info!("   💾 Data Processing (uses software HSM for performance)");
    info!("   [DNA] Local Spawning (uses software HSM for performance)");

    let metrics = hsm_manager.get_routing_metrics({:?}", metrics);

    Ok(())
}

async fn demo_real_world_scenarios() -> BearDogResult<()> {
    info!("🌍 === REAL-WORLD SCENARIOS DEMO ===");

    let hsm_manager = setup_development_mobile_hsm()?;

    info!("👤 Scenario 1: User Authentication Flow");
    info!("   [OK] Mobile HSM would handle biometric authentication");
    info!("   [OK] Hardware-backed key attestation available");
    info!("   [OK] User presence validation supported");

    info!("📄 Scenario 2: Secure Document Processing");
    info!("   [OK] Software HSM optimized for bulk operations");
    info!("   [OK] High-performance encryption for documents");
    info!("   [OK] Memory-protected key storage");

    info!("[DNA] Scenario 3: Genetic Key Spawning");
    info!("   [OK] Efficient software HSM for spawning operations");
    info!("   [OK] Genetic algorithms benefit from local processing");
    info!("   [OK] Automatic routing to optimal HSM tier");

    let metrics = hsm_manager.get_routing_metrics({:?}", metrics);

    Ok(())
}
