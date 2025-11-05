// Chaos Testing Framework - Main Module
// Migrated October 7, 2025 - Updated for modular architecture

pub mod controller;
pub mod fault_injection;
pub mod metrics;
pub mod models;
pub mod recovery;
pub mod reporting;
pub mod scenarios;

// Specialized chaos test modules
pub mod network_chaos;
pub mod resource_chaos;
pub mod comprehensive_fault_testing;
pub mod integration_tests;

// Re-export commonly used types
pub use controller::{ChaosController, ChaosStatistics, FaultEvent};
pub use fault_injection::{
    DatabaseFaultInjector, FaultInjector, NetworkFaultInjector, 
    ResourceFaultInjector, SecurityFaultInjector,
};
pub use metrics::{ChaosMetricsCollector, MetricsTrend};
pub use models::*;
pub use recovery::{
    CoreRecoveryValidator, DatabaseRecoveryValidator, NetworkRecoveryValidator,
    RecoveryValidator, SecurityRecoveryValidator,
};
pub use reporting::{generate_chaos_report, print_chaos_summary};
pub use scenarios::{create_default_scenarios, run_chaos_scenario};

use beardog_errors::BearDogError;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

/// Main chaos testing framework
pub struct ChaosTestFramework {
    pub chaos_controller: Arc<ChaosController>,
    pub metrics_collector: Arc<tokio::sync::RwLock<ChaosMetricsCollector>>,
    pub fault_injectors: HashMap<String, Arc<dyn FaultInjector>>,
    pub recovery_validators: Vec<Arc<dyn RecoveryValidator>>,
    pub scenarios: Vec<ChaosScenario>,
    pub config: ChaosTestConfig,
}

impl ChaosTestFramework {
    /// Create a new chaos testing framework
    pub fn new(config: ChaosTestConfig) -> Result<Self, BearDogError> {
        let chaos_controller = Arc::new(ChaosController::new(config.clone()));
        let metrics_collector = Arc::new(tokio::sync::RwLock::new(
            ChaosMetricsCollector::new(config.metrics_interval_ms)
        ));

        // Initialize fault injectors
        let mut fault_injectors = HashMap::with_capacity(4);
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

        // Initialize recovery validators
        let mut recovery_validators: Vec<Arc<dyn RecoveryValidator>> = Vec::new();
        recovery_validators.push(Arc::new(CoreRecoveryValidator::new("core".to_string())));
        recovery_validators.push(Arc::new(SecurityRecoveryValidator::new()));
        recovery_validators.push(Arc::new(NetworkRecoveryValidator::new()));
        recovery_validators.push(Arc::new(DatabaseRecoveryValidator::new()));

        // Load default scenarios
        let scenarios = create_default_scenarios();

        Ok(Self {
            chaos_controller,
            metrics_collector,
            fault_injectors,
            recovery_validators,
            scenarios,
            config,
        })
    }

    /// Run comprehensive chaos testing
    pub async fn run_chaos_testing(&self) -> Result<ChaosTestReport, BearDogError> {
        info!("🌪️  Starting Comprehensive Chaos Testing");
        info!("   Scenarios to run: {}", self.scenarios.len());

        // Start metrics collection
        {
            let mut metrics = self.metrics_collector.write().await;
            metrics.start_collection()?;
        }

        // Start the chaos controller
        self.chaos_controller.start();

        let start_time = std::time::SystemTime::now();
        let mut scenario_results = Vec::new();

        // Run each scenario
        for scenario in &self.scenarios {
            info!("Running scenario: {}", scenario.name);
            
            match run_chaos_scenario(scenario, &self.fault_injectors).await {
                Ok(result) => {
                    info!("Scenario '{}' completed: {}", scenario.name, if result.success { "SUCCESS" } else { "FAILED" });
                    scenario_results.push(result);
                }
                Err(e) => {
                    tracing::error!("Scenario '{}' failed: {}", scenario.name, e);
                    scenario_results.push(ScenarioResult {
                        scenario_name: scenario.name.clone(),
                        success: false,
                        duration_ms: 0,
                        faults_injected: vec![],
                        recovery_results: vec![],
                        impact_summary: SystemImpact::default(),
                        failure_reasons: vec![format!("Scenario execution failed: {}", e)],
                    });
                }
            }
        }

        // Stop the chaos controller
        self.chaos_controller.stop();

        // Stop metrics collection
        {
            let mut metrics = self.metrics_collector.write().await;
            metrics.stop_collection();
        }

        // Generate report
        let report = generate_chaos_report(scenario_results, start_time);

        info!("🌪️  Chaos Testing Complete");
        info!("   Overall Resilience Score: {:.2}%", report.overall_resilience_score);

        Ok(report)
    }

    /// Add a custom scenario
    pub fn add_scenario(&mut self, scenario: ChaosScenario) {
        self.scenarios.push(scenario);
    }

    /// Add a custom fault injector
    pub fn add_fault_injector(&mut self, name: String, injector: Arc<dyn FaultInjector>) {
        self.fault_injectors.insert(name, injector);
    }

    /// Add a custom recovery validator
    pub fn add_recovery_validator(&mut self, validator: Arc<dyn RecoveryValidator>) {
        self.recovery_validators.push(validator);
    }

    /// Get the chaos controller
    pub fn controller(&self) -> &Arc<ChaosController> {
        &self.chaos_controller
    }

    /// Get the configuration
    pub fn config(&self) -> &ChaosTestConfig {
        &self.config
    }
}

/// Default implementation
impl Default for ChaosTestFramework {
    fn default() -> Self {
        Self::new(ChaosTestConfig::default())
            .expect("Failed to create default chaos framework")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chaos_framework_creation() {
        let config = ChaosTestConfig::default();
        let framework = ChaosTestFramework::new(config);
        assert!(framework.is_ok());
    }

    #[tokio::test]
    async fn test_chaos_controller_lifecycle() {
        let config = ChaosTestConfig::default();
        let controller = ChaosController::new(config);
        
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(!controller.is_running());
        controller.start();
        assert!(controller.is_running());
        controller.stop();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(!controller.is_running());
    }

    #[tokio::test]
    async fn test_metrics_collection() {
        let mut collector = ChaosMetricsCollector::new(1000);
        assert!(collector.start_collection().is_ok());
        
        let baseline = collector.get_baseline();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: core
        // TEST_PRIORITY: normal
        assert!(baseline.is_some());
    }

    #[tokio::test]
    async fn test_fault_injection_helpers() {
        let fault = FaultType::NetworkPartition { duration_ms: 1000 };
         // TEST_CATEGORY: unit
         // TEST_DOMAIN: core
         // TEST_PRIORITY: normal
        
        let component = fault_injection::determine_target_component(&fault);
        assert_eq!(component, "network");
        
        let duration = fault_injection::get_fault_duration(&fault);
        assert_eq!(duration, 1000);
        
        let severity = fault_injection::determine_fault_severity(&fault);
        assert!(matches!(severity, FaultSeverity::High));
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: core
    // TEST_PRIORITY: normal
    #[tokio::test]
    async fn test_default_scenarios() {
        let scenarios = create_default_scenarios();
        assert!(!scenarios.is_empty());
        assert!(scenarios.len() >= 5);
    }
}

