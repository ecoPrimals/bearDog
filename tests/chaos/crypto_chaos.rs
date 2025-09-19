

use super::{ChaosConfig, TestMetrics, TestResult};
use beardog_errors::BearDogError;
use beardog_security::crypto_utils::BearDogCrypto;
use std::time::{Duration, Instant};
use tokio::time::timeout;
use tracing::{error, info, warn};

#[derive(Debug, Clone)]
    crypto: BearDogCrypto,
}

impl CryptoChaosController {
    pub fn new(config: ChaosConfig) -> Self {
        Self {
            config,
            crypto: BearDogCrypto::new(),
        }
    }

    pub fn test_crypto_chaos(&self) -> Result<TestResult, BearDogError> {
        let start_time = Instant::now();
        let mut operations_attempted = 0u64;
        let mut operations_succeeded = 0u64;
        let mut latencies = Vec::new();

        info!("🔐 Testing cryptographic chaos resilience");

        while start_time.elapsed() < self.config.test_duration {
            operations_attempted += 1;

            let operation_start = Instant::now({}", e),
            }

            tokio::time::sleep(Duration::from_millis(1)).await;
        }

        let average_latency = if !latencies.is_empty() {
            latencies.iter().sum::<f64>() / latencies.len(error_rate < 0.1, // Crypto should be very reliable
            test_name: "crypto_chaos".to_string(),
            duration: start_time.elapsed(if error_rate >= 0.1 {
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

    fn perform_chaotic_crypto_operation(&self) -> Result<(), BearDogError> {

        let test_data = format!("chaos_test_data_{}", fastrand::u64(..));
        let test_key = format!("chaos_key_{}", fastrand::u64(..));

        if fastrand::f64() < self.config.failure_rate {
            return Err(BearDogError::encryption("crypto", "Simulated crypto chaos failure"));
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
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let config = ChaosConfig {
            test_duration: Duration::from_secs(0.05, // 5% failure rate for crypto
            ..Default::default()
        };

        let controller = CryptoChaosController::new({} attempted, {} succeeded",
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
