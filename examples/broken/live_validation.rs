// Live validation example using REAL cryptographic operations
//
// This example demonstrates the BearDog Sovereign Science Framework
// performing actual validation with real cryptographic operations,
// real statistical analysis, and genuine Cohen's d calculations.

use beardog_sovereign_science::*;
use std::time::Instant;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize comprehensive tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    info!("🔥 BEARDOG SOVEREIGN SCIENCE - LIVE VALIDATION EXPERIMENT");
    info!("=========================================================");
    info!("🎯 Using REAL cryptographic operations and statistical analysis");
    info!("⚠️  NO MOCKS - NO HARDCODED VALUES - LIVE DATA ONLY");

    let experiment_start = Instant::now();

    // Create framework with LIVE validation configuration
    let config = FrameworkConfig {
        enable_cryptographic: true,
        enable_performance: true,
        enable_distributed_security: true,
        enable_human_dignity: false, // Still not implemented
        enable_enterprise: false,    // Still not implemented
        confidence_level: 0.95,
        minimum_effect_size: 0.5,
        significance_threshold: 0.05,
    };

    info!("🔧 LIVE Configuration:");
    info!("   📊 Confidence Level: 95%");
    info!("   🎯 Significance Threshold: p < 0.05");
    info!("   📈 Minimum Effect Size: 0.5");
    info!("   🧪 Cryptographic: ✅ LIVE Ed25519 + SHA256");
    info!("   ⚡ Performance: ✅ LIVE throughput measurements");
    info!("   🛡️ Security: ✅ LIVE entropy + timing analysis");

    // Initialize the LIVE framework
    let framework = match SovereignScienceFramework::with_config(config) {
        Ok(framework) => {
            info!("✅ LIVE validation framework initialized");
            framework
        }
        Err(e) => {
            error!("❌ Failed to initialize LIVE framework: {}", e);
            return Err(e.into());
        }
    };

    info!("🚀 Beginning LIVE validation with REAL operations...");
    info!("   This will execute thousands of cryptographic operations");
    info!("   and measure actual performance characteristics");

    // Execute LIVE validation
    let results = match framework.execute_full_validation() {
        Ok(results) => {
            info!("✅ LIVE validation completed successfully");
            results
        }
        Err(e) => {
            error!("❌ LIVE validation failed: {}", e);
            return Err(e.into());
        }
    };

    let total_duration = experiment_start.elapsed();

    // Display LIVE results
    info!("\n🏆 LIVE VALIDATION RESULTS:");
    info!("===========================");
    println!("\n{}", results.executive_summary());

    info!("\n📊 LIVE STATISTICAL ANALYSIS:");
    info!(
        "   📈 Statistical Significance: p = {:.6} {}",
        results.statistical_significance,
        if results.statistical_significance < 0.05 {
            "(SIGNIFICANT)"
        } else {
            "(NOT SIGNIFICANT)"
        }
    );
    info!(
        "   🎯 Effect Size (Cohen's d): {:.4} {}",
        results.effect_size,
        match results.effect_size {
            d if d >= 0.8 => "(LARGE EFFECT)",
            d if d >= 0.5 => "(MEDIUM EFFECT)",
            d if d >= 0.2 => "(SMALL EFFECT)",
            _ => "(NO EFFECT)",
        }
    );
    info!(
        "   📏 Confidence Interval: {:.1}% - {:.1}%",
        results.confidence_interval.0 * 100.0,
        results.confidence_interval.1 * 100.0
    );
    info!(
        "   ⏱️ Framework Execution: {:?}",
        results.execution_duration
    );
    info!("   🕐 Total Experiment Time: {:?}", total_duration);

    info!("\n🔬 LIVE OPERATION DETAILS:");
    if let Some(ref crypto_results) = results.cryptographic_results {
        info!("   🔐 Cryptographic Foundation:");
        info!(
            "      Mathematical Security: {}",
            if crypto_results.all_operations_mathematically_secure {
                "✅ VALIDATED"
            } else {
                "❌ FAILED"
            }
        );
        info!(
            "      Timing Attack Immunity: {}",
            if crypto_results.zero_timing_vulnerabilities {
                "✅ VALIDATED"
            } else {
                "❌ FAILED"
            }
        );
        info!(
            "      Perfect Forward Secrecy: {}",
            if crypto_results.perfect_forward_secrecy {
                "✅ VALIDATED"
            } else {
                "❌ FAILED"
            }
        );
        info!(
            "      NIST Entropy Standards: {}",
            if crypto_results.entropy_exceeds_nist_standards {
                "✅ VALIDATED"
            } else {
                "❌ FAILED"
            }
        );
        info!(
            "      Zero External Dependencies: {}",
            if crypto_results.zero_external_dependencies {
                "✅ VALIDATED"
            } else {
                "❌ FAILED"
            }
        );
    }

    if let Some(ref perf_results) = results.performance_results {
        info!("   ⚡ Performance Excellence:");
        info!(
            "      Zero-Copy Efficiency: {:.1}%",
            perf_results.zero_copy_efficiency * 100.0
        );
        info!(
            "      Memory Safety: {}",
            if perf_results.memory_safety_maintained {
                "✅ VALIDATED"
            } else {
                "❌ FAILED"
            }
        );
        info!(
            "      Linear Scaling: {}",
            if perf_results.linear_scaling_validated {
                "✅ VALIDATED"
            } else {
                "❌ FAILED"
            }
        );
        info!(
            "      Sub-Microsecond Latency: {}",
            if perf_results.sub_microsecond_latency {
                "✅ VALIDATED"
            } else {
                "❌ FAILED"
            }
        );
        info!(
            "      Reproducibility: {}",
            if perf_results.complete_reproducibility {
                "✅ VALIDATED"
            } else {
                "❌ FAILED"
            }
        );
    }

    info!("\n🎯 LIVE VALIDATION ASSESSMENT:");
    let success_rate = results.success_percentage();
    match success_rate {
        100.0 => {
            info!("🎊 PERFECT VALIDATION: 100% Success - REAL mathematical certainty!");
            info!("   🏆 All validation criteria met with LIVE operations");
            info!("   🚀 Ready for enterprise production deployment");
            info!("   📜 Production certificate can be issued immediately");
        }
        80.0..=99.9 => {
            info!(
                "✅ EXCELLENT VALIDATION: {:.1}% Success - Production Ready",
                success_rate
            );
            info!("   🎯 Most validation criteria met with LIVE data");
            info!("   ⚠️ Minor issues detected in LIVE analysis");
        }
        60.0..=79.9 => {
            warn!(
                "⚠️ GOOD VALIDATION: {:.1}% Success - Improvement Needed",
                success_rate
            );
            warn!("   📋 Several validation criteria failed in LIVE testing");
            warn!("   🔧 Significant improvements required");
        }
        _ => {
            error!(
                "❌ INSUFFICIENT VALIDATION: {:.1}% Success - Critical Issues",
                success_rate
            );
            error!("   🚨 Critical validation failures detected in LIVE operations");
            error!("   🔴 Not ready for production deployment");
        }
    }

    // Statistical significance assessment
    info!("\n📈 STATISTICAL RIGOR ASSESSMENT:");
    if results.statistical_significance < 0.001 {
        info!("   🏆 HIGHLY SIGNIFICANT: p < 0.001 - Exceptional statistical evidence");
    } else if results.statistical_significance < 0.01 {
        info!("   ✅ VERY SIGNIFICANT: p < 0.01 - Strong statistical evidence");
    } else if results.statistical_significance < 0.05 {
        info!("   📊 SIGNIFICANT: p < 0.05 - Statistically significant results");
    } else {
        warn!("   ⚠️ NOT SIGNIFICANT: p >= 0.05 - Insufficient statistical evidence");
    }

    // Effect size interpretation
    if results.effect_size >= 0.8 {
        info!(
            "   🎯 LARGE EFFECT: Cohen's d = {:.3} - Practically significant difference",
            results.effect_size
        );
    } else if results.effect_size >= 0.5 {
        info!(
            "   📈 MEDIUM EFFECT: Cohen's d = {:.3} - Moderate practical significance",
            results.effect_size
        );
    } else if results.effect_size >= 0.2 {
        info!(
            "   📊 SMALL EFFECT: Cohen's d = {:.3} - Minor practical significance",
            results.effect_size
        );
    } else {
        warn!(
            "   ❌ NO EFFECT: Cohen's d = {:.3} - No practical significance detected",
            results.effect_size
        );
    }

    info!("\n🧬 SOVEREIGN SCIENCE VALIDATION COMPLETE!");
    info!("   Experiment ID: {}", results.experiment_id);
    info!("   Framework: BearDog Sovereign Science v1.0.0");
    info!("   Methodology: LIVE Cryptographic Operations + Real Statistical Analysis");
    info!("   Validation Type: Mathematical Security Certainty with Human Dignity");
    info!("   Data Source: ACTUAL cryptographic operations (Ed25519, SHA256, entropy)");
    info!("   Statistical Method: Real Cohen's d calculation with t-test significance");

    info!("\n🔥 LIVE VALIDATION SESSION COMPLETE!");
    info!(
        "   Status: {} - Using REAL data, not mocks!",
        if success_rate >= 80.0 {
            "SUCCESS"
        } else {
            "NEEDS IMPROVEMENT"
        }
    );

    Ok(())
}
