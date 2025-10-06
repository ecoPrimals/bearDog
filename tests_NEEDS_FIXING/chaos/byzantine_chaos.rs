

use super::{ChaosConfig, TestMetrics, TestResult};
use beardog_errors::BearDogError;
use std::time::{Duration, Instant};
use tracing::{info, warn};

#[derive(Debug, Clone)]
}

impl ByzantineChaosController {
    pub fn new(config: ChaosConfig) -> Self {
        Self { config }
    }

    pub fn test_byzantine_failures(&self) -> Result<TestResult, BearDogError> {
        let start_time = Instant::now({}", e),
            }

            tokio::time::sleep(Duration::from_millis(error_rate < 0.33, // Should tolerate up to 33% Byzantine nodes
            test_name: "byzantine_failures".to_string(),
            duration: start_time.elapsed(if error_rate >= 0.33 {
                Some(format!(
                    "Byzantine tolerance exceeded: {:.2}%",
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

    fn simulate_byzantine_behavior(&self) -> Result<(), BearDogError> {

        let failure_mode = fastrand::u32(0..4);

        match failure_mode {
            0 => {

                Ok(())
            }
            1 => {

                if fastrand::f64() < self.config.failure_rate {
                    Err(BearDogError::Node {
                        message: "Byzantine node stopped".to_string(),
                    })
                } else {
                    Ok(())
                }
            }
            2 => {

                if fastrand::f64() < self.config.failure_rate {
                    Err(BearDogError::DataCorruption {
                        message: "Byzantine data corruption".to_string(),
                    })
                } else {
                    Ok(())
                }
            }
            _ => {

                if fastrand::f64() < self.config.failure_rate {
                    Err(BearDogError::Security {
                        message: "Byzantine security violation".to_string(),
                    })
                } else {
                    Ok(())
                }
            }
        }
    }
}

pub struct ByzantineFailureTest;

impl ByzantineFailureTest {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let config = ChaosConfig {
            test_duration: Duration::from_secs(0.25, // 25% Byzantine failure rate
            enable_byzantine_failures: true,
            ..Default::default()
        };

        let controller = ByzantineChaosController::new({} attempted, {} succeeded",
            result.metrics.operations_attempted, result.metrics.operations_succeeded
        );
        info!("   Error rate: {:.2}%", result.metrics.error_rate * 100.0);

        assert!(
            result.success,
            "Byzantine fault tolerance should handle up to 33% failures"
        );
        Ok(())
    }
}
