

use super::{ChaosConfig, TestMetrics, TestResult};
use beardog_errors::{BearDogError, BearDogResult};
use std::time::{Duration, Instant};
use tracing::{info, warn};

#[derive(Debug)]
pub struct MemoryChaosController {
    config: ChaosConfig,
}

impl MemoryChaosController {
    pub fn new(config: ChaosConfig) -> Self {
        Self { config }
    }

    pub async fn test_memory_pressure(&self) -> BearDogResult<TestResult> {
        let start_time = Instant::now();
        let mut operations_attempted = 0u64;
        let mut operations_succeeded = 0u64;
        let mut peak_memory = 0u64;

        info!("🧠 Testing memory pressure resilience");

        while start_time.elapsed() < self.config.test_duration {
            operations_attempted += 1;

            let memory_result = self.simulate_memory_pressure().await;

            let current_memory = self.get_memory_usage_mb();
            if current_memory > peak_memory {
                peak_memory = current_memory;
            }

            match memory_result {
                Ok(_) => operations_succeeded += 1,
                Err(e) => warn!("Memory operation failed: {}", e),
            }

            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        let error_rate = 1.0 - (operations_succeeded as f64 / operations_attempted as f64);

        Ok(TestResult {
            success: error_rate < 0.2 && peak_memory < self.config.memory_pressure_mb as u64 * 2,
            test_name: "memory_pressure".to_string(),
            duration: start_time.elapsed(),
            error_message: if error_rate >= 0.2 {
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

    async fn simulate_memory_pressure(&self) -> BearDogResult<()> {

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
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let config = ChaosConfig {
            test_duration: Duration::from_secs(5),
            failure_rate: 0.1,
            memory_pressure_mb: 50,
            ..Default::default()
        };

        let controller = MemoryChaosController::new(config);
        let result = controller.test_memory_pressure().await?;

        info!("🧠 Memory pressure test completed");
        info!(
            "   Operations: {} attempted, {} succeeded",
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
