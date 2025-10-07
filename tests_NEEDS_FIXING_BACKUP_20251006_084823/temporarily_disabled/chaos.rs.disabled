use beardog_errors::BearDogError;

use beardog::{config::*, core::*, BearDogCore, BearDogResult};
use std::{collections::HashMap, sync::Arc, time::Instant};
use tracing::info;

pub use controller::*;
pub use fault_injection::*;
pub use metrics::*;
pub use models::*;
pub use recovery::*;
pub use reporting::*;
pub use scenarios::*;

pub mod controller; // Chaos controller for orchestration
pub mod fault_injection; // Fault injector trait and implementations
pub mod metrics; // Metrics collection and analysis
pub mod models; // Data structures, config, and types
pub mod recovery; // Recovery validator trait and implementations
pub mod reporting;
pub mod scenarios; // Chaos scenario definitions and execution // Result types and report generation

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
    pub fn new(core: Arc<BearDogCore>) -> Result<Self, BearDogError> {
        let config = ChaosTestConfig::default();
        let chaos_controller = Arc::new(ChaosController::new(config.clone()));
        let metrics_collector = Arc::new(ChaosMetricsCollector::new(config.metrics_interval_ms));

        let mut fault_injectors = HashMap::with_capacity(16);
        fault_injectors.insert(
            "network".to_string(),
            Arc::new(NetworkFaultInjector::new()) as Arc<dyn FaultInjector>,
        );
        fault_injectors.insert(
            "security".to_string(),
            Arc::new(SecurityFaultInjector::new()) as Arc<dyn FaultInjector>,
        );
        fault_injectors.insert(
            "database".to_string(),
            Arc::new(DatabaseFaultInjector::new()) as Arc<dyn FaultInjector>,
        );
        fault_injectors.insert(
            "resource".to_string(),
            Arc::new(ResourceFaultInjector::new()) as Arc<dyn FaultInjector>,
        );

        let mut recovery_validators = Vec::new();
        recovery_validators
            .push(Arc::new(CoreRecoveryValidator::new(core.clone())) as Arc<dyn RecoveryValidator>);
        recovery_validators
            .push(Arc::new(SecurityRecoveryValidator::new()) as Arc<dyn RecoveryValidator>);

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

    pub async fn run_chaos_testing(&mut self) -> Result<ChaosTestReport, BearDogError> {
        info!("🌪️  Starting Comprehensive Chaos Testing");

        self.metrics_collector.start_collection()?;

        let mut scenario_results = Vec::new({}", scenario.name);
            let result = self.run_chaos_scenario({:.2}",
            report.overall_resilience_score
        );

        Ok(&ChaosScenario,
    ) -> Result<ScenarioResult, BearDogError> {
        scenarios::run_chaos_scenario(Vec<ScenarioResult>,
    ) -> Result<ChaosTestReport, BearDogError> {
        reporting::generate_chaos_report(FaultType,
    ) -> Result<FaultResult, BearDogError> {
        fault_injection::inject_and_monitor_fault(self, fault)
    }

    pub async fn wait_for_recovery(&self, component: &str) -> Result<u64, BearDogError> {
        recovery::wait_for_recovery(self, component)
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

    pub async fn collect_baseline_metrics(&self) -> Result<SystemImpact, BearDogError> {
        metrics::collect_baseline_metrics()
    }

    pub async fn collect_current_metrics(&self) -> Result<SystemImpact, BearDogError> {
        metrics::collect_current_metrics()
    }

    pub async fn measure_fault_impact(&self) -> Result<SystemImpact, BearDogError> {
        metrics::measure_fault_impact()
    }

    pub async fn validate_all_recoveries(&self) -> Result<Vec<RecoveryResult, BearDogError>> {
        recovery::validate_all_recoveries(&ChaosScenario,
        baseline: &SystemImpact,
        post_chaos: &SystemImpact,
        recovery_results: &[RecoveryResult],
    ) -> Result<bool, BearDogError> {
        scenarios::evaluate_scenario_success(scenario, baseline, post_chaos, recovery_results)
    }
}
