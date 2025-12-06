#![allow(unused_imports, unused_variables, dead_code, unused_comparisons, clippy::all)]

// Chaos Testing Reporting
// Migrated October 7, 2025 - Updated for modular architecture

use super::models::*;
use std::time::SystemTime;
use tracing::info;

/// Generate a chaos test report
pub fn generate_chaos_report(
    scenario_results: Vec<ScenarioResult>,
    start_time: SystemTime,
) -> ChaosTestReport {
    let end_time = SystemTime::now();
    let total_scenarios = scenario_results.len();
    let successful_scenarios = scenario_results.iter().filter(|r| r.success).count();

    // Calculate overall resilience score
    let resilience_score = if total_scenarios > 0 {
        (successful_scenarios as f64 / total_scenarios as f64) * 100.0
    } else {
        0.0
    };

    // Generate recommendations
    let recommendations = generate_recommendations(&scenario_results);

    info!("Generated chaos test report:");
    info!("  Total scenarios: {}", total_scenarios);
    info!("  Successful: {}", successful_scenarios);
    info!("  Resilience score: {:.2}%", resilience_score);

    ChaosTestReport {
        start_time,
        end_time,
        total_scenarios,
        successful_scenarios,
        scenario_results,
        overall_resilience_score: resilience_score,
        recommendations,
    }
}

/// Generate recommendations based on test results
fn generate_recommendations(results: &[ScenarioResult]) -> Vec<String> {
    let mut recommendations = Vec::new();

    // Count failures by type
    let total_failures = results.iter().filter(|r| !r.success).count();

    if total_failures > 0 {
        recommendations.push(format!(
            "Address {} failed scenarios to improve resilience",
            total_failures
        ));
    }

    // Check recovery times
    let slow_recoveries: Vec<_> = results
        .iter()
        .filter(|r| {
            r.faults_injected.iter().any(|f| {
                f.recovery_time_ms.unwrap_or(0) > 10000
            })
        })
        .map(|r| r.scenario_name.clone())
        .collect();

    if !slow_recoveries.is_empty() {
        recommendations.push(format!(
            "Improve recovery time for: {}",
            slow_recoveries.join(", ")
        ));
    }

    // Check for high error rates
    let high_error_scenarios: Vec<_> = results
        .iter()
        .filter(|r| r.impact_summary.error_rate_increase > 0.10)
        .map(|r| r.scenario_name.clone())
        .collect();

    if !high_error_scenarios.is_empty() {
        recommendations.push(format!(
            "Reduce error rates in: {}",
            high_error_scenarios.join(", ")
        ));
    }

    // Add general recommendations
    if results.iter().all(|r| r.success) {
        recommendations.push(
            "Excellent resilience! Consider adding more complex scenarios.".to_string()
        );
    }

    if recommendations.is_empty() {
        recommendations.push("System shows good resilience to chaos testing.".to_string());
    }

    recommendations
}

/// Print a summary of chaos test results
pub fn print_chaos_summary(report: &ChaosTestReport) {
    println!("\n╔════════════════════════════════════════════════╗");
    println!("║        CHAOS TESTING SUMMARY                   ║");
    println!("╚════════════════════════════════════════════════╝\n");

    println!("📊 Overall Results:");
    println!("   Total Scenarios:    {}", report.total_scenarios);
    println!("   Successful:         {}", report.successful_scenarios);
    println!("   Failed:             {}", report.total_scenarios - report.successful_scenarios);
    println!("   Resilience Score:   {:.2}%", report.overall_resilience_score);

    println!("\n🔍 Scenario Details:");
    for result in &report.scenario_results {
        let status = if result.success { "✅" } else { "❌" };
        println!("   {} {} ({:.2}s)", status, result.scenario_name, result.duration_ms as f64 / 1000.0);
        
        if !result.failure_reasons.is_empty() {
            for reason in &result.failure_reasons {
                println!("      - {}", reason);
            }
        }
    }

    println!("\n💡 Recommendations:");
    for (i, recommendation) in report.recommendations.iter().enumerate() {
        println!("   {}. {}", i + 1, recommendation);
    }

    println!("\n");
}

