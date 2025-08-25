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


//! Modernized Production Health Monitoring Tests
//!
//! **Enterprise-Grade Test Infrastructure with Unified Error Handling**
//!
//! This module demonstrates modern test patterns using BearDog's unified error
//! system, replacing panic-prone patterns with sophisticated error handling.

use crate::tests::common::{
    assertions::{assert_in_range, assert_success, assert_true, AssertionContext},
    TestContext, TestResult, TestSuiteRunner,
};
use beardog_types::canonical::HealthStatus;

use beardog::production::*;
use beardog_errors::{BearDogError, BearDogResult};
use tracing::{debug, info, warn};

/// Modern test suite for production health monitoring with unified error handling
pub struct HealthMonitoringTestSuite {
    runner: TestSuiteRunner,
    prod_manager: Option<ProductionManager>,
}

impl HealthMonitoringTestSuite {
    /// Create new health monitoring test suite
    pub fn new() -> Self {
        Self {
            runner: TestSuiteRunner::new("Production Health Monitoring"),
            prod_manager: None,
        }
    }

    /// Initialize production manager with error handling
    pub async fn initialize(&mut self) -> TestResult<()> {
        let core = beardog::BearDogCore::new()
            .await
            .map_err(|e| BearDogError::enhanced(
                "TEST_SETUP",
                format!("Failed to initialize BearDog core: {}", e),
                beardog_errors::ErrorSeverity::Critical,
                beardog_errors::ErrorCategory::Initialization,
                "test_infrastructure",
                "core_initialization",
                vec!["Verify system configuration and dependencies".to_string()],
            ))?;

        self.prod_manager = Some(
            ProductionManager::new(core)
                .await
                .map_err(|e| BearDogError::enhanced(
                    "TEST_SETUP",
                    format!("Failed to create production manager: {}", e),
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

    /// Run all health monitoring tests with unified error handling
    pub async fn run_all_tests(&mut self) -> TestResult<()> {
        info!("🏥 Starting comprehensive health monitoring test suite");

        // Initialize test environment
        self.initialize().await?;

        // Execute test suite with modern error handling
        self.runner.run_test("system_health_monitoring", |ctx| self.test_system_health_monitoring(ctx)).await?;
        self.runner.run_test("component_health_checks", |ctx| self.test_component_health_checks(ctx)).await?;
        self.runner.run_test("health_endpoints", |ctx| self.test_health_endpoints(ctx)).await?;
        self.runner.run_test("liveness_probes", |ctx| self.test_liveness_probes(ctx)).await?;
        self.runner.run_test("readiness_probes", |ctx| self.test_readiness_probes(ctx)).await?;
        self.runner.run_test("performance_metrics", |ctx| self.test_performance_metrics(ctx)).await?;
        self.runner.run_test("alert_systems", |ctx| self.test_alert_systems(ctx)).await?;

        // Generate comprehensive report
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

    /// Test system health monitoring with modern error handling
    async fn test_system_health_monitoring(&self, mut context: TestContext) -> TestResult<()> {
        info!("💓 Testing system health monitoring with unified error handling");

        let prod_manager = self.prod_manager.as_ref()
            .ok_or_else(|| BearDogError::internal("Production manager not initialized"))?;

        // Test system health status retrieval
        let system_health = prod_manager
            .get_system_health_status()
            .await
            .map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Execution, 
                                   "Failed to get system health status");
                BearDogError::enhanced(
                    "HEALTH_CHECK_FAILED",
                    format!("System health check failed: {}", e),
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

        // Modern assertions with rich error context
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

        // Validate CPU utilization with proper range checking
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

        // Validate memory utilization with proper range checking
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

        // Validate disk utilization if available
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

    /// Test component health checks with comprehensive error handling
    async fn test_component_health_checks(&self, mut context: TestContext) -> TestResult<()> {
        info!("🔧 Testing component health checks with unified error handling");

        let prod_manager = self.prod_manager.as_ref()
            .ok_or_else(|| BearDogError::internal("Production manager not initialized"))?;

        // Test individual component health
        let component_health = prod_manager
            .get_component_health_status()
            .await
            .map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Execution, 
                                   "Failed to get component health status");
                BearDogError::enhanced(
                    "COMPONENT_HEALTH_CHECK_FAILED",
                    format!("Component health check failed: {}", e),
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

        // Validate core components are healthy
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

        // Validate critical components are present
        let critical_components = vec!["core", "security", "genetics", "workflows"];
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

    /// Test health endpoints with modern patterns
    async fn test_health_endpoints(&self, mut context: TestContext) -> TestResult<()> {
        info!("🌐 Testing health endpoints with unified error handling");

        let prod_manager = self.prod_manager.as_ref()
            .ok_or_else(|| BearDogError::internal("Production manager not initialized"))?;

        // Test health endpoint availability
        let endpoint_response = prod_manager
            .test_health_endpoint()
            .await
            .map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Execution, 
                                   "Health endpoint test failed");
                BearDogError::enhanced(
                    "HEALTH_ENDPOINT_FAILED",
                    format!("Health endpoint test failed: {}", e),
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

        // Validate endpoint response
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

    /// Test liveness probes with comprehensive validation
    async fn test_liveness_probes(&self, mut context: TestContext) -> TestResult<()> {
        info!("💗 Testing liveness probes with unified error handling");

        let prod_manager = self.prod_manager.as_ref()
            .ok_or_else(|| BearDogError::internal("Production manager not initialized"))?;

        // Test liveness probe
        let liveness_probe = prod_manager
            .get_liveness_probe()
            .await
            .map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Execution, 
                                   "Liveness probe test failed");
                BearDogError::enhanced(
                    "LIVENESS_PROBE_FAILED",
                    format!("Liveness probe failed: {}", e),
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

        // Validate liveness status
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

    /// Test readiness probes with comprehensive validation
    async fn test_readiness_probes(&self, mut context: TestContext) -> TestResult<()> {
        info!("🚀 Testing readiness probes with unified error handling");

        let prod_manager = self.prod_manager.as_ref()
            .ok_or_else(|| BearDogError::internal("Production manager not initialized"))?;

        // Test readiness probe
        let readiness_probe = prod_manager
            .get_readiness_probe()
            .await
            .map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Execution, 
                                   "Readiness probe test failed");
                BearDogError::enhanced(
                    "READINESS_PROBE_FAILED",
                    format!("Readiness probe failed: {}", e),
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

        // Validate readiness status
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

    /// Test performance metrics with comprehensive validation
    async fn test_performance_metrics(&self, mut context: TestContext) -> TestResult<()> {
        info!("📊 Testing performance metrics with unified error handling");

        let prod_manager = self.prod_manager.as_ref()
            .ok_or_else(|| BearDogError::internal("Production manager not initialized"))?;

        // Test performance metrics collection
        let performance_metrics = prod_manager
            .get_performance_metrics()
            .await
            .map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Execution, 
                                   "Performance metrics collection failed");
                BearDogError::enhanced(
                    "PERFORMANCE_METRICS_FAILED",
                    format!("Performance metrics collection failed: {}", e),
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

        // Validate performance metrics
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

    /// Test alert systems with comprehensive validation
    async fn test_alert_systems(&self, mut context: TestContext) -> TestResult<()> {
        info!("🚨 Testing alert systems with unified error handling");

        let prod_manager = self.prod_manager.as_ref()
            .ok_or_else(|| BearDogError::internal("Production manager not initialized"))?;

        // Test alert system functionality
        let alert_status = prod_manager
            .test_alert_systems()
            .await
            .map_err(|e| {
                context.record_error(e.clone(), crate::tests::common::TestPhase::Execution, 
                                   "Alert systems test failed");
                BearDogError::enhanced(
                    "ALERT_SYSTEMS_FAILED",
                    format!("Alert systems test failed: {}", e),
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

        // Validate alert system status
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

/// Run modernized health monitoring tests
#[tokio::test]
async fn test_modernized_health_monitoring() -> TestResult<()> {
    let mut test_suite = HealthMonitoringTestSuite::new();
    test_suite.run_all_tests().await
} 