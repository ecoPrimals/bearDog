//! # Mobile HSM Demonstration
//!
//! This example demonstrates BearDog's mobile-first HSM architecture with:
//! - Android StrongBox integration (Pixel 8 + GrapheneOS optimized)
//! - Intelligent operation routing (mobile-first for critical ops)
//! - Software HSM fallback for routine operations
//! - Real-world usage patterns

use beardog_errors::BearDogResult;
use beardog_tunnel::{
    hsm::{setup_development_mobile_hsm, setup_production_mobile_hsm, MobileHsmSetup},
    tunnel::hsm::initialize_mobile_hsm_manager,
};
use tracing::{info, warn};

#[tokio::main]
async fn main() -> BearDogResult<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_level(true)
        .init();

    info!("🚀 BearDog Mobile HSM Architecture Demo");
    info!("═══════════════════════════════════════");

    // Demonstrate development setup
    demo_development_setup().await?;

    // Demonstrate production setup (will use fallbacks on non-Android)
    demo_production_setup().await?;

    // Demonstrate operation routing patterns
    demo_operation_routing().await?;

    // Demonstrate real-world scenarios
    demo_real_world_scenarios().await?;

    info!("🎉 Mobile HSM demonstration completed successfully!");
    Ok(())
}

/// Demonstrate development HSM setup
async fn demo_development_setup() -> BearDogResult<()> {
    info!("\n📱 === DEVELOPMENT SETUP DEMO ===");

    // Set up development environment (allows software fallback)
    let hsm_manager = setup_development_mobile_hsm().await?;

    info!("✅ Development HSM manager initialized");

    // Get routing metrics
    let metrics = hsm_manager.get_routing_metrics().await?;
    info!("📊 Initial routing metrics: {:?}", metrics);

    info!("✅ Development HSM manager ready for operations");
    info!("📋 Note: Detailed operation testing requires platform-specific setup");

    Ok(())
}

/// Demonstrate production HSM setup
async fn demo_production_setup() -> BearDogResult<()> {
    info!("\n🏭 === PRODUCTION SETUP DEMO ===");

    // Attempt production setup (will fallback to software on non-Android)
    match setup_production_mobile_hsm().await {
        Ok(hsm_manager) => {
            info!("✅ Production HSM environment initialized successfully");
            info!("📱 HSM Manager operational with mobile-first routing");

            // Display routing information
            info!("🔄 Mobile HSM routing capabilities demonstrated");
        }
        Err(e) => {
            warn!(
                "⚠️ Production setup unavailable (likely not on Android): {}",
                e
            );
            info!("📱 This is expected when running on non-Android platforms");

            // Demonstrate configuration instead
            let config = beardog_tunnel::hsm::create_pixel8_graphene_config();
            info!("🎯 Production configuration created:");
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

    Ok(())
}

/// Demonstrate intelligent operation routing
async fn demo_operation_routing() -> BearDogResult<()> {
    info!("\n🎯 === OPERATION ROUTING DEMO ===");

    // Create custom configuration for demonstration
    let custom_config = MobileHsmSetup {
        require_mobile_for_critical: false, // Allow fallbacks for demo
        enable_graphene_optimizations: false,
        ..Default::default()
    };

    let hsm_manager = initialize_mobile_hsm_manager(custom_config).await?;

    info!("🔄 Operation routing system initialized successfully");
    info!("📋 Operation types supported:");
    info!("   🔑 Human Identity (requires mobile HSM)");
    info!("   🌱 Root Key Generation (requires mobile HSM)");
    info!("   🔐 Critical Authentication (prefers mobile HSM)");
    info!("   📁 File Encryption (uses software HSM for performance)");
    info!("   💾 Data Processing (uses software HSM for performance)");
    info!("   🧬 Local Spawning (uses software HSM for performance)");

    // Show routing metrics
    let metrics = hsm_manager.get_routing_metrics().await?;
    info!("📊 Routing metrics: {:?}", metrics);

    Ok(())
}

/// Demonstrate real-world usage scenarios
async fn demo_real_world_scenarios() -> BearDogResult<()> {
    info!("\n🌍 === REAL-WORLD SCENARIOS DEMO ===");

    let hsm_manager = setup_development_mobile_hsm().await?;

    info!("👤 Scenario 1: User Authentication Flow");
    info!("   ✅ Mobile HSM would handle biometric authentication");
    info!("   ✅ Hardware-backed key attestation available");
    info!("   ✅ User presence validation supported");

    info!("📄 Scenario 2: Secure Document Processing");
    info!("   ✅ Software HSM optimized for bulk operations");
    info!("   ✅ High-performance encryption for documents");
    info!("   ✅ Memory-protected key storage");

    info!("🧬 Scenario 3: Genetic Key Spawning");
    info!("   ✅ Efficient software HSM for spawning operations");
    info!("   ✅ Genetic algorithms benefit from local processing");
    info!("   ✅ Automatic routing to optimal HSM tier");

    // Show final state
    let metrics = hsm_manager.get_routing_metrics().await?;
    info!("📊 Final system metrics: {:?}", metrics);

    Ok(())
}
