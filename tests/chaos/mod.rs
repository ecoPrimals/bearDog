

use beardog::{
    config::*,
    core::*,
    BearDogCore, BearDogResult,
};
use std::{collections::HashMap, sync::Arc, time::Instant};
use tracing::info;

pub use models::*;
pub use fault_injection::*;
pub use recovery::*;
pub use scenarios::*;
pub use metrics::*;
pub use controller::*;
pub use reporting::*;

pub mod models;         // Data structures, config, and types
pub mod fault_injection; // Fault injector trait and implementations
pub mod recovery;       // Recovery validator trait and implementations
pub mod scenarios;      // Chaos scenario definitions and execution
pub mod metrics;        // Metrics collection and analysis
pub mod controller;     // Chaos controller for orchestration
pub mod reporting;      // Result types and report generation

pub struct ChaosTestFramework {

    pub core: Arc<BearDogCore>,

    pub chaos_controller: Arc<ChaosController>,

    pub metrics_collector: Arc<ChaosMetricsCollector>,

    pub fault_injectors: HashMap<String, Arc<dyn FaultInjector>>,

    pub recovery_validators: Vec<Arc<dyn RecoveryValidator>>,

    pub scenarios: Vec<ChaosScenario>,

    pub config: ChaosTestConfig,
}

impl ChaosTestFramework {

    pub async fn new(core: Arc<BearDogCore>) -> BearDogResult<Self> {
        let config = ChaosTestConfig::default();
        let chaos_controller = Arc::new(ChaosController::new(config.clone()));
        let metrics_collector = Arc::new(ChaosMetricsCollector::new(config.metrics_interval_ms));

        let mut fault_injectors = HashMap::with_capacity(16);
        fault_injectors.insert("network".to_string(), Arc::new(NetworkFaultInjector::new()) as Arc<dyn FaultInjector>);
        fault_injectors.insert("security".to_string(), Arc::new(SecurityFaultInjector::new()) as Arc<dyn FaultInjector>);
        fault_injectors.insert("database".to_string(), Arc::new(DatabaseFaultInjector::new()) as Arc<dyn FaultInjector>);
        fault_injectors.insert("resource".to_string(), Arc::new(ResourceFaultInjector::new()) as Arc<dyn FaultInjector>);

        let mut recovery_validators = Vec::new();
        recovery_validators.push(Arc::new(CoreRecoveryValidator::new(core.clone())) as Arc<dyn RecoveryValidator>);
        recovery_validators.push(Arc::new(SecurityRecoveryValidator::new()) as Arc<dyn RecoveryValidator>);

        let scenarios = scenarios::create_default_scenarios();

        Ok(Self {
            core,
            chaos_controller,
            metrics_collector,
            fault_injectors,
            recovery_validators,
            scenarios,
            config,
        })
    }

    pub async fn run_chaos_testing(&mut self) -> BearDogResult<ChaosTestReport> {
        info!("🌪️  Starting Comprehensive Chaos Testing");

        self.metrics_collector.start_collection().await?;
        
        let mut scenario_results = Vec::new();
        
        for scenario in &self.scenarios.clone() {
            info!("🎯 Running chaos scenario: {}", scenario.name);
            let result = self.run_chaos_scenario(scenario).await?;
            scenario_results.push(result);
        }

        self.metrics_collector.stop_collection().await?;

        let report = self.generate_chaos_report(scenario_results).await?;
        
        info!("📊 Chaos Testing Completed - Resilience Score: {:.2}", 
              report.overall_resilience_score);
        
        Ok(report)
    }

    pub async fn run_chaos_scenario(&mut self, scenario: &ChaosScenario) -> BearDogResult<ScenarioResult> {
        scenarios::run_chaos_scenario(self, scenario).await
    }

    pub async fn generate_chaos_report(&self, scenario_results: Vec<ScenarioResult>) -> BearDogResult<ChaosTestReport> {
        reporting::generate_chaos_report(self, scenario_results).await
    }

    pub async fn inject_and_monitor_fault(&mut self, fault: FaultType) -> BearDogResult<FaultResult> {
        fault_injection::inject_and_monitor_fault(self, fault).await
    }

    pub async fn wait_for_recovery(&self, component: &str) -> BearDogResult<u64> {
        recovery::wait_for_recovery(self, component).await
    }

    pub fn determine_target_component(&self, fault: &FaultType) -> String {
        fault_injection::determine_target_component(fault)
    }

    pub fn get_fault_duration(&self, fault: &FaultType) -> u64 {
        fault_injection::get_fault_duration(fault)
    }

    pub fn determine_fault_severity(&self, fault: &FaultType) -> FaultSeverity {
        fault_injection::determine_fault_severity(fault)
    }

    pub async fn collect_baseline_metrics(&self) -> BearDogResult<SystemImpact> {
        metrics::collect_baseline_metrics().await
    }

    pub async fn collect_current_metrics(&self) -> BearDogResult<SystemImpact> {
        metrics::collect_current_metrics().await
    }

    pub async fn measure_fault_impact(&self) -> BearDogResult<SystemImpact> {
        metrics::measure_fault_impact().await
    }

    pub async fn validate_all_recoveries(&self) -> BearDogResult<Vec<RecoveryResult>> {
        recovery::validate_all_recoveries(self).await
    }

    pub async fn evaluate_scenario_success(
        &self,
        scenario: &ChaosScenario,
        baseline: &SystemImpact,
        post_chaos: &SystemImpact,
        recovery_results: &[RecoveryResult],
    ) -> BearDogResult<bool> {
        scenarios::evaluate_scenario_success(scenario, baseline, post_chaos, recovery_results).await
    }
} 