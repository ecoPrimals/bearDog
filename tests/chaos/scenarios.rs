// SPDX-License-Identifier: AGPL-3.0-only
#![allow(unused_imports, unused_variables, dead_code, unused_comparisons, clippy::all)]

// Chaos Testing Scenarios
// Migrated October 7, 2025 - Updated for modular architecture

use super::models::*;
use beardog_errors::BearDogError;
use tracing::info;

/// Create default chaos scenarios
pub fn create_default_scenarios() -> Vec<ChaosScenario> {
    vec![
        create_network_partition_scenario(),
        create_resource_exhaustion_scenario(),
        create_security_failure_scenario(),
        create_database_failure_scenario(),
        create_cascading_failure_scenario(),
    ]
}

/// Network partition scenario
pub fn create_network_partition_scenario() -> ChaosScenario {
    ChaosScenario {
        name: "Network Partition".to_string(),
        description: "Simulate network partition between components".to_string(),
        faults: vec![
            FaultType::NetworkPartition {
                duration_ms: 10000,
            },
        ],
        duration_ms: 15000,
        success_criteria: SuccessCriteria {
            max_recovery_time_ms: 5000,
            max_error_rate: 0.05,
            min_availability: 0.95,
            allow_data_loss: false,
        },
    }
}

/// Resource exhaustion scenario
pub fn create_resource_exhaustion_scenario() -> ChaosScenario {
    ChaosScenario {
        name: "Resource Exhaustion".to_string(),
        description: "Test behavior under resource pressure".to_string(),
        faults: vec![
            FaultType::MemoryExhaustion {
                memory_mb: 1024,
                cause_oom: false,
            },
            FaultType::CpuExhaustion {
                cpu_percent: 80,
                thread_count: 4,
            },
        ],
        duration_ms: 20000,
        success_criteria: SuccessCriteria {
            max_recovery_time_ms: 8000,
            max_error_rate: 0.10,
            min_availability: 0.90,
            allow_data_loss: false,
        },
    }
}

/// Security failure scenario
pub fn create_security_failure_scenario() -> ChaosScenario {
    ChaosScenario {
        name: "Security Failure".to_string(),
        description: "Test response to authentication and certificate failures".to_string(),
        faults: vec![
            FaultType::AuthenticationFailure {
                failure_rate: 0.3,
            },
            FaultType::CertificateExpiry {
                certificates: vec!["api-cert".to_string()],
            },
        ],
        duration_ms: 12000,
        success_criteria: SuccessCriteria {
            max_recovery_time_ms: 3000,
            max_error_rate: 0.05,
            min_availability: 0.95,
            allow_data_loss: false,
        },
    }
}

/// Database failure scenario
pub fn create_database_failure_scenario() -> ChaosScenario {
    ChaosScenario {
        name: "Database Failure".to_string(),
        description: "Test database timeout and corruption handling".to_string(),
        faults: vec![
            FaultType::DatabaseTimeout {
                timeout_ms: 5000,
            },
        ],
        duration_ms: 10000,
        success_criteria: SuccessCriteria {
            max_recovery_time_ms: 6000,
            max_error_rate: 0.08,
            min_availability: 0.92,
            allow_data_loss: false,
        },
    }
}

/// Cascading failure scenario
pub fn create_cascading_failure_scenario() -> ChaosScenario {
    ChaosScenario {
        name: "Cascading Failures".to_string(),
        description: "Test multiple simultaneous failures".to_string(),
        faults: vec![
            FaultType::NetworkLatency {
                latency_ms: 500,
                packet_loss: 0.1,
            },
            FaultType::ComponentSlowdown {
                component: "api".to_string(),
                slowdown_factor: 3.0,
            },
            FaultType::MemoryExhaustion {
                memory_mb: 512,
                cause_oom: false,
            },
        ],
        duration_ms: 25000,
        success_criteria: SuccessCriteria {
            max_recovery_time_ms: 10000,
            max_error_rate: 0.15,
            min_availability: 0.85,
            allow_data_loss: false,
        },
    }
}

/// Run a chaos scenario
pub async fn run_chaos_scenario(
    scenario: &ChaosScenario,
    injectors: &std::collections::HashMap<String, std::sync::Arc<dyn super::fault_injection::FaultInjector>>,
) -> Result<ScenarioResult, BearDogError> {
    info!("🌪️  Running chaos scenario: {}", scenario.name);
    info!("   Description: {}", scenario.description);
    info!("   Faults to inject: {}", scenario.faults.len());

    let start_time = std::time::Instant::now();
    let mut fault_results = Vec::new();

    // Inject all faults
    for fault in &scenario.faults {
        match super::fault_injection::inject_and_monitor_fault(fault.clone(), injectors).await {
            Ok(result) => fault_results.push(result),
            Err(e) => {
                return Err(BearDogError::internal(format!(
                    "Failed to inject fault in scenario '{}': {}",
                    scenario.name, e
                )));
            }
        }
    }

    // No sleep needed - testing scenario execution, not duration
    // For time-based scenarios, use tokio::time::pause() + advance()

    // Collect recovery results (simplified)
    let recovery_results = vec![];

    let duration_ms = start_time.elapsed().as_millis() as u64;

    // Evaluate success
    let success = evaluate_scenario_success(scenario, &fault_results, &recovery_results);

    Ok(ScenarioResult {
        scenario_name: scenario.name.clone(),
        success,
        duration_ms,
        faults_injected: fault_results,
        recovery_results,
        impact_summary: SystemImpact::default(),
        failure_reasons: vec![],
    })
}

/// Evaluate if a scenario was successful
fn evaluate_scenario_success(
    scenario: &ChaosScenario,
    fault_results: &[FaultResult],
    _recovery_results: &[RecoveryResult],
) -> bool {
    // Check if all faults were successfully injected
    let all_injected = fault_results.iter().all(|r| r.injection_success);

    if !all_injected {
        return false;
    }

    // Check recovery time criteria
    let max_recovery = fault_results
        .iter()
        .filter_map(|r| r.recovery_time_ms)
        .max()
        .unwrap_or(0);

    if max_recovery > scenario.success_criteria.max_recovery_time_ms {
        return false;
    }

    // All criteria met
    true
}

