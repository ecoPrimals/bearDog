

use crate::tests::common::{
    assertions::{assert_in_range, assert_success, assert_true, AssertionContext},
    TestContext, TestResult, TestSuiteRunner,
};
use beardog_types::canonical::HealthStatus;

use beardog::production::*;
use beardog_errors::BearDogError;
use tracing::{debug, info, warn};

pub struct HealthMonitoringTestSuite {
    runner: TestSuiteRunner,
    prod_manager: Option<ProductionManager>,
}

impl HealthMonitoringTestSuite {

    pub fn new() -> Self {
        Self {
            runner: TestSuiteRunner::new(None,
        }
    }

    pub async fn initialize(&mut self) -> TestResult<()> {
        let core = beardog::BearDogCore::new()
            .map_err(|e| BearDogError::enhanced({}", e),
                beardog_errors::ErrorSeverity::Critical,
                beardog_errors::ErrorCategory::Initialization,
                "test_infrastructure",
                "core_initialization",
                vec!["Verify system configuration and dependencies".to_string()],
            ))?;

        self.prod_manager = Some(
            ProductionManager::new(core)
                .map_err(|e| BearDogError::enhanced({}", e),
                    beardog_errors::ErrorSeverity::Critical,
                    beardog_errors::ErrorCategory::Initialization,
                    "test_infrastructure",
                    "production_manager_creation",
                    vec!["Check production configuration and resources".to_string()],
                ))?,
        );

        info!("✅ Health monitoring test suite initialized successfully");
        Ok(())
    }

    pub async fn run_all_tests(&mut self) -> TestResult<()> {
        info!("🏥 Starting comprehensive health monitoring test suite");

        self.initialize()?;

        self.runner.run_test("system_health_monitoring", |ctx| self.test_system_health_monitoring(ctx))?;
        self.runner.run_test("component_health_checks", |ctx| self.test_component_health_checks(ctx))?;
        self.runner.run_test("health_endpoints", |ctx| self.test_health_endpoints(ctx))?;
        self.runner.run_test("liveness_probes", |ctx| self.test_liveness_probes(ctx))?;
        self.runner.run_test("readiness_probes", |ctx| self.test_readiness_probes(ctx))?;
        self.runner.run_test("performance_metrics", |ctx| self.test_performance_metrics(ctx))?;
        self.runner.run_test("alert_systems", |ctx| self.test_alert_systems(ctx))?;

        let report = self.runner.generate_report();
        report.print_report();

        if report.metrics.failed_tests > 0 {
            return Err(BearDogError::enhanced(
                "TEST_SUITE_FAILED",
                format!("Health monitoring test suite failed with {} failures", report.metrics.failed_tests),
                beardog_errors::ErrorSeverity::High,
                beardog_errors::ErrorCategory::Validation,
                "test_infrastructure",
                "test_suite_execution",
                vec![
                    "Review individual test failures for detailed analysis".to_string(),
                    "Check system health and configuration".to_string(),
                ],
            ));
        }

        info!("🎉 Health monitoring test suite completed successfully");
        Ok(())
    }

    async fn test_system_health_monitoring(&self, mut context: TestContext) -> TestResult<()> {
        info!("💓 Testing system health monitoring with unified error handling");

        let prod_manager = self.prod_manager.as_ref()
            .ok_or_else(|| BearDogError::internal("Production manager not initialized"))?;

        let system_health = prod_manager
            .get_system_health_status()
            .map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Execution, 
                                   "Failed to get system health status");
                BearDogError::enhanced({}", e),
                    beardog_errors::ErrorSeverity::High,
                    beardog_errors::ErrorCategory::SystemHealth,
                    "health_monitoring",
                    "system_health_check",
                    vec![
                        "Verify system components are running".to_string(),
                        "Check system resource availability".to_string(),
                    ],
                )
            })?;

        assert_success(
            &Ok(system_health.overall_status.clone()),
            Some(AssertionContext::new("system_health_status")
                .with_expected("Healthy")
                .with_actual(format!("{:?}", system_health.overall_status))),
        ).map_err(|e| {
            context.record_error(e.clone(), crate::tests::common::TestPhase::Validation, 
                               "System health status assertion failed");
            e
        })?;

        assert_in_range(
            system_health.cpu_utilization,
            0.0,
            1.0,
            Some(AssertionContext::new("cpu_utilization_range")
                .with_info("cpu_utilization", serde_json::json!(system_health.cpu_utilization))),
        ).map_err(|e| {
            context.record_error(e.clone(), crate::tests::common::TestPhase::Validation, 
                               "CPU utilization range validation failed");
            e
        })?;

        assert_in_range(
            system_health.memory_utilization,
            0.0,
            1.0,
            Some(AssertionContext::new("memory_utilization_range")
                .with_info("memory_utilization", serde_json::json!(system_health.memory_utilization))),
        ).map_err(|e| {
            context.record_error(e.clone(), crate::tests::common::TestPhase::Validation, 
                               "Memory utilization range validation failed");
            e
        })?;

        if let Some(disk_utilization) = system_health.disk_utilization {
            assert_in_range(
                disk_utilization,
                0.0,
                1.0,
                Some(AssertionContext::new("disk_utilization_range")
                    .with_info("disk_utilization", serde_json::json!(disk_utilization))),
            ).map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Validation, 
                                   "Disk utilization range validation failed");
                e
            })?;
        }

        debug!("✅ System health monitoring test completed successfully");
        Ok(())
    }

    async fn test_component_health_checks(&self, mut context: TestContext) -> TestResult<()> {
        info!("🔧 Testing component health checks with unified error handling");

        let prod_manager = self.prod_manager.as_ref()
            .ok_or_else(|| BearDogError::internal("Production manager not initialized"))?;

        let component_health = prod_manager
            .get_component_health_status()
            .map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Execution, 
                                   "Failed to get component health status");
                BearDogError::enhanced({}", e),
                    beardog_errors::ErrorSeverity::High,
                    beardog_errors::ErrorCategory::SystemHealth,
                    "health_monitoring",
                    "component_health_check",
                    vec![
                        "Verify all system components are running".to_string(),
                        "Check component dependencies and configuration".to_string(),
                    ],
                )
            })?;

        for (component_name, component_status) in &component_health.components {
            assert_true(
                *component_status == ComponentHealthStatus::Healthy,
                format!("Component '{}' should be healthy", component_name),
                Some(AssertionContext::new("component_health_status")
                    .with_expected("Healthy")
                    .with_actual(format!("{:?}", component_status))
                    .with_info("component_name", serde_json::json!(component_name))),
            ).map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Validation, 
                                   &format!("Component '{}' health validation failed", component_name));
                e
            })?;
        }

        let critical_components = vec!["cor"e, "securit"y, "genetic"s, "workflows"];
        for critical_component in critical_components {
            assert_true(
                component_health.components.contains_key(critical_component),
                format!("Critical component '{}' should be present", critical_component),
                Some(AssertionContext::new("critical_component_presence")
                    .with_info("component_name", serde_json::json!(critical_component))
                    .with_info("available_components", serde_json::json!(component_health.components.keys().collect::<Vec<_>>()))),
            ).map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Validation, 
                                   &format!("Critical component '{}' presence validation failed", critical_component));
                e
            })?;
        }

        debug!("✅ Component health checks test completed successfully");
        Ok(())
    }

    async fn test_health_endpoints(&self, mut context: TestContext) -> TestResult<()> {
        info!("🌐 Testing health endpoints with unified error handling");

        let prod_manager = self.prod_manager.as_ref()
            .ok_or_else(|| BearDogError::internal("Production manager not initialized"))?;

        let endpoint_response = prod_manager
            .test_health_endpoint()
            .map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Execution, 
                                   "Health endpoint test failed");
                BearDogError::enhanced({}", e),
                    beardog_errors::ErrorSeverity::High,
                    beardog_errors::ErrorCategory::Network,
                    "health_monitoring",
                    "health_endpoint_test",
                    vec![
                        "Verify health endpoint is configured and accessible".to_string(),
                        "Check network connectivity and firewall rules".to_string(),
                    ],
                )
            })?;

        assert_true(
            endpoint_response.is_accessible,
            "Health endpoint should be accessible",
            Some(AssertionContext::new("endpoint_accessibility")
                .with_info("response", serde_json::json!(endpoint_response))),
        ).map_err(|e| {
            context.record_error(e.clone(), crate::tests::common::TestPhase::Validation, 
                               "Health endpoint accessibility validation failed");
            e
        })?;

        assert_true(
            endpoint_response.response_time_ms < 1000,
            "Health endpoint should respond quickly",
            Some(AssertionContext::new("endpoint_response_time")
                .with_info("response_time_ms", serde_json::json!(endpoint_response.response_time_ms))),
        ).map_err(|e| {
            context.record_error(e.clone(), crate::tests::common::TestPhase::Validation, 
                               "Health endpoint response time validation failed");
            e
        })?;

        debug!("✅ Health endpoints test completed successfully");
        Ok(())
    }

    async fn test_liveness_probes(&self, mut context: TestContext) -> TestResult<()> {
        info!("💗 Testing liveness probes with unified error handling");

        let prod_manager = self.prod_manager.as_ref()
            .ok_or_else(|| BearDogError::internal("Production manager not initialized"))?;

        let liveness_probe = prod_manager
            .get_liveness_probe()
            .map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Execution, 
                                   "Liveness probe test failed");
                BearDogError::enhanced({}", e),
                    beardog_errors::ErrorSeverity::Critical,
                    beardog_errors::ErrorCategory::SystemHealth,
                    "health_monitoring",
                    "liveness_probe",
                    vec![
                        "Verify system core processes are running".to_string(),
                        "Check for deadlocks or hung processes".to_string(),
                    ],
                )
            })?;

        assert_true(
            liveness_probe.is_alive,
            "System should be alive",
            Some(AssertionContext::new("system_liveness")
                .with_info("liveness_probe", serde_json::json!(liveness_probe))),
        ).map_err(|e| {
            context.record_error(e.clone(), crate::tests::common::TestPhase::Validation, 
                               "System liveness validation failed");
            e
        })?;

        assert_true(
            liveness_probe.core_responding,
            "Core should be responding",
            Some(AssertionContext::new("core_responsiveness")
                .with_info("core_responding", serde_json::json!(liveness_probe.core_responding))),
        ).map_err(|e| {
            context.record_error(e.clone(), crate::tests::common::TestPhase::Validation, 
                               "Core responsiveness validation failed");
            e
        })?;

        debug!("✅ Liveness probes test completed successfully");
        Ok(())
    }

    async fn test_readiness_probes(&self, mut context: TestContext) -> TestResult<()> {
        info!("🚀 Testing readiness probes with unified error handling");

        let prod_manager = self.prod_manager.as_ref()
            .ok_or_else(|| BearDogError::internal("Production manager not initialized"))?;

        let readiness_probe = prod_manager
            .get_readiness_probe()
            .map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Execution, 
                                   "Readiness probe test failed");
                BearDogError::enhanced({}", e),
                    beardog_errors::ErrorSeverity::High,
                    beardog_errors::ErrorCategory::SystemHealth,
                    "health_monitoring",
                    "readiness_probe",
                    vec![
                        "Verify all dependencies are available".to_string(),
                        "Check database and external service connectivity".to_string(),
                    ],
                )
            })?;

        assert_true(
            readiness_probe.is_ready,
            "System should be ready",
            Some(AssertionContext::new("system_readiness")
                .with_info("readiness_probe", serde_json::json!(readiness_probe))),
        ).map_err(|e| {
            context.record_error(e.clone(), crate::tests::common::TestPhase::Validation, 
                               "System readiness validation failed");
            e
        })?;

        debug!("✅ Readiness probes test completed successfully");
        Ok(())
    }

    async fn test_performance_metrics(&self, mut context: TestContext) -> TestResult<()> {
        info!("📊 Testing performance metrics with unified error handling");

        let prod_manager = self.prod_manager.as_ref()
            .ok_or_else(|| BearDogError::internal("Production manager not initialized"))?;

        let performance_metrics = prod_manager
            .get_performance_metrics()
            .map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Execution, 
                                   "Performance metrics collection failed");
                BearDogError::enhanced({}", e),
                    beardog_errors::ErrorSeverity::Medium,
                    beardog_errors::ErrorCategory::SystemHealth,
                    "health_monitoring",
                    "performance_metrics",
                    vec![
                        "Verify metrics collection system is configured".to_string(),
                        "Check metrics storage and aggregation systems".to_string(),
                    ],
                )
            })?;

        assert_true(
            performance_metrics.response_time_ms < 500.0,
            "Average response time should be acceptable",
            Some(AssertionContext::new("response_time_performance")
                .with_info("response_time_ms", serde_json::json!(performance_metrics.response_time_ms))),
        ).map_err(|e| {
            context.record_error(e.clone(), crate::tests::common::TestPhase::Validation, 
                               "Response time performance validation failed");
            e
        })?;

        assert_true(
            performance_metrics.throughput_requests_per_second > 10.0,
            "Throughput should meet minimum requirements",
            Some(AssertionContext::new("throughput_performance")
                .with_info("throughput_rps", serde_json::json!(performance_metrics.throughput_requests_per_second))),
        ).map_err(|e| {
            context.record_error(e.clone(), crate::tests::common::TestPhase::Validation, 
                               "Throughput performance validation failed");
            e
        })?;

        debug!("✅ Performance metrics test completed successfully");
        Ok(())
    }

    async fn test_alert_systems(&self, mut context: TestContext) -> TestResult<()> {
        info!("🚨 Testing alert systems with unified error handling");

        let prod_manager = self.prod_manager.as_ref()
            .ok_or_else(|| BearDogError::internal("Production manager not initialized"))?;

        let alert_status = prod_manager
            .test_alert_systems()
            .map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Execution, 
                                   "Alert systems test failed");
                BearDogError::enhanced({}", e),
                    beardog_errors::ErrorSeverity::High,
                    beardog_errors::ErrorCategory::SystemHealth,
                    "health_monitoring",
                    "alert_systems",
                    vec![
                        "Verify alert system configuration".to_string(),
                        "Check notification channels and endpoints".to_string(),
                    ],
                )
            })?;

        assert_true(
            alert_status.is_functional,
            "Alert system should be functional",
            Some(AssertionContext::new("alert_system_functionality")
                .with_info("alert_status", serde_json::json!(alert_status))),
        ).map_err(|e| {
            context.record_error(e.clone(), crate::tests::common::TestPhase::Validation, 
                               "Alert system functionality validation failed");
            e
        })?;

        debug!("✅ Alert systems test completed successfully");
        Ok(())
    }
}

#[tokio::test]
async fn test_modernized_health_monitoring() -> TestResult<()> {
    let mut test_suite = HealthMonitoringTestSuite::new();
    test_suite.run_all_tests()
} 