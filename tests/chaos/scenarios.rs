

use super::models::*;
use super::ChaosTestFramework;
use beardog::BearDogResult;
use std::time::Instant;
use tokio::time::sleep;
use tracing::info;

pub fn create_default_scenarios() -> Vec<ChaosScenario> {
    vec![
        ChaosScenario {
            name: "Network Partition".to_string(),
            description: "Test system behavior under network partitions".to_string(),
            faults: vec![
                FaultType::NetworkPartition {
                    segments: vec!["primary".to_string(), "secondary".to_string()],
                    duration_ms: 10000,
                }
            ],
            duration_ms: 15000,
            success_criteria: SuccessCriteria {
                max_recovery_time_ms: 30000,
                max_error_rate: 0.1,
                min_availability: 0.8,
                max_response_time_degradation: 3.0,
            },
        },
        
        ChaosScenario {
            name: "Component Cascade Failure".to_string(),
            description: "Test cascading failure handling".to_string(),
            faults: vec![
                FaultType::ComponentCrash {
                    component: "security".to_string(),
                    crash_type: CrashType::Immediate,
                },
                FaultType::ComponentCrash {
                    component: "database".to_string(),
                    crash_type: CrashType::Graceful,
                },
            ],
            duration_ms: 20000,
            success_criteria: SuccessCriteria {
                max_recovery_time_ms: 45000,
                max_error_rate: 0.15,
                min_availability: 0.7,
                max_response_time_degradation: 5.0,
            },
        },
        
        ChaosScenario {
            name: "Resource Exhaustion".to_string(),
            description: "Test behavior under resource constraints".to_string(),
            faults: vec![
                FaultType::MemoryExhaustion {
                    memory_mb: 512,
                    cause_oom: false,
                },
                FaultType::CpuExhaustion {
                    cpu_percent: 90,
                    thread_count: 8,
                },
            ],
            duration_ms: 25000,
            success_criteria: SuccessCriteria {
                max_recovery_time_ms: 20000,
                max_error_rate: 0.2,
                min_availability: 0.6,
                max_response_time_degradation: 10.0,
            },
        },
        
        ChaosScenario {
            name: "Byzantine Behavior".to_string(),
            description: "Test Byzantine fault tolerance".to_string(),
            faults: vec![
                FaultType::ByzantineBehavior {
                    behavior_type: ByzantineType::ConflictingMessages,
                    nodes: vec!["node1".to_string(), "node2".to_string()],
                },
            ],
            duration_ms: 30000,
            success_criteria: SuccessCriteria {
                max_recovery_time_ms: 60000,
                max_error_rate: 0.25,
                min_availability: 0.5,
                max_response_time_degradation: 8.0,
            },
        },
    ]
}

pub async fn run_chaos_scenario(framework: &mut ChaosTestFramework, scenario: &ChaosScenario) -> BearDogResult<ScenarioResult> {
    info!("🎬 Executing scenario: {} - {}", scenario.name, scenario.description);
    
    let start_time = Instant::now();
    let mut fault_results = Vec::new();

    let baseline_metrics = framework.collect_baseline_metrics().await?;

    for fault in &scenario.faults {
        let fault_result = framework.inject_and_monitor_fault(fault.clone()).await?;
        fault_results.push(fault_result);

        sleep(std::time::Duration::from_millis(500)).await;
    }

    let remaining_time = scenario.duration_ms.saturating_sub(start_time.elapsed().as_millis() as u64);
    if remaining_time > 0 {
        sleep(std::time::Duration::from_millis(remaining_time)).await;
    }

    let recovery_results = framework.validate_all_recoveries().await?;

    let post_chaos_metrics = framework.collect_current_metrics().await?;

    let scenario_success = evaluate_scenario_success(
        scenario,
        &baseline_metrics,
        &post_chaos_metrics,
        &recovery_results,
    ).await?;
    
    Ok(ScenarioResult {
        scenario_name: scenario.name.clone(),
        fault_results,
        recovery_results,
        baseline_metrics,
        post_chaos_metrics,
        success: scenario_success,
        duration_ms: start_time.elapsed().as_millis() as u64,
    })
}

pub async fn evaluate_scenario_success(
    scenario: &ChaosScenario,
    baseline: &SystemImpact,
    post_chaos: &SystemImpact,
    recovery_results: &[RecoveryResult],
) -> BearDogResult<bool> {

    let response_time_degradation = post_chaos.response_time_increase / baseline.response_time_increase.max(1.0);
    let error_rate = post_chaos.error_rate_increase;
    let availability = 1.0 - post_chaos.availability_decrease;
    
    let all_recovered = recovery_results.iter()
        .all(|r| r.status == RecoveryStatus::FullyRecovered);
    
    Ok(all_recovered &&
       response_time_degradation <= scenario.success_criteria.max_response_time_degradation &&
       error_rate <= scenario.success_criteria.max_error_rate &&
       availability >= scenario.success_criteria.min_availability)
} 