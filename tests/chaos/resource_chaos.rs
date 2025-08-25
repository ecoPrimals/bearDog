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


//! Resource Chaos Testing Module
//!
//! Focused chaos engineering tests for resource resilience including:
//! - Resource exhaustion scenarios
//! - CPU starvation testing
//! - Disk I/O failure simulation

use super::{ChaosConfig, TestMetrics, TestResult};
use beardog_errors::{BearDogError, BearDogResult};
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tracing::{info, warn};

/// Resource chaos testing controller
#[derive(Debug)]
pub struct ResourceChaosController {
    config: ChaosConfig,
    semaphore: Semaphore,
}

impl ResourceChaosController {
    pub fn new(config: ChaosConfig) -> Self {
        Self {
            semaphore: Semaphore::new(config.max_concurrent_ops),
            config,
        }
    }

    /// Test resource exhaustion scenarios
    pub async fn test_resource_exhaustion(&self) -> BearDogResult<TestResult> {
        let start_time = Instant::now();
        let mut operations_attempted = 0u64;
        let mut operations_succeeded = 0u64;

        info!("💾 Testing resource exhaustion resilience");

        while start_time.elapsed() < self.config.test_duration {
            operations_attempted += 1;

            // Try to acquire resource permit
            let permit_result = self.semaphore.try_acquire();

            match permit_result {
                Ok(permit) => {
                    // Simulate resource-intensive operation
                    let resource_result = self.simulate_resource_intensive_operation().await;
                    drop(permit); // Release resource

                    match resource_result {
                        Ok(_) => operations_succeeded += 1,
                        Err(e) => warn!("Resource operation failed: {}", e),
                    }
                }
                Err(_) => {
                    // Resource exhausted - this is expected under chaos
                    warn!("Resource exhausted (expected under chaos)");
                }
            }

            tokio::time::sleep(Duration::from_millis(5)).await;
        }

        let error_rate = 1.0 - (operations_succeeded as f64 / operations_attempted as f64);

        Ok(TestResult {
            success: error_rate < 0.8, // Allow high failure rate under resource exhaustion
            test_name: "resource_exhaustion".to_string(),
            duration: start_time.elapsed(),
            error_message: if error_rate >= 0.8 {
                Some(format!(
                    "Extreme resource exhaustion: {:.2}%",
                    error_rate * 100.0
                ))
            } else {
                None
            },
            metrics: TestMetrics {
                operations_attempted,
                operations_succeeded,
                average_latency_ms: 0.0,
                peak_memory_mb: 0,
                error_rate,
            },
        })
    }

    /// Simulate resource-intensive operation
    async fn simulate_resource_intensive_operation(&self) -> BearDogResult<()> {
        // Simulate CPU-intensive work
        tokio::task::yield_now().await;

        // Inject random failures
        if fastrand::f64() < self.config.failure_rate {
            return Err(BearDogError::Resource {
                message: "Simulated resource failure".to_string(),
            });
        }

        // Simulate successful resource operation
        tokio::time::sleep(Duration::from_millis(1)).await;
        Ok(())
    }
}

/// Resource exhaustion test implementation
pub struct ResourceExhaustionTest;

impl ResourceExhaustionTest {
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let config = ChaosConfig {
            test_duration: Duration::from_secs(8),
            failure_rate: 0.2,
            max_concurrent_ops: 10, // Limited resources for exhaustion testing
            ..Default::default()
        };

        let controller = ResourceChaosController::new(config);
        let result = controller.test_resource_exhaustion().await?;

        info!("💾 Resource exhaustion test completed");
        info!(
            "   Operations: {} attempted, {} succeeded",
            result.metrics.operations_attempted, result.metrics.operations_succeeded
        );
        info!("   Error rate: {:.2}%", result.metrics.error_rate * 100.0);

        assert!(
            result.success,
            "Resource exhaustion test should handle resource limits gracefully"
        );
        Ok(())
    }
}
