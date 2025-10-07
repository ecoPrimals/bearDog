// Data Collection Experiment for BearDog Sovereign Science Framework
//
// This experiment runs multiple validation cycles and collects comprehensive data
// for analysis, saving results to the data directory.

use beardog_sovereign_science::*;
use chrono::Utc;
use serde_json;
use std::fs;
use std::path::Path;
use std::time::Instant;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    info!("🧬 BEARDOG SOVEREIGN SCIENCE - DATA COLLECTION EXPERIMENT");
    info!("=========================================================");
    info!("🎯 Running multiple validation cycles for comprehensive data collection");

    let experiment_start = Instant::now();
    let experiment_id = format!("DATA-COLLECTION-{}", Utc::now().format("%Y%m%d-%H%M%S"));

    // Ensure data directory exists
    let data_dir = Path::new("../data");
    if !data_dir.exists() {
        fs::create_dir_all(data_dir)?;
        info!("📁 Created data directory: {:?}", data_dir);
    }

    let mut all_results = Vec::new();
    let validation_cycles = 5;

    info!("🔬 Running {} validation cycles...", validation_cycles);

    // Run multiple validation cycles with different configurations
    for cycle in 1..=validation_cycles {
        info!("\n🚀 Validation Cycle {}/{}", cycle, validation_cycles);

        // Vary configurations for each cycle
        let config = match cycle {
            1 => FrameworkConfig {
                enable_cryptographic: true,
                enable_performance: false,
                enable_distributed_security: false,
                enable_human_dignity: false,
                enable_enterprise: false,
                confidence_level: 0.95,
                minimum_effect_size: 0.5,
                significance_threshold: 0.05,
            },
            2 => FrameworkConfig {
                enable_cryptographic: true,
                enable_performance: true,
                enable_distributed_security: false,
                enable_human_dignity: false,
                enable_enterprise: false,
                confidence_level: 0.95,
                minimum_effect_size: 0.5,
                significance_threshold: 0.05,
            },
            3 => FrameworkConfig {
                enable_cryptographic: true,
                enable_performance: true,
                enable_distributed_security: true,
                enable_human_dignity: false,
                enable_enterprise: false,
                confidence_level: 0.99,
                minimum_effect_size: 0.3,
                significance_threshold: 0.01,
            },
            4 => FrameworkConfig {
                enable_cryptographic: true,
                enable_performance: true,
                enable_distributed_security: true,
                enable_human_dignity: true,
                enable_enterprise: false,
                confidence_level: 0.99,
                minimum_effect_size: 0.3,
                significance_threshold: 0.01,
            },
            5 => FrameworkConfig {
                enable_cryptographic: true,
                enable_performance: true,
                enable_distributed_security: true,
                enable_human_dignity: true,
                enable_enterprise: true,
                confidence_level: 0.999,
                minimum_effect_size: 0.2,
                significance_threshold: 0.001,
            },
            _ => FrameworkConfig::default(),
        };

        let stages_enabled = [
            config.enable_cryptographic,
            config.enable_performance,
            config.enable_distributed_security,
            config.enable_human_dignity,
            config.enable_enterprise,
        ]
        .iter()
        .filter(|&&x| x)
        .count();

        info!(
            "   📊 Configuration: {} stages, {:.1}% confidence",
            stages_enabled,
            config.confidence_level * 100.0
        );

        // Run validation
        let framework = SovereignScienceFramework::with_config(config)?;
        let cycle_start = Instant::now();
        let results = framework.execute_full_validation()?;
        let cycle_duration = cycle_start.elapsed();

        info!("   ✅ Cycle {} completed in {:?}", cycle, cycle_duration);
        info!("   📈 Success Rate: {:.1}%", results.success_percentage());
        info!(
            "   🔬 Statistical Significance: p = {:.6}",
            results.statistical_significance
        );

        // Create cycle data
        let cycle_data = serde_json::json!({
            "cycle": cycle,
            "experiment_id": experiment_id,
            "timestamp": Utc::now().to_rfc3339(),
            "duration_micros": cycle_duration.as_micros(),
            "configuration": {
                "stages_enabled": stages_enabled,
                "confidence_level": config.confidence_level,
                "significance_threshold": config.significance_threshold,
                "minimum_effect_size": config.minimum_effect_size
            },
            "results": {
                "success_percentage": results.success_percentage(),
                "mathematical_certainty": results.mathematical_certainty,
                "performance_excellence": results.performance_excellence,
                "human_dignity_preserved": results.human_dignity_preserved,
                "enterprise_ready": results.enterprise_ready,
                "sovereign_independence": results.sovereign_independence,
                "statistical_significance": results.statistical_significance,
                "effect_size": results.effect_size,
                "confidence_interval": results.confidence_interval,
                "execution_duration_micros": results.execution_duration.as_micros()
            }
        });

        all_results.push(cycle_data);
    }

    let total_duration = experiment_start.elapsed();

    // Calculate aggregate statistics
    let success_rates: Vec<f64> = all_results
        .iter()
        .map(|r| r["results"]["success_percentage"].as_f64().unwrap_or(0.0))
        .collect();

    let avg_success_rate = success_rates.iter().sum::<f64>() / success_rates.len() as f64;
    let min_success_rate = success_rates.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_success_rate = success_rates
        .iter()
        .fold(f64::NEG_INFINITY, |a, &b| a.max(b));

    // Create comprehensive experiment report
    let experiment_report = serde_json::json!({
        "experiment_id": experiment_id,
        "framework_version": "1.0.0",
        "methodology": "BearDog Sovereign Science",
        "start_time": format!("{:?}", experiment_start),
        "total_duration_secs": total_duration.as_secs_f64(),
        "validation_cycles": validation_cycles,
        "aggregate_statistics": {
            "average_success_rate": avg_success_rate,
            "minimum_success_rate": min_success_rate,
            "maximum_success_rate": max_success_rate,
            "success_rate_range": max_success_rate - min_success_rate,
            "improvement_trend": success_rates.last().unwrap() - success_rates.first().unwrap()
        },
        "cycles": all_results
    });

    // Save results to data directory
    let results_file = data_dir.join(format!("{}.json", experiment_id));
    fs::write(
        &results_file,
        serde_json::to_string_pretty(&experiment_report)?,
    )?;

    info!("\n🎊 DATA COLLECTION EXPERIMENT COMPLETE!");
    info!("=======================================");
    info!("   🆔 Experiment ID: {}", experiment_id);
    info!("   📁 Results saved to: {:?}", results_file);
    info!("   ⏱️ Total Duration: {:?}", total_duration);
    info!("   🔬 Validation Cycles: {}", validation_cycles);

    info!("\n📊 AGGREGATE RESULTS:");
    info!("   📈 Average Success Rate: {:.1}%", avg_success_rate);
    info!("   📉 Minimum Success Rate: {:.1}%", min_success_rate);
    info!("   📊 Maximum Success Rate: {:.1}%", max_success_rate);
    info!(
        "   📏 Success Rate Range: {:.1}%",
        max_success_rate - min_success_rate
    );

    let improvement = success_rates.last().unwrap() - success_rates.first().unwrap();
    if improvement > 0.0 {
        info!(
            "   📈 Improvement Trend: +{:.1}% (Positive progression)",
            improvement
        );
    } else if improvement < 0.0 {
        info!(
            "   📉 Improvement Trend: {:.1}% (Needs attention)",
            improvement
        );
    } else {
        info!("   ➡️ Improvement Trend: {:.1}% (Stable)", improvement);
    }

    // Provide recommendations based on results
    info!("\n🎯 EXPERIMENTAL RECOMMENDATIONS:");
    if avg_success_rate >= 80.0 {
        info!("   ✅ EXCELLENT: Framework performing at enterprise level");
        info!("   🚀 Recommendation: Proceed to production deployment");
    } else if avg_success_rate >= 60.0 {
        info!("   ⚠️ GOOD: Framework showing solid performance");
        info!("   🔧 Recommendation: Focus on failing validation stages");
    } else {
        warn!("   ❌ NEEDS IMPROVEMENT: Framework requires significant work");
        warn!("   🚨 Recommendation: Address fundamental validation issues");
    }

    info!("\n🧬 Sovereign Science data collection complete!");
    info!("   Next steps: Analyze collected data for patterns and insights");

    Ok(())
}
