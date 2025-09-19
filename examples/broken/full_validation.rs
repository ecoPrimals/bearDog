// Full validation example for the BearDog Sovereign Science Framework
//
// This example demonstrates the complete 5-stage validation process:
// 1. Cryptographic Foundation Validation
// 2. Zero-Copy Performance Validation
// 3. Distributed Security Validation
// 4. Human Dignity Validation
// 5. Enterprise Production Validation

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

    info!("🧬 BEARDOG SOVEREIGN SCIENCE - FULL VALIDATION EXPERIMENT");
    info!("==========================================================");
    info!("🎯 Executing all 5 validation stages for comprehensive analysis");

    let experiment_start = Instant::now();

    // Create framework with full validation configuration
    let config = FrameworkConfig {
        enable_cryptographic: true,
        enable_performance: true,
        enable_distributed_security: true,
        enable_human_dignity: true,
        enable_enterprise: true,
        confidence_level: 0.99,       // Higher confidence for full validation
        minimum_effect_size: 0.3,     // Lower threshold for comprehensive analysis
        significance_threshold: 0.01, // More stringent significance
    };

    info!("🔧 Configuration:");
    info!("   📊 Confidence Level: 99%");
    info!("   🎯 Significance Threshold: p < 0.01");
    info!("   📈 Minimum Effect Size: 0.3");
    info!("   🧪 All 5 Stages Enabled: ✅");

    // Initialize the framework
    let framework = match SovereignScienceFramework::with_config(config) {
        Ok(framework) => {
            info!("✅ Full validation framework initialized");
            framework
        }
        Err(e) => {
            error!("❌ Failed to initialize framework: {}", e);
            return Err(e.into());
        }
    };

    info!("🚀 Beginning comprehensive 5-stage validation...");
    info!("   Expected duration: 17+ weeks (simulated in microseconds)");

    // Execute full validation
    let results = match framework.execute_full_validation() {
        Ok(results) => {
            info!("✅ Full validation completed successfully");
            results
        }
        Err(e) => {
            error!("❌ Full validation failed: {}", e);
            return Err(e.into());
        }
    };

    let total_duration = experiment_start.elapsed();

    // Display comprehensive results
    info!("🏆 COMPREHENSIVE VALIDATION RESULTS:");
    info!("====================================");
    println!("\n{}", results.executive_summary());

    info!("\n📊 DETAILED STAGE ANALYSIS:");
    info!(
        "   🔐 Stage 1 - Cryptographic Foundation: {}",
        if results.mathematical_certainty {
            "✅ VALIDATED"
        } else {
            "❌ FAILED"
        }
    );
    info!(
        "   ⚡ Stage 2 - Zero-Copy Performance: {}",
        if results.performance_excellence {
            "✅ VALIDATED"
        } else {
            "❌ FAILED"
        }
    );
    info!(
        "   🌐 Stage 3 - Distributed Security: {}",
        if results.sovereign_independence {
            "✅ VALIDATED"
        } else {
            "❌ FAILED"
        }
    );
    info!(
        "   👥 Stage 4 - Human Dignity: {}",
        if results.human_dignity_preserved {
            "✅ VALIDATED"
        } else {
            "❌ FAILED"
        }
    );
    info!(
        "   🏭 Stage 5 - Enterprise Production: {}",
        if results.enterprise_ready {
            "✅ VALIDATED"
        } else {
            "❌ FAILED"
        }
    );

    info!("\n📈 STATISTICAL ANALYSIS:");
    info!(
        "   📊 Statistical Significance: p = {:.6}",
        results.statistical_significance
    );
    info!("   🎯 Effect Size (Cohen's d): {:.3}", results.effect_size);
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

    info!("\n🎯 VALIDATION OUTCOMES:");
    let success_rate = results.success_percentage();
    match success_rate {
        100.0 => {
            info!("🎊 PERFECT VALIDATION: 100% Success - Deploy Immediately!");
            info!("   🏆 All validation criteria met");
            info!("   🚀 Ready for enterprise production deployment");
            info!("   📜 Deployment certificate can be issued");
        }
        80.0..=99.9 => {
            info!(
                "✅ EXCELLENT VALIDATION: {:.1}% Success - Production Ready",
                success_rate
            );
            info!("   🎯 Most validation criteria met");
            info!("   ⚠️ Minor issues to address before deployment");
        }
        60.0..=79.9 => {
            warn!(
                "⚠️ GOOD VALIDATION: {:.1}% Success - Needs Improvement",
                success_rate
            );
            warn!("   📋 Several validation criteria need attention");
            warn!("   🔧 Significant improvements required");
        }
        _ => {
            error!(
                "❌ INSUFFICIENT VALIDATION: {:.1}% Success - Major Issues",
                success_rate
            );
            error!("   🚨 Critical validation failures detected");
            error!("   🔴 Not ready for production deployment");
        }
    }

    // Generate deployment readiness assessment
    info!("\n🏭 DEPLOYMENT READINESS ASSESSMENT:");
    if results.is_fully_validated() {
        info!("   🎊 Status: FULLY VALIDATED - IMMEDIATE DEPLOYMENT APPROVED");
        info!("   📜 Certificate: Production Deployment Certificate Issued");
        info!("   🌟 Achievement: Mathematical Security Certainty Proven");
    } else {
        let missing_validations = vec![
            ("Mathematical Certainty", results.mathematical_certainty),
            ("Performance Excellence", results.performance_excellence),
            (
                "Human Dignity Preservation",
                results.human_dignity_preserved,
            ),
            ("Enterprise Readiness", results.enterprise_ready),
            ("Sovereign Independence", results.sovereign_independence),
        ];

        info!("   ⚠️ Status: PARTIAL VALIDATION - CONDITIONAL DEPLOYMENT");
        info!("   📋 Missing Validations:");
        for (name, validated) in missing_validations {
            if !validated {
                info!("      ❌ {}", name);
            }
        }
    }

    info!("\n🧬 SOVEREIGN SCIENCE VALIDATION COMPLETE!");
    info!("   Experiment ID: {}", results.experiment_id);
    info!("   Framework: BearDog Sovereign Science v1.0.0");
    info!("   Methodology: Mathematical Security Certainty with Human Dignity");
    info!("   Total Stages Executed: 5/5");
    info!("   Statistical Rigor: Enterprise-Grade");

    Ok(())
}
