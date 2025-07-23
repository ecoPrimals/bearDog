//! Chaos Testing Framework for BearDog
//!
//! **Advanced Fault Injection & Resilience Validation**
//!
//! This framework provides comprehensive chaos testing capabilities:
//! - Network partitions and latency injection
//! - Component failure simulation
//! - Resource exhaustion testing
//! - Byzantine fault tolerance validation
//! - Recovery time measurement
//! - System degradation analysis

// Use the new modular chaos testing structure
mod chaos;

// Re-export everything from the chaos module
pub use chaos::*;

// Chaos testing integration tests

#[tokio::test]
async fn test_chaos_framework_initialization() -> beardog::BearDogResult<()> {
    use beardog::{config::BearDogConfig, core::BearDogCore};
    use std::sync::Arc;
    
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);
    
    let chaos_framework = ChaosTestFramework::new(core).await?;
    
    assert!(!chaos_framework.fault_injectors.is_empty());
    assert!(!chaos_framework.recovery_validators.is_empty());
    assert!(!chaos_framework.scenarios.is_empty());
    
    tracing::info!("✅ Chaos framework initialization test passed");
    Ok(())
}

#[tokio::test]
async fn test_network_fault_injection() -> beardog::BearDogResult<()> {
    let network_injector = NetworkFaultInjector::new();
    
    let fault = FaultType::NetworkPartition {
        segments: vec!["test1".to_string(), "test2".to_string()],
        duration_ms: 1000,
    };
    
    let fault_id = network_injector.inject_fault(fault).await?;
    assert!(!fault_id.is_empty());
    
    network_injector.remove_fault(&fault_id).await?;
    
    tracing::info!("✅ Network fault injection test passed");
    Ok(())
}

#[tokio::test]
async fn test_chaos_scenario_execution() -> beardog::BearDogResult<()> {
    use beardog::{config::BearDogConfig, core::BearDogCore};
    use std::sync::Arc;
    
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);
    let mut chaos_framework = ChaosTestFramework::new(core).await?;
    
    // Create a simple test scenario
    let test_scenario = ChaosScenario {
        name: "Simple Network Test".to_string(),
        description: "Basic network fault test".to_string(),
        faults: vec![
            FaultType::NetworkLatency {
                latency_ms: 100,
                packet_loss: 0.01,
            }
        ],
        duration_ms: 2000,
        success_criteria: SuccessCriteria {
            max_recovery_time_ms: 5000,
            max_error_rate: 0.1,
            min_availability: 0.9,
            max_response_time_degradation: 2.0,
        },
    };
    
    let result = chaos_framework.run_chaos_scenario(&test_scenario).await?;
    
    assert_eq!(result.scenario_name, "Simple Network Test");
    assert!(!result.fault_results.is_empty());
    
    tracing::info!("✅ Chaos scenario execution test passed");
    Ok(())
}

#[tokio::test]
async fn test_comprehensive_chaos_testing() -> beardog::BearDogResult<()> {
    use beardog::{config::BearDogConfig, core::BearDogCore};
    use std::sync::Arc;
    
    let config = BearDogConfig::default();
    let core = Arc::new(BearDogCore::new(config).await?);
    let mut chaos_framework = ChaosTestFramework::new(core).await?;
    
    let report = chaos_framework.run_chaos_testing().await?;
    
    assert!(report.overall_resilience_score >= 0.0);
    assert!(report.overall_resilience_score <= 100.0);
    assert!(!report.scenario_results.is_empty());
    assert!(!report.recommendations.is_empty());
    
    tracing::info!("🎉 Comprehensive chaos testing completed with resilience score: {:.2}%", 
          report.overall_resilience_score);
    
    Ok(())
} 