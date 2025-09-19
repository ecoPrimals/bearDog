

use super::{ChaosConfig, TestMetrics, TestResult};
use beardog_errors::BearDogError;
use std::time::{Duration, Instant};
use tracing::{info, warn};

#[derive(Debug, Clone)]
}

impl MemoryChaosController {
    pub fn new(config: ChaosConfig) -> Self {
        Self { config }
    }

    pub fn test_memory_pressure(&self) -> Result<TestResult, BearDogError> {
        let start_time = Instant::now({}", e),
            }

            tokio::time::sleep(Duration::from_millis(error_rate < 0.2 && peak_memory < self.config.memory_pressure_mb as u64 * 2,
            test_name: "memory_pressure".to_string(),
            duration: start_time.elapsed(if error_rate >= 0.2 {
                Some(format!(
                    "High memory error rate: {:.2}%",
                    error_rate * 100.0
                ))
            } else {
                None
            },
            metrics: TestMetrics {
                operations_attempted,
                operations_succeeded,
                average_latency_ms: 0.0,
                peak_memory_mb: peak_memory,
                error_rate,
            },
        })
    }

    fn simulate_memory_pressure(&self) -> Result<(), BearDogError> {

        let _memory_pressure: Vec<u8> = vec![0u8; self.config.memory_pressure_mb * 1024];

        if fastrand::f64() < self.config.failure_rate {
            return Err(BearDogError::Resource {
                message: "Simulated memory pressure failure".to_string(),
            });
        }

        Ok(())
    }

    fn get_memory_usage_mb(&self) -> u64 {

        self.config.memory_pressure_mb as u64
    }
}

pub struct MemoryPressureTest;

impl MemoryPressureTest {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let config = ChaosConfig {
            test_duration: Duration::from_secs(0.1,
            memory_pressure_mb: 50,
            ..Default::default()
        };

        let controller = MemoryChaosController::new({} attempted, {} succeeded",
            result.metrics.operations_attempted, result.metrics.operations_succeeded
        );
        info!("   Peak memory: {} MB", result.metrics.peak_memory_mb);
        info!("   Error rate: {:.2}%", result.metrics.error_rate * 100.0);

        assert!(
            result.success,
            "Memory pressure test should handle reasonable memory load"
        );
        Ok(())
    }
}
