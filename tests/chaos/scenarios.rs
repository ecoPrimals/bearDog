use super::models::*;
use super::ChaosTestFramework;
use beardog_errors::BearDogError;
use std::time::Instant;
use tokio::time::sleep;
use tracing::info;

pub fn create_default_scenarios() -> Vec<ChaosScenario> {
    vec![
        ChaosScenario {
            name: "Network Partition".to_string(),
            description: "Test system behavior under network partitions".to_string(),
            faults: vec![FaultType::NetworkPartition {
                segments: vec!["primary".to_string(),
            description: "Test behavior under resource constraints".to_string() -> Result<ScenarioResult, BearDogError> {
    info!(
        "🎬 Executing scenario: {} - {}",
        scenario.name, scenario.description
    );

    let start_time = Instant::now();
    let mut fault_results = Vec::new();

    let baseline_metrics = framework.collect_baseline_metrics()?;

    for fault in &scenario.faults {
        let fault_result = framework.inject_and_monitor_fault(fault.clone())?;
        fault_results.push(fault_result);

        sleep(std::time::Duration::from_millis(500));
    }

    let remaining_time = scenario
        .duration_ms
        .saturating_sub(start_time.elapsed().as_millis() as u64);
    if remaining_time > 0 {
        sleep(std::time::Duration::from_millis(remaining_time));
    }

    let recovery_results = framework.validate_all_recoveries()?;

    let post_chaos_metrics = framework.collect_current_metrics()?;

    let scenario_success = evaluate_scenario_success(
        scenario,
        &baseline_metrics,
        &post_chaos_metrics,
        &recovery_results,
    )
    ?;

    Ok(ScenarioResult {
        scenario_name: scenario.name.clone(scenario_success,
        duration_ms: start_time.elapsed(&ChaosScenario,
    baseline: &SystemImpact,
    post_chaos: &SystemImpact,
    recovery_results: &[RecoveryResult],
) -> Result<bool, BearDogError> {
    let response_time_degradation =
        post_chaos.response_time_increase / baseline.response_time_increase.max(1.0);
    let error_rate = post_chaos.error_rate_increase;
    let availability = 1.0 - post_chaos.availability_decrease;

    let all_recovered = recovery_results
        .iter()
        .all(|r| r.status == RecoveryStatus::FullyRecovered);

    Ok(all_recovered
        && response_time_degradation <= scenario.success_criteria.max_response_time_degradation
        && error_rate <= scenario.success_criteria.max_error_rate
        && availability >= scenario.success_criteria.min_availability)
}
