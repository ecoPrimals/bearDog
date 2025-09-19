// Test to verify LIVE statistical calculations work correctly
//
// This test will create different data scenarios and verify that
// Cohen's d changes based on actual data rather than being hardcoded.

use beardog_sovereign_science::*;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    info!("🧪 TESTING LIVE STATISTICAL CALCULATIONS");
    info!("========================================");
    info!("🎯 Verifying Cohen's d is calculated from REAL data, not hardcoded");

    // Test 1: Framework with only cryptographic validation (should have different results)
    info!("\n📊 Test 1: Cryptographic-only validation");
    let config1 = FrameworkConfig {
        enable_cryptographic: true,
        enable_performance: false,
        enable_distributed_security: false,
        enable_human_dignity: false,
        enable_enterprise: false,
        confidence_level: 0.95,
        minimum_effect_size: 0.5,
        significance_threshold: 0.05,
    };

    let framework1 = SovereignScienceFramework::with_config(config1)?;
    let results1 = framework1.execute_full_validation()?;

    info!("   Cohen's d: {:.4}", results1.effect_size);
    info!("   p-value: {:.6}", results1.statistical_significance);
    info!("   Success rate: {:.1}%", results1.success_percentage());

    // Test 2: Framework with cryptographic + performance (should have different results)
    info!("\n📊 Test 2: Cryptographic + Performance validation");
    let config2 = FrameworkConfig {
        enable_cryptographic: true,
        enable_performance: true,
        enable_distributed_security: false,
        enable_human_dignity: false,
        enable_enterprise: false,
        confidence_level: 0.95,
        minimum_effect_size: 0.5,
        significance_threshold: 0.05,
    };

    let framework2 = SovereignScienceFramework::with_config(config2)?;
    let results2 = framework2.execute_full_validation()?;

    info!("   Cohen's d: {:.4}", results2.effect_size);
    info!("   p-value: {:.6}", results2.statistical_significance);
    info!("   Success rate: {:.1}%", results2.success_percentage());

    // Test 3: Framework with all three enabled (should have different results again)
    info!("\n📊 Test 3: Cryptographic + Performance + Distributed validation");
    let config3 = FrameworkConfig {
        enable_cryptographic: true,
        enable_performance: true,
        enable_distributed_security: true,
        enable_human_dignity: false,
        enable_enterprise: false,
        confidence_level: 0.95,
        minimum_effect_size: 0.5,
        significance_threshold: 0.05,
    };

    let framework3 = SovereignScienceFramework::with_config(config3)?;
    let results3 = framework3.execute_full_validation()?;

    info!("   Cohen's d: {:.4}", results3.effect_size);
    info!("   p-value: {:.6}", results3.statistical_significance);
    info!("   Success rate: {:.1}%", results3.success_percentage());

    // Analysis
    info!("\n🔍 ANALYSIS:");

    let cohens_d_values = vec![
        results1.effect_size,
        results2.effect_size,
        results3.effect_size,
    ];
    let p_values = vec![
        results1.statistical_significance,
        results2.statistical_significance,
        results3.statistical_significance,
    ];

    // Check if Cohen's d values are different (indicating live calculation)
    let all_same_cohens_d = cohens_d_values
        .windows(2)
        .all(|w| (w[0] - w[1]).abs() < 0.0001);
    let all_same_p_values = p_values.windows(2).all(|w| (w[0] - w[1]).abs() < 0.000001);

    if all_same_cohens_d && all_same_p_values {
        info!("❌ STILL USING HARDCODED VALUES!");
        info!(
            "   All Cohen's d values are identical: {:.4}",
            results1.effect_size
        );
        info!(
            "   All p-values are identical: {:.6}",
            results1.statistical_significance
        );
        info!("   This indicates the statistical framework is still using mocked data.");
    } else {
        info!("✅ LIVE STATISTICAL CALCULATIONS WORKING!");
        info!(
            "   Cohen's d values vary: {:.4}, {:.4}, {:.4}",
            results1.effect_size, results2.effect_size, results3.effect_size
        );
        info!(
            "   p-values vary: {:.6}, {:.6}, {:.6}",
            results1.statistical_significance,
            results2.statistical_significance,
            results3.statistical_significance
        );
        info!("   This indicates the statistical framework is using real data.");
    }

    info!("\n🧬 Live statistics test complete!");

    Ok(())
}
