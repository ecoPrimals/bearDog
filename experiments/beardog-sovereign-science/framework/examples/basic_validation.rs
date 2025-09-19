//! Basic validation example for the BearDog Sovereign Science Framework

use beardog_sovereign_science::*;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🧬 BearDog Sovereign Science Framework - Basic Validation Demo");
    info!("===========================================================");
    
    // Create framework with basic configuration
    let config = FrameworkConfig {
        enable_cryptographic: true,
        enable_performance: true,
        enable_distributed_security: false, // Disable for basic demo
        enable_human_dignity: false,        // Disable for basic demo
        enable_enterprise: false,           // Disable for basic demo
        confidence_level: 0.95,
        minimum_effect_size: 0.5,
        significance_threshold: 0.05,
    };
    
    // Initialize the framework
    let framework = match SovereignScienceFramework::with_config(config).await {
        Ok(framework) => {
            info!("✅ Framework initialized successfully");
            framework
        }
        Err(e) => {
            error!("❌ Failed to initialize framework: {}", e);
            return Err(e.into());
        }
    };
    
    // Execute validation
    info!("🚀 Starting basic validation (2 stages)...");
    let results = match framework.execute_full_validation().await {
        Ok(results) => {
            info!("✅ Validation completed successfully");
            results
        }
        Err(e) => {
            error!("❌ Validation failed: {}", e);
            return Err(e.into());
        }
    };
    
    // Display results
    info!("📊 VALIDATION RESULTS:");
    info!("=====================");
    println!("\n{}", results.executive_summary());
    
    info!("\n🎯 DETAILED RESULTS:");
    info!("   🔐 Mathematical Certainty: {}", 
        if results.mathematical_certainty() { "✅ ACHIEVED" } else { "❌ FAILED" });
    info!("   ⚡ Performance Excellence: {}", 
        if results.performance_excellence { "✅ ACHIEVED" } else { "❌ FAILED" });
    info!("   📈 Statistical Significance: p = {:.6}", results.statistical_significance);
    info!("   🎯 Effect Size: {:.2}", results.effect_size);
    info!("   ⏱️ Execution Time: {:?}", results.execution_duration);
    
    let success_rate = results.success_percentage();
    match success_rate {
        100.0 => info!("\n🎊 SUCCESS: 100% validation achieved!"),
        80.0..=99.9 => info!("\n✅ SUCCESS: {:.1}% validation achieved", success_rate),
        _ => error!("\n❌ PARTIAL SUCCESS: {:.1}% validation achieved", success_rate),
    }
    
    info!("\n🏆 Basic validation demonstration complete!");
    
    Ok(())
} 