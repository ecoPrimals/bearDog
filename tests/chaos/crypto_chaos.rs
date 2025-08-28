

use super::{ChaosConfig, TestMetrics, TestResult};
use beardog_errors::BearDogError;
use beardog_security::crypto_utils::BearDogCrypto;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use tracing::{error, info, warn};

#[derive(Debug)]
pub struct CryptoChaosController {
    config: ChaosConfig,
    crypto: BearDogCrypto,
}

impl CryptoChaosController {
    pub fn new(config: ChaosConfig) -> Self {
        Self {
            config,
            crypto: BearDogCrypto::new(),
        }
    }

    pub async fn test_crypto_chaos(&self) -> Result<TestResult, BearDogError> {
        let start_time = Instant::now();
        let mut operations_attempted = 0u64;
        let mut operations_succeeded = 0u64;
        let mut latencies = Vec::new();

        info!("🔐 Testing cryptographic chaos resilience");

        while start_time.elapsed() < self.config.test_duration {
            operations_attempted += 1;

            let operation_start = Instant::now();
            let operation_result = self.perform_chaotic_crypto_operation().await;
            let latency = operation_start.elapsed();
            latencies.push(latency.as_millis() as f64);

            match operation_result {
                Ok(_) => operations_succeeded += 1,
                Err(e) => warn!("Crypto operation failed: {}", e),
            }

            tokio::time::sleep(Duration::from_millis(1)).await;
        }

        let average_latency = if !latencies.is_empty() {
            latencies.iter().sum::<f64>() / latencies.len() as f64
        } else {
            0.0
        };
        let error_rate = 1.0 - (operations_succeeded as f64 / operations_attempted as f64);

        Ok(TestResult {
            success: error_rate < 0.1, // Crypto should be very reliable
            test_name: "crypto_chaos".to_string(),
            duration: start_time.elapsed(),
            error_message: if error_rate >= 0.1 {
                Some(format!(
                    "High crypto error rate: {:.2}%",
                    error_rate * 100.0
                ))
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

    async fn perform_chaotic_crypto_operation(&self) -> Result<(), BearDogError> {

        let test_data = format_args!("chaos_test_data_{}", fastrand::u64(..).to_string());
        let test_key = format_args!("chaos_key_{}", fastrand::u64(..).to_string());

        if fastrand::f64() < self.config.failure_rate {
            return Err(BearDogError::encryption("crypto", "Simulated crypto chaos failure".to_string(),
            ));
        }

        let encrypted =
            self.crypto
                .encrypt_aes_gcm(test_key.as_bytes(), test_data.as_bytes(), None)?;

        let _decrypted = self.crypto.decrypt_aes_gcm(
            test_key.as_bytes(),
            &encrypted.ciphertext,
            &encrypted.nonce,
        )?;

        Ok(())
    }
}

pub struct CryptoStressTest;

impl CryptoStressTest {
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let config = ChaosConfig {
            test_duration: Duration::from_secs(5),
            failure_rate: 0.05, // 5% failure rate for crypto
            ..Default::default()
        };

        let controller = CryptoChaosController::new(config);
        let result = controller.test_crypto_chaos().await?;

        info!("🔐 Crypto chaos test completed");
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
            "Crypto chaos test should have low error rate"
        );
        Ok(())
    }
}
