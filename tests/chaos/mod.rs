// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


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

use beardog::{
    config::*,
    core::*,
    BearDogCore, BearDogResult,
};
use std::{collections::HashMap, sync::Arc, time::Instant};
use tracing::info;

// Re-export from sub-modules
pub use models::*;
pub use fault_injection::*;
pub use recovery::*;
pub use scenarios::*;
pub use metrics::*;
pub use controller::*;
pub use reporting::*;

// Sub-modules with focused responsibilities
pub mod models;         // Data structures, config, and types
pub mod fault_injection; // Fault injector trait and implementations
pub mod recovery;       // Recovery validator trait and implementations
pub mod scenarios;      // Chaos scenario definitions and execution
pub mod metrics;        // Metrics collection and analysis
pub mod controller;     // Chaos controller for orchestration
pub mod reporting;      // Result types and report generation

/// Comprehensive chaos testing framework
pub struct ChaosTestFramework {
    /// Core system under test
    pub core: Arc<BearDogCore>,
    
    /// Chaos controller for fault injection
    pub chaos_controller: Arc<ChaosController>,
    
    /// Metrics collector
    pub metrics_collector: Arc<ChaosMetricsCollector>,
    
    /// Fault injectors for different subsystems
    pub fault_injectors: HashMap<String, Arc<dyn FaultInjector>>,
    
    /// Recovery validators
    pub recovery_validators: Vec<Arc<dyn RecoveryValidator>>,
    
    /// Test scenarios
    pub scenarios: Vec<ChaosScenario>,
    
    /// Configuration
    pub config: ChaosTestConfig,
}

impl ChaosTestFramework {
    /// Create a new chaos testing framework
    pub async fn new(core: Arc<BearDogCore>) -> BearDogResult<Self> {
        let config = ChaosTestConfig::default();
        let chaos_controller = Arc::new(ChaosController::new(config.clone()));
        let metrics_collector = Arc::new(ChaosMetricsCollector::new(config.metrics_interval_ms));

        let mut fault_injectors = HashMap::new();
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

    /// Run comprehensive chaos testing
    pub async fn run_chaos_testing(&mut self) -> BearDogResult<ChaosTestReport> {
        info!("🌪️  Starting Comprehensive Chaos Testing");
        
        // Start metrics collection
        self.metrics_collector.start_collection().await?;
        
        let mut scenario_results = Vec::new();
        
        for scenario in &self.scenarios.clone() {
            info!("🎯 Running chaos scenario: {}", scenario.name);
            let result = self.run_chaos_scenario(scenario).await?;
            scenario_results.push(result);
        }
        
        // Stop metrics collection
        self.metrics_collector.stop_collection().await?;
        
        // Generate comprehensive report
        let report = self.generate_chaos_report(scenario_results).await?;
        
        info!("📊 Chaos Testing Completed - Resilience Score: {:.2}", 
              report.overall_resilience_score);
        
        Ok(report)
    }

    /// Run a specific chaos scenario
    pub async fn run_chaos_scenario(&mut self, scenario: &ChaosScenario) -> BearDogResult<ScenarioResult> {
        scenarios::run_chaos_scenario(self, scenario).await
    }

    /// Generate comprehensive chaos test report
    pub async fn generate_chaos_report(&self, scenario_results: Vec<ScenarioResult>) -> BearDogResult<ChaosTestReport> {
        reporting::generate_chaos_report(self, scenario_results).await
    }

    /// Inject a fault and monitor its impact
    pub async fn inject_and_monitor_fault(&mut self, fault: FaultType) -> BearDogResult<FaultResult> {
        fault_injection::inject_and_monitor_fault(self, fault).await
    }

    /// Wait for system recovery and measure recovery time
    pub async fn wait_for_recovery(&self, component: &str) -> BearDogResult<u64> {
        recovery::wait_for_recovery(self, component).await
    }

    /// Helper methods for fault management
    pub fn determine_target_component(&self, fault: &FaultType) -> String {
        fault_injection::determine_target_component(fault)
    }

    pub fn get_fault_duration(&self, fault: &FaultType) -> u64 {
        fault_injection::get_fault_duration(fault)
    }

    pub fn determine_fault_severity(&self, fault: &FaultType) -> FaultSeverity {
        fault_injection::determine_fault_severity(fault)
    }

    /// Helper methods for metrics collection
    pub async fn collect_baseline_metrics(&self) -> BearDogResult<SystemImpact> {
        metrics::collect_baseline_metrics().await
    }

    pub async fn collect_current_metrics(&self) -> BearDogResult<SystemImpact> {
        metrics::collect_current_metrics().await
    }

    pub async fn measure_fault_impact(&self) -> BearDogResult<SystemImpact> {
        metrics::measure_fault_impact().await
    }

    /// Helper methods for scenario evaluation
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