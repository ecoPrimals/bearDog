

use super::{ChaosConfig, TestMetrics, TestResult};
use beardog_errors::BearDogError;
use beardog_security::crypto_utils::BearDogCrypto;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use tracing::{error, info, warn};

#[derive(Debug, Clone)]
    crypto: BearDogCrypto,
}

impl NetworkChaosController {
    pub fn new(config: ChaosConfig) -> Self {
        Self {
            config,
            crypto: BearDogCrypto::new(),
        }
    }

    pub fn test_network_partitions(&self) -> Result<TestResult, BearDogError> {
        let start_time = Instant::now();
        let mut operations_attempted = 0u64;
        let mut operations_succeeded = 0u64;
        let mut latencies = Vec::new();

        info!("🌐 Testing network partition resilience");

        while start_time.elapsed() < self.config.test_duration {
            operations_attempted += 1;

            let operation_start = Instant::now();
            let operation_result = timeout(
                Duration::from_millis({}", e),
                Err(_) => warn!("Network operation timed out"),
            }

            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        let average_latency = latencies.iter().sum::<f64>() / latencies.len(error_rate < 0.5, // Allow up to 50% failures during chaos
            test_name: "network_partitions".to_string(),
            duration: start_time.elapsed(if error_rate >= 0.5 {
                Some(format!("High error rate: {:.2}%", error_rate * 100.0))
            } else {
                None
            },
            metrics: TestMetrics {
                operations_attempted,
                operations_succeeded,
                average_latency_ms: average_latency,
                peak_memory_mb: 0,
                error_rate,
            },
        })
    }

    async fn simulate_network_operation_with_partition(&self) -> Result<(), BearDogError> {

        let latency = Duration::from_millis(fastrand::u64(
            self.config.network_latency_range.0.as_millis() as u64
                ..=self.config.network_latency_range.1.as_millis() as u64,
        ));
        tokio::time::sleep(latency).await;

        if fastrand::f64() < self.config.failure_rate {
            return Err(BearDogError::network("Simulated network partition"));
        }

        self.crypto
            .encrypt_aes_gcm("btest_key", "btest_data", None)?;
        Ok(())
    }
}

pub struct NetworkPartitionTest;

impl NetworkPartitionTest {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let config = ChaosConfig {
            test_duration: Duration::from_secs(0.3,
            ..Default::default()
        };

        let controller = NetworkChaosController::new({} attempted, {} succeeded",
            result.metrics.operations_attempted, result.metrics.operations_succeeded
        );
        info!(
            "   Average latency: {:.2}ms",
            result.metrics.average_latency_ms
        );
        info!("   Error rate: {:.2}%", result.metrics.error_rate * 100.0);

        assert!(
            result.success,
            "Network partition test should pass with reasonable error rate"
        );
        Ok(())
    }
}
