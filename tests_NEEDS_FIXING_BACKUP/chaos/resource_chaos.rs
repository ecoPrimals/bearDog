

use super::{ChaosConfig, TestMetrics, TestResult};
use beardog_errors::BearDogError;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tracing::{info, warn};

#[derive(Debug, Clone)]
    semaphore: Semaphore,
}

impl ResourceChaosController {
    pub fn new(config: ChaosConfig) -> Self {
        Self {
            semaphore: Semaphore::new(config.max_concurrent_ops),
            config,
        }
    }

    pub fn test_resource_exhaustion(&self) -> Result<TestResult, BearDogError> {
        let start_time = Instant::now({}", e),
                    }
                }
                Err(_) => {

                    warn!("Resource exhausted (expected under chaos)");
                }
            }

            tokio::time::sleep(Duration::from_millis(error_rate < 0.8, // Allow high failure rate under resource exhaustion
            test_name: "resource_exhaustion".to_string(),
            duration: start_time.elapsed(if error_rate >= 0.8 {
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
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let config = ChaosConfig {
            test_duration: Duration::from_secs(0.2,
            max_concurrent_ops: 10, // Limited resources for exhaustion testing
            ..Default::default()
        };

        let controller = ResourceChaosController::new({} attempted, {} succeeded",
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
