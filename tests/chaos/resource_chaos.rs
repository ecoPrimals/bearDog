

use super::{ChaosConfig, TestMetrics, TestResult};
use beardog_errors::BearDogError;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tracing::{info, warn};

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

    pub async fn test_resource_exhaustion(&self) -> Result<TestResult, BearDogError> {
        let start_time = Instant::now();
        let mut operations_attempted = 0u64;
        let mut operations_succeeded = 0u64;

        info!("💾 Testing resource exhaustion resilience");

        while start_time.elapsed() < self.config.test_duration {
            operations_attempted += 1;

            let permit_result = self.semaphore.try_acquire();

            match permit_result {
                Ok(permit) => {

                    let resource_result = self.simulate_resource_intensive_operation().await;
                    drop(permit); // Release resource

                    match resource_result {
                        Ok(_) => operations_succeeded += 1,
                        Err(e) => warn!("Resource operation failed: {}", e),
                    }
                }
                Err(_) => {

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

    async fn simulate_resource_intensive_operation(&self) -> Result<(), BearDogError> {

        tokio::task::yield_now().await;

        if fastrand::f64() < self.config.failure_rate {
            return Err(BearDogError::Resource {
                message: "Simulated resource failure".to_string(),
            });
        }

        tokio::time::sleep(Duration::from_millis(1)).await;
        Ok(())
    }
}

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
