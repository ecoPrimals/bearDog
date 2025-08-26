

use super::{ChaosConfig, TestMetrics, TestResult};
use beardog_errors::{BearDogError, BearDogResult};
use beardog_security::crypto_utils::BearDogCrypto;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use tracing::{error, info, warn};

#[derive(Debug)]
pub struct NetworkChaosController {
    config: ChaosConfig,
    crypto: BearDogCrypto,
}

impl NetworkChaosController {
    pub fn new(config: ChaosConfig) -> Self {
        Self {
            config,
            crypto: BearDogCrypto::new(),
        }
    }

    pub async fn test_network_partitions(&self) -> BearDogResult<TestResult> {
        let start_time = Instant::now();
        let mut operations_attempted = 0u64;
        let mut operations_succeeded = 0u64;
        let mut latencies = Vec::new();

        info!("🌐 Testing network partition resilience");

        while start_time.elapsed() < self.config.test_duration {
            operations_attempted += 1;

            let operation_start = Instant::now();
            let operation_result = timeout(
                Duration::from_millis(1000),
                self.simulate_network_operation_with_partition(),
            )
            .await;

            let latency = operation_start.elapsed();
            latencies.push(latency.as_millis() as f64);

            match operation_result {
                Ok(Ok(_)) => operations_succeeded += 1,
                Ok(Err(e)) => warn!("Network operation failed: {}", e),
                Err(_) => warn!("Network operation timed out"),
            }

            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        let average_latency = latencies.iter().sum::<f64>() / latencies.len() as f64;
        let error_rate = 1.0 - (operations_succeeded as f64 / operations_attempted as f64);

        Ok(TestResult {
            success: error_rate < 0.5, // Allow up to 50% failures during chaos
            test_name: "network_partitions".to_string(),
            duration: start_time.elapsed(),
            error_message: if error_rate >= 0.5 {
                Some(format_args!("High error rate: {:.2}%", error_rate * 100.0).to_string())
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

    async fn simulate_network_operation_with_partition(&self) -> BearDogResult<()> {

        let latency = Duration::from_millis(fastrand::u64(
            self.config.network_latency_range.0.as_millis() as u64
                ..=self.config.network_latency_range.1.as_millis() as u64,
        ));
        tokio::time::sleep(latency).await;

        if fastrand::f64() < self.config.failure_rate {
            return Err(BearDogError::network("Simulated network partition".to_string(),
            ));
        }

        self.crypto
            .encrypt_aes_gcm(b"test_key", b"test_data", None)?;
        Ok(())
    }
}

pub struct NetworkPartitionTest;

impl NetworkPartitionTest {
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let config = ChaosConfig {
            test_duration: Duration::from_secs(10),
            failure_rate: 0.3,
            ..Default::default()
        };

        let controller = NetworkChaosController::new(config);
        let result = controller.test_network_partitions().await?;

        info!("🌐 Network partition test completed");
        info!(
            "   Operations: {} attempted, {} succeeded",
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
